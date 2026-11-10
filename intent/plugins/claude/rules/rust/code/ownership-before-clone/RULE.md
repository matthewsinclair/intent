---
id: IN-RS-CODE-002
language: rust
category: code
severity: warning
title: Ownership before clone
summary: >
  Reflexive `.clone()` to silence the borrow checker is a smell. Prefer
  borrows (`&T`, `&mut T`, `&[T]`, `&str`) and move semantics; clone only
  when a new owned value is genuinely required.
principles:
  - highlander
applies_when:
  - "Function signatures that take `Vec<T>` or `String` when a slice or `&str` would do"
  - "`.clone()` calls inside hot paths, loops, or trait impls"
  - "`.to_owned()` / `.to_string()` on values already owned elsewhere"
applies_to:
  - "src/**/*.rs"
does_not_apply_when:
  - "Types that are `Copy` — `.clone()` compiles to a bitwise copy, no allocation"
  - "Spawning work onto another thread: `move` captures require owned data"
  - "Cross-`Arc<Mutex<T>>` boundaries where cloning the `Arc` is the intended cheap operation"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-RS-CODE-001
aliases: []
tags:
  - rust
  - ownership
  - performance
critic_tool: clippy
critic_tool_context: workspace
critic_tool_codes: [clippy::clone_on_copy, clippy::needless_pass_by_value]
status: active
version: 1
---

# Ownership before clone

Think about who owns the data first. Clone is the fallback when ownership truly needs to split, not the first response to a borrow-checker error.

## Problem

The Rust borrow checker teaches by example: return an owned `Vec<T>` and you pay for an allocation, take a `&[T]` and you do not. Reflexive `.clone()` to make an error go away hides that lesson. Small clones are free in microbenchmarks; they are expensive in hot paths, serialisation loops, request handlers, trait impls that run per-element. Large clones (a `Vec<LargeStruct>`, a deserialised JSON tree) allocate, copy, and drop — every call.

The second cost is conceptual. A signature that takes `Vec<T>` by value tells the caller "I need to own this; you will not have it afterwards". A signature that takes `&[T]` says "I just read from it; keep your copy". Conflating the two blurs the contract.

## Detection

Static signals:

- `.clone()` calls in function bodies where the cloned value is only read (never mutated, never moved into an owned collection).
- Function signatures that take `Vec<T>` / `String` / `HashMap<K, V>` when the body only iterates, reads, or slices.
- `.to_owned()` immediately after `.clone()` — double allocation.
- `.clone()` on iterator elements inside `.map(|x| x.clone())` where `.copied()` or `.cloned()` with a borrow would be cheaper.

Clippy lints: `clippy::needless_pass_by_value` and `clippy::clone_on_copy` answer for this rule. `clippy::redundant_clone` reads its primary signal but is not one of its tool codes (below).

**TOOL-ARMED AT A STATED COST, AND THE COST IS THE STABILITY OF THE LINT THAT ANSWERS THE RULE BEST.** `clone_on_copy` is in `clippy::all` and is free. `needless_pass_by_value` is `pedantic` -- opt-in, but stable. **`redundant_clone` is NURSERY: unstable by clippy's own classification, known for false positives, and it is the lint answering this rule's PRIMARY signal, which is a `.clone()` that exists only to quiet the borrow checker.**

**WHAT RUNS THEM.** The headless critic reports this rule ARMED but NOT RUN HERE and never invokes clippy, so each tool code is answered by a workspace run. `clone_on_copy` is in `clippy::all`, which CI and `bin/devbin check clippy` deny. `needless_pass_by_value` has its own CI step and devbin twin, `cargo clippy --workspace --lib --bins -- -D clippy::needless_pass_by_value`: library and binary code only, so tests and colocated `#[cfg(test)]` code stay out, as they do for IN-RS-CODE-001. Where a signature is a `map_err` adapter, `Result::map_err` hands the error over by value, and the site carries `#[allow(clippy::needless_pass_by_value, reason = "...")]`. **`redundant_clone` IS NOT A TOOL CODE, ON PURPOSE** (vc, 2026-11-10): a gate that moves with clippy's nursery group is not one to trust, so no gate runs it and the census does not claim it. A reviewer reads for the clone it would catch, and the pass that armed this rule fixed every site it raised in library and binary code. **A prediction made before this partition was re-derived said this rule was cleanly armable; it was not, and predicting "armable" from the mere PRESENCE of a lint name skipped the question of whether the lint is FIT to gate on.** Naming a parser is the start of the question, not the end of it.

**Context is `workspace`.** Reported as ARMED but NOT RUN HERE in a per-file run.

## Bad

```rust
pub fn sum_lengths(items: Vec<String>) -> usize {
  let copy = items.clone();
  copy.iter().map(|s| s.len()).sum()
}

pub fn find_user(users: Vec<User>, id: u32) -> Option<User> {
  users.iter().find(|u| u.id == id).cloned()
}
```

`sum_lengths` takes ownership it does not need, then clones again for no reason. `find_user` forces the caller to hand over the whole `Vec`.

## Good

```rust
pub fn sum_lengths(items: &[String]) -> usize {
  items.iter().map(|s| s.len()).sum()
}

pub fn find_user<'a>(users: &'a [User], id: u32) -> Option<&'a User> {
  users.iter().find(|u| u.id == id)
}
```

The signatures declare "I only read". The caller retains ownership and pays no allocation cost.

## When This Applies

- Library API design: default to borrows in parameters, return owned values only when producing new data.
- Inner loops: scan `.clone()` calls; most are signals that the ownership model should be rethought, not a valid use.
- Trait impls called per-element (`Iterator::map`, `FromStr::from_str`): an allocation per call compounds.

## When This Does Not Apply

- Types that implement `Copy` (integers, small structs with `#[derive(Copy)]`). `.clone()` is synonymous with `*` and equally cheap.
- Thread or task boundaries: `tokio::spawn` and `std::thread::spawn` require `'static + Send` captures, forcing owned data.
- Reference-counted sharing: `Arc::clone(&x)` is the deliberate, cheap way to share ownership across threads.

## Further Reading

- The Rust Programming Language, ch. 4 "Understanding Ownership" (<https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>)
- Rust API Guidelines, "Functions validate their arguments" (<https://rust-lang.github.io/api-guidelines/>)
- Clippy lints: `clippy::needless_pass_by_value`, `clippy::redundant_clone`, `clippy::clone_on_copy`
- IN-AG-HIGHLANDER-001 — single source of truth applies to data as well as code
