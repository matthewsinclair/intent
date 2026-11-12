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
#
# SERIAL, BECAUSE ITS TESTS SHARE ONE FILE OUTSIDE THEIR TEMPORARY DIRECTORIES.
# The script under test always writes /tmp/intent/in-session-unknown.sentinel,
# the host-wide fallback the gate reads for a session it cannot name, and one
# test reads that file's first line. Run in parallel, another test's write lands
# between that test's run and its read. devbin's lanes runner keeps a file
# tagged `serial` out of its parallel lane (ST0093 WP-06).
# bats file_tags=serial

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

# --------------------------------------------------------------------
# Issue 0676: the session's sentinel records the languages /in-session loaded
# skills for, so the prompt gate and `intent wb pickup` can say when the
# project's declaration moves after it. A stub `intent` first on PATH stands
# in for `intent lang declared`.
# --------------------------------------------------------------------

BOOTSTRAP_HEADER="# intent: languages /in-session loaded skills for"

stub_declared() {
  STUB_BIN="$BATS_TEST_TMPDIR/stub-bin"
  mkdir -p "$STUB_BIN"
  cat > "$STUB_BIN/intent" <<'EOS'
#!/usr/bin/env bash
[ "$1 $2" = "lang declared" ] || exit 0
[ -n "${STUB_DECLARED-}" ] && printf '%s\n' "$STUB_DECLARED"
exit "${STUB_DECLARED_RC:-0}"
EOS
  chmod +x "$STUB_BIN/intent"
}

@test "release-gate.sh records the declared languages under the header" {
  stub_declared
  sid="test-uuid-$$-ddd"
  rm -f "/tmp/intent/in-session-${sid}.sentinel"
  CLAUDE_CODE_SESSION_ID="$sid" STUB_DECLARED=$'elixir\nrust' PATH="$STUB_BIN:$PATH" run bash "$SCRIPT"
  [ "$status" -eq 0 ]
  [ "$(cat "/tmp/intent/in-session-${sid}.sentinel")" = "$BOOTSTRAP_HEADER"$'\nelixir\nrust' ]
  rm -f "/tmp/intent/in-session-${sid}.sentinel"
}

@test "release-gate.sh records an empty declaration as the header alone" {
  stub_declared
  sid="test-uuid-$$-eee"
  CLAUDE_CODE_SESSION_ID="$sid" STUB_DECLARED= PATH="$STUB_BIN:$PATH" run bash "$SCRIPT"
  [ "$status" -eq 0 ]
  [ "$(cat "/tmp/intent/in-session-${sid}.sentinel")" = "$BOOTSTRAP_HEADER" ]
  rm -f "/tmp/intent/in-session-${sid}.sentinel"
}

@test "release-gate.sh still releases, recording nothing, when intent cannot answer" {
  stub_declared
  sid="test-uuid-$$-fff"
  CLAUDE_CODE_SESSION_ID="$sid" STUB_DECLARED=rust STUB_DECLARED_RC=4 PATH="$STUB_BIN:$PATH" run bash "$SCRIPT"
  [ "$status" -eq 0 ]
  [ -f "/tmp/intent/in-session-${sid}.sentinel" ]
  [ ! -s "/tmp/intent/in-session-${sid}.sentinel" ]
  [ ! -s "/tmp/intent/in-session-unknown.sentinel" ] || [ "$(head -n 1 /tmp/intent/in-session-unknown.sentinel)" != "$BOOTSTRAP_HEADER" ]
  rm -f "/tmp/intent/in-session-${sid}.sentinel"
}
