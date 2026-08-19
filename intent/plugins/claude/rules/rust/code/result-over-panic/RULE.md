---
id: IN-RS-CODE-001
language: rust
category: code
severity: critical
title: Result over panic in library code
summary: >
  `.unwrap()`, `.expect()`, and `panic!()` in library code crash every caller.
  Prefer `Result<T, E>` and `?` so consumers can recover. Panics belong in
  `main()`, tests, and documented-infallible invariants only.
principles:
  - no-silent-errors
  - honest-data
applies_when:
  - "Functions in a library crate (`src/lib.rs` and descendants)"
  - "Any fallible operation: I/O, parsing, database lookup, network"
  - "Methods that return `Option<T>` or `Result<T, E>` from upstream APIs"
applies_to:
  - "src/**/*.rs"
does_not_apply_when:
  - "Binary crate `main.rs` where panics terminate the process cleanly"
  - "Tests (`#[cfg(test)]`, `tests/`, `benches/`) — panic is the failure signal"
  - "Infallible invariants documented with `// SAFETY:` or `// INVARIANT:` comments"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-RS-CODE-004
aliases: []
tags:
  - rust
  - error-handling
  - no-silent-errors
critic_tool: clippy
critic_tool_context: workspace
critic_tool_codes: [clippy::unwrap_used, clippy::expect_used, clippy::panic]
status: active
version: 1
---

# Result over panic in library code

Library code must never decide that a caller's program should die. Return `Result` and let the caller choose.

## Problem

`.unwrap()` and `panic!()` terminate the current thread. In a binary, that is a user-facing crash. In a library, it is a crash inside _someone else's_ program — a web server, a CLI tool, a GUI app — which had no way to anticipate your failure mode. The caller's error handling, logging, retries, and graceful shutdown all bypass; the process dies with a stack trace nobody planned for.

Beyond the production impact, `.unwrap()` hides the API's fallibility. A signature `fn load(id: u32) -> User` tells the reader "this cannot fail". A signature `fn load(id: u32) -> Result<User, Error>` tells the reader "handle the error". The second one is honest; the first lies until it panics.

## Detection

Static signals (Clippy lints `unwrap_used`, `expect_used`, `panic` are the canonical enforcers):

- `.unwrap()` or `.unwrap_or_else(|| panic!(..))` on an `Option` or `Result` in non-test code.
- `.expect("...")` outside tests. Even with a message, this is still a panic.
- Bare `panic!()`, `unreachable!()`, or `todo!()` in production paths.
- `assert!`, `assert_eq!`, `debug_assert!` used for input validation in a library API rather than invariant checks.

A rough grep — `rg -n '\.unwrap\(\)|\.expect\(' src/ --type rust | rg -v '#\[cfg\(test\)\]|// SAFETY:'` — flags candidates. The Critic inspects each to confirm the call is on a fallible value and not inside a `#[cfg(test)]` block.

**TOOL-ARMED: clippy -- `unwrap_used`, `expect_used`, `panic`, all in the `restriction` group and therefore opt-in.**

**THE COST THIS ROW USED TO CARRY IS GONE, AND IT WENT BY MEASUREMENT RATHER THAN BY ARGUMENT.** The concern was that arming here would flood test code, where `.unwrap()` is normal and correct. **Driven against clippy 0.1.97: default `cargo clippy` flags the production `.unwrap()` and does NOT flag a colocated `#[cfg(test)]` one; `--all-targets` flags both.** **So the remedy is the ABSENCE of a flag** -- and that is precisely why a rule file must never supply flags. A rule naming `--all-targets` would be a rule file contributing shell in order to make a lint see LESS, and the next reader would have no way to tell the flag was load-bearing. **The rule names the tool and the lints; the runner owns the invocation, and its job here is to not pass one specific flag.**

**RUN CONTEXT IS A SEPARATE AXIS FROM ARMING, AND THIS ROW IS WHY THE SECOND AXIS EXISTS.** `cargo clippy` is a whole-workspace compile. It does not belong in a per-commit hook at any arming mode -- it belongs where the compile already happens. So this rule is `critic_tool_context: workspace`, and a per-file run reports it as **ARMED but NOT RUN HERE** rather than as armed-and-clean. **Reporting a question that was never asked is the defect this pack exists to end; a real capability hidden behind a word meaning "nothing can answer this" is the same defect facing the other way.**

## Bad

```rust
use crate::db;

pub fn load(id: u32) -> User {
  let user = db::find(id).unwrap();
  user
}

pub fn parse_port(s: &str) -> u16 {
  s.parse().expect("port must be numeric")
}
```

Both functions panic on normal failure paths (missing user, bad input). Callers have no way to recover; they crash with the caller's thread.

## Good

```rust
use crate::db;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("user {0} not found")]
  NotFound(u32),
  #[error("invalid port: {0}")]
  InvalidPort(String),
}

pub fn load(id: u32) -> Result<User, Error> {
  db::find(id).ok_or(Error::NotFound(id))
}

pub fn parse_port(s: &str) -> Result<u16, Error> {
  s.parse().map_err(|_| Error::InvalidPort(s.into()))
}
```

Callers use `?` to propagate or `match` to handle. The API surface tells the truth.

## When This Applies

- Any function exported from a library crate that wraps a fallible operation.
- Internal helpers called from exported functions — propagating an `Option::None` up with `.unwrap()` leaks the panic through the public API boundary.
- Parser / validator / deserialiser entry points where user or network input is involved.

## When This Does Not Apply

- Binary crate `main()` where the application-level decision is "crash on unexpected state". `.expect("CONFIG must be set")` at startup is reasonable.
- Unit and integration tests: panic _is_ the failure mechanism.
- Invariants the compiler cannot verify but the author can. A `SAFETY:` comment plus `.unwrap()` on `Option<T>` where the `Some` case is structurally guaranteed is acceptable, but rare.

## Further Reading

- Rust API Guidelines, "Error types are meaningful" (<https://rust-lang.github.io/api-guidelines/dependability.html>)
- Clippy lints: `clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic`
- The Rust Programming Language, ch. 9 "Error Handling" (<https://doc.rust-lang.org/book/ch09-00-error-handling.html>)
- IN-RS-CODE-004 — error type selection (`thiserror` for libraries, `anyhow` for binaries)
- IN-AG-NO-SILENT-001 — the agnostic no-silent-errors principle this rule concretises
