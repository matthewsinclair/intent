import AppKit
import Foundation
import Testing

@testable import Intent

/// A test's stand-ins for what the row asks and tells: the CLI's version, which
/// the test changes, and the refusals the row logs.
@MainActor
private final class World {
  var state: VersionState = .pending
  var logged: [String] = []
}

/// Issue 0607: the menu's masthead row, arca_swift's MastheadRow, made by the
/// app's one function for it, as gtools#0106's tests pin Gtools'. These pin the
/// values the app gives it, the files it reads from the app's bundle and from a
/// user's folder, and where it sits as the app builds menus. The host is the
/// built app, so its bundle is the one that ships.
@MainActor
@Suite("Masthead")
struct MastheadTests {
  /// A folder of the test's own for the row's config folder to sit under, so a
  /// copy in the user's ~/.config/intent cannot change the answer.
  private func folder() -> URL {
    FileManager.default.temporaryDirectory.appendingPathComponent("intent-masthead-\(UUID().uuidString)")
  }

  private func row(_ world: World, under folder: URL) -> NSMenuItem {
    IntentMasthead.row(
      state: { world.state }, environment: ["XDG_CONFIG_HOME": folder.path], log: { world.logged.append($0) })
  }

  /// The row reads its files and values again each time its menu begins
  /// tracking, which is what opening the menu posts.
  private func open(_ menu: NSMenu) {
    NotificationCenter.default.post(name: NSMenu.didBeginTrackingNotification, object: menu)
  }

  private func statusMenu() -> NSMenu {
    let menu = NSMenu()
    menu.addItem(NSMenuItem(title: "status", action: nil, keyEquivalent: ""))
    return menu
  }

  @Test("each value is named once, and the CLI's three follow its answer")
  func values() {
    #expect(IntentMasthead.owned == ["version", "commit", "kind", "app", "tree"])
    let answered = VersionState.answered(CLIVersion(version: "3.2.4", commit: "f89b8285", kind: "release"))
    #expect(IntentMasthead.value("version", version: answered, bundle: .main) == "3.2.4")
    #expect(IntentMasthead.value("commit", version: answered, bundle: .main) == "f89b8285")
    #expect(IntentMasthead.value("kind", version: answered, bundle: .main) == "release")
    #expect(IntentMasthead.value("version", version: .pending, bundle: .main) == nil)
    #expect(IntentMasthead.value("commit", version: .pending, bundle: .main) == nil)
    #expect(IntentMasthead.value("kind", version: .pending, bundle: .main) == nil)
    let failed = VersionState.failed("intent was not found on the login shell's PATH")
    #expect(IntentMasthead.value("version", version: failed, bundle: .main) == "(version unavailable)")
    #expect(IntentMasthead.value("commit", version: failed, bundle: .main) == nil)
    #expect(IntentMasthead.value("menu", version: answered, bundle: .main) == nil)
  }

  @Test("the app's own stamps come from the bundle's Info.plist, as the About page shows them")
  func stamps() throws {
    let contents = folder().appendingPathComponent("Stamped.bundle/Contents")
    try FileManager.default.createDirectory(at: contents, withIntermediateDirectories: true)
    let info: [String: Any] = [
      "CFBundleShortVersionString": "3.2.4", "CFBundleVersion": "2100", "IntentCommit": "f89b8285",
      "IntentSourceTree": "4f2c9e1b",
    ]
    let plist = try PropertyListSerialization.data(fromPropertyList: info, format: .xml, options: 0)
    try plist.write(to: contents.appendingPathComponent("Info.plist"))
    let bundle = try #require(Bundle(url: contents.deletingLastPathComponent()))
    #expect(IntentMasthead.value("app", version: .pending, bundle: bundle) == "3.2.4 (build 2100, f89b8285)")
    #expect(IntentMasthead.value("tree", version: .pending, bundle: bundle) == "4f2c9e1b")
  }

  @Test("the app's bundle carries the row's file, its About page, the page's stylesheet and its picture")
  func bundled() {
    for (name, type) in [("masthead", "yaml"), ("about", "md"), ("about", "css"), ("intent-logo", "png")] {
      #expect(Bundle.main.url(forResource: name, withExtension: type) != nil, "\(name).\(type)")
    }
  }

