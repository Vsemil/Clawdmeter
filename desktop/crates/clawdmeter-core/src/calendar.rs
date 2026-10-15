//! Meeting reminders: the `calendar` setting names a source, the daemon
//! refetches it every [`FETCH_EVERY`] seconds, and a meeting crossing a
//! `cal_remind` threshold (15 and 5 minutes by default) goes to the device
//! as a `cal` alert, "HH:MM title" in its project slot; within
//! [`STARTED_GRACE`] after the start, a `calstart` one.
//!
//! Sources:
//!
//! * `eventkit`: the macOS system calendar, through the signed `calnext`
//!   helper (`tools/calnext/`; its JSON is `[{"start": epoch, "title"}]`).
//!   A bare binary can't show the calendar permission prompt; the helper
//!   embeds the Info.plist that can, and keeps the grant across runs;
//! * an `https://` ICS feed, or an absolute path to a `.ics` file: the
//!   VEVENTs in the next [`HORIZON`] seconds, recurring ones expanded
//!   (RRULE, EXDATE), all-day and cancelled ones dropped.
//!
//! Only the smallest threshold already crossed fires (an app started seven
//! minutes before a meeting reminds once, at 5, not at 15 too), and a
//! reminder counts as sent only once it reached the device, so a dropped
//! link doesn't eat it.

use std::collections::HashSet;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::{Duration, Instant};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

/// Refetch the source this often (s).
pub const FETCH_EVERY: i64 = 300;
/// Look this far ahead (s).
pub const HORIZON: i64 = 26 * 3600;
/// "Meeting started" goes out within this long after the start (s); an app
/// that was down through all of it stays quiet, a late one is just noise.
pub const STARTED_GRACE: i64 = 120;
/// Default reminder thresholds (minutes before the start).
pub const DEFAULT_THRESHOLDS: [u32; 2] = [15, 5];

/// One meeting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meeting {
    /// Start, unix seconds.
    pub start: i64,
    /// Title, whitespace collapsed.
    pub title: String,
}

/// A reminder due now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reminder {
    /// The meeting's start.
    pub start: i64,
    /// Minutes before the start this reminder is for; 0 = it has started.
    pub threshold: u32,
    /// "HH:MM title" for the device.
    pub text: String,
}

/// Parse `cal_remind`: minutes, comma or space separated; nothing valid
/// gives [`DEFAULT_THRESHOLDS`].
pub fn parse_thresholds(v: Option<&str>) -> Vec<u32> {
    let mut t: Vec<u32> =
        v.unwrap_or("").split([',', ' ']).filter_map(|w| w.trim().parse().ok()).filter(|&m| m > 0).collect();
    t.sort_unstable();
    t.dedup();
    if t.is_empty() {
        DEFAULT_THRESHOLDS.iter().rev().copied().collect()
    } else {
        t
    }
}

/// The meetings and what has been sent about them.
#[derive(Debug, Clone, Default)]
pub struct Calendar {
    meetings: Vec<Meeting>,
    sent: HashSet<(i64, u32)>,
}

impl Calendar {
    /// A fresh fetch. Sent marks older than an hour are dropped.
    pub fn set_meetings(&mut self, mut meetings: Vec<Meeting>, now: i64) {
        meetings.sort_by_key(|m| m.start);
        meetings.dedup();
        self.meetings = meetings;
        self.sent.retain(|(start, _)| *start > now - 3600);
    }

    /// The meetings as last fetched.
    pub fn meetings(&self) -> &[Meeting] {
        &self.meetings
    }

    /// The first reminder due at `now` that hasn't been sent. `thresholds`
    /// in minutes; the time is shown at `utc_offset_secs`, 12-hour when
    /// `h12`.
    pub fn due(&self, now: i64, thresholds: &[u32], utc_offset_secs: i64, h12: bool) -> Option<Reminder> {
        let widest = thresholds.iter().copied().max().unwrap_or(0) as i64 * 60;
        for m in &self.meetings {
            if m.start - widest > now {
                break; // sorted: everything after is further off
            }
            let threshold = if m.start <= now {
                if now >= m.start + STARTED_GRACE {
                    continue;
                }
                0
            } else {
                // The closest threshold already crossed; the larger ones are stale.
                let Some(t) = thresholds.iter().copied().filter(|&t| m.start - t as i64 * 60 <= now).min() else {
                    continue;
                };
                t
            };
            if self.sent.contains(&(m.start, threshold)) {
                continue;
            }
            return Some(Reminder {
                start: m.start,
                threshold,
                text: format!("{} {}", clock(m.start, utc_offset_secs, h12), m.title),
            });
        }
        None
    }

