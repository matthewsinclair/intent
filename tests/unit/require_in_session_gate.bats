#!/usr/bin/env bats
# Tests for the require-in-session.sh strict UserPromptSubmit gate.
#
# v2.11.5: covers the INTENT_SKIP_IN_SESSION_GATE env-var bypass added so
# non-interactive automation (eg `intent treeindex`) can spawn `claude -p`
# against an Intent project without being silently blocked. The bypass
# branch is the new code path; the slash-command and sentinel branches
# are smokes that the prepended bypass block does not regress them.

bats_require_minimum_version 1.5.0
load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/require-in-session.sh"

setup_gate() {
  GATE_SENTINEL_DIR="/tmp/intent"
  mkdir -p "$GATE_SENTINEL_DIR"
  GATE_SESSION_ID="bats-$(date +%s)-$$"
  GATE_SENTINEL="$GATE_SENTINEL_DIR/in-session-${GATE_SESSION_ID}.sentinel"
  rm -f "$GATE_SENTINEL"
}

teardown_gate() {
  rm -f "$GATE_SENTINEL"
}

# --------------------------------------------------------------------
# File presence and shape
# --------------------------------------------------------------------

@test "require-in-session.sh exists and is executable" {
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]
}

# --------------------------------------------------------------------
# Bypass branch (v2.11.5)
# --------------------------------------------------------------------

@test "INTENT_SKIP_IN_SESSION_GATE=1 short-circuits with exit 0 and empty stderr" {
  setup_gate
  # No sentinel exists, prompt is not a slash command -- normally would
  # block. Bypass must short-circuit before any other check.
  run --separate-stderr env INTENT_SKIP_IN_SESSION_GATE=1 \
    bash "$SCRIPT" <<< '{"session_id":"'"$GATE_SESSION_ID"'","prompt":"hello"}'
  [ "$status" -eq 0 ]
  [ -z "$output" ]
  [ -z "$stderr" ]
  teardown_gate
}

@test "INTENT_SKIP_IN_SESSION_GATE=1 short-circuits with empty stdin" {
  # Treeindex pipes prompt content via heredoc/echo; the bypass must not
  # depend on payload shape.
  run --separate-stderr env INTENT_SKIP_IN_SESSION_GATE=1 bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  [ -z "$stderr" ]
}

@test "empty INTENT_SKIP_IN_SESSION_GATE still gates" {
  setup_gate
  # ${VAR:-} treats empty as unset for [ -n ... ]; the bypass must be
  # value-bearing, not presence-bearing.
  run --separate-stderr env INTENT_SKIP_IN_SESSION_GATE= \
    bash "$SCRIPT" <<< '{"session_id":"'"$GATE_SESSION_ID"'","prompt":"hello"}'
  [ "$status" -eq 2 ]
  [[ "$stderr" == *"/in-session must run"* ]]
  teardown_gate
}

@test "unset INTENT_SKIP_IN_SESSION_GATE still gates" {
  setup_gate
  # Smoke for the set -u + ${VAR:-} interaction. No env var at all.
  run --separate-stderr bash "$SCRIPT" <<< '{"session_id":"'"$GATE_SESSION_ID"'","prompt":"hello"}'
  [ "$status" -eq 2 ]
  [[ "$stderr" == *"/in-session must run"* ]]
  teardown_gate
}

# --------------------------------------------------------------------
# Existing pass-throughs (regression smokes for the prepended bypass)
# --------------------------------------------------------------------

@test "slash-command prompt passes through without bypass" {
  setup_gate
  # No sentinel, no bypass, but prompt is a slash command -- must exit 0.
  run --separate-stderr bash "$SCRIPT" <<< '{"session_id":"'"$GATE_SESSION_ID"'","prompt":"/help"}'
  [ "$status" -eq 0 ]
  [ -z "$stderr" ]
  teardown_gate
}

@test "sentinel pass-through still works without bypass" {
  setup_gate
  touch "$GATE_SENTINEL"
  run --separate-stderr bash "$SCRIPT" <<< '{"session_id":"'"$GATE_SESSION_ID"'","prompt":"hello"}'
  [ "$status" -eq 0 ]
  [ -z "$stderr" ]
  teardown_gate
}
