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
  /// The login shell's `CLAUDE_CONFIG_DIR`, or nil when it sets none (issue
  /// 0671). Claude Code keeps its sessions there when it is set, and a daemon
  /// this app starts finds them only if it is handed the same value.
  let claudeConfigDir: String?

  static let fallbackPath = "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"

  /// The variable the probe reads besides PATH, and the tag on its line.
  static let claudeConfigDirVariable = "CLAUDE_CONFIG_DIR"
  static let claudeConfigDirTag = claudeConfigDirVariable + "="

  static func capture() -> LoginShell {
    let out = zsh(
      "printf '%s\\n' \"$PATH\"; command -v intent || true; "
        + "printf '\(claudeConfigDirTag)%s\\n' \"${\(claudeConfigDirVariable)-}\""
    )
    return parse(out)
  }

  /// First line is PATH, the next non-empty untagged line (if any) is where
  /// the shell found intent, and the line tagged `CLAUDE_CONFIG_DIR=` is that
  /// variable, empty when the shell sets none. Tagged rather than placed: when
  /// `command -v` finds nothing it prints no line, so a third line by position
  /// would be read as intent's. Separated from `capture` so it is testable
  /// without a shell.
  static func parse(
    _ out: String,
    home: String = NSHomeDirectory(),
    isExecutable: (String) -> Bool = executable
  ) -> LoginShell {
    let lines = out.split(separator: "\n", omittingEmptySubsequences: false).map(String.init)
    let path = lines.first.flatMap { $0.isEmpty ? nil : $0 } ?? fallbackPath
    let rest = lines.dropFirst()
    let found = rest.first { !$0.isEmpty && !$0.hasPrefix(claudeConfigDirTag) }
    let tagged = rest.first(where: { $0.hasPrefix(claudeConfigDirTag) })
    let value = tagged.map { String($0.dropFirst(claudeConfigDirTag.count)) }
    let dir = value.flatMap { $0.isEmpty ? nil : $0 }

    if let found, isExecutable(found) {
      return LoginShell(
        path: path, intent: found, source: "login shell: \(found)", claudeConfigDir: dir)
    }
    let symlink = home + "/.local/bin/intent"
    if isExecutable(symlink) {
      return LoginShell(
        path: path, intent: symlink, source: "~/.local/bin/intent", claudeConfigDir: dir)
    }
    return LoginShell(
      path: path, intent: nil, source: "not on the login shell's PATH", claudeConfigDir: dir)
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
