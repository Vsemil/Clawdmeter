//! What the hook needs to know about a session beyond its input: whether it
//! still has background work running, and which project it's in.
//!
//! A turn that only yielded to a background task (a build, a flash, a
//! monitor) isn't done: the harness resumes the session when the task
//! reports back. Both kinds of task live in the session's tasks dir,
//! `<tmp>/claude-<uid>/<project>/<session>/tasks`:
//!
//! * shell tasks: `.output` files the running shell holds open (`lsof`);
//! * agents: symlinks to the agent's transcript, appended in bursts and never
//!   held open. An agent that finished announces itself in the session's own
//!   transcript as `<task-id>ID</task-id>`, the only unambiguous completion
//!   mark on disk; one with a recently written transcript and no such mark
//!   is still working.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

/// An agent transcript idle longer than this is dead or long finished.
const AGENT_FRESH: Duration = Duration::from_secs(300);

/// A session id that is safe as a path component.
fn safe_id(sid: &str) -> bool {
    !sid.is_empty() && sid != "." && sid != ".." && !sid.contains(['/', '\\', '\0', '\n'])
}

#[cfg(unix)]
fn task_dirs(sid: &str) -> Vec<PathBuf> {
    use std::os::unix::fs::MetadataExt as _;
    let Some(home) = std::env::var_os("HOME") else { return Vec::new() };
    let Ok(uid) = fs::metadata(home).map(|m| m.uid()) else { return Vec::new() };
    let mut out = Vec::new();
    for root in ["/private/tmp", "/tmp"] {
        let Ok(projects) = fs::read_dir(Path::new(root).join(format!("claude-{uid}"))) else { continue };
        for p in projects.flatten() {
            let t = p.path().join(sid).join("tasks");
            if t.is_dir() && !out.contains(&t) {
                out.push(t);
            }
        }
    }
    out
}

#[cfg(not(unix))]
fn task_dirs(_sid: &str) -> Vec<PathBuf> {
    Vec::new()
}

/// A process holds a file in `dir` open (a running shell task).
fn shell_task_open(dir: &Path) -> bool {
    // lsof's exit code is 1 even with matches: test its output.
    Command::new("lsof").arg("-w").arg("+d").arg(dir).output().is_ok_and(|o| !o.stdout.is_empty())
}

/// An agent in `dir` is still working: its transcript (the link's target)
/// was written within [`AGENT_FRESH`] and it hasn't reported back to the
/// session (`transcript`, read only when there's a candidate).
fn agent_running(dir: &Path, transcript: Option<&Path>, now: SystemTime) -> bool {
    let Ok(entries) = fs::read_dir(dir) else { return false };
    let mut session_log: Option<String> = None;
    for e in entries.flatten() {
        let link = e.path();
        if !fs::symlink_metadata(&link).is_ok_and(|m| m.file_type().is_symlink()) {
            continue;
        }
        let Ok(target) = fs::metadata(&link) else { continue }; // dangling: gone
        let fresh = target.modified().is_ok_and(|t| now.duration_since(t).unwrap_or_default() < AGENT_FRESH);
        if !target.is_file() || !fresh {
            continue;
        }
        let name = e.file_name().to_string_lossy().into_owned();
        let id = name.strip_suffix(".output").unwrap_or(&name);
        let log =
            session_log.get_or_insert_with(|| transcript.and_then(|t| fs::read_to_string(t).ok()).unwrap_or_default());
        if !log.contains(&format!("<task-id>{id}</task-id>")) {
            return true;
        }
    }
    false
}

/// The session still has a shell task or an agent running.
pub fn has_running_tasks(sid: &str, transcript: Option<&Path>) -> bool {
    if !safe_id(sid) {
        return false;
    }
    let now = SystemTime::now();
    task_dirs(sid).iter().any(|t| shell_task_open(t) || agent_running(t, transcript, now))
}

/// The project a hook input is from: the name of the git root above `cwd`
/// (a `.git` dir or file), else of `cwd` itself.
pub fn project_of(cwd: &str) -> String {
    let p = Path::new(cwd);
    let root = p.ancestors().find(|a| a.join(".git").exists()).unwrap_or(p);
    root.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_is_the_git_root() {
        let d = std::env::temp_dir().join(format!("cm-proj-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("repo/.git")).unwrap();
        fs::create_dir_all(d.join("repo/src/deep")).unwrap();
        fs::create_dir_all(d.join("loose")).unwrap();
        assert_eq!(project_of(d.join("repo/src/deep").to_str().unwrap()), "repo");
        assert_eq!(project_of(d.join("loose").to_str().unwrap()), "loose");
        assert_eq!(project_of(""), "");
        let _ = fs::remove_dir_all(&d);
    }

    #[cfg(unix)]
    #[test]
    fn agents_run_until_they_report_back() {
        let d = std::env::temp_dir().join(format!("cm-tasks-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("tasks")).unwrap();
        fs::write(d.join("agent.jsonl"), "{}\n").unwrap();
        std::os::unix::fs::symlink(d.join("agent.jsonl"), d.join("tasks/a1.output")).unwrap();
        let session = d.join("session.jsonl");
        fs::write(&session, "{\"type\":\"user\"}\n").unwrap();
        let now = SystemTime::now();
        assert!(agent_running(&d.join("tasks"), Some(&session), now), "fresh and silent: running");
        fs::write(&session, "... <task-id>a1</task-id> ...\n").unwrap();
        assert!(!agent_running(&d.join("tasks"), Some(&session), now), "reported back: done");
        assert!(!agent_running(&d.join("tasks"), Some(&session), now + AGENT_FRESH * 2), "stale: dead");
        assert!(!has_running_tasks("../escape", None));
        let _ = fs::remove_dir_all(&d);
    }
}
