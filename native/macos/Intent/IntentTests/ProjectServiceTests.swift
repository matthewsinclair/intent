import XCTest

@testable import Intent

/// Pins `ProjectService.decodeThreadCount`, the GraphQL count read behind
/// AC-01.3. A well-formed `{"data":{"threads":[…]}}` yields the count; a shape
/// the app does not recognise yields nil -- the menu shows nothing -- never a
/// wrong count. Mirrors Health.decode's unknown-tolerant contract so a
/// daemon-side schema drift surfaces here rather than as a fabricated number.
final class ProjectServiceTests: XCTestCase {
  func testCountsThreadsFromAWellFormedResponse() {
    let json = #"{"data":{"threads":[{"id":"ST0001"},{"id":"ST0002"},{"id":"ST0003"}]}}"#
    XCTAssertEqual(ProjectService.decodeThreadCount(json), 3)
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
