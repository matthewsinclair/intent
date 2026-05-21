#!/usr/bin/env bats
# Tests for the release-gate.sh helper that the /in-session skill invokes.
# Extracted from SKILL.md inline so any pipeline survives Claude Code's
# skill-renderer token-stripping.
#
# v2.11.8: identity resolves from a single source, $CLAUDE_CODE_SESSION_ID,
# matching require-in-session.sh. The earlier shared per-project state file
# (the concurrent-session corruption source) was removed. These tests must
# control CLAUDE_CODE_SESSION_ID explicitly -- the ambient value is exported
# into the bats process and would otherwise leak into every run.

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
  # Defensive: awk in this script re-introduces the renderer-strip
  # vulnerability that silently emptied the inline form.
  run grep -q '| awk' "$SCRIPT"
  [ "$status" -ne 0 ]
}

# --------------------------------------------------------------------
# Sentinel-touching behaviour
# --------------------------------------------------------------------

@test "release-gate.sh touches the sentinel for CLAUDE_CODE_SESSION_ID" {
  sid="test-uuid-$$-aaa"
  rm -f "/tmp/intent/in-session-${sid}.sentinel"

  CLAUDE_CODE_SESSION_ID="$sid" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [[ "$output" == *"session=${sid}"* ]]
  [ -f "/tmp/intent/in-session-${sid}.sentinel" ]

  rm -f "/tmp/intent/in-session-${sid}.sentinel"
}

@test "release-gate.sh always touches unknown.sentinel" {
  sid="test-uuid-$$-bbb"
  CLAUDE_CODE_SESSION_ID="$sid" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [ -f "/tmp/intent/in-session-unknown.sentinel" ]

  rm -f "/tmp/intent/in-session-${sid}.sentinel"
}

@test "release-gate.sh degrades to unknown-only when env var is empty" {
  CLAUDE_CODE_SESSION_ID="" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [[ "$output" == *"session=(none)"* ]]
  [ -f "/tmp/intent/in-session-unknown.sentinel" ]
}

@test "release-gate.sh degrades to unknown-only when env var is unset" {
  run env -u CLAUDE_CODE_SESSION_ID bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [[ "$output" == *"session=(none)"* ]]
  [ -f "/tmp/intent/in-session-unknown.sentinel" ]
}

# --------------------------------------------------------------------
# Concurrent-session regression (the original deadlock)
# --------------------------------------------------------------------

@test "release-gate.sh ignores any leftover shared state file" {
  # The pre-v2.11.8 deadlock: a shared per-project state file held another
  # session's id, so the releaser touched the wrong sentinel. Prove that file
  # is no longer load-bearing -- a poisoned copy must not divert the release.
  sid="test-uuid-$$-ccc"
  poison="test-uuid-$$-poison"
  poison_key="$(printf '%s' "/some/project" | cksum | awk '{print $1}')"
  echo "$poison" > "/tmp/intent-claude-session-current-id-${poison_key}"
  echo "$poison" > "/tmp/intent-claude-session-current-id"
  rm -f "/tmp/intent/in-session-${sid}.sentinel" \
        "/tmp/intent/in-session-${poison}.sentinel"

  CLAUDE_CODE_SESSION_ID="$sid" run bash "$SCRIPT"

  [ "$status" -eq 0 ]
  [ -f "/tmp/intent/in-session-${sid}.sentinel" ]
  [ ! -f "/tmp/intent/in-session-${poison}.sentinel" ]

  rm -f "/tmp/intent/in-session-${sid}.sentinel" \
        "/tmp/intent-claude-session-current-id-${poison_key}" \
        "/tmp/intent-claude-session-current-id"
}
