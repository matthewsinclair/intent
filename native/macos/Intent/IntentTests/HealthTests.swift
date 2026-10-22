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
  /// **hv, 2026-09-08: the live state reads `active`.** `answering` was the
  /// CLI's word for a different question (does a round trip complete), which is
  /// why hv asked for the change.
  ///
  /// **ISSUE 0607: THE STATE IS THE INTENTD ROW'S SECOND LINE, IN GTOOLS'
  /// SHAPE** (hv: "the same controls the same way"), where it was one line,
  /// place then state then details (hv, 2026-09-13). The row's title is the
  /// place, `intentd`, and the line carries the port only when the daemon
  /// published a url to read it from; absent, it never invents one.
  func testTheLineNamesThePortOnlyWhenTheDaemonPublishedOne() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(live.menuSubtitle(busy: nil, threadCount: nil), "active on :51737")
    XCTAssertEqual(
      Health.live(endpoint: "/tmp/x.sock", url: nil).menuSubtitle(busy: nil, threadCount: nil), "active")
  }

  /// The thread count is the live line's detail, the way Gtools' request count
  /// is of a running CMS: after a `·`, with its plural, and gone entirely when
  /// the project query has not answered -- never a zero standing in for "not
  /// read".
  func testTheThreadCountJoinsTheLiveLineWithItsPlural() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(live.menuSubtitle(busy: nil, threadCount: 73), "active on :51737 · 73 steel threads")
    XCTAssertEqual(live.menuSubtitle(busy: nil, threadCount: 1), "active on :51737 · 1 steel thread")
  }

  /// The icon's tooltip is the row's two lines as one, as Gtools' site summary
  /// is, so the two can never describe the daemon differently.
  func testTheTooltipIsTheRowsTwoLinesAsOne() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(
      live.menuSummary(busy: nil, threadCount: 73), "intentd: active on :51737 · 73 steel threads")
    XCTAssertEqual(Health.absent.menuSummary(busy: nil, threadCount: nil), "intentd: not running")
  }

  /// `AT-02.2` (ST0074 WP-02).
  ///
  /// **NO STATE BUT `live` CARRIES THE COUNT.** It is a separate poll, so beside
  /// a daemon the line says is gone it would be a figure from the state before.
  /// Each state is pinned whole, with a count to hand, so a count leaking back
  /// in fails here.
  func testOnlyTheLiveLineCarriesTheThreadCount() {
    XCTAssertEqual(Health.absent.menuSubtitle(busy: nil, threadCount: 73), "not running")
    XCTAssertEqual(
      Health.stale(pid: 42).menuSubtitle(busy: nil, threadCount: 73),
      "pid 42 holds the socket but is not answering, investigate it")
    XCTAssertEqual(
      Health.unknown("not yet polled").menuSubtitle(busy: nil, threadCount: 73),
      "status unknown (not yet polled)")
  }

  /// `AT-02.3` (ST0074 WP-02).
  ///
  /// **A LIFECYCLE VERB IN FLIGHT OWNS THE LINE, AND SHOWS NO PORT.** Polling
  /// pauses while the verb runs, so the only port to hand is the one from before
  /// it -- dead once a restart's stop returns, and the new daemon's is assigned
  /// by the kernel.
  func testALifecycleVerbInFlightOwnsTheLineWithoutAPort() {
    let live = Health.live(endpoint: "/tmp/x.sock", url: "http://127.0.0.1:51737")
    XCTAssertEqual(live.menuSubtitle(busy: "Restarting…", threadCount: 73), "restarting…")
  }

  /// **NO STATE BUT `live` CARRIES A URL, WHICH IS WHAT KEEPS OPEN INERT
  /// ELSEWHERE.** The intentd flyout's Open binds on `case .live(_, let url)`
  /// (`DaemonMenu`), so an absent url is structurally unclickable rather than
  /// remembered -- a menu item that looks clickable and does nothing is worse
  /// than none.
  func testOnlyLiveCanCarryAURLToOpen() {
    guard
      case .live(_, let live) = Health.decode(
        #"{"endpoint":"/tmp/x.sock","state":"live","url":"http://127.0.0.1:51737"}"#)
    else { return XCTFail("live decodes as live") }
    XCTAssertNotNil(live)

    for json in [#"{"pid":1,"state":"stale"}"#, #"{"state":"absent"}"#, #"{"state":"zzz"}"#] {
      guard case .live = Health.decode(json) else { continue }
      XCTFail("\(json) decoded as live, which would make its flyout's Open clickable")
    }
  }

  /// The remedy travels with the state (AC-01.6): stale names its pid and points
  /// the operator at it, absent does not -- the difference the display gates on.
  func testStaleLineNamesThePidAndAbsentDoesNot() {
    XCTAssertEqual(
      Health.stale(pid: 42).menuSubtitle(busy: nil, threadCount: nil),
      "pid 42 holds the socket but is not answering, investigate it")
    XCTAssertEqual(Health.absent.menuSubtitle(busy: nil, threadCount: nil), "not running")
  }
}