    /// Today's next meeting, "HH:MM title": the first that starts after
    /// `now` on the same local day. The device shows it in place of its title.
    pub fn next_today(&self, now: i64, utc_offset_secs: i64, h12: bool) -> Option<String> {
        let day = |t: i64| (t + utc_offset_secs).div_euclid(86_400);
        self.meetings
            .iter()
            .find(|m| m.start > now)
            .filter(|m| day(m.start) == day(now))
            .map(|m| format!("{} {}", clock(m.start, utc_offset_secs, h12), m.title))
    }

    /// The reminder reached the device.
    pub fn mark_sent(&mut self, r: &Reminder) {
        self.sent.insert((r.start, r.threshold));
    }
}

/// "09:30" (or "9:30 AM") for a unix time at `utc_offset_secs`.
fn clock(at: i64, utc_offset_secs: i64, h12: bool) -> String {
    let t = DateTime::<Utc>::from_timestamp(at + utc_offset_secs, 0).unwrap_or_default().naive_utc();
    if h12 {
        t.format("%-I:%M %p").to_string()
    } else {
        t.format("%H:%M").to_string()
    }
}

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The `calnext` helper's output: `[{"start": epoch, "title": "..."}]`.
pub fn parse_calnext(json: &str) -> Result<Vec<Meeting>, String> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("calnext: {e}"))?;
    let list = v.as_array().ok_or("calnext: not a list")?;
    Ok(list
        .iter()
        .filter_map(|e| {
            let start = e.get("start")?.as_f64()? as i64;
            let title = collapse(e.get("title").and_then(|t| t.as_str()).unwrap_or(""));
            Some(Meeting { start, title })
        })
        .collect())
}

/// The `calnext` helper may wait this long: its first run shows the
/// calendar permission prompt (it gives up after 120 s itself).
pub const HELPER_TIMEOUT: Duration = Duration::from_secs(130);

/// Fetch `source`'s meetings (blocking: run it off the daemon's tick).
pub fn fetch(source: &str, helper: &Path, now: i64) -> Result<Vec<Meeting>, String> {
    if source == "eventkit" {
        return parse_calnext(&run_helper(helper)?);
    }
    let text = if source.starts_with("https://") || source.starts_with("http://") {
        let agent: ureq::Agent =
            ureq::Agent::config_builder().timeout_global(Some(crate::oauth::HTTP_TIMEOUT)).build().into();
        agent
            .get(source)
            .header("User-Agent", &crate::oauth::user_agent())
            .call()
            .map_err(|e| format!("calendar feed: {e}"))?
            .body_mut()
            .read_to_string()
            .map_err(|e| format!("calendar feed: {e}"))?
    } else {
        std::fs::read_to_string(source).map_err(|e| format!("{source}: {e}"))?
    };
    Ok(parse_ics(&text, now))
}

/// Run the helper, kill it past [`HELPER_TIMEOUT`].
fn run_helper(helper: &Path) -> Result<String, String> {
    use std::io::Read as _;
    let mut child = Command::new(helper)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", helper.display()))?;
    let deadline = Instant::now() + HELPER_TIMEOUT;
    let status = loop {
        if let Some(st) = child.try_wait().map_err(|e| e.to_string())? {
            break st;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("calnext timed out".into());
        }
        std::thread::sleep(Duration::from_millis(200));
    };
    let (mut out, mut err) = (String::new(), String::new());
    if let Some(mut o) = child.stdout.take() {
        let _ = o.read_to_string(&mut out);
    }
    if let Some(mut e) = child.stderr.take() {
        let _ = e.read_to_string(&mut err);
    }
    if !status.success() {
        let err = err.trim();
        return Err(if err.is_empty() { format!("calnext exited with {status}") } else { format!("calnext: {err}") });
    }
    Ok(out)
}

// ─── ICS ─────────────────────────────────────────────────────────────────────

/// One content line: name, parameters, value.
struct Prop<'a> {
    name: String,
    params: Vec<(String, &'a str)>,
    value: &'a str,
}

fn prop(line: &str) -> Option<Prop<'_>> {
    let colon = line.find(':')?;
    let (head, value) = (&line[..colon], &line[colon + 1..]);
    let mut parts = head.split(';');
    let name = parts.next()?.to_ascii_uppercase();
    let params = parts
        .filter_map(|p| {
            let (k, v) = p.split_once('=')?;
            Some((k.to_ascii_uppercase(), v.trim_matches('"')))
        })
        .collect();
    Some(Prop { name, params, value })
}

