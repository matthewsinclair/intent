#!/usr/bin/env bats
# Tests for critic language dispatch (ST0042 T10).
#
# Two surfaces:
#   1. The /in-review SKILL.md prose: names every critic, reads the
#      `languages` array, no filesystem-probe regression (doc pins).
#   2. The shipped pre-commit hook (lib/templates/hooks/pre-commit.sh):
#      the REAL mechanical dispatcher. Earlier versions of this file
#      tested a test-local reimplementation of the dispatch logic; these
#      tests run the actual hook in a fixture git repo with a stub
#      `intent` on PATH that records its invocations.

load "../lib/test_helper.bash"

SKILL_FILE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-review/SKILL.md"
HOOK="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit.sh"

# ====================================================================
# Skill file health (doc pins)
# ====================================================================

@test "in-review skill file exists" {
  assert_file_exists "$SKILL_FILE"
}

@test "in-review skill reads languages from intent/.config/config.json" {
  assert_file_contains "$SKILL_FILE" "intent/.config/config.json"
  assert_file_contains "$SKILL_FILE" "languages"
}

@test "in-review skill names every critic" {
  for critic in critic-elixir critic-rust critic-swift critic-lua critic-shell; do
    assert_file_contains "$SKILL_FILE" "$critic"
  done
}

@test "in-review skill shows a Task() invocation for each critic" {
  for critic in critic-elixir critic-rust critic-swift critic-lua critic-shell; do
    assert_file_contains "$SKILL_FILE" "subagent_type=\"$critic\""
  done
}

@test "in-review skill documents the polyglot dispatch path" {
  assert_file_contains "$SKILL_FILE" "Polyglot"
  grep -qiE 'multiple|each critic|narrowed' "$SKILL_FILE"
}

@test "in-review skill does not document filesystem-probe-based detection" {
  run grep -E '(mix\.exs|Cargo\.toml|Package\.swift|\.luarc\.json) (exists|→|->)' "$SKILL_FILE"
  [ "$status" -ne 0 ]
}

# ====================================================================
# author pack dispatch (ST0052 WP05) -- the first non-code critic
# ====================================================================

@test "in-review skill dispatches the author pack to critic-author" {
  assert_file_contains "$SKILL_FILE" "critic-author"
  assert_file_contains "$SKILL_FILE" "subagent_type=\"critic-author\""
}

@test "in-review skill documents the D7 author/code-critic exclusion" {
  # author-only projects run no code critic; mixed projects run both on their
  # own subtrees. The note names D7 and the author-only case.
  assert_file_contains "$SKILL_FILE" "author-only"
  assert_file_contains "$SKILL_FILE" "D7"
}

# ====================================================================
# Pre-commit hook dispatch (real product behaviour)
# ====================================================================

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"

  # Stub `intent` that records every invocation and exits clean.
  STUB_BIN="$TEST_TEMP_DIR/stub-bin"
  CALL_LOG="$TEST_TEMP_DIR/intent-calls.log"
  mkdir -p "$STUB_BIN"
  cat > "$STUB_BIN/intent" << EOF
#!/bin/bash
echo "\$@" >> "$CALL_LOG"
exit 0
EOF
  chmod +x "$STUB_BIN/intent"
}

teardown() {
  rm -rf "$TEST_TEMP_DIR"
}

# Create a git repo that looks like an Intent project with the given
# languages JSON array, and run the shipped hook from inside it.
run_hook_with_languages() {
  local langs_json="$1"
  local repo="$TEST_TEMP_DIR/repo"
  mkdir -p "$repo/intent/.config"
  printf '{"languages":%s}\n' "$langs_json" > "$repo/intent/.config/config.json"
  git -C "$repo" init -q
  ( cd "$repo" && PATH="$STUB_BIN:$PATH" run_hook_inner )
}

run_hook_inner() {
  bash "$HOOK"
}

@test "hook dispatches one critic invocation per declared language" {
  run run_hook_with_languages '["elixir","rust"]'
  [ "$status" -eq 0 ]
  assert_file_exists "$CALL_LOG"
  grep -q '^critic elixir --staged --severity-min ' "$CALL_LOG" \
    || fail "no critic elixir invocation: $(cat "$CALL_LOG")"
  grep -q '^critic rust --staged --severity-min ' "$CALL_LOG" \
    || fail "no critic rust invocation: $(cat "$CALL_LOG")"
  [ "$(grep -c '^critic ' "$CALL_LOG")" -eq 2 ]
}

@test "hook with empty languages runs no critic and exits 0" {
  run run_hook_with_languages '[]'
  [ "$status" -eq 0 ]
  [ ! -f "$CALL_LOG" ] || [ "$(grep -c '^critic ' "$CALL_LOG" || true)" -eq 0 ]
}

@test "hook fails open outside an Intent project" {
  local repo="$TEST_TEMP_DIR/bare-repo"
  mkdir -p "$repo"
  git -C "$repo" init -q
  run bash -c "cd '$repo' && PATH='$STUB_BIN:$PATH' bash '$HOOK'"
  [ "$status" -eq 0 ]
  assert_output_contains "not inside an Intent project"
}

@test "hook fails open when the intent CLI is not on PATH" {
  local repo="$TEST_TEMP_DIR/no-cli-repo"
  mkdir -p "$repo/intent/.config"
  printf '{"languages":["shell"]}\n' > "$repo/intent/.config/config.json"
  git -C "$repo" init -q
  # Restrict PATH to core utils only -- no intent anywhere.
  run bash -c "cd '$repo' && PATH='/usr/bin:/bin' bash '$HOOK'"
  [ "$status" -eq 0 ]
  assert_output_contains "'intent' CLI not on PATH"
}

@test "hook blocks the commit when a critic reports findings" {
  # Stub exits 1 (findings at threshold) -> hook must exit 1.
  cat > "$STUB_BIN/intent" << EOF
#!/bin/bash
echo "\$@" >> "$CALL_LOG"
echo "[WARNING] IN-SH-TEST-901 at file.sh:1"
exit 1
EOF
  chmod +x "$STUB_BIN/intent"
  run run_hook_with_languages '["shell"]'
  [ "$status" -eq 1 ]
  assert_output_contains "commit blocked by findings"
}

@test "hook fails open on critic invocation error (exit 2)" {
  cat > "$STUB_BIN/intent" << EOF
#!/bin/bash
echo "\$@" >> "$CALL_LOG"
exit 2
EOF
  chmod +x "$STUB_BIN/intent"
  run run_hook_with_languages '["shell"]'
  [ "$status" -eq 0 ]
  assert_output_contains "invocation error (exit 2); fail-open"
}
