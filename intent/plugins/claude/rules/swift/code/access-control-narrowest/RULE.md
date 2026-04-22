---
id: IN-SW-CODE-004
language: swift
category: code
severity: recommendation
title: Access control — narrowest first
summary: >
  Default to `private`. Widen to `fileprivate`, `internal`, `package`, or
  `public` only when a caller outside the current scope actually needs
  access. Never use `public` as a shortcut to make linking succeed.
principles:
  - public-interface
applies_when:
  - "Declaring any type, method, property, or initialiser"
  - "Exposing helpers across files or modules"
  - "Marking symbols public to satisfy a unit test or sample-app import"
applies_to:
  - "**/*.swift"
does_not_apply_when:
  - "Symbols that are genuinely part of a library's documented public API"
  - "`@testable import` usage where internal symbols are the test surface"
  - "Protocol requirements that must match the protocol's visibility"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-SW-CODE-001
aliases: []
tags:
  - swift
  - access-control
  - api-design
status: active
version: 1
---

# Access control — narrowest first

Every `public` is a contract you will have to keep. Every `private` is a rename you can do without asking.

## Problem

Swift has five access levels (`private`, `fileprivate`, `internal`, `package`, `public`) because each one buys a different scope of commitment. `private` means "this symbol can change freely". `public` means "breaking this symbol breaks every downstream consumer". Authors who default to `public` — usually because a test or sample app could not see the symbol otherwise — leak every internal helper into the API, where it has to be maintained as part of the contract.

The Swift default is `internal`, which is reasonable for an app target but too wide for a library. Library crates should reach for `public` deliberately; everything else stays `internal` or tighter. And `private` (same file) versus `fileprivate` (same file, different scope): the distinction is real, and sloppy `fileprivate` usage hides helpers that should live in their owning type.

## Detection

Static signals:

- `public` on types whose users are only the current module.
- `public` that was added "to make tests compile" rather than because a consumer needs it. `@testable import` is the correct tool for tests, not widened public surface.
- `internal` on standalone helpers inside a single file where `private` or `fileprivate` would do.
- Missing `private` on extension methods whose only callers are inside the same type.

SwiftLint rule: `private_over_fileprivate`, `no_extension_access_modifier`.

## Bad

```swift
public class UserStore {
  public var cache: [Int: User] = [:]

  public func load(id: Int) -> User? { cache[id] ?? fetch(id) }

  public func fetch(_ id: Int) -> User? {
    let data = try? Data(contentsOf: URL(string: "https://api/\(id)")!)
    return data.flatMap { try? JSONDecoder().decode(User.self, from: $0) }
  }

  public func resetCache() { cache.removeAll() }
}
```

`cache` is implementation detail. `fetch` is an internal helper. `resetCache` is for tests. All three are now part of the public contract.

## Good

```swift
public class UserStore {
  private var cache: [Int: User] = [:]

  public func load(id: Int) -> User? {
    if let cached = cache[id] { return cached }
    return fetch(id)
  }

  private func fetch(_ id: Int) -> User? {
    let data = try? Data(contentsOf: URL(string: "https://api/\(id)")!)
    return data.flatMap { try? JSONDecoder().decode(User.self, from: $0) }
  }
}

// In the test target:
@testable import MyModule

final class UserStoreTests: XCTestCase {
  func testCacheHit() {
    let store = UserStore()
    // ... exercises internal cache via public load() ...
  }
}
```

Public surface is `load(id:)`. Tests use `@testable` for any internal state they need.

## When This Applies

- Every new declaration: start at `private`, widen only when a caller outside the scope needs the symbol.
- Library / framework targets: reach for `public` deliberately; keep `internal` as the default.
- Types with many helpers: most helpers should be `private` or `fileprivate`, not `internal`.

## When This Does Not Apply

- Symbols that are documented API: those must be `public` regardless of whether callers exist today.
- Protocol requirements: the visibility is constrained by the protocol's visibility.
- Xcode / SwiftPM package targets exporting symbols for sibling modules: `package` is the right level there.

## Further Reading

- The Swift Programming Language, "Access Control" (<https://docs.swift.org/swift-book/documentation/the-swift-programming-language/accesscontrol/>)
- Swift Evolution SE-0386 "New Access Modifier: package" (<https://github.com/swiftlang/swift-evolution/blob/main/proposals/0386-package-access-modifier.md>)
- SwiftLint rules: `private_over_fileprivate`, `no_extension_access_modifier`
- Apple "Framework Programming Guide" on API surface management
