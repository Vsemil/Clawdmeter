#include "volume_swipe.h"

uint8_t volume_clamp(int pct) { return pct < 0 ? 0 : pct > 100 ? 100 : (uint8_t)pct; }

void volume_swipe_press(VolumeSwipe* s, int16_t y, uint8_t level) {
    s->swiping = false;
    s->y0 = y;
    s->from = s->level = level;
}

bool volume_swipe_move(VolumeSwipe* s, int16_t y) {
    const int dy = s->y0 - y;                       // up is positive
    if (!s->swiping && (dy >= VOLUME_SWIPE_START_PX || dy <= -VOLUME_SWIPE_START_PX))
        s->swiping = true;
    if (!s->swiping) return false;
    const uint8_t level = volume_clamp(s->from + (dy / VOLUME_STEP_PX) * VOLUME_STEP_PCT);
    if (level == s->level) return false;
    s->level = level;
    return true;
}

bool volume_swipe_release(VolumeSwipe* s) {
    const bool was = s->swiping;
    s->swiping = false;
    return was;
}
