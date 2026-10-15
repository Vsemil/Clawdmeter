//! `clawdmeter mcp`: a stdio MCP server, so an assistant can put a line on
//! the device itself. Register it once:
//!
//! ```text
//! claude mcp add --scope user clawdmeter -- /path/to/clawdmeter mcp
//! ```
//!
//! No new transport: `show_message` writes the same `attention` file the
//! hooks write, so the daemon picks the message up on its next tick and the
//! device treats it like any alert (caption, colour and melody by style,
//! the text after the caption). `device_status` reads the tray's
//! `app-status.json`. Each message is its own session (`mcp-<time>`), so a
//! second one replaces the first; `clear_message` clears the last one.

use std::fs;
use std::io::{self, BufRead as _, Write as _};
use std::path::Path;

use clawdmeter_core::{unix_now, Paths};
use serde_json::{json, Value};

use crate::ipc;

const PROTOCOL_VERSION: &str = "2024-11-05";
/// The device shows this many bytes after the caption.
const TEXT_LIMIT: usize = clawdmeter_core::attention::PROJECT_LIMIT;
/// A status snapshot older than this: the app isn't running.
const STATUS_STALE: i64 = 180;

/// Style → (alert kind, what it means). The caption is the device's own, in
/// its language, so the styles are described by role.
const STYLES: [(&str, &str, &str); 6] = [
    ("done", "done", "green: a job finished"),
    ("waiting", "idle", "orange: Claude is blocked on an answer"),
    ("permission", "permission", "red: a tool wants approval"),
    ("meeting", "cal", "blue: something starts shortly"),
    ("meeting_started", "calstart", "yellow: it is starting now"),
    ("message", "msg", "orange, no caption: just the text"),
];

fn tools() -> Value {
    let styles: Vec<&str> = STYLES.iter().map(|s| s.0).collect();
    let roles = STYLES.iter().map(|(s, _, r)| format!("{s} = {r}")).collect::<Vec<_>>().join("; ");
    json!([
        {
            "name": "show_message",
            "description": "Show a message on the user's Clawdmeter (the ESP32 usage display on their desk): \
                            wakes the screen, plays the style's melody and shows the text after the style's \
                            caption. Use it when the user asks to be told something on the device, or to flag \
                            a long job finishing while they are away from the screen.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": format!("Text shown on the device, up to {TEXT_LIMIT} bytes (longer is cut)."),
                    },
                    "style": {
                        "type": "string",
                        "enum": styles,
                        "default": "done",
                        "description": format!("Caption, colour and melody: {roles}"),
                    },
                },
                "required": ["text"],
            },
        },
        {
            "name": "clear_message",
            "description": "Dismiss the message show_message put up. Silent.",
            "inputSchema": { "type": "object", "properties": {} },
        },
        {
            "name": "device_status",
            "description": "Is the Clawdmeter connected, and what is it showing? Reports the app's link \
                            state and the usage it last read.",
            "inputSchema": { "type": "object", "properties": {} },
        },
    ])
}

fn last_session_file(paths: &Paths) -> std::path::PathBuf {
    paths.state_dir.join("mcp-session")
}

/// Write the attention file the way the hook does (aside, then renamed in).
fn write_alert(dir: &Path, body: &Value) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let tmp = dir.join(format!("attention.mcp.{}", std::process::id()));
    fs::write(&tmp, body.to_string() + "\n")?;
    fs::rename(&tmp, dir.join("attention")).inspect_err(|_| {
        let _ = fs::remove_file(&tmp);
    })
}

/// What the app last reported: (reporting, connected, details).
fn app_state(paths: &Paths) -> (bool, bool, Option<ipc::StatusSnapshot>) {
    match ipc::read_status(paths) {
        Some(s) => {
            let live = unix_now() - s.updated_at <= STATUS_STALE;
            (live, live && s.link == "connected", Some(s))
        }
        None => (false, false, None),
    }
}

fn show_message(paths: &Paths, args: &Value) -> Result<String, String> {
    let text: String =
        args.get("text").and_then(Value::as_str).unwrap_or("").split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() {
        return Err("text is empty: nothing to show".into());
    }
    let style = args.get("style").and_then(Value::as_str).unwrap_or("done");
    let Some((_, kind, _)) = STYLES.iter().find(|s| s.0 == style) else {
        let names: Vec<&str> = STYLES.iter().map(|s| s.0).collect();
        return Err(format!("unknown style {style:?}; use one of {}", names.join(", ")));
    };
    let session = format!("mcp-{}", unix_now());
    write_alert(&paths.state_dir, &json!({ "k": kind, "m": text, "s": session, "p": text }))
        .map_err(|e| format!("couldn't write the alert: {e}"))?;
    let _ = fs::write(last_session_file(paths), &session);
    let mut note = format!("Shown on the Clawdmeter ({style}): {text:?}");
    match app_state(paths) {
        (false, _, _) => note.push_str(
            ". Warning: the Clawdmeter app isn't reporting; the message only reaches the device if it starts within a minute.",
        ),
        (true, false, _) => note.push_str(". Warning: the app has no Bluetooth link to the device right now."),
        _ => {}
    }
    Ok(note)
}

