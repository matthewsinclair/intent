#!/usr/bin/env bats
# Tests for `intent init --lang <list>` flag (ST0035/WP-19).

load '../lib/test_helper'

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-init-lang-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# intent init prompts for Claude agent install when stdin/stdout are TTY; redirect
# stdin from /dev/null in tests to skip the interactive prompt.

@test "intent init --help mentions --lang" {
  run "${INTENT_BIN_DIR}/intent" init --help
  assert_failure  # init --help exits non-zero (usage convention)
  assert_output_contains "--lang"
}

@test "intent init --lang requires an argument" {
  run bash -c "'${INTENT_BIN_DIR}/intent' init proj/ --lang </dev/null"
  assert_failure
  assert_output_contains "--lang requires an argument"
}

@test "intent init --lang elixir installs single language" {
  run bash -c "'${INTENT_BIN_DIR}/intent' init proj/ --lang elixir </dev/null"
  assert_success
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/ARCHITECTURE-elixir.md"
}

@test "intent init --lang elixir,rust,shell installs all three" {
  run bash -c "'${INTENT_BIN_DIR}/intent' init proj/ --lang elixir,rust,shell </dev/null"
  assert_success
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-rust.md"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-shell.md"
  assert_file_contains "$TEST_TEMP_DIR/proj/intent/llm/RULES.md" "**elixir**"
  assert_file_contains "$TEST_TEMP_DIR/proj/intent/llm/RULES.md" "**rust**"
  assert_file_contains "$TEST_TEMP_DIR/proj/intent/llm/RULES.md" "**shell**"
}

@test "intent init --lang=elixir (equals form) also works" {
  run bash -c "'${INTENT_BIN_DIR}/intent' init proj/ --lang=elixir </dev/null"
  assert_success
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
}

@test "intent init with no --lang produces agnostic-canon-only project" {
  run bash -c "'${INTENT_BIN_DIR}/intent' init proj/ </dev/null"
  assert_success
  # Project initialised with agnostic _default RULES.md but no language-specific ones.
  assert_directory_exists "$TEST_TEMP_DIR/proj/intent/llm"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES.md"
  refute_output_contains "Installing per-language canon"
}

@test "intent init --lang bogus,elixir installs elixir, errors on bogus" {
  run bash -c "'${INTENT_BIN_DIR}/intent' init proj/ --lang bogus,elixir </dev/null"
  # init succeeds overall (lang init failures are non-fatal per || true);
  # elixir is still installed; bogus produces an error in the output.
  assert_success
  assert_output_contains "no template for 'bogus'"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
}
