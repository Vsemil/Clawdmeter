#include "../../hal/sound_hal.h"
#include <stdio.h>

void sound_hal_init(void) {}
void sound_hal_tick(void) {}
void sound_hal_play_reset(void) { printf("[sim] chime! (session reset)\n"); }
// Alerts print their ATTN_* kind so a scenario run shows which melody the
// hardware would have played.
void sound_hal_play_alert(uint8_t kind) { printf("[sim] alert melody #%u\n", kind); }
