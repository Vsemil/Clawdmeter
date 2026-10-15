#!/bin/bash
# Build calnext, the EventKit helper behind meeting reminders (calnext.swift).
# It embeds calnext.plist in the __info_plist section and signs ad-hoc: both
# are needed, or macOS won't show the calendar permission prompt.
#
# The calendar grant is tied to the binary: a rebuild asks again. Install it
# where the app looks (`cal_helper`, default ~/.config/clawdmeter/calnext)
# and run it once by hand from a terminal launched by launchd (Terminal.app),
# not from inside another app, so the prompt is yours to answer.
set -euo pipefail
cd "$(dirname "$0")"
swiftc calnext.swift -O -o calnext \
    -Xlinker -sectcreate -Xlinker __TEXT -Xlinker __info_plist -Xlinker calnext.plist
codesign --force --sign - calnext
echo "OK: $(pwd)/calnext"
