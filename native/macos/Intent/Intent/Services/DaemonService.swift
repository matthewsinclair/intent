import Foundation
import OSLog
import Observation

/// The daemon as the app sees it: a five-second poll of
/// `intent daemon status --format json` -- cc's projection, the ONE health
/// predicate (AC-01.2), read through the CLI verb and never reimplemented in
/// Swift -- plus Start / Stop / Restart through the same CLI. Geodica's
/// CmsService with the HTTP probe swapped for the status verb.
///
/// The connect-then-lock ORDER that separates `live`/`stale`/`absent` is
/// load-bearing and lives in `intentsvcs::daemon` (cc mutation-tested it:
/// lock-first reports every healthy daemon as stale). This reads cc's answer;
/// it never recomputes the order.
@MainActor @Observable
final class DaemonService {
  static let shared = DaemonService()
  private static let logger = Logger(
    subsystem: "com.matthewsinclair.intent.macos", category: "DaemonService")

  private(set) var health: Health = .unknown("not yet polled")
  /// "Starting…" / "Stopping…" / "Restarting…" while a lifecycle verb runs, so
  /// the icon does not flicker through a wrong state on a restart.
  private(set) var busy: String?

  private var pollTask: Task<Void, Never>?

  func startPolling() {
    guard pollTask == nil else { return }
    pollTask = Task { [weak self] in
      while !Task.isCancelled {
        await self?.poll()
        try? await Task.sleep(for: .seconds(5))
      }
    }
  }

  func stopPolling() {
    pollTask?.cancel()
    pollTask = nil
  }

  /// One read of the health predicate. `daemon status` reports a state for all
  /// three cases and its stdout is the answer whatever the exit code; a launch
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

  /// intentd has no `restart` verb; stop then start, an order the CLI owns.
  func restart() async throws {
    try await lifecycle("Restarting…", [["daemon", "stop"], ["daemon", "start"]])
  }

  private func lifecycle(_ label: String, _ commands: [[String]]) async throws {
    busy = label
    defer { busy = nil }
    do {
      for args in commands {
        let out = try await IntentCLI.run(args)
        Self.logger.info(
          "\(args.joined(separator: " "), privacy: .public): \(out.trimmingCharacters(in: .whitespacesAndNewlines), privacy: .public)"
        )
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
