//! Attention alerts from the hook (`clawdmeter hook alert|clear`).
//!
//! The hook writes `<state>/attention` (`{"m": message, "s": session id,
//! "k": kind}`, see [`AlertKind`])
//! when a session needs the user and touches `<state>/clear-<session id>` when
//! that session moves on. Each tick the daemon consumes both:
//!
//! * an alert file older than 60 s is dropped (written while the device was
//!   away), a bare `touch` still alerts with no message;
//! * a second alert from the session already alerting is ignored
//!   (AskUserQuestion notifies twice);
//! * a clear only ends the alert when it names the alerting session and is
//!   newer than the alert (nanosecond mtimes: both can land in one second).
//!
//! An alert also ends without a clear: when the device reports that it was
//! dismissed there (a tap, or its own timeout), and after [`ALERT_TTL`]
//! ([`TEST_ALERT_TTL`] for the Hooks page's test alert), because a session
//! that is closed or killed never sends its clear.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use unicode_normalization::UnicodeNormalization;

/// Alerts older than this when first seen are ignored.
pub const ATTENTION_MAX_AGE: Duration = Duration::from_secs(60);
/// An alert nothing has cleared ends this long after it was raised: the
/// device's own alert timeout, so both drop it together.
pub const ALERT_TTL: Duration = Duration::from_secs(2 * 60);
/// The test alert ends sooner: nothing ever clears its session.
pub const TEST_ALERT_TTL: Duration = Duration::from_secs(30);
/// A moment (a finished turn) is news for this long, then it's gone.
pub const DONE_ALERT_TTL: Duration = Duration::from_secs(30);
/// The session id of the Hooks page's test alert.
pub const TEST_SESSION: &str = "clawdmeter-test";
/// The device's message buffer holds this many UTF-8 bytes.
pub const MESSAGE_LIMIT: usize = 120;
/// ...and its project name buffer this many.
pub const PROJECT_LIMIT: usize = 40;

/// What a session is waiting for. The hook records it (`k`) from Claude
/// Code's hook input; which kinds alert is the `alert_on` setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertKind {
    /// A tool needs permission (`Notification`, `permission_prompt`). Also
    /// a `Notification` with no type, as older Claude Code sends.
    Permission,
    /// Claude asks a question (`PreToolUse` on `AskUserQuestion`).
    Question,
    /// The prompt has sat idle, waiting for input (`idle_prompt`).
    Idle,
    /// An MCP server asks for input (`elicitation_dialog`).
    Elicitation,
    /// Any other notification type; follows [`AlertKind::Permission`].
    Other,
    /// A long turn finished (the hook on `Stop`). A moment, not a wait:
    /// it never takes the screen from a session that is waiting for you.
    Done,
    /// A meeting soon (the calendar).
    Calendar,
    /// The meeting has started.
    CalendarStart,
    /// Free text put up from outside (the MCP server): shown as written.
    Message,
}

