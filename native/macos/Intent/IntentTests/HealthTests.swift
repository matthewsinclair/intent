import XCTest

@testable import Intent

/// cc's tripwire, mirrored on the Swift side. cc asserts the discriminator
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

  /// A state this build does not know is `unknown`, never live, stale or absent -- so
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

  /// `AT-02.1` (ST0074 WP-02), with the thread-count test below it.
  ///
  /// **hv, 2026-09-08: the live line reads `active`, and the wording is pinned
  /// because it is also the CLICK TARGET.** The status line opens the web face,
  /// so this string is the affordance's label rather than a caption -- and
  /// `answering` was the CLI's word for a different question (does a round trip
  /// complete), which is why hv asked for the change.
  ///
  /// **hv, 2026-09-13: ONE LINE IN GTOOLS' SHAPE**, place then state then
  /// details. The place carries the port only when the daemon published a url
  /// to read it from; absent, the line never invents one.
  func testTheLineNamesThePortOnlyWhenTheDaemonPublishedOne() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(live.menuLine(busy: nil, threadCount: nil), "intentd :51737 — active")
    XCTAssertEqual(
      Health.live(endpoint: "/tmp/x.sock", url: nil).menuLine(busy: nil, threadCount: nil),
      "intentd — active")
  }

  /// The thread count is the live line's detail, the way Gtools' request count
  /// is of a running CMS: after a `·`, with its plural, and gone entirely when
  /// the project query has not answered -- never a zero standing in for "not
  /// read".
  func testTheThreadCountJoinsTheLiveLineWithItsPlural() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(
      live.menuLine(busy: nil, threadCount: 73), "intentd :51737 — active · 73 steel threads")
    XCTAssertEqual(
      live.menuLine(busy: nil, threadCount: 1), "intentd :51737 — active · 1 steel thread")
  }

  /// `AT-02.2` (ST0074 WP-02).
  ///
  /// **NO STATE BUT `live` CARRIES THE COUNT.** It is a separate poll, so beside
  /// a daemon the line says is gone it would be a figure from the state before.
  /// Each state is pinned whole, with a count to hand, so a count leaking back
  /// in fails here.
  func testOnlyTheLiveLineCarriesTheThreadCount() {
    XCTAssertEqual(Health.absent.menuLine(busy: nil, threadCount: 73), "intentd — not running")
    XCTAssertEqual(
      Health.stale(pid: 42).menuLine(busy: nil, threadCount: 73),
      "intentd — pid 42 holds the socket but is not answering, investigate it")
    XCTAssertEqual(
      Health.unknown("not yet polled").menuLine(busy: nil, threadCount: 73),
      "intentd — status unknown (not yet polled)")
  }

  /// `AT-02.3` (ST0074 WP-02).
  ///
  /// **A LIFECYCLE VERB IN FLIGHT OWNS THE LINE, AND SHOWS NO PORT.** Polling
  /// pauses while the verb runs, so the only port to hand is the one from before
  /// it -- dead once a restart's stop returns, and the new daemon's is assigned
  /// by the kernel.
  func testALifecycleVerbInFlightOwnsTheLineWithoutAPort() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(live.menuLine(busy: "Restarting…", threadCount: 73), "intentd — restarting…")
  }

  /// **NO STATE BUT `live` CARRIES A URL, WHICH IS WHAT KEEPS THE OTHER LINES
  /// INERT.** The menu binds its action on `case .live(_, let url)` with
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
  func testStaleLineNamesThePidAndAbsentDoesNot() {
    XCTAssertEqual(
      Health.stale(pid: 42).menuLine(busy: nil, threadCount: nil),
      "intentd — pid 42 holds the socket but is not answering, investigate it")
    XCTAssertEqual(Health.absent.menuLine(busy: nil, threadCount: nil), "intentd — not running")
  }
}
