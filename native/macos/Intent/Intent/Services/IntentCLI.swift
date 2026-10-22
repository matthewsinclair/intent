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
}
