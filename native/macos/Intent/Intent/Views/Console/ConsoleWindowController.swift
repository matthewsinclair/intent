import AppKit

/// The Console: a dark monospaced view over `intent daemon logs --follow`,
/// styled from the design system's dark tokens the way a terminal window is.
/// Footer: Follow, Clear, Copy, and the files being tailed. The tail runs while
/// the window is visible. Copied from Gtools' ConsoleWindowController (ST0075:
/// copied, not shared), without its Verbose box, because intentd writes no
/// request lines to filter.
@MainActor
final class ConsoleWindowController: NSWindowController, NSWindowDelegate, ConsoleSink {
  private static let frameAutosaveName = "IntentConsole"

  private let runner = ConsoleRunner.shared
  private let scrollView: NSScrollView
  private let textView: NSTextView
  private let followBox = NSButton(checkboxWithTitle: "Follow", target: nil, action: nil)
  private let pathLabel = NSTextField(labelWithString: "")
  private var observation: ContinuousObservation?

  /// Auto-scroll; cleared when the user scrolls up, set again at the bottom.
  private var follow = true

  init() {
    let scroll = NSTextView.scrollableTextView()
    guard let text = scroll.documentView as? NSTextView else {
      preconditionFailure("NSTextView.scrollableTextView() carries an NSTextView as its document view")
    }
    scrollView = scroll
    textView = text

    let window = NSWindow(
      contentRect: NSRect(x: 0, y: 0, width: 920, height: 520),
      styleMask: [.titled, .closable, .miniaturizable, .resizable],
      backing: .buffered,
      defer: false
    )
    super.init(window: window)

    window.title = "Intent Console"
    window.appearance = NSAppearance(named: .darkAqua)
    window.backgroundColor = Theme.ground
    window.minSize = NSSize(width: 520, height: 240)
    window.isReleasedWhenClosed = false
    window.delegate = self
    window.setFrameAutosaveName(Self.frameAutosaveName)
    if !window.setFrameUsingName(Self.frameAutosaveName) {
      window.center()
    }

    build(in: window)
  }

  required init?(coder: NSCoder) {
    fatalError("init(coder:) not implemented")
  }

  // MARK: - Showing

  /// **OPENING STARTS THE TAIL, AND THE START RENDERS THE BACKLOG** through
  /// `consoleDidReset`, whether or not the verb launches.
  func show() {
    guard let window else { return }
    let opening = !window.isVisible
    runner.sink = self
    if runner.tailing {
      if opening { renderBacklog() }
    } else {
      runner.startTail()
    }
    window.makeKeyAndOrderFront(nil)
    NSApp.activate()
    observe()
  }

  func toggle() {
    if let window, window.isVisible {
      window.performClose(nil)
    } else {
      show()
    }
  }

  nonisolated func windowWillClose(_ notification: Notification) {
    MainActor.assumeIsolated {
      runner.stopTail()
      runner.sink = nil
      observation?.stop()
      observation = nil
    }
  }

  // MARK: - Layout

