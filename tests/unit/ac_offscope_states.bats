#!/usr/bin/env bats
# Issue 0013: an AC's state was modelled as a boolean when practice has four.
#
# A requirement that moved to another thread, or was withdrawn outright, had no
# honest representation: `satisfy` is a lie (the work was not done) and leaving
# it unsatisfied is permanent (the AC counts against the thread forever, the
# gate reports BLOCKED, and a genuinely finished thread cannot be closed by the
# tool -- only by a human deciding to ignore it). These assert the two things
# that matter: the off-scope states do not block, and they are never silently
# dropped from the record.

load "../lib/test_helper.bash"

setup_two_threads() {
  project_dir=$(create_test_project "AC Offscope Test")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Owner Thread"
  assert_success
  run run_intent st new "Receiver Thread"
  assert_success
  ACC="intent/st/NOT-STARTED/ST0001/acceptance.md"
  mkdir -p tests/unit
  printf 'AT-01.1 lives here\n' > tests/unit/d.bats
  cat > "$ACC" <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

- AC-01.1 first criterion
- AC-01.2 second criterion, moving elsewhere
- AC-01.3 (non-test) a doc criterion -- evidence: read it -- satisfied: yes
- AC-01.4 a criterion being withdrawn

## Acceptance Tests

- AT-01.1 `tests/unit/d.bats` -- covers AC-01.1 -- status: green
EOF
}

@test "a descoped AC stops blocking and the gate passes a genuinely finished thread" {
  setup_two_threads
  # The live shape from the report: a thread sitting BLOCKED with no outstanding
  # work, because one AC moved to another thread by a hypervisor ruling.
  run run_intent ac status ST0001
  assert_output_contains "2/4"
  assert_output_contains "BLOCKED"

  run run_intent ac descope ST0001 AC-01.2 --to ST0002 --by hv --reason "belongs with the receiver work"
  assert_success
  run run_intent ac withdraw ST0001 AC-01.4 --reason "requirement dropped by hv ruling" --by hv
  assert_success

  run run_intent ac gate ST0001
  assert_success
  assert_output_contains "PASS"
}

@test "off-scope ACs are reported separately, never folded away" {
  setup_two_threads
  run run_intent ac descope ST0001 AC-01.2 --to ST0002 --by hv
  assert_success
  run run_intent ac withdraw ST0001 AC-01.4 --reason "dropped" --by hv
  assert_success

  # A thread that descoped half its contract has to look like one: the counts
  # are stated, not silently removed from the denominator.
  run run_intent ac status ST0001
  assert_success
  assert_output_contains "2/2 satisfied, 1 descoped, 1 withdrawn"
  assert_output_contains "PASS"

  run run_intent ac list ST0001
  assert_success
  assert_output_contains "descoped-to: ST0002"
  assert_output_contains "withdrawn: dropped"
}

@test "the audit trail lands on the line, which is the whole point over deleting it" {
  setup_two_threads
  run run_intent ac descope ST0001 AC-01.2 --to ST0002 --by hv --reason "moved with the work"
  assert_success
  run run_intent ac withdraw ST0001 AC-01.4 --reason "superseded by AC-01.1" --by hv
  assert_success

  # Which thread took it, on whose ruling, and when -- greppable and diffable.
  run grep -cE '^- AC-01\.2 .* -- descoped-to: ST0002 -- by: hv -- on: [0-9]{4}-[0-9]{2}-[0-9]{2} -- reason: moved with the work -- satisfied: n/a$' "$ACC"
  assert_output "1"
  run grep -cE '^- AC-01\.4 .* -- withdrawn: superseded by AC-01\.1 -- by: hv -- on: [0-9]{4}-[0-9]{2}-[0-9]{2} -- satisfied: n/a$' "$ACC"
  assert_output "1"
}

@test "a descope names a real thread, never itself, and never nothing" {
  setup_two_threads
  # A descope to a thread that does not exist is a strike with extra steps: the
  # value of recording a move is that the requirement stays owned.
  run run_intent ac descope ST0001 AC-01.2 --to ST9999
  assert_failure
  assert_output_contains "no such steel thread"

  run run_intent ac descope ST0001 AC-01.2 --to ST0001
  assert_failure
  assert_output_contains "its own thread"

  run run_intent ac descope ST0001 AC-01.2
  assert_failure
  assert_output_contains "requires --to"

  # Nothing was written by any of the three refusals.
  run grep -c 'descoped-to' "$ACC"
  assert_output "0"
}

