import Foundation

/// The menu's intentd row's flyout (issue 0607), in Gtools' `VerbMenu` shape
/// (gtools#0113, "Site > [ Open | Start | Stop | Restart ]"): Open, Start, Stop
/// and Restart, in place of the flat rows. Decided from facts the app already
/// holds, and pure, so what the flyout offers in each state is tested without
/// AppKit; the AppDelegate renders it.
enum DaemonMenu {
  /// What an entry runs when it is chosen. Open runs `openCommand`; Start,
  /// Stop and Restart are the daemon service's.
  enum Action: Sendable, Equatable {
    case open, start, stop, restart
  }

  /// What Open runs, through the `intent` the app resolves (issue 0646): the
  /// CLI's own `--browser`, the one builder of the page's address, which
  /// carries the daemon's token in the fragment. A bare address opens the
  /// page with no token, and every read it makes is refused.
  static let openCommand = ["estate", "status", "--browser"]

  /// One entry of the flyout: a verb, enabled or held, or the caption that
  /// stands in for the verbs while one runs ("Restarting…"), which runs nothing.
  struct Entry: Sendable, Equatable {
    let title: String
    let action: Action?
    let isEnabled: Bool
  }

  /// The flyout. Open while the daemon is live with a url and no verb runs, as
  /// Gtools' Open is while its site answers; Start, Stop and Restart by the ONE
  /// health predicate (AC-01.6), each held rather than hidden, as Gtools'
  /// are: Start when intentd is absent or unknown, Stop and Restart when it is
  /// live, and Restart alone when it is stale, because the socket has an owner
  /// to investigate and is never unlinked. While a lifecycle verb runs, its
  /// caption stands in for the three.
  static func entries(busy: String?, health: Health) -> [Entry] {
    let url: String? = if case .live(_, let published) = health { published } else { nil }
    let open = Entry(title: "Open", action: url.map { _ in .open }, isEnabled: busy == nil && url != nil)
    if let busy { return [open, Entry(title: busy, action: nil, isEnabled: false)] }
    let (start, stop, restart): (Bool, Bool, Bool) =
      switch health {
      case .live: (false, true, true)
      case .stale: (false, false, true)
      case .absent, .unknown: (true, false, false)
      }
    return [
      open,
      Entry(title: "Start", action: .start, isEnabled: start),
      Entry(title: "Stop", action: .stop, isEnabled: stop),
      Entry(title: "Restart", action: .restart, isEnabled: restart),
    ]
  }
}
