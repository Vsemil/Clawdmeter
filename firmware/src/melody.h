#pragma once
#include <stdint.h>

// One melody per alert kind, so you can tell what Claude wants without
// looking: each is a few strikes of the ding (ding.h) at other pitches,
// ringing into each other. MELODY_DING is the ding itself.
//
// Pure math, no Arduino: native-tested in test/test_melody.

enum Melody : uint8_t {
    MELODY_DING,       // the one sound (session reset, unknown kinds)
    MELODY_PERM,       // A5 A5 A5: a tool needs permission
    MELODY_QUESTION,   // E5 A5, rising: Claude asks a question
    MELODY_INPUT,      // E5 . E5: waiting for your reply
    MELODY_DONE,       // C5 E5 G5: a long turn finished
    MELODY_LIMIT,      // B5 A5 G5, falling: the session limit is close
    MELODY_CAL,        // G5 C6 E6: a meeting soon
    MELODY_CAL_START,  // C6 C6 G5: the meeting has started
    MELODY_COUNT
};

// Frames in `m` at `rate` Hz (its last strike's ring included).
uint32_t melody_frames(Melody m, uint32_t rate);
// Longest melody, for sizing a buffer.
uint32_t melody_max_frames(uint32_t rate);
// Frame `n` of `m`, in -1..1 (0 past the end).
float melody_value(Melody m, uint32_t n, uint32_t rate);
// Frame `n` as a 16-bit sample at `gain` (0..1).
int16_t melody_sample(Melody m, uint32_t n, uint32_t rate, float gain);
// "perm", "done", ...: for logs and the `melody` serial command.
const char* melody_name(Melody m);
// The melody for a name (melody_name), MELODY_COUNT when unknown.
Melody melody_by_name(const char* name);
