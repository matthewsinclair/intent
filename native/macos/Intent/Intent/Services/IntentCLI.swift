import Foundation
import OSLog
import os

/// Result of a completed `intent` invocation.
struct CLIRunResult: Sendable {
  let exitCode: Int32
  let stdout: String
  let stderr: String

  var isSuccess: Bool { exitCode == 0 }
}

enum IntentCLIError: LocalizedError {
  case binaryNotFound
  case failedToLaunch(underlying: String)
  case commandFailed(command: String, exitCode: Int32, stderr: String)
  case projectRootInvalid(path: String)

  var errorDescription: String? {
    switch self {
    case .binaryNotFound:
      "intent was not found on the login shell's PATH. Settings -> Estate -> Locate…"
    case .failedToLaunch(let underlying):
      "Could not launch intent: \(underlying)"
    case .commandFailed(let command, let code, let stderr):
      "`\(command)` exited \(code): \(stderr)"
    case .projectRootInvalid(let path):
      "the configured project root is not an Intent project: \(path) (no intent/.config/config.json). Settings -> Estate -> set a project directory."
    }
  }
}

/// Which Intent project this menubar is pointed at.
///
/// INTERIM, and it knows it: this is a per-app-instance stand-in for D07's
/// machine registry ("one intentd per machine, N projects, per-project DBs,
/// REGISTRY"). intentd holds that registry -- projects registered on first
/// contact (`intentd/src/registry.rs`, ST0056 AC-08.1) -- but no `intent` verb
/// surfaces it, and the app reaches intentd only through verbs. When a verb
/// does, the app READS IT and this local store goes away -- it must never
/// become a parallel home that has to agree with the registry forever, which is
/// issue 0204's shape. A short-lived second home that knows it is second is
/// fine; one that forgets is not -- the two would then diverge silently, and the
/// disagreement never announces itself (0206), so the exit is to READ the
/// registry once a verb surfaces it, never to keep this store in sync with it.
/// (vc ruling (a) + condition (ii), 2026-08-31.)
///
/// Per-app-instance is the right scope regardless of D07: a machine holds many
/// Intent projects, so "which project is this app controlling" is app state,
/// like a window position -- not a machine-level fact. The root is stored as a
/// path in the `IntentProjectRoot` user default, which nothing in the app sets
/// (there is no settings UI): `defaults write com.matthewsinclair.intent.macos
/// IntentProjectRoot <dir>`. Every child IntentCLI spawns runs with it as the
/// working directory, so verbs resolve by CWD walk-up exactly as in a terminal
/// (no new resolution path, no flag for `edit`/`graphql` to learn).
enum ProjectConfig {
  static let rootKey = "IntentProjectRoot"

  /// The configured project root, or nil if none is set.
  static func configuredRoot() -> String? {
    guard let root = UserDefaults.standard.string(forKey: rootKey), !root.isEmpty else { return nil }
    return root
  }

  /// A directory is an Intent project iff it carries `intent/.config/config.json`.
  static func isIntentProject(_ path: String) -> Bool {
    let marker = (path as NSString).appendingPathComponent("intent/.config/config.json")
    var isDir: ObjCBool = false
    return FileManager.default.fileExists(atPath: marker, isDirectory: &isDir) && !isDir.boolValue
  }
}

/// The one shell-out path to `intent`, with the login shell's PATH handed to
/// every child. The only other spawn in the app is LoginShell's `/bin/zsh`
/// probe that finds that PATH, so binary resolution, environment and error
/// mapping live here once. (AC-01.1 / AC-01.9)
enum IntentCLI {
  static let overrideKey = "IntentBinary"

  /// The `IntentBinary` user default wins when it names an executable (set with
  /// `defaults write`; there is no settings UI); otherwise what LoginShell found.
  static func binary() -> String? {
    if let override = UserDefaults.standard.string(forKey: overrideKey), !override.isEmpty,
      FileManager.default.isExecutableFile(atPath: override)
    {
      return override
    }
    return LoginShellStore.shared.current().intent
  }

