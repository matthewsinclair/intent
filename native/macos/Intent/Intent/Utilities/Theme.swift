import AppKit

/// Menubar rendering, and the app's own stamps. It carries no palette: nothing
/// in the app draws text of its own.
enum Theme {
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
  /// for a build from inside Xcode. The About page shows it as `{app}`
  /// (`IntentMasthead`), as Gtools' shows its own.
  var intentVersionString: String {
    let version = infoDictionary?["CFBundleShortVersionString"] as? String ?? "dev"
    let build = infoDictionary?["CFBundleVersion"] as? String ?? "0"
    let commit = infoDictionary?["IntentCommit"] as? String ?? "dev"
    return "\(version) (build \(build), \(commit))"
  }

  /// The source tree the app was built from, as `int macos app-build` stamps
  /// it; "dev" for a build from inside Xcode. The About page's `{tree}`.
  var intentSourceTree: String {
    infoDictionary?["IntentSourceTree"] as? String ?? "dev"
  }
}
