---
verblock: "11 Apr 2026:v0.1: matts - Initial version"
intent_version: 2.8.1
status: WIP
slug: fix-intent-s-elixir-credo-checks
created: 20260411
completed:
---

# ST0032: Fix Intent's Elixir Credo Checks

## Objective

Fix Intent's Credo check installation so that custom checks are properly wired into `.credo.exs` and fire when users run `mix credo --strict` directly. Clean up broken/noisy check templates and add missing checks.

## Context

Intent's `st zero install` (D5a) scaffolds custom Credo check `.ex` files into `credo_checks/` but never configures `.credo.exs` to load them. Both `intent_st_zero` and `intent_audit` print a wrong hint about adding `credo_checks` to `elixirc_paths` in `mix.exs` -- that's the Mix compilation mechanism, not Credo's loading system. The `intent audit` command works around this with Credo's `--checks-dir` flag, but any direct `mix credo` invocation misses the custom checks entirely. Every project that went through `st zero` has dead Credo checks.

Additionally, 2 of the 7 check templates produce 100% false positives (boolean_operators, dependency_graph), and 4 others have bugs ranging from over-flagging to incorrect return types. A new check (bracket_access_on_struct) is needed.

The fix uses a standalone Elixir script (following the autopsy.exs precedent) to programmatically parse and patch `.credo.exs` using `Code.eval_file`. Igniter was considered but requires being a target project dependency -- overkill for modifying a data literal.

Reference implementation: Lamplight's working setup in commit 50b122f0 (ST0131).

## Related Steel Threads

- ST0026: Added `intent audit` command and Credo integration (original implementation)
- ST0031: TCA suite hardening (post-Lamplight) -- discovered the broken wiring during Lamplight audit

## Scope

### In scope

- Create `configure_credo.exs` standalone script to patch `.credo.exs`
- Delete 2 broken templates (boolean_operators, dependency_graph)
- Create 1 new template (bracket_access_on_struct)
- Fix 4 buggy templates (map_get_on_struct, missing_impl_annotation, debug_artifacts, thick_coordinator)
- Update `intent_st_zero` D5a to call configure script instead of printing wrong hint
- Update `intent_audit` to remove `--checks-dir` workaround and stale rule references
- Update rule mappings (remove R8/D11, add R16)
- Update MODULES.md, help files

### Out of scope

- Auto-modifying mix.exs (too risky for a data-level script)
- Changing the `Mix.Checks.*` module namespace (breaking change for existing installs)
- Adding new check types beyond bracket_access_on_struct
