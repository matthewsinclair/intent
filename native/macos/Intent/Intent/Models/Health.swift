import Foundation

/// The daemon's health as the app sees it, decoded from
/// `intent daemon status --format json` -- cc's projection above `route()`, the
/// ONE health predicate (AC-01.2), read through the CLI verb and never a Swift
/// reimplementation of the probe.
///
/// The three states and their REMEDIES are vc's AC-01.6 ruling: the state IS
/// the remedy, so the UI reads it rather than deriving a second fact beside it.
///
/// - `live`    answering at its endpoint. Stop / Restart, and -- when the daemon
///             published a loopback address that is answering -- `url`, the
///             browser-openable face. `url` is OPTIONAL on a live daemon and
///             that is the contract, not a gap: the CLI omits it when there is
///             nowhere to send a browser, so `nil` here means do not offer the
///             item rather than offer one that opens nothing.
/// - `stale`   a process holds the lock and is NOT answering. Investigate that
///             pid; NEVER offer to remove the socket it still owns (AC-08.12).
///             Restart recovers it.
/// - `absent`  nothing owns the endpoint; residue is safe to clear. Start.
/// - `unknown` not yet polled, or a line this build could not decode. It is
///             never rendered as one of the three, so a renamed `Health`
///             variant on the daemon side shows as unknown here rather than
///             silently as the wrong state -- cc's tripwire, mirrored in
///             `HealthTests`.
enum Health: Sendable, Equatable {
  case live(endpoint: String, url: String?)
  case stale(pid: Int)
  case absent
  case unknown(String)

  /// Decode one line of `daemon status --format json`. Key order on the wire is
  /// alphabetical (serde `BTreeMap`) and irrelevant to `Codable`.
  static func decode(_ text: String) -> Health {
    let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
    guard let data = trimmed.data(using: .utf8), !data.isEmpty else {
      return .unknown("empty status")
    }
    guard let dto = try? JSONDecoder().decode(DTO.self, from: data) else {
      return .unknown(trimmed)
    }
    switch dto.state {
    case "live": return .live(endpoint: dto.endpoint ?? "", url: dto.url)
    case "stale": return .stale(pid: dto.pid ?? 0)
    case "absent": return .absent
    default: return .unknown(dto.state)
    }
  }

  private struct DTO: Decodable {
    let state: String
    let endpoint: String?
    let pid: Int?
    /// The browser-openable face, `http://127.0.0.1:<port>`, present iff a
    /// loopback address is answering. **RENDERED, NEVER BUILT HERE** -- the
    /// port is assigned by the kernel and published by the daemon, so there is
    /// no constant to reconstruct it from and a Swift-side URL would be the
    /// second resolver `AC-01.1` forbids.
    let url: String?
  }

  /// The menu's one-line summary. `stale` names the pid and points the operator
  /// at it -- the remedy, never an unlink.
  var summary: String {
    switch self {
    case .live: "intentd is answering"
    case .stale(let pid): "intentd (pid \(pid)) holds the socket but is not answering -- investigate it"
    case .absent: "intentd is not running"
    case .unknown(let why): "intentd status unknown (\(why))"
    }
  }
}
