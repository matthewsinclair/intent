---
st_id: ST0048
title: Acceptance close-gate fails empty or missing contract
---

# ST0048: Acceptance close-gate fails empty or missing contract -- Acceptance

> Canonical acceptance contract. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) The shipped close-gate matches the hv ruling: a WP/ST with no verifiable contract (zero in-scope ACs, or no `acceptance.md`) fails to close, and `acceptance: exempt` is the sole, visible escape. -- evidence: hv signed off 2026-06-29 (whiteboard session); fix + 2.13.x release notes reviewed -- satisfied: yes

### WP-01 -- Gate hard-fails empty/missing contract + exemption (status: Done)

- AC-01.1 `intent ac gate <st>` exits non-zero with a BLOCKED report when `acceptance.md` is present but has zero in-scope ACs and no exemption (was a vacuous exit 0). -- satisfied: yes (computed)
- AC-01.2 `intent ac gate <st>` exits non-zero with a BLOCKED report when `acceptance.md` is absent (no contract = fail), and the message names the escape hatch. -- satisfied: yes (computed)
- AC-01.3 `acceptance: exempt` in `acceptance.md` frontmatter opens the gate (exit 0) for a zero-AC contract AND the gate prints an `EXEMPT` line naming the exemption (No-Silent: announced, not silent). -- satisfied: yes (computed)
- AC-01.4 A real contract is unaffected: all-satisfied still passes (exit 0); any-unsatisfied still BLOCKS; malformed AC/AT lines still BLOCK (F1 regression guard). -- satisfied: yes (computed)
- AC-01.5 `intent wp done <st>/NN` honours the ratified WP-granularity rule (design.md D3, WP-lenient): a WP with no own `AC-NN.x` closes iff the thread carries >=1 real AC or is exempt; a fully-empty, non-exempt thread BLOCKS `wp done`. -- satisfied: yes (computed)

### WP-02 -- Template + canon docs reflect fail-by-default (status: Done)

- AC-02.1 (non-test) The stamped `acceptance.md` template documents the `acceptance: exempt` marker and ships WITHOUT it (default = enforced). -- evidence: template documents 'acceptance: exempt' in the preamble and ships WITHOUT the marker (default = enforced) -- satisfied: yes
- AC-02.2 No "opt-in / legacy-safe / closes exactly as before" close-gate claim survives in the canon or code comments (gate header `bin/intent_acceptance`, `bin/intent_st` + `bin/intent_wp` consumer comments, `intent/docs/working-with-llms.md` D11); all describe fail-by-default + exemption. (Highlander) -- satisfied: yes (computed)

### WP-03 -- Migration + 2.13.1 release wrap (status: Done)

- AC-03.1 (non-test) `docs/releases/2.13.1/RELEASE_NOTES.md` exists and LEADS with the behaviour change + the migration recipe (author ACs, or add `acceptance: exempt`). -- evidence: docs/releases/2.13.1/RELEASE_NOTES.md authored -- satisfied: yes
- AC-03.2 (non-test) Version stamped 2.13.1 (config.json + templates) + a CHANGELOG entry. -- evidence: v2.13.1 tagged (d01a1b2) + pushed to both remotes + GitHub release; VERSION=2.13.1; config.json intent_version=2.13.1 (intent upgrade stamp) -- satisfied: yes
- AC-03.3 (non-test) Dogfood: ST0048's own contract passes the NEW gate at close (`intent ac gate ST0048` exit 0 with real, satisfied ACs) -- the feature gates its own steel thread. -- evidence: dogfood: WP-01 + WP-02 both closed THROUGH the hardened gate (done: ST0048/WP-01, ST0048/WP-02) -- satisfied: yes

## Acceptance Tests

### ST-level

_(no tests in this group)_

### WP-01 -- Gate hard-fails empty/missing contract + exemption (status: Done)

- AT-01.1 `tests/unit/acceptance_close_gate.bats` -- covers AC-01.1 -- status: green -- test: "gate blocks a present contract with zero in-scope ACs"
- AT-01.2 `tests/unit/acceptance_close_gate.bats` -- covers AC-01.2 -- status: green -- test: "gate blocks a missing acceptance.md (no contract)"
- AT-01.3 `tests/unit/acceptance_close_gate.bats` -- covers AC-01.3 -- status: green -- test: "gate passes and reports EXEMPT for acceptance: exempt with zero ACs"
- AT-01.4 `tests/unit/acceptance_close_gate.bats` -- covers AC-01.4 -- status: green -- test: "gate unaffected by a real satisfied contract; still blocks unsatisfied and malformed"
- AT-01.5 `tests/unit/acceptance_close_gate.bats` -- covers AC-01.5 -- status: green -- test: "wp done honours the WP-lenient granularity rule"

### WP-02 -- Template + canon docs reflect fail-by-default (status: Done)

- AT-02.1 `tests/unit/acceptance_close_gate.bats` -- covers AC-02.2 -- status: green -- test: "no opt-in or closes-as-before claim survives in canon and comments"

### WP-03 -- Migration + 2.13.1 release wrap (status: Done)

_(no tests in this group)_

---

_Generated by Intent v3.0.0-dev from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
