---
id: IN-SW-CODE-005
language: swift
category: code
severity: warning
title: Codable over manual JSON parsing
summary: >
  Use `Codable` with `JSONEncoder` / `JSONDecoder` for every JSON boundary.
  Hand-rolled `JSONSerialization` + `as? [String: Any]` dictionary casting
  is type-unsafe, verbose, and loses error-location information.
principles:
  - honest-data
  - no-silent-errors
applies_when:
  - "Parsing or emitting JSON at an API boundary (HTTP, persistence, IPC)"
  - "Converting a model to / from `Data` for storage"
  - "Interop with services returning JSON payloads"
applies_to:
  - "**/*.swift"
does_not_apply_when:
  - "Genuinely dynamic payloads where the schema is not known ahead of time"
  - "Partial updates against a known schema that would force an explosion of optional fields"
  - "Performance-critical paths where `JSONDecoder` overhead is measured and problematic"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-SW-CODE-002
aliases: []
tags:
  - swift
  - codable
  - json
  - type-safety
status: active
version: 1
---

# Codable over manual JSON parsing

Describe the shape in the type system. Let the compiler and the standard library do the tedious part.

## Problem

`JSONSerialization` gives you `Any`. Every access after that is a type cast: `(json as? [String: Any])?["user"] as? [String: Any])?["id"] as? Int`. Each `as?` is a potential silent failure — a wrong key, a server response with the number as a string, a missing field — and the compiler cannot warn about any of it. Errors surface as `nil` at the call site, far from the actual problem. When the decoder can only say "somewhere in that tree, something was wrong", debugging is painful.

`Codable` (`Encodable` + `Decodable`) gives the compiler a schema to work with. A `struct User: Codable { let id: Int; let name: String }` is self-describing: the decoder walks the JSON, matches keys, enforces types, and emits a precise error (`"keyNotFound: 'id' at .user"`) when the input lies. `CodingKeys` handles snake-case or renamed fields. `init(from:)` handles migrations.

## Detection

Static signals:

- `JSONSerialization.jsonObject(with:)` followed by a chain of `as?` casts.
- Dictionary literals passed to `JSONSerialization.data(withJSONObject:)` to build request bodies.
- Models that are `class` or `struct` with no `Codable` conformance despite flowing through JSON at some boundary.
- Helpers named `dictionary(for:)` / `from(_:Dictionary)` on model types — they exist because `Codable` was skipped.

## Bad

```swift
func parse(data: Data) -> User? {
  guard let root = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
        let id = root["id"] as? Int,
        let name = root["name"] as? String,
        let emailDict = root["contact"] as? [String: Any],
        let email = emailDict["email"] as? String else {
    return nil
  }
  return User(id: id, name: name, email: email)
}
```

Six potential silent failures. The `return nil` reports none of them. A server change (number arrives as a string) fails at runtime with no diagnostic.

## Good

```swift
struct User: Codable {
  let id: Int
  let name: String
  let contact: Contact

  struct Contact: Codable {
    let email: String
  }
}

func parse(data: Data) throws -> User {
  try JSONDecoder().decode(User.self, from: data)
}
```

`throws` surfaces precise errors (`DecodingError.keyNotFound`, `.typeMismatch`, `.dataCorrupted`) with the path to the failing field. Adding a field is one line in the struct.

## When This Applies

- Every JSON-shaped payload crossing a boundary: HTTP request/response bodies, cached persistence, inter-process messages.
- Configuration file parsing where the schema is known (`Decodable` from `.json` / `.plist`).
- Migration code that reads older schema versions: `init(from:)` with a custom `CodingKeys` and version-check.

## When This Does Not Apply

- Truly dynamic data: user-generated JSON in a general-purpose JSON editor; payloads whose schema comes from an upstream caller at runtime.
- Partial-update endpoints where fields are independently optional for reasons of API semantics; consider `@propertyWrapper` patterns like `@AnyCodable` before abandoning `Codable`.
- Extremely hot paths where custom hand-rolled parsing is measurably faster (profile first — `JSONDecoder` is usually fast enough).

## Further Reading

- The Swift Programming Language, "Encoding and Decoding Custom Types" (<https://docs.swift.org/swift-book/documentation/the-swift-programming-language/>)
- Apple Developer: "Encoding and Decoding Custom Types" (<https://developer.apple.com/documentation/foundation/archives_and_serialization/encoding_and_decoding_custom_types>)
- SE-0166 "Swift Archival & Serialization" (<https://github.com/swiftlang/swift-evolution/blob/main/proposals/0166-swift-archival-serialization.md>)
- Hacking with Swift, "How to parse JSON using Codable" (<https://www.hackingwithswift.com/articles/119/codable-cheat-sheet>)
- IN-AG-NO-SILENT-001 — dictionary casting silently maps every misconfiguration to `nil`
