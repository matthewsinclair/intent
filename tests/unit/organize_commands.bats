#!/usr/bin/env bats
# Behavioural coverage for bin/intent_organize (ST0042 T10/F-TEST-3: the
# module had no test -- and adding coverage exposed that `intent organize`
# did not dispatch at all: help text and `intent st organize` both use the
# "organize" spelling but the script was named intent_organise, so the
# default `intent_$COMMAND` dispatch arm missed it).

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Seeds a Completed thread at intent/st/ST0001 inside $1.
seed_completed_thread() {
  local project="$1"
  mkdir -p "$project/intent/st/ST0001"
  cat > "$project/intent/st/ST0001/info.md" << 'EOF'
---
intent_version: 2.0.0
status: Completed
created: 20250117
---
# ST0001: Finished Thread

- **Status**: Completed
EOF
}

@test "intent organize dispatches (the documented spelling resolves a script)" {
  # Regression guard: help text and the st-subcommand both say "organize",
  # but the script was intent_organise -- so the top-level command errored
  # with "Unknown command 'organize'". Requires project context.
  local project
  project="$(create_test_project "Organize Dispatch Test")"
  cd "$project"
  run run_intent organize --help
  assert_success
  assert_output_contains "Usage: intent organize"
}

@test "organize --dry-run previews moves without making them" {
  local project
  project="$(create_test_project "Organize Dry Test")"
  seed_completed_thread "$project"
  cd "$project"

  run run_intent organize --dry-run
  assert_success
  assert_output_contains "[dry run]"
  # Nothing moved.
  [ -d "$project/intent/st/ST0001" ] || fail "dry-run moved the thread"
  [ ! -d "$project/intent/st/COMPLETED/ST0001" ] || fail "dry-run created COMPLETED copy"
}

@test "organize moves a Completed thread into COMPLETED/" {
  local project
  project="$(create_test_project "Organize Move Test")"
  seed_completed_thread "$project"
  cd "$project"

  run run_intent organize
  assert_success
  [ -d "$project/intent/st/COMPLETED/ST0001" ] || fail "thread not moved to COMPLETED/"
  [ ! -d "$project/intent/st/ST0001" ] || fail "thread still at root after organize"
}

@test "organize errors outside an Intent project" {
  mkdir bare && cd bare
  run run_intent organize
  assert_failure
}
