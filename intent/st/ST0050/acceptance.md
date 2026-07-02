---
verblock: "02 Jul 2026:v0.1: matts - Initial version"
st_id: ST0050
title: "intent todo: a flat DOING/TODO/DONE view of steel threads and work packages -- acceptance contract"
---

# ST0050 intent todo: a flat DOING/TODO/DONE view of steel threads and work packages -- Acceptance

> Canonical acceptance contract for ST0050. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.
>
> Exemption (ST0048): the close-gate is fail-by-default -- a unit with an empty or missing contract is refused. A unit that is deliberately AC-free (eg a pure content / authorial task) declares `acceptance: exempt` in the frontmatter above; the gate then passes and announces the exemption. Omit it (the default) and the contract is enforced. Never inferred from emptiness; always declared.

## Acceptance Criteria

### ST-level

[The "whole steel thread is done" bar, or "none -- WP-distributed".]

### WP-01 -- [WP title] (status: ...)

[Add real AC lines at column 0 -- the parser and close-gate read only column-0 `- AC-` lines, so the indented examples below are inert guidance. Copy one to column 0 and fill it in:]

    - AC-01.1 [a test-backed criterion -- what must be verifiably true]
    - AC-01.2 (non-test) [a doc / eyeball / gate criterion] -- evidence: [named evidence] -- satisfied: no

## Acceptance Tests

### WP-01

[Add real AT lines at column 0 -- the parser reads only column-0 `- AT-` lines, so the indented examples below are inert guidance. Copy one to column 0 and fill it in:]

    - AT-01.1 [test path::name] -- covers AC-01.1 -- status: to-write (red-first)
    - Coverage: [every AC has an AT, or list the uncovered ACs; non-test ACs carry evidence on the AC line]
