import AppKit

/// Menubar rendering, the Console's palette, and the version string.
///
/// **THE PALETTE IS docs/design/design-system.md's DARK TOKENS (§3), TYPED BY
/// HAND** because the app carries no stylesheet: change them there first. The
/// Console is a dark window, so it takes the dark set whatever the system's
/// appearance. **The accent is steel, as that document's Decision A rules**, in
/// its dark-set value.
enum Theme {
  static let ground = rgb(0x0E1013)  // --bg
  static let ink = rgb(0xE6E8EC)  // --ink
  static let inkMuted = rgb(0x9AA1AC)  // --ink-muted
  static let accent = rgb(0x74A6DD)  // --accent, steel (Decision A)
  static let warning = rgb(0xC9973A)  // --warning
  static let error = rgb(0xE06767)  // --error

  /// Machine text is IBM Plex Mono (design-system.md §4) when it is installed,
  /// else the system's monospace, which is the document's own fallback genre.
  static func mono(_ size: CGFloat) -> NSFont {
    NSFont(name: "IBMPlexMono", size: size)
      ?? NSFont(name: "IBM Plex Mono", size: size)
      ?? NSFont.monospacedSystemFont(ofSize: size, weight: .regular)
  }

  private static func rgb(_ hex: UInt32) -> NSColor {
    NSColor(
      srgbRed: CGFloat((hex >> 16) & 0xFF) / 255,
      green: CGFloat((hex >> 8) & 0xFF) / 255,
      blue: CGFloat(hex & 0xFF) / 255,
      alpha: 1
    )
  }

  /// The tint for a health state, or nil for template rendering (the glyph
  /// adapts black/white to the bar). Live is green, the transient `stale` is
  /// orange, and `absent` and `unknown` are left template -- tinting the
  /// not-running state would read as an alarm for the ordinary state of a
  /// machine whose daemon is simply not up. Computed at paint time from the one predicate, with no
  /// cached `lastKnownState` that could outlive it (AC-01.8).
  static func menuBarTint(for health: Health) -> NSColor? {
    switch health {
    case .live: .systemGreen
    case .stale: .systemOrange
    case .absent, .unknown: nil
    }
  }
}

extension Bundle {
  /// "<latest tag> (build <commit count>, <short sha>)" from the stamps
  /// `int macos app-build` substitutes into Info.plist; "dev (build 0, dev)"
  /// for a build from inside Xcode. Nothing in the app displays it: there is no
  /// About item.
  var intentVersionString: String {
    let version = infoDictionary?["CFBundleShortVersionString"] as? String ?? "dev"
    let build = infoDictionary?["CFBundleVersion"] as? String ?? "0"
    let commit = infoDictionary?["IntentCommit"] as? String ?? "dev"
    return "\(version) (build \(build), \(commit))"
  }
}