impl Prop<'_> {
    fn param(&self, k: &str) -> Option<&str> {
        self.params.iter().find(|(n, _)| n == k).map(|(_, v)| *v)
    }
}

/// Unfold continuation lines (a leading space or tab continues the last).
fn unfold(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for raw in text.lines() {
        let raw = raw.trim_end_matches('\r');
        if let Some(rest) = raw.strip_prefix([' ', '\t']) {
            if let Some(last) = out.last_mut() {
                last.push_str(rest);
                continue;
            }
        }
        out.push(raw.to_owned());
    }
    out
}

/// The zone a TZID names: an IANA name, a few common Windows names, else
/// the local zone (a floating time).
fn zone(tzid: Option<&str>) -> rrule::Tz {
    let Some(id) = tzid else { return rrule::Tz::LOCAL };
    if let Ok(tz) = chrono_tz::Tz::from_str(id) {
        return rrule::Tz::Tz(tz);
    }
    let iana = match id {
        "Russian Standard Time" => "Europe/Moscow",
        "UTC" | "Coordinated Universal Time" | "GMT Standard Time" => "Europe/London",
        "W. Europe Standard Time" => "Europe/Berlin",
        "Central European Standard Time" => "Europe/Warsaw",
        "E. Europe Standard Time" => "Europe/Chisinau",
        "FLE Standard Time" => "Europe/Kiev",
        "Pacific Standard Time" => "America/Los_Angeles",
        "Eastern Standard Time" => "America/New_York",
        "Central Standard Time" => "America/Chicago",
        "Ekaterinburg Standard Time" => "Asia/Yekaterinburg",
        "N. Central Asia Standard Time" => "Asia/Novosibirsk",
        _ => return rrule::Tz::LOCAL,
    };
    chrono_tz::Tz::from_str(iana).map(rrule::Tz::Tz).unwrap_or(rrule::Tz::LOCAL)
}

/// A DATE-TIME property in its zone. `None` for an all-day DATE.
fn date_time(p: &Prop) -> Option<DateTime<rrule::Tz>> {
    if p.param("VALUE") == Some("DATE") || p.value.len() == 8 {
        return None;
    }
    let (v, utc) = match p.value.strip_suffix('Z') {
        Some(v) => (v, true),
        None => (p.value, false),
    };
    let naive = NaiveDateTime::parse_from_str(v, "%Y%m%dT%H%M%S").ok()?;
    let tz = if utc { rrule::Tz::UTC } else { zone(p.param("TZID")) };
    tz.from_local_datetime(&naive).earliest()
}

/// The meetings of an ICS calendar starting in `[now, now + HORIZON]`.
pub fn parse_ics(text: &str, now: i64) -> Vec<Meeting> {
    let lines = unfold(text);
    let mut out = Vec::new();
    let mut inside = false;
    let mut ev: Vec<&str> = Vec::new();
    for line in &lines {
        match line.as_str() {
            "BEGIN:VEVENT" => {
                inside = true;
                ev.clear();
            }
            "END:VEVENT" => {
                inside = false;
                out.extend(expand(&ev, now));
            }
            l if inside => ev.push(l),
            _ => {}
        }
    }
    out
}

fn expand(lines: &[&str], now: i64) -> Vec<Meeting> {
    let props: Vec<Prop> = lines.iter().filter_map(|l| prop(l)).collect();
    let get = |n: &str| props.iter().find(|p| p.name == n);
    if get("STATUS").is_some_and(|p| p.value.eq_ignore_ascii_case("CANCELLED")) {
        return Vec::new();
    }
    let Some(start) = get("DTSTART").and_then(date_time) else { return Vec::new() };
    let title = collapse(&unescape(get("SUMMARY").map_or("", |p| p.value)));
    let (from, to) = (now, now + HORIZON);
    let starts: Vec<i64> = match get("RRULE") {
        None => vec![start.timestamp()],
        Some(r) => {
            let Ok(rule) = rrule::RRule::<rrule::Unvalidated>::from_str(r.value) else { return Vec::new() };
            let Ok(mut set) = rule.build(start) else { return Vec::new() };
            for ex in props.iter().filter(|p| p.name == "EXDATE") {
                for v in ex.value.split(',') {
                    let one = Prop { name: ex.name.clone(), params: ex.params.clone(), value: v };
                    if let Some(d) = date_time(&one) {
                        set = set.exdate(d);
                    }
                }
            }
            let lo = rrule::Tz::UTC.timestamp_opt(from - 1, 0).single();
            let hi = rrule::Tz::UTC.timestamp_opt(to + 1, 0).single();
            let (Some(lo), Some(hi)) = (lo, hi) else { return Vec::new() };
            set.after(lo).before(hi).all(200).dates.iter().map(DateTime::timestamp).collect()
        }
    };
    starts.into_iter().filter(|&s| s >= from && s <= to).map(|start| Meeting { start, title: title.clone() }).collect()
}

