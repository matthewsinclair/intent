import AppKit
import OSLog
import ServiceManagement

/// Intent.app: the menubar item, the intent:// handler, and control of intentd.
/// Every action here runs an `intent` verb; every fact shown comes from the
/// daemon or the CLI, never from a Swift-side derivation (AC-01.1). The shape
/// is Geodica's AppDelegate, cut to intentd. The health display, the console
/// and settings land as their gated seams arrive: cc's machine-readable
/// `daemon status` for the LIVE/STALE/ABSENT indicator (AC-01.2 / AC-01.6), and
/// the log-source ruling for the console (AC-01.4).
@main
@MainActor
final class AppDelegate: NSObject, NSApplicationDelegate {
  static let logger = Logger(subsystem: "com.matthewsinclair.intent.macos", category: "App")

  private var statusItem: NSStatusItem?

  static let firstRunKey = "FirstRunDone"

  static func main() {
    let app = NSApplication.shared
    let delegate = AppDelegate()
    app.delegate = delegate
    app.run()
  }

  func applicationDidFinishLaunching(_ notification: Notification) {
    setupMainMenu()

    statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
    if let button = statusItem?.button {
      let image = Self.menuBarImage()
      image?.isTemplate = true
      button.image = image
      if image == nil { button.title = "◆" }  // never a zero-width item
    }
    rebuildMenu()

    let shell = LoginShellStore.shared.current()
    Self.logger.info(
      "launched; intent: \(shell.intent ?? "not found", privacy: .public) (\(shell.source, privacy: .public))"
    )

    if !UserDefaults.standard.bool(forKey: Self.firstRunKey) {
      UserDefaults.standard.set(true, forKey: Self.firstRunKey)
      enableLaunchAtLoginByDefault()
    }
  }

  /// Registered as a login item on first run so the menubar comes back after a
  /// reboot. Non-fatal, but never silent.
  private func enableLaunchAtLoginByDefault() {
    guard SMAppService.mainApp.status != .enabled else { return }
    do {
      try SMAppService.mainApp.register()
    } catch {
      Self.logger.error(
        "launch-at-login registration failed: \(error.localizedDescription, privacy: .public)")
    }
  }

  /// The turtle -- slow and steady wins the race (AC-01.8). The rasterised asset
  /// lands with that row; until then the system tortoise stands in,
  /// template-rendered so it adapts black/white to the bar. Its STATE semantics
  /// arrive with the health predicate, derived at paint time and never cached.
  private static func menuBarImage() -> NSImage? {
    if let asset = NSImage(named: "MenuBarIcon") { return asset }
    return NSImage(systemSymbolName: "tortoise.fill", accessibilityDescription: "Intent")
  }

  /// Standard Edit menu so Cmd-C/V/X/A work in text fields (an LSUIElement app
  /// has no menu bar of its own, but the responder chain still needs it).
  private func setupMainMenu() {
    let mainMenu = NSMenu()
    let editMenu = NSMenu(title: "Edit")
    editMenu.addItem(NSMenuItem(title: "Cut", action: #selector(NSText.cut(_:)), keyEquivalent: "x"))
    editMenu.addItem(
      NSMenuItem(title: "Copy", action: #selector(NSText.copy(_:)), keyEquivalent: "c"))
    editMenu.addItem(
      NSMenuItem(title: "Paste", action: #selector(NSText.paste(_:)), keyEquivalent: "v"))
    editMenu.addItem(
      NSMenuItem(title: "Select All", action: #selector(NSText.selectAll(_:)), keyEquivalent: "a"))
    let editMenuItem = NSMenuItem(title: "Edit", action: nil, keyEquivalent: "")
    editMenuItem.submenu = editMenu
    mainMenu.addItem(editMenuItem)
    NSApp.mainMenu = mainMenu
  }

  // MARK: - Menu

  /// A static control menu until cc's machine-readable `daemon status` lands;
  /// then Start / Stop / Restart gate on LIVE / STALE / ABSENT and the socket
  /// affordance appears only on ABSENT, never on STALE (AC-01.6).
  private func rebuildMenu() {
    let menu = NSMenu()

    let identity = NSMenuItem(title: "Intent", action: nil, keyEquivalent: "")
    identity.isEnabled = false
    identity.attributedTitle = NSAttributedString(
      string: "Intent",
      attributes: [.font: NSFont.boldSystemFont(ofSize: 13)]
    )
    menu.addItem(identity)
    menu.addItem(.separator())

    menu.addItem(NSMenuItem(title: "Start intentd", action: #selector(startDaemon), keyEquivalent: ""))
    menu.addItem(NSMenuItem(title: "Stop intentd", action: #selector(stopDaemon), keyEquivalent: ""))
    menu.addItem(
      NSMenuItem(title: "Restart intentd", action: #selector(restartDaemon), keyEquivalent: ""))
    menu.addItem(NSMenuItem(title: "Run Doctor", action: #selector(runDoctorVerb), keyEquivalent: ""))
    menu.addItem(.separator())

    menu.addItem(NSMenuItem(title: "Quit Intent", action: #selector(quit), keyEquivalent: "q"))

    statusItem?.menu = menu
  }

  // MARK: - intent://

  /// LaunchServices delivers every intent:// URL here -- the app is the
  /// registered handler -- and the handler hands the WHOLE address to the one
  /// resolver, opening what it names (AC-01.5). The app parses no address
  /// itself: it is a client of the addressing rules, never a second resolver.
  func application(_ application: NSApplication, open urls: [URL]) {
    for url in urls {
      openAddress(url.absoluteString)
    }
  }

  /// AC-01.5, gated. The resolver door -- a pipe-safe CLI verb that accepts an
  /// `intent://` URL and yields the entity to open -- is the URI-uniformity
  /// work, in flight (cc's narrow-door promote-then-narrow). `intent explore`
  /// accepts the URL but needs a terminal a `.app` cannot give it, and
  /// `intent edit` takes <kind> <id>, not a URL, until those doors accept the
  /// scheme. So this holds rather than parsing the address here -- which the
  /// row forbids -- and wires to that door the moment it lands.
  private func openAddress(_ address: String) {
    Self.logger.info(
      "intent:// open \(address, privacy: .public) -- awaiting the address-resolver door")
  }

  // MARK: - Actions

  @objc private func startDaemon() { runVerb(["daemon", "start"], failing: "Start failed") }
  @objc private func stopDaemon() { runVerb(["daemon", "stop"], failing: "Stop failed") }
  @objc private func runDoctorVerb() { runVerb(["doctor"], failing: "Doctor failed") }

  /// intentd has no `restart` verb; the order -- stop, then start -- is the
  /// CLI's to own, not the app's to invent.
  @objc private func restartDaemon() {
    Task {
      do {
        _ = try await IntentCLI.capture(["daemon", "stop"])
        _ = try await IntentCLI.run(["daemon", "start"])
      } catch {
        showAlert("Restart failed", message: error.localizedDescription)
      }
    }
  }

  private func runVerb(_ args: [String], failing: String) {
    Task {
      do {
        _ = try await IntentCLI.run(args)
      } catch {
        showAlert(failing, message: error.localizedDescription)
      }
    }
  }

  @objc private func quit() {
    NSApp.terminate(nil)
  }

  // MARK: - Helpers

  private func showAlert(_ title: String, message: String) {
    let alert = NSAlert()
    alert.messageText = title
    alert.informativeText = message
    alert.alertStyle = .warning
    alert.runModal()
  }
}
