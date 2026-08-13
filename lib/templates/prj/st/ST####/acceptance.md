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
> AT status vocabulary: to-write (red-first) | red | green | n/a. `n/a` belongs to non-test rows ONLY -- it is the doc / eyeball / gate status, and a row carrying it must be marked `(non-test)`.
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). A `(non-test)` AT RECORDS a doc / eyeball check; it never satisfies anything, because `n/a` is not green -- the satisfaction lives on the AC's own `(non-test)` line.
>
> **The AT row has an enforced grammar (`intent at lint`, and the close-gate).** Two shapes, and nothing else parses:
>
> ```
> - AT-<gg>.<n> `<repo-relative-path>` -- covers <AC-id>[, <AC-id>...] -- status: to-write|red|green[ -- <free note>]
> - AT-<gg>.<n> (non-test) <prose> -- covers <AC-id>[, <AC-id>...] -- status: n/a[ -- <free note>]
> ```
>
> The reference is the test FILE, backticked, repo-relative, with at least one `/` and no `:` -- not a test name, not a bare filename, not a selector. Name the test by putting the AT's own id INSIDE the test (`describe "AT-03.2 / AC-03.2: ..."`), which is checkable from both ends and survives rewording; a cited name is not. Coverage ids are comma-separated with nothing fused to them (no `and`, no trailing `:`, no possessive). Any trailing note is introduced by a spaced `--` separator, exactly as in the two shapes above, and is never parsed. `intent at lint <ID> --fix` migrates the mechanical part of a legacy contract.
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

    - AT-01.1 `[path/to/the_test_file]` -- covers AC-01.1 -- status: to-write -- red-first
    - AT-01.2 (non-test) [what was read / eyeballed] -- covers AC-01.2 -- status: n/a -- doc / eyeball
    - Coverage: [every AC has an AT, or list the uncovered ACs; non-test ACs carry evidence on the AC line]
