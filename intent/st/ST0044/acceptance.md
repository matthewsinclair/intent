---
verblock: "13 Jun 2026:v0.2: matts - model A: non-test ACs carry inline evidence/satisfied"
st_id: ST0044
title: "Add in acceptance.md and supporting process -- acceptance contract"
---

# ST0044 Add in acceptance.md and supporting process -- Acceptance

> Canonical acceptance contract for ST0044. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.
>
> STATUS: RATIFIED (matts, verifier, 2026-06-13) -- boundary = the 8-WP set below. Candidate ACs raised and held out of boundary: `intent st list/show` gate-verdict surfacing, close-gate override policy, `ac`/`at` help-text (revisit if needed). Owner directed default-on stamping: adding the template to the doc-set dir is the whole mechanism, so AC-01.2 (default-off) and AC-02.2 (prove-before-flip gate) are removed as void. Per the five-step the verifier witnesses each RED before production code. Bootstrap: AT states are tracked by hand in this file until WP-03 builds `intent at`. WP-01 GREEN (2026-06-13).

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) ST0044 is done when: `acceptance.md` is stamped into every new ST by default; the `intent ac` / `intent at` instrumentation and the close-gate are green; and ST0044's own build has been carried through the five-step with an independent verifier (matts) -- evidence: every AT below green + matts sign-off -- satisfied: no

### WP-01 -- acceptance.md in the default doc-set

- AC-01.1 `intent st new` stamps `acceptance.md` into every new ST's doc-set (default on, via the `lib/templates/prj/st/ST####/*.md` glob).
- AC-01.2 The stamped `acceptance.md` is sourced from `lib/templates/prj/st/ST####/acceptance.md` with placeholders substituted (no raw `ST####` survives).

### WP-02 -- acceptance.md template content

- AC-02.1 `lib/templates/prj/st/ST####/acceptance.md` carries the contract preamble, the AC section (ST-level + per-WP), and the AT section. (Content provisional -- refinable in place; no flip gate, since the default is on.)

### WP-03 -- intent ac / intent at instrumentation

- AC-03.1 `intent at list <stid>[/NN]` lists ATs (id, cited `path::name`, status); `intent ac list <stid>[/NN]` lists ACs with covering AT + derived satisfied state.
- AC-03.2 `intent at green` is reachable only from `red`; `intent at red` / `intent at na` set those states; `done` / `notdone` alias `green` / `red`.
- AC-03.3 `intent ac status <stid>` reports N/M ACs satisfied and the close-gate verdict (PASS / BLOCKED).
- AC-03.4 `intent ac satisfy <stid> <acid> --evidence` succeeds for non-test ACs only; test-backed ACs cannot be hand-satisfied.
- AC-03.5 All `ac` / `at` commands read and write `acceptance.md` only (single source of truth); in-place status edits are linter-stable.

### WP-04 -- Close-gate

- AC-04.1 `intent st done` / `intent wp done` refuse to close while any AC is unsatisfied (no green AT and no non-test evidence) or sign-off is unrecorded; the verdict is computed, never read from a hand-ticked box.

### WP-05 -- Template references (Highlander)

- AC-05.1 The `info.md` and `WP/info.md` templates reference `acceptance.md` and contain no restated ACs.

### WP-06 -- Skill / process integration

- AC-06.1 (non-test) The five-step is documented and mapped onto the skill set; the open-gate and close-gate are described where a builder will meet them -- evidence: doc + skill refs (pending WP-06) -- satisfied: no

### WP-07 -- Dogfood

- AC-07.1 ST0043 and ST0044 each carry an `acceptance.md`.
- AC-07.2 (non-test) ST0044's own build is run through the five-step with an independent verifier (matts) for at least one WP -- evidence: this file's AT states + matts sign-off -- satisfied: no

### WP-08 -- MODULES.md registration

- AC-08.1 (non-test) The parser + gate module is registered in `intent/llm/MODULES.md` before its code exists -- evidence: MODULES.md row precedes module code -- satisfied: no

## Acceptance Tests

### WP-01

- AT-01.1 `tests/unit/st_commands.bats::st new stamps acceptance.md into the doc-set` -- covers AC-01.1 -- status: green (red witnessed by matts 2026-06-13, then green)
- AT-01.2 `tests/unit/st_commands.bats::st new stamps acceptance.md content from the template file` -- covers AC-01.2, AC-02.1 -- status: green (red witnessed by matts 2026-06-13, then green)
- Coverage: AC-01.1 by AT-01.1; AC-01.2 and AC-02.1 by AT-01.2.

### WP-02

- Coverage: AC-02.1 covered by AT-01.2 (WP-01); no WP-02-specific AT.

### WP-03

- AT-03.1 `tests/unit/intent_acceptance_cli.bats::at list and ac list render ids, paths, status` -- covers AC-03.1 -- status: green
- AT-03.2 `tests/unit/intent_acceptance_cli.bats::green only from red; to-write to green is refused` -- covers AC-03.2 -- status: green
- AT-03.3 `tests/unit/intent_acceptance_cli.bats::done aliases green and notdone aliases red` -- covers AC-03.2 -- status: green
- AT-03.4 `tests/unit/intent_acceptance_cli.bats::ac status reports counts and gate verdict` -- covers AC-03.3 -- status: green
- AT-03.5 `tests/unit/intent_acceptance_cli.bats::ac satisfy refuses test-backed ACs` -- covers AC-03.4 -- status: green
- AT-03.6 `tests/unit/intent_acceptance_cli.bats::status edit is linter-stable` -- covers AC-03.5 -- status: green
- AT-03.7 `tests/unit/intent_acceptance_cli.bats::list accepts a bare numeric st id (normalised), like intent wp` -- covers AC-03.1 -- status: green (normalisation lap: red then green, 2026-06-13)
- Coverage: AC-03.1 (incl. id normalisation) through AC-03.5 each covered. Lap: red witnessed by matts 2026-06-13 (6/6 fail) -> green (6/6 pass); AT states set via the new `intent at` CLI. Review finding: `at` bypassed `normalise_st_id` (Highlander) -- fixed at the shared home (helpers:286), which also closed a latent octal misresolution (0044 -> ST0036) and added ST-prefixed padding; `tests/unit/helpers.bats` guards it.

### WP-04

- AT-04.1 `tests/unit/acceptance_close_gate.bats::wp done blocked while an AC is uncovered` -- covers AC-04.1 -- status: to-write (red-first)
- AT-04.2 `tests/unit/acceptance_close_gate.bats::st done blocked without recorded sign-off` -- covers AC-04.1 -- status: to-write (red-first)
- AT-04.3 `tests/unit/acceptance_close_gate.bats::done allowed when all green and signed` -- covers AC-04.1 -- status: to-write (red-first)
- Coverage: AC-04.1 covered.

### WP-05

- AT-05.1 `tests/unit/st_new_acceptance.bats::info templates reference acceptance.md and restate no ACs` -- covers AC-05.1 -- status: to-write (red-first)
- Coverage: AC-05.1 covered.

### WP-06

- Coverage: AC-06.1 is non-test; evidence + satisfied state live on the AC line.

### WP-07

- AT-07.1 `tests/unit/st_new_acceptance.bats::open STs ST0043 and ST0044 each have an acceptance.md` -- covers AC-07.1 -- status: to-write (red-first)
- Coverage: AC-07.1 by AT-07.1; AC-07.2 is non-test (evidence on the AC line).

### WP-08

- Coverage: AC-08.1 is non-test; evidence + satisfied state live on the AC line.
