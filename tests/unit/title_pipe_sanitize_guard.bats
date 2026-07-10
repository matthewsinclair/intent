#!/usr/bin/env bats
# Guard: a `|` in a title never reaches a stored title (ST0055 companion chore).
#
# The shared render_table splits rows on IFS='|', so a raw pipe in a title cell
# corrupts every markdown table it lands in (steel_threads.md, `wp list`,
# todo.md). sanitize_title() replaces `|` with `/` at the input boundary of
# every title-taking command (st new / wp new / issues add). This guard pins the
# helper and the two structural commands.

load ../lib/test_helper

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-pipe-test-XXXXXX)"
  PROJECT_DIR="$(create_test_project "Pipe Test" "$TEST_TEMP_DIR/proj")"
  cd "$PROJECT_DIR" || exit 1
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}

@test "helper: sanitize_title replaces every pipe with a slash" {
  run bash -c "source '${INTENT_BIN_DIR}/intent_helpers'; sanitize_title 'a | b | c'"
  assert_success
  assert_output "a / b / c"
}

@test "st new: a piped title stores no raw pipe in the ST heading" {
  run run_intent st new "Alpha | Beta"
  assert_success
  run bash -c "grep -m1 '^# ST0001:' intent/st/NOT-STARTED/ST0001/info.md"
  assert_success
  assert_output_contains "Alpha / Beta"
  refute_output_contains "Alpha | Beta"
}

@test "wp new: a piped title stores no raw pipe in the wp title" {
  run run_intent st new "Host thread"
  assert_success
  run run_intent wp new ST0001 "Gamma | Delta"
  assert_success
  run bash -c "grep -m1 '^title:' intent/st/NOT-STARTED/ST0001/WP/01/info.md"
  assert_success
  assert_output_contains "Gamma / Delta"
  refute_output_contains "Gamma | Delta"
}
