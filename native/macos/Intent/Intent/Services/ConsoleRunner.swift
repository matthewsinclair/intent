import Foundation
import Observation

/// What the Console shows: a child `intent daemon logs --follow`, streamed line
/// by line. The verb owns the tailing; the app only colours lines. Owns the
/// backlog, so a line the app writes while the window is closed is there when
/// it opens. Copied from Gtools' ConsoleRunner (ST0075: copied, not shared).
@MainActor @Observable
final class ConsoleRunner {
  static let shared = ConsoleRunner()

  static let capacity = 5_000

  private(set) var ring = LineRing(capacity: ConsoleRunner.capacity)
  private(set) var tailing = false
  /// What the verb's first line said it is tailing, for the footer (AC-02.1).
  private(set) var header = TailHeader.awaiting

  /// The view attaches here for incremental updates; the backlog is `ring`. Not
  /// observed: nothing renders from the attachment itself.
  @ObservationIgnored weak var sink: ConsoleSink?

  /// The running tail: the one task reading its events, and the child they
  /// come from.
  @ObservationIgnored private var tail: (reader: Task<Void, Never>, child: RunningProcess)?

  // MARK: - The tail

  /// **THE CHILD'S STDIN IS HELD, AND THAT IS WHAT ENDS IT.** The verb follows
  /// until its own stdin closes (issue 0281's ruling, built in `daemon_logs`), so
  /// `stopTail()` closing the pipe, or the app dying however it dies, takes the
  /// verb and its `tail` down together.
  ///
  /// **A START REPLACES THE LINES AN EARLIER TAIL SUPPLIED** and keeps the app's
  /// own (`ConsoleLine.Source`), because the verb replays each log's last lines
  /// whenever it starts. It drops them only once the verb has launched: a verb
  /// that cannot launch replays nothing, so the earlier lines stay beside the
  /// reason. Either way the view renders the backlog again.
  func startTail() {
    guard tail == nil else { return }
    let stream: IntentCLI.StreamedChild
    do {
      stream = try IntentCLI.stream(["daemon", "logs", "--follow"], holdStdin: true)
    } catch {
      sink?.consoleDidReset()
      append(ConsoleLine(text: error.localizedDescription, kind: .error, source: .app))
      return
    }
    ring.removeAll(from: .tail)
    sink?.consoleDidReset()
    header = .awaiting
    tailing = true
    let reader = Task { [weak self] in
      for await event in stream.events {
        // A stopped tail is let go: once its reader is cancelled, nothing the
        // child still writes reaches the backlog.
        guard let self, !Task.isCancelled else { continue }
        switch event {
        case .line(let text): self.received(text)
        case .exited(let status): self.tailExited(status)
        }
      }
    }
    tail = (reader: reader, child: stream.child)
  }

  func stopTail() {
    tail?.reader.cancel()
    tail?.child.terminate()
    tail = nil
    tailing = false
  }

  private func received(_ text: String) {
    let read = Self.read(text, after: header)
    header = read.header
    if let line = read.line { append(line) }
  }

  private func tailExited(_ status: Int32) {
    tail = nil
    tailing = false
    if status != 0 {
      append(ConsoleLine(text: "intent daemon logs exited \(status)", kind: .error, source: .app))
    }
  }

  /// What one line of the tail does, given the header so far. The verb's first
  /// line is its header, `tailing <intentd.log> and <intentd.err.log>`
  /// (`daemon_logs` in render.rs, a contract), which names the footer's logs and
  /// goes no further.
  ///
  /// **NO LATER LINE IS READ AS THE HEADER.** A log line that happens to open
  /// "tailing " reaches the backlog like any other, and a first line that is not
  /// the header leaves the footer saying so rather than waiting for one.
  nonisolated static func read(_ text: String, after header: TailHeader) -> (header: TailHeader, line: ConsoleLine?) {
    guard header == .awaiting else { return (header, ConsoleLine(tail: text)) }
    if let logs = tailedLogs(text) { return (.logs(logs), nil) }
    return (.unrecognised, ConsoleLine(tail: text))
  }

  /// The two logs in the verb's header, or nil for any other line.
  ///
  /// **BOTH LOGS LIVE IN ONE DIRECTORY, SO THE SPLIT IS THE ONE WHOSE HALVES
  /// SHARE A PARENT.** A home directory may itself contain " and ", and splitting
  /// at the first occurrence would then name two paths that are neither log.
  nonisolated static func tailedLogs(_ line: String) -> TailedLogs? {
    let prefix = "tailing "
    guard line.hasPrefix(prefix) else { return nil }
    let rest = String(line.dropFirst(prefix.count))
    var searched = rest.startIndex..<rest.endIndex
    while let split = rest.range(of: " and ", range: searched) {
      let log = String(rest[..<split.lowerBound])
      let errLog = String(rest[split.upperBound...])
      if (log as NSString).deletingLastPathComponent == (errLog as NSString).deletingLastPathComponent {
        return TailedLogs(log: log, errLog: errLog)
      }
      searched = split.upperBound..<rest.endIndex
    }
    return nil
  }

  // MARK: - The backlog

  func append(_ line: ConsoleLine) {
    let dropped = ring.append(line)
    if dropped > 0 { sink?.consoleDidTrim(dropped) }
    sink?.consoleDidAppend(line)
  }

  func clear() {
    ring.clear()
    sink?.consoleDidReset()
  }
}

/// What the verb's first line said, for the footer.
enum TailHeader: Equatable, Sendable {
  case awaiting  // the tail has printed nothing yet
  case logs(TailedLogs)  // `tailing <intentd.log> and <intentd.err.log>`
  case unrecognised  // the first line was not the header, and went to the backlog
}

/// The two files `intent daemon logs` names in its header.
struct TailedLogs: Equatable, Sendable {
  let log: String
  let errLog: String
}

@MainActor
protocol ConsoleSink: AnyObject {
  func consoleDidAppend(_ line: ConsoleLine)
  func consoleDidTrim(_ count: Int)
  /// The backlog changed other than at its ends: render it again from `ring`.
  func consoleDidReset()
}
