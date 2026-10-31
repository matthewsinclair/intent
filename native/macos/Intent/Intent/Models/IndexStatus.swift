import Foundation

/// The menu's Rebuild Search Index row (ST0090 WP-02), which is Gtools' Refresh
/// Search row file for file (`CmsStatus.refreshTitle`, `refreshSubtitle` and
/// `uptimeText`; gtools#0066, gtools#0108), because the menubar apps use one set
/// of controls the same way (hv, 2026-10-22). `IndexStatus` is the one fact the
/// row reads from `intent index status --json`: when the index was last
/// reconciled, as the store stamped it.
struct IndexStatus: Decodable, Sendable, Equatable {
  /// The store's `reconciled_at` (WP-01): nil when no reconcile has stamped the
  /// index, and absent from a CLI that predates it, which the row reads as no
  /// second line rather than an age the CLI never named.
  let reconciledAt: String?

  private enum CodingKeys: String, CodingKey {
    case reconciledAt = "reconciled_at"
  }

  static func decode(_ data: Data) throws -> IndexStatus {
    try JSONDecoder().decode(IndexStatus.self, from: data)
  }

  /// The verb the row runs, and the read that names its stamp.
  static let rebuildCommand = ["index", "rebuild"]
  static let statusCommand = ["index", "status", "--json"]

  /// The alert a rebuild that fails, or cannot be run, raises, as Gtools'
  /// refresh raises `Refresh failed`.
  static let rebuildFailedTitle = "Rebuild failed"

  /// The row's title, and the busy caption that stands in for it while the
  /// rebuild runs, as Gtools' "Refreshing Search…" does. Neither changed.
  static func rebuildTitle(rebuilding: Bool) -> String {
    rebuilding ? "Rebuilding Search Index…" : "Rebuild Search Index"
  }

  /// The row's second line: how long ago the index was reconciled, in Gtools'
  /// age words, when the CLI names a stamp, and nil when it does not.
  ///
  /// **THE VERB IS THE INDEX'S OWN, NOT GTOOLS' `built`.** intentd's watcher
  /// stamps the same column after every source edit (`Facade::index_refresh`),
  /// so `built` would claim a rebuild that did not happen. The age is the CLI's
  /// stamp against `now`, never the app's own record of its runs, and never
  /// negative, so a clock behind the store's reads `0s`.
  static func rebuildSubtitle(status: IndexStatus?, now: Date) -> String? {
    guard let reconciled = date(status?.reconciledAt) else { return nil }
    return "reconciled \(ageText(seconds: max(0, Int(now.timeIntervalSince(reconciled))))) ago"
  }

  /// Gtools' age words (`CmsStatus.uptimeText`): "1d 4h", "2h 3m", "5m", "12s".
  static func ageText(seconds: Int) -> String {
    let d = seconds / 86_400
    let h = (seconds % 86_400) / 3_600
    let m = (seconds % 3_600) / 60
    if d > 0 { return "\(d)d \(h)h" }
    if h > 0 { return "\(h)h \(m)m" }
    if m > 0 { return "\(m)m" }
    return "\(seconds)s"
  }

  /// The one parse of the stamp, in the one form the store writes it,
  /// `strftime('%Y-%m-%dT%H:%M:%fZ')`: UTC with milliseconds. Nil when the CLI
  /// names none or the form is not that one.
  static func date(_ iso: String?) -> Date? {
    iso.flatMap { try? Date.ISO8601FormatStyle(includingFractionalSeconds: true).parse($0) }
  }

  /// What the alert says for a rebuild that exited non-zero: the CLI's own
  /// `error:`, `caused by:`, `warning:` and `remedy:` lines as it printed them
  /// (`LogBlock.isAlarm`'s tokens), stderr first, or its exit when it printed
  /// none. The app's log keeps every line either way.
  static func failure(_ result: CLIRunResult) -> String {
    let said = [result.stderr, result.stdout]
      .flatMap { $0.split(separator: "\n").map(String.init) }
      .filter(LogBlock.isAlarm)
    guard said.isEmpty else { return said.joined(separator: "\n") }
    return "`\(IntentCLI.label(rebuildCommand))` exited \(result.exitCode)."
  }
}
