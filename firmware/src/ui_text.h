#pragma once
#include "alert_kind.h"

// Every phrase the device shows, one table per language (lang.cpp). The app
// picks the language with the payload's `lg` key; the device keeps it in NVS
// and restarts to apply it (clawd_prefs.cpp), so ui.cpp and layout.cpp only
// ever see one table: `S`, fixed for the life of the program.
//
// The copy on the meters decides the layout: layout.cpp sizes the screen for
// the widest of the strings marked "widest" below, so a rewording in either
// language is checked by test_layout on every panel size.

struct UiText {
    const char* code;              // "en", "ru": the `lg` value that picks it

    const char* title;             // usage view header while there's no clock

    // Pairing hint: heading, two steps, and a note line, per board kind /
    // state (ui.cpp apply_pair_text).
    const char* pair_pwr[3];
    const char* pair_touch[3];
    const char* pair_cleared[3];
    const char* pair_holding;

    // Card pills: session, weekly, then the spend-capped account's spending
    // and period. The short set is for panels too narrow for the long one.
    const char* pill[4];
    const char* pill_short[4];

    // Reset line: "Resets in" countdown formats (minutes; hours + minutes;
    // days + hours) and what a missing session reset means.
    const char* reset_m;
    const char* reset_hm;
    const char* reset_dh;
    const char* not_started;
    // The same countdown worded shorter, for panels too narrow for it.
    const char* reset_short[3];    // m, h m, d h

    // Spend-capped account: caption under the number, pace words, and the
    // period line's "Resets <date>".
    const char* of_spending_limit;
    const char* pace_under;
    const char* pace_on;
    const char* pace_over;
    const char* resets_on;         // "Resets %s"

    // Widest text a card's reset line shows: the weekly window's last day,
    // the spend-capped account's caption, and its period line (its widest
    // pace word and date), with and without the pace word.
    const char* reset_widest[3];
    const char* reset_short_widest[3];
    const char* period_widest;
    const char* period_short_widest;

    // Status-line states.
    const char* st_waiting;        // advertising, no host
    const char* st_listening;      // connected, between payloads
    const char* st_connected;
    const char* st_no_data;
    const char* st_stale;
    // Why the numbers aren't fresh (`er`), in place of st_stale/st_no_data
    // where it fits: auth, token, rate, net, app.
    const char* st_err[5];

    // Status-line words every panel must fit (the whimsical verbs are
    // skipped when they don't): "<spinner> <word>…".
    const char* status_required[5];

    // Weekday abbreviations, Sunday first: the weekly reset's wall-clock
    // time, "Resets in 6d 2h (Sun 00:20)".
    const char* weekday[7];

    // An alert's caption on the status line, by kind (alert_kind.h); the
    // kinds that show their message as sent have none.
    const char* alert_caption[AK_COUNT];

    // The whimsical verbs, Claude Code's spinner words.
    const char* const* words;
    int word_count;
};

extern const UiText UI_TEXT_EN;
extern const UiText UI_TEXT_RU;

// The table in use. English until lang_select() picks another.
extern const UiText* S;

// The table for a `lg` code, nullptr for "" or an unknown code.
const UiText* lang_find(const char* code);
// Make `t` the table in use (null keeps the current one).
void lang_select(const UiText* t);
// The `er` code's text in the table in use, nullptr for "" or unknown.
const char* lang_err_text(const char* code);
