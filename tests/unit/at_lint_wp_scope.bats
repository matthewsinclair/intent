#!/usr/bin/env bats
# Issue 0024: `at lint` and `ac gate` accept a `<ID>/NN` work-package scope and
# silently dropped it.
#
# The defect is not that the scope was rejected -- it was ACCEPTED, documented
# in `at lint`'s own usage, and then ignored. Every AT row in the thread was
# linted, counted and reported under a question naming one work package, so a
# finished WP read as blocked by rows it does not own, and `--fix` under a scope
# rewrote rows the scope excluded. An instrument that takes a narrowing argument
# and answers the wider question reads exactly like a correct answer, which is
# why this went unseen in a 1639-row consumer estate until a WP that was never
# blocked could not close.
#
# The shape of these guards matters. The fixable row lives in the OUT-of-scope
# work package, because a fixture whose fixable row sits in scope passes with or
# without the filter -- it would certify the status quo. Mutation-proven: with
# the `in_wp_filter` calls removed from `at_lint_report` / `at_lint_fix`, the
# scoped-count and out-of-scope-rewrite tests fail; restored, all pass.

load "../lib/test_helper.bash"

# ST0001 with two work packages. WP-01 carries a row `--fix` CAN migrate (an
# unbackticked path); WP-02 carries a row that already conforms. So any rewrite
# of WP-01's row while WP-02 is the named scope is an out-of-scope mutation.
setup_two_wp_contract() {
  project_dir=$(create_test_project "AT Lint WP Scope")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Scope Thread"; assert_success
  run run_intent wp new ST0001 "first"; assert_success
  run run_intent wp new ST0001 "second"; assert_success

  ACC="intent/st/NOT-STARTED/ST0001/acceptance.md"
  mkdir -p tests/unit
  printf 'AT-01.1 AT-02.1 live here\n' > tests/unit/deck.bats
  cat > "$ACC" <<'EOF'
---
st_id: ST0001
---
# ST0001 -- Acceptance

## Acceptance Criteria

- AC-01.1 a criterion in WP-01
- AC-02.1 a criterion in WP-02

## Acceptance Tests

- AT-01.1 tests/unit/deck.bats -- covers AC-01.1 -- status: green
- AT-02.1 `tests/unit/deck.bats` -- covers AC-02.1 -- status: green
EOF
}

@test "at lint <ID>/NN counts only that work package's rows and names the scope" {
  setup_two_wp_contract

  # Before the fix both of these answered "ST0001 ... 2 AT row(s)".
  run run_intent at lint ST0001/02
  assert_output_contains "ST0001/02"
  assert_output_contains "1 AT row(s)"

  run run_intent at lint ST0001/01
  assert_output_contains "ST0001/01"
  assert_output_contains "1 AT row(s)"
}

@test "at lint on the whole thread still sees every row" {
  setup_two_wp_contract

  # The regression direction: a filter that over-narrows would break the
  # thread-scoped path, which is the one every existing caller uses.
  run run_intent at lint ST0001
  assert_output_contains "2 AT row(s)"
  echo "$output" | grep -qv "ST0001/" || false
}

@test "at lint <ID>/NN --fix does not rewrite a row outside the scope" {
  setup_two_wp_contract
  cp "$ACC" before.md

  # WP-02's row already conforms, so a correctly-scoped fix has nothing to do.
  # Before the fix this rewrote AT-01.1 -- a work package the user never named.
  run run_intent at lint ST0001/02 --fix
  assert_output_contains "0 AT row(s) rewritten"

  run diff before.md "$ACC"
  assert_success
}

@test "at lint <ID>/NN --fix still migrates rows inside the scope" {
  setup_two_wp_contract

  run run_intent at lint ST0001/01 --fix
  assert_output_contains "1 AT row(s) rewritten"
  # No leading `- ` in the needle: grep reads it as an option flag.
  assert_file_contains "$ACC" 'AT-01.1 `tests/unit/deck.bats`'
}

@test "ac gate <ID>/NN does not report findings from another work package" {
  setup_two_wp_contract
  # Make WP-01's row unfixable-and-bad so the lint arm fires on WP-01 only.
  # WP-02 is clean, so its gate verdict must not carry WP-01's finding.
  printf -- '- AT-01.2 `no/such/file.bats` -- covers AC-01.1 -- status: green\n' >> "$ACC"

  run run_intent ac gate ST0001/01
  assert_failure
  assert_output_contains "AT contract finding"

  # Before the fix this returned WP-01's findings under WP-02's name. Asserted
  # as a bare negated test, NOT `grep -q ... && false || true` -- that form's
  # trailing `|| true` swallows the failure, so it can never go red. It was
  # written that way first and survived a mutation that should have killed it.
  run run_intent ac gate ST0001/02
  [[ "$output" != *"AT contract finding"* ]]
}
