#!/usr/bin/env bats
# ST0044 WP-04: the acceptance close-gate on intent st done / intent wp done.
#
# Red-first: AT-04.1 and AT-04.2 (the "must block" cases) fail until the gate
# exists -- with no gate, done always succeeds, so the assert_failure trips.
# AT-04.3 / AT-04.4 guard the allow side: done must still succeed when every
# in-scope AC is satisfied (incl. sign-off), and must NOT be gated for a
# freshly-stamped ST that carries no real AC lines (opt-in / legacy-safe).
# @test names are cited by AT-04.1..04.4 in intent/st/ST0044/acceptance.md.

load "../lib/test_helper.bash"

@test "wp done is blocked while a WP AC is uncovered" {
  project_dir=$(create_test_project "Close-gate WP Test"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Gate Thread"; assert_success
  run run_intent wp new ST0001 "Build it"; assert_success
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### WP-01 -- build (status: WIP)

- AC-01.1 the work is verifiably done

## Acceptance Tests

### WP-01

- AT-01.1 `tests/unit/x.bats::it works` -- covers AC-01.1 -- status: red
EOF
  run run_intent wp done ST0001/01
  assert_failure
  assert_output_contains "BLOCKED"
}

@test "st done is blocked when the ST-level sign-off AC is unsatisfied" {
  project_dir=$(create_test_project "Close-gate ST Test"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Gate Thread"; assert_success
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) the whole thread is signed off -- evidence: TBD -- satisfied: no

### WP-01 -- build (status: WIP)

- AC-01.1 the work is verifiably done

## Acceptance Tests

### WP-01

- AT-01.1 `tests/unit/x.bats::it works` -- covers AC-01.1 -- status: green
EOF
  # Every test-backed AC is green, but the sign-off AC is unsatisfied -> blocked.
  run run_intent st done ST0001
  assert_failure
  assert_output_contains "BLOCKED"
}

@test "st done is allowed once every AC including sign-off is satisfied" {
  project_dir=$(create_test_project "Close-gate Allow Test"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Gate Thread"; assert_success
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) the whole thread is signed off -- evidence: TBD -- satisfied: no

### WP-01 -- build (status: WIP)

- AC-01.1 the work is verifiably done

## Acceptance Tests

### WP-01

- AT-01.1 `tests/unit/x.bats::it works` -- covers AC-01.1 -- status: green
EOF
  run run_intent ac satisfy ST0001 AC-00.1 --evidence "signed off by tester"
  assert_success
  # Now AC-01.1 (green AT) + AC-00.1 (evidence) are both satisfied -> allowed.
  run run_intent st done ST0001
  assert_success
}

@test "st done is not gated for a freshly stamped ST with no real ACs" {
  project_dir=$(create_test_project "Close-gate Optin Test"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Fresh Thread"; assert_success
  # The stamped acceptance.md carries authoring guidance only, no live "- AC-"
  # lines, so the gate stays open and a fresh thread still closes.
  run run_intent st done ST0001
  assert_success
}

# v2.12.0 parser hardening (No Silent Errors): an AC/AT line that looks like an
# AC/AT but fails the strict numeric grammar must NOT be silently dropped.
# Pre-fix the malformed AC-U.1 vanished, leaving only the satisfied AC-00.1, so
# the gate PASSED (exit 0) -- this assert_failure would trip. Post-fix the gate
# detects the malformed line and blocks loudly.
@test "gate blocks loudly on a malformed AC id instead of silently dropping it" {
  project_dir=$(create_test_project "Close-gate Malformed Test"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Gate Thread"; assert_success
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) the only well-formed AC -- evidence: done -- satisfied: yes

### Skill

- AC-U.1 (non-test) a malformed letter-group id the parser must not silently drop -- evidence: x -- satisfied: yes
EOF
  run run_intent ac gate ST0001
  assert_failure
  assert_output_contains "malformed"
}

# Opt-in by presence (clarified 2026-06-16): a thread with NO acceptance.md has
# not opted into the AC regime, so the gate stays OPEN (exit 0) and it closes as
# it always did. Only a PRESENT acceptance.md is enforced (unsatisfied ACs or
# malformed lines block). Old STs are never forced to author a contract.
@test "gate stays open for a thread with no acceptance.md (opt-out by absence)" {
  project_dir=$(create_test_project "Close-gate No-Contract Test"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Contractless Thread"; assert_success
  rm -f intent/st/NOT-STARTED/ST0001/acceptance.md
  run run_intent ac gate ST0001
  assert_success
}
