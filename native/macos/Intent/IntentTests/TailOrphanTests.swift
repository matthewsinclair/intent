import XCTest

@testable import Intent

/// ST0064 `AC-01.4`: the tail-orphan trap, verified BEFORE any console is built
/// on it, against SIGTERM, SIGINT and SIGKILL **separately**.
///
/// **THE THREE SIGNALS DO NOT SHARE A PATH, WHICH IS WHY THE ROW NAMES THEM
/// SEPARATELY.** A handler that cleans up on SIGTERM and SIGINT proves nothing
/// about SIGKILL, which cannot be handled at all -- so the orphan has to be
/// prevented by construction rather than by a shutdown hook, and only a probe
/// that actually sends all three can tell those two designs apart.
///
/// **THE CONTROL ARM IS NOT DECORATION, IT IS WHAT MAKES THE PASS MEAN
/// ANYTHING.** `no tail is running` is also the answer you get from a probe
/// that never started one, from a probe that started one and lost its pid, and
/// from a probe whose fixture silently failed to launch. The `plain` arm spawns
/// the pipeline the way an unguarded runtime would and **must LEAK in every
/// cell**; if it ever comes back clean, this file is measuring nothing and the
/// guarded results are worthless. Six cells, two arms, three signals, and the
/// arms must disagree in all three.
///
/// **THE PRECONDITION `0281`'s RULING CARRIES IS ASSERTED INSIDE THE PROBE, NOT
/// HERE.** The runtime must be its own process-group leader or a group kill
/// names somebody else's group -- and a test that only checked "the tail is
/// gone" would PASS ON A BUILD THAT KILLED THE APP, since both leave no tail
/// running and only one of them is the fix. The probe refuses rather than
/// reporting a clean run it cannot justify.
///
/// The process logic lives in `tail-orphan-probe.sh` beside this file rather
/// than in Swift, because the subject is POSIX process groups and pipe EOF --
/// expressing that through `Process` would put a Foundation layer between the
/// test and the thing it is testing, which is the layer `0281` is about.
final class TailOrphanTests: XCTestCase {
  private static let signals = ["TERM", "INT", "KILL"]

  private var probePath: String {
    URL(fileURLWithPath: #filePath)
      .deletingLastPathComponent()
      .appendingPathComponent("tail-orphan-probe.sh")
      .path
  }

  /// Runs one cell and returns the probe's LAST line, which is the verdict
  /// TOKEN and carries nothing else: `clean`, `LEAKED`, `probe-indeterminate`,
  /// or a `probe-error:` / `probe-refuse:` line, which are failures rather than
  /// results.
  ///
  /// **THE TOKEN IS DELIBERATELY BARE AND THE EVIDENCE IS THE LINE ABOVE IT.**
  /// The probe first emitted `LEAKED (tail 16960 alive, reparented to ppid=1,
  /// ...)` and every control cell then failed an exact-equality assertion while
  /// the probe was behaving perfectly -- the producer gained information and
  /// this consumer still demanded the old form. **Relaxing the assertion to
  /// `hasPrefix` was the cheap fix and the wrong one**: it keeps passing if the
  /// evidence text later becomes wrong, because nothing would read it. A bare
  /// token plus a separate evidence line keeps both assertable.
  private func runProbe(arm: String, signal: String, stateDir: URL) throws -> String {
    let process = Process()
    // **`/bin/bash` IS 3.2.57 ON macOS AND IS NOT `bash` ON PATH (5.3.15 here).**
    // That difference is not incidental: the probe's first version used
    // `BASHPID`, which is bash 4.0+, so every cell died before writing a pid
    // file under `xcodebuild` while running perfectly for its author. Drive the
    // probe with `/bin/bash` when checking it by hand, never with `bash`.
    process.executableURL = URL(fileURLWithPath: "/bin/bash")
    process.arguments = [probePath, arm, signal, stateDir.path]
    let pipe = Pipe()
    process.standardOutput = pipe
    process.standardError = pipe
    try process.run()
    let data = pipe.fileHandleForReading.readDataToEndOfFile()
    process.waitUntilExit()
    let out = String(data: data, encoding: .utf8) ?? ""
    return out.split(separator: "\n").last.map(String.init) ?? ""
  }

  private func withStateDir(_ body: (URL) throws -> Void) rethrows {
    let dir = URL(fileURLWithPath: NSTemporaryDirectory())
      .appendingPathComponent("tail-orphan-\(UUID().uuidString)")
    try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
    defer { try? FileManager.default.removeItem(at: dir) }
    try body(dir)
  }

  /// The remedy ruled for `0281` (option (i)): the wrapper reads its own stdin,
  /// the runtime holds the write end, and the runtime's death closes it however
  /// it dies. No tail survives any of the three signals.
  func testGuardedPipelineLeavesNoOrphanUnderAnySignal() throws {
    try withStateDir { dir in
      for signal in Self.signals {
        let verdict = try runProbe(arm: "guarded", signal: signal, stateDir: dir)
        XCTAssertEqual(
          verdict, "clean",
          "guarded arm under SIG\(signal): expected no orphan, got \(verdict)")
      }
    }
  }

  /// THE CONTROL. An unguarded runtime leaks a tail under every signal. If this
  /// test ever passes-as-clean, the probe has stopped being able to see the
  /// failure and the guarded result above means nothing.
  func testPlainPipelineLeaksUnderEverySignalSoTheProbeCanSeeTheFailure() throws {
    try withStateDir { dir in
      for signal in Self.signals {
        let verdict = try runProbe(arm: "plain", signal: signal, stateDir: dir)
        XCTAssertEqual(
          verdict, "LEAKED",
          "control arm under SIG\(signal): the probe must be able to observe a leak, got \(verdict)")
      }
    }
  }

  /// RIG SELF-TEST. The probe has a third verdict -- `probe-indeterminate`, for
  /// when the tail is alive and the runtime has not been reaped, so neither
  /// `clean` nor `LEAKED` is available -- and this asserts it can actually be
  /// produced.
  ///
  /// **IT EXISTS BECAUSE THE THIRD VERDICT WAS UNREACHABLE WHEN IT WAS FIRST
  /// WRITTEN, AND ONLY TRYING TO FIRE IT FOUND THAT OUT.** Shrinking the poll
  /// budget to a single tick did not reach it: on this machine reparenting is
  /// effectively instantaneous, so the runtime is already gone and the tail
  /// already shows a new parent by the first iteration. **A verdict that cannot
  /// be produced is decoration, and it would have been decoration in the very
  /// branch added to stop a misreading.**
  ///
  /// The `stubborn` arm traps the signal so the runtime SURVIVES it, which is
  /// the one state where the tail is alive under its ORIGINAL parent. The
  /// negative control is the same arm under SIGKILL, which cannot be trapped
  /// and must therefore come back LEAKED -- so the indeterminate is a property
  /// of the runtime surviving rather than of the arm being special.
  func testTheProbeCanReportIndeterminateRatherThanGuessing() throws {
    try withStateDir { dir in
      let stuck = try runProbe(arm: "stubborn", signal: "TERM", stateDir: dir)
      XCTAssertEqual(
        stuck, "probe-indeterminate",
        "the probe must be able to say it does not know, got: \(stuck)")

      let killed = try runProbe(arm: "stubborn", signal: "KILL", stateDir: dir)
      XCTAssertEqual(
        killed, "LEAKED",
        "negative control: SIGKILL cannot be trapped, so the same arm must reach a real verdict, got: \(killed)")
    }
  }
}
