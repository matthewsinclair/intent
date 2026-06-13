---
verblock: "[Date]:v0.1: [Author] - Initial version"
st_id: ST####
title: "[Title] -- acceptance contract"
---

# ST#### [Title] -- Acceptance

> Canonical acceptance contract for ST####. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
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

[The "whole steel thread is done" bar, or "none -- WP-distributed".]

### WP-01 -- [WP title] (status: ...)

- AC-01.1 [a test-backed criterion -- what must be verifiably true]
- AC-01.2 (non-test) [a doc / eyeball / gate criterion] -- evidence: [named evidence] -- satisfied: no

## Acceptance Tests

### WP-01

- AT-01.1 [test path::name] -- covers AC-01.1 -- status: to-write (red-first)
- Coverage: [every AC has an AT, or list the uncovered ACs; non-test ACs carry evidence on the AC line]
