#!/usr/bin/env python3
"""MCP server: let Claude Code put a message on the Clawdmeter itself.

The BLE daemon already watches a flag file for the hook-driven alerts
(tools/claude-attention-hook.sh writes it; the daemon forwards it as the
"n"/"np" payload fields within one TICK). This server writes the same file,
so an assistant can raise the device's alert screen — chime, creature and
all — with no new firmware, transport or pairing involved.

Stdio JSON-RPC, newline-delimited, no third-party imports: it has to start
under any Python the user has, in any project, without a venv.

Register it once, globally:

    claude mcp add --scope user clawdmeter -- python3 \
        ~/IdeaProjects/Clawdmeter/tools/clawdmeter_mcp.py
"""
import json
import os
import sys
import time
from pathlib import Path

# Keep in sync with daemon/claude_usage_daemon.py — importing it isn't an
# option (it pulls in bleak/httpx from the daemon's venv) and this server has
# to start under any python.
STATE_DIR = Path.home() / ".config" / "claude-usage-monitor"
ATTN_FILE = STATE_DIR / "attention"
STATUS_FILE = STATE_DIR / "status.json"
# Firmware context-line budget ("np"): two wrapped lines on the alert screen.
NP_MAX_CHARS = 48
# A status beat older than this means the daemon stopped writing it.
STATUS_STALE_S = 180
# The daemon only forwards these (its ATTN_TYPES); each carries its own
# caption, color and melody on the device — the caption is fixed and follows
# the device's language, so describe the styles by role, not by quoting it.
STYLES = {
    "waiting": ("input", "amber — Claude is blocked on an answer"),
    "permission": ("perm", "amber — a tool wants approval"),
    "done": ("done", "green — a job finished"),
    "meeting": ("cal", "blue — something starts shortly"),
    "meeting_started": ("calstart", "yellow — it is starting now"),
}
PROTOCOL_VERSION = "2024-11-05"

TOOLS = [
    {
        "name": "show_message",
        "description": (
            "Show a message on the user's Clawdmeter (the ESP32 usage display "
            "on their desk): wakes the screen, plays the style's melody and "
            "prints the text under the caption. Use it when the user asks to "
            "be told something on the device, or to flag a long job finishing "
            "while they are away from the screen."
        ),
        "inputSchema": {
            "type": "object",
            "properties": {
                "text": {
                    "type": "string",
                    "description": f"Line shown on the device, up to {NP_MAX_CHARS} "
                                   "characters (longer text is cut).",
                },
                "style": {
                    "type": "string",
                    "enum": list(STYLES),
                    "default": "done",
                    "description": "Caption, color and melody: "
                                   + "; ".join(f"{k} = {v[1]}" for k, v in STYLES.items()),
                },
            },
            "required": ["text"],
        },
    },
    {
        "name": "clear_message",
        "description": "Dismiss whatever the Clawdmeter is showing and return it "
                       "to the usage screen. Silent — no melody.",
        "inputSchema": {"type": "object", "properties": {}},
    },
    {
        "name": "device_status",
        "description": "Is the Clawdmeter connected, and what is it showing? "
                       "Reports the daemon's link state and its last usage payload.",
        "inputSchema": {"type": "object", "properties": {}},
    },
]


def write_flag(kind: str, text: str = "") -> None:
    STATE_DIR.mkdir(parents=True, exist_ok=True)
    # Line 1 = event type, line 2 = the context line. Written whole so the
    # daemon never reads a half-written flag.
    tmp = ATTN_FILE.with_suffix(".tmp")
    tmp.write_text(f"{kind}\n{text}\n")
    os.replace(tmp, ATTN_FILE)


def daemon_state() -> dict:
    """The daemon's own status beat: {connected, payload, ts}, or empty.

    The daemon rewrites STATUS_FILE on every write to the device, so this is
    the state it actually reached the display with — no log scraping, no
    guessing from prose that isn't a contract.
    """
    try:
        st = json.loads(STATUS_FILE.read_text())
    except (OSError, ValueError):
        return {"live": False, "connected": False, "payload": None, "age": None}
    age = max(0.0, time.time() - float(st.get("ts") or 0))
    return {
        "live": age <= STATUS_STALE_S,
        "connected": bool(st.get("connected")) and age <= STATUS_STALE_S,
        "payload": st.get("payload"),
        "age": age,
    }


def call_tool(name: str, args: dict) -> str:
    if name == "show_message":
        text = str(args.get("text", "")).strip()[:NP_MAX_CHARS]
        if not text:
            raise ValueError("text is empty — nothing to show")
        style = args.get("style", "done")
        if style not in STYLES:
            raise ValueError(f"unknown style {style!r}; use one of {', '.join(STYLES)}")
        write_flag(STYLES[style][0], text)      # first — the daemon is polling
        note = f"Shown on the Clawdmeter ({style}): {text!r}"
        state = daemon_state()
        if not state["live"]:
            note += ". Warning: the daemon isn't reporting — the message only " \
                    "reaches the device if it comes back within a minute."
        elif not state["connected"]:
            note += ". Warning: the daemon reports no BLE link right now."
        return note

    if name == "clear_message":
        write_flag("clear")
        return "Cleared — the Clawdmeter returns to the usage screen."

    if name == "device_status":
        s = daemon_state()
        p = s["payload"] or {}
        parts = [
            f"daemon: {'reporting' if s['live'] else 'not reporting'}",
            f"link: {'connected' if s['connected'] else 'not connected'}",
        ]
        if p:
            parts.append(f"last beat {s['age']:.0f}s ago")
            if p.get("ok"):
                parts.append(f"session {p.get('s')}% (resets {p.get('srt')})")
                parts.append(f"weekly {p.get('w')}% (resets {p.get('wrt')})")
                if p.get("f") is not None:
                    parts.append(f"{p.get('fn', 'scoped')} {p.get('f')}%")
                parts.append(f"active sessions: {p.get('a', 0)}")
            else:
                parts.append(f"no data ({p.get('err', 'unknown')})")
        return "\n".join(parts)

    raise ValueError(f"unknown tool {name!r}")


def respond(msg_id, result=None, error=None) -> None:
    msg = {"jsonrpc": "2.0", "id": msg_id}
    msg["error" if error else "result"] = error or result
    sys.stdout.write(json.dumps(msg) + "\n")
    sys.stdout.flush()


def main() -> None:
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except json.JSONDecodeError:
            continue
        method, msg_id = req.get("method"), req.get("id")
        params = req.get("params") or {}

        if method == "initialize":
            respond(msg_id, {
                "protocolVersion": params.get("protocolVersion", PROTOCOL_VERSION),
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "clawdmeter", "version": "1.0.0"},
            })
        elif method == "tools/list":
            respond(msg_id, {"tools": TOOLS})
        elif method == "tools/call":
            try:
                text = call_tool(params.get("name", ""), params.get("arguments") or {})
                respond(msg_id, {"content": [{"type": "text", "text": text}]})
            except Exception as e:  # surfaced to the model, not the transport
                respond(msg_id, {"content": [{"type": "text", "text": f"Error: {e}"}],
                                 "isError": True})
        elif method == "ping":
            respond(msg_id, {})
        elif msg_id is not None:
            respond(msg_id, error={"code": -32601, "message": f"unknown method {method}"})
        # notifications (no id) need no reply


if __name__ == "__main__":
    main()
