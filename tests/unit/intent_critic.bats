#!/usr/bin/env bats
# Tests for bin/intent_critic (headless critic runner, ST0035/WP-05).
#
# Exercises the CLI surface (help, arg validation, exit codes) and runs
# the runner against real rule fixtures in intent/plugins/claude/rules/
# (the strong-assertions rule's own bad_test.exs / good_test.exs).

load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/bin/intent_critic"
FIX_BAD="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/strong-assertions/bad_test.exs"
FIX_GOOD="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/strong-assertions/good_test.exs"

# ====================================================================
# CLI: help and arg validation
# ====================================================================

@test "intent_critic exists and is executable" {
  [ -x "$SCRIPT" ]
}

@test "--help prints usage" {
  run "$SCRIPT" --help
  assert_success
  assert_output_contains "intent critic <lang>"
  assert_output_contains "--severity-min"
  assert_output_contains "--format text|json"
}

@test "no args exits 2 with usage" {
  run "$SCRIPT"
  [ "$status" -eq 2 ]
  assert_output_contains "intent critic <lang>"
}

@test "invalid language exits 2" {
  run "$SCRIPT" cobol --files /tmp/x
  [ "$status" -eq 2 ]
  assert_output_contains "must be a language"
}

@test "invalid --severity-min exits 2" {
  run "$SCRIPT" elixir --files "$FIX_BAD" --severity-min nope
  [ "$status" -eq 2 ]
  assert_output_contains "invalid --severity-min"
}

@test "invalid --format exits 2" {
  run "$SCRIPT" elixir --files "$FIX_BAD" --format xml
  [ "$status" -eq 2 ]
  assert_output_contains "invalid --format"
}

@test "no --files and no --staged exits 2" {
  run "$SCRIPT" elixir
  [ "$status" -eq 2 ]
  assert_output_contains "no files specified"
}

# ====================================================================
# Detection: bad fixture produces findings, good fixture clean
# ====================================================================

@test "elixir bad fixture produces critical findings and exits 1" {
  run "$SCRIPT" elixir --files "$FIX_BAD" --severity-min critical
  [ "$status" -eq 1 ]
  assert_output_contains "CRITICAL"
  assert_output_contains "IN-EX-TEST-001"
}

@test "elixir good fixture at critical threshold is clean, exits 0" {
  run "$SCRIPT" elixir --files "$FIX_GOOD" --severity-min critical
  assert_success
  assert_output_contains "ok:"
}

# ====================================================================
# --severity-min filters correctly
# ====================================================================

@test "severity-min critical excludes warnings" {
  run "$SCRIPT" elixir --files "$FIX_BAD" --severity-min critical
  [ "$status" -eq 1 ]
  refute_output_contains "== WARNING"
}

# ====================================================================
# Format: JSON is valid JSON
# ====================================================================

@test "--format json produces parseable JSON" {
  run "$SCRIPT" elixir --files "$FIX_BAD" --severity-min critical --format json
  [ "$status" -eq 1 ]
  # output should be a JSON array with length > 0
  printf '%s' "$output" | jq 'length > 0' | grep -q true
}

@test "--format json on clean fixture emits empty array" {
  run "$SCRIPT" elixir --files "$FIX_GOOD" --severity-min critical --format json
  assert_success
  assert_output "[]"
}

# ====================================================================
# --staged: pre-commit invocation
# ====================================================================

@test "--staged outside git repo exits 2" {
  cd "$TEST_TEMP_DIR"
  run "$SCRIPT" elixir --staged
  [ "$status" -eq 2 ]
  assert_output_contains "git repository"
}

@test "--staged with no staged files of the language exits 0" {
  cd "$TEST_TEMP_DIR"
  git init -q .
  git -c user.email=t@t.com -c user.name=Test commit --allow-empty -q -m init
  run "$SCRIPT" elixir --staged
  assert_success
  assert_output_contains "no staged elixir files"
}

# ====================================================================
# Dispatch: `intent critic` delegates correctly
# ====================================================================

@test "disabled rule in .intent_critic.yml is suppressed" {
  cd "$TEST_TEMP_DIR"
  mkdir -p intent/.config
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.9.1","project_name":"Disabled Test","author":"t","created_date":"2026-04-24T00:00:00Z"}
EOF
  cat > .intent_critic.yml <<'EOF'
disabled:
  - IN-EX-TEST-001
severity_min: critical
EOF
  run "$SCRIPT" elixir --files "$FIX_BAD" --severity-min critical
  # With IN-EX-TEST-001 disabled and no other critical rules firing on
  # the fixture, the runner should exit 0 clean.
  assert_success
  refute_output_contains "IN-EX-TEST-001"
}

@test "non-disabled rule still fires when config disables a different rule" {
  cd "$TEST_TEMP_DIR"
  mkdir -p intent/.config
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.9.1","project_name":"Disabled Test","author":"t","created_date":"2026-04-24T00:00:00Z"}
EOF
  cat > .intent_critic.yml <<'EOF'
disabled:
  - IN-EX-TEST-999
severity_min: critical
EOF
  run "$SCRIPT" elixir --files "$FIX_BAD" --severity-min critical
  [ "$status" -eq 1 ]
  assert_output_contains "IN-EX-TEST-001"
}

@test "intent critic dispatches to bin/intent_critic" {
  # `intent critic` is a project command and requires a valid intent/.config/
  # project root. Run from INTENT_PROJECT_ROOT itself (the Intent repo).
  cd "${INTENT_PROJECT_ROOT}"
  run "${INTENT_BIN_DIR}/intent" critic --help
  assert_success
  assert_output_contains "intent critic <lang>"
}
