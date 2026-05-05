#!/usr/bin/env bats
# Tests for migrate_v2_10_x_to_v2_11_0 in bin/intent_helpers (ST0037).
#
# Each scenario constructs a synthetic v2.10.x project layout under
# TEST_TEMP_DIR, sources the migration helpers, invokes the migration
# directly, and asserts on the resulting config.json.

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-mig-v211-XXXXXX)"
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/bin/intent_helpers"
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# --- 1. Clean migration: empty back-fill (no rules, no hook) ---------------

@test "empty back-fill: no RULES-*.md, no pre-commit hook -> languages: []" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.1","project_name":"clean","author":"t"}' > "$proj/intent/.config/config.json"

  run migrate_v2_10_x_to_v2_11_0 "$proj"
  assert_success
  assert_output_contains "back-filled: languages = []"
  # Stamp is the live target (read from VERSION). v2.11.5 changed the
  # migration to drop the hard-coded "2.11.0" literal.
  local target
  target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  assert_output_contains "stamped: intent_version = $target"

  local langs
  langs=$(jq -r '.languages | length' "$proj/intent/.config/config.json")
  [ "$langs" = "0" ] || fail "expected empty languages, got length $langs"

  local v
  v=$(jq -r '.intent_version' "$proj/intent/.config/config.json")
  [ "$v" = "$target" ] || fail "expected stamp $target, got '$v'"
}

# --- 2. Empty back-fill but pre-commit hook present -> ["shell"] ----------

@test "empty back-fill + pre-commit hook present -> languages: [\"shell\"]" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config" "$proj/.git/hooks"
  echo '{"intent_version":"2.10.1","project_name":"shellfb","author":"t"}' > "$proj/intent/.config/config.json"
  touch "$proj/.git/hooks/pre-commit"

  run migrate_v2_10_x_to_v2_11_0 "$proj"
  assert_success
  assert_output_contains "back-filled: languages = [\"shell\"]"

  local langs
  langs=$(jq -r '.languages | join(",")' "$proj/intent/.config/config.json")
  [ "$langs" = "shell" ] || fail "expected [shell], got [$langs]"
}

# --- 3. RULES-*.md presence drives back-fill ------------------------------

@test "back-fill from RULES-*.md presence (alphabetical for determinism)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config" "$proj/intent/llm"
  echo '{"intent_version":"2.10.1","project_name":"poly","author":"t"}' > "$proj/intent/.config/config.json"
  touch "$proj/intent/llm/RULES-shell.md"
  touch "$proj/intent/llm/RULES-elixir.md"
  touch "$proj/intent/llm/RULES-rust.md"

  run migrate_v2_10_x_to_v2_11_0 "$proj"
  assert_success

  local langs
  langs=$(jq -r '.languages | join(",")' "$proj/intent/.config/config.json")
  [ "$langs" = "elixir,rust,shell" ] || fail "expected alphabetical [elixir,rust,shell], got [$langs]"
}

# --- 4. Idempotence: re-run is a no-op for the field ----------------------

@test "idempotent: languages already present -> no back-fill, only stamp" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.1","project_name":"id","author":"t","languages":["lua"]}' > "$proj/intent/.config/config.json"

  run migrate_v2_10_x_to_v2_11_0 "$proj"
  assert_success
  assert_output_contains "field: languages already present (no back-fill)"
  local target
  target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  assert_output_contains "stamped: intent_version = $target"

  local langs
  langs=$(jq -r '.languages | join(",")' "$proj/intent/.config/config.json")
  [ "$langs" = "lua" ] || fail "expected preserved [lua], got [$langs]"
}

# --- 5. Idempotence: full no-op on second call ----------------------------

@test "second invocation is a complete no-op (already stamped at live target)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.1","project_name":"idem","author":"t"}' > "$proj/intent/.config/config.json"

  migrate_v2_10_x_to_v2_11_0 "$proj" >/dev/null 2>&1
  local before_sum
  before_sum=$(shasum "$proj/intent/.config/config.json" | awk '{print $1}')

  run migrate_v2_10_x_to_v2_11_0 "$proj"
  assert_success
  local target
  target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  assert_output_contains "stamp: already at v$target"

  local after_sum
  after_sum=$(shasum "$proj/intent/.config/config.json" | awk '{print $1}')
  [ "$before_sum" = "$after_sum" ] || fail "config mutated on second call"
}

# --- 6. Missing config: warns and returns 0 (fail-open) -------------------

@test "missing config -> warning and skip" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent"  # but no .config/

  run migrate_v2_10_x_to_v2_11_0 "$proj"
  assert_success
  assert_output_contains "warning:"
  assert_output_contains "not found"
}

# --- 7. needs_v2_11_0_upgrade gate ----------------------------------------

@test "needs_v2_11_0_upgrade returns 0 when languages field absent" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.1"}' > "$proj/intent/.config/config.json"

  run needs_v2_11_0_upgrade "2.10.1" "$proj"
  assert_success  # exit 0 = needs upgrade
}

@test "needs_v2_11_0_upgrade returns 1 when languages field present" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.11.0","languages":["shell"]}' > "$proj/intent/.config/config.json"

  run needs_v2_11_0_upgrade "2.11.0" "$proj"
  assert_failure  # exit 1 = no upgrade needed
}

@test "needs_v2_11_0_upgrade falls back to version compare when no config" {
  run needs_v2_11_0_upgrade "2.10.1" "$TEST_TEMP_DIR/no-such-proj"
  assert_success  # 2.10.x needs upgrade

  run needs_v2_11_0_upgrade "2.11.0" "$TEST_TEMP_DIR/no-such-proj"
  assert_failure  # 2.11.x does not
}
