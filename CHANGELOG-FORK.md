# What this fork changes

A fork of [HermannBjorgvin/Clawdmeter](https://github.com/HermannBjorgvin/Clawdmeter)
that turns the device from a passive usage meter into a desk-side Claude Code
**pager** — typed sound/screen alerts driven by Claude Code hooks, meeting
reminders, a live activity indicator, and a runtime-switchable RU/EN
interface. Verified on the Waveshare ESP32-S3-Touch-AMOLED-2.16 with the
macOS daemon.

## Localization

- Every firmware string lives in a string table
  (`firmware/src/lang.{h,cpp}`) with Russian and English sets, including all
  ~90 whimsical status verbs ("Clauding…", "Combobulating…").
- The language switches via `lang = ru|en` in the daemon config
  (`~/.config/claude-usage-monitor/config`); the choice persists on the
  device (NVS), so even the pairing screen speaks the right language from
  the next boot. Without the option the firmware behaves exactly like
  upstream (English).
- Cyrillic in the brand fonts via LVGL fallbacks: Golos Text → Styrene,
  PT Serif → Tiempos; the mono font regenerated from DejaVu Sans Mono.

## Claude Code alerts

- Claude Code hooks (`Notification` / `Stop` / `UserPromptSubmit` /
  `PostToolUse` → `tools/claude-attention-hook.sh`) classify events, the
  daemon forwards them, and the firmware shows a typed screen with its own
  melody (sine synthesis through the ES8311, no PCM clips):
  - **"Awaiting your reply"** — the session is blocked on your input;
  - **"Permission needed"** — a tool-permission prompt;
  - **"Done!"** — a turn finished (turns shorter than 45 s stay silent);
  - **"Limit near!"** — crossing 80 % / 95 % of the session window;
  - **"Limits refreshed!"** — the session window reset (a wink).
- The alert header shows the project name (the git-root basename), wrapped
  over up to two lines. Start typing on the Mac and the screen dismisses
  itself.
- Each alert draws its creature at random from a small cast rather than
  always the same one (never twice in a row), so the screen still reads as
  a reaction when the same event fires all day: pointing, waving or the
  magnifier when Claude is blocked on you; dancing, the trumpet or the
  cloud when a turn finishes; the laptop when the limit is close.
- **Overlapping events keep their meaning.** Alerts used to share one flag
  file and one screen slot, so anything landing inside the daemon's 5 s tick
  overwrote what was already there — and the loser was gone for good. Three
  changes remove that:
  - the hooks and the MCP server drop one file per event into a spool; the
    daemon forwards the most important one per beat and leaves the rest for
    the following ticks (`ATTN_PRIORITY`: perm > input > calstart > cal >
    done > clear; the firmware ranks the same order in its per-type table);
  - `clear` is addressed. Typing in one session used to dismiss the
    permission prompt another session was blocked on — and nothing raised it
    again. It now carries the project it belongs to. Events with no project
    of their own — an MCP message, a meeting reminder, the local limit
    flashes — address themselves as "" and stay dismissable by anyone;
  - the firmware separates **states** (INPUT/PERM — a session stays blocked
    until it's answered) from **flashes** (done, limit, reset, calendar — a
    moment that passed). A flash plays on top of a state and hands the screen
    back when it expires, so "Done!" from one project can no longer swallow
    another's prompt. A tap does the same, one alert at a time.
  The melody now follows the same decision as the screen: `ui_show_attention`
  reports whether the event won the slot, and the caller chimes only then —
  the device can't show one event while playing another's.
- The calendar competes on that ladder instead of standing aside whenever a
  hook flag existed. "Meeting started" has a two-minute window and used to
  lose it whole to an active session's chatter.
- False positives are filtered out: background tasks and parallel agents
  are verified for real — a shell task by the output file its shell holds
  open (lsof), an agent by whether it has reported back to its session
  (the harness's own completion notification), with transcript freshness
  bounding both so a killed one can't pin the session as busy. System
  notifications don't count as user input, autonomous sessions never ring.

## Talking to the device from Claude

- `tools/clawdmeter_mcp.py` — a dependency-free stdio MCP server, so an
  assistant can put a line on the display itself: `show_message` (text +
  alert style, with its caption, color and melody), `clear_message`, and
  `device_status` (link state and the last usage payload). Register once
  with `claude mcp add --scope user clawdmeter -- python3
  tools/clawdmeter_mcp.py`.
- No new transport: it writes into the same event spool the hooks use, so the
  daemon picks the message up within one TICK and the firmware treats it
  exactly like a hook alert. `show_message` says so when the daemon is down
  or the BLE link is missing, instead of pretending the message landed.

- The corner mascot on the usage screen plays the persona scenes too —
  the magnifier while things are quiet, the laptop at a normal pace, then
  basketball, soccer and the skateboard as the burn rate climbs. Its buffer
  is measured from the act table instead of assuming the widest one, so the
  cast can grow without overrunning it. `mascot` on the serial console
  starts the next act immediately (they're otherwise 3.5–10 s apart).

## Calendar

- Meeting reminders **15 and 5 minutes** ahead (configurable via
  `cal_remind`) and a separate **"Meeting started!"** event (yellow, within
  a two-minute window after the start — a daemon that overslept stays
  silent).
- The source is the `cal_ics_url` option: an https ICS feed (a published
  Outlook/Google calendar), a local `.ics`, or `eventkit` — the macOS
  system calendar read via the signed Swift helper `daemon/calnext`
  (plain python gets silently refused EventKit access; build with
  `daemon/build_calnext.sh`).
- The ICS path expands recurring events (RRULE, Windows TZIDs) and drops
  all-day and cancelled entries; only the closest active threshold fires,
  and a reminder is marked sent only after a successful BLE write.

## Limits

- Usage comes from the read-only `GET /api/oauth/usage` endpoint (consumes
  nothing); the upstream probe request stays as a fallback and for
  Enterprise accounts.
- A **third gauge** — the model-scoped weekly limit (`weekly_scoped`, e.g.
  Fable): a compact pill in the Weekly panel colored by thresholds.
- Next to the "resets in …" countdown — the wall-clock reset time:
  "Resets in 4h 20m (Sun 00:20)".
- On poll failures the device honestly names the cause ("Update token",
  "No network"…) instead of rendering stale numbers; the poll interval
  backs off exponentially. The daemon deliberately never refreshes the
  token — refresh-token rotation could log Claude Code out.
- **Credentials that outlive the app.** Claude Code refreshes its 8-hour
  OAuth token only while it is running, so any longer gap used to leave the
  display stuck on "Update token". The daemon now prefers a long-lived
  token from `claude setup-token` (Keychain item `Clawdmeter-token`,
  overridable via `token_keychain_service`) and falls back to Claude Code's
  own entry when it is absent or rejected. It also skips a locally-expired
  token instead of spending a guaranteed 401, and cuts the backoff short
  the moment the stored credentials change — so the display recovers within
  seconds of Claude Code refreshing, not up to 10 minutes later.

## Live activity indicator

- An active-session counter (heartbeat hooks) — "·N" in the corner and a
  "Resting" status with a frozen spinner when Claude is idle.
- Splash animations follow the number of working sessions, rotating within
  the tier the workload calls for: 0 — resting (lurking, cloud, sailing),
  1–2 — heads-down work (laptop, magnifier, pointing, crab walking), 3 —
  picking up (basketball, skateboard, soccer), 4 — excited (jumping,
  trumpet, waving), 5+ — dancing at double tempo.

## Misc

- Numeric battery percent next to the icon, visible while charging too.
- Screen off after 5 minutes of Claude inactivity (auto-wake on activity
  and events).
- Serial commands `usage` / `splash` switch screens without buttons (for
  QA screenshots via `screenshot.sh`).
- Upstream bugfix: ArduinoJson's `doc["x"] | false` silently drops integer
  flags — replaced with `.as<bool>()`.
