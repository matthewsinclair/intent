---
id: IN-RS-TEST-003
language: rust
category: test
severity: warning
title: One declared test target per crate
summary: >
  A crate with integration tests turns off automatic discovery and declares its
  targets. Under the default `autotests = true` every `.rs` under `tests/`
  becomes its own target -- a separate compile and a full link of the crate and
  all its dependencies.
principles:
  - build-cost
applies_when:
  - "Adding the first file under a crate's `tests/` directory"
  - "Standing up a new crate that will carry integration tests"
  - "Reviewing a Cargo.toml that gained a `tests/` directory"
applies_to:
  - "**/Cargo.toml"
does_not_apply_when:
  - "A crate whose tests are entirely inline `#[cfg(test)] mod tests` in `src/` -- there is no `tests/` directory and nothing to declare"
  - "A test file that genuinely needs its own process (mutating process cwd, binding a fixed port, driving a per-process watch stream) -- it keeps its own `[[test]]` with the reason recorded at the declaration"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-RS-TEST-001
  - IN-RS-TEST-004
aliases: []
tags:
  - rust
  - testing
  - cargo
  - build-cost
status: active
version: 1
---

# One declared test target per crate

Cargo's default is one linked binary per test file. Nothing warns you, and it grows one file at a time.

## Problem

Under `autotests = true` -- the default, in force whenever the key is absent -- **every `.rs` file directly under a crate's `tests/` directory becomes its own cargo target.** Each is a separate compile and a separate FULL LINK against the crate and its whole dependency graph.

It is invisible in the output, it is nobody's decision, and it is paid on every `cargo test`.

The fix is `autotests = false` and a declared `[[test]]` target in the manifest, plus a `tests/suite.rs` whose members are `#[path]` module declarations. **Use `#[path]` so that NO FILE MOVES**: relocating tests under `tests/suite/` breaks every acceptance-test row citing a test by path, and on a published release there may be no verb that retargets a row's file.

**TURNING DISCOVERY OFF INVERTS THE FAILURE RATHER THAN REMOVING IT.** Before, a stray `tests/quick.rs` silently became another target. After, a stray `tests/quick.rs` silently becomes NOTHING -- not compiled, not run, not reported. **Both are silent; only the second loses coverage somebody believed they had.** So `autotests = false` ships with an orphan guard or it does not ship.

## Detection

Static signals:

- A crate directory holding `tests/*.rs` whose `Cargo.toml` carries no `autotests` key.
- `autotests = false` with no `[[test]]` declared -- the armed quiet hole: any file under `tests/` is compiled by nothing.
- `cargo test --workspace --no-run 2>&1 | grep -c 'Executable tests/'` returning more targets than the manifests declare.

**NO GREPPABLE PROXY IS AUTHORITATIVE FOR THIS RULE, AND THE REASON IS STRUCTURAL RATHER THAN INCIDENTAL.** The violation is an ABSENCE -- a manifest that does not carry a key -- and the headless runner can express only a positive match. The same limit `IN-RS-TEST-001` records.

**AND A PROXY AIMED AT THE INVERSE WOULD FIRE ON CORRECT MANIFESTS. MEASURED, NOT ASSUMED:** the runner extracts only the single-quoted pattern from a grep block and discards flags and paths, and it strips no comments -- so a pattern matching `autotests` or `debug = true` matches the PROSE of a manifest documenting the rule. Intent's own workspace root carries `debug = true` inside a comment explaining what the default is, and would be reported as a violation by the file that implements the rule correctly.

**SO THE MECHANICAL ENFORCEMENT IS A TEST, NOT THE GATE.** A workspace-level guard walks every crate manifest with comments stripped and asserts the property, and asserts its own population before asserting the property. Apply this rule via the LLM-driven `critic-rust` subagent during `/in-review`; do not arm it in the headless pre-commit gate.

## Bad

```toml
# crates/mycrate/Cargo.toml -- one linked binary per file under tests/
[package]
name = "mycrate"
```

```toml
# Worse: discovery off, nothing declared. Passes today; the first file
# added under tests/ is compiled by nothing and runs nowhere.
autotests = false
```

## Good

```toml
autotests = false

[[test]]
name = "suite"
path = "tests/suite.rs"
```

```rust
// tests/suite.rs -- the files stay exactly where they were.
#[path = "cli_routing.rs"]
mod cli_routing;

// The orphan guard's own declaration is load-bearing: dropped from the suite it
// stops being compiled and stops reporting, which is its own defect applied to
// itself. No test closes that. Do not tidy it away.
#[path = "no_orphan_suite_member.rs"]
mod no_orphan_suite_member;
```

## When This Applies

- Any crate with at least one `.rs` directly under `tests/`.
- A new crate at the moment it gains its first integration test -- greenfield is the cheapest time, with nothing to migrate and no green rows at risk.

## When This Does Not Apply

- A crate whose tests are entirely inline `#[cfg(test)]` modules. There is no `tests/` directory, nothing is auto-discovered, and setting `autotests = false` would only arm the quiet hole. Leave the default and guard the TRANSITION instead.
- A test that genuinely needs process isolation. It keeps its own `[[test]]`, and the reason belongs at the declaration. **Ask what the test asserts: a test asserting something about the process it runs in wants its own process; a test asserting something about the product wants the product fixed.** Reaching for isolation whenever a merged suite goes red converts every product race it surfaces into a green.

## Further Reading

- `intent/docs/notes/tn001-one-test-target-per-crate.md` -- the estate-wide ruling, its measured cost, and the verification traps.
- The cargo book, `autotests` and target auto-discovery.
