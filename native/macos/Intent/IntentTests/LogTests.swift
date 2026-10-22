import XCTest

@testable import Intent

/// Issue 0607: what replaced the Console. The menu's Log item and the macOS
/// Console app are driven by hand; what a test can hold is the header the logs
/// are found by, the block a command leaves in the app's log, the append, and
/// `run`'s record and refusals, over stub commands and a scratch directory.
final class LogTests: XCTestCase {
  // MARK: - The header names the logs

  /// The verb's first line names intentd's two logs, and the app's own sits
  /// beside them. The literal is `daemon_logs`'s header (render.rs), with both
  /// logs under one state directory as `userstate` places them.
  func testTheHeaderNamesBothLogsAndTheAppsSitsBesideThem() {
    let paths = LogPaths.parse(
      header: "tailing /Users/op/.local/state/intent/intentd.log and /Users/op/.local/state/intent/intentd.err.log")
    XCTAssertEqual(
      paths,
      LogPaths(
        daemonLog: "/Users/op/.local/state/intent/intentd.log",
        daemonErrLog: "/Users/op/.local/state/intent/intentd.err.log"))
    XCTAssertEqual(paths?.appLog, "/Users/op/.local/state/intent/Intent.app.log")
  }

  /// **A PATH CONTAINING " and " STILL SPLITS WHERE THE TWO LOGS SHARE A
  /// DIRECTORY.** Splitting at the first occurrence would name two paths that
  /// are neither log.
  func testAPathContainingAndSplitsAtTheSharedDirectory() {
    XCTAssertEqual(
      LogPaths.parse(
        header: "tailing /Volumes/Ops and Build/state/intentd.log and /Volumes/Ops and Build/state/intentd.err.log"),
      LogPaths(
        daemonLog: "/Volumes/Ops and Build/state/intentd.log", daemonErrLog: "/Volumes/Ops and Build/state/intentd.err.log"))
  }

  /// Any other line is not a header, including the verb's own absent notice
  /// and intentd's listening line.
  func testNoOtherLineIsTakenForTheHeader() {
    XCTAssertNil(
      LogPaths.parse(header: "absent: /Users/op/.local/state/intent/intentd.err.log -- intentd has not written it yet"))
    XCTAssertNil(LogPaths.parse(header: "intentd listening on /tmp/intentd.sock and 127.0.0.1:51737"))
    XCTAssertNil(LogPaths.parse(header: "tailing nothing"))
  }

  // MARK: - The block a command leaves

  /// `» intent doctor`, its lines, `» exit 0 · 1.2s`: **A PASS LEAVES ITS LINES
  /// TOO**, so a clean doctor is seen, which was ST0075's whole trigger.
  func testACommandLeavesItsOutputBetweenItsMarkers() {
    let result = CLIRunResult(
      exitCode: 0, stdout: "ok: store intent/.cache/intent.db\nwarning: the index is 3 files behind\n", stderr: "")
    XCTAssertEqual(
      LogBlock.ran("intent doctor", result, after: .milliseconds(1_200)),
      ["» intent doctor", "ok: store intent/.cache/intent.db", "warning: the index is 3 files behind", "» exit 0 · 1.2s"])
  }

  /// Each stream splits on its own, so a stdout with no final newline does not
  /// run into stderr's first line, and a failing exit is a marker, not an error.
  func testStdoutThenStderrEachOnTheirOwnLines() {
    let result = CLIRunResult(
      exitCode: 1, stdout: "rebuilding the index", stderr: "error: database is locked\n  remedy: stop the writer and rerun\n")
    XCTAssertEqual(
      LogBlock.ran("intent index rebuild", result, after: .milliseconds(40)),
      [
        "» intent index rebuild", "rebuilding the index", "error: database is locked",
        "  remedy: stop the writer and rerun", "» exit 1 · 0.0s",
      ])
  }

