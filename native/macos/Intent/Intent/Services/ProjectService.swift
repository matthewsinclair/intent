import Foundation
import OSLog
import Observation

/// The project as the app sees it, read through the ONE GraphQL door: the app
/// shells `intent graphql <query>` (IntentCLI, the single shell-out home) and
/// RENDERS what intentd returns, deriving nothing in Swift (AC-01.1, AC-01.3).
/// `intent graphql`'s own help: "Execute a GraphQL document against this project
/// through intentd -- the read-only escape hatch." Liveness stays the daemon's
/// `daemon status` predicate (DaemonService, AC-01.2); this is the app moving a
/// real GraphQL byte, which is what makes AC-01.3 refutable rather than true by
/// construction -- an app that only ran `daemon status` would move none.
///
/// It renders a COUNT of what the query returned and never classifies a status
/// or computes a project-side fact: knowing which statuses are terminal is the
/// daemon's, so keeping it out of Swift keeps AC-01.1's no-product-logic rule.
@MainActor @Observable
final class ProjectService {
  static let shared = ProjectService()
  private static let logger = Logger(
    subsystem: "com.matthewsinclair.intent.macos", category: "ProjectService")

  /// The number of steel threads intentd reports for the configured project, or
  /// nil when no project is configured or the query has not answered. The menu
  /// renders this; a failure is nil and logged, never a fabricated zero.
  private(set) var threadCount: Int?

  private var pollTask: Task<Void, Never>?

  /// Mirrors DaemonService's cadence: one GraphQL read every five seconds, so
  /// the count stays live and the query is exercised continuously.
  func startPolling() {
    guard pollTask == nil else { return }
    pollTask = Task { [weak self] in
      while !Task.isCancelled {
        await self?.refresh()
        try? await Task.sleep(for: .seconds(5))
      }
    }
  }

  func stopPolling() {
    pollTask?.cancel()
    pollTask = nil
  }

  /// One read through the GraphQL door. No configured project -> nil, and the
  /// menu shows nothing rather than a machine-level query's not-in-project
  /// error; a query the app cannot decode is nil, never a wrong count.
  func refresh() async {
    guard ProjectConfig.configuredRoot() != nil else {
      threadCount = nil
      return
    }
    do {
      let json = try await IntentCLI.run(["graphql", "{threads{id}}"])
      guard let count = Self.decodeThreadCount(json) else {
        Self.logger.error("graphql {threads{id}} returned an unrecognised shape")
        threadCount = nil
        return
      }
      threadCount = count
      Self.logger.info("graphql {threads{id}} -> \(count, privacy: .public) threads")
    } catch {
      Self.logger.error(
        "graphql {threads{id}} failed: \(error.localizedDescription, privacy: .public)")
      threadCount = nil
    }
  }

  /// Decode `intent graphql {threads{id}}` -> `{"data":{"threads":[{"id":…}]}}`.
  /// A shape the app does not recognise decodes to nil (the menu shows nothing),
  /// never to a wrong count -- Health.decode's unknown-tolerant contract. Pure
  /// and `nonisolated`: it reads its argument and touches no actor state, so it
  /// is callable off the main actor (its tests, like Health.decode's, are).
  nonisolated static func decodeThreadCount(_ text: String) -> Int? {
    let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
    guard let data = trimmed.data(using: .utf8), !data.isEmpty else { return nil }
    guard let envelope = try? JSONDecoder().decode(Envelope.self, from: data) else { return nil }
    return envelope.data.threads.count
  }

  private struct Envelope: Decodable {
    let data: DataField
    struct DataField: Decodable { let threads: [Thread] }
    struct Thread: Decodable { let id: String }
  }
}
