---
id: IN-RS-TEST-002
language: rust
category: test
severity: warning
title: assert_matches! for tagged variants
summary: >
  When asserting on an enum variant that wraps data, use `assert_matches!`
  (or a `match`-based assertion) to test variant + selected fields. Avoid
  `assert_eq!` with a long literal of the full wrapped value; tests break
  on unrelated field additions.
principles:
  - honest-data
  - public-interface
applies_when:
  - "Asserting on a `Result<T, E>` where `T` or `E` is a complex struct"
  - "Asserting on an `Option<T>` where `T` has many fields"
  - "Asserting on a custom enum variant that wraps a struct"
applies_to:
  - "src/**/*.rs"
  - "tests/**/*.rs"
does_not_apply_when:
  - "Unit-type variants: `assert_eq!(result, Err(Error::NotFound))` is as specific as it gets"
  - "Primitive payloads: `assert_eq!(res, Ok(42))` is fine"
  - "Tests where the entire wrapped struct genuinely matters"
references:
  - IN-EX-TEST-001
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-RS-TEST-001
aliases: []
tags:
  - rust
  - testing
  - assertions
status: active
version: 1
---

# assert_matches! for tagged variants

Assert on the variant and the fields that matter. `assert_eq!` with a 20-line literal couples the test to every irrelevant field.

## Problem

`assert_eq!(result, Ok(User { id: 1, name: "Alice".into(), email: "alice@test".into(), created_at: Utc::now(), roles: vec![] }))` fails when anyone adds a field to `User`, even a field the test does not care about. Because `assert_eq!` requires structural equality, every test becomes a magnet for irrelevant edits — and worse, because `Utc::now()` changes per run, the test cannot even be expressed this way without time mocking.

The honest shape is: "assert the variant is `Ok` and that `id == 1` and `name == "Alice"`". That is what `assert_matches!` (from `std::assert_matches` in nightly, or the widely used `assert_matches` crate) was designed for.

## Detection

Static signals:

- `assert_eq!(result, Ok(...))` where the right-hand side is a struct literal with 3+ fields.
- Tests that break when an unrelated field is added to the wrapped type.
- Repetitive pattern-matching boilerplate: `match result { Ok(u) => { assert_eq!(u.id, 1); assert_eq!(u.name, "Alice"); } _ => panic!("...") }`.

Clippy has no direct lint; structural review and test-brittleness are the signals.

## Bad

```rust
#[test]
fn find_user_returns_alice() {
  let result = find_user(1);
  assert_eq!(
    result,
    Ok(User {
      id: 1,
      name: "Alice".into(),
      email: "alice@example.com".into(),
      created_at: Utc::now(),
      roles: vec![],
    })
  );
}
```

The test fails when `created_at` drifts by a microsecond or when `User` grows a `last_login` field.

## Good

```rust
use assert_matches::assert_matches;

#[test]
fn find_user_returns_alice() {
  let result = find_user(1);
  assert_matches!(result, Ok(User { id: 1, ref name, .. }) if name == "Alice");
}

#[test]
fn find_user_not_found_is_error() {
  assert_matches!(find_user(999), Err(Error::NotFound(999)));
}
```

The assertions name exactly what the test cares about. New fields do not break the test.

## When This Applies

- Any assertion on a `Result<T, E>` where `T` or `E` is a multi-field struct.
- Any assertion on an `Option<T>` where `T` has fields beyond the one the test is about.
- Custom enum variants whose payloads are structs rather than primitives.

## When This Does Not Apply

- Unit-type or primitive-payload variants. `assert_eq!(port, Ok(8080))` is more readable than `assert_matches!`.
- Tests where the full wrapped value genuinely matters (a serialisation round-trip, a data migration verifying byte-exact output).
- Property-based tests generating arbitrary structs — invariant-based assertions replace shape-based ones entirely.

## Further Reading

- `assert_matches` crate (<https://docs.rs/assert_matches>)
- `std::assert_matches!` RFC (<https://github.com/rust-lang/rust/issues/82775>)
- IN-EX-TEST-001 — Elixir equivalent (strong assertions against concrete values)
- IN-AG-HIGHLANDER-001 — one source of truth for assertion shape; do not repeat test shape everywhere
