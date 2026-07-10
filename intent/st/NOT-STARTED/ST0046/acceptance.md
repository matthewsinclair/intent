---
verblock: "15 Jun 2026:v0.1: matts - Initial version"
st_id: ST0046
title: "Add modules (properly) to the intent cli -- acceptance contract"
---

# ST0046 Add modules (properly) to the intent cli -- Acceptance

> Canonical acceptance contract for ST0046. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.

## Acceptance Criteria

### ST-level

none -- WP-distributed.

### WP-01 -- Full-tree unregistered detector (status: open)

- AC-01.1 `intent modules check` flags a top-level module that exists on disk but has no `MODULES.md` row -- exits 1 and names the module + path.
- AC-01.2 `check` is umbrella-aware: it enumerates modules under every `apps/*/lib` source root, not just `./lib`.
- AC-01.3 `check` respects the configured exclusion set (test / support / generated dirs, `deps`, `_build` are not flagged).
- AC-01.4 `check` still reports stale rows (a registry row whose file is gone) -- no regression on existing behaviour.
- AC-01.5 (non-test) Run on the Lamplight umbrella, `check` now flags the known backlog (eg `Lamplight.Ingestor.Parser.Action`) instead of reporting `ok: registry matches filesystem` -- evidence: manual `intent modules check` on Lamplight post-fix -- satisfied: no

### WP-02 -- Generator / mechanical registration (status: open)

- AC-02.1 `intent modules sync` (or `check --write`) adds a row for every unregistered top-level module; afterwards `intent modules check` exits 0.
- AC-02.2 `sync` is idempotent: a second run produces a zero diff.
- AC-02.3 `sync` preserves existing rows verbatim -- hand-annotated provenance (ST/WP refs, `Highlander`) and curated thematic sub-sections survive.
- AC-02.4 Each new row carries the correct `module -> path` mapping and a default `auto-discovered` provenance.
- AC-02.5 Output ordering is deterministic (stable sort), so diffs are minimal and reviewable.

## Acceptance Tests

### WP-01

- AT-01.1 [test: modules check -- unregistered fixture flagged] -- covers AC-01.1 -- status: to-write (red-first)
- AT-01.2 [test: modules check -- umbrella apps/*/lib enumerated] -- covers AC-01.2 -- status: to-write (red-first)
- AT-01.3 [test: modules check -- exclusion set honoured] -- covers AC-01.3 -- status: to-write (red-first)
- AT-01.4 [test: modules check -- stale row still flagged] -- covers AC-01.4 -- status: to-write (red-first)
- Coverage: AC-01.1..4 AT-backed; AC-01.5 is non-test (evidence on the AC line).

### WP-02

- AT-02.1 [test: modules sync -- registers unregistered, check then green] -- covers AC-02.1 -- status: to-write (red-first)
- AT-02.2 [test: modules sync -- idempotent zero-diff on re-run] -- covers AC-02.2 -- status: to-write (red-first)
- AT-02.3 [test: modules sync -- preserves provenance + sub-sections] -- covers AC-02.3 -- status: to-write (red-first)
- AT-02.4 [test: modules sync -- new row module->path + provenance] -- covers AC-02.4 -- status: to-write (red-first)
- AT-02.5 [test: modules sync -- deterministic ordering] -- covers AC-02.5 -- status: to-write (red-first)
- Coverage: every WP-02 AC has an AT.
