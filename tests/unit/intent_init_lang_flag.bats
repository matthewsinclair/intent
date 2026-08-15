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
  run "$INTENT_BIN" init --help
  # v2 exits non-zero here. That is NOT a convention -- it is a RATIFIED
  # DEVIATION (parity.md, `Corrected` class, hv 2026-08-14; INV-07): asking for
  # help and being told you failed is a defect, and v3 exits 0. This assertion
  # is correct as it stands because it asserts the INCUMBENT, which is what
  # INTENT_BIN defaults to. Under the conformance runner it fails against v3,
  # and that is the harness working: expect red, and expect it for ratified
  # reasons. Do not flip it -- that reds v2's own suite and deletes the evidence
  # that the deviation exists.
  assert_failure
  assert_output_contains "--lang"
}

@test "intent init --lang requires an argument" {
  run bash -c "'$INTENT_BIN' init proj/ --lang </dev/null"
  assert_failure
  assert_output_contains "--lang requires an argument"
}

@test "intent init --lang elixir installs single language" {
  run bash -c "'$INTENT_BIN' init proj/ --lang elixir </dev/null"
  assert_success
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/ARCHITECTURE-elixir.md"
}

@test "intent init --lang elixir,rust,shell installs all three" {
  run bash -c "'$INTENT_BIN' init proj/ --lang elixir,rust,shell </dev/null"
  assert_success
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-rust.md"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-shell.md"
  assert_file_contains "$TEST_TEMP_DIR/proj/intent/llm/RULES.md" "**elixir**"
  assert_file_contains "$TEST_TEMP_DIR/proj/intent/llm/RULES.md" "**rust**"
  assert_file_contains "$TEST_TEMP_DIR/proj/intent/llm/RULES.md" "**shell**"
}

@test "intent init --lang=elixir (equals form) also works" {
  run bash -c "'$INTENT_BIN' init proj/ --lang=elixir </dev/null"
  assert_success
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
}

@test "intent init with no --lang produces agnostic-canon-only project" {
  run bash -c "'$INTENT_BIN' init proj/ </dev/null"
  assert_success
  # Project initialised with agnostic _default RULES.md but no language-specific ones.
  assert_directory_exists "$TEST_TEMP_DIR/proj/intent/llm"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES.md"
  refute_output_contains "Installing per-language canon"
}

@test "intent init --lang bogus,elixir installs elixir, errors on bogus" {
  run bash -c "'$INTENT_BIN' init proj/ --lang bogus,elixir </dev/null"
  # init succeeds overall (lang init failures are non-fatal per || true);
  # elixir is still installed; bogus produces an error in the output.
  assert_success
  assert_output_contains "no template for 'bogus'"
  assert_file_exists "$TEST_TEMP_DIR/proj/intent/llm/RULES-elixir.md"
}
