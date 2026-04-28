#!/usr/bin/env bats
# Tests for the release-gate.sh helper that the /in-session skill invokes.
# Extracted from SKILL.md inline so the awk pipeline survives Claude Code's
# skill-renderer token-stripping. Tests cover the per-project + legacy +
# unknown sentinel waterfall and the malformed-cksum-output guard.

load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-session/scripts/release-gate.sh"

# --------------------------------------------------------------------
# File presence and shape
# --------------------------------------------------------------------

@test "release-gate.sh exists and is executable" {
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]
}

@test "release-gate.sh uses pure-shell substitution (no awk \$1)" {
  # Defensive: if awk reappears in this script we re-introduce the
  # renderer-strip vulnerability. Anything that pipes to awk would still be
  # safe in a script file, but we want one obvious form.
  run grep -q '| awk' "$SCRIPT"
  [ "$status" -ne 0 ]
}

# --------------------------------------------------------------------
# Sentinel-touching behaviour against an isolated tmp tree
# --------------------------------------------------------------------

# All tests below set CLAUDE_PROJECT_DIR to TEST_TEMP_DIR (a unique mktemp
# per test) and override SENTINEL_DIR by running inside a custom HOME-like
# layout. We cannot override SENTINEL_DIR via env (the script hard-codes it)
# without editing the script, so we sandbox by clearing the per-project
# state file before each test and restoring after.

setup_isolated() {
  ISO_PROJECT="$TEST_TEMP_DIR/iso-project"
  mkdir -p "$ISO_PROJECT"
  ISO_KEY="$(printf '%s' "$ISO_PROJECT" | cksum | awk '{print $1}')"
  ISO_STATE="/tmp/intent-claude-session-current-id-${ISO_KEY}"
}

@test "release-gate.sh resolves per-project session_id and touches the sentinel" {
  setup_isolated
  echo "test-uuid-aaa" > "$ISO_STATE"
  rm -f "/tmp/intent/in-session-test-uuid-aaa.sentinel"

  CLAUDE_PROJECT_DIR="$ISO_PROJECT" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [[ "$output" == *"per-project=test-uuid-aaa"* ]]
  [ -f "/tmp/intent/in-session-test-uuid-aaa.sentinel" ]

  rm -f "$ISO_STATE" "/tmp/intent/in-session-test-uuid-aaa.sentinel"
}

@test "release-gate.sh always touches unknown.sentinel" {
  setup_isolated

  CLAUDE_PROJECT_DIR="$ISO_PROJECT" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [ -f "/tmp/intent/in-session-unknown.sentinel" ]
}

@test "release-gate.sh handles missing per-project state file gracefully" {
  setup_isolated
  rm -f "$ISO_STATE"

  CLAUDE_PROJECT_DIR="$ISO_PROJECT" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [[ "$output" == *"per-project=(none)"* ]]
}

@test "release-gate.sh project_key is just the cksum number, not cksum+length" {
  setup_isolated
  echo "test-uuid-bbb" > "$ISO_STATE"
  rm -f "/tmp/intent/in-session-test-uuid-bbb.sentinel"

  CLAUDE_PROJECT_DIR="$ISO_PROJECT" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  # Regression guard: the old broken inline form produced a malformed key
  # like "1234567890 99" with a space and the byte count. If that returns,
  # this test fails because the per-project lookup would have missed.
  [ -f "/tmp/intent/in-session-test-uuid-bbb.sentinel" ]

  rm -f "$ISO_STATE" "/tmp/intent/in-session-test-uuid-bbb.sentinel"
}

@test "release-gate.sh also touches legacy sentinel when set differently" {
  setup_isolated
  echo "test-uuid-ccc" > "$ISO_STATE"
  echo "test-uuid-legacy-ddd" > "/tmp/intent-claude-session-current-id"
  rm -f "/tmp/intent/in-session-test-uuid-ccc.sentinel" \
        "/tmp/intent/in-session-test-uuid-legacy-ddd.sentinel"

  CLAUDE_PROJECT_DIR="$ISO_PROJECT" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [ -f "/tmp/intent/in-session-test-uuid-ccc.sentinel" ]
  [ -f "/tmp/intent/in-session-test-uuid-legacy-ddd.sentinel" ]

  rm -f "$ISO_STATE" \
        "/tmp/intent-claude-session-current-id" \
        "/tmp/intent/in-session-test-uuid-ccc.sentinel" \
        "/tmp/intent/in-session-test-uuid-legacy-ddd.sentinel"
}
