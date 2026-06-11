#!/usr/bin/env bats
# Behavioural coverage for intent claude prime (ST0042 T10/F-TEST-3: the
# module had no test). prime writes to ~/.claude/projects/<dir>/memory/ --
# HOME is sandboxed so tests never touch the real memory files.

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  setup_fake_home
}

teardown() {
  teardown_fake_home
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "claude prime --dry-run previews MEMORY.md without writing" {
  local project
  project="$(create_test_project "PrimeDry")"
  cd "$project"

  run run_intent claude prime --dry-run
  assert_success
  assert_output_contains "DRY RUN"
  assert_output_contains "MEMORY.md"
  # Nothing written under the sandboxed HOME.
  local written
  written=$(find "$HOME/.claude/projects" -name 'MEMORY.md' 2>/dev/null | head -1)
  [ -z "$written" ] || fail "dry-run wrote $written"
}

@test "claude prime writes MEMORY.md under HOME/.claude/projects" {
  local project
  project="$(create_test_project "PrimeWrite")"
  cd "$project"

  run run_intent claude prime
  assert_success

  local written
  written=$(find "$HOME/.claude/projects" -name 'MEMORY.md' 2>/dev/null | head -1)
  [ -n "$written" ] || fail "prime wrote no MEMORY.md under sandboxed HOME"
  grep -q 'Intent' "$written" || fail "MEMORY.md content missing Intent context"
}