  static func environment() -> [String: String] {
    var env = ProcessInfo.processInfo.environment
    env["PATH"] = LoginShellStore.shared.current().path
    return env
  }

  /// The child's working directory: the configured project root, VALIDATED as an
  /// Intent project. nil means none is configured -- machine-level verbs still
  /// work and a project verb gets the CLI's own not-in-project error, which is
  /// honest. A configured-but-invalid root is a LOUD refusal (condition (i)),
  /// never a silent spawn into a directory where CWD walk-up finds a different
  /// project or none. With no settings UI to validate on set, this runtime check
  /// is the only guard. (AC-01.3/01.5.)
  static func projectDirectory() throws -> URL? {
    guard let root = ProjectConfig.configuredRoot() else { return nil }
    guard ProjectConfig.isIntentProject(root) else {
      throw IntentCLIError.projectRootInvalid(path: root)
    }
    return URL(fileURLWithPath: root, isDirectory: true)
  }

  /// Runs and returns stdout; a non-zero exit is an error carrying stderr.
  static func run(_ args: [String]) async throws -> String {
    try checked(await capture(args), args)
  }

  /// A captured result's stdout, or the error its non-zero exit is.
  static func checked(_ result: CLIRunResult, _ args: [String]) throws -> String {
    guard result.isSuccess else {
      throw IntentCLIError.commandFailed(
        command: label(args),
        exitCode: result.exitCode,
        stderr: result.stderr.trimmingCharacters(in: .whitespacesAndNewlines)
      )
    }
    return result.stdout
  }

  /// The command as the operator would type it: `intent doctor`.
  static func label(_ args: [String]) -> String {
    (["intent"] + args).joined(separator: " ")
  }

  /// Runs and returns the full result whatever the exit code.
  static func capture(_ args: [String]) async throws -> CLIRunResult {
    guard let binary = binary() else { throw IntentCLIError.binaryNotFound }
    let env = environment()
    let cwd = try projectDirectory()
    return try await Task.detached(priority: .userInitiated) {
      try runProcess(binary: binary, args: args, env: env, cwd: cwd)
    }.value
  }

  /// One thing a streamed child did: printed a line (its stdout and stderr
  /// merged, in the order they were read), or exited, which is always the last
  /// event.
  enum StreamEvent: Sendable, Equatable {
    case line(String)
    case exited(Int32)
  }

  /// A launched child: what it does, as events, and the handle that ends it
  /// early.
  struct StreamedChild: Sendable {
    let events: AsyncStream<StreamEvent>
    let child: RunningProcess
  }

  /// Launches `intent <args>` and returns at once: lines arrive on `events` in
  /// the order the child wrote them, then its exit.
  ///
  /// **NO THREAD WAITS ON THE CHILD.** The pipe's readability handler and the
  /// process's termination handler feed the events, so a child that runs for as
  /// long as the Console is open holds no thread of the concurrency pool, and
  /// the one task reading `events` is what a stop cancels.
  ///
  /// **`holdStdin` IS HOW A FOLLOWING VERB ENDS, AND THE CONSOLE PASSES IT.**
  /// Issue `0281`'s ruling (option (i), vc under hv's pen, 2026-09-09) is built:
  /// `intent daemon logs --follow` runs its tail under a shell that reads its
  /// own stdin, and the verb ends when ITS stdin closes. So the app gives that
  /// child a stdin pipe and holds the write end: `RunningProcess.terminate()`
  /// closes it, and the app's death closes it however the app dies, SIGKILL
  /// included. Without the pipe a GUI app's child inherits an empty stdin, and
  /// the verb would end the moment it started.
  ///
  /// **THE ALTERNATIVE WAS DECLINED FOR A REASON WORTH KEEPING HERE.** Giving
  /// the app the job means `kill(-pgid, SIGTERM)`, which works only because a
  /// probe measured `child.pgid == child.pid` on one Foundation on one day --
  /// no documented guarantee. If that ever stops holding, the child inherits
  /// the app's group and `-pgid` names THE APP: the remedy does not degrade to
  /// doing nothing, it degrades to killing the app. **Anything that later
  /// reaches for group signalling must assert `child.pgid == child.pid` FIRST,
  /// because a test that only checks the tail is gone PASSES ON A BUILD THAT
  /// KILLED THE APP.**
  static func stream(_ args: [String], holdStdin: Bool = false) throws -> StreamedChild {
    guard let binary = binary() else { throw IntentCLIError.binaryNotFound }
    let env = environment()
    let cwd = try projectDirectory()
    return try launchStreaming(binary: binary, args: args, env: env, cwd: cwd, holdStdin: holdStdin)
  }

