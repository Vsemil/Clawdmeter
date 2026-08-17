#pragma once
#include "data.h"
#include "ble.h"

enum screen_t {
    SCREEN_SPLASH,
    SCREEN_USAGE,
    SCREEN_COUNT,
};

void ui_init(void);
void ui_update(const UsageData* data);
void ui_tick_anim(void);
void ui_show_screen(screen_t screen);
void ui_toggle_splash(void);
screen_t ui_get_current_screen(void);
void ui_update_ble_status(ble_state_t state, const char* name, const char* mac);
// Show the attention view: wakes the display, jumps to the usage screen and
// overlays a creature + caption matching the event type (an ATTN_* value
// from data.h, ATTN_INPUT..ATTN_RESET); the header shows `project` (may be
// "" for none) so it's clear which session wants you. Dismissed by a tap or
// automatically after a per-type timeout.
//
// Returns false when the event was outranked by what's already on screen —
// the caller then skips its melody, so what you hear always matches what you
// see. INPUT/PERM are states (a session stays blocked until answered): a
// passing flash plays over them and hands the screen back on its way out.
// `scope` is who the event belongs to — usually the project, but an MCP
// message shows free text and passes "" so any clear dismisses it. NULL = the
// same as `project`.
bool ui_show_attention(uint8_t type, const char* project, const char* scope = NULL);
// Dismiss remotely (the user started typing on the host again). `scope`
// addresses one session's alert; "" or NULL dismisses whatever is up.
void ui_hide_attention(const char* scope);
// Daemon error beat ("ok":false): force the idle view now (don't render stale
// numbers as live) and show the reason in the status line. Codes: auth /
// token / rate / net / http. Cleared automatically by the next good payload.
void ui_set_data_error(const char* code);
// Switch the UI language ("ru"/"en", persisted; "" = keep current) and
// restamp the labels that aren't rewritten on every update.
void ui_set_lang(const char* lang);
void ui_update_battery(int percent, bool charging);
