---
id: IN-SW-CODE-002
language: swift
category: code
severity: warning
title: Optionals over sentinel values
summary: >
  Use `T?` to represent "maybe absent". Never use `-1`, `NSNotFound`, an
  empty string, or a magic default to mean "no value". Sentinels collide
  with real values and bypass the compiler's absence checks.
principles:
  - honest-data
  - no-silent-errors
applies_when:
  - "Function signatures returning `Int`, `String`, or a custom type where `-1` / empty / `nil`-default could mean 'not found'"
  - "Protocol methods whose documentation says 'returns X if absent'"
  - "Bridged Objective-C APIs surfacing `NSNotFound`, `NSNull`, sentinel enums"
applies_to:
  - "**/*.swift"
does_not_apply_when:
  - "APIs constrained by protocol conformance that already returns sentinels (wrap at the boundary, don't export)"
  - "Bit-packed numeric fields where a designated value is the documented absence indicator and `Int?` would cost too much"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-SW-CODE-001
aliases: []
tags:
  - swift
  - optionals
  - type-safety
status: active
version: 1
---

# Optionals over sentinel values

Swift has a first-class "maybe absent" type. Using an in-band value to mean "not there" throws that away.

## Problem

Swift's optional is the canonical way to express "this may have no value". The compiler forces callers to decide how to handle absence — unwrap, default, propagate. When APIs return `-1` for "not found" or empty string for "no value", the compiler cannot help: every call site must remember the convention, and any forgotten check silently produces wrong output downstream. Worse, the sentinel occupies a value the type _could legitimately hold_ — an array index of `-1` is nonsense, but a temperature of `-1` is a winter day in Canberra.

Sentinels also couple the type to the absence convention. If you change from `-1` to `Int.min` later, every caller must be audited. With `Int?`, the absence semantics are in the type system.

## Detection

Static signals:

- Function signatures returning `Int`, `UInt`, or any non-optional primitive whose documentation contains "returns -1 if...", "returns NSNotFound when...", "returns 0 for absent", "returns empty string if missing".
- Call sites that compare against sentinel constants: `if result != -1 { ... }`, `if !name.isEmpty { ... }` where "empty" means "absent".
- Objective-C bridges: `firstIndex(of:)` returns `NSNotFound` in Obj-C; Swift's overlay returns `Int?`.

## Bad

```swift
struct UserStore {
  private var users: [User] = []

  func index(of user: User) -> Int {
    for (i, u) in users.enumerated() where u.id == user.id { return i }
    return -1
  }

  func displayName(for id: Int) -> String {
    users.first(where: { $0.id == id })?.name ?? ""
  }
}

let idx = store.index(of: user)
if idx != -1 { use(store[idx]) }

let name = store.displayName(for: id)
if !name.isEmpty { greet(name) }
```

`-1` and `""` both occupy the domain of legitimate values ("user at index 0 with empty display name"). Every caller must remember the convention.

## Good

```swift
struct UserStore {
  private var users: [User] = []

  func index(of user: User) -> Int? {
    users.firstIndex(where: { $0.id == user.id })
  }

  func displayName(for id: Int) -> String? {
    users.first(where: { $0.id == id })?.name
  }
}

if let idx = store.index(of: user) { use(store[idx]) }
if let name = store.displayName(for: id) { greet(name) }
```

Absence is expressed in the type. `if let` is the only way to reach inside, so forgetting is a compile error.

## When This Applies

- New Swift APIs designed from scratch: make "absent" be `nil`, always.
- Bridging Objective-C sentinel APIs: wrap at the boundary and export `Optional`-returning Swift signatures.
- Collections: prefer `firstIndex(where:) -> Int?` over hand-rolled loops returning `-1`.

## When This Does Not Apply

- Conforming to a protocol whose contract already specifies a sentinel return — you cannot change the signature, but you can add an `Optional`-returning convenience overload.
- Performance-critical numeric code where an `Optional<Int>` doubles storage and the documented sentinel is cheap to check. Rare; document with a comment explaining the constraint.

## Further Reading

- The Swift Programming Language, "Optionals" (<https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics/#Optionals>)
- Swift API Design Guidelines, "Use type information to establish expected behaviour"
- Mike Ash, "Friday Q&A — Why Optional?" (<https://www.mikeash.com/pyblog/friday-qa-2015-04-17-lets-build-swifts-optional-type.html>)
- IN-AG-NO-SILENT-001 — sentinels are silent-error machines
