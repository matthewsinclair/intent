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

- AT-01.1 `tests/unit/foo.bats` -- covers AC-01.1 -- status: to-write -- red-first
- AT-01.2 `tests/unit/foo.bats` -- covers AC-01.2 -- status: red
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
  # The AT grammar's L2/L3 resolve the citation against the tree, and `at
  # red|green` refuse a dangling one (issues 0015 + 0017), so the fixture's
  # cited deck has to be real and has to carry the ids that cite it.
  mkdir -p tests/unit
  printf 'AT-01.1 AT-01.2 AT-02.1 AT-03.1\n' > tests/unit/foo.bats
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
  assert_output_contains "tests/unit/foo.bats"
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
  # THE STATUS LANDED **AND** THE NOTE SURVIVED (issue 0033). The `$` anchor
  # after `green` used to be the whole assertion, and it PASSED ONLY BECAUSE THE
  # NOTE HAD BEEN DESTROYED -- the fixture row carries ` -- red-first`, so a row
  # ending at the status is a row that lost a declared field. The test encoded
  # the defect as the expected shape, which is why twelve green tests over a
  # live data-loss bug were not a contradiction.
  run grep -E '^- AT-01\.1 .*-- status: green -- red-first$' "$ACC"
  assert_success
}

@test "done aliases green and notdone aliases red" {
  setup_fixture_st

  run run_intent at notdone ST0001 AT-01.1
  assert_success
  run grep -E '^- AT-01\.1 .*-- status: red -- red-first$' "$ACC"
  assert_success

  run run_intent at done ST0001 AT-01.1
  assert_success
  run grep -E '^- AT-01\.1 .*-- status: green -- red-first$' "$ACC"
  assert_success
}

@test "an at transition preserves the row note (issue 0033)" {
  # THE NOTE IS A DECLARED FIELD, NOT TRAILING TEXT. `AT_G_NOTE` is part of the
  # row grammar, the tool NORMALISES a malformed note and COACHES the author on
  # the delimiter -- and until 2026-08-19 every `at red` and `at green` then
  # discarded it, because the substitution ended `-- status:.*` and carried
  # nothing forward. Certain rather than probable: it fired on every transition
  # of every row that had one.
  #
  # It survived because NOTHING READ THE FIELD BACK. There was no `at_note`
  # reader; the post-write verifier checked field 5 while the damage was in
  # field 6; and the two assertions above anchored `status: green$`, so they
  # PASSED ONLY WHEN THE NOTE WAS GONE. A destroyed note leaves a grammatical
  # row, a correct status, a quiet linter and a satisfied gate -- the only
  # witness is an author who remembers writing something that is not there.
  setup_fixture_st

  run run_intent at red ST0001 AT-01.1
  assert_success
  run grep -cF -- '-- status: red -- red-first' "$ACC"
  assert_success

  run run_intent at green ST0001 AT-01.1
  assert_success
  run grep -cF -- '-- status: green -- red-first' "$ACC"
  assert_success
}

@test "a row with no note stays a row with no note" {
  # The other arm, and it is not symmetry for its own sake: a fix that preserves
  # notes by appending an empty ` -- ` would satisfy every assertion above and
  # push every unannotated row out of the grammar. AT-01.2 is seeded `red` with
  # no note.
  setup_fixture_st

  run run_intent at green ST0001 AT-01.2
  assert_success
  run grep -E '^- AT-01\.2 .*-- status: green$' "$ACC"
  assert_success
  run grep -cF -- 'status: green -- ' "$ACC"
  assert_failure
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

# ---- issue 0006 + 0007: the sed-non-match-is-invisible pair ----------------
# Both defects had the same shape in the same file. On the WRITE side, `ac
# satisfy` substituted into a ` -- evidence:` segment the row need not carry,
# and sed exits 0 having substituted nothing -- so the tool that records
# verified state printed `ok:` having written nothing. On the READ side, the
# field extractors were bare `sed -E 's/.../\1/'`, and sed prints the input
# unchanged on a non-match -- so a status the pattern could not parse came back
# as the whole AT line, compared false against "green", and gated a green test
# as unsatisfied in silence. These encode both directions.

@test "ac satisfy writes a bare row instead of reporting a success it did not perform" {
  setup_fixture_st

  # A non-test AC with NO evidence tail -- the shape that reproduced issue 0006.
  printf -- '- AC-00.9 (non-test) criterion whose text ends without an evidence tail\n' >> "$ACC"

  run run_intent ac satisfy ST0001 AC-00.9 --evidence "a real ref"
  assert_success

  # The claim in the message must be true on disk, which is what failed before.
  run grep -E '^- AC-00\.9 .* -- evidence: a real ref -- satisfied: yes$' "$ACC"
  assert_success

  # And the reader must agree with the record it just wrote.
  run run_intent ac list ST0001
  assert_success
  echo "$output" | grep -E 'AC-00\.9.*satisfied: yes'
}

@test "ac satisfy verifies the write landed rather than trusting sed's exit code" {
  setup_fixture_st

  printf -- '- AC-00.8 (non-test) criterion on a contract that cannot be written\n' >> "$ACC"
  chmod 444 "$ACC"
  run run_intent ac satisfy ST0001 AC-00.8 --evidence "ref"
  chmod 644 "$ACC"

  # No silent success: an unwritable contract is an error, not an `ok:`.
  assert_failure
  assert_output_contains "NOT updated"
  run grep -F "satisfied: yes" "$ACC"
  assert_failure
}

@test "an AT status carrying markdown emphasis is named, not silently read as not-green" {
  setup_fixture_st

  # A green test whose status field is bolded -- well-formed to every other
  # guard, and the exact shape that understated a live thread by half.
  printf -- '- AC-02.1 test-backed criterion\n' >> "$ACC"
  printf -- '- AT-02.1 `tests/unit/foo.bats` -- covers AC-02.1 -- status: **green; mutation-proved**\n' >> "$ACC"

  run run_intent ac list ST0001
  assert_success
  # The AC still reads unsatisfied (the parse genuinely failed) -- but it now
  # says so out loud, quoting the vocabulary, instead of leaving the author to
  # infer it from a gate that will not move.
  assert_output_contains "unreadable AT status"
  assert_output_contains "AT-02.1"
  assert_output_contains "to-write | red | green | n/a"

  # And the transition command diagnoses the row rather than the transition.
  run run_intent at green ST0001 AT-02.1
  assert_failure
  assert_output_contains "unreadable status"
}

@test "an out-of-vocabulary AT status is reported rather than ignored" {
  setup_fixture_st

  printf -- '- AT-03.1 `tests/unit/foo.bats` -- covers AC-03.1 -- status: BUILT\n' >> "$ACC"

  run run_intent at list ST0001
  assert_success
  assert_output_contains "unreadable AT status"
  assert_output_contains "AT-03.1"
}

@test "an unclosed non-test marker names the marker, not a missing test" {
  setup_fixture_st

  # `(non-test,` fails the exact-literal match, so the row is classified
  # test-backed: the gate hunts a covering AT, finds none, and reports the AC
  # unsatisfied while ignoring the satisfied: yes it already carries.
  printf -- '- AC-04.1 (non-test, see note) already flagged -- evidence: x -- satisfied: yes\n' >> "$ACC"

  run run_intent ac list ST0001
  assert_success
  assert_output_contains "unclosed non-test marker"
  assert_output_contains "AC-04.1"

  # The refusal must name the cause the author can act on -- the marker text --
  # not the conclusion the parser reached.
  run run_intent ac satisfy ST0001 AC-04.1 --evidence "y"
  assert_failure
  assert_output_contains "unclosed non-test marker"
}
