---
id: IN-SW-CODE-001
language: swift
category: code
severity: warning
title: guard over nested if
summary: >
  Early exit with `guard` flattens control flow. Nested `if let` / `else`
  pyramids hide the happy path under defensive layers. Write the
  preconditions up front and let the body read straight.
principles:
  - pfic
applies_when:
  - "A function that unwraps one or more optionals before doing its work"
  - "Any precondition check whose failure returns, throws, or aborts"
  - "Nested `if let` chains that push the core logic three or more levels deep"
applies_to:
  - "**/*.swift"
does_not_apply_when:
  - "Control flow that genuinely needs the negative branch to run (not a guard)"
  - "Single-condition `if let` where the body is one line"
references:
  - IN-AG-PFIC-001
related_rules:
  - IN-SW-CODE-002
aliases: []
tags:
  - swift
  - control-flow
  - guard
status: active
version: 1
---

# guard over nested if

Exits go at the top. The body of the function is for the happy path.

## Problem

Swift inherited the `if let` pattern from optional unwrapping. Chained, it drifts into pyramid code — three levels of `if let ... { if let ... { if let ... { do the work } } }` — and the actual work hides inside the rightmost brace. The function's intent is no longer scannable from the signature and top-of-body; the reader has to trace nesting to find the real logic.

`guard` inverts the shape. Every precondition appears at the top, each with its own explicit exit path (`return`, `throw`, `fatalError`, `continue`). The function body, once past the guards, contains only the happy path. `guard` is exactly the "early return" discipline that imperative codebases learn the hard way; Swift builds it into the syntax.

## Detection

Static signals:

- `if let` chains 2+ deep where each binding is "fail the whole function if absent".
- Functions whose body is more than 50% defensive nesting.
- `else { return nil }` or `else { throw ... }` at the tail of an `if let` block — this is literally what `guard` was designed for.
- SwiftLint rule `early_exit` enforces the guard preference.

## Bad

```swift
func process(payload: Data?, user: User?, context: Context?) -> Result? {
  if let payload = payload {
    if let user = user {
      if let context = context, context.isValid {
        let enriched = enrich(payload, user: user)
        return Result(data: enriched, owner: user.id, ctx: context)
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
```

The actual work — `enrich` and `Result(...)` — lives four levels deep under defensive scaffolding.

## Good

```swift
func process(payload: Data?, user: User?, context: Context?) -> Result? {
  guard let payload = payload else { return nil }
  guard let user = user else { return nil }
  guard let context = context, context.isValid else { return nil }

  let enriched = enrich(payload, user: user)
  return Result(data: enriched, owner: user.id, ctx: context)
}
```

Preconditions up front; the work reads linearly at the same indentation level as the function signature.

## When This Applies

- Function bodies that start with one or more optional unwraps.
- Methods that validate input before acting.
- Loop bodies: `guard let` inside `for` is the idiomatic "skip this iteration" pattern (with `continue`).
- Any codepath where the negative branch is "give up and leave".

## When This Does Not Apply

- `if let` where both branches do substantive work: `guard` assumes the failure branch is a clean exit, not a second execution path.
- Single-binding one-liners: `if let v = opt { use(v) }` is not worth the ceremony of `guard`.
- Nested pattern matching where a `switch` is clearer than either nested `if let` or chained `guard`.

## Further Reading

- The Swift Programming Language, "Control Flow — Early Exit" (<https://docs.swift.org/swift-book/documentation/the-swift-programming-language/controlflow/#Early-Exit>)
- Swift API Design Guidelines, "Clarity at the point of use" (<https://www.swift.org/documentation/api-design-guidelines/>)
- SwiftLint rule: `early_exit`
- IN-SW-CODE-002 — optionals over sentinels is the other half of this story
