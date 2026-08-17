#!/usr/bin/env python3
"""Tests for the attention spool — the queue behind overlapping alerts.

Events used to share one file, so anything written inside the daemon's 5 s
tick overwrote whatever was already there: a permission prompt could be eaten
by a `clear` from a different session and never came back. The spool keeps one
file per event, hands the daemon the most important one, and leaves the rest
for the following ticks.

Run: python -m pytest daemon/tests/test_attention_spool.py -x -q
"""
import asyncio
import itertools
import json
import os
import time
from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from daemon import claude_usage_daemon as d
from daemon.tests.conftest import connected_client


@pytest.fixture
def spool(tmp_path, monkeypatch):
    """Point the module's spool at a temp dir; return a writer for events."""
    path = tmp_path / "attention.d"
    path.mkdir()
    monkeypatch.setattr(d, "ATTN_SPOOL", path)
    monkeypatch.setattr(d, "ATTN_FILE", tmp_path / "attention")
    seq = itertools.count()

    def write(kind: str, project: str = "", age: float = 0.0,
              scope: str | None = None):
        f = path / f"{next(seq)}-{kind}"
        body = f"{kind}\n{project}\n" + ("" if scope is None else f"{scope}\n")
        f.write_text(body)
        if age:
            stamp = time.time() - age
            os.utime(f, (stamp, stamp))
        return f

    return write


def _ordered():
    """The spool as the loop sees it: the ladder is applied by the caller."""
    return sorted(d.read_attention_spool(), key=d._attn_order)


def test_two_events_in_one_tick_both_survive(spool):
    """The old single slot kept only the last writer; now both are queued."""
    spool("done", "clawdmeter")
    spool("perm", "birthday-bot")
    got = _ordered()
    assert [(e.kind, e.project) for e in got] == [
        ("perm", "birthday-bot"), ("done", "clawdmeter")]


def test_clear_never_jumps_ahead_of_an_alert(spool):
    """Typing in one session must not swallow another session's prompt."""
    spool("perm", "birthday-bot")
    spool("clear", "clawdmeter")
    assert _ordered()[0].kind == "perm"


def test_priority_order_is_blocking_then_timed_then_cheerful(spool):
    for kind in ("done", "cal", "input", "calstart", "perm", "clear"):
        spool(kind)
    assert [e.kind for e in _ordered()] == [
        "perm", "input", "calstart", "cal", "done", "clear"]


def test_same_priority_is_oldest_first(spool):
    old = spool("done", "first", age=30)
    spool("done", "second")
    got = _ordered()
    assert [e.project for e in got] == ["first", "second"]
    assert got[0].paths == (old,)


def test_duplicate_event_collapses_but_keeps_both_files(spool):
    """Two agents in one repo raise one alert — and both files must go."""
    a, b = spool("perm", "clawdmeter"), spool("perm", "clawdmeter")
    got = _ordered()
    assert len(got) == 1
    assert set(got[0].paths) == {a, b}
    d.retire_attention(got[0])
    assert not a.exists() and not b.exists()


def test_stale_events_are_dropped(spool):
    stale = spool("done", "clawdmeter", age=d.ATTN_MAX_AGE + 1)
    fresh = spool("input", "clawdmeter")
    assert [e.kind for e in _ordered()] == ["input"]
    assert not stale.exists() and fresh.exists()


def test_retiring_only_drops_the_delivered_event(spool):
    perm = spool("perm", "birthday-bot")
    done = spool("done", "clawdmeter")
    got = _ordered()
    d.retire_attention(got[0])
    assert not perm.exists() and done.exists()
    assert [e.kind for e in _ordered()] == ["done"]


def test_legacy_single_slot_is_still_read(spool):
    """An older hook or MCP copy keeps writing the pre-spool file."""
    d.ATTN_FILE.write_text("input\nclawdmeter\n")
    got = _ordered()
    assert [(e.kind, e.project) for e in got] == [("input", "clawdmeter")]
    d.retire_attention(got[0])
    assert not d.ATTN_FILE.exists()


def test_unknown_type_falls_back_to_input(spool):
    spool("banana", "clawdmeter")
    assert _ordered()[0].kind == "input"


