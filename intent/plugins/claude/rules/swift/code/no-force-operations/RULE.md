---
id: IN-SW-CODE-006
language: swift
category: code
severity: warning
title: No force operations
summary: >
  `try!`, `as!` and a postfix force unwrap `!` turn a recoverable failure into
  a crash with no handler. They are Swift's panics. Propagate with `try`,
  narrow with `as?` and unwrap with `guard let`, `if let` or `??` so the
  failure reaches code that can answer it.
principles:
  - no-silent-errors
  - honest-data
applies_when:
  - "Calling a throwing function"
  - "Casting a value to a narrower type"
  - "Reading the value out of an optional"
  - "Declaring a stored property or variable of an implicitly unwrapped optional type"
applies_to:
  - "**/*.swift"
does_not_apply_when:
  - "`fatalError(` as the exit of a `guard`'s `else`, which IN-SW-CODE-001 names as a legitimate exit"
  - "`@IBOutlet` properties, which the framework declares implicitly unwrapped and fills before use"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-RS-CODE-001
  - IN-SW-CODE-001
  - IN-SW-CODE-002
aliases: []
tags:
  - swift
  - error-handling
  - optionals
  - no-silent-errors
status: active
version: 1
---

# No force operations

A force operation is the code deciding that a failure cannot happen, and the process dying on the day it does.

## Problem

`try!` calls a throwing function and crashes if it throws. `as!` casts and crashes if the value is not of that type. A postfix `!` on an optional crashes if it is `nil`. Each is a panic written as one character, and each discards the error the language was about to hand to a caller: the throw's error value, the cast's failure, the absence.

The crash is also a lie about the signature. `let config = try! load()` reads as though `load` cannot fail, and a reader has to know the function to learn that it can. `let config = try load()` in a throwing function says the failure exists and where it goes. This is the Swift form of the no-silent-errors principle: Rust's `.unwrap()` and `panic!()` (IN-RS-CODE-001) are the same decision.

An implicitly unwrapped optional (`var name: String!`) is a force unwrap deferred to every read of it, so its declaration is in scope too.

## Detection

Static signals:

- `try!` on any throwing call.
- `as!` on any cast.
- A postfix `!` after an identifier, a call's `)` or a subscript's `]`: `user!.name`, `load()!`, `items[0]!`.
- A type written with a trailing `!`: an implicitly unwrapped optional.
- `fatalError(` outside a `guard`'s `else`. Inside one it is the guard's exit, which IN-SW-CODE-001 allows, and only a reading can tell the two apart. So this signal is the `critic-swift` subagent's, not the runner's.

**GREPPABLE PROXY, ARMED FOR THREE SIGNALS. `fatalError(` IS NOT ONE OF THEM.** A line grep cannot see whether the line sits in a `guard`'s `else`, so arming it would set this rule against IN-SW-CODE-001, which names it as a legitimate exit.

**WHAT THE POSTFIX PATTERN SEES WRONGLY, AND WHAT IT CANNOT SEE.** The runner keeps only each line's single-quoted pattern and runs it as an extended regex with no lookbehind, so the postfix `!` is written as a bracket of what can precede it, the `!`, then anything but `=` or the end of the line:

- A `!` inside a string literal or a comment directly after a word fires: `print("done!")` and `// ready!` are findings. They are false positives, and the author answers them by reading the line.
- `!=` and `!==` do not fire: the `=` that follows is excluded. So `x!= y`, a force unwrap assigned through with no space before the `=`, does not fire either, and that is a false negative. With the space, `x! = y` fires.
- A prefix `!`, logical not, does not fire: `!x` and `if !done` have no identifier, `)` or `]` before the `!`.
- An implicitly unwrapped optional's declaration fires: `var name: String!`. That is in scope, not a cost, because the declaration is the deferred force unwrap.
- `try!` and `as!` also match the postfix pattern, since `y` and `s` precede the `!`. The runner reports one finding per rule per line, so such a line is one finding, not two.

**SCOPE IS `**/*.swift`, TESTS INCLUDED, AND THAT IS STATED RATHER THAN HIDDEN.** The runner reaches a file by `applies_to` alone, the glob language cannot subtract a directory, and an Xcode project has no production-only glob the way a cargo crate has `src/`. `Sources/**` would miss an app laid out under its own name. In a test, the answer to a force operation is `try XCTUnwrap(value)` in a test method marked `throws`, which fails that test and names the line, where `value!` crashes the whole test run. The `does_not_apply_when` list above is read by the `critic-swift` subagent; the headless runner reports every matching line in every `.swift` file.

Greppable proxy (the headless `intent critic swift` runner, which is the pre-commit gate, reports every matching line in a file `applies_to` admits as a finding at this rule's severity; only the `critic-swift` subagent confirms by reading the body):

```bash
grep -rnE 'try!' .
grep -rnE 'as!' .
grep -rnE '[]A-Za-z0-9_)]!([^=]|$)' .
```

## Bad

```swift
func loadConfig() -> Config {
  let data = try! Data(contentsOf: configURL)
  let json = try! JSONSerialization.jsonObject(with: data) as! [String: Any]
  let name = json["name"] as? String
  return Config(name: name!)
}
```

Three ways to crash, none visible in the signature: a missing file, a payload that is not an object, and an object with no `name`.

## Good

```swift
enum ConfigError: Error {
  case notAnObject
  case missing(String)
}

func loadConfig() throws -> Config {
  let data = try Data(contentsOf: configURL)
  guard let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
    throw ConfigError.notAnObject
  }
  guard let name = json["name"] as? String else {
    throw ConfigError.missing("name")
  }
  return Config(name: name)
}
```

The signature says it throws, each failure has a name, and the caller decides what a missing config means. (IN-SW-CODE-005 would take this further, to `Codable`.)

In a test:

```swift
func testTheFixtureNamesItsProject() throws {
  let object = try JSONSerialization.jsonObject(with: fixture) as? [String: Any]
  let name = try XCTUnwrap(object?["name"] as? String)
  XCTAssertEqual(name, "intent")
}
```

## When This Applies

- Any call to a throwing function, any narrowing cast, and any read of an optional, in app, library and test code alike.
- Implicitly unwrapped optional declarations outside the framework-filled cases below.

## When This Does Not Apply

- `fatalError(` as the exit of a `guard`'s `else` (IN-SW-CODE-001), where the author has stated the precondition and its failure in one place.
- `@IBOutlet` properties, which Interface Builder declares implicitly unwrapped and connects before the view loads.
- A force unwrap of a value the compiler cannot prove present but the line next to it does, such as a `URL(string:)` on a literal the author has checked. It is rare, and it carries a comment saying why.

## Further Reading

- The Swift Programming Language, "Error Handling" (<https://docs.swift.org/swift-book/documentation/the-swift-programming-language/errorhandling/>)
- The Swift Programming Language, "Optionals" and "Implicitly Unwrapped Optionals" (<https://docs.swift.org/swift-book/documentation/the-swift-programming-language/thebasics/>)
- XCTest, `XCTUnwrap(_:_:file:line:)` (<https://developer.apple.com/documentation/xctest/3380195-xctunwrap>)
- IN-RS-CODE-001 -- the Rust form of the same decision (`.unwrap()`, `panic!()`)
- IN-AG-NO-SILENT-001 -- the agnostic no-silent-errors principle this rule concretises
