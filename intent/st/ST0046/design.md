# Design - ST0046: Add modules (properly) to the intent cli

## Approach

Two capabilities on the existing `intent modules` command, sharing one scan.

1. **Full-tree unregistered detector (fix `check`).** `intent modules check`
   enumerates every top-level module under the configured source roots and
   flags any without a `MODULES.md` row. For Elixir, a top-level module is a
   column-0 `defmodule X do` in a `*.ex` file; its expected row is
   ``| `X` | `<path>` | <provenance> |``. Umbrella-aware: when the project is
   an umbrella (a top-level `apps/` with per-app `mix.exs`), scan every
   `apps/*/lib`; otherwise scan `lib/`. Exclude a configurable set
   (test / support / generated, `deps`, `_build`). Exit 1 and name each
   unregistered module (and each stale row).

2. **Generator (`intent modules sync`, or `check --write`).** Mechanically
   reconcile `MODULES.md` to the filesystem: add a row for every unregistered
   module and flag (or remove) stale rows, idempotently. Preserve existing rows
   verbatim, including hand-annotated provenance (ST/WP refs, `Highlander`) and
   any curated thematic sub-sections; new rows take a default provenance
   (`auto-discovered`). Output is deterministically ordered so a re-run yields a
   zero diff.

## Design Decisions

- **Merge, never clobber.** The generator only ADDS missing rows (and flags
  stale ones); it preserves existing rows, their provenance, and curated
  thematic sub-sections (eg `### Reasoning subsystem`). Wholesale regeneration
  is rejected -- it would destroy curated provenance and structure.
- **Top-level modules only.** A column-0 `defmodule` is the unit; nested
  `defmodule`s belong to their file's top-level module and are not separately
  registered. This matches the registry's current granularity and keeps the
  scan unambiguous. (Registering nested modules is a possible later refinement,
  out of scope.)
- **One scan, two commands.** `check` and `sync` enumerate identically; `sync`
  is `check` plus a write.
- **Language-pluggable.** The module-enumeration step is per-language. Ship
  Elixir (column-0 `defmodule`); leave a seam for Rust / Swift / others, since
  Intent is multi-language.
- **Structural umbrella detection.** Detect umbrellas by `apps/*/mix.exs`, not
  by per-project config, to avoid setup.

## Architecture

```
source-root resolution (umbrella? apps/*/lib : lib/)
  -> per-language module enumeration (Elixir: column-0 defmodule -> {module, path})
  -> parse MODULES.md rows ({module, path, provenance})
  -> set difference:
       unregistered = on-disk \ registry      (check flags; sync adds)
       stale        = registry \ on-disk       (check flags; sync removes/flags)
  -> report (check, exit 0/1)  |  write (sync, merge-preserving)
```

`sync` writing rule: keep every existing row byte-for-byte; insert new rows in
the matching app/section in sorted position; default provenance
`auto-discovered`; never reorder or re-provenance existing rows.

## Alternatives Considered

- **Wholesale regeneration of `MODULES.md` from the scan.** Rejected: destroys
  curated provenance (ST/WP refs) and thematic sub-sections.
- **Rely solely on the creation-time hook.** Rejected: it cannot reconcile a
  pre-existing backlog, which is the actual problem.
- **Hand-author the backlog.** Rejected by the project owner: defeats the
  registry's purpose and does not scale (Lamplight alone is ~40 modules).