impl AlertKind {
    /// The `k` value the hook writes.
    pub fn as_str(self) -> &'static str {
        match self {
            AlertKind::Permission => "permission",
            AlertKind::Question => "question",
            AlertKind::Idle => "idle",
            AlertKind::Elicitation => "elicitation",
            AlertKind::Other => "other",
            AlertKind::Done => "done",
            AlertKind::Calendar => "cal",
            AlertKind::CalendarStart => "calstart",
            AlertKind::Message => "msg",
        }
    }

    /// Which alert keeps the screen when two sessions want it: the higher
    /// rank wins, a tie goes to the newer one.
    pub fn rank(self) -> u8 {
        match self {
            AlertKind::Permission | AlertKind::Other => 6,
            AlertKind::Question | AlertKind::Elicitation | AlertKind::CalendarStart => 5,
            AlertKind::Idle | AlertKind::Calendar | AlertKind::Message => 4,
            AlertKind::Done => 2,
        }
    }

    /// A session waiting for you (until it's answered), as opposed to a
    /// moment that passes. A wait that loses the screen queues for it; a
    /// moment that loses it is dropped.
    pub fn waits(self) -> bool {
        matches!(
            self,
            AlertKind::Permission | AlertKind::Other | AlertKind::Question | AlertKind::Idle | AlertKind::Elicitation
        )
    }

    /// Read `k`; a missing value (older hooks, the legacy script) is a
    /// permission request.
    pub fn parse(s: Option<&str>) -> Self {
        match s {
            None | Some("permission") => AlertKind::Permission,
            Some("question") => AlertKind::Question,
            Some("idle") => AlertKind::Idle,
            Some("elicitation") => AlertKind::Elicitation,
            Some("done") => AlertKind::Done,
            Some("cal") => AlertKind::Calendar,
            Some("calstart") => AlertKind::CalendarStart,
            Some("msg") => AlertKind::Message,
            Some(_) => AlertKind::Other,
        }
    }

    /// The kind for one hook input: `PreToolUse` is the question tool,
    /// `Notification` is told apart by its `notification_type`.
    pub fn from_hook(event: Option<&str>, notification_type: Option<&str>) -> Self {
        match (event, notification_type) {
            (Some("PreToolUse"), _) => AlertKind::Question,
            (Some("Stop"), _) => AlertKind::Done,
            (_, None | Some("permission_prompt")) => AlertKind::Permission,
            (_, Some("idle_prompt")) => AlertKind::Idle,
            (_, Some("elicitation_dialog")) => AlertKind::Elicitation,
            (_, Some(_)) => AlertKind::Other,
        }
    }

    /// The fixed line the device shows for this kind, in place of Claude
    /// Code's own (often long) notification text.
    pub fn device_text(self) -> &'static str {
        match self {
            AlertKind::Question => "Question for you",
            AlertKind::Permission | AlertKind::Other => "Permission needed!",
            AlertKind::Idle => "Waiting for you!",
            AlertKind::Elicitation => "MCP request!",
            AlertKind::Done => "Done!",
            AlertKind::Calendar => "Meeting soon!",
            AlertKind::CalendarStart => "Meeting started!",
            AlertKind::Message => "Message",
        }
    }
}

/// One alert read from the attention file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alert {
    /// Message folded to the device's glyphs (may be empty).
    pub message: String,
    /// Claude Code session id (may be empty).
    pub session: String,
    /// The project it's from (the session's git root, else its cwd), for
    /// the device's alert line (`pj`); may be empty.
    pub project: String,
    /// What the session waits for.
    pub kind: AlertKind,
    /// The file's mtime.
    pub at: SystemTime,
}

impl Alert {
    /// What the device shows: the kind's [`AlertKind::device_text`], or the
    /// test alert's own text.
    pub fn device_message(&self) -> &str {
        if (self.session == TEST_SESSION || self.kind == AlertKind::Message) && !self.message.is_empty() {
            &self.message
        } else {
            self.kind.device_text()
        }
    }
}

/// A character the device's fonts draw as is: ASCII, Russian Cyrillic and
/// the few marks its Cyrillic companions add (`tools/gen_cyr_fonts.sh`).
fn device_glyph(c: char) -> bool {
    c.is_ascii() || matches!(c, '\u{410}'..='\u{44F}' | 'Ё' | 'ё' | '«' | '»' | '—' | '№')
}

/// Fold to the device's glyphs (anything else loses its accents, or is
/// dropped), collapse whitespace, cap at `limit` UTF-8 bytes with a `...`
/// suffix, never splitting a character.
pub fn to_device_text(text: &str, limit: usize) -> String {
    // Per character, so "й" stays whole: NFKD would split it into "и" + breve.
    let mut folded = String::with_capacity(text.len());
    for c in text.chars() {
        if device_glyph(c) {
            folded.push(c);
        } else {
            folded.extend(c.nfkd().filter(char::is_ascii));
        }
    }
    let collapsed = folded.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.len() <= limit {
        return collapsed;
    }
    let mut end = limit.saturating_sub(3);
    while !collapsed.is_char_boundary(end) {
        end -= 1;
    }
    let mut cut = collapsed[..end].trim_end().to_owned();
    cut.push_str("...");
    cut
}

/// Reads and consumes the hook's files in one directory.
#[derive(Debug, Clone)]
pub struct AttentionInbox {
    dir: PathBuf,
}

