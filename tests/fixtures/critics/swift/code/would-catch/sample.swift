// Fixture: triggers IN-SW-CODE-001 (guard-over-nested-if) and
// IN-SW-CODE-002 (optionals-over-sentinels) by design.

import Foundation

struct User {
  let id: Int
  let name: String
}

struct UserService {
  let users: [User]

  // Nested if-let chain with `else { return nil }` tail - textbook guard case.
  func activeEmailForRole(roleName: String?, userId: Int?) -> String? {
    if let role = roleName {
      if let id = userId {
        if let user = users.first(where: { $0.id == id }) {
          if role == "admin" {
            return "\(user.name)@example.com"
          } else {
            return nil
          }
        } else {
          return nil
        }
      } else {
        return nil
      }
    } else {
      return nil
    }
  }

  // Sentinel return value: returns -1 when the name is absent.
  // Callers must compare against -1 to detect "not found".
  func firstIndex(of name: String) -> Int {
    for (i, u) in users.enumerated() where u.name == name {
      return i
    }
    return -1
  }

  // Another sentinel - returns empty string for "not configured".
  func displayNameOrEmpty(for id: Int) -> String {
    for u in users where u.id == id {
      return u.name
    }
    return ""
  }
}
