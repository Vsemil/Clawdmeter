// calnext prints the macOS system calendar's meetings for the next 26 hours
// as JSON: [{"start": <epoch>, "title": "..."}, ...]. The desktop app runs it
// for `calendar = eventkit` (clawdmeter-core::calendar). A bare binary has
// no Info.plist with NSCalendars*UsageDescription, so macOS silently refuses
// it EventKit access; this one embeds its description through the linker
// (-sectcreate __TEXT __info_plist), so the permission prompt shows as it
// should. Build: ./build.sh
import EventKit
import Foundation

let store = EKEventStore()
let sem = DispatchSemaphore(value: 0)
var granted = false
if #available(macOS 14.0, *) {
    store.requestFullAccessToEvents { ok, _ in granted = ok; sem.signal() }
} else {
    store.requestAccess(to: .event) { ok, _ in granted = ok; sem.signal() }
}
// The first run waits for the user to answer the system prompt.
_ = sem.wait(timeout: .now() + 120)
guard granted else {
    FileHandle.standardError.write(Data("calendar access denied\n".utf8))
    exit(2)
}

let now = Date()
let end = now.addingTimeInterval(26 * 3600)   // = calendar::HORIZON
let pred = store.predicateForEvents(withStart: now, end: end, calendars: nil)
var out: [[String: Any]] = []
for ev in store.events(matching: pred) {
    if ev.isAllDay || ev.status == .canceled { continue }
    guard let start = ev.startDate else { continue }
    out.append(["start": start.timeIntervalSince1970,
                "title": ev.title ?? ""])
}
FileHandle.standardOutput.write(try JSONSerialization.data(withJSONObject: out))
