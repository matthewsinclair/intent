import XCTest

@testable import Intent

/// The menu's identity row, rendered from `intent version`. **WHAT THESE PIN IS
/// THE SWIFT SIDE'S RENDERING OF THE SHAPES IT KNOWS**, written here as
/// literals copied from spine.rs's `"{version} ({SOURCE_COMMIT})"`. They cannot
/// see a change to that line on the Rust side: a shape this build does not
/// recognise is shown verbatim by design, so such a change surfaces as a long
/// row in the menu rather than as a red here.
final class VersionServiceTests: XCTestCase {
  func testACleanBuildShortensItsShaToTheFirstEight() {
    XCTAssertEqual(
      VersionService.menuTitle("intent 3.0.1 (8a48430ee9b8dceeb5ecebb83a678941bc44848a)\n"),
      "intent 3.0.1 (8a48430e)")
  }

  /// **THE DIRT MARKER SURVIVES THE SHORTENING.** A title that dropped it would
  /// report a dirty build as the clean commit it was built beside.
  func testADirtyBuildKeepsItsMarker() {
    XCTAssertEqual(
      VersionService.menuTitle("intent 3.0.2 (dirty-8a48430ee9b8dceeb5ecebb83a678941bc44848a)\n"),
      "intent 3.0.2 (dirty-8a48430e)")
  }

  func testASha256CommitShortensTheSameWay() {
    let sha256 = "8a48430ee9b8dceeb5ecebb83a678941bc44848a8a48430ee9b8dceeb5ecebb8"
    XCTAssertEqual(VersionService.menuTitle("intent 3.0.1 (\(sha256))"), "intent 3.0.1 (8a48430e)")
  }

  /// `unknown` is the CLI's own word for a commit git could not answer, and it
  /// is rendered as given.
  func testAnUnknownCommitIsRenderedAsGiven() {
    XCTAssertEqual(VersionService.menuTitle("intent 3.0.1 (unknown)\n"), "intent 3.0.1 (unknown)")
  }

  /// A shape this build does not recognise is shown verbatim, never parsed into
  /// something the CLI did not say.
  func testAnUnrecognisedShapeIsRenderedVerbatim() {
    XCTAssertEqual(VersionService.menuTitle("intent 4.0.0-rc1\n"), "intent 4.0.0-rc1")
  }

  func testEmptyOutputIsNil() {
    XCTAssertNil(VersionService.menuTitle(""))
    XCTAssertNil(VersionService.menuTitle("  \n"))
  }

  /// **THE FAILED STATE RENDERS DIFFERENTLY FROM THE PENDING ONE.** This pins
  /// the rendering only: that `refresh()` sets `failed` on a thrown or empty read
  /// is not reached here, because the CLI call is not injectable.
  func testFailedAndPendingStatesRenderDifferently() {
    XCTAssertEqual(VersionState.pending.menuTitle, "Intent")
    let failed = VersionState.failed("intent was not found on the login shell's PATH")
    XCTAssertEqual(failed.menuTitle, "Intent (version unavailable)")
  }

  func testAnAnsweredReadRendersTheCLIsTitle() {
    let answered = VersionState.answered("intent 3.0.1 (8a48430e)")
    XCTAssertEqual(answered.menuTitle, "intent 3.0.1 (8a48430e)")
  }
}
