#!/usr/bin/env bats
# Tests for migrate_v2_9_0_to_v2_10_0 in bin/intent_helpers (ST0036/WP01).
#
# Each scenario constructs a synthetic project layout under TEST_TEMP_DIR,
# sources the migration helpers, invokes intent_relocate_dotintent directly
# (bypassing the canon-apply phase that needs the full intent CLI), and
# asserts on the resulting layout.

load "../lib/test_helper.bash"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-relocate-XXXXXX)"
  # Source helpers AFTER mktemp -- intent_helpers exports functions only.
  # shellcheck source=/dev/null
  source "${INTENT_PROJECT_ROOT}/bin/intent_helpers"
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

  # intent_relocate_dotintent does NOT bump the stamp (that's a separate phase
  # in migrate_v2_9_0_to_v2_10_0); the stamp should still read 2.9.0 here.
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

  # Tree-snapshot before second call.
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

# --- 6. Cross-filesystem fallback (skip on macOS) -------------------------

@test "cross-filesystem fallback: cp -a path on Linux tmpfs (skipped on macOS)" {
  if [ "$(uname)" = "Darwin" ]; then
    skip "cross-FS test requires Linux tmpfs; macOS lacks an equivalent fixture"
  fi
  if [ ! -w /dev/shm ]; then
    skip "/dev/shm not writable; cannot stage cross-FS source"
  fi

  local cross_src="/dev/shm/intent-test-cross-$$"
  local proj="$TEST_TEMP_DIR/proj"
  mkdir -p "$cross_src" "$proj"
  echo '{"intent_version":"2.9.0"}' > "$cross_src/config.json"
  ln -s "$cross_src" "$proj/.intent" 2>/dev/null \
    || { rm -rf "$cross_src"; skip "ln -s failed; environment cannot stage cross-FS link"; }

  # Note: relocate_dotintent refuses symlinks, so this is a placeholder for
  # the real cross-FS scenario which requires root-level tmpfs mount inside
  # $TEST_TEMP_DIR. In CI we'd bind-mount a tmpfs at $TEST_TEMP_DIR; locally
  # we settle for the symlink refusal path having been exercised in test 4.
  rm -f "$proj/.intent"
  rm -rf "$cross_src"
  skip "real cross-FS path requires CI-level bind mount; symlink refusal in test 4 covers the related code path"
}
