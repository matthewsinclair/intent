#!/usr/bin/env bats
# Tests for the languages_field ledger step in bin/intent_migrations (ST0037,
# moved out of bin/intent_helpers by ST0043). step_languages_field_run does the
# back-fill of the languages[] config field; step_languages_field_needs is the
# state probe. The step does NOT stamp the version -- the orchestrator owns the
# stamp (see tests/unit/intent_upgrade_orchestrator.bats).

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-langs-XXXXXX)"
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/bin/intent_helpers"
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/bin/intent_migrations"
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# --- back-fill: empty (no rules, no hook) ---------------------------------

@test "step_languages_field_run: no RULES-*.md, no pre-commit hook -> languages: []" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.1","project_name":"clean","author":"t"}' > "$proj/intent/.config/config.json"

  run step_languages_field_run "$proj"
  assert_success
  assert_output_contains "back-filled: languages = []"

  local langs
  langs=$(jq -r '.languages | length' "$proj/intent/.config/config.json")
  [ "$langs" = "0" ] || fail "expected empty languages, got length $langs"
}

# --- back-fill: pre-commit hook present -> ["shell"] ----------------------

@test "step_languages_field_run: pre-commit hook present -> languages: [\"shell\"]" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config" "$proj/.git/hooks"
  echo '{"intent_version":"2.10.1","project_name":"shellfb","author":"t"}' > "$proj/intent/.config/config.json"
  touch "$proj/.git/hooks/pre-commit"

  run step_languages_field_run "$proj"
  assert_success
  assert_output_contains "back-filled: languages = [\"shell\"]"

  local langs
  langs=$(jq -r '.languages | join(",")' "$proj/intent/.config/config.json")
  [ "$langs" = "shell" ] || fail "expected [shell], got [$langs]"
}

# --- back-fill: RULES-*.md presence drives the set (alphabetical) ---------

@test "step_languages_field_run: back-fill from RULES-*.md presence (alphabetical)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config" "$proj/intent/llm"
  echo '{"intent_version":"2.10.1","project_name":"poly","author":"t"}' > "$proj/intent/.config/config.json"
  touch "$proj/intent/llm/RULES-shell.md"
  touch "$proj/intent/llm/RULES-elixir.md"
  touch "$proj/intent/llm/RULES-rust.md"

  run step_languages_field_run "$proj"
  assert_success

  local langs
  langs=$(jq -r '.languages | join(",")' "$proj/intent/.config/config.json")
  [ "$langs" = "elixir,rust,shell" ] || fail "expected alphabetical [elixir,rust,shell], got [$langs]"
}

# --- probe: needs is true iff languages absent ----------------------------

@test "step_languages_field_needs: absent -> needs (0); present -> not needed (1)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.1"}' > "$proj/intent/.config/config.json"
  step_languages_field_needs "$proj" || fail "expected needs=0 when languages absent"

  echo '{"intent_version":"2.11.0","languages":["shell"]}' > "$proj/intent/.config/config.json"
  if step_languages_field_needs "$proj"; then
    fail "expected needs=1 when languages already present"
  fi
}

# --- probe: missing config is not "needed" (orchestrator skips) -----------

@test "step_languages_field_needs: missing config -> not needed (1)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent"  # but no .config/
  if step_languages_field_needs "$proj"; then
    fail "expected needs=1 (skip) when config is missing"
  fi
}

# --- run guards a missing config (warns, returns 0) -----------------------

@test "step_languages_field_run: missing config -> warning and skip" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent"  # but no .config/

  run step_languages_field_run "$proj"
  assert_success
  assert_output_contains "warning:"
  assert_output_contains "not found"
}

# --- verify: postcondition holds only after back-fill ---------------------

@test "step_languages_field_verify: false before back-fill, true after" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.1"}' > "$proj/intent/.config/config.json"

  if step_languages_field_verify "$proj"; then
    fail "verify should be false before the languages field exists"
  fi
  step_languages_field_run "$proj" >/dev/null 2>&1
  step_languages_field_verify "$proj" || fail "verify should be true after back-fill"
}
