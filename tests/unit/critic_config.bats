#!/usr/bin/env bats
# Tests for .intent_critic.yml handling (ST0042 T10).
#
# Earlier versions of this file only verified that a host YAML parser could
# parse fixture files -- green regardless of product behaviour. The real
# consumers of .intent_critic.yml are:
#   1. The headless runner (`intent critic`): honours `disabled:` per rule.
#   2. The pre-commit hook: reads `severity_min:` and passes it through as
#      --severity-min (falling back to warning on invalid values).
# These tests exercise both, plus structural checks of the shipped sample.

load "../lib/test_helper.bash"

SAMPLE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/_schema/sample-intent-critic.yml"
HOOK="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit.sh"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  # Fixed paths so assertions in the parent shell see what the `run`
  # subshell wrote.
  STUB_BIN="$TEST_TEMP_DIR/stub-bin"
  CALL_LOG="$TEST_TEMP_DIR/calls.log"
  export INTENT_EXT_DISABLE=1
}

teardown() {
  unset INTENT_EXT_DISABLE
  rm -rf "$TEST_TEMP_DIR"
}

# ====================================================================
# Sample file shipped at rules/_schema/ (structural)
# ====================================================================

@test "config: sample file exists at rules/_schema/" {
  assert_file_exists "$SAMPLE"
}

@test "config: sample file documents the three schema keys" {
  assert_file_contains "$SAMPLE" "disabled:"
  assert_file_contains "$SAMPLE" "severity_min:"
  assert_file_contains "$SAMPLE" "show_all:"
}

@test "config: sample disabled entries carry a # reason comment" {
  local lines_with_ids lines_with_reasons
  lines_with_ids=$(grep -cE '^  - IN-[A-Z]{2,3}-' "$SAMPLE")
  lines_with_reasons=$(grep -cE '^  - IN-[A-Z]{2,3}-.*# reason:' "$SAMPLE")
  [ "$lines_with_ids" -gt 0 ]
  [ "$lines_with_ids" = "$lines_with_reasons" ]
}

@test "config: critics.md documents every sample key" {
  local docs="${INTENT_PROJECT_ROOT}/intent/docs/critics.md"
  assert_file_exists "$docs"
  assert_file_contains "$docs" "disabled"
  assert_file_contains "$docs" "severity_min"
  assert_file_contains "$docs" "show_all"
}

# ====================================================================
# Runner honours `disabled:` (real product behaviour)
# ====================================================================

# Lay down a minimal Intent project containing a shell file that trips a
# synthetic rule, served from a --rules dir.
make_runner_fixture() {
  PROJ="$TEST_TEMP_DIR/proj"
  mkdir -p "$PROJ/intent/.config"
  printf '{"languages":["shell"]}\n' > "$PROJ/intent/.config/config.json"

  printf '#!/bin/bash\necho x # CFG_MARKER\n' > "$PROJ/target.sh"

  RULES_DIR="$TEST_TEMP_DIR/rules"
  local dir="$RULES_DIR/shell/code/cfg-marker"
  mkdir -p "$dir"
  cat > "$dir/RULE.md" << 'EOF'
---
id: IN-SH-TEST-903
title: "Config marker"
language: shell
category: code
severity: warning
---

# Config marker

Synthetic rule for config tests.

## Problem

N/A

## Detection

Greppable proxy (not authoritative):

```bash
grep -nE 'CFG_MARKER'
```

## Bad

N/A

## Good

N/A

## When This Applies

Always.

## When This Does Not Apply

Never.

## Further Reading

N/A
EOF
}

@test "config: without .intent_critic.yml the finding fires (defaults path)" {
  make_runner_fixture
  run bash -c "cd '$PROJ' && '${INTENT_BIN_DIR}/intent' critic shell --rules '$RULES_DIR' --files target.sh --format text"
  [ "$status" -eq 1 ]
  assert_output_contains "IN-SH-TEST-903"
}

@test "config: disabled rule id suppresses the finding" {
  make_runner_fixture
  cat > "$PROJ/.intent_critic.yml" << 'EOF'
disabled:
  - IN-SH-TEST-903 # reason: synthetic rule disabled for config test
EOF
  run bash -c "cd '$PROJ' && '${INTENT_BIN_DIR}/intent' critic shell --rules '$RULES_DIR' --files target.sh --format text"
  [ "$status" -eq 0 ]
  refute_output_contains "IN-SH-TEST-903"
}

@test "config: disabling one rule does not suppress others" {
  make_runner_fixture
  cat > "$PROJ/.intent_critic.yml" << 'EOF'
disabled:
  - IN-SH-OTHER-999 # reason: unrelated id
EOF
  run bash -c "cd '$PROJ' && '${INTENT_BIN_DIR}/intent' critic shell --rules '$RULES_DIR' --files target.sh --format text"
  [ "$status" -eq 1 ]
  assert_output_contains "IN-SH-TEST-903"
}

# ====================================================================
# Pre-commit hook honours `severity_min:` (real product behaviour)
# ====================================================================

# Fixture git repo + stub `intent` that records the args it was called with.
run_hook_with_yml() {
  local yml_content="$1"
  local repo="$TEST_TEMP_DIR/repo"
  mkdir -p "$STUB_BIN" "$repo/intent/.config"
  cat > "$STUB_BIN/intent" << EOF
#!/bin/bash
echo "\$@" >> "$CALL_LOG"
exit 0
EOF
  chmod +x "$STUB_BIN/intent"
  printf '{"languages":["shell"]}\n' > "$repo/intent/.config/config.json"
  [ -n "$yml_content" ] && printf '%s\n' "$yml_content" > "$repo/.intent_critic.yml"
  git -C "$repo" init -q
  ( cd "$repo" && PATH="$STUB_BIN:$PATH" bash "$HOOK" )
}

@test "config: hook passes severity_min from .intent_critic.yml to the runner" {
  run run_hook_with_yml 'severity_min: recommendation'
  [ "$status" -eq 0 ]
  grep -q -- '--severity-min recommendation' "$CALL_LOG" \
    || fail "severity_min not propagated: $(cat "$CALL_LOG")"
}

@test "config: hook falls back to warning on an invalid severity_min value" {
  run run_hook_with_yml 'severity_min: bananas'
  [ "$status" -eq 0 ]
  grep -q -- '--severity-min warning' "$CALL_LOG" \
    || fail "invalid severity_min did not fall back to warning: $(cat "$CALL_LOG")"
}

@test "config: hook defaults to warning when .intent_critic.yml is absent" {
  run run_hook_with_yml ''
  [ "$status" -eq 0 ]
  grep -q -- '--severity-min warning' "$CALL_LOG" \
    || fail "absent config did not default to warning: $(cat "$CALL_LOG")"
}
