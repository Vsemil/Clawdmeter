#include "clawd_prefs.h"
#include "clawd_director.h"
#include "clawd_policy.h"
#include "ui_text.h"
#include "volume_swipe.h"
#include "hal/board_info_hal.h"
#include <Preferences.h>
#include <Arduino.h>
#include <string.h>

static uint8_t volume_pct = 100;

void clawd_prefs_init(void) {
    Preferences prefs;
    prefs.begin("clawdmeter", true);
    const int lively = prefs.getUChar("lively", CLAWD_LIVELINESS_DEFAULT);
    const int attn = prefs.getUChar("attn_anim", CLAWD_ATTN_ANIM_DEFAULT_S);
    volume_pct = volume_clamp(prefs.getUChar("volume", 100));
    prefs.end();
    director_set_liveliness(clawd_liveliness_clamp(lively));
    director_set_attn_anim(clawd_attn_anim_clamp(attn));
    Serial.printf("Clawd prefs init: liveliness %d, attention animation %d s, volume %d%%\n",
                  clawd_liveliness(), clawd_attn_anim(), volume_pct);
}

static void save(const char* key, uint8_t v) {
    Preferences prefs;
    prefs.begin("clawdmeter", false);
    prefs.putUChar(key, v);
    prefs.end();
}

void liveliness_set(int level) {
    level = clawd_liveliness_clamp(level);
    if (level == clawd_liveliness()) return;
    save("lively", (uint8_t)level);
    director_set_liveliness(level);
    Serial.printf("Liveliness: %d\n", level);
}

int liveliness_get(void) { return clawd_liveliness(); }

void attn_anim_set(int seconds) {
    seconds = clawd_attn_anim_clamp(seconds);
    if (seconds == clawd_attn_anim()) return;
    save("attn_anim", (uint8_t)seconds);
    director_set_attn_anim(seconds);
    Serial.printf("Attention animation: %d s\n", seconds);
}

int attn_anim_get(void) { return clawd_attn_anim(); }

int volume_get(void) { return volume_pct; }

void volume_set(int pct) {
    pct = volume_clamp(pct);
    if (pct == volume_pct) return;
    volume_pct = (uint8_t)pct;
    save("volume", (uint8_t)pct);
    Serial.printf("Volume: %d%%\n", pct);
}

void lang_pref_init(void) {
    Preferences prefs;
    prefs.begin("clawdmeter", true);
    char code[8] = "";
    prefs.getString("lang", code, sizeof(code));
    prefs.end();
    lang_select(lang_find(code));
    Serial.printf("Language: %s\n", S->code);
}

void lang_pref_set(const char* code) {
    const UiText* t = lang_find(code);
    if (!t || t == S) return;
    Preferences prefs;
    prefs.begin("clawdmeter", false);
    prefs.putString("lang", t->code);
    char back[8] = "";
    prefs.getString("lang", back, sizeof(back));
    prefs.end();
    // Restart only once the choice is on flash: a failed write would
    // otherwise restart on every payload.
    if (strcmp(back, t->code) != 0) {
        Serial.printf("Language %s: NVS write failed, keeping %s\n", t->code, S->code);
        return;
    }
    Serial.printf("Language: %s -> %s, restarting to lay it out\n", S->code, t->code);
    Serial.flush();
    board_hal_restart();
    lang_select(t);   // no restart (simulator): texts switch, the layout stays
}