  /// Every line opens with the write's time as an RFC 3339 UTC stamp with
  /// milliseconds, the shape of intentd's own lines (issue 0321).
  func testEveryLineOpensWithTheWritesUTCStamp() {
    let at = Date(timeIntervalSince1970: 1_761_152_462.125)
    XCTAssertEqual(
      LogBlock.stamped(["» intent doctor", "ok"], at: at),
      "2025-10-22T17:01:02.125Z » intent doctor\n2025-10-22T17:01:02.125Z ok\n")
  }

  /// The CLI's own tokens, after any indent, choose OSLog's error level; the
  /// same words later in a line do not.
  func testAlarmsAreTheCLIsOwnTokens() {
    let lines = [
      "error: x", "  caused by: y", "warning: z", "  remedy: w", "ok: store", "intentd: the error: token was quoted",
      "» intent doctor",
    ]
    XCTAssertEqual(lines.map(LogBlock.isAlarm), [true, true, true, true, false, false, false])
  }

  // MARK: - The append

  /// A block lands whole at the end of the file, and a missing directory is
  /// made first.
  func testAnAppendMakesTheDirectoryAndAddsAtTheEnd() throws {
    let file = try scratch().appendingPathComponent("state/intent/Intent.app.log").path
    XCTAssertNil(LogService.append("first\n", to: file))
    XCTAssertNil(LogService.append("second\nthird\n", to: file))
    XCTAssertEqual(try String(contentsOfFile: file, encoding: .utf8), "first\nsecond\nthird\n")
  }

  /// An append that cannot write says why, rather than nothing.
  func testAnAppendThatCannotWriteSaysWhy() throws {
    let blocker = try scratch().appendingPathComponent("a-file")
    try "".write(to: blocker, atomically: true, encoding: .utf8)
    XCTAssertNotNil(LogService.append("x\n", to: blocker.appendingPathComponent(LogPaths.appLogName).path))
  }

  // MARK: - Running and recording

  /// A run records its block in the app's log, beside intentd's, and hands the
  /// log back for the caller to open in Console.
  @MainActor
  func testARunRecordsItsBlockInTheAppsLog() async throws {
    let dir = try scratch()
    let log = LogService()
    let kept = try await log.run(
      ["doctor"], capture: Self.cli(dir, CLIRunResult(exitCode: 0, stdout: "ok: store intent/.cache/intent.db\n", stderr: "")),
      now: Self.readings(.milliseconds(1_200)))
    let file = try XCTUnwrap(kept)
    XCTAssertEqual(file, dir.appendingPathComponent(LogPaths.appLogName).path)
    XCTAssertEqual(try Self.unstamped(file), ["» intent doctor", "ok: store intent/.cache/intent.db", "» exit 0 · 1.2s"])
    XCTAssertNil(log.running)
  }

  /// **ONE AT A TIME, REFUSED RATHER THAN QUEUED**: a second command while one
  /// runs is refused naming the running one, before it records anything.
  @MainActor
  func testASecondRunWhileOneRunsIsRefusedNamingIt() async throws {
    let dir = try scratch()
    let log = LogService()
    let gate = Gate()
    let first = Task {
      try await log.run(
        ["index", "rebuild"],
        capture: { args in
          guard args.first != "daemon" else { return Self.header(dir) }
          await gate.hold()
          return CLIRunResult(exitCode: 0, stdout: "", stderr: "")
        },
        now: Self.readings(.zero))
    }
    await gate.arrival()
    do {
      _ = try await log.run(["doctor"], capture: Self.cli(dir, CLIRunResult(exitCode: 0, stdout: "", stderr: "")))
      XCTFail("a second run while one runs was not refused")
    } catch let error as LogError {
      XCTAssertEqual(error, .busy("intent index rebuild"))
    }
    await gate.open()
    let kept = try await first.value
    XCTAssertEqual(try Self.unstamped(try XCTUnwrap(kept)), ["» intent index rebuild", "» exit 0 · 0.0s"])
  }

