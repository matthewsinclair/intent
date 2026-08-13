#!/usr/bin/env bats
# Issue 0017 (with 0014 + 0015): the AT row grammar and `intent at lint`.
#
# The defect these guard is not any single bad row -- it is that the AT row had
# no grammar at all, so every field was recovered by its own best-effort regex
# and each failed differently and silently. A row could name a CSS class as its
# test file, cite a deleted deck, or drop half its coverage, and stay green with
# nothing said. These assert the two things that fixes it: a conforming row is
# parsed whole, and a non-conforming row is REJECTED with a diagnosis that names
# the edit.

load "../lib/test_helper.bash"

# A project with ST0001, a real cited deck, and whatever AT rows the test needs.
# The deck is real because L2/L3 resolve the citation against the tree.
setup_contract() {
  project_dir=$(create_test_project "AT Grammar Test")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Grammar Thread"
  assert_success
  ACC="intent/st/NOT-STARTED/ST0001/acceptance.md"
  mkdir -p tests/unit
  printf 'AT-01.1 AT-01.2 AT-02.1 lives here\n' > tests/unit/deck.bats
  cat > "$ACC" <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

- AC-01.1 first criterion
- AC-01.2 second criterion
- AC-02.1 (non-test) a doc criterion -- evidence: read it -- satisfied: yes

## Acceptance Tests

EOF
}

at_row() { printf -- '%s\n' "$1" >> "$ACC"; }

@test "a conforming row is parsed whole: both arms, multi-AC coverage, delimited note" {
  setup_contract
  at_row '- AT-01.1 `tests/unit/deck.bats` -- covers AC-01.1, AC-01.2 -- status: green'
  at_row '- AT-02.1 (non-test) read the design doc -- covers AC-02.1 -- status: n/a -- doc / eyeball'

  run run_intent at lint ST0001
  assert_success
  assert_output_contains "2 AT row(s) conform"

  # Every field of the test row round-trips, including the multi-AC coverage
  # that 0014 reported as dropped.
  run run_intent at list ST0001
  assert_success
  assert_output_contains "tests/unit/deck.bats"
  run run_intent ac list ST0001
  assert_success
  echo "$output" | grep -E 'AC-01\.1.*AT-01\.1'
  echo "$output" | grep -E 'AC-01\.2.*AT-01\.2|AC-01\.2.*AT-01\.1'
}

@test "every retired reference form is rejected and told which edit to make" {
  setup_contract
  at_row '- AT-01.1 tests/unit/deck.bats::"a test name" -- covers AC-01.1 -- status: green'
  at_row '- AT-01.2 `deck.bats` -- covers AC-01.2 -- status: green'
  at_row '- AT-02.1 `md:absolute` -- covers AC-01.1 -- status: green'

  run run_intent at lint ST0001
  assert_failure
  # Each message names the row's OWN defect, not a generic non-match: the whole
  # point of a grammar is that the rejection is actionable.
  assert_output_contains "path::name' form is retired"
  assert_output_contains "bare filename"
  assert_output_contains "not a repo-relative path"
}

@test "a coverage clause that does not fully parse is named with the ids it did see" {
  setup_contract
  # 0014, corrected: the bare `and` separator WORKED at HEAD (at_covers
  # space-tokenises after comma translation) -- what silently dropped a link was
  # punctuation FUSED to an id. Both are now grammar failures, and the message
  # quotes what resolved so a reader cannot infer a false rule from a low count.
  at_row '- AT-01.1 `tests/unit/deck.bats` -- covers AC-01.1 and AC-01.2 -- status: green'
  at_row "- AT-01.2 \`tests/unit/deck.bats\` -- covers AC-01.2's second half -- status: green"

  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "coverage clause does not parse"
  assert_output_contains "comma-separated"
  assert_output_contains "ids seen: AC-01.1 AC-01.2"
}

@test "a green AT whose cited test file does not exist is a finding, and to-write is exempt" {
  setup_contract
  # Issue 0015 exactly: the gate got MORE permissive as citations rotted.
  at_row '- AT-01.1 `tests/unit/gone.bats` -- covers AC-01.1 -- status: green'
  at_row '- AT-01.2 `tests/unit/not_yet.bats` -- covers AC-01.2 -- status: to-write'

  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "L2 AT-01.1"
  assert_output_contains "does not exist"
  # A missing file is the CORRECT state for a test not yet written: a naive
  # existence check reds correct rows, which is why to-write is exempt.
  refute_output_contains "AT-01.2"
}

@test "the cited file must carry the AT id, so the link is checkable from both ends" {
  setup_contract
  printf 'this deck names nothing\n' > tests/unit/silent.bats
  at_row '- AT-01.1 `tests/unit/silent.bats` -- covers AC-01.1 -- status: green'

  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "L3 AT-01.1"
  assert_output_contains "does not carry the literal id"
}

@test "coverage of an id that is not an AC row in this contract is a finding" {
  setup_contract
  at_row '- AT-01.1 `tests/unit/deck.bats` -- covers AC-09.9 -- status: green'

  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "L4 AT-01.1"
  assert_output_contains "not an AC row in this contract"
}