  // MARK: - Processes

  private static func runProcess(binary: String, args: [String], env: [String: String], cwd: URL?)
    throws -> CLIRunResult
  {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: binary)
    process.arguments = args
    process.environment = env
    process.currentDirectoryURL = cwd

    let stdout = Pipe()
    let stderr = Pipe()
    process.standardOutput = stdout
    process.standardError = stderr

    do {
      try process.run()
    } catch {
      throw IntentCLIError.failedToLaunch(underlying: error.localizedDescription)
    }
    // Read both before waiting: a chatty child fills a pipe and blocks.
    // Issue 0335: and read them AT THE SAME TIME. Reading stdout to EOF first
    // deadlocks a child that fills its stderr pipe before it closes stdout.
    let errBox = DataBox()
    let errRead = DispatchGroup()
    errRead.enter()
    DispatchQueue.global(qos: .userInitiated).async {
      errBox.set(stderr.fileHandleForReading.readDataToEndOfFile())
      errRead.leave()
    }
    let outData = stdout.fileHandleForReading.readDataToEndOfFile()
    errRead.wait()
    let errData = errBox.get()
    process.waitUntilExit()

    // Bytes that are not valid UTF-8 show as U+FFFD rather than as an empty
    // answer, which a caller would read as "printed nothing".
    return CLIRunResult(
      exitCode: process.terminationStatus,
      stdout: String(decoding: outData, as: UTF8.self),
      stderr: String(decoding: errData, as: UTF8.self)
    )
  }

  /// One pipe's bytes, handed from the reading queue to the caller.
  private final class DataBox: @unchecked Sendable {
    private let lock = NSLock()
    private var data = Data()

    func set(_ value: Data) {
      lock.lock()
      defer { lock.unlock() }
      data = value
    }

    func get() -> Data {
      lock.lock()
      defer { lock.unlock() }
      return data
    }
  }

  /// A streamed child's output, assembled into lines and handed to its events.
  /// The readability handler and the termination handler run on queues of
  /// their own, and everything here happens under one lock, so the events keep
  /// the order the bytes arrived in and the exit comes last.
  ///
  /// **A LINE THAT IS NOT VALID UTF-8 STILL ARRIVES**, its bad bytes shown as
  /// U+FFFD: dropping it would lose a log line with nothing to say so.
  private final class StreamState: Sendable {
    private struct Progress {
      var pending = Data()
      var outputClosed = false
      var status: Int32?
      var finished = false
    }

    private let progress = OSAllocatedUnfairLock(initialState: Progress())
    private let continuation: AsyncStream<StreamEvent>.Continuation

    init(_ continuation: AsyncStream<StreamEvent>.Continuation) {
      self.continuation = continuation
    }

    func received(_ chunk: Data) {
      progress.withLock { progress in
        progress.pending.append(chunk)
        while let newline = progress.pending.firstIndex(of: 0x0A) {
          let line = progress.pending.subdata(in: progress.pending.startIndex..<newline)
          progress.pending.removeSubrange(progress.pending.startIndex...newline)
          continuation.yield(.line(String(decoding: line, as: UTF8.self)))
        }
      }
    }

    /// The end of the output: a trailing fragment with no newline is a line too.
    func outputClosed() {
      progress.withLock { progress in
        if !progress.pending.isEmpty {
          continuation.yield(.line(String(decoding: progress.pending, as: UTF8.self)))
          progress.pending.removeAll()
        }
        progress.outputClosed = true
        finishIfDone(&progress)
      }
    }

    func exited(_ status: Int32) {
      progress.withLock { progress in
        progress.status = status
        finishIfDone(&progress)
      }
    }

    /// **THE EVENTS END WHEN BOTH ENDS HAVE BEEN SEEN.** Output can outlive the
    /// exit by a moment, and finishing at the exit alone would lose what the
    /// child wrote last.
    private func finishIfDone(_ progress: inout Progress) {
      guard progress.outputClosed, let status = progress.status, !progress.finished else { return }
      progress.finished = true
      continuation.yield(.exited(status))
      continuation.finish()
    }
  }

  /// `stream` once the binary is found, and the seam its tests drive with
  /// `/bin/sh`, so they exercise the plumbing rather than a verb.
  static func launchStreaming(
    binary: String,
    args: [String],
    env: [String: String],
    cwd: URL?,
    holdStdin: Bool
  ) throws -> StreamedChild {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: binary)
    process.arguments = args
    process.environment = env
    process.currentDirectoryURL = cwd

    let output = Pipe()
    process.standardOutput = output
    process.standardError = output
    // A held stdin is how a following verb learns to end (see `stream`).
    let input = holdStdin ? Pipe() : nil
    if let input { process.standardInput = input }
    let held = input.map { HeldInput($0.fileHandleForWriting) }

    let (events, continuation) = AsyncStream.makeStream(of: StreamEvent.self)
    let state = StreamState(continuation)
    output.fileHandleForReading.readabilityHandler = { handle in
      let chunk = handle.availableData
      if chunk.isEmpty {
        handle.readabilityHandler = nil
        state.outputClosed()
      } else {
        state.received(chunk)
      }
    }
    process.terminationHandler = { process in
      held?.close()
      state.exited(process.terminationStatus)
    }

    do {
      try process.run()
    } catch {
      output.fileHandleForReading.readabilityHandler = nil
      process.terminationHandler = nil
      held?.close()
      throw IntentCLIError.failedToLaunch(underlying: error.localizedDescription)
    }
    return StreamedChild(events: events, child: RunningProcess(process, input: held))
  }
}

