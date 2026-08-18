---
st_id: ST0044
title: Add in acceptance.md and supporting process
---

# ST0044: Add in acceptance.md and supporting process -- Acceptance

> Canonical acceptance contract. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) ST0044 is done when: `acceptance.md` is stamped into every new ST by default; the `intent ac` / `intent at` instrumentation and the close-gate are green; and ST0044's own build has been carried through the five-step with an independent verifier (matts) -- evidence: acceptance.md stamped by default; intent ac/at + close-gate green; ST0044's own build carried through the five-step with matts as verifier; every AT green + matts ST-level sign-off 2026-06-14 -- satisfied: yes

### Group 01

- AC-01.1 `intent st new` stamps `acceptance.md` into every new ST's doc-set (default on, via the `lib/templates/prj/st/ST####/*.md` glob). -- satisfied: yes (computed)
- AC-01.2 The stamped `acceptance.md` is sourced from `lib/templates/prj/st/ST####/acceptance.md` with placeholders substituted (no raw `ST####` survives). -- satisfied: yes (computed)

### Group 02

- AC-02.1 `lib/templates/prj/st/ST####/acceptance.md` carries the contract preamble, the AC section (ST-level + per-WP), and the AT section. (Content provisional -- refinable in place; no flip gate, since the default is on.) -- satisfied: yes (computed)

### Group 03

- AC-03.1 `intent at list <stid>[/NN]` lists ATs (id, cited `path::name`, status); `intent ac list <stid>[/NN]` lists ACs with covering AT + derived satisfied state. -- satisfied: yes (computed)
- AC-03.2 `intent at green` is reachable only from `red`; `intent at red` / `intent at na` set those states; `done` / `notdone` alias `green` / `red`. -- satisfied: yes (computed)
- AC-03.3 `intent ac status <stid>` reports N/M ACs satisfied and the close-gate verdict (PASS / BLOCKED). -- satisfied: yes (computed)
- AC-03.4 `intent ac satisfy <stid> <acid> --evidence` succeeds for non-test ACs only; test-backed ACs cannot be hand-satisfied. -- satisfied: yes (computed)
- AC-03.5 All `ac` / `at` commands read and write `acceptance.md` only (single source of truth); in-place status edits are linter-stable. -- satisfied: yes (computed)

### Group 04

- AC-04.1 `intent st done` / `intent wp done` refuse to close while any AC is unsatisfied (no green AT and no non-test evidence) or sign-off is unrecorded; the verdict is computed, never read from a hand-ticked box. -- satisfied: yes (computed)

### Group 05

- AC-05.1 The `info.md` and `WP/info.md` templates reference `acceptance.md` and contain no restated ACs. -- satisfied: yes (computed)
- AC-05.2 `intent st show <id> acceptance` displays `acceptance.md` and `intent st show <id> all` includes it in the doc-set; `intent st edit <id> <type>` (incl. `acceptance`) prints the file's absolute path instead of launching an editor. -- satisfied: yes (computed)

### Group 06

- AC-06.1 (non-test) The five-step is documented and mapped onto the skill set; the open-gate and close-gate are described where a builder will meet them -- evidence: working-with-llms.md D11 + in-plan/in-verify/in-finish pointers -- satisfied: yes

### Group 07

- AC-07.1 ST0043 and ST0044 each carry an `acceptance.md`. -- satisfied: yes (computed)
- AC-07.2 (non-test) ST0044's own build is run through the five-step with an independent verifier (matts) for at least one WP -- evidence: ST0044 built through the five-step with matts as verifier: ACs ratified open-gate (STATUS block) before code; matts witnessed RED on the WP-01/03/04/05 laps before each green build (Coverage lines); close-gate verdict computed by intent ac gate, not hand-ticked -- satisfied: yes

### Group 08

- AC-08.1 (non-test) The parser + gate module is registered in `intent/llm/MODULES.md` before its code exists -- evidence: intent/llm/MODULES.md bin/intent_acceptance row, registered before code (fa90bb2) -- satisfied: yes

## Acceptance Tests

### ST-level

_(no tests in this group)_

### Group 01

- AT-01.1 `tests/unit/st_commands.bats` -- covers AC-01.1 -- status: green -- red witnessed by matts 2026-06-13, then green -- test: st new stamps acceptance.md into the doc-set
- AT-01.2 `tests/unit/st_commands.bats` -- covers AC-01.2, AC-02.1 -- status: green -- red witnessed by matts 2026-06-13, then green -- test: st new stamps acceptance.md content from the template file

### Group 02

_(no tests in this group)_

### Group 03

- AT-03.1 `tests/unit/intent_acceptance_cli.bats` -- covers AC-03.1 -- status: green -- test: at list and ac list render ids, paths, status
- AT-03.2 `tests/unit/intent_acceptance_cli.bats` -- covers AC-03.2 -- status: green -- test: green only from red; to-write to green is refused
- AT-03.3 `tests/unit/intent_acceptance_cli.bats` -- covers AC-03.2 -- status: green -- test: done aliases green and notdone aliases red
- AT-03.4 `tests/unit/intent_acceptance_cli.bats` -- covers AC-03.3 -- status: green -- test: ac status reports counts and gate verdict
- AT-03.5 `tests/unit/intent_acceptance_cli.bats` -- covers AC-03.4 -- status: green -- test: ac satisfy refuses test-backed ACs
- AT-03.6 `tests/unit/intent_acceptance_cli.bats` -- covers AC-03.5 -- status: green -- test: status edit is linter-stable
- AT-03.7 `tests/unit/intent_acceptance_cli.bats` -- covers AC-03.1 -- status: green -- normalisation lap: red then green, 2026-06-13 -- test: list accepts a bare numeric st id (normalised), like intent wp

### Group 04

- AT-04.1 `tests/unit/acceptance_close_gate.bats` -- covers AC-04.1 -- status: green -- test: wp done is blocked while a WP AC is uncovered
- AT-04.2 `tests/unit/acceptance_close_gate.bats` -- covers AC-04.1 -- status: green -- test: st done is blocked when the ST-level sign-off AC is unsatisfied
- AT-04.3 `tests/unit/acceptance_close_gate.bats` -- covers AC-04.1 -- status: green -- test: st done is allowed once every AC including sign-off is satisfied
- AT-04.4 `tests/unit/acceptance_close_gate.bats` -- covers AC-04.1 -- status: green -- test: st done is not gated for a freshly stamped ST with no real ACs

### Group 05

- AT-05.1 `tests/unit/st_new_acceptance.bats` -- covers AC-05.1 -- status: green -- test: info templates reference acceptance.md and restate no ACs
- AT-05.2 `tests/unit/st_new_acceptance.bats` -- covers AC-05.2 -- status: green -- test: st show and edit know the acceptance file type

### Group 06

_(no tests in this group)_

### Group 07

- AT-07.1 `tests/unit/st_new_acceptance.bats` -- covers AC-07.1 -- status: green -- test: open STs ST0043 and ST0044 each have an acceptance.md

### Group 08

_(no tests in this group)_

---

_Generated by Intent v3.0.0-dev from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
