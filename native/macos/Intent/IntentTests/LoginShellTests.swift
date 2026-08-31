import XCTest

@testable import Intent

/// AC-01.9: the login-shell PATH parse, tested WITHOUT a shell. `parse` is pure
/// over the captured text and an injected existence check, so the fact a `.app`
/// has no PATH is exercised deterministically -- never by running the app from a
/// terminal that already carries the developer's PATH, which is the instrument
/// that cannot fail on the one axis this row is about.
final class LoginShellTests: XCTestCase {
  func testFirstLineIsPathAndSecondLocatesIntent() {
    let out = "/opt/homebrew/bin:/usr/bin\n/opt/homebrew/bin/intent\n"
    let shell = LoginShell.parse(out, home: "/Users/test") { $0 }  // every path "exists"
    XCTAssertEqual(shell.path, "/opt/homebrew/bin:/usr/bin")
    XCTAssertEqual(shell.intent, "/opt/homebrew/bin/intent")
    XCTAssertEqual(shell.source, "login shell: /opt/homebrew/bin/intent")
  }

  func testFallsBackToLocalBinSymlinkWhenShellDidNotFindIt() {
    let out = "/usr/bin\n"  // PATH only, no `command -v intent` line
    let shell = LoginShell.parse(out, home: "/Users/test") { p in
      p == "/Users/test/.local/bin/intent" ? p : nil
    }
    XCTAssertEqual(shell.intent, "/Users/test/.local/bin/intent")
    XCTAssertEqual(shell.source, "~/.local/bin/intent")
  }

  func testEmptyOutputYieldsFallbackPathAndNoBinary() {
    let shell = LoginShell.parse("", home: "/Users/test") { _ in nil }
    XCTAssertEqual(shell.path, LoginShell.fallbackPath)
    XCTAssertNil(shell.intent)
    XCTAssertEqual(shell.source, "not on the login shell's PATH")
  }
}
