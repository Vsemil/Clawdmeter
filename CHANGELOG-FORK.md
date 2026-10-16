# What this fork changes

A fork of [HermannBjorgvin/Clawdmeter](https://github.com/HermannBjorgvin/Clawdmeter)
2.0 that turns the device into a desk-side Claude Code pager in Russian or
English. Verified on the Waveshare ESP32-S3-Touch-AMOLED-2.16 with the macOS
app. Every payload key below is additive: upstream firmware ignores them, and
this firmware behaves like upstream when the app doesn't send them.

## Language

- Every firmware string is in a per-language table (`firmware/src/lang.cpp`,
  `ui_text.h`): English and Russian, the whimsical verbs included.
- The app's `lang = en|ru` goes out as `lg`; the device keeps it in NVS and
  restarts into it, so the layout is sized for that language's words.
  `test_layout` checks every panel in both.
- Cyrillic companion fonts for every UI size, wired through `.fallback`:
  Golos Text behind Styrene, PT Serif behind Tiempos, DejaVu Sans Mono behind
  the mono font (`tools/gen_cyr_fonts.sh`). Alert text keeps Cyrillic.

## Alerts

- Typed alerts (`k`): each kind has its caption, colour and melody on the
  device, and the big Clawd plays its own acts (a finished turn dances). The
  melodies are strikes of the ding at other notes (`melody.cpp`).
- `done`: the Stop hook raises it for a turn of 45 s or more that left no
  shell task or agent running. Prompts the harness writes don't count as you;
  an idle prompt waiting on background work doesn't alert.
- The project (`pj`, the session's git root) follows the caption.
- Ranked alerts: a session waiting for you is never replaced by a
  lower-ranked alert, and a wait that loses the screen queues for it and comes
  back without ringing again.
- The device's own alerts: the session limit close (80 / 95 %) and the window
  refilled.
- `clawdmeter mcp`: an MCP server, so an assistant can put a message on the
  device (`show_message`, `clear_message`, `device_status`).

## Limits

- The model-scoped weekly limit (`f`/`fn`, e.g. Fable) at the right of the
  weekly card, read from the endpoint's `limits[]` entries.
- The wall-clock reset time after the countdown, where it fits.
- The reason the numbers aren't fresh (`er`: log in again, token expired,
  rate limited, no network) on the status line.

## Calendar

- `calendar = eventkit | https ICS | .ics path`: reminders 15 and 5 minutes
  ahead (`cal_remind`) and "meeting started"; sent only once they reach the
  device. `tools/calnext` is the EventKit helper.
- Today's next meeting (`nm`) takes the title's place.

## Sound and resting

- A vertical swipe on the panel sets the device's own volume, 10 % per step,
  shown on the status line. A ding plays at the new level and the level is
  kept in NVS (`volume_swipe.cpp`). A touch now taps on its release, so a
  swipe never taps.
- Quiet hours (`quiet_hours`, `quiet_volume`): the app sends `vo`, which
  scales that volume; there is no boot ding.
- The status line says "Resting" when Claude isn't working, and the screen
  goes dark after 5 minutes of it with nobody at the device, USB power or not.

## Tooling

- `tools/packaging/macos-app.sh` builds and installs `Clawdmeter.app` for this
  Mac, signed with a local certificate when there is one so macOS keeps its
  Bluetooth and calendar grants across rebuilds.
- Serial QA commands `lang`, `alert_kind`, `melody`, `volume`; `SIM_LANG` for the
  simulator.
