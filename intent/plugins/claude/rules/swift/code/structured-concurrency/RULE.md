---
id: IN-SW-CODE-003
language: swift
category: code
severity: warning
title: Structured concurrency over GCD
summary: >
  Prefer `async`/`await`, `Task`, and `actor` to `DispatchQueue` callbacks
  and completion handlers. Structured concurrency carries cancellation,
  error propagation, and type information that GCD strips away.
principles:
  - pfic
  - no-silent-errors
applies_when:
  - "New asynchronous code on Swift 5.5+ / iOS 15+ / macOS 12+"
  - "Refactoring completion-handler APIs to produce readable call sites"
  - "Coordinating concurrent work that shares state (use `actor`)"
applies_to:
  - "**/*.swift"
does_not_apply_when:
  - "Maintaining codebases targeting pre-Swift-5.5 platforms"
  - "Interop with C / Objective-C APIs that only expose `dispatch_async` / completion handlers"
  - "Low-latency audio, graphics, or realtime paths where GCD's scheduling guarantees are load-bearing"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-SW-CODE-002
aliases: []
tags:
  - swift
  - concurrency
  - async-await
  - actor
status: active
version: 1
---

# Structured concurrency over GCD

`DispatchQueue.async { }` with a completion handler is the old shape. `async`/`await` is the new shape. Pick the new shape for every new line.

## Problem

GCD completion handlers strip type information, bury errors in untyped closures, and make cancellation the caller's problem. `func load(completion: @escaping (Result<Data, Error>?) -> Void)` forces every caller to remember to handle the `nil`, to check for cancellation manually, to avoid retain cycles in the closure, and to unwrap a `Result` buried in a callback.

Swift's structured concurrency (SE-0296, `async`/`await` + `Task` + `actor`) gives you typed `async throws` signatures, automatic cancellation propagation via `Task.checkCancellation()`, data-race-safe shared state via `actor`, and code that reads top-to-bottom instead of nested-callback-first. An `await` is visibly a suspension point; the compiler checks that every `async` call is handled.

## Detection

Static signals:

- Function signatures with `@escaping` closure parameters on iOS 15+ / macOS 12+ targets.
- `DispatchQueue.global().async { }` / `DispatchQueue.main.async { }` blocks containing business logic (not just UI updates).
- Shared mutable state protected by `DispatchQueue` "sync queues" — these are actors-in-disguise.
- `dispatch_semaphore` or `DispatchGroup` used to aggregate parallel work — `async let` and `TaskGroup` replace both.

## Bad

```swift
func loadUser(id: Int, completion: @escaping (Result<User, Error>?) -> Void) {
  DispatchQueue.global().async {
    do {
      let data = try Data(contentsOf: URL(string: "https://api/\(id)")!)
      let user = try JSONDecoder().decode(User.self, from: data)
      DispatchQueue.main.async { completion(.success(user)) }
    } catch {
      DispatchQueue.main.async { completion(.failure(error)) }
    }
  }
}

class UserCache {
  private var store: [Int: User] = [:]
  private let queue = DispatchQueue(label: "cache.sync")

  func get(_ id: Int) -> User? {
    queue.sync { store[id] }
  }
  func set(_ id: Int, _ user: User) {
    queue.async { self.store[id] = user }
  }
}
```

Untyped callback, manual main-queue hop, hand-rolled mutual exclusion.

## Good

```swift
func loadUser(id: Int) async throws -> User {
  let url = URL(string: "https://api/\(id)")!
  let (data, _) = try await URLSession.shared.data(from: url)
  return try JSONDecoder().decode(User.self, from: data)
}

actor UserCache {
  private var store: [Int: User] = [:]

  func get(_ id: Int) -> User? { store[id] }
  func set(_ id: Int, _ user: User) { store[id] = user }
}

// Fan-out with async let.
let (alice, bob) = try await (loadUser(id: 1), loadUser(id: 2))
```

Types tell the truth. `actor` guarantees data-race safety. `async let` parallelises without a `DispatchGroup`.

## When This Applies

- Any new async API on platforms supporting Swift 5.5+ concurrency.
- Any function that waits on network, disk, cross-process IPC, or time.
- Any shared mutable state in a concurrent program — `actor` replaces manual sync queues and locks.
- Cancellation-aware workflows: `Task` cancellation propagates automatically to child `async let` calls and `TaskGroup` children.

## When This Does Not Apply

- Deployment targets that cannot adopt Swift concurrency (pre-iOS 15, pre-macOS 12). Backport via `AsyncCompat` shims if supported; otherwise stay on GCD.
- Objective-C interop points that emit callbacks — wrap with `withCheckedContinuation` / `withCheckedThrowingContinuation` at the boundary, then work in `async` internally.
- Real-time audio / video dispatch where GCD's QoS classes and priority inheritance are specifically tuned for the workload.

## Further Reading

- The Swift Programming Language, "Concurrency" (<https://docs.swift.org/swift-book/documentation/the-swift-programming-language/concurrency/>)
- SE-0296 "async/await" (<https://github.com/swiftlang/swift-evolution/blob/main/proposals/0296-async-await.md>)
- SE-0306 "Actors" (<https://github.com/swiftlang/swift-evolution/blob/main/proposals/0306-actors.md>)
- WWDC21 "Meet async/await in Swift" (<https://developer.apple.com/videos/play/wwdc2021/10132/>)
- IN-SW-CODE-002 — optionals and async signatures compose cleanly (`func f() async throws -> T?`)
