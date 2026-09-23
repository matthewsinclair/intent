import Foundation
import OSLog
import Observation

/// What the app knows about the CLI's version. **A FAILED READ IS NOT A
/// PENDING ONE**: both would otherwise render as the app's bare name, and an
/// operator could not tell a CLI that has not answered yet from one that
/// cannot answer -- the same reason `Health` carries `unknown` with its cause.
enum VersionState: Sendable, Equatable {
  case pending
  case answered(String)
  case failed(String)

  /// The identity row's text: the CLI's version once it has answered, the
  /// app's name before then, and the name marked unavailable after a failure.
  var menuTitle: String {
    switch self {
    case .pending: "Intent"
    case .answered(let title): title
    case .failed: "Intent (version unavailable)"
    }
  }
}

/// The version of the `intent` this app drives, read through `intent version`
/// and rendered as the menu's identity row (hv, 2026-09-13). **IT IS THE CLI'S
/// VERSION, NOT THE APP'S**: every fact the menu shows comes from the CLI or the
/// daemon (AC-01.1), and the build a person is asking about is the `intent` the
/// app shells, which is rebuilt far more often than the app is. The app's own
/// Info.plist stamps stay where they are (`Bundle.intentVersionString`).
@MainActor @Observable
final class VersionService {
  static let shared = VersionService()
  private static let logger = AppLog.logger("VersionService")

  private(set) var state: VersionState = .pending

  private let poller = Poller()

  /// Polled rather than read once, so a pair rebuilt under a running app shows
  /// its new commit within one poll rather than at the next launch.
  func startPolling() {
    poller.start { [weak self] in await self?.refresh() }
  }

  func stopPolling() {
    poller.stop()
  }

  /// One read of `intent version`. A failure is logged and carried as `failed`
  /// with its cause, never a title kept from an earlier read. The state is
  /// assigned only when it changes, so an unchanged answer does not repaint the
  /// menu every poll.
  func refresh() async {
    let next: VersionState
    do {
      let output = try await IntentCLI.run(["version"])
      if let title = Self.menuTitle(output) {
        next = .answered(title)
      } else {
        Self.logger.error("intent version printed nothing")
        next = .failed("intent version printed nothing")
      }
    } catch {
      Self.logger.error("intent version failed: \(error.localizedDescription, privacy: .public)")
      next = .failed(error.localizedDescription)
    }
    if next != state { state = next }
  }

  /// `intent version` prints `intent <version> (<commit>) <kind>`, where the
  /// commit is a full sha, `dirty-<sha>`, or `unknown`, and the kind is
  /// `release` or `dev` (spine.rs, issue 0534). A full sha, SHA-1 or SHA-256, is
  /// shortened to its FIRST eight characters, a `dirty-` marker is KEPT so a
  /// dirty build never reads as a clean one, and the kind is kept after it. A
  /// line with no kind, as an `intent` before 3.2.1 prints, reads the same way
  /// without one. Any other shape is rendered as the CLI printed it rather than
  /// guessed at; empty output is nil. Pure and `nonisolated`, like
  /// Health.decode, so its tests run off the main actor.
  nonisolated static func menuTitle(_ output: String) -> String? {
    let line = output.split(whereSeparator: \.isNewline).first.map(String.init) ?? ""
    let trimmed = line.trimmingCharacters(in: .whitespaces)
    guard !trimmed.isEmpty else { return nil }
    let shape = /^(\S+) (\S+) \((dirty-)?([0-9a-f]{64}|[0-9a-f]{40})\)( release| dev)?$/
    guard let match = trimmed.wholeMatch(of: shape) else { return trimmed }
    let dirty = match.output.3 ?? ""
    let kind = match.output.5 ?? ""
    return "\(match.output.1) \(match.output.2) (\(dirty)\(match.output.4.prefix(8)))\(kind)"
  }
}
