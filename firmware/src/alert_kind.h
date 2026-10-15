#pragma once
#include <stdint.h>
#include "melody.h"

// What an alert is about (`k` in the payload): its caption (ui_text.h),
// colour and melody, and which Clawd acts the attention walk-on plays.
// The first five are Claude Code's own (the app's AlertKind); the rest are
// this fork's: a long turn finished, the usage limits, the calendar, and a
// message put up from outside (the MCP server).
//
// Pure: no Arduino, native-tested in test/test_alert_kind.

enum AlertKind : uint8_t {
    AK_NONE,         // no `k` (an older app): the message is shown as sent
    AK_PERMISSION,   // a tool needs permission ("other" types too)
    AK_QUESTION,     // Claude asks a question
    AK_IDLE,         // the prompt is waiting for your reply
    AK_ELICITATION,  // an MCP server asks for input
    AK_DONE,         // a long turn finished
    AK_LIMIT,        // the session limit is close (the device's own)
    AK_RESET,        // the session window refilled (the device's own)
    AK_CAL,          // a meeting soon
    AK_CAL_START,    // the meeting has started
    AK_MESSAGE,      // free text: shown as sent
    AK_COUNT
};

AlertKind alert_kind_parse(const char* k);   // "" / unknown: AK_NONE
Melody    alert_kind_melody(AlertKind k);
// The text is the message as sent rather than the kind's caption.
bool      alert_kind_shows_message(AlertKind k);
// The two acts the big Clawd alternates for this kind.
void      alert_kind_acts(AlertKind k, const char** a, const char** b);
