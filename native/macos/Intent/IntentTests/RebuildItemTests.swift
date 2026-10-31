import XCTest

@testable import Intent

/// AT-02.1 / AC-02.1 to AC-02.3 (ST0090 WP-02): **Intent.app's Rebuild Search
/// Index row is Gtools' Refresh Search row.** Its second line says how long ago
/// the index was reconciled, in Gtools' age words, from the `reconciled_at` that
/// `intent index status --json` names, and there is none without one; its title
/// and busy caption are as they were; a rebuild that exits non-zero, or cannot
/// be run, is `Rebuild failed` with the CLI's own `error:` and `remedy:` lines;
/// and every rebuild is one block in the app's log and opens nothing. The arms
/// are Gtools' `CmsStatusTests` refresh arms, run over stub commands and a
/// scratch directory as `LogTests`' are. Run Doctor still opens the log through
/// `runLogged`, which this package leaves as it was.
final class RebuildItemTests: XCTestCase {
  /// What `intent index status --json` prints, its keys as `report_index`
  /// writes them (render.rs), with the stamp the last reconcile wrote.
  static let reconciled = """
    {"held":{"source":1834},"grammars":{},"skipped":{},"sizes":{},"resolution":{},
     "reconciled_at":"2026-10-31T14:44:38.721Z","empty":false,"rebuilt":false}
    """

  // MARK: - The second line

  /// gtools#0108's line with the index's own verb: the age in Gtools' words at
  /// each of their units, and never a negative age. Each reading is half a
  /// second past its whole second, because the stamp carries milliseconds and a
  /// reading on the second sits on the boundary the words truncate at.
  func testTheSecondLineSaysHowLongAgoTheIndexWasReconciledInGtoolsWords() throws {
    let status = try IndexStatus.decode(Data(Self.reconciled.utf8))
    let at = try XCTUnwrap(IndexStatus.date(status.reconciledAt))
    XCTAssertEqual(at.timeIntervalSince1970, 1_793_457_878.721, accuracy: 0.000_5)
    XCTAssertEqual(
      IndexStatus.rebuildSubtitle(status: status, now: at.addingTimeInterval(12.5)),
      "reconciled 12s ago")
    XCTAssertEqual(
      IndexStatus.rebuildSubtitle(status: status, now: at.addingTimeInterval(300.5)),
      "reconciled 5m ago")
    XCTAssertEqual(
      IndexStatus.rebuildSubtitle(status: status, now: at.addingTimeInterval(7_380.5)),
      "reconciled 2h 3m ago")
    XCTAssertEqual(
      IndexStatus.rebuildSubtitle(status: status, now: at.addingTimeInterval(100_800.5)),
      "reconciled 1d 4h ago")
    XCTAssertEqual(
      IndexStatus.rebuildSubtitle(status: status, now: at.addingTimeInterval(-300)),
      "reconciled 0s ago")
  }

