---
id: IN-RS-TEST-001
language: rust
category: test
severity: warning
title: Colocated cfg(test) modules for unit tests
summary: >
  Unit tests that exercise private items live inside the module they test,
  behind `#[cfg(test)] mod tests { ... }`. The `tests/` directory is for
  integration tests against the public API only.
principles:
  - public-interface
applies_when:
  - "Writing a unit test that calls a `pub(crate)` or private function"
  - "Adding regression tests alongside a fix"
  - "Deciding between `#[cfg(test)]` vs `tests/` for a new test file"
applies_to:
  - "src/**/*.rs"
  - "tests/**/*.rs"
does_not_apply_when:
  - "Integration tests that exercise only the public API and read like a consumer would"
  - "Benchmarks under `benches/`"
  - "Doc tests in `///` comments (already colocated by definition)"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-RS-TEST-002
aliases: []
tags:
  - rust
  - testing
  - cfg-test
status: active
version: 1
---

# Colocated cfg(test) modules for unit tests

Unit tests belong next to the unit. Integration tests belong in `tests/`. Confusing the two is why people say "Rust makes testing hard".

## Problem

Rust's `tests/` directory is a special place: each file there is compiled as a separate crate that depends on the main crate as an external library. That means `tests/` files can only call `pub` items — private helpers, `pub(crate)` utilities, and module-internal details are invisible. Authors who put every test under `tests/` hit this wall, then widen visibility to make tests compile, gradually turning implementation details into parts of the public API.

The correct split is structural: `#[cfg(test)] mod tests { use super::*; }` inside the module under test has full access to private items and is excluded from release builds. `tests/` exists specifically for tests that read like a downstream crate — instantiating public types, calling public functions, asserting on public behaviour. Putting a unit test in `tests/` either leaks visibility or duplicates the thing under test.

## Detection

Static signals:

- A `tests/` file that imports `pub(crate)` or non-public items (shouldn't compile, but `pub` widening often follows).
- Functions or types marked `pub` solely because a `tests/` file needs them.
- Source files with no colocated `#[cfg(test)] mod tests { ... }` despite having non-trivial internal logic.
- Tests in `tests/` whose first action is to call a setup function that exists only to reach private state.

## Bad

```rust
// src/parser.rs
pub fn parse(input: &str) -> Result<Ast, Error> { /* ... */ }

// Private helpers — but widened to `pub` so tests/ can reach them.
pub fn tokenize(input: &str) -> Vec<Token> { /* ... */ }
pub fn build_ast(tokens: &[Token]) -> Result<Ast, Error> { /* ... */ }

// tests/parser_internals.rs
use my_crate::{tokenize, build_ast};

#[test]
fn tokenizes_empty_input() {
  assert_eq!(tokenize(""), vec![]);
}
```

The helpers leaked into the public API just to be testable.

## Good

```rust
// src/parser.rs
pub fn parse(input: &str) -> Result<Ast, Error> {
  let tokens = tokenize(input);
  build_ast(&tokens)
}

fn tokenize(input: &str) -> Vec<Token> { /* ... */ }
fn build_ast(tokens: &[Token]) -> Result<Ast, Error> { /* ... */ }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tokenizes_empty_input() {
    assert_eq!(tokenize(""), vec![]);
  }

  #[test]
  fn parses_end_to_end() {
    assert_matches!(parse("1 + 2"), Ok(_));
  }
}

// tests/public_api.rs — only public API.
use my_crate::parse;

#[test]
fn public_parse_succeeds_on_well_formed_input() {
  assert!(parse("1 + 2").is_ok());
}
```

Private helpers stay private. Unit tests reach them via `use super::*;`. Integration tests pretend to be a downstream crate.

## When This Applies

- Any unit test that needs access to a private function, type, or constant.
- Regression tests for fixes inside a module.
- Tests that mock or stub internal collaborators by replacing private types.

## When This Does Not Apply

- Integration tests modelled as "what would a user of this crate see?" — put those in `tests/`.
- Benchmarks — `benches/` is the right place, not `#[cfg(test)]`.
- Tests that depend on features compiled only for the test binary (e.g. `#[cfg(feature = "test-util")]` exposed types).

## Further Reading

- The Rust Programming Language, ch. 11 "Writing Automated Tests" (<https://doc.rust-lang.org/book/ch11-00-testing.html>)
- Rust API Guidelines, "Unit tests live in the same file" (<https://rust-lang.github.io/api-guidelines/>)
- IN-RS-TEST-002 — use `assert_matches!` to test variants rather than `assert_eq!` on long literals
