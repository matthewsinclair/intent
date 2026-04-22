---
id: IN-SW-TEST-001
language: swift
category: test
severity: warning
title: XCTAssertEqual with specific values
summary: >
  Assert on concrete field values, not shape. `XCTAssertNotNil(user)` passes
  for any non-nil user; `XCTAssertEqual(user.email, "alice@test")` proves
  the call did what the test claims.
principles:
  - honest-data
  - public-interface
applies_when:
  - "Any XCTest assertion on a return value or computed property"
  - "Tests that currently rely on `XCTAssertNotNil`, `XCTAssertTrue(!= nil)`, or type checks"
  - "Async tests awaiting a value and then asserting"
applies_to:
  - "**/*Tests.swift"
  - "**/Tests/**/*.swift"
does_not_apply_when:
  - "Tests where the shape itself is the contract (protocol conformance, instance identity)"
  - "Existence-only tests verifying a side effect fired — in which case assert the effect, not the return"
  - "Property-based tests that assert invariants rather than exact values"
references:
  - IN-EX-TEST-001
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-SW-CODE-002
aliases: []
tags:
  - swift
  - xctest
  - testing
  - assertions
status: active
version: 1
---

# XCTAssertEqual with specific values

A shape assertion passes for every value of the right type. Assert on the specific value the test claims the code produces.

## Problem

`XCTAssertNotNil(user)` tells the reader: "after this call, we got a `User?` that was not nil". It does not tell you the user has the expected id, name, email, or any other attribute. It passes whether the function returned the right user, the wrong user, or a cached stub. The test reads like a test but is actually an existence check; when the underlying logic regresses to "return the first user in the database", the test does not notice.

Strong assertions — `XCTAssertEqual(user.email, "alice@test")` — bind the test to the domain contract. They fail when the function returns the wrong user, not just when it returns nothing. Over time, a test suite of strong assertions becomes the documentation; a suite of shape assertions becomes decoration.

## Detection

Static signals:

- `XCTAssertNotNil(result)` at the end of a test (shape, not value).
- `XCTAssertTrue(result != nil)` — same issue phrased differently.
- Type-identity checks: `XCTAssertTrue(result is User)` when the caller already typed `result` as `User?`.
- `XCTAssertGreaterThan(count, 0)` where the test claims a specific count is produced.
- Tests that await an async call and then only assert the call did not throw.

## Bad

```swift
func testLoadsUser() async throws {
  let user = try await store.load(id: 1)
  XCTAssertNotNil(user)
}

func testListIsPopulated() {
  let items = repository.all()
  XCTAssertFalse(items.isEmpty)
}
```

Both pass for any non-empty result, including "returned the wrong user" and "returned yesterday's data".

## Good

```swift
func testLoadsUser() async throws {
  let user = try await store.load(id: 1)
  XCTAssertEqual(user.id, 1)
  XCTAssertEqual(user.name, "Alice")
  XCTAssertEqual(user.email, "alice@test.com")
}

func testListContainsExpectedIds() {
  let items = repository.all()
  XCTAssertEqual(items.map(\.id), [1, 2, 3])
}
```

The tests now encode the actual expected output. Regressions produce diffs instead of silent passes.

## When This Applies

- Any assertion on a value-returning call where the specific value is knowable ahead of time.
- Tests named like `test_creates_user_with_expected_fields` — if the name promises specific fields, assert them.
- Collection assertions: compare the whole value (or a projection) rather than count + arbitrary element.

## When This Does Not Apply

- Instance-identity checks: `XCTAssertTrue(delegate === expectedObject)`. Identity is the contract.
- Side-effect tests: `XCTAssertTrue(mock.sendWasCalled)`. Here the boolean is the real assertion.
- Property-based tests (`SwiftCheck`, similar frameworks) where the assertion is an invariant across generated inputs.
- Negative tests: `XCTAssertNil(store.load(id: 999))` is honest when "not found" is the expectation.

## Further Reading

- Apple XCTest documentation: "XCTAssertEqual" (<https://developer.apple.com/documentation/xctest/xctassertequal>)
- WWDC18 "Testing Tips & Tricks" (<https://developer.apple.com/videos/play/wwdc2018/417/>)
- IN-EX-TEST-001 — Elixir counterpart with the same discipline
- IN-AG-HIGHLANDER-001 — assertion shape is one of the concerns the Highlander rule polices
