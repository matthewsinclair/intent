import AppKit
import Testing

@testable import Intent

/// Intent.app hosts its own tests, so xcodebuild launches the app before any
/// test runs. Without a guard, the host runs the launch path: a second menu bar
/// item, and the polls, each of which runs `intent` inside the test run. The
/// delegate's launch begins with arca_swift's ArcaLaunch (issue I5169c5b),
/// which holds the launch path back when XCTest hosts the process. How it
/// tells a host is arca_swift's, and its own tests pin it.
@MainActor
@Suite("Test host")
struct TestHostTests {
  @Test("the host's delegate ran none of the launch path: no menu bar item and no poll")
  func skipped() throws {
    let delegate = try #require(NSApplication.shared.delegate as? AppDelegate)
    #expect(!delegate.hasStatusItem)
    #expect(!delegate.isPolling)
  }
}
