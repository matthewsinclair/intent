#!/usr/bin/env bats
# Behavioural coverage for bin/intent_llm (ST0042 T10/F-TEST-3: the module
# had no test). Covers display, symlink creation, and error paths.

load "../lib/test_helper.bash"

# `llm` requires project context (not a GLOBAL_COMMAND) -- run inside a
# fixture project.
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  PROJECT_DIR="$(create_test_project "LLM Cmd Test")"
  cd "$PROJECT_DIR"
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "llm with no arguments shows usage" {
  run run_intent llm
  assert_success
  assert_output_contains "Usage: intent llm <subcommand>"
  assert_output_contains "usage_rules"
}

@test "llm usage_rules displays the root canon usage-rules.md" {
  # Regression guard: the command read the retired intent/llm/usage-rules.md
  # location and errored on every invocation since the v2.10.0 root-canon
  # move (found adding this coverage, ST0042 T10).
  run run_intent llm usage_rules
  assert_success
  assert_output_contains "Usage Rules"
  refute_output_contains "Usage rules file not found"
}

@test "llm usage_rules --symlink creates a symlink in the target directory" {
  mkdir target
  run run_intent llm usage_rules --symlink target
  assert_success
  assert_output_contains "created: target/usage-rules.md"
  [ -L "target/usage-rules.md" ] || fail "no symlink created"
  [ -f "target/usage-rules.md" ] || fail "symlink does not resolve"
}

@test "llm usage_rules --symlink errors on a missing target directory" {
  run run_intent llm usage_rules --symlink ./does-not-exist
  assert_failure
  assert_output_contains "Target directory does not exist"
}

@test "llm errors on an unknown subcommand" {
  run run_intent llm bogus
  assert_failure
  assert_output_contains "Unknown subcommand: bogus"
}
