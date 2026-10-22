import Foundation

/// Where the logs are (issue 0607): intentd's two, as `intent daemon logs` names
/// them in its first line, and the app's own beside them. **The app derives no
/// daemon log path of its own** (ST0075 AC-02.1): the CLI's header is the one
/// answer, and the app's log goes in the directory it names, so a state
/// directory the CLI moves moves all three together.
struct LogPaths: Equatable, Sendable {
  let daemonLog: String
  let daemonErrLog: String

  /// The one file the app writes: every command it runs for the operator.
  static let appLogName = "Intent.app.log"

  var appLog: String {
    ((daemonLog as NSString).deletingLastPathComponent as NSString).appendingPathComponent(Self.appLogName)
  }

  /// Every log, in the order the menu's Log item hands them to Console.
  var all: [String] { [daemonLog, daemonErrLog, appLog] }

  /// The two logs in `intent daemon logs`'s first line, `tailing <intentd.log>
  /// and <intentd.err.log>` (`daemon_logs` in render.rs, a contract), or nil
  /// for any other line.
  ///
  /// **BOTH LOGS LIVE IN ONE DIRECTORY, SO THE SPLIT IS THE ONE WHOSE HALVES
  /// SHARE A PARENT.** A home directory may itself contain " and ", and
  /// splitting at the first occurrence would then name two paths that are
  /// neither log.
  static func parse(header line: String) -> LogPaths? {
    let prefix = "tailing "
    guard line.hasPrefix(prefix) else { return nil }
    let rest = String(line.dropFirst(prefix.count))
    var searched = rest.startIndex..<rest.endIndex
    while let split = rest.range(of: " and ", range: searched) {
      let log = String(rest[..<split.lowerBound])
      let errLog = String(rest[split.upperBound...])
      if (log as NSString).deletingLastPathComponent == (errLog as NSString).deletingLastPathComponent {
        return LogPaths(daemonLog: log, daemonErrLog: errLog)
      }
      searched = split.upperBound..<rest.endIndex
    }
    return nil
  }
}

/// What one command the app ran leaves in the app's log: a `» ` marker naming
/// it, its stdout then its stderr, and a marker saying how it ended. The lines
/// are text; `stamped` gives each the time of the write, which the caller reads.
enum LogBlock {
  static let markerPrefix = "» "

  /// `» intent doctor`, its output, `» exit 0 · 1.2s`. Each stream is split on
  /// its own, so a stdout with no final newline cannot run into stderr's first
  /// line.
  static func ran(_ label: String, _ result: CLIRunResult, after elapsed: Duration) -> [String] {
    let output = [result.stdout, result.stderr].flatMap { $0.split(separator: "\n").map(String.init) }
    return [markerPrefix + label] + output + [markerPrefix + exitMarker(result.exitCode, after: elapsed)]
  }

  /// `» intent doctor`, why it could not run, `» failed`.
  static func failed(_ label: String, because reason: String) -> [String] {
    [markerPrefix + label, "error: " + reason, markerPrefix + "failed"]
  }

  /// `exit 0 · 1.2s`.
  static func exitMarker(_ status: Int32, after elapsed: Duration) -> String {
    let seconds = Double(elapsed.components.seconds) + Double(elapsed.components.attoseconds) / 1e18
    return "exit \(status) · \(String(format: "%.1f", seconds))s"
  }

  /// The lines as one write's text, each opening with `date` as an RFC 3339 UTC
  /// time with milliseconds and one space: the shape intentd gives its own lines
  /// (issue 0321), so the three logs read alike in Console.
  static func stamped(_ lines: [String], at date: Date) -> String {
    let stamp = date.formatted(Date.ISO8601FormatStyle(includingFractionalSeconds: true))
    return lines.map { "\(stamp) \($0)\n" }.joined()
  }

  /// Whether a line reports a failure or a warning by the CLI's own tokens,
  /// matched after any indent: `error:`, `caused by:`, `warning:`, `remedy:`.
  /// OSLog has no warning level of its own, so both take its error level.
  static func isAlarm(_ line: String) -> Bool {
    let body = line.drop(while: { $0 == " " })
    return ["error:", "caused by:", "warning:", "remedy:"].contains { body.hasPrefix($0) }
  }
}
