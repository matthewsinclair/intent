#!/usr/bin/env bats
# The Stop hook's /in-finish reminder (issue 0576) is a JSON `systemMessage`,
# which Claude Code shows in the transcript, and never plain stdout, which the
# docs do not say reaches anyone for a Stop hook. Exit 0 on every path.

bats_require_minimum_version 1.5.0
load "../lib/test_helper.bash"

SCRIPT="${INTENT_PROJECT_ROOT}/lib/templates/.claude/scripts/session-finish.sh"

fin_repo() {
  FIN_DIR="$BATS_TEST_TMPDIR/repo"
  mkdir -p "$FIN_DIR"
  git -C "$FIN_DIR" -c init.defaultBranch=main init -q
  printf 'x\n' > "$FIN_DIR/a.txt"
  git -C "$FIN_DIR" add a.txt
  git -C "$FIN_DIR" -c user.name=t -c user.email=t@invalid -c commit.gpgsign=false commit -qm fixture
}

@test "a dirty tree gets the counted reminder as a JSON systemMessage" {
  command -v jq >/dev/null || skip "jq not on PATH"
  fin_repo
  printf 'y\n' > "$FIN_DIR/dirty.txt"
  run --separate-stderr env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE CLAUDE_PROJECT_DIR="$FIN_DIR" bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  run jq -er '.systemMessage' <<< "$output"
  [ "$status" -eq 0 ] || fail "stdout is not a JSON systemMessage: $output"
  [[ "$output" == *"1 uncommitted path(s)"*"/in-finish"* ]]
}

@test "a clean tree says nothing" {
  fin_repo
  run --separate-stderr env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE CLAUDE_PROJECT_DIR="$FIN_DIR" bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

@test "outside a git tree the plain reminder is a JSON systemMessage too" {
  command -v jq >/dev/null || skip "jq not on PATH"
  mkdir -p "$BATS_TEST_TMPDIR/nogit"
  run --separate-stderr env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE GIT_CEILING_DIRECTORIES="$BATS_TEST_TMPDIR" CLAUDE_PROJECT_DIR="$BATS_TEST_TMPDIR/nogit" bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  run jq -er '.systemMessage' <<< "$output"
  [ "$status" -eq 0 ] || fail "stdout is not a JSON systemMessage: $output"
  [[ "$output" == "Session wrap-up reminder: run /in-finish"* ]]
}