impl AttentionInbox {
    /// Inbox over the daemon state dir.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        AttentionInbox { dir: dir.into() }
    }

    fn flag(&self) -> PathBuf {
        self.dir.join("attention")
    }

    /// Consume the attention file. `None` when absent or stale. The file is
    /// moved to a private name before it is read, so an alert the hook
    /// writes meanwhile lands in a new `attention` file for the next tick
    /// instead of being removed unread.
    pub fn take_alert(&self, now: SystemTime) -> Option<Alert> {
        let path = self.dir.join("attention.taking");
        fs::rename(self.flag(), &path).ok()?;
        let mtime = fs::metadata(&path).and_then(|m| m.modified()).ok();
        let raw = fs::read_to_string(&path).unwrap_or_default();
        let _ = fs::remove_file(&path);
        let mtime = mtime?;
        if now.duration_since(mtime).unwrap_or_default() >= ATTENTION_MAX_AGE {
            return None;
        }
        let v: serde_json::Value = serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null);
        let field = |k: &str| match v.get(k) {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Null) | None => String::new(),
            Some(other) => other.to_string(),
        };
        let kind = AlertKind::parse(v.get("k").and_then(|k| k.as_str()));
        Some(Alert {
            message: to_device_text(&field("m"), MESSAGE_LIMIT),
            session: field("s"),
            project: to_device_text(&field("p"), PROJECT_LIMIT),
            kind,
            at: mtime,
        })
    }

    /// Consume every `clear-<sid>` file: `{sid: mtime}`.
    pub fn take_clears(&self) -> HashMap<String, SystemTime> {
        let mut out = HashMap::new();
        let Ok(rd) = fs::read_dir(&self.dir) else { return out };
        for entry in rd.flatten() {
            let name = entry.file_name();
            let Some(sid) = name.to_str().and_then(|n| n.strip_prefix("clear-")) else { continue };
            let mtime = entry.metadata().and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH);
            if fs::remove_file(entry.path()).is_ok() {
                out.insert(sid.to_owned(), mtime);
            }
        }
        out
    }

    /// Drop leftovers (on connect).
    pub fn reset(&self) {
        let _ = fs::remove_file(self.flag());
        let _ = fs::remove_file(self.dir.join("attention.taking"));
        let _ = self.take_clears();
    }

    /// The directory this inbox reads.
    pub fn dir(&self) -> &Path {
        &self.dir
    }
}

/// What the next payload must say about attention.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AttentionState {
    /// The alert on (or headed to) the screen, kept to match its clear.
    pub alert: Option<Alert>,
    /// The alert hasn't been sent yet.
    pub alert_unsent: bool,
    /// It rings when sent. A wait coming back from the queue doesn't ring
    /// again: it already did when it went up.
    pub alert_beeps: bool,
    /// A clear for a shown alert hasn't been sent yet.
    pub clear_unsent: bool,
    /// Other sessions still waiting for you behind the one on screen
    /// ([`AlertKind::waits`]); the best ([`AlertKind::rank`], then the
    /// oldest) takes the screen when it frees up.
    pub queued: Vec<Alert>,
}

/// Fields to merge into the next payload.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AttentionFields {
    /// `b`: beep (only when the user opted in).
    pub beep: bool,
    /// `m`: message for the status line.
    pub message: Option<String>,
    /// `k`: what the alert is about ([`AlertKind::as_str`]).
    pub kind: Option<&'static str>,
    /// `pj`: the project it's from.
    pub project: Option<String>,
    /// `x`: end the alert.
    pub clear: bool,
}

/// `alert`'s session was cleared after it went up.
fn cleared(alert: &Alert, clears: &HashMap<String, SystemTime>) -> bool {
    !alert.session.is_empty() && clears.get(&alert.session).is_some_and(|t| *t > alert.at)
}

/// How long `alert` lives with nobody clearing it.
fn ttl(alert: &Alert) -> Duration {
    if alert.session == TEST_SESSION {
        TEST_ALERT_TTL
    } else if alert.kind == AlertKind::Done {
        DONE_ALERT_TTL
    } else {
        ALERT_TTL
    }
}

impl AttentionState {
    /// Fold one tick's files into the state.
    /// Returns whether a new alert went up (and is still up after the clears).
    pub fn ingest(&mut self, new_alert: Option<Alert>, clears: &HashMap<String, SystemTime>) -> bool {
        let mut raised = new_alert.is_some_and(|a| self.offer(a));
        // Waits answered while they queued just go; the one on screen ends
        // and the next waiting session takes its place.
        self.queued.retain(|q| !cleared(q, clears));
        if self.alert.as_ref().is_some_and(|cur| cleared(cur, clears)) {
            self.end_current(true);
            raised = false;
        }
        raised
    }

