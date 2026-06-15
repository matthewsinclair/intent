#!/usr/bin/env bats
# Tests for the relocate_config ledger step in bin/intent_migrations (ST0036,
# moved out of bin/intent_helpers by ST0043). intent_relocate_dotintent performs
# the atomic .intent/ -> intent/.config/ move; step_relocate_config_needs is the
# state probe the orchestrator uses. The version stamp is the orchestrator's job
# (see tests/unit/intent_upgrade_orchestrator.bats), never this step's.

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-relocate-XXXXXX)"
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

# --- 1. Clean migration ----------------------------------------------------

@test "clean migration: .intent v2.9.0 -> intent/.config v2.9.0 (stamp untouched by relocate)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/.intent"
  cat > "$proj/.intent/config.json" <<JSON
{"intent_version":"2.9.0","project_name":"clean","author":"t"}
JSON

  run intent_relocate_dotintent "$proj"
  assert_success
  assert_output_contains "relocated: .intent/ -> intent/.config/"

  [ -d "$proj/intent/.config" ] || fail "intent/.config not present"
  [ ! -e "$proj/.intent" ] || fail ".intent should be removed"
  [ -f "$proj/intent/.config/config.json" ] || fail "config.json missing in new location"
  [ ! -f "$proj/intent/.config/.migration-in-progress" ] || fail "sentinel left behind"

  # The relocate step does NOT bump the stamp (the orchestrator owns the stamp);
  # the version should still read 2.9.0 here.
  local v
  v=$(jq -r '.intent_version' "$proj/intent/.config/config.json")
  [ "$v" = "2.9.0" ] || fail "expected stamp 2.9.0 (relocate-only), got '$v'"
}

# --- 2. Idempotent re-run --------------------------------------------------

@test "idempotent re-run: second invocation is a no-op" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/.intent"
  echo '{"intent_version":"2.9.0","project_name":"idem","author":"t"}' > "$proj/.intent/config.json"

  intent_relocate_dotintent "$proj" >/dev/null 2>&1 || fail "first relocate failed"

  local before_sum
  before_sum=$(cd "$proj" && find . -type f | sort | xargs shasum 2>/dev/null | shasum | awk '{print $1}')

  run intent_relocate_dotintent "$proj"
  assert_success
  assert_output_contains "intent/.config already in place"

  local after_sum
  after_sum=$(cd "$proj" && find . -type f | sort | xargs shasum 2>/dev/null | shasum | awk '{print $1}')
  [ "$before_sum" = "$after_sum" ] || fail "filesystem mutated on idempotent re-run"
}

# --- 3. Sentinel recovery refusal -----------------------------------------

@test "sentinel recovery: existing migration-in-progress refuses, no mutation" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  touch "$proj/intent/.config/.migration-in-progress"
  echo '{"x":1}' > "$proj/intent/.config/config.json"

  local before_sum
  before_sum=$(cd "$proj" && find . -type f | sort | xargs shasum 2>/dev/null | shasum | awk '{print $1}')

  run intent_relocate_dotintent "$proj"
  [ "$status" -ne 0 ] || fail "expected non-zero exit; got success"
  assert_output_contains "interrupted migration"
  assert_output_contains "migration-v2.10.0.md#recovery-from-interrupted-migration"

  local after_sum
  after_sum=$(cd "$proj" && find . -type f | sort | xargs shasum 2>/dev/null | shasum | awk '{print $1}')
  [ "$before_sum" = "$after_sum" ] || fail "filesystem mutated despite recovery refusal"
}

# --- 4. Symlink refusal ----------------------------------------------------

@test "symlink refusal: .intent is a symlink, no relocation, target unchanged" {
  local proj="$TEST_TEMP_DIR/proj"
  local elsewhere="$TEST_TEMP_DIR/elsewhere"
  mkdir -p "$proj" "$elsewhere"
  echo '{"intent_version":"2.9.0"}' > "$elsewhere/config.json"
  ln -s "$elsewhere" "$proj/.intent"

  local before_target
  before_target=$(readlink "$proj/.intent")
  local before_content
  before_content=$(cat "$elsewhere/config.json")

  run intent_relocate_dotintent "$proj"
  [ "$status" -ne 0 ] || fail "expected non-zero exit; got success"
  assert_output_contains "symbolic link"

  [ -L "$proj/.intent" ] || fail "symlink removed despite refusal"
  [ "$(readlink "$proj/.intent")" = "$before_target" ] || fail "symlink target changed"
  [ "$(cat "$elsewhere/config.json")" = "$before_content" ] || fail "target content modified"
}

# --- 5. Conflict refusal ---------------------------------------------------

@test "conflict refusal: both .intent/ and intent/.config/ exist, no mutation" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/.intent" "$proj/intent/.config"
  echo '{"old":1}' > "$proj/.intent/config.json"
  echo '{"new":1}' > "$proj/intent/.config/config.json"

  local before_sum
  before_sum=$(cd "$proj" && find . -type f | sort | xargs shasum 2>/dev/null | shasum | awk '{print $1}')

  run intent_relocate_dotintent "$proj"
  [ "$status" -ne 0 ] || fail "expected non-zero exit; got success"
  assert_output_contains "both .intent/ and intent/.config/ exist"

  local after_sum
  after_sum=$(cd "$proj" && find . -type f | sort | xargs shasum 2>/dev/null | shasum | awk '{print $1}')
  [ "$before_sum" = "$after_sum" ] || fail "filesystem mutated despite conflict refusal"
}

# --- 6. step_relocate_config_needs is layout-aware ------------------------

@test "step_relocate_config_needs: .intent/ present and no intent/.config/ -> needs (0)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/.intent"
  echo '{"intent_version":"2.10.0"}' > "$proj/.intent/config.json"

  # Stamp may be at target, but the pre-relocation layout means relocate must run.
  step_relocate_config_needs "$proj"
  [ "$?" -eq 0 ] || fail "expected relocate needed when layout is .intent/"
}

@test "step_relocate_config_needs: already on intent/.config/ -> not needed (1)" {
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$proj/intent/.config"
  echo '{"intent_version":"2.10.0"}' > "$proj/intent/.config/config.json"

  if step_relocate_config_needs "$proj"; then
    fail "expected relocate not needed when already on intent/.config/"
  fi
}
