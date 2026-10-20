#pragma once

// Device settings from the app, kept in NVS so they apply from boot, before
// the app connects. The app sends them in every data payload.
//   liveliness       `lv`  1 No-nonsense .. 5 All-nonsense (clawd_policy.h)
//   attention anim   `aa`  seconds he points and waves for an alert; 0 = off
//   pace marker      `pm`  1 = pace marker on the usage bars (pace.h)

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
// volume_swipe.h), kept in NVS and loaded by clawd_prefs_init(); default
// 100. The app's quiet hours scale it.
int  volume_get(void);
void volume_set(int pct);              // saved only when it changes
void pace_marker_set(bool on);      // apply; saved only when it changes
bool pace_marker_get(void);
