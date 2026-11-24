#!/usr/bin/env bats
# The Stop hook's line (issue 0576) is a JSON `systemMessage`, which Claude
# Code shows in the transcript, and never plain stdout, which the docs do not
# say reaches anyone for a Stop hook. Exit 0 on every path. Claude Code runs
# the hook at the end of every turn, so the line names the uncommitted paths
# and gives no instruction (issue I6dd6e43).

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

@test "a dirty tree gets its uncommitted paths as a JSON systemMessage, with no instruction" {
  require_tool jq "the systemMessage, which is parsed with jq," || return 1
  fin_repo
  printf 'y\n' > "$FIN_DIR/dirty.txt"
  run --separate-stderr env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE CLAUDE_PROJECT_DIR="$FIN_DIR" bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  run jq -er '.systemMessage' <<< "$output"
  [ "$status" -eq 0 ] || fail "stdout is not a JSON systemMessage: $output"
  [ "$output" = "1 uncommitted path(s): dirty.txt." ] || fail "unexpected line: $output"
}

@test "past the shown limit the line counts the rest, and an odd path stays valid JSON" {
  require_tool jq "the systemMessage, which is parsed with jq," || return 1
  fin_repo
  local shown_max extra=2 i
  shown_max="$(sed -n 's/^shown_max=\([0-9][0-9]*\)$/\1/p' "$SCRIPT")"
  [ -n "$shown_max" ] || fail "the script names no shown_max"
  printf 'q\n' > "$FIN_DIR/a\"quote.txt"
  for ((i = 1; i < shown_max + extra; i++)); do printf 'z\n' > "$FIN_DIR/p$i.txt"; done
  run --separate-stderr env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE CLAUDE_PROJECT_DIR="$FIN_DIR" bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  run jq -er '.systemMessage' <<< "$output"
  [ "$status" -eq 0 ] || fail "stdout is not a JSON systemMessage: $output"
  [[ "$output" == "$((shown_max + extra)) uncommitted path(s): "*", and $extra more." ]] || fail "unexpected line: $output"
  [[ "$output" == *'a\"quote.txt'* ]] || fail "the quoted path is not named: $output"
}

@test "a clean tree says nothing" {
  fin_repo
  run --separate-stderr env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE CLAUDE_PROJECT_DIR="$FIN_DIR" bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}

@test "outside a git tree it says nothing" {
  mkdir -p "$BATS_TEST_TMPDIR/nogit"
  run --separate-stderr env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE GIT_CEILING_DIRECTORIES="$BATS_TEST_TMPDIR" CLAUDE_PROJECT_DIR="$BATS_TEST_TMPDIR/nogit" bash "$SCRIPT" < /dev/null
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}