def test_project_is_truncated_to_the_device_budget(spool):
    spool("done", "x" * (d.NP_MAX_CHARS + 20))
    assert len(_ordered()[0].project) == d.NP_MAX_CHARS


# --- the connected loop: what it forwards and what it retires ---------------

def _run(monkeypatch, cal=None, then=None, beats=1):
    """Run connect_and_run's loop until `beats` payloads went out.

    `then` fires after the first payload — use it to drop another event into
    the spool and watch what the following beat does with it. Returns the
    payloads sent and how many API polls they cost.
    """
    client = connected_client()
    writes: list[dict] = []
    polls: list[int] = []
    stop_event = asyncio.Event()

    async def fake_poll():
        polls.append(1)
        return {"s": 42, "ok": True}

    async def cap_write(uuid, data, response=False):
        writes.append(json.loads(bytes(data).decode()))
        if len(writes) == 1 and then:
            then()
        if len(writes) >= beats:
            stop_event.set()

    client.write_gatt_char = AsyncMock(side_effect=cap_write)
    monkeypatch.setattr(d, "read_cal_config", lambda: ("ics" if cal else "", (15, 5)))
    monkeypatch.setattr(d, "refresh_cal_events", lambda url: None)
    monkeypatch.setattr(d, "check_cal_reminder", lambda th: cal)
    monkeypatch.setattr(d, "count_active_sessions", lambda: 1)
    monkeypatch.setattr(d, "TICK", 0.01)   # don't idle a real tick per test
    monkeypatch.setattr(d, "write_status", lambda *a, **k: None)
    async def go():
        # A loop that never reaches `beats` would hang the suite — fail instead.
        await asyncio.wait_for(
            d.connect_and_run(MagicMock(address="AA:BB"), stop_event), timeout=5)

    with patch.object(d, "BleakClient", return_value=client), \
         patch.object(d, "poll_active_payload", new=fake_poll):
        asyncio.run(go())
    return writes, len(polls)


def _one_beat(monkeypatch, cal=None):
    """Just the payloads of a single beat."""
    return _run(monkeypatch, cal=cal)[0]


def test_plain_beat_with_no_events_survives(monkeypatch, spool):
    """Regression: retiring the delivered event tripped over `None is None`
    and crashed the daemon on every eventless beat."""
    sent = _one_beat(monkeypatch)
    assert sent and "n" not in sent[-1]


def test_spool_event_rides_the_beat_and_is_retired(monkeypatch, spool):
    f = spool("perm", "clawdmeter")
    sent = _one_beat(monkeypatch)
    assert sent[-1]["n"] == "perm" and sent[-1]["np"] == "clawdmeter"
    assert not f.exists()


def test_hook_events_are_addressed_by_their_project(monkeypatch, spool):
    """No third line: the context line doubles as the address, so no "ns"."""
    spool("perm", "clawdmeter")
    assert "ns" not in _one_beat(monkeypatch)[-1]


def test_a_message_addresses_itself_as_nothing(monkeypatch, spool):
    """An MCP message shows free text and stays dismissable by any clear."""
    spool("done", "deploy finished", scope="")
    sent = _one_beat(monkeypatch)[-1]
    assert sent["np"] == "deploy finished" and sent["ns"] == ""


def test_an_alert_does_not_wait_behind_a_poll(monkeypatch, spool):
    """An alert needs to ring, not to refresh the numbers. It used to force a
    poll — and while the API is unreachable that poll is two connect timeouts,
    so the alert landed ~40 s late and the screen flashed "No network"."""
    writes, polls = _run(monkeypatch, then=lambda: spool("perm", "clawdmeter"),
                         beats=2)
    assert polls == 1, "the alert beat must not spend a second poll"
    assert writes[-1]["n"] == "perm"
    assert writes[-1]["s"] == 42, "it rides the last good numbers"


def test_calendar_no_longer_waits_for_a_quiet_spool(monkeypatch, spool):
    """A started meeting outranks a finished turn — it used to be skipped
    outright whenever any hook event was present."""
    f = spool("done", "clawdmeter")
    sent = _one_beat(monkeypatch, cal=("14:00 Grooming", 1785474000.0, 0))
    assert sent[-1]["n"] == "calstart"
    assert f.exists(), "the hook event must keep its place in the spool"