  @Test("the one row is the app's name until the CLI answers, then its version over its commit and kind, read again at each open")
  func readsAtEachOpen() {
    let world = World()
    let item = row(world, under: folder())
    #expect(item.title == "Intent")
    #expect(item.subtitle == nil)
    let menu = statusMenu()
    menu.insertItem(item, at: 0)
    world.state = .answered(CLIVersion(version: "3.2.4", commit: "f89b8285", kind: "release"))
    #expect(item.title == "Intent")
    open(menu)
    #expect(item.title == "Intent 3.2.4")
    #expect(item.subtitle == "f89b8285 · release")
    world.state = .failed("intent was not found on the login shell's PATH")
    open(menu)
    #expect(item.title == "Intent (version unavailable)")
    #expect(item.subtitle == nil)
    #expect(world.logged.isEmpty)
  }

  /// An `intent` before 3.2.1 prints no kind, and each of the row's lines
  /// stands on its own: the subheading waits for both its values rather than
  /// showing half of itself.
  @Test("a CLI that names no kind leaves the subheading hidden rather than half shown")
  func noKind() {
    let world = World()
    world.state = .answered(CLIVersion(version: "3.0.1", commit: "8a48430e", kind: nil))
    let item = row(world, under: folder())
    #expect(item.title == "Intent 3.0.1")
    #expect(item.subtitle == nil)
  }

  /// Issue 0646: the line the dev pair printed when hv read the row, through
  /// the parser to the row. Version over commit and kind, as Gtools' and
  /// menuz's read; the tree's root is parsed and never shown.
  @Test("the 3.2.7 version line gives the row its version over its commit and kind, and no root")
  func treeLine() throws {
    let world = World()
    let parsed = try #require(
      VersionService.parse("intent 3.2.6 (dev, 397d7568e1d8) at /Users/matts/Devel/prj/Intent dev\n"))
    world.state = .answered(parsed)
    let item = row(world, under: folder())
    #expect(item.title == "Intent 3.2.6")
    #expect(item.subtitle == "397d7568 · dev")
  }

  /// The positive control for the folder: the row reads the user's copy, and a
  /// copy that fails its checks is refused, logged, and leaves the bundle's.
  @Test("a masthead.yaml in the user's folder replaces the bundle's, and a broken one is refused, logged, and leaves the bundle's")
  func usersCopy() throws {
    let world = World()
    world.state = .answered(CLIVersion(version: "3.2.4", commit: "f89b8285", kind: "release"))
    let base = folder()
    let mine = base.appendingPathComponent("intent")
    try FileManager.default.createDirectory(at: mine, withIntermediateDirectories: true)
    defer { try? FileManager.default.removeItem(at: base) }
    try Data("heading: \"Mine {version}\"\nabout:\n  body: about.md\n".utf8)
      .write(to: mine.appendingPathComponent("masthead.yaml"))
    let item = row(world, under: base)
    #expect(item.title == "Mine 3.2.4")
    #expect(item.subtitle == nil)
    #expect(world.logged.isEmpty)
    try Data("heading: [\n".utf8).write(to: mine.appendingPathComponent("masthead.yaml"))
    let menu = statusMenu()
    menu.insertItem(item, at: 0)
    open(menu)
    #expect(item.title == "Intent 3.2.4")
    #expect(world.logged.count == 1)
  }

  @Test("each menu the app installs takes the row at its top, the same item, out of the one before")
  func moves() {
    let item = NSMenuItem(title: "row", action: nil, keyEquivalent: "")
    let placement = MastheadPlacement(row: item)
    let first = statusMenu()
    let second = statusMenu()
    placement.installed(first)
    #expect(first.items.first === item)
    placement.installed(second)
    #expect(second.items.first === item)
    #expect(second.items.count == 2)
    #expect(!first.items.contains(item))
  }

  /// Moving the row out of the menu on screen would take the top row away
  /// under the pointer, so it waits for that menu to close.
  @Test("the row stays in the open menu while newer ones are installed, and moves to the newest when it closes")
  func staysInTheOpenMenu() {
    let item = NSMenuItem(title: "row", action: nil, keyEquivalent: "")
    let placement = MastheadPlacement(row: item)
    let shown = statusMenu()
    let newer = statusMenu()
    let newest = statusMenu()
    placement.installed(shown)
    placement.willOpen(shown)
    placement.installed(newer)
    placement.installed(newest)
    #expect(item.menu === shown)
    #expect(newer.items.count == 1)
    placement.didClose(shown)
    #expect(newest.items.first === item)
    #expect(!shown.items.contains(item))
  }
}