@test "a non-test AT covering a test-backed AC is named as unsatisfiable" {
  setup_contract
  # The trap the non-test arm would otherwise bless: n/a is never green, so the
  # covered AC can never be satisfied and the contract can never close -- with
  # no symptom except a gate that will not move.
  at_row '- AT-01.1 (non-test) eyeballed it -- covers AC-01.1 -- status: n/a'

  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "L5 AT-01.1"
  assert_output_contains "never green"
}

@test "n/a and the non-test marker are enforced as a biconditional" {
  setup_contract
  at_row '- AT-01.1 `tests/unit/deck.bats` -- covers AC-01.1 -- status: n/a'
  at_row '- AT-01.2 (non-test) eyeballed it -- covers AC-01.2 -- status: green'

  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "n/a is the NON-TEST status"
  assert_output_contains "must carry 'status: n/a'"
}

@test "the trailing note must be delimited, and the shipped template teaches that form" {
  setup_contract
  at_row '- AT-01.1 `tests/unit/deck.bats` -- covers AC-01.1 -- status: green (mutation-proved)'

  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "trailing note must be delimited"

  # The template is the first thing an author copies, so it cannot teach a form
  # the parser rejects -- which is exactly how the retired `path::name` and the
  # parenthetical note entered the estate in the first place.
  run grep -c 'test path::name' "$INTENT_HOME/lib/templates/prj/st/ST####/acceptance.md"
  assert_output "0"
  run bash -c "grep -cE '^    - AT-01\.1 .*status: to-write -- red-first' '$INTENT_HOME/lib/templates/prj/st/ST####/acceptance.md'"
  assert_output "1"
}

@test "at lint --fix migrates the mechanical forms and leaves judgement alone" {
  setup_contract
  at_row '- AT-01.1 tests/unit/deck.bats::"a name" -- covers AC-01.1 -- status: green'
  at_row '- AT-01.2 `deck.bats` -- covers AC-01.2 -- status: **green**'
  at_row '- AT-02.1 (doc) read it -- covers AC-02.1 -- status: n/a (doc / eyeball)'
  at_row "- AT-02.2 \`tests/unit/deck.bats\` -- covers AC-01.1's half -- status: green"
  before=$(wc -l < "$ACC")

  run run_intent at lint ST0001 --fix
  # The mechanical half lands: name suffix stripped, bare filename expanded to
  # the one real match, emphasis dropped, (doc) marker and note canonicalised.
  run grep -cF -- '- AT-01.1 `tests/unit/deck.bats` -- covers AC-01.1 -- status: green' "$ACC"
  assert_output "1"
  run grep -cF -- '- AT-01.2 `tests/unit/deck.bats` -- covers AC-01.2 -- status: green' "$ACC"
  assert_output "1"
  run grep -cF -- '- AT-02.1 (non-test) read it -- covers AC-02.1 -- status: n/a -- doc / eyeball' "$ACC"
  assert_output "1"

  # The judgement call is NOT guessed: a fragment fused to a coverage id needs a
  # human to decide where the prose goes, so the row is left and reported. A
  # migrator that guessed would launder the drift the grammar exists to stop.
  run grep -cF -- "covers AC-01.1's half" "$ACC"
  assert_output "1"

  # Linter-stable: rewriting rows in place never reflows the file.
  run bash -c "test \$(wc -l < '$ACC') -eq $before"
  assert_success
}

@test "the close-gate blocks on a grammar finding and names the command that fixes it" {
  setup_contract
  at_row '- AT-01.1 `tests/unit/deck.bats::name` -- covers AC-01.1 -- status: green'
  at_row '- AT-01.2 `tests/unit/deck.bats` -- covers AC-01.2 -- status: green'

  run run_intent ac gate ST0001
  assert_failure
  assert_output_contains "BLOCKED"
  assert_output_contains "AT contract finding"
  assert_output_contains "intent at lint"
}

@test "at green and at red refuse a citation that does not resolve" {
  setup_contract
  # The rename is caught at the point of the lie rather than at the next gate,
  # which is where a green AT otherwise stays green forever (issue 0015).
  at_row '- AT-01.1 `tests/unit/gone.bats` -- covers AC-01.1 -- status: to-write'

  run run_intent at red ST0001 AT-01.1
  assert_failure
  assert_output_contains "cites a test file that does not exist"

  # And a real citation still transitions normally.
  at_row '- AT-01.2 `tests/unit/deck.bats` -- covers AC-01.2 -- status: to-write'
  run run_intent at red ST0001 AT-01.2
  assert_success
  run run_intent at green ST0001 AT-01.2
  assert_success
}

