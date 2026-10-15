#include "board.h"
#include <Preferences.h>
#include <stdlib.h>

// No I2C bus, no IO expander — nothing to bring up before the HAL inits.
// SIM_LANG=ru boots in that language, as a board does after the restart a
// language change triggers (the sim can't restart, and its NVS is in memory).
extern "C" void board_init(void) {
    if (const char* lang = getenv("SIM_LANG")) {
        Preferences prefs;
        prefs.begin("clawdmeter", false);
        prefs.putString("lang", lang);
        prefs.end();
    }
}
