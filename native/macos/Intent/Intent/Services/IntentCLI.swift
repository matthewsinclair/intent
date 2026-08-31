import Foundation

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

  var errorDescription: String? {
    switch self {
    case .binaryNotFound:
      "intent was not found on the login shell's PATH. Settings -> Estate -> Locate…"
    case .failedToLaunch(let underlying):
      "Could not launch intent: \(underlying)"
    case .commandFailed(let command, let code, let stderr):
      "`\(command)` exited \(code): \(stderr)"
    }
  }
}

/// The one shell-out path to `intent`, with the login shell's PATH handed to
/// every child. Nothing else in the app spawns anything, so binary resolution,
/// environment and error mapping live here once. (AC-01.1 / AC-01.9)
enum IntentCLI {
  static let overrideKey = "IntentBinary"

  /// Settings -> Estate's override wins; otherwise what the login shell found.
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

  /// Runs and returns stdout; a non-zero exit is an error carrying stderr.
  static func run(_ args: [String]) async throws -> String {
    let result = try await capture(args)
    guard result.isSuccess else {
      throw IntentCLIError.commandFailed(
        command: "intent " + args.joined(separator: " "),
        exitCode: result.exitCode,
        stderr: result.stderr.trimmingCharacters(in: .whitespacesAndNewlines)
      )
    }
    return result.stdout
  }

  /// Runs and returns the full result whatever the exit code.
  static func capture(_ args: [String]) async throws -> CLIRunResult {
    guard let binary = binary() else { throw IntentCLIError.binaryNotFound }
    let env = environment()
    return try await Task.detached(priority: .userInitiated) {
      try runProcess(binary: binary, args: args, env: env)
    }.value
  }

  /// Streams stdout and stderr, merged and line by line, to `onLine` (called on
  /// an arbitrary queue -- hop to the main actor before touching UI), and still
  /// returns the full result on exit. `onStart` receives a handle so a caller
  /// can end the child early -- the console's tail, whose pipeline the verb owns
  /// and takes down on SIGTERM (the verb's job, not the app's).
  static func stream(
    _ args: [String],
    onStart: (@Sendable (RunningProcess) -> Void)? = nil,
    onLine: @Sendable @escaping (String) -> Void
  ) async throws -> CLIRunResult {
    guard let binary = binary() else { throw IntentCLIError.binaryNotFound }
    let env = environment()
    return try await Task.detached(priority: .userInitiated) {
      try runProcessStreaming(binary: binary, args: args, env: env, onStart: onStart, onLine: onLine)
    }.value
  }

  // MARK: - Processes

  private static func runProcess(binary: String, args: [String], env: [String: String]) throws
    -> CLIRunResult
  {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: binary)
    process.arguments = args
    process.environment = env

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
    let outData = stdout.fileHandleForReading.readDataToEndOfFile()
    let errData = stderr.fileHandleForReading.readDataToEndOfFile()
    process.waitUntilExit()

    return CLIRunResult(
      exitCode: process.terminationStatus,
      stdout: String(data: outData, encoding: .utf8) ?? "",
      stderr: String(data: errData, encoding: .utf8) ?? ""
    )
  }

  /// Line assembly for the streaming pipe. `readabilityHandler` fires on a
  /// private queue; the lock keeps the buffer honest.
  private final class StreamState: @unchecked Sendable {
    private let lock = NSLock()
    private var buffer = Data()
    private var full = Data()

    func append(_ chunk: Data) -> [String] {
      lock.lock()
      defer { lock.unlock() }
      full.append(chunk)
      buffer.append(chunk)
      var lines: [String] = []
      while let nl = buffer.firstIndex(of: 0x0A) {
        let lineData = buffer.subdata(in: buffer.startIndex..<nl)
        buffer.removeSubrange(buffer.startIndex...nl)
        if let line = String(data: lineData, encoding: .utf8) { lines.append(line) }
      }
      return lines
    }

    func flushTail() -> String? {
      lock.lock()
      defer { lock.unlock() }
      guard !buffer.isEmpty else { return nil }
      let s = String(data: buffer, encoding: .utf8)
      buffer.removeAll()
      return s
    }

    func all() -> String {
      lock.lock()
      defer { lock.unlock() }
      return String(data: full, encoding: .utf8) ?? ""
    }
  }

  private static func runProcessStreaming(
    binary: String,
    args: [String],
    env: [String: String],
    onStart: (@Sendable (RunningProcess) -> Void)?,
    onLine: @Sendable @escaping (String) -> Void
  ) throws -> CLIRunResult {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: binary)
    process.arguments = args
    process.environment = env

    let pipe = Pipe()
    process.standardOutput = pipe
    process.standardError = pipe

    let state = StreamState()
    pipe.fileHandleForReading.readabilityHandler = { handle in
      let chunk = handle.availableData
      guard !chunk.isEmpty else { return }
      for line in state.append(chunk) { onLine(line) }
    }

    do {
      try process.run()
    } catch {
      pipe.fileHandleForReading.readabilityHandler = nil
      throw IntentCLIError.failedToLaunch(underlying: error.localizedDescription)
    }
    onStart?(RunningProcess(process))
    process.waitUntilExit()
    pipe.fileHandleForReading.readabilityHandler = nil

    let trailing = pipe.fileHandleForReading.availableData
    if !trailing.isEmpty {
      for line in state.append(trailing) { onLine(line) }
    }
    if let tail = state.flushTail() { onLine(tail) }

    return CLIRunResult(exitCode: process.terminationStatus, stdout: state.all(), stderr: "")
  }
}

/// A handle the main actor may hold on a child that lives on another queue:
/// terminate() is all it offers, and Process is safe to signal from anywhere.
final class RunningProcess: @unchecked Sendable {
  private let process: Process

  init(_ process: Process) {
    self.process = process
  }

  func terminate() {
    if process.isRunning { process.terminate() }
  }
}
