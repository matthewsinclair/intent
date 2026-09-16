import XCTest

@testable import Intent

/// ST0075 WP-02's three criteria, each proved here. The window itself is driven
/// by hand once, in hv's quiet window; what a test can hold is the header, the
/// classification and the backlog the window renders from.
final class ConsoleTests: XCTestCase {
  // MARK: - AT-02.1: the footer names the files being tailed

  /// AT-02.1: the verb's first line becomes the footer's two logs. The literal
  /// is `daemon_logs`'s header (render.rs), with both logs under one state
  /// directory as `userstate` places them.
  func testTheTailingHeaderNamesBothLogs() {
    XCTAssertEqual(
      ConsoleRunner.tailedLogs(
        "tailing /Users/op/.local/state/intent/intentd.log and /Users/op/.local/state/intent/intentd.err.log"),
      TailedLogs(
        log: "/Users/op/.local/state/intent/intentd.log", errLog: "/Users/op/.local/state/intent/intentd.err.log"))
  }

  /// AT-02.1: **A PATH CONTAINING " and " STILL SPLITS WHERE THE TWO LOGS SHARE
  /// A DIRECTORY.** Splitting at the first occurrence would name two paths that
  /// are neither log, and the footer would read as though it were right.
  func testAPathContainingAndSplitsAtTheSharedDirectory() {
    XCTAssertEqual(
      ConsoleRunner.tailedLogs(
        "tailing /Volumes/Ops and Build/state/intentd.log and /Volumes/Ops and Build/state/intentd.err.log"),
      TailedLogs(log: "/Volumes/Ops and Build/state/intentd.log", errLog: "/Volumes/Ops and Build/state/intentd.err.log"))
  }

  /// AT-02.1: any other line is not a header, including the verb's own absent
  /// notice and intentd's listening line.
  func testNoOtherLineIsTakenForTheHeader() {
    XCTAssertNil(ConsoleRunner.tailedLogs("absent: /Users/op/.local/state/intent/intentd.err.log -- intentd has not written it yet"))
    XCTAssertNil(ConsoleRunner.tailedLogs("intentd listening on /tmp/intentd.sock and 127.0.0.1:51737"))
    XCTAssertNil(ConsoleRunner.tailedLogs("tailing nothing"))
  }

  /// AT-02.1: **ONLY THE VERB'S FIRST LINE IS READ AS ITS HEADER.** The header
  /// names the footer's logs and reaches no backlog; a later line of the same
  /// shape is a log line like any other, and the footer keeps what the first
  /// line named.
  func testOnlyTheFirstLineIsReadAsTheHeader() {
    let first = ConsoleRunner.read("tailing /s/intentd.log and /s/intentd.err.log", after: .awaiting)
    XCTAssertEqual(first.header, .logs(TailedLogs(log: "/s/intentd.log", errLog: "/s/intentd.err.log")))
    XCTAssertNil(first.line)

    let later = ConsoleRunner.read("tailing /t/intentd.log and /t/intentd.err.log", after: first.header)
    XCTAssertEqual(later.header, first.header)
    XCTAssertEqual(later.line, ConsoleLine(tail: "tailing /t/intentd.log and /t/intentd.err.log"))
  }

  /// AT-02.1: a first line that is not the header leaves the footer saying so,
  /// rather than waiting for a header that will not come, and the line itself
  /// reaches the backlog with its colour.
  func testAFirstLineThatIsNotTheHeaderIsKeptAndSaidSo() {
    let read = ConsoleRunner.read("error: the state directory could not be resolved", after: .awaiting)
    XCTAssertEqual(read.header, .unrecognised)
    XCTAssertEqual(read.line, ConsoleLine(text: "error: the state directory could not be resolved", kind: .error, source: .tail))
  }

  // MARK: - AT-02.2: lines are classified for colour

