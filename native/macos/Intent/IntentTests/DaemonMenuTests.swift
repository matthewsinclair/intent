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

  @Test("a live daemon offers Open, Stop, Restart and Copy URL, and holds Start")
  func live() {
    let entries = DaemonMenu.entries(busy: nil, health: .live(endpoint: "/tmp/x.sock", url: url))
    let actions: [DaemonMenu.Action?] = [.open, .start, .stop, .restart, .copyURL]
    #expect(entries.map(\.title) == ["Open", "Start", "Stop", "Restart", "Copy URL"])
    #expect(entries.map(\.action) == actions)
    #expect(entries.map(\.isEnabled) == [true, false, true, true, true])
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

  /// Issue 0652: Copy URL runs the CLI's own `--url`, which prints the address
  /// `--browser` opens, token and all, for whichever browser it is pasted into.
  @Test("Copy URL runs intent estate status --url")
  func copyURLRunsTheCLIsURL() {
    let entries = DaemonMenu.entries(busy: nil, health: .live(endpoint: "/tmp/x.sock", url: url))
    #expect(entries.last?.action == .copyURL)
    #expect(IntentCLI.label(DaemonMenu.urlCommand) == "intent estate status --url")
  }

  /// Open opens the web face the daemon published and Copy URL copies its
  /// address, so a live daemon that published none holds both.
  @Test("a live daemon that published no url holds Open and Copy URL")
  func liveWithoutAURL() {
    let entries = DaemonMenu.entries(busy: nil, health: .live(endpoint: "/tmp/x.sock", url: nil))
    #expect(entries.map(\.isEnabled) == [false, false, true, true, false])
    #expect(entries.first == Entry(title: "Open", action: nil, isEnabled: false))
    #expect(entries.last == Entry(title: "Copy URL", action: nil, isEnabled: false))
  }

  /// AC-01.6: the socket has an owner to investigate, so the remedy is a
  /// restart, never an unlink.
  @Test("a stale daemon offers Restart alone")
  func stale() {
    let entries = DaemonMenu.entries(busy: nil, health: .stale(pid: 42))
    #expect(entries.map(\.title) == ["Open", "Start", "Stop", "Restart", "Copy URL"])
    #expect(entries.map(\.isEnabled) == [false, false, false, true, false])
  }

  @Test("an absent daemon, and one whose state is unknown, offer Start alone")
  func absentOrUnknown() {
    #expect(DaemonMenu.entries(busy: nil, health: .absent).map(\.isEnabled) == [false, true, false, false, false])
    #expect(
      DaemonMenu.entries(busy: nil, health: .unknown("not yet polled")).map(\.isEnabled) == [false, true, false, false, false])
  }

  /// Mid-restart is exactly when the old address is dead, so Open is held too,
  /// and Copy URL with it, still last.
  @Test("while a lifecycle verb runs, its caption stands in for Start, Stop and Restart, and Open and Copy URL are held")
  func busy() {
    let entries = DaemonMenu.entries(busy: "Restarting…", health: .live(endpoint: "/tmp/x.sock", url: url))
    #expect(
      entries == [
        Entry(title: "Open", action: .open, isEnabled: false),
        Entry(title: "Restarting…", action: nil, isEnabled: false),
        Entry(title: "Copy URL", action: .copyURL, isEnabled: false),
      ])
  }
}
