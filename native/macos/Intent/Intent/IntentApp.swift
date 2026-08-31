import AppKit
import OSLog
import ServiceManagement

/// Intent.app: the menubar item, the intent:// handler, and control of intentd.
/// Every action here runs an `intent` verb; every fact shown comes from the
/// daemon or the CLI, never from a Swift-side derivation (AC-01.1). The shape is
/// Geodica's AppDelegate, cut to intentd. The console and settings land as their
/// gated seams arrive: the console on cc's `intent daemon logs` verb (AC-01.4),
/// the intent:// wire-in on the resolver door (AC-01.5).
@main
@MainActor
final class AppDelegate: NSObject, NSApplicationDelegate {
  static let logger = Logger(subsystem: "com.matthewsinclair.intent.macos", category: "App")

  private var statusItem: NSStatusItem?
  private let daemon = DaemonService.shared
  private var observation: ContinuousObservation?

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

    let shell = LoginShellStore.shared.current()
    Self.logger.info(
      "launched; intent: \(shell.intent ?? "not found", privacy: .public) (\(shell.source, privacy: .public))"
    )

    daemon.startPolling()
    startObserving()  // renders the current state immediately, then on every change

    if !UserDefaults.standard.bool(forKey: Self.firstRunKey) {
      UserDefaults.standard.set(true, forKey: Self.firstRunKey)
      enableLaunchAtLoginByDefault()
    }
  }

  func applicationWillTerminate(_ notification: Notification) {
    daemon.stopPolling()
    observation?.stop()
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
  /// template-rendered so its tint follows the health predicate at paint time.
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

  // MARK: - Observation

  /// The icon and menu follow the daemon's health; ContinuousObservation re-arms
  /// after every change, so a single poll drives both.
  private func startObserving() {
    observation = ContinuousObservation(
      track: { [weak self] in
        guard let self else { return }
        _ = (self.daemon.health, self.daemon.busy)
      },
      onChange: { [weak self] in
        guard let self else { return }
        self.updateStatusIcon()
        self.rebuildMenu()
      }
    )
  }

  private func updateStatusIcon() {
    guard let button = statusItem?.button else { return }
    button.contentTintColor = Theme.menuBarTint(for: daemon.health)
    button.toolTip = statusSummary()
  }

  private func statusSummary() -> String {
    daemon.busy ?? daemon.health.summary
  }

  // MARK: - Menu

  /// Gated on the ONE health predicate. Start when absent, Stop/Restart when
  /// live; a stale daemon offers Restart to recover and NEVER an unlink -- the
  /// socket has an owner to investigate, not remove (AC-01.6 / AC-08.12), and
  /// the summary above names its pid.
  private func rebuildMenu() {
    let menu = NSMenu()

    let identity = NSMenuItem(title: "Intent", action: nil, keyEquivalent: "")
    identity.isEnabled = false
    identity.attributedTitle = NSAttributedString(
      string: "Intent",
      attributes: [.font: NSFont.boldSystemFont(ofSize: 13)]
    )
    menu.addItem(identity)

    let summary = NSMenuItem(title: statusSummary(), action: nil, keyEquivalent: "")
    summary.isEnabled = false
    menu.addItem(summary)
    menu.addItem(.separator())

    if let busy = daemon.busy {
      let item = NSMenuItem(title: busy, action: nil, keyEquivalent: "")
      item.isEnabled = false
      menu.addItem(item)
    } else {
      switch daemon.health {
      case .live:
        menu.addItem(
          NSMenuItem(title: "Stop intentd", action: #selector(stopDaemon), keyEquivalent: ""))
        menu.addItem(
          NSMenuItem(title: "Restart intentd", action: #selector(restartDaemon), keyEquivalent: ""))
      case .stale:
        menu.addItem(
          NSMenuItem(title: "Restart intentd", action: #selector(restartDaemon), keyEquivalent: ""))
      case .absent, .unknown:
        menu.addItem(
          NSMenuItem(title: "Start intentd", action: #selector(startDaemon), keyEquivalent: ""))
      }
    }

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

  /// AC-01.5. Hands the WHOLE address to the resolver via the pipe-safe
  /// `intent edit <address> --path` (cc's `9508788`), which realises the entity
  /// and prints its path; the app opens that and parses nothing itself.
  /// Addresses are `intent:///…` -- three slashes, an empty authority meaning
  /// *this project*; a bare number is refused until the ladder lands, so the URL
  /// scheme only ever delivers a full address here.
  private func openAddress(_ address: String) {
    Self.logger.info("open \(address, privacy: .public)")
    Task {
      do {
        let path = try await IntentCLI.run(["edit", address, "--path"])
          .trimmingCharacters(in: .whitespacesAndNewlines)
        guard !path.isEmpty else {
          return showAlert("Could not open \(address)", message: "the resolver returned no path")
        }
        NSWorkspace.shared.open(URL(fileURLWithPath: path))
      } catch {
        showAlert("Could not open \(address)", message: error.localizedDescription)
      }
    }
  }

  // MARK: - Actions

  @objc private func startDaemon() { runLifecycle("Start failed") { try await self.daemon.start() } }
  @objc private func stopDaemon() { runLifecycle("Stop failed") { try await self.daemon.stop() } }
  @objc private func restartDaemon() {
    runLifecycle("Restart failed") { try await self.daemon.restart() }
  }
  @objc private func runDoctorVerb() { runVerb(["doctor"], failing: "Doctor failed") }

  private func runLifecycle(_ failing: String, _ op: @escaping () async throws -> Void) {
    Task {
      do { try await op() } catch { showAlert(failing, message: error.localizedDescription) }
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