  private func build(in window: NSWindow) {
    scrollView.translatesAutoresizingMaskIntoConstraints = false
    scrollView.hasVerticalScroller = true
    scrollView.autohidesScrollers = true
    scrollView.borderType = .noBorder
    scrollView.drawsBackground = true
    scrollView.backgroundColor = Theme.ground

    textView.isEditable = false
    textView.isSelectable = true
    textView.isRichText = false
    textView.usesFindBar = true
    textView.drawsBackground = true
    textView.backgroundColor = Theme.ground
    textView.textColor = Theme.ink
    textView.insertionPointColor = Theme.ink
    textView.font = Theme.mono(12)
    textView.textContainerInset = NSSize(width: 10, height: 8)
    textView.isAutomaticQuoteSubstitutionEnabled = false
    textView.isAutomaticLinkDetectionEnabled = false
    textView.layoutManager?.allowsNonContiguousLayout = true

    // --- footer ---
    followBox.target = self
    followBox.action = #selector(toggleFollow(_:))
    followBox.toolTip = "Keep the newest line in view"
    followBox.state = .on

    let clear = NSButton(title: "Clear", target: self, action: #selector(clearConsole(_:)))
    clear.toolTip = "⌘K"
    let copy = NSButton(title: "Copy", target: self, action: #selector(copyAll(_:)))
    copy.toolTip = "Copy everything in the Console"
    for button in [clear, copy] {
      button.bezelStyle = .rounded
      button.controlSize = .small
    }

    pathLabel.font = Theme.mono(11)
    pathLabel.textColor = Theme.inkMuted
    pathLabel.lineBreakMode = .byTruncatingMiddle
    pathLabel.setContentCompressionResistancePriority(.defaultLow, for: .horizontal)

    let spacer = NSView()
    spacer.setContentHuggingPriority(.defaultLow, for: .horizontal)

    let footer = NSStackView(views: [followBox, pathLabel, spacer, clear, copy])
    footer.translatesAutoresizingMaskIntoConstraints = false
    footer.orientation = .horizontal
    footer.alignment = .centerY
    footer.spacing = 12
    footer.edgeInsets = NSEdgeInsets(top: 6, left: 12, bottom: 6, right: 12)

    let rule = NSBox()
    rule.boxType = .separator
    rule.translatesAutoresizingMaskIntoConstraints = false

    let root = NSView()
    root.addSubview(scrollView)
    root.addSubview(rule)
    root.addSubview(footer)

    NSLayoutConstraint.activate([
      scrollView.topAnchor.constraint(equalTo: root.topAnchor),
      scrollView.leadingAnchor.constraint(equalTo: root.leadingAnchor),
      scrollView.trailingAnchor.constraint(equalTo: root.trailingAnchor),
      scrollView.bottomAnchor.constraint(equalTo: rule.topAnchor),
      rule.leadingAnchor.constraint(equalTo: root.leadingAnchor),
      rule.trailingAnchor.constraint(equalTo: root.trailingAnchor),
      rule.bottomAnchor.constraint(equalTo: footer.topAnchor),
      footer.leadingAnchor.constraint(equalTo: root.leadingAnchor),
      footer.trailingAnchor.constraint(equalTo: root.trailingAnchor),
      footer.bottomAnchor.constraint(equalTo: root.bottomAnchor),
    ])
    window.contentView = root

    NotificationCenter.default.addObserver(
      self, selector: #selector(userScrolled(_:)),
      name: NSScrollView.didLiveScrollNotification, object: scrollView
    )
  }

  private func observe() {
    guard observation == nil else { return }
    observation = ContinuousObservation { [weak self] in
      guard let self else { return }
      _ = (self.runner.header, self.runner.tailing)
    } onChange: { [weak self] in
      self?.refreshFooter()
    }
  }

  /// The footer names the files being tailed, from the verb's own first line,
  /// so the app derives no log path of its own (AC-02.1).
  private func refreshFooter() {
    guard runner.tailing else {
      pathLabel.stringValue = "not tailing"
      return
    }
    switch runner.header {
    case .awaiting:
      pathLabel.stringValue = "starting the tail"
    case .logs(let logs):
      let paths = [logs.log, logs.errLog].map { ($0 as NSString).abbreviatingWithTildeInPath }
      pathLabel.stringValue = "tailing " + paths.joined(separator: " and ")
    case .unrecognised:
      pathLabel.stringValue = "tailing, but the verb's first line did not name its files"
    }
  }

  // MARK: - Rendering

  /// The whole backlog as one text, set at once.
  private func renderBacklog() {
    let text = NSMutableAttributedString()
    for line in runner.ring.lines {
      text.append(Self.rendered(line))
    }
    textView.textStorage?.setAttributedString(text)
    scrollToEnd()
  }

  private func appendText(_ line: ConsoleLine) {
    textView.textStorage?.append(Self.rendered(line))
  }

  private static func rendered(_ line: ConsoleLine) -> NSAttributedString {
    NSAttributedString(string: line.text + "\n", attributes: attributes(for: line.kind))
  }

  private static func attributes(for kind: ConsoleLine.Kind) -> [NSAttributedString.Key: Any] {
    let color: NSColor =
      switch kind {
      case .log: Theme.ink
      case .error: Theme.error
      case .warning: Theme.warning
      case .marker: Theme.accent
      }
    return [.font: Theme.mono(12), .foregroundColor: color]
  }

  private func scrollToEnd() {
    textView.scrollToEndOfDocument(nil)
  }

  private var atBottom: Bool {
    let visible = scrollView.contentView.bounds
    return visible.maxY >= textView.frame.maxY - 4
  }

  // MARK: - ConsoleSink

  func consoleDidAppend(_ line: ConsoleLine) {
    appendText(line)
    if follow { scrollToEnd() }
  }

  func consoleDidTrim(_ count: Int) {
    guard let storage = textView.textStorage, count > 0 else { return }
    let text = storage.string as NSString
    var cut = 0
    var found = 0
    while found < count {
      let newline = text.range(of: "\n", options: [], range: NSRange(location: cut, length: text.length - cut))
      guard newline.location != NSNotFound else {
        cut = text.length
        break
      }
      cut = newline.location + 1
      found += 1
    }
    storage.deleteCharacters(in: NSRange(location: 0, length: cut))
  }

  func consoleDidReset() {
    renderBacklog()
  }

  // MARK: - Actions

  @objc private func toggleFollow(_ sender: NSButton) {
    follow = sender.state == .on
    if follow { scrollToEnd() }
  }

  @objc private func userScrolled(_ note: Notification) {
    follow = atBottom
    followBox.state = follow ? .on : .off
  }

  /// Also the main menu's View → Clear Console (⌘K), through the responder chain.
  @objc func clearConsole(_ sender: Any?) {
    runner.clear()
  }

  @objc private func copyAll(_ sender: Any?) {
    NSPasteboard.general.clearContents()
    NSPasteboard.general.setString(textView.string, forType: .string)
  }
}
