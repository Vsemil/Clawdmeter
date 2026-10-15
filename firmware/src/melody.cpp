#include "melody.h"
#include "ding.h"
#include <math.h>
#include <string.h>

struct Strike { uint16_t at_ms; float hz; };

// Note pitches (equal temperament, A4 = 440).
#define C5 523.25f
#define E5 659.26f
#define G5 783.99f
#define A5 880.00f
#define B5 987.77f
#define C6 1046.50f
#define E6 1318.51f

static const Strike PERM[]      = { {0, A5}, {170, A5}, {340, A5} };
static const Strike QUESTION[]  = { {0, E5}, {190, A5} };
static const Strike INPUT[]     = { {0, E5}, {340, E5} };
static const Strike DONE[]      = { {0, C5}, {140, E5}, {280, G5} };
static const Strike LIMIT[]     = { {0, B5}, {130, A5}, {260, G5} };
static const Strike CAL[]       = { {0, G5}, {210, C6}, {420, E6} };
static const Strike CAL_START[] = { {0, C6}, {200, C6}, {400, G5} };

#define STRIKES(a) { a, (uint8_t)(sizeof(a) / sizeof((a)[0])) }
static const struct { const Strike* s; uint8_t n; const char* name; } MELODIES[MELODY_COUNT] = {
    { nullptr, 0, "ding" },
    { PERM, 3, "perm" },
    { QUESTION, 2, "question" },
    { INPUT, 2, "input" },
    { DONE, 3, "done" },
    { LIMIT, 3, "limit" },
    { CAL, 3, "cal" },
    { CAL_START, 3, "calstart" },
};

// Each strike at this share of the ding's level: overlapping rings add up.
static const float STRIKE_LEVEL = 0.62f;

uint32_t melody_frames(Melody m, uint32_t rate) {
    if (m >= MELODY_COUNT) return 0;
    if (!MELODIES[m].n) return ding_frames(rate);
    const Strike& last = MELODIES[m].s[MELODIES[m].n - 1];
    return (uint32_t)((uint64_t)last.at_ms * rate / 1000) + ding_frames(rate);
}

uint32_t melody_max_frames(uint32_t rate) {
    uint32_t most = 0;
    for (int m = 0; m < MELODY_COUNT; m++) {
        const uint32_t f = melody_frames((Melody)m, rate);
        if (f > most) most = f;
    }
    return most;
}

float melody_value(Melody m, uint32_t n, uint32_t rate) {
    if (m >= MELODY_COUNT || rate == 0) return 0.0f;
    if (!MELODIES[m].n) return ding_value(n, rate);
    float y = 0.0f;
    for (uint8_t i = 0; i < MELODIES[m].n; i++) {
        const uint32_t start = (uint32_t)((uint64_t)MELODIES[m].s[i].at_ms * rate / 1000);
        if (n < start) break;
        y += STRIKE_LEVEL * ding_note_value(n - start, rate, MELODIES[m].s[i].hz);
    }
    if (y > 1.0f) y = 1.0f;
    if (y < -1.0f) y = -1.0f;
    return y;
}

int16_t melody_sample(Melody m, uint32_t n, uint32_t rate, float gain) {
    if (gain < 0.0f) gain = 0.0f;
    if (gain > 1.0f) gain = 1.0f;
    return (int16_t)lrintf(melody_value(m, n, rate) * gain * 32767.0f);
}

const char* melody_name(Melody m) {
    return m < MELODY_COUNT ? MELODIES[m].name : "?";
}

Melody melody_by_name(const char* name) {
    for (int m = 0; m < MELODY_COUNT; m++)
        if (name && !strcmp(name, MELODIES[m].name)) return (Melody)m;
    return MELODY_COUNT;
}
