import AppKit
import Observation

/// Why a command was not run, or the logs were not found or opened.
enum LogError: LocalizedError, Equatable {
  /// Another command is running; it is named, and nothing is queued.
  case busy(String)
  /// `intent daemon logs` did not open with the header that names the logs.
  case noHeader(String)
  /// None of the logs has been written yet.
  case noLogYet
  /// Launch Services finds no Console app.
  case noConsole

  var errorDescription: String? {
    switch self {
    case .busy(let label): "`\(label)` is still running. One command at a time."
    case .noHeader(let said): "`intent daemon logs` did not name the logs: \(said)"
    case .noLogYet: "No log has been written yet."
    case .noConsole: "The macOS Console app (\(LogService.consoleBundleID)) was not found."
    }
  }
}

/// The app's log (issue 0607), in Utilz menuz's shape. Every command the app
/// runs for the operator -- doctor, the index rebuild, and the daemon's
/// lifecycle verbs -- is recorded as one block in `Intent.app.log`, beside
/// intentd's logs, and the menu's Log item hands all three to the macOS Console
/// app, which does the search, the filter, the live follow and the window. The
/// app keeps no backlog and draws no view: this replaced ST0075's Console
/// window.
///
/// **A RECORD THAT CANNOT BE WRITTEN IS NEVER DROPPED IN SILENCE.** It is an
/// OSLog error, and `reportUnwritable` tells the operator, every time. menuz
/// reports once per run of failures because it logs what nobody clicked; every
/// record this app keeps is a command somebody chose from the menu, so a second
/// failure reported nowhere would be a click that did nothing (ST0075's trigger).
@MainActor @Observable
final class LogService {
  static let shared = LogService()
  private static let logger = AppLog.logger("Log")
  nonisolated static let consoleBundleID = "com.apple.Console"

  /// Where the logs are, once the CLI has said: nil before that and after a
  /// read that failed, never a path guessed in Swift.
  private(set) var paths: LogPaths?
  /// The command running now, as the operator would type it: its menu item
  /// says so while it runs.
  private(set) var running: String?

  /// How a record that cannot be written reaches the operator: the app's alert.
  @ObservationIgnored var reportUnwritable: (@MainActor (String) -> Void)?

  /// What runs a command: `IntentCLI.capture` in the app, a stub in the tests.
  typealias Capture = ([String]) async throws -> CLIRunResult

  /// Whether any log exists, for the menu's Log item. Nil while the CLI has not
  /// said where they are, when the item stays live, so a click asks again and
  /// says why it cannot open them.
  var logExists: Bool? {
    paths.map { $0.all.contains { FileManager.default.fileExists(atPath: $0) } }
  }

  // MARK: - The logs

  /// At launch: where the logs are, kept for the Log item. A failure is logged,
  /// and leaves the item live.
  func findPaths() async {
    do {
      try await refreshPaths()
    } catch {
      Self.logger.error("the logs could not be found: \(error.localizedDescription, privacy: .public)")
    }
  }

  /// Asks `intent daemon logs --lines 0` where the logs are, and keeps the
  /// answer. A read that fails clears what was kept, and is thrown.
  @discardableResult
  func refreshPaths(capture: Capture = IntentCLI.capture) async throws -> LogPaths {
    do {
      let result = try await capture(["daemon", "logs", "--lines", "0"])
      let header = result.stdout.split(separator: "\n", maxSplits: 1).first.map(String.init) ?? ""
      guard let read = LogPaths.parse(header: header) else {
        let said = header.isEmpty ? result.stderr.trimmingCharacters(in: .whitespacesAndNewlines) : header
        throw LogError.noHeader(said.isEmpty ? "exit \(result.exitCode), and nothing printed" : said)
      }
      paths = read
      return read
    } catch {
      paths = nil
      throw error
    }
  }

  /// The Log item: every log that exists, handed to Console. The paths are
  /// asked for again first, so a click after a failed read says why.
  func openAll(capture: Capture = IntentCLI.capture) async throws {
    try await open(refreshPaths(capture: capture).all)
  }