    /// A new alert wants the screen. The same session again only counts
    /// when it outranks what it said before (AskUserQuestion notifies
    /// twice); another session takes the screen unless it ranks lower, and
    /// a wait that loses it (either one) queues.
    fn offer(&mut self, a: Alert) -> bool {
        let Some(cur) = &self.alert else {
            self.show(a, true);
            return true;
        };
        let same = !cur.session.is_empty() && cur.session == a.session;
        if same {
            if a.kind.rank() <= cur.kind.rank() {
                return false;
            }
        } else if a.kind.rank() < cur.kind.rank() {
            if a.kind.waits() {
                self.enqueue(a);
            }
            return false;
        } else if let Some(old) = self.alert.take().filter(|o| o.kind.waits()) {
            self.enqueue(old);
        }
        self.show(a, true);
        true
    }

    fn enqueue(&mut self, a: Alert) {
        self.queued.retain(|q| q.session.is_empty() || q.session != a.session);
        self.queued.push(a);
    }

    fn show(&mut self, a: Alert, beeps: bool) {
        self.alert = Some(a);
        self.alert_unsent = true;
        self.alert_beeps = beeps;
        self.clear_unsent = false;
    }

    /// The alert on screen is over: the best queued wait takes its place
    /// (a new `m` replaces the device's alert, no clear needed), else a
    /// clear goes out when `tell_device` and it was shown.
    fn end_current(&mut self, tell_device: bool) {
        let shown = !self.alert_unsent;
        self.alert = None;
        self.alert_unsent = false;
        let best = (0..self.queued.len()).max_by(|&i, &j| {
            let (a, b) = (&self.queued[i], &self.queued[j]);
            a.kind.rank().cmp(&b.kind.rank()).then(b.at.cmp(&a.at))
        });
        match best {
            Some(i) => {
                let next = self.queued.remove(i);
                self.show(next, false);
            }
            None => self.clear_unsent = tell_device && shown,
        }
    }

    /// End an alert that has been up for its time to live ([`ALERT_TTL`],
    /// [`TEST_ALERT_TTL`], [`DONE_ALERT_TTL`]), and drop queued waits that
    /// outlived theirs. A shown one gets a clear so the device drops it too,
    /// unless a queued wait takes its place. Returns whether it ended.
    pub fn expire(&mut self, now: SystemTime) -> bool {
        let alive = |a: &Alert| now.duration_since(a.at).unwrap_or_default() < ttl(a);
        self.queued.retain(alive);
        let Some(cur) = &self.alert else { return false };
        if alive(cur) {
            return false;
        }
        self.end_current(true);
        true
    }

    /// The device ended the alert itself: forget it without sending a clear
    /// (a queued wait still comes up).
    pub fn dismissed(&mut self) {
        if !self.alert_unsent {
            self.end_current(false);
        }
    }

    /// Something attention-related is waiting to be sent.
    pub fn pending(&self) -> bool {
        self.alert_unsent || self.clear_unsent
    }

    /// An alert is currently up (sent or not).
    pub fn active(&self) -> bool {
        self.alert.is_some()
    }

    /// The fields for the next payload; `beep_on` is the `beep` setting.
    pub fn fields(&self, beep_on: bool) -> AttentionFields {
        let mut f = AttentionFields::default();
        if self.alert_unsent {
            if let Some(a) = &self.alert {
                f.beep = beep_on && self.alert_beeps;
                f.message = Some(a.device_message().to_owned());
                f.kind = Some(a.kind.as_str());
                f.project = (!a.project.is_empty()).then(|| a.project.clone());
            }
        }
        f.clear = self.clear_unsent;
        f
    }

    /// The payload carrying [`AttentionState::fields`] was written.
    pub fn mark_sent(&mut self) {
        self.alert_unsent = false;
        self.clear_unsent = false;
    }