fn clear_message(paths: &Paths) -> Result<String, String> {
    let Ok(session) = fs::read_to_string(last_session_file(paths)) else {
        return Ok("Nothing to clear.".into());
    };
    let f = paths.state_dir.join(format!("clear-{}", session.trim()));
    fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(&f)
        .and_then(|f| f.set_modified(std::time::SystemTime::now()))
        .map_err(|e| format!("couldn't clear: {e}"))?;
    let _ = fs::remove_file(last_session_file(paths));
    Ok("Cleared: the Clawdmeter goes back to its usage screen.".into())
}

fn device_status(paths: &Paths) -> String {
    let (live, connected, snap) = app_state(paths);
    let mut parts = vec![
        format!("app: {}", if live { "reporting" } else { "not reporting" }),
        format!("link: {}", if connected { "connected" } else { "not connected" }),
    ];
    if let Some(s) = snap {
        if let Some(name) = &s.device_name {
            parts.push(format!("device: {name}"));
        }
        if s.logged_out {
            parts.push("no data: every account is logged out".into());
        } else {
            if let Some(p) = s.session_pct {
                parts.push(format!("session {p}%{}", if s.window_started { "" } else { " (window not started)" }));
            }
            if let Some(p) = s.week_pct {
                parts.push(format!("weekly {p}%"));
            }
            if s.stale {
                parts.push("numbers are stale (polls failing)".into());
            }
        }
        for e in &s.errors {
            parts.push(format!("{}: {}", e.dir, e.message));
        }
        parts.push(format!("alert up: {}", if s.attention { "yes" } else { "no" }));
        parts.push(format!("active sessions: {}", s.sessions));
    }
    parts.join("\n")
}

fn call(paths: &Paths, name: &str, args: &Value) -> Result<String, String> {
    match name {
        "show_message" => show_message(paths, args),
        "clear_message" => clear_message(paths),
        "device_status" => Ok(device_status(paths)),
        _ => Err(format!("unknown tool {name:?}")),
    }
}

/// One JSON-RPC request → its response (`None` for a notification).
pub fn handle(paths: &Paths, req: &Value) -> Option<Value> {
    let id = req.get("id").cloned();
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(Value::Null);
    let result = match method {
        "initialize" => json!({
            "protocolVersion": params.get("protocolVersion").and_then(Value::as_str).unwrap_or(PROTOCOL_VERSION),
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "clawdmeter", "version": crate::version() },
        }),
        "tools/list" => json!({ "tools": tools() }),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(Value::Null);
            match call(paths, name, &args) {
                Ok(text) => json!({ "content": [{ "type": "text", "text": text }] }),
                Err(e) => json!({ "content": [{ "type": "text", "text": format!("Error: {e}") }], "isError": true }),
            }
        }
        "ping" => json!({}),
        _ => {
            let id = id?; // a notification: no reply
            return Some(json!({ "jsonrpc": "2.0", "id": id,
                                "error": { "code": -32601, "message": format!("unknown method {method}") } }));
        }
    };
    let id = id?;
    Some(json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

/// Entry point for `clawdmeter mcp`: newline-delimited JSON-RPC on stdio.
pub fn main() -> i32 {
    let paths = Paths::for_user();
    let stdout = io::stdout();
    for line in io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        let Ok(req) = serde_json::from_str::<Value>(line.trim()) else { continue };
        if let Some(resp) = handle(&paths, &req) {
            let mut out = stdout.lock();
            let _ = writeln!(out, "{resp}");
            let _ = out.flush();
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(tag: &str) -> Paths {
        let d = std::env::temp_dir().join(format!("cm-mcp-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        Paths::under(&d)
    }

    fn call_tool(p: &Paths, name: &str, args: Value) -> Value {
        handle(
            p,
            &json!({"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": name, "arguments": args}}),
        )
        .unwrap()
    }

    #[test]
    fn show_then_clear_goes_through_the_attention_files() {
        let p = paths("show");
        let r = call_tool(&p, "show_message", json!({"text": "Сборка  готова", "style": "done"}));
        assert!(r["result"]["isError"].is_null(), "{r}");
        let a: Value = serde_json::from_str(&fs::read_to_string(p.state_dir.join("attention")).unwrap()).unwrap();
        assert_eq!((a["k"].as_str(), a["p"].as_str()), (Some("done"), Some("Сборка готова")));
        let session = a["s"].as_str().unwrap().to_owned();
        call_tool(&p, "clear_message", json!({}));
        assert!(p.state_dir.join(format!("clear-{session}")).exists());
        let _ = fs::remove_dir_all(p.home);
    }

    #[test]
    fn bad_calls_are_tool_errors_and_notifications_get_no_reply() {
        let p = paths("bad");
        assert_eq!(call_tool(&p, "show_message", json!({"text": "  "}))["result"]["isError"], true);
        assert_eq!(call_tool(&p, "show_message", json!({"text": "x", "style": "loud"}))["result"]["isError"], true);
        assert!(handle(&p, &json!({"jsonrpc": "2.0", "method": "notifications/initialized"})).is_none());
        let list = handle(&p, &json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"})).unwrap();
        assert_eq!(list["result"]["tools"].as_array().unwrap().len(), 3);
        let _ = fs::remove_dir_all(p.home);
    }
}
