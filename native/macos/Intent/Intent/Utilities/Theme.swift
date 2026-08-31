import AppKit

/// Menubar rendering and the version string. The full brand palette lands with
/// the console and settings styling; the spine needs only the state tint.
enum Theme {
  /// The tint for a health state, or nil for template rendering (the glyph
  /// adapts black/white to the bar). Live is coloured, the transient `stale`
  /// is orange, and `absent` is left template -- tinting the not-running state
  /// would read as an alarm for the ordinary state of a machine whose daemon
  /// is simply not up. Computed at paint time from the one predicate, with no
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
  /// "0.2.0 (build 143, 6df55d5)" from the stamps `int macos app-build`
  /// substitutes into Info.plist; "dev" for a build from inside Xcode.
  var intentVersionString: String {
    let version = infoDictionary?["CFBundleShortVersionString"] as? String ?? "dev"
    let build = infoDictionary?["CFBundleVersion"] as? String ?? "0"
    let commit = infoDictionary?["IntentCommit"] as? String ?? "dev"
    return "\(version) (build \(build), \(commit))"
  }
}
