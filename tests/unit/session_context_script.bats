#!/usr/bin/env bats
# Tests for lib/templates/.claude/scripts/session-context.sh
#
# Exercises the SessionStart hook script in three scenarios: git repo
# with intent/wip.md present, git repo without wip.md, and a non-git
# directory (graceful degradation). Also verifies that a session_id on
# stdin is persisted to /tmp/intent-claude-session-current-id for the
# cooperating /in-session gate release.

load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/session-context.sh"

# Compute the per-project state file path for a given project directory.
# Mirrors the derivation in session-context.sh.
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
    # Per-project state files use cksum of the abs project_dir; clean up
    # both the temp dir itself and any subdir the test cd'd into.
    rm -f "$(state_file_for "$TEST_TEMP_DIR")" 2>/dev/null || true
    if [ -d "$TEST_TEMP_DIR/proj" ]; then
      rm -f "$(state_file_for "$TEST_TEMP_DIR/proj")" 2>/dev/null || true
    fi
    if [ -d "$TEST_TEMP_DIR/bare" ]; then
      rm -f "$(state_file_for "$TEST_TEMP_DIR/bare")" 2>/dev/null || true
    fi
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

@test "persists session_id from stdin JSON for /in-session gate release" {
  if ! command -v jq >/dev/null 2>&1; then
    skip "jq not available"
  fi
  mkdir -p "$TEST_TEMP_DIR/proj"
  cd "$TEST_TEMP_DIR/proj"

  run bash -c "printf '%s' '{\"session_id\":\"abc-123\"}' | CLAUDE_PROJECT_DIR=\"$PWD\" bash \"$SCRIPT\""
  assert_success
  state_file="$(state_file_for "$PWD")"
  [ -f "$state_file" ]
  [ "$(cat "$state_file")" = "abc-123" ]
}
