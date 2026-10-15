#include "hal/board_info_hal.h"

// Weak default: boards with one hardware revision don't need to implement it.
__attribute__((weak)) const char* board_hal_rev(void) { return ""; }

#if __has_include(<esp_system.h>)
#include <esp_system.h>
__attribute__((weak)) void board_hal_restart(void) { esp_restart(); }
#else
__attribute__((weak)) void board_hal_restart(void) {}   // simulator: keep running
#endif
