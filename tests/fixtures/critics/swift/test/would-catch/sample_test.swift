// Fixture: triggers IN-SW-TEST-001 (xctassertequal-specific-values)
// by asserting on shape-only properties rather than concrete values.

import XCTest
@testable import App

final class UserServiceTests: XCTestCase {
  func testFirstIndexReturnsSomething() {
    let service = UserService(users: [User(id: 1, name: "Alice")])
    let result = service.firstIndex(of: "Alice")
    XCTAssertNotNil(result)
  }

  func testActiveEmailIsNonNilForAdmin() {
    let service = UserService(users: [User(id: 1, name: "Alice")])
    let result = service.activeEmailForRole(roleName: "admin", userId: 1)
    XCTAssertTrue(result != nil)
  }

  func testTypeIdentityOnly() {
    let service = UserService(users: [User(id: 1, name: "Alice")])
    let result = service.activeEmailForRole(roleName: "admin", userId: 1)
    XCTAssertTrue(result is String?)
  }
}
