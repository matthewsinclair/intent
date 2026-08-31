import Foundation

/// What launchd does not give a GUI app: the user's PATH, and where `intent`
/// is. A `.app` bundle launches with a bare PATH and none of the developer's
/// environment, and a Homebrew-installed `intent` under `/opt/homebrew/bin` is
/// not on it. So: ask the login shell once (`-ilc`, so PATH additions in
/// .zshrc count too; stderr goes nowhere, there is no tty) and give every child
/// what a terminal has. Nothing is baked into the bundle. (AC-01.9)
struct LoginShell: Sendable {
  /// The login shell's PATH, or a sane default if the shell said nothing.
  let path: String
  /// `command -v intent`, realpath'd -- the ~/.local/bin symlink resolves to
  /// native/rust/target/release/intent.
  let intent: String?
  /// How `intent` was found, for Settings -> Estate.
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
    exists: (String) -> String? = realpath
  ) -> LoginShell {
    let lines = out.split(separator: "\n", omittingEmptySubsequences: false).map(String.init)
    let path = lines.first.flatMap { $0.isEmpty ? nil : $0 } ?? fallbackPath
    let found = lines.dropFirst().first { !$0.isEmpty }

    if let found, let real = exists(found) {
      return LoginShell(path: path, intent: real, source: "login shell: \(found)")
    }
    let symlink = home + "/.local/bin/intent"
    if let real = exists(symlink) {
      return LoginShell(path: path, intent: real, source: "~/.local/bin/intent")
    }
    return LoginShell(path: path, intent: nil, source: "not on the login shell's PATH")
  }

  static func realpath(_ p: String) -> String? {
    let url = URL(fileURLWithPath: p).resolvingSymlinksInPath()
    return FileManager.default.isExecutableFile(atPath: url.path) ? url.path : nil
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

/// One capture per launch, refreshed on demand (Settings -> Estate).
final class LoginShellStore: @unchecked Sendable {
  static let shared = LoginShellStore()

  private let lock = NSLock()
  private var cached: LoginShell?

  func current() -> LoginShell {
    lock.lock()
    defer { lock.unlock() }
    if let cached { return cached }
    let shell = LoginShell.capture()
    cached = shell
    return shell
  }

  @discardableResult
  func refresh() -> LoginShell {
    let shell = LoginShell.capture()
    lock.lock()
    cached = shell
    lock.unlock()
    return shell
  }
}
