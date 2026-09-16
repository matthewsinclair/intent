import Foundation
import Observation

/// Why the Console would not run a command.
enum ConsoleError: LocalizedError, Equatable {
  /// Another command is running; it is named, and nothing is queued (AC-03.4).
  case busy(String)
  /// The command's output ended with no exit status, so how it went is unknown.
  case noExitStatus(String)

  var errorDescription: String? {
    switch self {
    case .busy(let label): "The Console is still running `\(label)`. One command at a time."
    case .noExitStatus(let label): "`\(label)` ended without an exit status."
    }
  }
}

/// What the Console shows: a child `intent daemon logs --follow`, streamed line
/// by line, with the one-off commands the menu runs -- doctor, the index
/// rebuild, and the daemon's lifecycle verbs -- between `»` markers. The verb
/// owns the tailing; the app only colours lines. Owns the
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
  /// The one-off command running now, as the operator would type it.
  private(set) var commandRunning: String?

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
  /// own (`ConsoleLine.Source`), because the verb replays the logs' last lines
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

  // MARK: - One-off commands

  /// What starts a one-off: its args in, its events out. `IntentCLI.stream` in
  /// the app; a stub in the tests.
  typealias Launch = ([String]) throws -> AsyncStream<IntentCLI.StreamEvent>

  /// `» intent doctor`, the command's lines, then `» exit 0 · 1.2s` (AC-03.1,
  /// AC-03.2), and the exit status back to the caller. **A NON-ZERO EXIT IS NOT
  /// THROWN**: the command's own lines already say what went wrong, in the
  /// window the caller brought forward.
  ///
  /// **ONE AT A TIME, REFUSED RATHER THAN QUEUED** (AC-03.4): a second command
  /// while one runs throws `busy` naming the running one, before it writes
  /// anything. A command that cannot launch at all throws too, after its
  /// reason and a `» failed` marker, so the caller alerts and the Console
  /// keeps the record; so does one whose output ends with no exit status.
  @discardableResult
  func run(
    _ args: [String],
    launch: Launch = { try IntentCLI.stream($0).events },
    now: () -> ContinuousClock.Instant = { ContinuousClock.now }
  ) async throws -> Int32 {
    let label = IntentCLI.label(args)
    if let running = commandRunning { throw ConsoleError.busy(running) }
    commandRunning = label
    defer { commandRunning = nil }

    append(.marker(label))
    let started = now()
    let events: AsyncStream<IntentCLI.StreamEvent>
    do {
      events = try launch(args)
    } catch {
      append(ConsoleLine(text: error.localizedDescription, kind: .error, source: .app))
      append(.marker("failed"))
      throw error
    }
    var status: Int32?
    for await event in events {
      switch event {
      case .line(let text): append(ConsoleLine(app: text))
      case .exited(let code): status = code
      }
    }
    guard let status else {
      let error = ConsoleError.noExitStatus(label)
      append(ConsoleLine(text: error.localizedDescription, kind: .error, source: .app))
      append(.marker("failed"))
      throw error
    }
    append(.marker(Self.exitMarker(status, after: now() - started)))
    return status
  }

  /// `exit 0 · 1.2s`.
  nonisolated static func exitMarker(_ status: Int32, after elapsed: Duration) -> String {
    let seconds = Double(elapsed.components.seconds) + Double(elapsed.components.attoseconds) / 1e18
    return "exit \(status) · \(String(format: "%.1f", seconds))s"
  }

  /// A command that ran elsewhere reports here as a marked block: Start, Stop
  /// and Restart intentd (AC-03.3), whose output went only to the system log
  /// before. Its stdout, then its stderr, then `» exit N`. It does not bring
  /// the Console forward.
  func note(command: String, result: CLIRunResult) {
    append(.marker(command))
    // Each stream split on its own: a stdout with no final newline must not
    // run into stderr's first line.
    for line in [result.stdout, result.stderr].flatMap({ $0.split(separator: "\n") }) {
      append(ConsoleLine(app: String(line)))
    }
    append(.marker("exit \(result.exitCode)"))
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
