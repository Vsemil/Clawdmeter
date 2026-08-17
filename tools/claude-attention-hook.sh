#!/bin/bash
# Claude Code hook → Clawdmeter attention flag.
# Wire it in ~/.claude/settings.json:
#   Notification      → claude-attention-hook.sh notification
#   Stop              → claude-attention-hook.sh stop
#   UserPromptSubmit  → claude-attention-hook.sh prompt
# Writes an event type (input|perm|done|clear) into the spool the BLE daemon
# watches. Reads the hook JSON from stdin.

set -u
DIR="$HOME/.config/claude-usage-monitor"
SPOOL="$DIR/attention.d"   # one file per event — see write_flag()
SESS="$DIR/sessions"   # per-session liveness: mtime = last heartbeat, content = fg|bg
MIN_TURN_S=45          # "done" only for turns longer than this — short Q&A means you're right there
IN=$(</dev/stdin)      # builtin read — no cat fork
# One stat, on the newest of the three dirs: this runs before the heartbeat
# early-exit below, i.e. on every tool call of every session.
[[ -d "$SPOOL" ]] || mkdir -p "$DIR" "$SESS" "$SPOOL"

# Hot path: PostToolUse fires on EVERY tool call of every session — handle it
# with zero subprocess spawns (bash regex instead of jq; session_id is the
# first field in the hook JSON, so the first match is the right one).
if [[ "${1:-}" == heartbeat ]]; then
    [[ "$IN" =~ \"session_id\"[[:space:]]*:[[:space:]]*\"([^\"]+)\" ]] \
        && echo fg > "$SESS/${BASH_REMATCH[1]}"
    exit 0
fi

jqr() { printf '%s' "$IN" | jq -r "$1 // \"\"" 2>/dev/null; }

# Does the session still have live background work? Two kinds share the tasks
# dir (keyed by the session's ORIGINAL project dir while the hook's cwd
# follows shell cd's, so locate it by globbing the unique session id):
#   - shell tasks: regular .output files, held open by the running shell —
#     lsof sees them (NB: its exit code is useless — 1 even with matches —
#     test stdout);
#   - async agents: symlinks to the agent's transcript, appended in bursts
#     and never held open. A finished agent's transcript ends with its final
#     end_turn message; anything else plus a recent write = still working.
# When the last task finishes, the harness wakes the main session for a
# summary turn — that turn's Stop is where "done" fires. Same heuristic as
# _bg_session_still_running in daemon/claude_usage_daemon.py — keep in sync.
AGENT_FRESH_S=300      # transcript idle longer than this = agent dead/killed
# An agent that finished announces itself to its session as a <task-id>
# notification in the session transcript — the only unambiguous completion
# signal on disk. (The agent's own transcript tail can't provide one: a final
# answer and a mid-turn message look alike, and 28% of finished agents don't
# end on "end_turn".) Only fresh transcripts are worth the grep.
has_running_tasks() {
    local t link aid main
    for main in "$HOME"/.claude/projects/*/"$1".jsonl; do
        [[ -f "$main" ]] && break
    done
    for t in /private/tmp/claude-$(id -u)/*/"$1"/tasks; do
        [[ -d "$t" ]] || continue
        lsof -w +d "$t" 2>/dev/null | grep -q . && return 0
        for link in "$t"/*; do
            [[ -L "$link" && -f "$link" ]] || continue   # agent, target alive
            [[ -n "$(find -L "$link" -mtime -${AGENT_FRESH_S}s 2>/dev/null)" ]] \
                || continue                             # dead, or long finished
            aid=${link##*/}; aid=${aid%.output}
            [[ -f "$main" ]] && grep -qF "<task-id>$aid</task-id>" "$main" \
                && continue                             # already reported back
            return 0
        done
    done
    return 1
}

# Drop one event into the daemon's spool (entry format is documented there,
# next to ATTN_SPOOL): line 1 = type, line 2 = the project shown on the device
# — the git repo root basename, falling back to cwd. Line 3 is left off, so
# the project doubles as the address a later `clear` is matched against.
#
# One file per event: a single shared slot meant two events inside the daemon's
# 5 s tick overwrote each other, and the loser was gone for good. Written aside
# and renamed in, so the daemon can never read a half-written event; the name
# only has to be unique ($$ + $RANDOM are builtins, `date` would be a fork).
write_flag() {
    local type="$1" cwd; cwd=$(jqr '.cwd')
    local root=""
    [[ -n "$cwd" ]] && root=$(git -C "$cwd" rev-parse --show-toplevel 2>/dev/null)
    [[ -z "$root" ]] && root="$cwd"
    local tmp="$SPOOL/.tmp.$$"
    printf '%s\n%s\n' "$type" "${root##*/}" > "$tmp" \
        && mv -f "$tmp" "$SPOOL/$$-$RANDOM-$type"
}

case "${1:-}" in
notification)
    # Claude is waiting: a permission prompt or an idle "waiting for your input".
    msg=$(jqr '.message' | tr '[:upper:]' '[:lower:]')
    if [[ "$msg" == *permission* || "$msg" == *разрешен* ]]; then
        write_flag perm
    else
        # Idle notifications also fire for autonomous sessions parked on their
        # own background work (builds, monitors, loops) — they're waiting for
        # the task, not for the user. Don't ring the bell for those.
        sid=$(jqr '.session_id')
        [[ -n "$sid" ]] && has_running_tasks "$sid" && exit 0
        write_flag input
    fi
    ;;
stop)
    sid=$(jqr '.session_id')
    # Don't celebrate a turn that merely yielded to a still-running background
    # task (build, flash, monitor) — the harness resumes it later.
    if [[ -n "$sid" ]]; then
        if has_running_tasks "$sid"; then
            echo bg > "$SESS/$sid"       # still working, just in the background
            exit 0
        fi
        rm -f "$SESS/$sid"               # turn really finished — session is idle
    fi
    # Skip short interactive turns — the user is at the keyboard anyway.
    if [[ -n "$sid" && -f "$DIR/turn-start-$sid" ]]; then
        started=$(cat "$DIR/turn-start-$sid" 2>/dev/null || echo 0)
        (( $(date +%s) - started < MIN_TURN_S )) && exit 0
    fi
    write_flag done
    ;;
prompt)
    # Any prompt — real or harness-generated — means the session is working.
    sid=$(jqr '.session_id')
    [[ -n "$sid" ]] && echo fg > "$SESS/$sid"
    # Harness-generated turns (background-task notifications, scheduled
    # wakeups) also fire UserPromptSubmit — they are NOT the user coming back,
    # so they must neither dismiss an alert nor restart the turn clock.
    p=$(jqr '.prompt')
    case "$p" in *"[SYSTEM NOTIFICATION"*|*"<task-notification>"*|*"<system-reminder>"*) exit 0;; esac
    # The user is typing: stamp the turn start and dismiss this project's
    # alert. Addressed, not global — typing here used to wipe the permission
    # prompt another session was blocked on, and nothing ever raised it again.
    [[ -n "$sid" ]] && date +%s > "$DIR/turn-start-$sid"
    write_flag clear
    # One sweep covers all three: turn-start stamps in $DIR, stale session
    # files in $SESS, and spool events the daemon never got to (both live
    # inside $DIR).
    find "$DIR" -type f \( -name 'turn-start-*' -o -path "$SESS/*" \
         -o -path "$SPOOL/*" \) -mtime +1 -delete 2>/dev/null
    ;;
esac
exit 0