  /// No stamp is no second line, rather than an age the CLI never named: an
  /// index no reconcile has stamped (`null`), a CLI from before WP-01 (no key), a
  /// stamp in another form, and no answer at all.
  func testNoStampIsNoSecondLine() throws {
    let now = Date(timeIntervalSince1970: 1_793_457_900)
    let unstamped = try IndexStatus.decode(
      Data(#"{"held":{},"reconciled_at":null,"empty":true}"#.utf8))
    let older = try IndexStatus.decode(Data(#"{"held":{},"empty":true}"#.utf8))
    XCTAssertNil(IndexStatus.rebuildSubtitle(status: unstamped, now: now))
    XCTAssertNil(IndexStatus.rebuildSubtitle(status: older, now: now))
    XCTAssertNil(
      IndexStatus.rebuildSubtitle(status: IndexStatus(reconciledAt: "yesterday"), now: now))
    XCTAssertNil(IndexStatus.rebuildSubtitle(status: nil, now: now))
  }

  // MARK: - The title and the busy caption

  /// The row's title, and the caption that stands in for it while the rebuild
  /// runs, are as they were before this package.
  func testTheTitleAndTheBusyCaptionAreAsTheyWere() {
    XCTAssertEqual(IndexStatus.rebuildTitle(rebuilding: false), "Rebuild Search Index")
    XCTAssertEqual(IndexStatus.rebuildTitle(rebuilding: true), "Rebuilding Search Index…")
  }

  // MARK: - A rebuild

  /// AC-02.2: a rebuild that exits non-zero is `Rebuild failed` carrying the
  /// CLI's own `error:` and `remedy:` lines, its whole block is still in the
  /// app's log, and the stamp is read after it.
  @MainActor
  func testAFailedRebuildIsTheAlertWithTheCLIsOwnErrorAndRemedyLines() async throws {
    let dir = try scratch()
    let log = LogService()
    let index = IndexService()
    let failed = CLIRunResult(
      exitCode: 1, stdout: "rebuilding the index\n",
      stderr: "error: database is locked\n  remedy: stop the writer and rerun\n")
    do {
      try await index.rebuild(
        log: log, capture: Self.cli(dir, rebuild: failed), configured: { true })
      XCTFail("a rebuild that exited 1 raised nothing")
    } catch {
      XCTAssertEqual(
        error.localizedDescription, "error: database is locked\n  remedy: stop the writer and rerun"
      )
    }
    XCTAssertEqual(IndexStatus.rebuildFailedTitle, "Rebuild failed")
    let lines = try LogTests.unstamped(dir.appendingPathComponent(LogPaths.appLogName).path)
    XCTAssertEqual(
      Array(lines.dropLast()),
      [
        "» intent index rebuild", "rebuilding the index", "error: database is locked",
        "  remedy: stop the writer and rerun",
      ])
    XCTAssertTrue(lines.last?.hasPrefix("» exit 1 · ") == true, "\(lines)")
    XCTAssertEqual(index.status?.reconciledAt, "2026-10-31T14:44:38.721Z")
  }

  /// A failed rebuild that printed none of the CLI's tokens is named by its
  /// exit, so the alert never says nothing.
  func testAFailureThatPrintedNoTokenIsNamedByItsExit() {
    XCTAssertEqual(
      IndexStatus.failure(
        CLIRunResult(exitCode: 101, stdout: "", stderr: "thread 'main' panicked\n")),
      "`intent index rebuild` exited 101.")
  }

  /// AC-02.2's other half: a rebuild that cannot be run raises the same alert
  /// with why, and is recorded as failed.
  @MainActor
  func testARebuildThatCannotRunIsTheAlertWithWhy() async throws {
    let dir = try scratch()
    let log = LogService()
    let index = IndexService()
    do {
      try await index.rebuild(
        log: log,
        capture: { args in
          guard args.first != "daemon" else { return LogTests.header(dir) }
          throw IntentCLIError.binaryNotFound
        },
        configured: { true })
      XCTFail("a rebuild that could not run raised nothing")
    } catch {
      XCTAssertEqual(error.localizedDescription, IntentCLIError.binaryNotFound.localizedDescription)
    }
    XCTAssertEqual(
      try LogTests.unstamped(dir.appendingPathComponent(LogPaths.appLogName).path),
      [
        "» intent index rebuild", "error: " + IntentCLIError.binaryNotFound.localizedDescription,
        "» failed",
      ])
    XCTAssertNil(index.status)
  }

  /// AC-02.3: a rebuild that succeeds is one block in the app's log, opens
  /// nothing, and is followed by a read of the stamp. The open after it is the
  /// control: the recorder sees an open when one is made.
  @MainActor
  func testASuccessfulRebuildIsOneBlockInTheLogAndOpensNothing() async throws {
    let dir = try scratch()
    let log = LogService()
    let opened = Opened()
    log.opener = { urls in opened.urls += urls }
    let index = IndexService()
    try await index.rebuild(
      log: log,
      capture: Self.cli(
        dir, rebuild: CLIRunResult(exitCode: 0, stdout: "ok: rebuilt the index\n", stderr: "")),
      configured: { true })
    let file = dir.appendingPathComponent(LogPaths.appLogName).path
    let lines = try LogTests.unstamped(file)
    XCTAssertEqual(Array(lines.dropLast()), ["» intent index rebuild", "ok: rebuilt the index"])
    XCTAssertTrue(lines.last?.hasPrefix("» exit 0 · ") == true, "\(lines)")
    XCTAssertEqual(opened.urls, [], "a rebuild opened the log")
    XCTAssertEqual(
      index.status?.reconciledAt, "2026-10-31T14:44:38.721Z",
      "the stamp was not read after the rebuild")
    try await log.open([file])
    XCTAssertEqual(opened.urls, [URL(fileURLWithPath: file)])
  }

  // MARK: - No project

  /// With no project configured the stamp is not read, as `ProjectService`
  /// reads nothing without one: the command never runs and `status` stays nil.
  /// The read with one configured after it is the control: the same answer is
  /// taken when it is asked for.
  @MainActor
  func testWithNoProjectConfiguredNoCommandRunsAndThereIsNoStamp() async {
    let index = IndexService()
    await index.read(
      capture: { args in
        XCTFail("no project is configured, and \(IntentCLI.label(args)) ran")
        return CLIRunResult(exitCode: 0, stdout: Self.reconciled, stderr: "")
      },
      configured: { false })
    XCTAssertNil(index.status)
    await index.read(
      capture: { _ in CLIRunResult(exitCode: 0, stdout: Self.reconciled, stderr: "") },
      configured: { true })
    XCTAssertEqual(index.status?.reconciledAt, "2026-10-31T14:44:38.721Z")
  }

  // MARK: - Fixtures

  /// A stub CLI: the logs' header for `daemon logs`, the stamped report for
  /// `index status --json`, and `rebuild` for the rebuild.
  private static func cli(_ dir: URL, rebuild: CLIRunResult) -> LogService.Capture {
    { args in
      if args.first == "daemon" { return LogTests.header(dir) }
      if args == IndexStatus.statusCommand {
        return CLIRunResult(exitCode: 0, stdout: reconciled, stderr: "")
      }
      return rebuild
    }
  }
}

/// What the log's opener was handed, in order.
@MainActor
private final class Opened {
  var urls: [URL] = []
}
