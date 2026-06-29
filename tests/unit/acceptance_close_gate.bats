#!/usr/bin/env bats
# ST0044 WP-04: the acceptance close-gate on intent st done / intent wp done.
#
# Red-first: AT-04.1 and AT-04.2 (the "must block" cases) fail until the gate
# exists -- with no gate, done always succeeds, so the assert_failure trips.
# AT-04.3 guards the allow side: done must still succeed when every in-scope AC
# is satisfied (incl. sign-off). @test names are cited by AT-04.x in
# intent/st/ST0044/acceptance.md.
#
# ST0048 hardens the gate: an empty or missing contract now FAILS the close
# (was a vacuous pass), with `acceptance: exempt` as the sole escape. The
# freshly-stamped-ST and no-acceptance.md "stays open" cases (retired AT-04.4 +
# its sibling) are inverted below; ST0048 @test names are cited by AT-01.x in
# intent/st/ST0048/acceptance.md.

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

# ST0048: the close-gate now FAILS an empty or missing contract (was a vacuous
# pass). These invert the retired AT-04.4 (freshly-stamped ST closed) and the
# retired no-acceptance.md test (absent file stayed open). Red-first: each new
# "must block" / "must report EXEMPT" case fails against the pre-fix gate.

@test "gate blocks a present contract with zero in-scope ACs" {
  project_dir=$(create_test_project "Gate Empty Contract"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Empty Thread"; assert_success
  # Freshly stamped acceptance.md is present but carries guidance only -- zero
  # real (column-0) AC lines -- so "every AC satisfied" must not read as PASS.
  run run_intent ac gate ST0001
  assert_failure
  assert_output_contains "BLOCKED"
}

@test "gate blocks a missing acceptance.md (no contract)" {
  project_dir=$(create_test_project "Gate No Contract"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Contractless Thread"; assert_success
  rm -f intent/st/NOT-STARTED/ST0001/acceptance.md
  run run_intent ac gate ST0001
  assert_failure
  assert_output_contains "BLOCKED"
}

@test "gate passes and reports EXEMPT for acceptance: exempt with zero ACs" {
  project_dir=$(create_test_project "Gate Exempt"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Exempt Thread"; assert_success
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
acceptance: exempt
---
# ST0001 -- Acceptance

## Acceptance Criteria

(deliberately none -- this unit is acceptance-exempt)
EOF
  run run_intent ac gate ST0001
  assert_success
  assert_output_contains "EXEMPT"
}

@test "gate unaffected by a real satisfied contract; still blocks unsatisfied and malformed" {
  project_dir=$(create_test_project "Gate Regression Guard"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Guarded Thread"; assert_success

  # (a) a fully-satisfied non-test AC -> gate passes (happy path unchanged).
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) signed off -- evidence: done -- satisfied: yes
EOF
  run run_intent ac gate ST0001
  assert_success

  # (b) flip it unsatisfied -> gate still blocks.
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) signed off -- evidence: TBD -- satisfied: no
EOF
  run run_intent ac gate ST0001
  assert_failure
  assert_output_contains "BLOCKED"

  # (c) a malformed AC id -> gate still blocks loudly (F1).
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) the only well-formed AC -- evidence: done -- satisfied: yes

### Skill

- AC-U.1 (non-test) malformed id -- evidence: x -- satisfied: yes
EOF
  run run_intent ac gate ST0001
  assert_failure
  assert_output_contains "malformed"
}

@test "wp done honours the WP-lenient granularity rule" {
  project_dir=$(create_test_project "Gate WP Lenient"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Lenient Thread"; assert_success
  run run_intent wp new ST0001 "wp one"; assert_success

  # (a) fully-empty, non-exempt thread -> wp done BLOCKS (no contract anywhere).
  run run_intent wp done ST0001/01
  assert_failure
  assert_output_contains "BLOCKED"

  # (b) thread contracted at ST level, WP has no own ACs -> wp done rolls up.
  cat > intent/st/NOT-STARTED/ST0001/acceptance.md <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) thread-level boundary -- evidence: signed -- satisfied: yes
EOF
  run run_intent wp done ST0001/01
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

# ST0048 / AT-02.1: the close-gate is fail-by-default, so the retired "opt-in /
# legacy-safe / closes exactly as before" framing must not survive in the canon
# narrative or the consumer comments. (Other features -- the whiteboard, the
# PostToolUse advisory -- keep their own legitimate "opt-in by presence" wording;
# this guards the close-gate description only.) Greps the real repo via INTENT_HOME.
@test "no opt-in or closes-as-before claim survives in canon and comments" {
  # The gate + its two consumer comments describe only the close-gate: no opt-in.
  run bash -c "cd '$INTENT_HOME' && grep -niE 'opt-in' bin/intent_acceptance bin/intent_st bin/intent_wp || true"
  [ -z "$output" ]
  # working-with-llms.md mixes topics; the retired close-gate phrasings must be gone.
  run bash -c "cd '$INTENT_HOME' && grep -niE 'closes exactly as before|legacy-safe' intent/docs/working-with-llms.md || true"
  [ -z "$output" ]
}
