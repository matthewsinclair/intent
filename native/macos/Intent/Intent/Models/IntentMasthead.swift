import Foundation

/// The values the menu's masthead row and its About page name that the app
/// gives rather than its bundle (issue 0607, Gtools' `GtoolsMasthead` for
/// Intent): the CLI's `{version}`, `{commit}` and `{kind}`, from `intent
/// version`, and the app's own stamps, `{app}` and `{tree}`, as `int macos
/// app-build` put them in the bundle. `{name}` is the bundle's: Intent.
/// Masthead/masthead.yaml and Masthead/about.md use them, and
/// Views/Menu/Masthead.swift makes the row.
enum IntentMasthead {
  /// Each value the app gives, named once: `owned` and `value` both read it.
  enum Value: String, CaseIterable {
    case version, commit, kind, app, tree
  }

  static let owned = Set(Value.allCases.map(\.rawValue))

  /// One value, asked at each open of the menu. The CLI's version is nil until
  /// it answers, so the row is the app's name alone, and "(version
  /// unavailable)" after a failure, whose cause is the row's tooltip; its
  /// commit and kind are nil until it answers, and after a failure.
  static func value(_ name: String, version: VersionState, bundle: Bundle) -> String? {
    guard let value = Value(rawValue: name) else { return nil }
    switch value {
    case .version: return version.answer?.version ?? (version.failure == nil ? nil : "(version unavailable)")
    case .commit: return version.answer?.commit
    case .kind: return version.answer?.kind
    case .app: return bundle.intentVersionString
    case .tree: return bundle.intentSourceTree
    }
  }
}