/// A handle the main actor may hold on a child that lives on another queue:
/// terminate() is all it offers, and Process is safe to signal from anywhere.
final class RunningProcess: @unchecked Sendable {
  private let process: Process
  private let input: HeldInput?

  fileprivate init(_ process: Process, input: HeldInput?) {
    self.process = process
    self.input = input
  }

  /// Closes the child's stdin when the app holds it, then signals the child. A
  /// following `intent daemon logs` ends on either; doing both makes the end
  /// independent of which the child notices first.
  func terminate() {
    input?.close()
    if process.isRunning { process.terminate() }
  }
}

/// The write end of a child's stdin, held open by the app (ST0075). Closing it
/// is how a following verb learns to end, and the app's death closes it however
/// the app dies. It is closed once, by whichever of `terminate()` and the
/// child's own exit comes first.
fileprivate final class HeldInput: Sendable {
  private static let logger = AppLog.logger("IntentCLI")
  private let handle: OSAllocatedUnfairLock<FileHandle?>

  init(_ handle: FileHandle) {
    self.handle = OSAllocatedUnfairLock(initialState: handle)
  }

  func close() {
    let open = handle.withLock { held in
      defer { held = nil }
      return held
    }
    guard let open else { return }
    do {
      try open.close()
    } catch {
      Self.logger.error("could not close a child's stdin: \(error.localizedDescription, privacy: .public)")
    }
  }
}