/// ICS text escapes: `\,` `\;` `\n` `\\`.
fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('n' | 'N') => out.push(' '),
            Some(o) => out.push(o),
            None => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cal(starts: &[(i64, &str)]) -> Calendar {
        let mut c = Calendar::default();
        c.set_meetings(starts.iter().map(|&(start, t)| Meeting { start, title: t.into() }).collect(), 0);
        c
    }

    const T: i64 = 1_800_000_000; // 2027-01-15 08:00:00 UTC

    #[test]
    fn thresholds() {
        assert_eq!(parse_thresholds(None), vec![5, 15]);
        assert_eq!(parse_thresholds(Some("10, 2 ,x,0")), vec![2, 10]);
    }

    #[test]
    fn the_closest_crossed_threshold_fires_once_then_the_start() {
        let mut c = cal(&[(T + 3600, "Standup")]);
        let th = parse_thresholds(None);
        assert_eq!(c.due(T, &th, 0, false), None, "an hour off: nothing");
        let r = c.due(T + 3600 - 7 * 60, &th, 0, false).unwrap();
        assert_eq!((r.threshold, r.text.as_str()), (15, "09:00 Standup"));
        c.mark_sent(&r);
        assert_eq!(c.due(T + 3600 - 6 * 60, &th, 0, false), None, "sent");
        let r = c.due(T + 3600 - 4 * 60, &th, 0, false).unwrap();
        assert_eq!(r.threshold, 5);
        c.mark_sent(&r);
        let r = c.due(T + 3600 + 30, &th, 3 * 3600, true).unwrap();
        assert_eq!((r.threshold, r.text.as_str()), (0, "12:00 PM Standup"));
        c.mark_sent(&r);
        assert_eq!(c.due(T + 3600 + STARTED_GRACE, &th, 0, false), None, "late: quiet");
    }

    #[test]
    fn next_meeting_is_today_only() {
        let c = cal(&[(T - 600, "Past"), (T + 3600, "Sync"), (T + 20 * 3600, "Tomorrow")]);
        assert_eq!(c.next_today(T, 0, false).as_deref(), Some("09:00 Sync"));
        assert_eq!(c.next_today(T + 3601, 0, false), None, "only tomorrow's is left");
    }

    #[test]
    fn started_late_skips_straight_to_five() {
        let c = cal(&[(T + 4 * 60, "Sync")]);
        assert_eq!(c.due(T, &parse_thresholds(None), 0, false).unwrap().threshold, 5);
    }

    #[test]
    fn calnext_json() {
        let m =
            parse_calnext(r#"[{"start": 1800000000.0, "title": "  Plan \n ning "}, {"title": "no start"}]"#).unwrap();
        assert_eq!(m, vec![Meeting { start: T, title: "Plan ning".into() }]);
        assert!(parse_calnext("nope").is_err());
    }

    #[test]
    fn ics_events_recurrence_and_exclusions() {
        let ics = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nDTSTART:20270115T090000Z\r\nSUMMARY:One\\, off\r\nEND:VEVENT\r\n\
                   BEGIN:VEVENT\r\nDTSTART;TZID=Europe/Moscow:20270101T100000\r\nRRULE:FREQ=DAILY\r\n\
                   EXDATE;TZID=Europe/Moscow:20270116T100000\r\nSUMMARY:Daily st\r\n and-up\r\nEND:VEVENT\r\n\
                   BEGIN:VEVENT\r\nDTSTART;VALUE=DATE:20270115\r\nSUMMARY:All day\r\nEND:VEVENT\r\n\
                   BEGIN:VEVENT\r\nDTSTART:20270115T100000Z\r\nSTATUS:CANCELLED\r\nSUMMARY:Gone\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let mut m = parse_ics(ics, T);
        m.sort_by_key(|x| x.start);
        let got: Vec<(i64, &str)> = m.iter().map(|x| (x.start - T, x.title.as_str())).collect();
        // 09:00Z one-off; the daily 10:00 Moscow (07:00Z) on the 16th is excluded,
        // so the next is the 17th... beyond the 26 h horizon from 08:00Z on the 15th.
        assert_eq!(got, vec![(3600, "One, off")]);
        let m = parse_ics(ics, T - 2 * 3600); // 06:00Z: today's 07:00Z daily is ahead
        assert!(m.iter().any(|x| x.start == T - 3600 && x.title == "Daily stand-up"));
    }
}
