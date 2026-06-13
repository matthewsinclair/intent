#!/usr/bin/env bats
# ST0044 WP-03: intent ac / intent at instrumentation over acceptance.md.
# Red-first: these encode the WP-03 ACs (AC-03.1..03.5) and must FAIL until
# bin/intent_acceptance + the ac/at dispatch wiring exist. The @test names are
# cited verbatim by AT-03.1..03.6 in intent/st/ST0044/acceptance.md (one home).

load "../lib/test_helper.bash"

# Seed a known canonical-grammar acceptance.md into an existing ST.
# Model A: non-test ACs carry `-- evidence: <ref> -- satisfied: yes|no` inline;
# test-backed ACs are satisfied by a green covering AT (computed, not written).
seed_acceptance() {
  local acc="$1"
  cat > "$acc" <<'EOF'
---
verblock: "13 Jun 2026:v0.1: test - fixture"
st_id: ST0001
title: "Fixture -- acceptance contract"
---

# ST0001 Fixture -- Acceptance

> Canonical acceptance fixture.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) the whole-thread bar -- evidence: TBD -- satisfied: no

### WP-01 -- sample (status: WIP)

- AC-01.1 first test-backed criterion
- AC-01.2 second test-backed criterion

## Acceptance Tests

### WP-01

- AT-01.1 `tests/unit/foo.bats::first` -- covers AC-01.1 -- status: to-write (red-first)
- AT-01.2 `tests/unit/foo.bats::second` -- covers AC-01.2 -- status: red
EOF
}

# Create a project + ST0001, seed the fixture; exports ACC and cds into project.
setup_fixture_st() {
  project_dir=$(create_test_project "AC/AT CLI Test")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Fixture Thread"
  assert_success
  ACC="intent/st/NOT-STARTED/ST0001/acceptance.md"
  seed_acceptance "$ACC"
}

@test "list accepts a bare numeric st id (normalised), like intent wp" {
  setup_fixture_st
  # Bare "1" must resolve to ST0001 just as ST0001 does -- parity with intent wp,
  # which routes the id through normalise_st_id before resolve_st_dir.
  run run_intent at list 1
  assert_success
  assert_output_contains "AT-01.1"
}

@test "at list and ac list render ids, paths, status" {
  setup_fixture_st

  run run_intent at list ST0001
  assert_success
  assert_output_contains "AT-01.1"
  assert_output_contains "tests/unit/foo.bats::first"
  assert_output_contains "to-write"

  run run_intent ac list ST0001
  assert_success
  assert_output_contains "AC-01.1"
  assert_output_contains "AT-01.1"
}

@test "green only from red; to-write to green is refused" {
  setup_fixture_st

  # AT-01.1 starts to-write: green must be refused (red-first guard).
  run run_intent at green ST0001 AT-01.1
  assert_failure
  assert_output_contains "only from red"

  # Legitimate path: to-write -> red -> green.
  run run_intent at red ST0001 AT-01.1
  assert_success
  run run_intent at green ST0001 AT-01.1
  assert_success
  run grep -E '^- AT-01\.1 .*status: green$' "$ACC"
  assert_success
}

@test "done aliases green and notdone aliases red" {
  setup_fixture_st

  run run_intent at notdone ST0001 AT-01.1
  assert_success
  run grep -E '^- AT-01\.1 .*status: red$' "$ACC"
  assert_success

  run run_intent at done ST0001 AT-01.1
  assert_success
  run grep -E '^- AT-01\.1 .*status: green$' "$ACC"
  assert_success
}

@test "ac status reports counts and gate verdict" {
  setup_fixture_st

  # Baseline: AC-00.1 (non-test, no) + AC-01.1 (AT to-write) + AC-01.2 (AT red) => 0/3 BLOCKED.
  run run_intent ac status ST0001
  assert_success
  assert_output_contains "0/3"
  assert_output_contains "BLOCKED"

  # Drive all three to satisfied.
  run run_intent at red ST0001 AT-01.1
  assert_success
  run run_intent at green ST0001 AT-01.1
  assert_success
  run run_intent at green ST0001 AT-01.2
  assert_success
  run run_intent ac satisfy ST0001 AC-00.1 --evidence "manual check"
  assert_success

  run run_intent ac status ST0001
  assert_success
  assert_output_contains "3/3"
  assert_output_contains "PASS"
}

@test "ac satisfy refuses test-backed ACs" {
  setup_fixture_st

  # AC-01.1 is test-backed: hand-satisfy must be refused.
  run run_intent ac satisfy ST0001 AC-01.1 --evidence "nope"
  assert_failure
  assert_output_contains "test-backed"

  # AC-00.1 is non-test: hand-satisfy stamps evidence + flips satisfied to yes.
  run run_intent ac satisfy ST0001 AC-00.1 --evidence "manual check"
  assert_success
  run grep -E '^- AC-00\.1 .*satisfied: yes$' "$ACC"
  assert_success
  run grep -F "manual check" "$ACC"
  assert_success
}

@test "status edit is linter-stable" {
  setup_fixture_st

  cp "$ACC" "$ACC.before"
  run run_intent at red ST0001 AT-01.1
  assert_success

  # Exactly one line changed, and it is the AT-01.1 status line.
  run bash -c "diff '$ACC.before' '$ACC' | grep -c '^> '"
  assert_output "1"
  run bash -c "diff '$ACC.before' '$ACC' | grep '^> '"
  assert_output_contains "AT-01.1"
  assert_output_contains "status: red"

  # No reflow: line count unchanged.
  run bash -c "test \$(wc -l < '$ACC.before') -eq \$(wc -l < '$ACC')"
  assert_success
}