  /// Hands the logs that exist to the macOS Console app, named by its bundle
  /// identifier rather than by whatever app owns `.log` files on this machine.
  /// A log not written yet is passed over; none at all is `noLogYet`.
  func open(_ files: [String]) async throws {
    let present = files.filter { FileManager.default.fileExists(atPath: $0) }
    guard !present.isEmpty else { throw LogError.noLogYet }
    guard let console = NSWorkspace.shared.urlForApplication(withBundleIdentifier: Self.consoleBundleID) else {
      throw LogError.noConsole
    }
    let urls = present.map { URL(fileURLWithPath: $0) }
    try await withCheckedThrowingContinuation { (opened: CheckedContinuation<Void, Error>) in
      NSWorkspace.shared.open(urls, withApplicationAt: console, configuration: NSWorkspace.OpenConfiguration()) {
        _, error in
        if let error { opened.resume(throwing: error) } else { opened.resume() }
      }
    }
  }

  // MARK: - Running and recording

  /// Runs a command for the operator and records it: `» intent doctor`, its
  /// lines, `» exit 0 · 1.2s`. Returns the app's log once the record is
  /// written, for the caller to open, and nil when it could not be.
  ///
  /// **ONE AT A TIME, REFUSED RATHER THAN QUEUED**: a second command while one
  /// runs throws `busy`, naming the running one, before it records anything. A
  /// command that cannot launch is recorded as failed, and its reason thrown. A
  /// non-zero exit is not thrown: the command's own lines say what went wrong.
  func run(
    _ args: [String],
    capture: Capture = IntentCLI.capture,
    now: () -> ContinuousClock.Instant = { ContinuousClock.now }
  ) async throws -> String? {
    let label = IntentCLI.label(args)
    if let running { throw LogError.busy(running) }
    running = label
    defer { running = nil }
    let started = now()
    let result: CLIRunResult
    do {
      result = try await capture(args)
    } catch {
      await record(LogBlock.failed(label, because: error.localizedDescription), capture: capture)
      throw error
    }
    return await record(LogBlock.ran(label, result, after: now() - started), capture: capture)
  }

  /// Appends one block to the app's log in one write, each line stamped with
  /// the time of the write, and mirrors each line to OSLog at the level its
  /// token implies. Returns the app's log, or nil when the block could not be
  /// written, which is logged and reported.
  @discardableResult
  func record(_ lines: [String], capture: Capture = IntentCLI.capture) async -> String? {
    for line in lines {
      if LogBlock.isAlarm(line) {
        Self.logger.error("\(line, privacy: .public)")
      } else {
        Self.logger.notice("\(line, privacy: .public)")
      }
    }
    let file: String
    do {
      file = try await known(capture: capture).appLog
    } catch {
      return unwritable("its directory is not known: \(error.localizedDescription)")
    }
    if let reason = Self.append(LogBlock.stamped(lines, at: Date()), to: file) {
      return unwritable(reason)
    }
    return file
  }

  /// The paths kept, or asked for when none are.
  private func known(capture: Capture) async throws -> LogPaths {
    if let paths { return paths }
    return try await refreshPaths(capture: capture)
  }

  /// A record that could not be written: logged and reported.
  private func unwritable(_ reason: String) -> String? {
    let said = "\(LogPaths.appLogName) could not be written: \(reason)"
    Self.logger.error("\(said, privacy: .public)")
    reportUnwritable?(said)
    return nil
  }

  /// Appends `text` in one write on a descriptor opened `O_APPEND`, making its
  /// directory first; why not, when it could not. One write per block means a
  /// block lands whole, after whatever was there (menuz's `AppLog.append`).
  nonisolated static func append(_ text: String, to file: String) -> String? {
    let directory = URL(fileURLWithPath: file).deletingLastPathComponent()
    do {
      try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)
    } catch {
      return error.localizedDescription
    }
    let descriptor = Darwin.open(file, O_WRONLY | O_APPEND | O_CREAT | O_CLOEXEC, 0o644)
    guard descriptor >= 0 else { return String(cString: strerror(errno)) }
    defer { Darwin.close(descriptor) }
    let bytes = Array(text.utf8)
    let written = bytes.withUnsafeBytes { Darwin.write(descriptor, $0.baseAddress, $0.count) }
    guard written == bytes.count else {
      return written < 0 ? String(cString: strerror(errno)) : "wrote \(written) of \(bytes.count) bytes"
    }
    return nil
  }
}