  /// A command that cannot launch is recorded as failed, with its reason, and
  /// the reason is thrown, so the app raises its alert.
  @MainActor
  func testACommandThatCannotLaunchIsRecordedAsFailedAndThrown() async throws {
    let dir = try scratch()
    let log = LogService()
    do {
      _ = try await log.run(
        ["doctor"],
        capture: { args in
          guard args.first != "daemon" else { return Self.header(dir) }
          throw IntentCLIError.binaryNotFound
        })
      XCTFail("a launch failure was not thrown")
    } catch {
      XCTAssertEqual(error.localizedDescription, IntentCLIError.binaryNotFound.localizedDescription)
    }
    XCTAssertEqual(
      try Self.unstamped(dir.appendingPathComponent(LogPaths.appLogName).path),
      ["» intent doctor", "error: " + IntentCLIError.binaryNotFound.localizedDescription, "» failed"])
    XCTAssertNil(log.running)
  }

  /// **A RECORD THAT CANNOT BE WRITTEN IS NEVER DROPPED IN SILENCE**: when the
  /// CLI will not say where the logs are, the operator is told EVERY time, since
  /// each record is a command somebody chose, and `run` hands back no log to
  /// open. A second failure told nowhere would be a click that did nothing.
  @MainActor
  func testEveryRecordThatCannotBeWrittenIsReported() async throws {
    let log = LogService()
    let reports = Reports()
    log.reportUnwritable = { reports.said.append($0) }
    let unanswered: LogService.Capture = { _ in
      CLIRunResult(exitCode: 1, stdout: "", stderr: "error: the state directory could not be resolved\n")
    }
    let first = try await log.run(["doctor"], capture: unanswered)
    let second = try await log.run(["doctor"], capture: unanswered)
    XCTAssertNil(first)
    XCTAssertNil(second)
    let said =
      "Intent.app.log could not be written: its directory is not known: `intent daemon logs` did not name the logs: "
      + "error: the state directory could not be resolved"
    XCTAssertEqual(reports.said, [said, said])
  }

  // MARK: - Fixtures

  /// A directory of the test's own, removed when it ends.
  private func scratch() throws -> URL {
    let dir = FileManager.default.temporaryDirectory.appendingPathComponent("LogTests-\(UUID().uuidString)")
    try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
    addTeardownBlock { try FileManager.default.removeItem(at: dir) }
    return dir
  }

  /// `intent daemon logs`'s header for logs under `dir`.
  private static func header(_ dir: URL) -> CLIRunResult {
    CLIRunResult(exitCode: 0, stdout: "tailing \(dir.path)/intentd.log and \(dir.path)/intentd.err.log\n", stderr: "")
  }

  /// A stub CLI: the logs' header for `daemon logs`, `result` for anything else.
  private static func cli(_ dir: URL, _ result: CLIRunResult) -> LogService.Capture {
    { args in args.first == "daemon" ? header(dir) : result }
  }

  /// A clock that reads `start`, then `start` plus `elapsed`.
  private static func readings(_ elapsed: Duration) -> () -> ContinuousClock.Instant {
    let start = ContinuousClock.now
    var next = [start, start.advanced(by: elapsed)]
    return { next.removeFirst() }
  }

  /// The app's log's lines with their stamps taken off, each stamp checked for
  /// intentd's shape first.
  private static func unstamped(_ file: String) throws -> [String] {
    let text = try String(contentsOfFile: file, encoding: .utf8)
    return try text.split(separator: "\n").map { line in
      let stamp = try XCTUnwrap(line.prefixMatch(of: /\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z[ ]/), String(line))
      return String(line[stamp.range.upperBound...])
    }
  }
}

/// What `reportUnwritable` was told, in order.
@MainActor
private final class Reports {
  var said: [String] = []
}

/// Holds a stub command until the test lets it go, and says when it got there,
/// so the test waits on the command rather than on the clock.
private actor Gate {
  private var arrived = false
  private var arrivalWaiter: CheckedContinuation<Void, Never>?
  private var released = false
  private var holder: CheckedContinuation<Void, Never>?

  func hold() async {
    arrived = true
    arrivalWaiter?.resume()
    arrivalWaiter = nil
    guard !released else { return }
    await withCheckedContinuation { holder = $0 }
  }

  func arrival() async {
    guard !arrived else { return }
    await withCheckedContinuation { arrivalWaiter = $0 }
  }

  func open() {
    released = true
    holder?.resume()
    holder = nil
  }
}
