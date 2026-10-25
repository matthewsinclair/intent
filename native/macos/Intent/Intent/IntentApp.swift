import AppKit
import OSLog
import ServiceManagement

/// Intent.app: the menubar item, the intent:// handler, the logs, and control of
/// intentd. Every daemon and project action here runs an `intent` verb; every
/// fact shown comes from the daemon or the CLI, never from a Swift-side
/// derivation (AC-01.1). The shape is Geodica's AppDelegate, cut to intentd. The
/// intent:// handler is wired below (AC-01.5). **The menu is Gtools' layout A,
/// with the controls Gtools and menuz use** (issue 0607; hv: "I want them all to
/// use the same controls the same way so that they look the same"): arca_swift's
/// masthead row on top, the intentd row and its flyout, and menuz's Log item,
/// which hands the logs to the macOS Console app. Every command the app runs is
/// recorded in its own log (`LogService`). There is no settings UI.
@main
@MainActor
final class AppDelegate: NSObject, NSApplicationDelegate, NSMenuDelegate {
  static let logger = AppLog.logger("App")

  private var statusItem: NSStatusItem?
  private let daemon = DaemonService.shared
  private let project = ProjectService.shared
  private let version = VersionService.shared
  private let log = LogService.shared
  private var observation: ContinuousObservation?
  /// The menu's top row, made once at launch and kept at the top of each menu
  /// the app builds (`MastheadPlacement`), as Gtools' is.
  private var masthead: MastheadPlacement?
  /// menuz's Log item in the current menu, set again as the menu opens.
  private var logItem: NSMenuItem?

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

    // The row reads its folder, ~/.config/intent, from launchd's environment,
    // the one every `intent` the app runs reads its own config with, as Gtools'
    // does.
    let version = self.version
    let row = IntentMasthead.row(
      state: { version.state }, log: { line in Self.logger.warning("masthead: \(line, privacy: .public)") })
    Self.drawsImage(row)
    masthead = MastheadPlacement(row: row)

    daemon.startPolling()
    project.startPolling()  // with a project set, one `intent graphql` read per poll (AC-01.3)
    version.startPolling()  // one `intent version` read per poll, for the masthead row
    log.reportUnwritable = { [weak self] said in self?.showAlert("Could not write the log", message: said) }
    Task { [log] in await log.findPaths() }  // where the Log item looks, from the CLI
    startObserving()  // renders the current state immediately, then on every change

