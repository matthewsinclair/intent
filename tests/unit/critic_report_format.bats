#!/usr/bin/env bats
# Tests for the headless critic runner's report format (ST0042 T10).
#
# Earlier versions of this file asserted on a heredoc fixture defined inside
# the test itself -- green regardless of product behaviour. These tests run
# the REAL `intent critic` pipeline (rule load -> proxy extraction -> scan ->
# emit) against a synthetic rules directory via --rules, and assert on the
# actual text and JSON reports the runner emits.

load "../lib/test_helper.bash"

# Build a rules tree with one warning-severity and one critical-severity
# shell rule, each with a deterministic Greppable proxy.
make_rules_dir() {
  local root="$1"
  local dir

  dir="$root/shell/code/warn-marker"
  mkdir -p "$dir"
  cat > "$dir/RULE.md" << 'EOF'
---
id: IN-SH-TEST-901
title: "Warn marker"
language: shell
category: code
severity: warning
---

# Warn marker

Synthetic warning rule for report-format tests.

## Problem

N/A

## Detection

Greppable proxy (not authoritative):

```bash
grep -nE 'WARN_MARKER'
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

  dir="$root/shell/code/crit-marker"
  mkdir -p "$dir"
  sed -e 's/IN-SH-TEST-901/IN-SH-TEST-902/' \
      -e 's/severity: warning/severity: critical/' \
      -e 's/WARN_MARKER/CRIT_MARKER/' \
      -e 's/Warn marker/Crit marker/' \
      "$root/shell/code/warn-marker/RULE.md" > "$dir/RULE.md"
}

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  RULES_DIR="$TEST_TEMP_DIR/rules"
  make_rules_dir "$RULES_DIR"

  TARGET="$TEST_TEMP_DIR/target.sh"
  cat > "$TARGET" << 'EOF'
#!/bin/bash
echo one   # WARN_MARKER
echo two   # CRIT_MARKER
EOF

  CLEAN_TARGET="$TEST_TEMP_DIR/clean.sh"
  cat > "$CLEAN_TARGET" << 'EOF'
#!/bin/bash
echo clean
EOF

  export INTENT_EXT_DISABLE=1
}

teardown() {
  unset INTENT_EXT_DISABLE
  rm -rf "$TEST_TEMP_DIR"
}

run_critic() {
  run "${INTENT_BIN_DIR}/intent" critic shell --rules "$RULES_DIR" "$@"
}

# ====================================================================
# Text format
# ====================================================================

@test "text report: finding line is [SEVERITY] <id> at <file>:<line> with excerpt" {
  run_critic --files "$TARGET" --severity-min warning --format text
  [ "$status" -eq 1 ]
  assert_output_contains "[WARNING] IN-SH-TEST-901 at $TARGET:2"
  assert_output_contains "[CRITICAL] IN-SH-TEST-902 at $TARGET:3"
  assert_output_contains "> echo one   # WARN_MARKER"
}

@test "text report: severity sections are headed and ordered critical-first" {
  run_critic --files "$TARGET" --severity-min warning --format text
  local crit_line warn_line
  crit_line=$(printf '%s\n' "$output" | grep -n '== CRITICAL (1) ==' | head -1 | cut -d: -f1)
  warn_line=$(printf '%s\n' "$output" | grep -n '== WARNING (1) ==' | head -1 | cut -d: -f1)
  [ -n "$crit_line" ] || fail "no CRITICAL section header: $output"
  [ -n "$warn_line" ] || fail "no WARNING section header: $output"
  [ "$crit_line" -lt "$warn_line" ] || fail "CRITICAL section does not precede WARNING"
}

@test "text report: clean run prints ok line and exits 0" {
  run_critic --files "$CLEAN_TARGET" --severity-min warning --format text
  [ "$status" -eq 0 ]
  assert_output_contains "ok: no shell findings at severity >= warning across 1 file(s)"
}

@test "text report: --severity-min critical filters the warning finding" {
  run_critic --files "$TARGET" --severity-min critical --format text
  [ "$status" -eq 1 ]
  assert_output_contains "IN-SH-TEST-902"
  refute_output_contains "IN-SH-TEST-901"
}

# ====================================================================
# JSON format
# ====================================================================

@test "json report: emits valid JSON with severity/rule_id/file/line/excerpt" {
  run_critic --files "$TARGET" --severity-min warning --format json
  [ "$status" -eq 1 ]
  printf '%s\n' "$output" | jq -e . > /dev/null \
    || fail "output is not valid JSON: $output"
  [ "$(printf '%s\n' "$output" | jq -r 'length')" = "2" ]
  [ "$(printf '%s\n' "$output" | jq -r '.[] | select(.rule_id == "IN-SH-TEST-901") | .severity')" = "warning" ]
  [ "$(printf '%s\n' "$output" | jq -r '.[] | select(.rule_id == "IN-SH-TEST-901") | .line')" = "2" ]
  [ "$(printf '%s\n' "$output" | jq -r '.[] | select(.rule_id == "IN-SH-TEST-902") | .file')" = "$TARGET" ]
}

@test "json report: clean run emits an empty array and exits 0" {
  run_critic --files "$CLEAN_TARGET" --severity-min warning --format json
  [ "$status" -eq 0 ]
  [ "$(printf '%s\n' "$output" | jq -r 'length')" = "0" ]
}

# ====================================================================
# Exit-code contract
# ====================================================================

@test "exit codes: 1 on findings at threshold, 0 below threshold" {
  run_critic --files "$TARGET" --severity-min warning --format text
  [ "$status" -eq 1 ]
  # Only the warning finding present, threshold critical -> exit 0.
  local warn_only="$TEST_TEMP_DIR/warn-only.sh"
  printf '#!/bin/bash\necho x # WARN_MARKER\n' > "$warn_only"
  run_critic --files "$warn_only" --severity-min critical --format text
  [ "$status" -eq 0 ]
}
