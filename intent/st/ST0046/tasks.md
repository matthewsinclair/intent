# Tasks - ST0046: Add modules (properly) to the intent cli

## Tasks

### WP-01 -- Full-tree unregistered detector

- [ ] Source-root resolution: umbrella detection (`apps/*/mix.exs`) -> `apps/*/lib`, else `lib/`
- [ ] Elixir module enumeration: column-0 `defmodule` in `*.ex` -> `{module, path}`
- [ ] Configurable exclusion set (test / support / generated, `deps`, `_build`)
- [ ] Wire enumeration into `intent modules check` (flag unregistered; keep stale detection)
- [ ] Red-first tests (AT-01.1..4)

### WP-02 -- Generator

- [ ] `intent modules sync` (or `check --write`): merge-preserving writer
- [ ] Preserve existing rows + provenance + thematic sub-sections; default new provenance `auto-discovered`
- [ ] Deterministic ordering; idempotent zero-diff on re-run
- [ ] Red-first tests (AT-02.1..5)

### Validation

- [ ] Run `intent modules sync` on the Lamplight umbrella; confirm `check` then exits 0 (AC-01.5 evidence)

## Task Notes

Language-pluggable enumeration: ship Elixir, stub the seam for Rust / Swift / etc.
Motivating evidence is captured in info.md (the Lamplight gen4-substrate backlog
plus the proven `check` false-clean).

## Dependencies

- None external. WP-02 depends on WP-01's enumeration (the shared scan).
