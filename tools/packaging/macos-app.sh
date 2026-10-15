#!/bin/bash
# Build Clawdmeter.app for this Mac (Apple Silicon) and install it, the way
# release.yml packages it: an app bundle, because CoreBluetooth needs
# NSBluetoothAlwaysUsageDescription in the responsible app's Info.plist or
# macOS kills the process under the LaunchAgent. GPUI compiles its shaders
# at run time (gpui/runtime_shaders), so no Metal toolchain is needed. The
# calendar descriptions are there because macOS asks about the calnext
# helper's calendar access on behalf of the app that runs it: without them it
# refuses silently, with no prompt.
#
#   tools/packaging/macos-app.sh [install dir, default /Applications]
set -euo pipefail
cd "$(dirname "$0")/../.."
dest="${1:-/Applications}"
ver="$(git describe --tags --always 2>/dev/null | sed 's/^v//')"
(cd desktop && CLAWDMETER_VERSION="$ver" cargo build --release --locked -p clawdmeter-app --features gpui/runtime_shaders)

stage="$(mktemp -d)"
app="$stage/Clawdmeter.app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp desktop/target/release/clawdmeter "$app/Contents/MacOS/clawdmeter"
cp tools/packaging/icons/clawdmeter.icns "$app/Contents/Resources/"
cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleIconFile</key><string>clawdmeter</string>
  <key>CFBundleIdentifier</key><string>com.clawdmeter.app</string>
  <key>CFBundleName</key><string>Clawdmeter</string>
  <key>CFBundleExecutable</key><string>clawdmeter</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$ver</string>
  <key>CFBundleVersion</key><string>$ver</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>LSUIElement</key><true/>
  <key>NSBluetoothAlwaysUsageDescription</key><string>Clawdmeter sends your Claude Code usage to your Clawdmeter over Bluetooth.</string>
  <key>NSCalendarsFullAccessUsageDescription</key><string>Clawdmeter shows meeting reminders on your desk display.</string>
  <key>NSCalendarsUsageDescription</key><string>Clawdmeter shows meeting reminders on your desk display.</string>
</dict></plist>
PLIST
plutil -lint "$app/Contents/Info.plist"
codesign --force --sign - "$app"
rm -rf "$dest/Clawdmeter.app"
ditto "$app" "$dest/Clawdmeter.app"
rm -rf "$stage"
echo "Installed $dest/Clawdmeter.app ($ver)"