    if !UserDefaults.standard.bool(forKey: Self.firstRunKey) {
      UserDefaults.standard.set(true, forKey: Self.firstRunKey)
      enableLaunchAtLoginByDefault()
    }
  }

  func applicationWillTerminate(_ notification: Notification) {
    daemon.stopPolling()
    project.stopPolling()
    version.stopPolling()
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

  /// The turtle -- slow and steady wins the race (AC-01.8). `MenuBarIcon` is the
  /// brand turtle rasterised from `docs/design/intent-logo.svg` by
  /// `int macos app-icons` (a template alpha silhouette, so its tint follows the
  /// health predicate at paint time); the system tortoise is a fallback only if
  /// the compiled catalogue is somehow absent.
  private static func menuBarImage() -> NSImage? {
    if let asset = NSImage(named: "MenuBarIcon") { return asset }
    return NSImage(systemSymbolName: "tortoise.fill", accessibilityDescription: "Intent")
  }

  /// Standard Edit menu so Cmd-C/V/X/A work in text fields, and File → Close for
  /// a window, as Gtools has them (an LSUIElement app has no menu bar of its own,
  /// but the responder chain still needs it).
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

    // File → Close (⌘W) reaches a window through the responder chain.
    let fileMenu = NSMenu(title: "File")
    fileMenu.addItem(
      NSMenuItem(title: "Close", action: #selector(NSWindow.performClose(_:)), keyEquivalent: "w"))
    let fileMenuItem = NSMenuItem(title: "File", action: nil, keyEquivalent: "")
    fileMenuItem.submenu = fileMenu

    mainMenu.addItem(fileMenuItem)
    mainMenu.addItem(editMenuItem)
    NSApp.mainMenu = mainMenu
  }

  // MARK: - Observation

  /// The icon and menu follow the daemon's health, any lifecycle verb in flight,
  /// the project's thread count, the CLI's version and the command running now;
  /// ContinuousObservation re-arms after every change, so a write from any poll
  /// repaints both.
  private func startObserving() {
    observation = ContinuousObservation(
      track: { [weak self] in
        guard let self else { return }
        _ = (self.daemon.health, self.daemon.busy, self.project.threadCount, self.version.state, self.log.running)
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

  /// The intentd row's two lines as one, for the icon's tooltip, so the two can
  /// never describe the daemon differently. See `Health.menuSummary`.
  private func statusSummary() -> String {
    daemon.health.menuSummary(busy: daemon.busy, threadCount: project.threadCount)
  }

  // MARK: - Menu

  /// hv's layout A (gtools#0108), as Gtools has it (issue 0607): under the
  /// masthead row, the intentd row, then one group each for the search, the
  /// tools and the app. A state is a row's second line, never part of its
  /// title, apart from the captions that stand in for a verb while it runs
  /// ("Restarting…", "Running Doctor…"), and no row below the masthead has an
  /// image.
  private func rebuildMenu() {
    let menu = NSMenu()

    // --- the masthead row (MastheadPlacement, at the end), then intentd ---
    menu.addItem(daemonItem())
    menu.addItem(.separator())

    // --- search ---
    menu.addItem(
      commandItem("Rebuild Search Index", running: "Rebuilding Search Index…", ["index", "rebuild"], #selector(rebuildSearchIndex)))
    menu.addItem(.separator())

    // --- the tools, where Gtools has Console… and Run Doctor ---
    // menuz's Log item, set as the menu opens; Run Doctor records its lines in
    // the app's log and opens it there, so a clean pass is seen.
    let logItem = NSMenuItem(title: Self.logTitle, action: #selector(openLog), keyEquivalent: "")
    self.logItem = logItem
    refreshLogItem()
    menu.addItem(logItem)
    menu.addItem(commandItem("Run Doctor", running: "Running Doctor…", ["doctor"], #selector(runDoctor)))
    menu.addItem(.separator())

    // --- the app ---
    menu.addItem(NSMenuItem(title: "Quit Intent", action: #selector(quit), keyEquivalent: "q"))

    menu.delegate = self
    statusItem?.menu = menu
    masthead?.row.toolTip = version.state.failure
    masthead?.installed(menu)
  }

  /// The intentd row, in the shape of Gtools' site row: `intentd` over its state
  /// as the row's second line (`Health.menuSubtitle`), and a flyout of Open,
  /// Start, Stop and Restart (`DaemonMenu`), gated on the ONE health predicate:
  /// a stale daemon offers Restart and NEVER an unlink -- the socket has an
  /// owner to investigate, not remove (AC-01.6 / AC-08.12), and the second line
  /// names its pid.
  ///
  /// **OPEN IS LIVE ONLY WHILE THE DAEMON PUBLISHES A WEB FACE.** `stale`,
  /// `absent` and `unknown` carry no `url` by the CLI's own iff-contract, so
  /// there is no page to open, and a lifecycle verb in flight holds it, because
  /// mid-restart is exactly when the old address is dead.
  private func daemonItem() -> NSMenuItem {
    let item = NSMenuItem(title: "intentd", action: nil, keyEquivalent: "")
    item.subtitle = daemon.health.menuSubtitle(busy: daemon.busy, threadCount: project.threadCount)
    let submenu = NSMenu()
    for entry in DaemonMenu.entries(busy: daemon.busy, health: daemon.health) {
      let row = NSMenuItem(title: entry.title, action: nil, keyEquivalent: "")
      if entry.isEnabled, let action = entry.action {
        row.action = selector(action)
      }
      row.isEnabled = row.action != nil
      submenu.addItem(row)
    }
    item.submenu = submenu
    return item
  }

  /// What each flyout entry runs.
  private func selector(_ action: DaemonMenu.Action) -> Selector {
    switch action {
    case .open: #selector(openWebFace)
    case .start: #selector(startDaemon)
    case .stop: #selector(stopDaemon)
    case .restart: #selector(restartDaemon)
    }
  }

  /// macOS 27 draws a menu item's image only when the item asks for it: the
  /// default visibility is automatic, and automatic drew no image in Gtools'
  /// menu or menuz's top row, so the masthead row asks for `.visible` (raw
  /// value 1 in NSMenuItem.h), as theirs do. It is set by key, once the item
  /// says it has the setter, because the property is new in the macOS 27 SDK.
  private static func drawsImage(_ item: NSMenuItem) {
    if item.responds(to: NSSelectorFromString("setPreferredImageVisibility:")) {
      item.setValue(1, forKey: "preferredImageVisibility")
    }
  }

  /// menuz's two titles for its Log item.
  static let logTitle = "Log"
  static let noLogTitle = logTitle + " (no log yet)"

  /// A command's item, or the caption that stands in for it while it runs, as
  /// Gtools' "Refreshing Search…" does. While another command runs the item
  /// stays live, and a click is refused naming the one running.
  private func commandItem(_ title: String, running caption: String, _ args: [String], _ action: Selector) -> NSMenuItem {
    guard log.running == IntentCLI.label(args) else {
      return NSMenuItem(title: title, action: action, keyEquivalent: "")
    }
    return NSMenuItem(title: caption, action: nil, keyEquivalent: "")
  }

  /// The Log item opens the logs, or says there is none yet, as menuz's does.
  /// Until the CLI has said where the logs are it stays live, so a click asks
  /// again and says why it cannot open them.
  private func refreshLogItem() {
    let none = log.logExists == false
    logItem?.title = none ? Self.noLogTitle : Self.logTitle
    logItem?.action = none ? nil : #selector(openLog)
  }

  /// The masthead row stays in a menu while it is open, as Gtools' does, and
  /// the Log item reads the logs' presence as the menu opens, as menuz's does.
  func menuWillOpen(_ menu: NSMenu) {
    masthead?.willOpen(menu)
    refreshLogItem()
  }

  /// A menu rebuilt while this one was open took no masthead row, so the row
  /// moves to it as this one closes.
  func menuDidClose(_ menu: NSMenu) {
    masthead?.didClose(menu)
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
  /// *this project*. LaunchServices delivers only `intent:` URLs, so `edit`
  /// always receives a full address here and never a bare number.
  private func openAddress(_ address: String) {
    Self.logger.info("open \(address, privacy: .public)")
    Task {
      do {
        let path = try await IntentCLI.run(["edit", address, "--path"])
          .trimmingCharacters(in: .whitespacesAndNewlines)
        guard !path.isEmpty else {
          return showAlert("Could not open \(address)", message: "the resolver returned no path")
        }
        Self.logger.info("resolved \(address, privacy: .public) -> \(path, privacy: .public)")
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
  @objc private func runDoctor() { runLogged(["doctor"]) }
  @objc private func rebuildSearchIndex() { runLogged(["index", "rebuild"]) }

  /// menuz's Log item: intentd's logs and the app's own, in the macOS Console
  /// app. A failure is an alert, never a click that did nothing.
  @objc private func openLog() {
    Task {
      do {
        try await log.openAll()
      } catch {
        Self.logger.error("could not open the logs: \(error.localizedDescription, privacy: .public)")
        showAlert("Could not open the log", message: error.localizedDescription)
      }
    }
  }

  /// Open the daemon's web face through the CLI's `intent estate status
  /// --browser` (issue 0646), the one builder of the page's address: it reads the
  /// address and token the running daemon published and opens the page with the
  /// token in the fragment, so the page can read the estate. The app opening a
  /// bare address of its own landed on the page's token refusal.
  ///
  /// **A FAILURE IS AN ALERT**, never a click that did nothing: the CLI's
  /// own error and remedy are what the operator reads.
  @objc private func openWebFace() {
    let args = DaemonMenu.openCommand
    Self.logger.info("open web face: \(IntentCLI.label(args), privacy: .public)")
    Task {
      do {
        _ = try await IntentCLI.run(args)
      } catch {
        let reason = error.localizedDescription
        Self.logger.error("could not open the web face: \(reason, privacy: .public)")
        showAlert("Could not open the web face", message: reason)
      }
    }
  }

  private func runLifecycle(_ failing: String, _ op: @escaping () async throws -> Void) {
    Task {
      do { try await op() } catch { showAlert(failing, message: error.localizedDescription) }
    }
  }

  /// Runs the command, records it in the app's log and opens that log in
  /// Console, so a clean pass is seen as well as a failing one (ST0075's
  /// trigger). An alert is raised when the command could not run at all -- no
  /// binary, no project, or another command still running -- and when the log
  /// cannot be opened; a record that cannot be written raises its own
  /// (`LogService.reportUnwritable`). A non-zero exit is the command's own lines
  /// to read.
  private func runLogged(_ args: [String]) {
    Task {
      let kept: String?
      do {
        kept = try await log.run(args)
      } catch {
        return showAlert("Could not run \(IntentCLI.label(args))", message: error.localizedDescription)
      }
      guard let kept else { return }
      do {
        try await log.open([kept])
      } catch {
        showAlert("Could not open the log", message: error.localizedDescription)
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
