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

  /// Runs one cell and returns the probe's last line: `clean`, `LEAKED`, or a
  /// `probe-error:` / `probe-refuse:` line, which are failures rather than
  /// results.
  private func runProbe(arm: String, signal: String, stateDir: URL) throws -> String {
    let process = Process()
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
}
