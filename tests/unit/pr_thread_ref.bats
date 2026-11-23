#!/usr/bin/env bats
# .github/scripts/pr-thread-ref.sh -- the pull-request check's thread reference
# asks the binary's id door, not a pattern (ST0095 WP-12, AC-12.1).
#
# Each case runs in a throwaway project under a sandboxed HOME, made by the
# binary under test, so the thread it names is one that build minted, in
# whichever id family it mints. The base carried this step inline in
# pr-checks.yml, so no arm here has a red on its base.

load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/.github/scripts/pr-thread-ref.sh"

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/pr-thread-ref-XXXXXX")" && pwd)"
  export HOME="$TEST_TEMP_DIR/home"
  mkdir -p "$HOME" "$TEST_TEMP_DIR/proj"
  unset_user_dirs
  cd "$TEST_TEMP_DIR/proj" || exit 1
  "$INTENT_BIN" init "PR ref" >/dev/null 2>&1 || fail "intent init did not make the fixture project"
  THREAD="$(mint st new "Release checklist")" || fail "st new minted no thread"
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# The script promises bash 3.2, so it runs under /bin/bash where that is the
# system bash, and under the bash on PATH elsewhere.
check() {
  local sh=bash
  [ -x /bin/bash ] && sh=/bin/bash
  run env PR_BODY="$1" INTENT_BIN="${2:-$INTENT_BIN}" "$sh" "$SCRIPT"
}

# stand_in <name> <body> -- a binary at $TEST_TEMP_DIR/<name>/intent running <body>.
stand_in() {
  mkdir -p "$TEST_TEMP_DIR/$1"
  printf "#!/bin/bash\n%s\n" "$2" >"$TEST_TEMP_DIR/$1/intent"
  chmod +x "$TEST_TEMP_DIR/$1/intent"
  printf "%s" "$TEST_TEMP_DIR/$1/intent"
}

@test "a thread the body names, punctuation and all, is found by the door, whose verdict names its record" {
  check "Fixes the checklist (${THREAD}). Size S, 2 files."
  assert_success
  assert_output_contains "Referenced thread: ${THREAD}"
  assert_output_contains "Steel thread ${THREAD} exists"
}

@test "a thread-shaped word the project does not hold fails the check" {
  check "Follows ST9999 and the plan."
  assert_failure
  assert_output_contains "Steel thread ST9999 not found"
}

@test "a body that names no thread passes with a notice, and a bare number is not a reference" {
  check "Tidies the docs for the 2026 release."
  assert_success
  assert_output_contains "No steel thread reference found"
  refute_output_contains "Referenced thread"
}

@test "a work package of a thread the project does not hold fails the check" {
  check "Lands ST9999/01 at last."
  assert_failure
  assert_output_contains "Steel thread ST9999/01 not found"
}

@test "a binary without the id door fails the check loudly and never falls back to a pattern" {
  # A build from before ST0095 WP-10 refuses the unknown subcommand and exits 1,
  # as a word that is not an id does, so only its lines can tell them apart.
  local old
  old="$(stand_in old-bin "echo \"error: unrecognized subcommand 'id'\" >&2; exit 1")"
  check "Fixes the checklist (${THREAD})." "$old"
  [ "$status" -eq 2 ] || fail "expected exit 2 (the door did not answer), got $status: $output"
  assert_output_contains "did not answer \`intent id\`"
  assert_output_contains "it said: error: unrecognized subcommand"
  assert_output_contains "remedy:"
  refute_output_contains "Referenced thread"
}

@test "a door that answers nothing fails the check rather than reading as no reference" {
  local silent
  silent="$(stand_in silent-bin "exit 0")"
  check "Fixes the checklist (${THREAD})." "$silent"
  [ "$status" -eq 2 ] || fail "expected exit 2 (no verdict for the words asked), got $status: $output"
  assert_output_contains "verdict(s) for the"
  refute_output_contains "No steel thread reference found"
}
