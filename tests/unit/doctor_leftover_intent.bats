#!/usr/bin/env bats
# Tests for the `intent doctor` check 4d: warn (not error) when a stale
# top-level .intent/ directory remains after a v2.9 -> v2.10 migration.
# Auto-staging is intentionally NOT done -- the user must run `git rm -rf`
# explicitly so the cleanup is visible in the commit.

load "../lib/test_helper.bash"

@test "doctor: 'leftover .intent' check passes when no .intent/ exists" {
  local project="$(create_test_project "leftover-clean")"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [ "$status" -eq 0 ]
  [[ "$output" == *"checking: leftover .intent ok"* ]]
}

@test "doctor: 'leftover .intent' check warns when .intent/ AND intent/.config/ both exist" {
  local project="$(create_test_project "leftover-stale")"
  mkdir -p "$project/.intent"
  echo '{"intent_version":"2.9.0"}' > "$project/.intent/config.json"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  # Status should be 0 (warning, not error)
  [ "$status" -eq 0 ]
  [[ "$output" == *"warning"* ]]
  [[ "$output" == *"stale .intent/ directory still present"* ]]
}

@test "doctor: leftover-.intent warning surfaces git-rm recovery hint" {
  local project="$(create_test_project "leftover-recovery")"
  mkdir -p "$project/.intent"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [[ "$output" == *"git rm -rf .intent/"* ]]
  [[ "$output" == *"chore: drop leftover .intent/"* ]]
}

@test "doctor: leftover-.intent check is silent in --quiet mode when ok" {
  local project="$(create_test_project "leftover-quiet-ok")"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor --quiet

  [ "$status" -eq 0 ]
  # Quiet mode suppresses the per-check "checking: ..." prefix
  [[ "$output" != *"checking: leftover .intent"* ]]
}

@test "doctor: leftover-.intent check still warns in --quiet mode" {
  local project="$(create_test_project "leftover-quiet-warn")"
  mkdir -p "$project/.intent"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor --quiet

  [[ "$output" == *"warning"* ]]
  [[ "$output" == *"stale .intent/"* ]]
}
