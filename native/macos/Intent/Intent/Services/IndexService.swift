import Foundation
import Observation

/// Why a rebuild did not succeed, in the CLI's own lines (`IndexStatus.failure`).
enum IndexError: LocalizedError, Equatable {
  /// The rebuild ran and exited non-zero.
  case rebuildFailed(String)

  var errorDescription: String? {
    switch self {
    case .rebuildFailed(let said): said
    }
  }
}

/// The search index as the menu's Rebuild Search Index row shows it (ST0090
/// WP-02), in the shape of Gtools' `CmsService.refresh`: the rebuild runs through
/// the app's log, which records it as one block and opens nothing, and the
/// stamp is read after it.
///
/// **THE STAMP IS READ AT LAUNCH, AFTER EACH REBUILD AND AS THE MENU OPENS.** No
/// read the polls already run can carry it: GraphQL has no index field, and
/// `daemon status --format json` names only the daemon. A second process on
/// every poll is what the row must not cost, and a read as the menu opens is one
/// per look, which keeps the line true between rebuilds, since intentd's
/// watcher stamps the index after every source edit.
@MainActor @Observable
final class IndexService {
  static let shared = IndexService()
  private static let logger = AppLog.logger("Index")

  /// The index as the CLI last named it: nil before a read answers, and after
  /// one that failed.
  private(set) var status: IndexStatus?
  /// Which read owns `status`: a read that answers after a later one began
  /// keeps nothing, so an older stamp never replaces a newer one.
  @ObservationIgnored private var reads = 0

  /// What runs a command: `IntentCLI.capture` in the app, a stub in the tests.
  typealias Capture = LogService.Capture
  /// Whether a project is configured: `projectConfigured` in the app, and the
  /// test's own answer in the tests, so no arm depends on the test host's
  /// defaults.
  typealias Configured = () -> Bool

  /// The app's answer, asked of `ProjectConfig.configuredRoot`, the one home of
  /// it, as `ProjectService.refresh` asks it.
  nonisolated static func projectConfigured() -> Bool {
    ProjectConfig.configuredRoot() != nil
  }

  /// `intent index status --json`, into `status`. A read that fails is nil and
  /// logged, never an age the CLI did not name.
  ///
  /// **WITH NO PROJECT CONFIGURED, NOTHING IS READ**, as `ProjectService` reads
  /// nothing without one: no command runs, `status` is nil and nothing is
  /// logged, where a read would only fail in the app's own directory.
  func read(
    capture: Capture = IntentCLI.capture,
    configured: Configured = IndexService.projectConfigured
  ) async {
    reads += 1
    let mine = reads
    guard configured() else {
      status = nil
      return
    }
    let answer: IndexStatus?
    do {
      let out = try IntentCLI.checked(
        await capture(IndexStatus.statusCommand), IndexStatus.statusCommand)
      answer = try IndexStatus.decode(Data(out.utf8))
    } catch {
      answer = nil
      Self.logger.error(
        "\(IntentCLI.label(IndexStatus.statusCommand), privacy: .public) failed: \(error.localizedDescription, privacy: .public)"
      )
    }
    guard mine == reads else { return }
    status = answer
  }

  /// One rebuild for the operator, as Gtools' refresh is one: recorded in the
  /// app's log as one block, opening nothing, with the stamp read after it. A
  /// rebuild that cannot run throws its reason, recorded as failed; one that
  /// exits non-zero throws `rebuildFailed` with the CLI's own lines. The app
  /// raises either as `IndexStatus.rebuildFailedTitle`. The rebuild runs with or
  /// without a configured project, so the operator's own click is always
  /// answered; only the read after it asks `configured`.
  func rebuild(
    log: LogService,
    capture: Capture = IntentCLI.capture,
    configured: Configured = IndexService.projectConfigured
  ) async throws {
    let ran: LogService.Ran
    do {
      ran = try await log.run(IndexStatus.rebuildCommand, capture: capture)
    } catch {
      await read(capture: capture, configured: configured)
      throw error
    }
    await read(capture: capture, configured: configured)
    guard ran.result.isSuccess else {
      throw IndexError.rebuildFailed(IndexStatus.failure(ran.result))
    }
  }
}
