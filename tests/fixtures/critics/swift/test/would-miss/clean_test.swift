// Fixture: clean Swift test. XCTAssertEqual pins specific values;
// each test asserts one outcome.

import XCTest
@testable import App

final class UserServiceTests: XCTestCase {
  func testFirstIndexReturnsExpectedPosition() {
    let service = UserService(users: [
      User(id: 1, name: "Alice"),
      User(id: 2, name: "Bob"),
    ])
    XCTAssertEqual(service.firstIndex(of: "Bob"), 1)
  }

  func testFirstIndexReturnsNilWhenAbsent() {
    let service = UserService(users: [User(id: 1, name: "Alice")])
    XCTAssertNil(service.firstIndex(of: "Charlie"))
  }

  func testActiveEmailForAdminReturnsAddress() {
    let service = UserService(users: [User(id: 1, name: "Alice")])
    XCTAssertEqual(
      service.activeEmailForRole(roleName: "admin", userId: 1),
      "Alice@example.com"
    )
  }

  func testActiveEmailForNonAdminReturnsNil() {
    let service = UserService(users: [User(id: 1, name: "Alice")])
    XCTAssertNil(service.activeEmailForRole(roleName: "guest", userId: 1))
  }
}
