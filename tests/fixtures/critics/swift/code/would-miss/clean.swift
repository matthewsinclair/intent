// Fixture: clean Swift code. Early-exit via guard, optionals over
// sentinels, no defensive nesting.

import Foundation

struct User {
  let id: Int
  let name: String
}

struct UserService {
  let users: [User]

  func activeEmailForRole(roleName: String?, userId: Int?) -> String? {
    guard let role = roleName else { return nil }
    guard let id = userId else { return nil }
    guard let user = users.first(where: { $0.id == id }) else { return nil }
    guard role == "admin" else { return nil }
    return "\(user.name)@example.com"
  }

  func firstIndex(of name: String) -> Int? {
    return users.firstIndex(where: { $0.name == name })
  }

  func displayName(for id: Int) -> String? {
    return users.first(where: { $0.id == id })?.name
  }
}