@test "a withdrawal without a stated reason is refused" {
  setup_two_threads
  # Deleting the line is the practice this verb replaces, and the reason is the
  # only thing that distinguishes the two.
  run run_intent ac withdraw ST0001 AC-01.4
  assert_failure
  assert_output_contains "requires --reason"
  run grep -c -- ' -- withdrawn: ' "$ACC"
  assert_output "0"
}

@test "rescope and reinstate undo their own state and refuse the other's" {
  setup_two_threads
  run run_intent ac descope ST0001 AC-01.2 --to ST0002 --by hv
  assert_success
  run run_intent ac withdraw ST0001 AC-01.4 --reason "dropped" --by hv
  assert_success

  # Undoing a withdrawal and undoing a descope are different rulings; a verb
  # that accepted both would teach that they are interchangeable.
  run run_intent ac rescope ST0001 AC-01.4
  assert_failure
  assert_output_contains "use 'intent ac reinstate'"
  run run_intent ac reinstate ST0001 AC-01.2
  assert_failure
  assert_output_contains "use 'intent ac rescope'"

  run run_intent ac rescope ST0001 AC-01.2
  assert_success
  run run_intent ac reinstate ST0001 AC-01.4
  assert_success

  # Back to exactly the starting state: bare prose, and blocking again.
  run grep -cE '^- AC-01\.2 second criterion, moving elsewhere$' "$ACC"
  assert_output "1"
  run run_intent ac status ST0001
  assert_output_contains "2/4"
  assert_output_contains "BLOCKED"
}

@test "an off-scope AC is not re-entered by another off-scope verb" {
  setup_two_threads
  run run_intent ac descope ST0001 AC-01.2 --to ST0002 --by hv
  assert_success
  run run_intent ac withdraw ST0001 AC-01.2 --reason "changed my mind"
  assert_failure
  assert_output_contains "already descoped"
  # The first ruling survives the refused second one.
  run grep -c 'descoped-to: ST0002' "$ACC"
  assert_output "1"
}

@test "a descoped test-backed AC is off-scope before it is hunted for a covering AT" {
  setup_two_threads
  # Checking satisfaction first is what produced the false BLOCKED: a
  # test-backed AC whose AT went with it to the receiving thread finds no cover
  # and reports unsatisfied, when its actual state is "not this thread's".
  run run_intent ac descope ST0001 AC-01.1 --to ST0002 --by hv
  assert_success
  run run_intent ac list ST0001
  assert_success
  echo "$output" | grep -E 'AC-01\.1.*descoped-to: ST0002'
  refute_output_contains "AC-01.1  covered-by: AT-01.1  satisfied"
}

@test "a contract emptied by off-scope moves is refused, not passed on an empty set" {
  setup_two_threads
  run run_intent ac descope ST0001 AC-01.1 --to ST0002 --by hv
  assert_success
  for ac in AC-01.2 AC-01.3 AC-01.4; do
    run run_intent ac withdraw ST0001 "$ac" --reason "cleared" --by hv
    assert_success
  done

  # ST0048's rule is that an exemption is announced, never inferred from
  # emptiness -- and a contract emptied one descope at a time is still
  # emptiness. The refusal names the existing declared escape rather than
  # inventing a second implicit one.
  run run_intent ac gate ST0001
  assert_failure
  assert_output_contains "nothing is left to verify"
  assert_output_contains "acceptance: exempt"

  run run_intent st done ST0001
  assert_failure
  assert_output_contains "BLOCKED"
}

@test "a field value carrying the delimiter is refused rather than mangled" {
  setup_two_threads
  # The delimiter is the record format; a value containing it is not a value.
  run run_intent ac withdraw ST0001 AC-01.4 --reason "a -- b"
  assert_failure
  assert_output_contains "may not contain"
  run grep -c -- ' -- withdrawn: ' "$ACC"
  assert_output "0"
}

@test "satisfied: n/a is read as itself, not as a truncated token" {
  setup_two_threads
  run run_intent ac withdraw ST0001 AC-01.4 --reason "dropped" --by hv
  assert_success
  # The off-scope rows write `satisfied: n/a`, and a reader whose character
  # class stopped at the slash returned "n" -- a value, not a parse failure, and
  # so invisible in exactly the way issue 0007 was.
  run bash -c "source '${INTENT_HOME}/bin/intent_acceptance' 2>/dev/null; true"
  line=$(grep '^- AC-01.4 ' "$ACC")
  run bash -c "line='$line'; source '${INTENT_HOME}/bin/intent_helpers'; \
    printf '%s\n' \"\$line\" | sed -E 's@.* -- satisfied: ([a-z/]+).*@\1@'"
  assert_output "n/a"
}
