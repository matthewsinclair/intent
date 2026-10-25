import Foundation
import Testing

@testable import Intent

/// Issue 0607: the intentd row's flyout, entry by entry in each state, as
/// Gtools' VerbMenuTests pin its Site flyout (gtools#0113). What the verbs do is
/// the CLI's, pinned by its own suites.
@Suite("intentd flyout")
struct DaemonMenuTests {
  typealias Entry = DaemonMenu.Entry

  let url = "http://127.0.0.1:51737"

  @Test("a live daemon offers Open, Stop and Restart, and holds Start")
  func live() {
    let entries = DaemonMenu.entries(busy: nil, health: .live(endpoint: "/tmp/x.sock", url: url))
    let actions: [DaemonMenu.Action?] = [.open, .start, .stop, .restart]
    #expect(entries.map(\.title) == ["Open", "Start", "Stop", "Restart"])
    #expect(entries.map(\.action) == actions)
    #expect(entries.map(\.isEnabled) == [true, false, true, true])
  }

  /// Issue 0646: Open runs the CLI's own `--browser`, which opens the page with
  /// the daemon's token in the fragment, rather than a bare address the page's
  /// every read refuses.
  @Test("Open runs intent estate status --browser")
  func openRunsTheCLIsBrowser() {
    let entries = DaemonMenu.entries(busy: nil, health: .live(endpoint: "/tmp/x.sock", url: url))
    #expect(entries.first?.action == .open)
    #expect(IntentCLI.label(DaemonMenu.openCommand) == "intent estate status --browser")
  }

  /// Open opens the web face the daemon published, so a live
  /// daemon that published none has no page for it to open.
  @Test("a live daemon that published no url holds Open")
  func liveWithoutAURL() {
    let entries = DaemonMenu.entries(busy: nil, health: .live(endpoint: "/tmp/x.sock", url: nil))
    #expect(entries.map(\.isEnabled) == [false, false, true, true])
    #expect(entries.first == Entry(title: "Open", action: nil, isEnabled: false))
  }

  /// AC-01.6: the socket has an owner to investigate, so the remedy is a
  /// restart, never an unlink.
  @Test("a stale daemon offers Restart alone")
  func stale() {
    let entries = DaemonMenu.entries(busy: nil, health: .stale(pid: 42))
    #expect(entries.map(\.title) == ["Open", "Start", "Stop", "Restart"])
    #expect(entries.map(\.isEnabled) == [false, false, false, true])
  }

  @Test("an absent daemon, and one whose state is unknown, offer Start alone")
  func absentOrUnknown() {
    #expect(DaemonMenu.entries(busy: nil, health: .absent).map(\.isEnabled) == [false, true, false, false])
    #expect(
      DaemonMenu.entries(busy: nil, health: .unknown("not yet polled")).map(\.isEnabled) == [false, true, false, false])
  }

  /// Mid-restart is exactly when the old address is dead, so Open is held too.
  @Test("while a lifecycle verb runs, its caption stands in for Start, Stop and Restart, and Open is held")
  func busy() {
    let entries = DaemonMenu.entries(busy: "Restarting…", health: .live(endpoint: "/tmp/x.sock", url: url))
    #expect(
      entries == [
        Entry(title: "Open", action: .open, isEnabled: false),
        Entry(title: "Restarting…", action: nil, isEnabled: false),
      ])
  }
}
