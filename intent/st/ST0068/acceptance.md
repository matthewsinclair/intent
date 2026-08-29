---
st_id: ST0068
title: Update Intent's docs for v3
---

# ST0068: Update Intent's docs for v3 -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### Group 01

- AC-01.1 (non-test) The v2 blog and release notes live under docs/v2/ as a frozen record, their canonical URLs still name their old locations, and the archive states why they deliberately do not match their paths -- evidence: vc 2026-08-29: all 7 moved posts' canonical: fields still name .../docs/blog/<file>, driven per-file with a positive control that a rewritten canonical IS caught (planted /docs/v2/blog/ in a temp copy, flagged). docs/v2/README.md states why they deliberately do not match their paths. -- satisfied: yes

### Group 02

- AC-02.1 (non-test) docs/ carries an end-to-end set a new user can go from nothing to a satisfied thread with: install, getting started, concepts, command reference, migration -- satisfied: no
- AC-02.2 (non-test) No page documents a verb, flag or behaviour absent from the release that page describes -- measured against the cut, never against main -- satisfied: no
- AC-02.3 (non-test) Every defect in the published build that a reader can hit is stated in the docs rather than omitted, including the ones not fixed in the cut -- satisfied: no
- AC-02.4 (non-test) The command reference is generated from the register against a named revision, and its output carries that revision -- satisfied: no

### Group 03

- AC-03.1 (non-test) The site design system specifies one page Laksa can build without asking a question this thread could have answered -- satisfied: no
- AC-03.2 (non-test) Every unresolved design decision reaches the Laksa design agent carrying the decision, the constraint it must respect, and what breaks if it goes the other way -- satisfied: no

### Group 04

- AC-04.1 (non-test) intent/docs/ is unchanged in role, and every cross-link from the public set into it resolves -- evidence: vc 2026-08-29: intent/docs/ unchanged in role -- no file moved, added or removed; six cross-links from the public set now resolve into it (working-with-llms, rules, critics, pre-commit-hook, creating-custom-agents, writing-extensions) from the new docs/working-with-agents.md. Full docs/ sweep: 30 relative links across 12 files, 0 broken, checker two-sided controlled. The criterion was VACUOUS before this -- the public set mentioned intent/docs/ and linked into it nowhere, so the second clause was true of an empty set. -- satisfied: yes
- AC-04.2 (non-test) The doc set is in the same tag as v3.0.1, not a later one -- satisfied: no

## Acceptance Tests

### Group 01

- AT-01.1 (non-test) Read docs/v2/README.md and grep the moved posts' canonical: fields against their pre-move values -- covers AC-01.1 -- status: green -- Driven by vc, 2026-08-29. Positive-controlled: the check was shown to flag a planted rewrite before its pass was accepted.

### Group 02

- AT-02.1 (non-test) Follow docs/getting-started.md end to end on a fresh project and reach a satisfied criterion -- covers AC-02.1 -- status: to-write
- AT-02.2 (non-test) Diff every verb named in docs/ against the cut-surface artefact for the release the docs describe -- covers AC-02.2 -- status: to-write
- AT-02.3 (non-test) Read install.md and migrating-from-v2.md against the measured defect list for the published build -- covers AC-02.3 -- status: to-write
- AT-02.4 (non-test) Read the reference output's provenance header for the revision it names -- covers AC-02.4 -- status: to-write

### Group 03

- AT-03.1 (non-test) Laksa builds the page and reports whether it had to invent an answer -- covers AC-03.1 -- status: to-write
- AT-03.2 (non-test) Read the design system's decision register: each entry carries decision, constraint, consequence -- covers AC-03.2 -- status: to-write

### Group 04

- AT-04.1 (non-test) Resolve every link from docs/ into intent/docs/ -- covers AC-04.1 -- status: green -- Driven by vc, 2026-08-29. The check was vacuous until the links existed; six were added rather than the empty set being reported as a pass.
- AT-04.2 (non-test) git tag --contains on the doc commits against the v3.0.1 tag -- covers AC-04.2 -- status: to-write

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
