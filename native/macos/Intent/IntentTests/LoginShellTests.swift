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
    let shell = LoginShell.parse(out, home: "/Users/test") { _ in true }  // every path runs
    XCTAssertEqual(shell.path, "/opt/homebrew/bin:/usr/bin")
    XCTAssertEqual(shell.intent, "/opt/homebrew/bin/intent")
    XCTAssertEqual(shell.source, "login shell: /opt/homebrew/bin/intent")
  }

  func testFallsBackToLocalBinSymlinkWhenShellDidNotFindIt() {
    let out = "/usr/bin\n"  // PATH only, no `command -v intent` line
    let shell = LoginShell.parse(out, home: "/Users/test") { p in
      p == "/Users/test/.local/bin/intent"
    }
    XCTAssertEqual(shell.intent, "/Users/test/.local/bin/intent")
    XCTAssertEqual(shell.source, "~/.local/bin/intent")
  }

  func testEmptyOutputYieldsFallbackPathAndNoBinary() {
    let shell = LoginShell.parse("", home: "/Users/test") { _ in false }
    XCTAssertEqual(shell.path, LoginShell.fallbackPath)
    XCTAssertNil(shell.intent)
    XCTAssertEqual(shell.source, "not on the login shell's PATH")
  }

  /// ST0087: the hit is kept AS FOUND. A real link to a real executable, read
  /// through the default check, stays the link, so a spawn follows whatever
  /// `intent use` points it at; resolving it pinned the launch-time tree.
  func testALinkHitIsKeptAsTheLinkAndNeverResolved() throws {
    let dir = FileManager.default.temporaryDirectory
      .appendingPathComponent("loginshell-\(UUID().uuidString)")
    try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
    defer { try? FileManager.default.removeItem(at: dir) }
    let target = dir.appendingPathComponent("intent-dev")
    try Data("#!/bin/sh\n".utf8).write(to: target)
    try FileManager.default.setAttributes([.posixPermissions: 0o755], ofItemAtPath: target.path)
    let link = dir.appendingPathComponent("intent")
    try FileManager.default.createSymbolicLink(at: link, withDestinationURL: target)

    let shell = LoginShell.parse("/usr/bin\n\(link.path)\n", home: "/Users/test")
    XCTAssertEqual(shell.intent, link.path)
  }

  /// ST0087: `intent use brew` removes the per-user links, so a cached binary
  /// that no longer runs is re-captured rather than spawned and failed.
  func testTheStoreRecapturesWhenTheCachedBinaryNoLongerRuns() {
    var live: Set<String> = ["/Users/test/.local/bin/intent"]
    var captures = 0
    let store = LoginShellStore(
      isExecutable: { live.contains($0) },
      capture: {
        captures += 1
        let hit = live.first ?? ""
        return LoginShell.parse("/usr/bin\n\(hit)\n", home: "/Users/test") { live.contains($0) }
      }
    )
    XCTAssertEqual(store.current().intent, "/Users/test/.local/bin/intent")
    XCTAssertEqual(store.current().intent, "/Users/test/.local/bin/intent")
    XCTAssertEqual(captures, 1, "a binary that still runs is not re-captured")

    live = ["/opt/homebrew/bin/intent"]
    XCTAssertEqual(store.current().intent, "/opt/homebrew/bin/intent")
    XCTAssertEqual(captures, 2)
  }
}
