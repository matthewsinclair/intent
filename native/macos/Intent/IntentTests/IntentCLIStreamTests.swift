import XCTest

@testable import Intent

/// `IntentCLI.stream`'s events, driven through `/bin/sh` so what is under test is
/// the plumbing rather than a verb. Each test names one property and passes only
/// while that property holds, and every drain is bounded, so a regression that
/// leaves a child running fails instead of hanging the run.
final class IntentCLIStreamTests: XCTestCase {
  private func launch(_ script: String, holdStdin: Bool = false) throws -> IntentCLI.StreamedChild {
    try IntentCLI.launchStreaming(
      binary: "/bin/sh", args: ["-c", script], env: ProcessInfo.processInfo.environment, cwd: nil,
      holdStdin: holdStdin)
  }

  /// Every event until the stream ends, each handed to `onEvent` as it arrives,
  /// or nil when the stream has not ended within `seconds`.
  private func drain(
    _ events: AsyncStream<IntentCLI.StreamEvent>,
    within seconds: Int = 10,
    onEvent: @escaping @Sendable (IntentCLI.StreamEvent) -> Void = { _ in }
  ) async -> [IntentCLI.StreamEvent]? {
    await withTaskGroup(of: [IntentCLI.StreamEvent]?.self) { group in
      group.addTask {
        var seen: [IntentCLI.StreamEvent] = []
        for await event in events {
          seen.append(event)
          onEvent(event)
        }
        return seen
      }
      group.addTask {
        try? await Task.sleep(for: .seconds(seconds))
        return nil
      }
      let first = await group.next() ?? nil
      group.cancelAll()
      return first
    }
  }

  private static func text(of event: IntentCLI.StreamEvent?) -> String? {
    guard case .line(let text)? = event else { return nil }
    return text
  }

  /// Lines arrive in the order they were written, stderr among stdout, a
  /// fragment with no newline is the last line, and the exit follows them all.
  func testLinesArriveInOrderThenTheExit() async throws {
    let child = try launch("printf 'one\\n'; printf 'two\\n' >&2; printf 'three'; exit 3")
    let events = await drain(child.events)
    XCTAssertEqual(events, [.line("one"), .line("two"), .line("three"), .exited(3)])
  }

  /// A line that is not valid UTF-8 still arrives, its bad byte shown as U+FFFD.
  func testALineOfBadBytesStillArrives() async throws {
    let child = try launch("printf 'caf\\351\\n'")
    let events = await drain(child.events)
    XCTAssertEqual(events, [.line("caf\u{FFFD}"), .exited(0)])
  }

  /// **OUTPUT THAT OUTLIVES THE EXIT STILL ARRIVES, AND BEFORE IT.** The shell
  /// exits at once and a child it left behind writes half a second later, so
  /// events that ended at the exit would lose that line on every run.
  func testOutputThatOutlivesTheExitStillArrivesBeforeIt() async throws {
    let child = try launch("printf 'one\\n'; (sleep 0.5; printf 'late\\n') & exit 3")
    let events = await drain(child.events)
    XCTAssertEqual(events, [.line("one"), .line("late"), .exited(3)])
  }

  /// **AN EXIT THAT OUTLIVES THE OUTPUT STILL ENDS THE EVENTS.** The child closes
  /// its output and exits half a second later, so events that ended with the
  /// output would never carry the exit.
  func testAnExitThatOutlivesTheOutputStillEndsTheEvents() async throws {
    let child = try launch("printf 'one\\n'; exec >&- 2>&-; sleep 0.5; exit 4")
    let events = await drain(child.events)
    XCTAssertEqual(events, [.line("one"), .exited(4)])
  }

  /// **A HELD STDIN IS A PIPE OF THE CHILD'S OWN.** The child names what its
  /// stdin is, and it is a pipe that is not the test host's own stdin, so the
  /// only thing that can close it is the app's end of that pipe.
  func testAHeldStdinIsAPipeOfTheChildsOwn() async throws {
    var host = stat()
    XCTAssertEqual(fstat(0, &host), 0, "the test host's own stdin can be examined")
    let child = try launch("stat -L -f '%HT %i' /dev/stdin", holdStdin: true)
    defer { child.child.terminate() }
    let events = await drain(child.events)
    XCTAssertEqual(events?.count, 2, "one line, then the exit: \(String(describing: events))")
    let named = Self.text(of: events?.first) ?? ""
    XCTAssertTrue(named.hasPrefix("Fifo File "), "the child's stdin is a pipe, not \(named)")
    XCTAssertNotEqual(named, "Fifo File \(host.st_ino)", "the child's stdin is not the test host's own")
  }

  /// **CLOSING THE HELD STDIN ENDS A CHILD THAT IGNORES SIGTERM.** Its `cat`
  /// waits on that pipe, and neither the shell nor `cat` can be ended by the
  /// signal, so only `terminate()` closing the pipe lets it print and exit.
  func testClosingTheHeldStdinEndsAChildThatIgnoresSIGTERM() async throws {
    let child = try launch("trap '' TERM; printf 'ready\\n'; cat; printf 'stdin closed\\n'", holdStdin: true)
    defer { child.child.terminate() }
    let events = await drain(child.events) { event in
      if event == .line("ready") { child.child.terminate() }
    }
    XCTAssertEqual(events, [.line("ready"), .line("stdin closed"), .exited(0)])
  }

  /// **`terminate()` SIGNALS A CHILD THAT NEVER READS ITS STDIN.** `sleep` does
  /// not notice its stdin closing, so only the signal ends it.
  func testTerminateSignalsAChildThatNeverReadsItsStdin() async throws {
    let child = try launch("printf 'ready\\n'; exec sleep 600", holdStdin: true)
    defer { child.child.terminate() }
    let events = await drain(child.events) { event in
      if event == .line("ready") { child.child.terminate() }
    }
    XCTAssertEqual(events, [.line("ready"), .exited(SIGTERM)])
  }
}