@test "a reference that cannot be a filename is never searched for, and never resolved by glob" {
  setup_contract
  # Reported from a 65GB estate where `at lint --fix` read as a hang: the
  # basename search walks the WHOLE repository, and it was running once per row
  # for the placeholder `[to-write]` -- a string that cannot be a filename.
  #
  # The correctness half is worse than the cost. `find -name` takes a GLOB, so
  # `[to-write]` is a character class matching any single-character filename.
  # This fixture plants exactly that: a file named `t`. Before the guard, the
  # search returned it as the unique hit and the row was rewritten to cite a
  # file with nothing to do with it -- a migrator laundering the drift the
  # grammar exists to stop. It must sit in a SUBDIRECTORY: the path group
  # requires a `/`, so a root-level match is rejected for an unrelated reason
  # and the fixture would prove nothing (this test passed for exactly that wrong
  # reason until mutation testing showed the guard could be deleted with no
  # failure).
  printf 'unrelated\n' > tests/unit/t
  at_row '- AT-01.1 [to-write] -- covers AC-01.1 -- status: to-write (red-first)'

  run run_intent at lint ST0001 --fix
  # The mechanical half still lands: the parenthetical note gets delimited.
  run grep -c -- '-- status: to-write -- red-first' "$ACC"
  assert_output "1"
  # The reference is untouched -- not resolved to the single-character file.
  run grep -c -- '\[to-write\]' "$ACC"
  assert_output "1"
  run grep -c 'tests/unit/t`' "$ACC"
  assert_output "0"
  # And it is still reported for a human, because it is genuinely owner work.
  run run_intent at lint ST0001
  assert_failure
  assert_output_contains "AT-01.1"
}

@test "a status from the wrong arm is refused before the write, not after it" {
  setup_contract
  # vc F1. Each arm has its own status vocabulary, and nothing enforced that: the
  # substitution landed, pushed the row out of the grammar, the strict reader
  # then read nothing back -- and the tool reported "the file was NOT updated"
  # about a file it had just corrupted. The refusal has to come BEFORE the write,
  # so the assertion that matters is that the contract is byte-identical after.
  at_row '- AT-01.1 `tests/unit/deck.bats` -- covers AC-01.1 -- status: red'
  at_row '- AT-02.1 (non-test) read the design doc -- covers AC-02.1 -- status: n/a'
  cp "$ACC" "$BATS_TEST_TMPDIR/before.md"

  run run_intent at na ST0001 AT-01.1
  assert_failure
  assert_output_contains "n/a is the non-test status"

  run run_intent at red ST0001 AT-02.1
  assert_failure
  assert_output_contains "n/a by definition"

  run run_intent at green ST0001 AT-02.1
  assert_failure
  assert_output_contains "n/a by definition"

  # Nothing was written, and nothing claimed otherwise.
  run diff "$BATS_TEST_TMPDIR/before.md" "$ACC"
  assert_success
  run run_intent at lint ST0001
  assert_success
}

@test "the write verifier reports what it observed, not what it did not check" {
  setup_contract
  # It verifies the RESULT, by design -- so it may not conclude anything about
  # the mechanism. Claiming "the file was NOT updated" was how a corrupted row
  # got denied into existence.
  #
  # Scoped to write-verification messages deliberately: the pre-write permission
  # checks make the same claim and are entitled to, because a write that was
  # refused before it began genuinely did not happen. Banning the phrase outright
  # would delete two true statements to remove one false one.
  run bash -c "grep 'write verification failed' '$INTENT_BIN_DIR/intent_acceptance' | grep -c 'the file was NOT updated'"
  assert_output "0"
  run bash -c "grep -c 'the recorded state is not what was asked for' '$INTENT_BIN_DIR/intent_acceptance'"
  assert_output "2"
}

@test "a row failing the grammar yields NO field rather than a plausible wrong one" {
  setup_contract
  # The whole shape of 0017: partial recovery is what made the failures silent,
  # because a half-parsed row still looked like a parsed row. Total failure is
  # the property that makes them visible.
  at_row '- AT-01.1 `md:absolute` -- covers AC-01.1 -- status: green'

  # The AC must NOT be counted as covered by a row the grammar rejects.
  run run_intent ac list ST0001
  assert_success
  echo "$output" | grep -E 'AC-01\.1 +covered-by: - +satisfied: no'
}

@test "the extraction seam cannot be reintroduced with a colliding sed delimiter" {
  # The AT grammar carries a literal / (the required path separator) and n/a in
  # its vocabulary, so the original s/.../.../ seam is a syntax error on it. The
  # delimiter is @, and no pattern may contain one.
  run grep -c 's@\$pattern@' "$INTENT_HOME/bin/intent_acceptance"
  assert_output "1"
  run bash -c "grep -E '^AT_G_|^AT_GRAMMAR_' '$INTENT_HOME/bin/intent_acceptance' | grep -c '@'"
  assert_output "0"

  # And the comment that sends a reader here names a file that exists -- this
  # one. It cited acceptance_grammar_guard.bats, which never existed (vc F3).
  # The same trick 0017 applies to AT rows: link by a token checkable from both
  # ends, so the pointer cannot rot in silence.
  run grep -c "$(basename "$BATS_TEST_FILENAME")" "$INTENT_HOME/bin/intent_acceptance"
  assert_output "1"
}
