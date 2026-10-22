#include "alert_kind.h"
#include <string.h>

static const struct { const char* code; AlertKind kind; } CODES[] = {
    { "permission", AK_PERMISSION }, { "other", AK_PERMISSION },
    { "question", AK_QUESTION },     { "idle", AK_IDLE },
    { "elicitation", AK_ELICITATION },
    { "done", AK_DONE },             { "limit", AK_LIMIT },
    { "reset", AK_RESET },           { "cal", AK_CAL },
    { "calstart", AK_CAL_START },    { "msg", AK_MESSAGE },
};

AlertKind alert_kind_parse(const char* k) {
    if (!k) return AK_NONE;
    for (const auto& c : CODES)
        if (!strcmp(k, c.code)) return c.kind;
    return AK_NONE;
}

Melody alert_kind_melody(AlertKind k) {
    switch (k) {
        case AK_PERMISSION:  return MELODY_PERM;
        case AK_QUESTION:
        case AK_ELICITATION: return MELODY_QUESTION;
        case AK_IDLE:        return MELODY_INPUT;
        case AK_DONE:        return MELODY_DONE;
        case AK_LIMIT:       return MELODY_LIMIT;
        case AK_CAL:         return MELODY_CAL;
        case AK_CAL_START:   return MELODY_CAL_START;
        default:             return MELODY_DING;
    }
}

bool alert_kind_shows_message(AlertKind k) {
    return k == AK_NONE || k == AK_MESSAGE;
}

// A finished turn's cheers: official loops only (each fits the corner too).
static const char* const CHEERS[][2] = {
    { "dancing",       "jumping happy" },
    { "jumping happy", "waving" },
    { "jumping",       "dancing" },
    { "dancing",       "waving" },
    { "jumping happy", "jumping" },
};

const char* alert_kind_fanfare(AlertKind k, uint32_t pick) {
    return (k == AK_DONE || k == AK_RESET) && pick % 3 == 0 ? "trumpet" : nullptr;
}

void alert_kind_acts(AlertKind k, uint32_t pick, const char** a, const char** b) {
    switch (k) {
        case AK_DONE:
        case AK_RESET: {
            const char* const* c = CHEERS[pick % (sizeof(CHEERS) / sizeof(CHEERS[0]))];
            *a = c[0]; *b = c[1];
            return;
        }
        case AK_IDLE:
        case AK_CAL:
        case AK_CAL_START:
        case AK_MESSAGE: *a = "waving";  *b = "pointing";      return;
        default:         *a = "pointing"; *b = "waving";       return;   // asks for you: point first
    }
}
