#pragma once
#include <stdint.h>

// A vertical swipe on the panel sets the device's own loudness: up is
// louder, down is quieter, VOLUME_STEP_PCT per VOLUME_STEP_PX. A touch that
// moves less than VOLUME_SWIPE_START_PX stays a tap (the director's).
//
// Pure: no Arduino, native-tested in test/test_volume_swipe.

#define VOLUME_SWIPE_START_PX 24
#define VOLUME_STEP_PX        36
#define VOLUME_STEP_PCT       10

struct VolumeSwipe {
    bool    swiping;     // it moved far enough: this touch is a swipe, not a tap
    int16_t y0;          // where it went down
    uint8_t from;        // the level when it went down
    uint8_t level;       // the level now
};

uint8_t volume_clamp(int pct);   // 0..100

// Call on the press edge, then move while held, release on the lift edge.
void volume_swipe_press(VolumeSwipe* s, int16_t y, uint8_t level);
// The finger moved to `y`: true when the level changed.
bool volume_swipe_move(VolumeSwipe* s, int16_t y);
// The finger lifted: true when this touch was a swipe (not a tap).
bool volume_swipe_release(VolumeSwipe* s);
