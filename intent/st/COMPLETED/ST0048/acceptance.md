---
verblock: "29 Jun 2026:v0.2: matts - Proposed AC/AT contract (pending hv ratification)"
st_id: ST0048
title: "Acceptance close-gate fails empty or missing contract -- acceptance contract"
---

# ST0048 Acceptance close-gate fails empty or missing contract -- Acceptance

> Canonical acceptance contract for ST0048. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.
>
> STATUS: PROPOSED. ACs await hv ratification (the open-gate). The one open fork is AC-01.5's WP-granularity rule (design.md D3) -- written here to the recommended WP-lenient reading.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) The shipped close-gate matches the hv ruling: a WP/ST with no verifiable contract (zero in-scope ACs, or no `acceptance.md`) fails to close, and `acceptance: exempt` is the sole, visible escape. -- evidence: hv signed off 2026-06-29 (whiteboard session); fix + 2.13.x release notes reviewed -- satisfied: yes

### WP-01 -- Gate hard-fails empty/missing contract + exemption (status: Not Started)

- AC-01.1 `intent ac gate <st>` exits non-zero with a BLOCKED report when `acceptance.md` is present but has zero in-scope ACs and no exemption (was a vacuous exit 0).
- AC-01.2 `intent ac gate <st>` exits non-zero with a BLOCKED report when `acceptance.md` is absent (no contract = fail), and the message names the escape hatch.
- AC-01.3 `acceptance: exempt` in `acceptance.md` frontmatter opens the gate (exit 0) for a zero-AC contract AND the gate prints an `EXEMPT` line naming the exemption (No-Silent: announced, not silent).
- AC-01.4 A real contract is unaffected: all-satisfied still passes (exit 0); any-unsatisfied still BLOCKS; malformed AC/AT lines still BLOCK (F1 regression guard).
- AC-01.5 `intent wp done <st>/NN` honours the ratified WP-granularity rule (design.md D3, WP-lenient): a WP with no own `AC-NN.x` closes iff the thread carries >=1 real AC or is exempt; a fully-empty, non-exempt thread BLOCKS `wp done`.

### WP-02 -- Template + canon docs reflect fail-by-default (status: Not Started)

- AC-02.1 (non-test) The stamped `acceptance.md` template documents the `acceptance: exempt` marker and ships WITHOUT it (default = enforced). -- evidence: template documents 'acceptance: exempt' in the preamble and ships WITHOUT the marker (default = enforced) -- lib/templates/prj/st/ST####/acceptance.md -- satisfied: yes
- AC-02.2 No "opt-in / legacy-safe / closes exactly as before" close-gate claim survives in the canon or code comments (gate header `bin/intent_acceptance`, `bin/intent_st` + `bin/intent_wp` consumer comments, `intent/docs/working-with-llms.md` D11); all describe fail-by-default + exemption. (Highlander)

### WP-03 -- Migration + 2.13.1 release wrap (status: Not Started)

- AC-03.1 (non-test) `docs/releases/2.13.1/RELEASE_NOTES.md` exists and LEADS with the behaviour change + the migration recipe (author ACs, or add `acceptance: exempt`). -- evidence: docs/releases/2.13.1/RELEASE_NOTES.md authored -- migration-led (leads with the behaviour change + the author-ACs / 'acceptance: exempt' recipe) -- satisfied: yes
- AC-03.2 (non-test) Version stamped 2.13.1 (config.json + templates) + a CHANGELOG entry. -- evidence: v2.13.1 tagged (d01a1b2) + pushed to both remotes + GitHub release; VERSION=2.13.1; config.json intent_version=2.13.1 (intent upgrade stamp) -- satisfied: yes
- AC-03.3 (non-test) Dogfood: ST0048's own contract passes the NEW gate at close (`intent ac gate ST0048` exit 0 with real, satisfied ACs) -- the feature gates its own steel thread. -- evidence: dogfood: WP-01 + WP-02 both closed THROUGH the hardened gate (done: ST0048/WP-01, ST0048/WP-02) -- the feature gated its own thread's units -- satisfied: yes

## Acceptance Tests

All test-backed ATs live in `tests/unit/acceptance_close_gate.bats` (extending the existing close-gate suite), authored red-first. AT ids map to its `@test` names.

### WP-01

- AT-01.1 acceptance_close_gate.bats::"gate blocks a present contract with zero in-scope ACs" -- covers AC-01.1 -- status: green
- AT-01.2 acceptance_close_gate.bats::"gate blocks a missing acceptance.md (no contract)" -- covers AC-01.2 -- status: green
- AT-01.3 acceptance_close_gate.bats::"gate passes and reports EXEMPT for acceptance: exempt with zero ACs" -- covers AC-01.3 -- status: green
- AT-01.4 acceptance_close_gate.bats::"gate unaffected by a real satisfied contract; still blocks unsatisfied and malformed" -- covers AC-01.4 -- status: green
- AT-01.5 acceptance_close_gate.bats::"wp done honours the WP-lenient granularity rule" -- covers AC-01.5 -- status: green
- Coverage: AC-01.1..01.5 test-backed (ATs above). AT-01.1 inverts the retired AT-04.4; AT-01.2 revises the retired no-acceptance.md open test.

### WP-02

- AT-02.1 acceptance_close_gate.bats::"no opt-in or closes-as-before claim survives in canon and comments" -- covers AC-02.2 -- status: green
- Coverage: AC-02.2 test-backed (mechanical grep guard above); AC-02.1 non-test (template diff evidence on the AC line).

### WP-03

- Coverage: AC-03.1..03.3 are non-test (release artifacts + the dogfood gate run); each carries its evidence inline on the AC line. No test-backed ATs in this WP.
