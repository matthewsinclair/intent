---
id: IN-RS-TEST-004
language: rust
category: test
severity: style
title: Trim debuginfo and never fail fast on a consolidated suite
summary: >
  One `[profile.dev] debug = "line-tables-only"` at the workspace root reaches
  every test target, keeping file and line in a backtrace while dropping the
  detail only a debugger reads. Pass `--no-fail-fast` at every `cargo test` call
  site, because consolidation makes a bare run report strictly less than before.
principles:
  - build-cost
applies_when:
  - "Standing up a cargo workspace"
  - "Consolidating test targets under IN-RS-TEST-003"
  - "Writing or reviewing a CI step that runs `cargo test`"
applies_to:
  - "**/Cargo.toml"
does_not_apply_when:
  - "A debugging session that genuinely needs full type and variable detail -- override locally rather than changing the committed profile"
  - "A single-crate project with one test target, where `--no-fail-fast` changes nothing today (it still costs nothing and survives the second target)"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-RS-TEST-003
aliases: []
tags:
  - rust
  - testing
  - cargo
  - build-cost
status: active
version: 1
---

# Trim debuginfo and never fail fast on a consolidated suite

Two keys, one per repo, and the second one buys back information the first change costs you.

## Problem

**Debuginfo.** Cargo's default `dev` profile is `debug = true` -- full debuginfo, emitted and linked into every test binary. `cargo test` inherits `dev`, so **one key at the workspace root reaches every test target** rather than needing a `[profile.test]` beside it. `line-tables-only` keeps the file and line a person actually reads off a failure and drops the type and variable detail only a debugger consumes.

**Fail-fast, and this is the half that gets forgotten.** With one target per file, a bare `cargo test` reported every failing target. **After consolidation the first failure hides all the others** -- Lamplight turned 17 independently-failing targets into one and lost sixteen reports. **You do not lose a flag's benefit by omitting it; you lose CI information you previously had.** Add `--no-fail-fast` in the same change as `autotests = false`, not after.

**It is not only a consolidation artefact.** Cargo stops after the first failing target, so the mechanism bites at two targets and worsens with every crate added. A workspace with a lib and a bin already has it.

## Detection

Static signals:

- A workspace root `Cargo.toml` with no `[profile.dev]` section, or one carrying `debug = true`.
- A CI step, script, or Makefile invoking `cargo test` with no `--no-fail-fast`.
- A suite that was consolidated under `IN-RS-TEST-003` in a change that did not touch a single call site.

**NO GREPPABLE PROXY IS AUTHORITATIVE, AND ARMING ONE WAS DRIVEN AND REJECTED RATHER THAN WAIVED.** The profile half's violation is an ABSENCE, which the headless runner cannot express -- it matches positively only. **The one positive token, `debug = true`, was measured against this estate and produces a FALSE POSITIVE ON THE MANIFEST THAT IMPLEMENTS THE RULE:** the runner strips no comments, and Intent's workspace root carries `debug = true` inside the comment explaining what the default is. A rule that warns on the sentence documenting its own remedy is worse than an absent rule, because it reads as coverage.

**AND THE CALL-SITE HALF IS NOT A RUST POPULATION AT ALL.** `--no-fail-fast` lives in CI YAML, shell scripts and Makefiles, which `critic-rust` never sees. That half belongs to whoever reviews the pipeline, and saying so is more useful than a rule that cannot reach it.

Apply via the `critic-rust` subagent during `/in-review`; the mechanical arm is a workspace guard, not the headless gate.

## Bad

```toml
# workspace root: no profile at all. Every test binary carries full debuginfo.
[workspace]
members = ["crates/a", "crates/b"]
```

```yaml
# CI, after consolidation: one target, so the first failure hides the rest.
- run: cargo test --workspace
```

## Good

```toml
[profile.dev]
# `cargo test` inherits `dev`, so one key here reaches every test target.
debug = "line-tables-only"
```

```yaml
- run: cargo test --workspace --no-fail-fast
```

## When This Applies

- Every cargo workspace, whether or not it has consolidated.
- Every call site that runs `cargo test`, including local convenience scripts.

## When This Does Not Apply

- A debugging session wanting full detail: override on the command line rather than committing a profile change that costs every future run.

## Further Reading

- `intent/docs/notes/tn001-one-test-target-per-crate.md` -- part 4, and Lamplight's measured sixteen lost reports.
- The cargo book, profile inheritance and `[profile.dev]`.
