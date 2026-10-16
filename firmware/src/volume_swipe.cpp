#include "volume_swipe.h"

void volume_swipe_press(VolumeSwipe* s, int16_t y, uint8_t level) {
    s->down = true;
    s->swiping = false;
    s->y0 = y;
    s->from = s->level = level > 100 ? 100 : level;
}

bool volume_swipe_move(VolumeSwipe* s, int16_t y) {
    if (!s->down) return false;
    const int dy = s->y0 - y;                       // up is positive
    if (!s->swiping && (dy >= VOLUME_SWIPE_START_PX || dy <= -VOLUME_SWIPE_START_PX))
        s->swiping = true;
    if (!s->swiping) return false;
    int level = s->from + (dy / VOLUME_STEP_PX) * VOLUME_STEP_PCT;
    if (level < 0) level = 0;
    if (level > 100) level = 100;
    if (level == s->level) return false;
    s->level = (uint8_t)level;
    return true;
}

bool volume_swipe_release(VolumeSwipe* s) {
    const bool was = s->down && s->swiping;
    s->down = false;
    s->swiping = false;
    return was;
}
