#include "hal/board_info_hal.h"

// Weak default: boards with one hardware revision don't need to implement it.
__attribute__((weak)) const char* board_hal_rev(void) { return ""; }

#if __has_include(<esp_system.h>)
#include <esp_system.h>
__attribute__((weak)) void board_hal_restart(void) { esp_restart(); }
#else
__attribute__((weak)) void board_hal_restart(void) {}   // simulator: keep running
#endif

#include "hal/sound_hal.h"
// Weak default: boards without melodies (or without sound) play the ding.
__attribute__((weak)) void sound_hal_melody(uint8_t melody) { (void)melody; sound_hal_chime(); }
__attribute__((weak)) void sound_hal_set_level(uint8_t pct) { (void)pct; }
