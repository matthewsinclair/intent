import Foundation

/// One line in the Console: its text, its colour, and where it came from. The
/// app has no opinion about the log format beyond this: what `intent daemon
/// logs` prints is what shows. Copied from Gtools' ConsoleLine (ST0075: copied,
/// not shared), with its request kind dropped, because intentd writes no
/// request lines to dim.
struct ConsoleLine: Sendable, Equatable {
  enum Kind: Sendable {
    case log  // everything intentd prints that is not one of the kinds below
    case error  // `error:` and `caused by:` lines
    case warning  // `warning:` and `remedy:` lines
    case marker  // `» ` lines, the accent
  }

  /// **A TAIL'S LINES ARE REPLACED WHEN A TAIL STARTS, AND THE APP'S ARE KEPT.**
  /// The verb replays the logs' last lines whenever it starts, so the lines an
  /// earlier tail supplied would show twice once the window reopens, and a log
  /// that grew past the replay while it was closed would leave a gap nothing
  /// marked. The app's own lines, such as the notice that a tail exited, are
  /// written once, so they stay: a line the app writes while the window is
  /// closed is there when it opens (AC-02.3).
  enum Source: Sendable {
    case tail  // printed by `intent daemon logs --follow`
    case app  // written by the app itself
  }

  let text: String
  let kind: Kind
  let source: Source

  static let markerPrefix = "» "

  /// **THE TOKENS ARE THE CLI'S OWN, INDENTED AS THE OPERATOR SEES THEM.** An
  /// error renders as `error: `, then `  caused by: ` for each cause, then
  /// `  remedy: ` (`intentsvcs::remedy`), so the causes and the remedy are
  /// matched after their indent. intentd's own notices open with its `warning: `
  /// or `error: ` token and carry a remedy line of their own (issue 0434), so a
  /// line with no token is a log line: the Console never guesses a severity from
  /// the words after `intentd:`.
  ///
  /// **`warning:` IS intentd's OWN SEVERITY AND IS COLOURED AS ONE** (vc's
  /// ruling (b) on AC-02.2, 2026-09-15). intentd prints its warnings with that
  /// prefix -- a connection it could not accept, a LaunchAgent it could not
  /// regenerate -- and a `remedy:` in the warning colour under an uncoloured
  /// warning would read backwards.
  ///
  /// **A STAMPED LINE CLASSIFIES AS THE LINE UNDER ITS STAMP.** intentd opens
  /// each line of both its logs with an RFC 3339 UTC timestamp and one space
  /// (vc's decision with issue 0321, 2026-09-15), so one leading token that
  /// parses as one is skipped before the tokens are matched. A line with no
  /// stamp classifies exactly as it would without this.
  static func kind(of text: String) -> Kind {
    let line = unstamped(text)
    if line.hasPrefix(markerPrefix) { return .marker }
    let body = line.drop(while: { $0 == " " })
    if body.hasPrefix("error:") || body.hasPrefix("caused by:") {
      return .error
    }
    if body.hasPrefix("warning:") || body.hasPrefix("remedy:") {
      return .warning
    }
    return .log
  }

  /// The text after one leading RFC 3339 UTC timestamp and its space, or all of
  /// it when it does not open with one.
  private static func unstamped(_ text: String) -> Substring {
    guard let space = text.firstIndex(of: " "), isTimestamp(text[..<space]) else { return text[...] }
    return text[text.index(after: space)...]
  }

  /// `YYYY-MM-DDTHH:MM:SS`, optional fractional seconds, then `Z`, with each
  /// field in its range: a token that only has the shape is not a stamp.
  private static func isTimestamp(_ token: Substring) -> Bool {
    let bytes = Array(token.utf8)
    let isDigit = { (byte: UInt8) in (UInt8(ascii: "0")...UInt8(ascii: "9")).contains(byte) }
    guard bytes.count >= 20, bytes.last == UInt8(ascii: "Z"),
      bytes[4] == UInt8(ascii: "-"), bytes[7] == UInt8(ascii: "-"), bytes[10] == UInt8(ascii: "T"),
      bytes[13] == UInt8(ascii: ":"), bytes[16] == UInt8(ascii: ":")
    else { return false }
    func field(_ range: Range<Int>) -> Int? {
      guard bytes[range].allSatisfy(isDigit) else { return nil }
      return bytes[range].reduce(0) { $0 * 10 + Int($1 - UInt8(ascii: "0")) }
    }
    guard field(0..<4) != nil,
      let month = field(5..<7), (1...12).contains(month),
      let day = field(8..<10), (1...31).contains(day),
      let hour = field(11..<13), hour <= 23,
      let minute = field(14..<16), minute <= 59,
      let second = field(17..<19), second <= 60
    else { return false }
    let fraction = bytes[19..<(bytes.count - 1)]
    guard let point = fraction.first else { return true }
    return point == UInt8(ascii: ".") && fraction.count > 1 && fraction.dropFirst().allSatisfy(isDigit)
  }
}

extension ConsoleLine {
  /// A line the tail printed, classified for colour.
  init(tail text: String) {
    self.init(text: text, kind: Self.kind(of: text), source: .tail)
  }

  /// A line of a command the app ran, classified for colour and kept when a
  /// tail starts (AC-03.1).
  init(app text: String) {
    self.init(text: text, kind: Self.kind(of: text), source: .app)
  }

  /// `» intent doctor`, `» exit 0 · 1.2s`: the app's own markers around a
  /// command it ran.
  static func marker(_ text: String) -> ConsoleLine {
    ConsoleLine(text: markerPrefix + text, kind: .marker, source: .app)
  }
}

/// A bounded backlog: the Console keeps the last `capacity` lines and says how
/// many it dropped, so the view can drop the same from its text.
struct LineRing: Sendable {
  let capacity: Int
  private(set) var lines: [ConsoleLine] = []

  init(capacity: Int) {
    self.capacity = capacity
  }

  /// Returns the number of leading lines discarded to stay within capacity.
  mutating func append(_ line: ConsoleLine) -> Int {
    lines.append(line)
    let excess = lines.count - capacity
    guard excess > 0 else { return 0 }
    lines.removeFirst(excess)
    return excess
  }

  /// Drops every line from one source, keeping the rest in order, and returns
  /// how many went.
  @discardableResult
  mutating func removeAll(from source: ConsoleLine.Source) -> Int {
    let before = lines.count
    lines.removeAll { $0.source == source }
    return before - lines.count
  }

  mutating func clear() {
    lines.removeAll()
  }
}
