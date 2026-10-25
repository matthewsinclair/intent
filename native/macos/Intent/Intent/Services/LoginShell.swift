import Foundation

/// What launchd does not give a GUI app: the user's PATH, and where `intent`
/// is. A `.app` bundle launches with a bare PATH and none of the developer's
/// environment, and a Homebrew-installed `intent` under `/opt/homebrew/bin` is
/// not on it. So: ask `/bin/zsh` once, as a login and interactive shell
/// (`-ilc`, so PATH additions in .zshrc count too; stderr goes nowhere, there
/// is no tty) and give every child what a terminal has. Nothing but a fallback
/// PATH is baked into the bundle. (AC-01.9)
struct LoginShell: Sendable {
  /// The login shell's PATH, or a sane default if the shell said nothing.
  let path: String
  /// `command -v intent` AS FOUND, never resolved (ST0087): the per-user link
  /// `~/.local/bin/intent` stays the link, so every spawn follows it to
  /// whichever tree `intent use` last pointed it at. Resolving it here pinned
  /// the app to the tree it saw at launch, and the menu's Restart then started
  /// that tree's daemon, undoing a switch.
  let intent: String?
  /// How `intent` was found, written to the launch log line.
  let source: String

  static let fallbackPath = "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"

  static func capture() -> LoginShell {
    let out = zsh("printf '%s\\n' \"$PATH\"; command -v intent || true")
    return parse(out)
  }

  /// First line is PATH, the next non-empty line (if any) is where the shell
  /// found intent. Separated from `capture` so it is testable without a shell.
  static func parse(
    _ out: String,
    home: String = NSHomeDirectory(),
    isExecutable: (String) -> Bool = executable
  ) -> LoginShell {
    let lines = out.split(separator: "\n", omittingEmptySubsequences: false).map(String.init)
    let path = lines.first.flatMap { $0.isEmpty ? nil : $0 } ?? fallbackPath
    let found = lines.dropFirst().first { !$0.isEmpty }

    if let found, isExecutable(found) {
      return LoginShell(path: path, intent: found, source: "login shell: \(found)")
    }
    let symlink = home + "/.local/bin/intent"
    if isExecutable(symlink) {
      return LoginShell(path: path, intent: symlink, source: "~/.local/bin/intent")
    }
    return LoginShell(path: path, intent: nil, source: "not on the login shell's PATH")
  }

  /// Whether `p` runs: an executable file, or a link to one. The link is
  /// followed to ANSWER this and is never what is kept.
  static func executable(_ p: String) -> Bool {
    FileManager.default.isExecutableFile(atPath: p)
  }

  private static func zsh(_ script: String) -> String {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: "/bin/zsh")
    process.arguments = ["-ilc", script]
    let out = Pipe()
    process.standardOutput = out
    process.standardError = FileHandle.nullDevice
    do {
      try process.run()
    } catch {
      return ""
    }
    let data = out.fileHandleForReading.readDataToEndOfFile()
    process.waitUntilExit()
    return String(data: data, encoding: .utf8) ?? ""
  }
}

/// One capture per launch, re-taken when the binary it found no longer runs:
/// `intent use brew` removes the per-user links, and Homebrew's own `intent`
/// then answers for a fresh login shell. `refresh()` re-captures on demand.
final class LoginShellStore: @unchecked Sendable {
  static let shared = LoginShellStore()

  private let lock = NSLock()
  private var cached: LoginShell?
  private let isExecutable: (String) -> Bool
  private let capture: () -> LoginShell

  init(
    isExecutable: @escaping (String) -> Bool = LoginShell.executable,
    capture: @escaping () -> LoginShell = LoginShell.capture
  ) {
    self.isExecutable = isExecutable
    self.capture = capture
  }

  func current() -> LoginShell {
    lock.lock()
    defer { lock.unlock() }
    if let cached, cached.intent.map(isExecutable) ?? true { return cached }
    let shell = capture()
    cached = shell
    return shell
  }

  @discardableResult
  func refresh() -> LoginShell {
    let shell = capture()
    lock.lock()
    cached = shell
    lock.unlock()
    return shell
  }
}
