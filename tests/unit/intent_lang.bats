#!/usr/bin/env bats
# Tests for `intent lang` command (ST0035/WP-19).

load '../lib/test_helper'

# Set up an Intent project with the agnostic _default RULES.md installed
# (this is what canon-installer would have produced).
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-lang-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  PROJECT_DIR="${TEST_TEMP_DIR}/proj"
  create_test_project "Lang Test" "$PROJECT_DIR" >/dev/null
  cp "${INTENT_HOME}/intent/plugins/agents/templates/_default/RULES.md" "${PROJECT_DIR}/intent/llm/RULES.md"
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "intent lang help displays usage" {
  run "${INTENT_BIN_DIR}/intent" lang help
  assert_success
  assert_output_contains "Usage: intent lang"
  assert_output_contains "list"
  assert_output_contains "show"
  assert_output_contains "init"
}

@test "intent lang (no subcommand) shows usage" {
  run "${INTENT_BIN_DIR}/intent" lang
  assert_success
  assert_output_contains "Usage: intent lang"
}

@test "intent lang list enumerates all five canon languages" {
  run "${INTENT_BIN_DIR}/intent" lang list
  assert_success
  assert_output_contains "elixir"
  assert_output_contains "rust"
  assert_output_contains "swift"
  assert_output_contains "lua"
  assert_output_contains "shell"
  refute_output_contains "_default"
}

@test "intent lang show <lang> describes installation targets" {
  run "${INTENT_BIN_DIR}/intent" lang show elixir
  assert_success
  assert_output_contains "Language pack: elixir"
  assert_output_contains "intent/llm/RULES-elixir.md"
  assert_output_contains "intent/llm/ARCHITECTURE-elixir.md"
}

@test "intent lang show on unknown language errors with available list" {
  run "${INTENT_BIN_DIR}/intent" lang show bogus-language
  assert_failure
  assert_output_contains "no template for 'bogus-language'"
  assert_output_contains "available:"
}

@test "intent lang init (no args) errors" {
  cd "$PROJECT_DIR"
  run "${INTENT_BIN_DIR}/intent" lang init
  assert_failure
  assert_output_contains "missing language argument"
}

@test "intent lang init <lang> installs RULES + ARCHITECTURE files" {
  cd "$PROJECT_DIR"
  run "${INTENT_BIN_DIR}/intent" lang init elixir
  assert_success
  assert_file_exists "$PROJECT_DIR/intent/llm/RULES-elixir.md"
  assert_file_exists "$PROJECT_DIR/intent/llm/ARCHITECTURE-elixir.md"
  assert_output_contains "installed: intent/llm/RULES-elixir.md"
  assert_output_contains "installed: intent/llm/ARCHITECTURE-elixir.md"
}

@test "intent lang init appends Language Packs entry to agnostic RULES.md" {
  cd "$PROJECT_DIR"
  run "${INTENT_BIN_DIR}/intent" lang init rust
  assert_success
  assert_file_contains "$PROJECT_DIR/intent/llm/RULES.md" "**rust** -- rule pack at"
}

@test "intent lang init is idempotent (zero diff on re-run)" {
  cd "$PROJECT_DIR"
  run "${INTENT_BIN_DIR}/intent" lang init shell
  assert_success
  local checksum_before
  # Use shasum for portability (md5 differs between BSD and GNU coreutils).
  checksum_before="$(find "$PROJECT_DIR/intent/llm" -type f -exec shasum {} \; | sort | shasum)"

  run "${INTENT_BIN_DIR}/intent" lang init shell
  assert_success
  local checksum_after
  checksum_after="$(find "$PROJECT_DIR/intent/llm" -type f -exec shasum {} \; | sort | shasum)"

  [ "$checksum_before" = "$checksum_after" ]
}

@test "intent lang init multi-lang installs each in order" {
  cd "$PROJECT_DIR"
  run "${INTENT_BIN_DIR}/intent" lang init rust shell lua
  assert_success
  assert_file_exists "$PROJECT_DIR/intent/llm/RULES-rust.md"
  assert_file_exists "$PROJECT_DIR/intent/llm/RULES-shell.md"
  assert_file_exists "$PROJECT_DIR/intent/llm/RULES-lua.md"
  assert_file_contains "$PROJECT_DIR/intent/llm/RULES.md" "**rust**"
  assert_file_contains "$PROJECT_DIR/intent/llm/RULES.md" "**shell**"
  assert_file_contains "$PROJECT_DIR/intent/llm/RULES.md" "**lua**"
}

@test "intent lang init unknown language errors but does not abort other languages" {
  cd "$PROJECT_DIR"
  run "${INTENT_BIN_DIR}/intent" lang init bogus elixir
  assert_failure
  assert_output_contains "no template for 'bogus'"
  assert_file_exists "$PROJECT_DIR/intent/llm/RULES-elixir.md"
  assert_output_contains "1 error"
}

@test "intent lang init outside an Intent project errors cleanly" {
  cd "$TEST_TEMP_DIR"
  run "${INTENT_BIN_DIR}/intent" lang init elixir
  assert_failure
  assert_output_contains "no intent/llm/ directory"
}
