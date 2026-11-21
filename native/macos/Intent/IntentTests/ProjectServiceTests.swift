import XCTest

@testable import Intent

/// Pins `ProjectService.decodeThreadIds` and its count, `decodeThreadCount`,
/// the GraphQL read behind AC-01.3 (and ST0095 AC-11.2's opaque ids). A
/// well-formed `{"data":{"threads":[…]}}` yields the count; a shape
/// the app does not recognise yields nil -- the menu shows nothing -- never a
/// wrong count. Mirrors Health.decode's unknown-tolerant contract so a
/// daemon-side schema drift surfaces here rather than as a fabricated number.
final class ProjectServiceTests: XCTestCase {
  func testCountsThreadsFromAWellFormedResponse() {
    let json = #"{"data":{"threads":[{"id":"ST0001"},{"id":"ST0002"},{"id":"ST0003"}]}}"#
    XCTAssertEqual(ProjectService.decodeThreadCount(json), 3)
  }

  // ST0095 AT-11.2: the arm below.
  /// ST0095 WP-11 (AC-11.2): a hashed thread id beside ordinals is held as the
  /// opaque string the daemon sent, in its order: not parsed, sorted or cut.
  func testAHashedThreadIdRoundTripsUnchangedBesideOrdinals() {
    // `ST` and the waist's worked example, intentsvcs::id::EXAMPLE_HEX,
    // copied because Swift cannot import the Rust const; the leak scanners
    // treat it as the example it is.
    let hashed = "ST3f9a2c1d4e5f6a7b"
    let ids = ["ST0001", hashed, "ST0003"]
    let threads = ids.map { #"{"id":""# + $0 + #""}"# }.joined(separator: ",")
    let json = #"{"data":{"threads":["# + threads + "]}}"
    XCTAssertEqual(ProjectService.decodeThreadIds(json), ids)
    XCTAssertEqual(ProjectService.decodeThreadCount(json), ids.count)
  }

  /// Zero threads is a real answer (an empty project), distinct from nil.
  func testAnEmptyThreadListIsZeroNotNil() {
    XCTAssertEqual(ProjectService.decodeThreadCount(#"{"data":{"threads":[]}}"#), 0)
  }

  func testWhitespaceWrappedResponseStillDecodes() {
    let json = "\n  {\"data\":{\"threads\":[{\"id\":\"ST0001\"}]}}\n"
    XCTAssertEqual(ProjectService.decodeThreadCount(json), 1)
  }

  /// A GraphQL `errors` envelope, or a `data` without `threads`, carries no
  /// count, so it is nil rather than a zero that would read as "no threads".
  func testAnUnrecognisedShapeIsNilNotZero() {
    XCTAssertNil(ProjectService.decodeThreadCount(#"{"errors":[{"message":"boom"}]}"#))
    XCTAssertNil(ProjectService.decodeThreadCount(#"{"data":{}}"#))
  }

  func testGarbageIsNilNotACrash() {
    XCTAssertNil(ProjectService.decodeThreadCount("not json"))
    XCTAssertNil(ProjectService.decodeThreadCount(""))
  }
}
