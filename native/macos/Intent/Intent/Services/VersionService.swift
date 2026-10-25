import Foundation
import OSLog
import Observation

/// The CLI's build, as `intent version` names it: its version, its commit when
/// the line carries one, and whether it is a release or a dev build when the
/// line says (issue 0534). Gtools' `CLIVersion`, with the kind Intent's line
/// carries, and since ST0087 the tree and the root the line names.
struct CLIVersion: Sendable, Equatable {
  let version: String
  let commit: String?
  let kind: String?
  /// `dev`, `opt`, `brew` or `unknown`, from a 3.2.7 line on; nil before.
  var tree: String? = nil
  /// The absolute root the line names, from a 3.2.7 line on; nil before.
  var root: String? = nil
}

/// What the app knows about the CLI's version. **A FAILED READ IS NOT A
/// PENDING ONE**: both would otherwise render as the app's bare name, and an
/// operator could not tell a CLI that has not answered yet from one that
/// cannot answer -- the same reason `Health` carries `unknown` with its cause.
enum VersionState: Sendable, Equatable {
  case pending
  case answered(CLIVersion)
  case failed(String)

  /// The CLI's answer, once it has given one; nil while pending and after a
  /// failure.
  var answer: CLIVersion? {
    if case .answered(let cli) = self { return cli }
    return nil
  }

  /// Why the read failed, for the masthead row's tooltip; nil unless it failed.
  var failure: String? {
    if case .failed(let why) = self { return why }
    return nil
  }
}

/// The version of the `intent` this app drives, read through `intent version`
/// for the menu's masthead row (hv, 2026-09-13; issue 0607). **IT IS THE CLI'S
/// VERSION, NOT THE APP'S**: every fact the menu shows comes from the CLI or the
/// daemon (AC-01.1), and the build a person is asking about is the `intent` the
/// app shells, which is rebuilt far more often than the app is. The app's own
/// stamps are on the About page (`Bundle.intentVersionString`).
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
  /// with its cause, never a version kept from an earlier read. The state is
  /// assigned only when it changes, so an unchanged answer does not repaint the
  /// menu every poll.
  func refresh() async {
    let next: VersionState
    do {
      let output = try await IntentCLI.run(["version"])
      if let cli = Self.parse(output) {
        next = .answered(cli)
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
  /// `release` or `dev` (spine.rs, issue 0534); an `intent` before 3.2.1 prints
  /// no kind. A line with no commit gives the version alone, and a line in any
  /// other shape, one that does not start with the program's name included, is
  /// all version, as the CLI printed it, rather than guessed at. Empty output is
  /// nil. Pure and `nonisolated`, so its tests run off the main actor.
  ///
  /// From 3.2.7 (ST0087 WP-01, the contract's T4) the line is `intent
  /// <version> (<tree>, <sha12>[+dirty]) at <root> <kind>`: the tree is `dev`,
  /// `opt`, `brew` or `unknown`, the sha is cut to eight with `+dirty` kept,
  /// `(dev, unknown)` is a dev tree whose git could not be read, and the root
  /// is absolute and may hold spaces. The root is optional: hv ruled on
  /// 2026-10-25 that the line drops ` at <root>`, so both forms read alike.
  nonisolated static func parse(_ output: String) -> CLIVersion? {
    let line = output.split(whereSeparator: \.isNewline).first.map(String.init) ?? ""
    let trimmed = line.trimmingCharacters(in: .whitespaces)
    guard !trimmed.isEmpty else { return nil }
    let treeShape = /^intent (\S+) \((dev|opt|brew|unknown), ([0-9a-f]{12}|unknown)(\+dirty)?\)(?: at (\/.*?))?(?: (release|dev))?$/
    if let match = trimmed.wholeMatch(of: treeShape) {
      let sha = String(match.output.3)
      let short = sha == "unknown" ? sha : String(sha.prefix(8))
      return CLIVersion(
        version: String(match.output.1), commit: short + (match.output.4.map(String.init) ?? ""),
        kind: match.output.6.map(String.init), tree: String(match.output.2),
        root: match.output.5.map(String.init))
    }
    let shape = /^intent (\S+)(?: \(([^()]+)\))?(?: (release|dev))?$/
    guard let match = trimmed.wholeMatch(of: shape) else {
      return CLIVersion(version: trimmed, commit: nil, kind: nil)
    }
    return CLIVersion(
      version: String(match.output.1), commit: match.output.2.map { shortened(String($0)) },
      kind: match.output.3.map(String.init))
  }

  /// The commit is a full sha, `dirty-<sha>`, or `unknown`. A full sha, SHA-1
  /// or SHA-256, is cut to its first eight characters and a `dirty-` marker is
  /// kept, so a dirty build never reads as a clean one; any other word is kept
  /// as the CLI printed it.
  private nonisolated static func shortened(_ commit: String) -> String {
    guard let match = commit.wholeMatch(of: /(dirty-)?([0-9a-f]{64}|[0-9a-f]{40})/) else { return commit }
    return "\(match.output.1 ?? "")\(match.output.2.prefix(8))"
  }
}
