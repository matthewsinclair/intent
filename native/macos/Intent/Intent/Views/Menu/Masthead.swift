import AppKit
import ArcaMasthead

extension IntentMasthead {
  /// The menu's top row (issue 0607, as gtools#0106 makes Gtools'): arca_swift's
  /// MastheadRow, made here alone. It reads its folder, `intent` under
  /// `environment`'s XDG_CONFIG_HOME or ~/.config, from the environment it is
  /// made with, and asks `state` for the CLI's version at each open of its
  /// menu. The bundle's masthead.yaml and about.md are its defaults, and a copy
  /// of either in that folder replaces it with no rebuild. It is an NSMenuItem
  /// to the app, which needs nothing else of it, so the tests never import
  /// ArcaMasthead: the test bundle links no copy of its own.
  @MainActor
  static func row(
    state: @escaping @MainActor () -> VersionState, environment: [String: String] = ProcessInfo.processInfo.environment,
    bundle: Bundle = .main, log: @escaping @MainActor (String) -> Void
  ) -> NSMenuItem {
    MastheadRow(
      app: "intent", environment: environment, bundle: bundle, owned: owned,
      value: { value($0, version: state(), bundle: bundle) }, log: log)
  }
}

/// Where the masthead row sits: at the top of the status item's newest menu.
/// The row is one item, made once and kept, since it owns its About window and
/// remembers the refusals it has logged, and an item belongs to one menu at a
/// time, so each menu the app builds takes it from the one before - except
/// while that one is open, when moving it would take the top row off the
/// screen under the pointer. It moves when that menu closes. The app's menu
/// delegate forwards each event here. Gtools' MastheadPlacement, copied, not
/// shared.
@MainActor
final class MastheadPlacement {
  let row: NSMenuItem
  private weak var newest: NSMenu?
  private weak var open: NSMenu?

  init(row: NSMenuItem) {
    self.row = row
  }

  /// `menu` is the status item's menu now.
  func installed(_ menu: NSMenu) {
    newest = menu
    place()
  }

  func willOpen(_ menu: NSMenu) {
    open = menu
  }

  func didClose(_ menu: NSMenu) {
    if open === menu { open = nil }
    place()
  }

  private func place() {
    guard let target = newest, row.menu !== target else { return }
    if let current = row.menu, current === open { return }
    row.menu?.removeItem(row)
    target.insertItem(row, at: 0)
  }
}
