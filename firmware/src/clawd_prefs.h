#pragma once

// Clawd's settings from the app, kept in NVS so they apply from boot, before
// the app connects. The app sends both in every data payload.
//   liveliness       `lv`  1 No-nonsense .. 5 All-nonsense (clawd_policy.h)
//   attention anim   `aa`  seconds he points and waves for an alert; 0 = off

void clawd_prefs_init(void);          // after director_init(): load and apply the saved values
void liveliness_set(int level);       // apply; saved only when it changes
int  liveliness_get(void);
void attn_anim_set(int seconds);      // apply; saved only when it changes
int  attn_anim_get(void);

// The interface language (`lg`, ui_text.h). The layout is sized for one
// language at boot, so a change is saved and the device restarts into it.
void lang_pref_init(void);            // before ui_init(): select the saved table
void lang_pref_set(const char* code); // "" or unknown: no change

// The device's own loudness, 0..100 (a vertical swipe sets it,
// volume_swipe.h), kept in NVS; default 100. The app's quiet hours scale it.
void volume_pref_init(void);
int  volume_get(void);
void volume_set(int pct);              // saved only when it changes