  /// AT-02.2: over intentd's own line shapes, and the renderer's tokens as the
  /// operator sees them: `error: `, `  caused by: ` and `  remedy: `. intentd's
  /// `warning:` lines take the warning colour (vc's ruling (b), 2026-09-15).
  func testLinesAreClassifiedOverIntentdsOwnShapes() {
    XCTAssertEqual(ConsoleLine.kind(of: "error: intentd takes no arguments and was given 2"), .error)
    XCTAssertEqual(ConsoleLine.kind(of: "  caused by: database is locked"), .error)
    XCTAssertEqual(
      ConsoleLine.kind(of: "intentd: could not refresh the index under `/Users/op/p`: the store is busy"), .error)
    XCTAssertEqual(
      ConsoleLine.kind(
        of: "  remedy: files under those paths may not be reaching `intent search`. Run `intent index rebuild` to catch it up"),
      .warning)
    XCTAssertEqual(
      ConsoleLine.kind(of: "warning: intentd could not accept a unix connection: Too many open files (os error 24)"),
      .warning)
    XCTAssertEqual(
      ConsoleLine.kind(of: "warning: intentd could not accept a loopback connection: Connection reset by peer (os error 54)"),
      .warning)
    XCTAssertEqual(
      ConsoleLine.kind(
        of:
          "warning: this machine's LaunchAgent was written by 3.0.2 and could not be regenerated: this process cannot resolve its own path"
      ),
      .warning)
    XCTAssertEqual(ConsoleLine.kind(of: "  remedy: `intent daemon start --at-login` rewrites it."), .warning)
    XCTAssertEqual(ConsoleLine.kind(of: "» intent doctor"), .marker)
    XCTAssertEqual(ConsoleLine.kind(of: "intentd listening on /tmp/intentd.sock and 127.0.0.1:51737"), .log)
    XCTAssertEqual(ConsoleLine.kind(of: "intentd: backed up `/Users/op/p` to /Users/op/.local/state/intent/backups"), .log)
  }

  /// AT-02.2: the tokens are matched as tokens, so a log line that merely
  /// mentions one is not coloured as one.
  func testALineThatMentionsATokenIsNotColouredAsOne() {
    XCTAssertEqual(ConsoleLine.kind(of: "intentd stopping: the error: token was in the reason"), .log)
    XCTAssertEqual(ConsoleLine.kind(of: "intentd: the remedy: text is quoted here"), .log)
  }

  /// AT-02.2: **A STAMPED LINE CLASSIFIES AS THE LINE UNDER ITS STAMP.** intentd
  /// opens each line of both logs with an RFC 3339 UTC timestamp and one space
  /// (vc's decision with issue 0321), so a stamped notice and its stamped remedy
  /// keep their colours; a token with the shape of a stamp and a month that
  /// does not exist is not skipped.
  func testAStampedLineClassifiesAsTheLineUnderItsStamp() {
    XCTAssertEqual(
      ConsoleLine.kind(
        of: "2026-09-15T21:40:03.512Z intentd: could not refresh the index under `/Users/op/p`: the store is busy"),
      .error)
    XCTAssertEqual(
      ConsoleLine.kind(
        of: "2026-09-15T21:40:03.512Z   remedy: files under those paths may not be reaching `intent search`."),
      .warning)
    XCTAssertEqual(ConsoleLine.kind(of: "2026-13-15T21:40:03.512Z error: a thirteenth month is not a stamp"), .log)
  }

  // MARK: - AT-02.3: the backlog is bounded

  /// AT-02.3: the ring keeps the last lines up to its capacity and says how many
  /// it dropped, so the view can drop the same lines from its text.
  func testTheRingKeepsTheLastLinesAndSaysHowManyItDropped() {
    var ring = LineRing(capacity: 3)
    var dropped = 0
    for i in 1...5 { dropped += ring.append(ConsoleLine(tail: "\(i)")) }
    XCTAssertEqual(dropped, 2)
    XCTAssertEqual(ring.lines.map(\.text), ["3", "4", "5"])
    ring.clear()
    XCTAssertTrue(ring.lines.isEmpty)
  }

  /// AT-02.3: the ring drops one source's lines and keeps the other's in order,
  /// which is what a tail's start asks of it: the verb's replay replaces the
  /// earlier tail's lines, and a line the app wrote while the window was closed
  /// is there when it opens. **ONLY THE RING IS TESTED HERE.** `startTail()`
  /// launches the verb itself, so no test reaches its call to `removeAll(from:)`,
  /// and closing and reopening the window is driven by hand.
  func testTheRingDropsOneSourcesLinesAndKeepsTheOthersInOrder() {
    var ring = LineRing(capacity: 10)
    _ = ring.append(ConsoleLine(tail: "intentd listening on /tmp/intentd.sock and 127.0.0.1:51737"))
    _ = ring.append(ConsoleLine(text: "intent daemon logs exited 1", kind: .error, source: .app))
    _ = ring.append(ConsoleLine(tail: "intentd stopping"))
    XCTAssertEqual(ring.removeAll(from: .tail), 2)
    XCTAssertEqual(ring.lines.map(\.text), ["intent daemon logs exited 1"])
  }
}
