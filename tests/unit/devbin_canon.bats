#!/usr/bin/env bats
# devbin_canon -- `bin/devbin canon`'s exit codes (issue 0471).
#
# The header promises exit 1 for a real divergence and exit 2 only when a
# control fails. Every control used to fail through `die`, which exits 1, so a
# broken instrument read exactly like a finding.

load "../lib/test_helper.bash"

CANON="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/canon"

@test "canon: a failed control exits 2, through die_control" {
  local fn="${TEST_TEMP_DIR}/die_control.sh"
  sed -n '/^die_control() {/,/^}/p' "$CANON" > "$fn"
  # shellcheck disable=SC1090
  . "$fn"
  DEVBIN_NAME=devbin run die_control "positive control FAILED -- a test"
  [ "$status" -eq 2 ]
  [[ "$output" == *"positive control FAILED"* ]]
}

@test "canon: every control failure goes through die_control, never die" {
  run grep -cE '^[[:space:]]*(\*\) )?die "(positive|negative|content|the content arm|divergence: unknown)' "$CANON"
  [ "$output" = "0" ]
  run grep -cE 'die_control "(positive control|positive control FAILED for|negative control|content positive|content negative|content third-state|the content arm|divergence: unknown)' "$CANON"
  [ "$output" = "8" ]
}

@test "canon: the report prints each list one path per line, quoted" {
  run grep -c "printf '      %s\\\\n' \\\$" "$CANON"
  [ "$output" = "0" ]
}

@test "canon: a clean estate exits 0" {
  cd "$INTENT_PROJECT_ROOT" || return 1
  run env DEVBIN_LIB="$INTENT_PROJECT_ROOT/bin/.devbin/lib" bash "$CANON"
  [ "$status" -eq 0 ]
  [[ "$output" == *"controls: membership both directions fired"* ]]
}
