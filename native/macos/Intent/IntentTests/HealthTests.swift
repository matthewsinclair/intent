import XCTest

@testable import Intent

/// cc's tripwire, mirrored on the Swift side. cc asserts the three discriminator
/// literals in the Rust renderer; a renamed `Health` variant would still
/// compile and serialise and SILENTLY stop decoding here, in my language, where
/// cc's tests cannot see it. These pin the decode so the rename trips on this
/// side too.
final class HealthTests: XCTestCase {
  func testLiveDecodesWithItsEndpoint() {
    // Key order is alphabetical on the wire (endpoint before state); Codable
    // does not care, and this proves it.
    let h = Health.decode(#"{"endpoint":"/tmp/intentd.sock","state":"live"}"#)
    XCTAssertEqual(h, .live(endpoint: "/tmp/intentd.sock", url: nil))
  }

  /// A live daemon that published an answering loopback address carries the
  /// browser face beside the socket. **THE TWO ARE DIFFERENT KINDS OF ADDRESS
  /// AND BOTH ARE REAL**: `endpoint` is where clients route, `url` is where a
  /// browser goes, and rendering one as the other is the bug this pins.
  func testLiveCarriesTheBrowserURLWhenThereIsOne() {
    let h = Health.decode(
      #"{"endpoint":"/tmp/intentd.sock","state":"live","url":"http://127.0.0.1:51737"}"#)
    XCTAssertEqual(h, .live(endpoint: "/tmp/intentd.sock", url: "http://127.0.0.1:51737"))
  }

  /// **`url` ABSENT IS `nil`, NOT AN EMPTY STRING.** The menu gates on this
  /// optional, so a decoder that substituted `""` would offer an item whose
  /// title is blank and whose action opens nothing -- present-and-meaningless,
  /// the exact shape the Rust side omits the key to avoid.
  func testLiveWithoutAURLIsNilAndNotEmpty() {
    guard case .live(_, let url) = Health.decode(#"{"endpoint":"/tmp/x.sock","state":"live"}"#)
    else {
      return XCTFail("a live daemon with no published web face is still live")
    }
    XCTAssertNil(url)
  }

  func testStaleDecodesWithItsPid() {
    let h = Health.decode(#"{"pid":12345,"state":"stale"}"#)
    XCTAssertEqual(h, .stale(pid: 12345))
  }

  func testAbsentDecodesFromStateAlone() {
    let h = Health.decode(#"{"state":"absent"}"#)
    XCTAssertEqual(h, .absent)
  }

  /// A state this build does not know is `unknown`, never one of the three -- so
  /// a renamed daemon-side variant surfaces rather than masquerading as the
  /// wrong state.
  func testAnUnknownStateIsNotSilentlyOneOfTheThree() {
    let h = Health.decode(#"{"state":"quiescent"}"#)
    XCTAssertEqual(h, .unknown("quiescent"))
  }

  func testGarbageIsUnknownNotACrash() {
    guard case .unknown = Health.decode("not json") else {
      return XCTFail("non-JSON should decode as unknown")
    }
  }

  /// **hv, 2026-09-08: the live line reads `active`, and the wording is pinned
  /// because it is now also the CLICK TARGET.** The status line opens the web
  /// face, so this string is the affordance's label rather than a caption --
  /// and `answering` was the CLI's word for a different question (does a round
  /// trip complete), which is why hv asked for the change.
  func testLiveSummaryReadsActiveAndNotAnswering() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(live.summary, "intentd is active")
    XCTAssertFalse(live.summary.contains("answering"))
  }

  /// **NO STATE BUT `live` CARRIES A URL, WHICH IS WHAT KEEPS THE OTHER THREE
  /// LINES INERT.** The menu binds its action on `case .live(_, let url)` with
  /// `let url`, so an absent url is structurally unclickable rather than
  /// remembered -- a menu item that looks clickable and does nothing is worse
  /// than the redundancy hv removed.
  func testOnlyLiveCanCarryAURLToOpen() {
    guard
      case .live(_, let live) = Health.decode(
        #"{"endpoint":"/tmp/x.sock","state":"live","url":"http://127.0.0.1:51737"}"#)
    else { return XCTFail("live decodes as live") }
    XCTAssertNotNil(live)

    for json in [#"{"pid":1,"state":"stale"}"#, #"{"state":"absent"}"#, #"{"state":"zzz"}"#] {
      guard case .live = Health.decode(json) else { continue }
      XCTFail("\(json) decoded as live, which would make its menu line clickable")
    }
  }

  /// The remedy travels with the state (AC-01.6): stale names its pid and points
  /// the operator at it, absent does not -- the difference the display gates on.
  func testStaleSummaryNamesThePidAndAbsentDoesNot() {
    XCTAssertTrue(Health.stale(pid: 42).summary.contains("42"))
    XCTAssertTrue(Health.stale(pid: 42).summary.contains("investigate"))
    XCTAssertFalse(Health.absent.summary.contains("investigate"))
  }
}
