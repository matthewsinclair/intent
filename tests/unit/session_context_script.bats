#!/usr/bin/env bats
# Tests for lib/templates/.claude/scripts/session-context.sh
#
# Exercises the SessionStart hook script in three scenarios: git repo
# with intent/wip.md present, git repo without wip.md, and a non-git
# directory (graceful degradation).
#
# v2.11.8: the hook no longer persists session_id anywhere. Identity now
# comes from $CLAUDE_CODE_SESSION_ID on both gate sides; the shared
# per-project state file that concurrent sessions stomped was removed. A
# negative test asserts no such file is written.

load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/session-context.sh"

# Compute the (now-retired) per-project state file path for a given project
# directory. Used only by the negative test that asserts it is NOT written.
state_file_for() {
  local proj="$1"
  local key
  key="$(printf '%s' "$proj" | cksum | awk '{print $1}')"
  printf '/tmp/intent-claude-session-current-id-%s' "$key"
}

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-session-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
}

teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "session-context.sh exists and is executable" {
  [ -x "$SCRIPT" ]
}

@test "emits project name and git context for a git project with wip.md" {
  mkdir -p "$TEST_TEMP_DIR/proj/intent"
  cd "$TEST_TEMP_DIR/proj"
  git init -q .
  git -c user.email=t@t.com -c user.name=Test commit --allow-empty -q -m init
  cat > intent/wip.md <<'EOF'
# Work In Progress

## Current State

**ST0035 (Canonical LLM Config + Fleet Rollout) active.**
EOF

  CLAUDE_PROJECT_DIR="$PWD" run bash "$SCRIPT" < /dev/null
  assert_success
  assert_output_contains "Intent project: proj"
  assert_output_contains "Git: "
  assert_output_contains "WIP: ST0035"
}

@test "emits project name and git context without wip.md" {
  mkdir -p "$TEST_TEMP_DIR/proj"
  cd "$TEST_TEMP_DIR/proj"
  git init -q .
  git -c user.email=t@t.com -c user.name=Test commit --allow-empty -q -m init

  CLAUDE_PROJECT_DIR="$PWD" run bash "$SCRIPT" < /dev/null
  assert_success
  assert_output_contains "Intent project: proj"
  assert_output_contains "Git: "
  refute_output_contains "WIP: "
}

@test "degrades gracefully with no git" {
  mkdir -p "$TEST_TEMP_DIR/bare"
  cd "$TEST_TEMP_DIR/bare"

  CLAUDE_PROJECT_DIR="$PWD" run bash "$SCRIPT" < /dev/null
  assert_success
  assert_output_contains "Intent project: bare"
  refute_output_contains "Git: "
}

@test "does not persist session_id to a shared state file" {
  mkdir -p "$TEST_TEMP_DIR/proj"
  cd "$TEST_TEMP_DIR/proj"
  state_file="$(state_file_for "$PWD")"
  rm -f "$state_file"

  run bash -c "printf '%s' '{\"session_id\":\"abc-123\"}' | CLAUDE_PROJECT_DIR=\"$PWD\" bash \"$SCRIPT\""
  assert_success
  # The shared file was the concurrent-session corruption source; it must
  # never be written. Identity comes from \$CLAUDE_CODE_SESSION_ID instead.
  [ ! -f "$state_file" ]
}
