---
id: IN-RS-CODE-005
language: rust
category: code
severity: style
title: Lifetime elision first
summary: >
  Prefer the elision rules to explicit `<'a>` annotations. The compiler
  accepts most signatures without them; reach for explicit lifetimes only
  when elision fails or when the relationship between references matters
  to the reader.
principles:
  - pfic
applies_when:
  - "Function signatures with reference parameters or reference return types"
  - "Struct definitions embedding references"
  - "Impl blocks binding lifetimes"
applies_to:
  - "src/**/*.rs"
does_not_apply_when:
  - "Signatures where elision would pick the wrong lifetime (rare — the compiler rejects these)"
  - "Struct types genuinely parameterised over lifetimes"
  - "Cases where the relationship between multiple input references needs to be visible to readers"
references:
  - IN-AG-PFIC-001
related_rules: []
aliases: []
tags:
  - rust
  - lifetimes
  - style
status: active
version: 1
---

# Lifetime elision first

Write the signature the compiler accepts without ceremony. Add lifetime names only when elision cannot derive them.

## Problem

Rust's three elision rules handle almost every common signature. When a function has a single `&self` or a single reference parameter, the compiler ties outputs to it automatically. Authors who annotate anyway — `fn f<'a>(x: &'a str) -> &'a str` when `fn f(x: &str) -> &str` compiles — add noise that makes the signature harder to scan. Reviewers learn to skip past the `<'a>` decorations instead of reading them; the real lifetime constraints, when they matter, get lost in the boilerplate.

The second problem is pedagogical. Beginners learning Rust copy every `<'a>` they see. If senior authors annotate unnecessarily, juniors believe the annotations are required and then panic when elision finally bites.

## Detection

Static signals:

- Function signatures with a single reference parameter that carry `<'a>`.
- `&self` plus one or more reference returns carrying an explicit lifetime that matches the `&self` lifetime (the second elision rule already does this).
- Impl blocks where every method in the block uses the same unused lifetime.

Clippy lints: `clippy::needless_lifetimes`, `clippy::extra_unused_lifetimes`.

## Bad

```rust
pub fn first_word<'a>(s: &'a str) -> &'a str {
  s.split_whitespace().next().unwrap_or("")
}

impl<'a> Greeter<'a> {
  pub fn greet<'b>(&'a self, name: &'b str) -> &'a str {
    self.prefix
  }
}
```

The first annotation is pure noise — elision handles it. The second invents `'b` for a parameter whose lifetime nobody cares about.

## Good

```rust
pub fn first_word(s: &str) -> &str {
  s.split_whitespace().next().unwrap_or("")
}

impl Greeter<'_> {
  pub fn greet(&self, _name: &str) -> &str {
    self.prefix
  }
}
```

Same signatures, same behaviour, no lifetime clutter. When readers encounter `<'a>` elsewhere in the codebase, it _means something_.

## When This Applies

- Library function signatures: prefer the elided form wherever the compiler allows.
- Impl blocks on types that carry lifetimes: `impl Foo<'_> { ... }` is usually enough.
- Struct methods that return a reference tied to `&self`: no annotation needed.

## When This Does Not Apply

- Signatures with two input references and a reference output where the output lifetime matches a specific input. The compiler cannot elide this; you must write `fn min<'a>(a: &'a T, b: &'a T) -> &'a T`.
- Structs carrying references in fields: these _must_ be parameterised (`struct Parser<'input>`).
- Cases where the relationship between lifetimes is part of the public contract and worth naming for clarity, even if the compiler would elide.
- Teaching examples in `intent/plugins/claude/rules/**` (rule fixtures) and `tests/fixtures/critics/rust/**` (critic test inputs). These files often carry deliberate explicit lifetimes as a contrast against an adjacent `bad.rs` or to exercise Detection on a clean target; do not flag them.

## Further Reading

- The Rust Reference, "Lifetime elision" (<https://doc.rust-lang.org/reference/lifetime-elision.html>)
- The Rust Programming Language, ch. 10.3 "Validating References with Lifetimes" (<https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html>)
- Clippy lints: `clippy::needless_lifetimes`, `clippy::extra_unused_lifetimes`