    /// Forget everything (new connection).
    pub fn reset(&mut self) {
        *self = AttentionState::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("cm-attn-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn set_mtime(p: &Path, t: SystemTime) {
        File::options().write(true).open(p).unwrap().set_modified(t).unwrap();
    }

    #[test]
    fn attention_flag() {
        let d = tmpdir("flag");
        let inbox = AttentionInbox::new(&d);
        let now = SystemTime::now();
        assert!(inbox.take_alert(now).is_none());

        File::create(d.join("attention")).unwrap();
        assert_eq!(inbox.take_alert(SystemTime::now()).unwrap().message, "");
        assert!(!d.join("attention").exists());

        fs::write(d.join("attention"), r#"{"m": "Você  aprova\na ação?", "s": "abc"}"#).unwrap();
        let a = inbox.take_alert(SystemTime::now()).unwrap();
        assert_eq!((a.message.as_str(), a.session.as_str()), ("Voce aprova a acao?", "abc"));
        assert_eq!(to_device_text(&"x".repeat(200), 120), "x".repeat(117) + "...");
        assert_eq!(to_device_text("Нужно  разрешение: «Йод» — ок", 120), "Нужно разрешение: «Йод» — ок");
        let long = to_device_text(&"ж".repeat(100), 120);
        assert!(long.len() <= 120 && long.ends_with("..."), "capped on a char boundary");

        File::create(d.join("attention")).unwrap();
        set_mtime(&d.join("attention"), SystemTime::now() - ATTENTION_MAX_AGE - Duration::from_secs(1));
        assert!(inbox.take_alert(SystemTime::now()).is_none());
        assert!(!d.join("attention").exists());

        fs::write(d.join("attention"), "not json").unwrap();
        assert_eq!(inbox.take_alert(SystemTime::now()).unwrap().session, "");
        assert!(!d.join("attention.taking").exists(), "nothing left behind");
        let _ = fs::remove_dir_all(&d);
    }

    fn raised(session: &str, at: SystemTime) -> Alert {
        Alert {
            message: "Approve?".into(),
            session: session.into(),
            project: String::new(),
            kind: AlertKind::Permission,
            at,
        }
    }

    #[test]
    fn shown_alert_expires_with_a_clear() {
        let t0 = SystemTime::now();
        let mut s = AttentionState::default();
        s.ingest(Some(raised("abc", t0)), &HashMap::new());
        s.mark_sent();
        assert!(!s.expire(t0 + ALERT_TTL - Duration::from_secs(1)));
        assert!(s.active());
        assert!(s.expire(t0 + ALERT_TTL));
        assert!(!s.active());
        assert!(s.fields(false).clear, "the device drops it too");
        s.mark_sent();
        assert!(!s.pending());
    }

    #[test]
    fn unsent_alert_expires_quietly() {
        let t0 = SystemTime::now();
        let mut s = AttentionState::default();
        s.ingest(Some(raised("abc", t0)), &HashMap::new());
        assert!(s.expire(t0 + ALERT_TTL));
        assert!(!s.pending(), "never shown: nothing to send");
    }

    #[test]
    fn test_alert_expires_after_thirty_seconds() {
        let t0 = SystemTime::now();
        let mut s = AttentionState::default();
        s.ingest(Some(raised(TEST_SESSION, t0)), &HashMap::new());
        s.mark_sent();
        assert!(!s.expire(t0 + Duration::from_secs(29)));
        assert!(s.expire(t0 + TEST_ALERT_TTL));
        assert!(s.fields(false).clear);
    }

    #[test]
    fn device_dismissal_ends_a_shown_alert_without_a_clear() {
        let t0 = SystemTime::now();
        let mut s = AttentionState::default();
        s.ingest(Some(raised("abc", t0)), &HashMap::new());
        // Not on the device yet: a dismissal is about an older alert.
        s.dismissed();
        assert!(s.active() && s.alert_unsent);
        s.mark_sent();
        s.dismissed();
        assert!(!s.active());
        assert!(!s.pending(), "the device already dropped it");
        // The same session alerting again is a new alert.
        s.ingest(Some(raised("abc", t0 + Duration::from_secs(5))), &HashMap::new());
        assert!(s.active() && s.alert_unsent);
    }

    // Ported from test_attention.py::test_clears.
    #[test]
    fn clears() {
        let d = tmpdir("clears");
        let inbox = AttentionInbox::new(&d);
        File::create(d.join("clear-abc")).unwrap();
        File::create(d.join("unrelated")).unwrap();
        let c = inbox.take_clears();
        assert_eq!(c.keys().collect::<Vec<_>>(), vec!["abc"]);
        assert!(inbox.take_clears().is_empty());
        assert!(d.join("unrelated").exists());
        let _ = fs::remove_dir_all(&d);
    }

    fn alert(sid: &str, msg: &str, at: SystemTime) -> Alert {
        Alert { message: msg.into(), session: sid.into(), project: String::new(), kind: AlertKind::Permission, at }
    }

    fn kinded(sid: &str, kind: AlertKind, at: SystemTime) -> Alert {
        Alert { message: String::new(), session: sid.into(), project: format!("p-{sid}"), kind, at }
    }

    #[test]
    fn a_moment_never_takes_the_screen_from_a_wait() {
        let t0 = SystemTime::now();
        let none = HashMap::new();
        let mut st = AttentionState::default();
        st.ingest(Some(kinded("a", AlertKind::Permission, t0)), &none);
        st.mark_sent();
        assert!(!st.ingest(Some(kinded("b", AlertKind::Done, t0)), &none), "done from b is dropped");
        assert_eq!(st.alert.as_ref().unwrap().session, "a");
        assert!(!st.pending() && st.queued.is_empty());
    }

    #[test]
    fn a_wait_that_loses_the_screen_comes_back() {
        let t0 = SystemTime::now();
        let none = HashMap::new();
        let mut st = AttentionState::default();
        st.ingest(Some(kinded("a", AlertKind::Idle, t0)), &none);
        st.mark_sent();
        assert!(st.ingest(Some(kinded("b", AlertKind::Permission, t0)), &none), "permission outranks idle");
        let f = st.fields(true);
        assert_eq!((f.kind, f.project.as_deref(), f.beep), (Some("permission"), Some("p-b"), true));
        st.mark_sent();
        assert_eq!(st.queued.len(), 1, "a waits behind it");
        let b_done: HashMap<_, _> = [("b".to_owned(), t0 + Duration::from_secs(1))].into();
        st.ingest(None, &b_done);
        let f = st.fields(true);
        assert_eq!((f.kind, f.project.as_deref()), (Some("idle"), Some("p-a")), "a is back");
        assert!(!f.beep && !f.clear, "without ringing again, and no clear in between");
        st.mark_sent();
        let a_done: HashMap<_, _> = [("a".to_owned(), t0 + Duration::from_secs(2))].into();
        st.ingest(None, &a_done);
        assert!(st.alert.is_none() && st.fields(true).clear, "the last one clears");
    }

    #[test]
    fn queued_waits_answered_meanwhile_just_go() {
        let t0 = SystemTime::now();
        let none = HashMap::new();
        let mut st = AttentionState::default();
        st.ingest(Some(kinded("a", AlertKind::Permission, t0)), &none);
        st.mark_sent();
        st.ingest(Some(kinded("b", AlertKind::Idle, t0)), &none);
        assert_eq!(st.queued.len(), 1, "a lower-ranked wait queues");
        let b: HashMap<_, _> = [("b".to_owned(), t0 + Duration::from_secs(1))].into();
        st.ingest(None, &b);
        assert!(st.queued.is_empty());
        let a: HashMap<_, _> = [("a".to_owned(), t0 + Duration::from_secs(2))].into();
        st.ingest(None, &a);
        assert!(st.alert.is_none() && st.clear_unsent);
    }

    #[test]
    fn the_same_session_escalates_but_never_repeats() {
        let t0 = SystemTime::now();
        let none = HashMap::new();
        let mut st = AttentionState::default();
        st.ingest(Some(kinded("a", AlertKind::Idle, t0)), &none);
        st.mark_sent();
        assert!(!st.ingest(Some(kinded("a", AlertKind::Idle, t0)), &none));
        assert!(st.ingest(Some(kinded("a", AlertKind::Permission, t0)), &none));
        assert!(st.queued.is_empty(), "its own lesser wait doesn't queue");
    }

    #[test]
    fn done_is_news_for_half_a_minute() {
        let t0 = SystemTime::now();
        let mut st = AttentionState::default();
        st.ingest(Some(kinded("a", AlertKind::Done, t0)), &HashMap::new());
        st.mark_sent();
        assert!(!st.expire(t0 + DONE_ALERT_TTL - Duration::from_secs(1)));
        assert!(st.expire(t0 + DONE_ALERT_TTL));
        assert!(st.clear_unsent);
    }

    #[test]
    fn session_matching_like_bash() {
        let t0 = SystemTime::now();
        let none = HashMap::new();
        let mut st = AttentionState::default();

        st.ingest(Some(alert("abc", "Voce aprova a acao?", t0)), &none);
        assert!(st.alert_unsent);
        assert_eq!(
            st.fields(false),
            AttentionFields {
                beep: false,
                message: Some("Permission needed!".into()),
                kind: Some("permission"),
                project: None,
                clear: false
            }
        );
        assert!(st.fields(true).beep);
        st.mark_sent();

        // Repeat from the same session is ignored.
        st.ingest(Some(alert("abc", "Claude needs your permission", t0)), &none);
        assert!(!st.alert_unsent);
        assert_eq!(st.alert.as_ref().unwrap().message, "Voce aprova a acao?");

        // Clear for another session is ignored.
        let other: HashMap<_, _> = [("other".to_owned(), t0 + Duration::from_secs(2))].into();
        st.ingest(None, &other);
        assert!(st.alert.is_some() && !st.clear_unsent);

        // Clear for the alerting session, newer: hide.
        let same: HashMap<_, _> = [("abc".to_owned(), t0 + Duration::from_secs(2))].into();
        st.ingest(None, &same);
        assert!(st.alert.is_none() && st.clear_unsent);
        assert_eq!(
            st.fields(true),
            AttentionFields { beep: false, message: None, kind: None, project: None, clear: true }
        );
        st.mark_sent();

        // Same second, later by 0.5 s: still clears.
        let t1 = SystemTime::UNIX_EPOCH + Duration::from_nanos(1_760_000_000_100_000_000);
        st.ingest(Some(alert("q", "quick", t1)), &none);
        st.mark_sent();
        let q: HashMap<_, _> = [("q".to_owned(), t1 + Duration::from_millis(500))].into();
        st.ingest(None, &q);
        assert!(st.alert.is_none() && st.clear_unsent);

        // An older clear doesn't end a newer alert.
        st.mark_sent();
        st.ingest(Some(alert("z", "", t1)), &none);
        let old: HashMap<_, _> = [("z".to_owned(), t1 - Duration::from_secs(1))].into();
        st.ingest(None, &old);
        assert!(st.alert.is_some());
    }

    #[test]
    fn clear_before_send_sends_nothing() {
        let t0 = SystemTime::now();
        let mut st = AttentionState::default();
        st.ingest(Some(alert("s", "hi", t0)), &HashMap::new());
        let c: HashMap<_, _> = [("s".to_owned(), t0 + Duration::from_secs(1))].into();
        st.ingest(None, &c);
        assert!(!st.pending(), "never shown, nothing to clear");
    }

    #[test]
    fn device_shows_a_fixed_line_per_kind() {
        let at = SystemTime::now();
        let line = |kind, session: &str, message: &str| {
            let mut st = AttentionState::default();
            st.ingest(
                Some(Alert { message: message.into(), session: session.into(), project: String::new(), kind, at }),
                &HashMap::new(),
            );
            st.fields(false).message.unwrap()
        };
        let hook_text = "Claude needs your permission to use Bash";
        assert_eq!(line(AlertKind::Question, "a", "Which file?"), "Question for you");
        assert_eq!(line(AlertKind::Permission, "a", hook_text), "Permission needed!");
        assert_eq!(line(AlertKind::Other, "a", hook_text), "Permission needed!");
        assert_eq!(line(AlertKind::Idle, "a", "Claude is waiting for your input"), "Waiting for you!");
        assert_eq!(line(AlertKind::Elicitation, "a", hook_text), "MCP request!");
        assert_eq!(line(AlertKind::Permission, "a", ""), "Permission needed!", "a bare touch still says why");
        assert_eq!(line(AlertKind::Permission, TEST_SESSION, "Clawdmeter test alert"), "Clawdmeter test alert");
    }

    #[test]
    fn alert_kinds_from_hook_input() {
        let k = AlertKind::from_hook;
        assert_eq!(k(Some("PreToolUse"), None), AlertKind::Question);
        assert_eq!(k(Some("Notification"), None), AlertKind::Permission);
        assert_eq!(k(Some("Notification"), Some("permission_prompt")), AlertKind::Permission);
        assert_eq!(k(Some("Notification"), Some("idle_prompt")), AlertKind::Idle);
        assert_eq!(k(Some("Notification"), Some("elicitation_dialog")), AlertKind::Elicitation);
        assert_eq!(k(Some("Notification"), Some("auth_success")), AlertKind::Other);
        for kind in [AlertKind::Permission, AlertKind::Question, AlertKind::Idle, AlertKind::Elicitation] {
            assert_eq!(AlertKind::parse(Some(kind.as_str())), kind);
        }
        assert_eq!(AlertKind::parse(None), AlertKind::Permission, "legacy files have no kind");
    }
}
