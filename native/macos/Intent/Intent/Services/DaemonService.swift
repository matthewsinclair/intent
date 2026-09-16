import Foundation
import Observation

/// The daemon as the app sees it: a poll, on the shared `Poller` cadence, of
/// `intent daemon status --format json` -- cc's projection, the ONE health
/// predicate (AC-01.2), read through the CLI verb and never reimplemented in
/// Swift -- plus Start / Stop / Restart through the same CLI, each noted in the
/// Console with its output (AC-03.3). Geodica's
/// CmsService with the HTTP probe swapped for the status verb.
///
/// The connect-then-lock ORDER that separates `live`/`stale`/`absent` is
/// load-bearing and lives in `intentsvcs::daemon` (cc mutation-tested it:
/// lock-first reports every healthy daemon as stale). This reads cc's answer;
/// it never recomputes the order.
@MainActor @Observable
final class DaemonService {
  static let shared = DaemonService()

  private(set) var health: Health = .unknown("not yet polled")
  /// "Starting…" / "Stopping…" / "Restarting…" while a lifecycle verb runs, so
  /// the icon does not flicker through a wrong state on a restart.
  private(set) var busy: String?

  private let poller = Poller()

  func startPolling() {
    poller.start { [weak self] in await self?.poll() }
  }

  func stopPolling() {
    poller.stop()
  }

  /// One read of the health predicate. `daemon status` reports live, stale or
  /// absent and its stdout is the answer whatever the exit code; a launch
  /// failure is `unknown`, never a silent `absent`. A lifecycle verb in flight
  /// owns the state, so a poll does not overwrite "Starting…".
  func poll() async {
    guard busy == nil else { return }
    do {
      let result = try await IntentCLI.capture(["daemon", "status", "--format", "json"])
      health = Health.decode(result.stdout)
    } catch {
      health = .unknown(error.localizedDescription)
    }
  }

  // MARK: - Lifecycle, through the CLI

  func start() async throws {
    try await lifecycle("Starting…", [["daemon", "start"]])
  }

  func stop() async throws {
    try await lifecycle("Stopping…", [["daemon", "stop"]])
  }

  /// The CLI's own `intent daemon restart`, which sequences the stop and the
  /// start and waits for the new daemon to answer (issue 0335).
  func restart() async throws {
    try await lifecycle("Restarting…", [["daemon", "restart"]])
  }

  private func lifecycle(_ label: String, _ commands: [[String]]) async throws {
    busy = label
    defer { busy = nil }
    do {
      for args in commands {
        let result = try await IntentCLI.capture(args)
        ConsoleRunner.shared.note(command: IntentCLI.label(args), result: result)
        _ = try IntentCLI.checked(result, args)
      }
    } catch {
      busy = nil
      await poll()
      throw error
    }
    busy = nil
    await poll()
  }
}
