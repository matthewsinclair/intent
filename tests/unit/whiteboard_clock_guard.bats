#!/usr/bin/env bats
# lib/templates/hooks/whiteboard-clock-guard.sh -- the three clock checks.
#
# Four of these are FALSE-POSITIVE CONTROLS. A guard that blocks honest commits
# gets bypassed, and a bypassed guard is decoration -- so the controls matter as
# much as the catches, and the archive/pre-existing-breakage ones each encode a
# constraint that came out of measurement rather than reasoning.
#
# Three harness bugs were found writing these, every one producing a CONFIDENT
# PASS rather than an error, so each is now asserted against explicitly:
#   1. the guard was not executable and exited 126, read as "blocked";
#   2. state leaked between cases, so later fixtures were never tracked;
#   3. the archive fixture was never written (git clean removed its dir), so the
#      control passed having staged nothing -- and it was masking a real bug in
#      the guard's own exclude pathspec.
# Hence assert_guard's exit-code contract and the explicit fixture assertions.

load "../lib/test_helper.bash"

setup() {
  GUARD="${INTENT_PROJECT_ROOT}/lib/templates/hooks/whiteboard-clock-guard.sh"
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/clockguard-XXXXXX")" && pwd)"
  cd "$TEST_TEMP_DIR" || return 1
  git init -q .
  git config user.email t@t
  git config user.name t

  PAST="$(date -u -v-2H '+%Y-%m-%d %H:%M' 2>/dev/null || date -u -d '2 hours ago' '+%Y-%m-%d %H:%M')"
  PAST3="$(date -u -v-3H '+%Y-%m-%d %H:%M' 2>/dev/null || date -u -d '3 hours ago' '+%Y-%m-%d %H:%M')"
  NOW="$(date -u '+%Y-%m-%d %H:%M')"
  FUTURE="$(date -u -v+2H '+%Y-%m-%d %H:%M' 2>/dev/null || date -u -d '2 hours' '+%Y-%m-%d %H:%M')"

  mkdir -p intent/whiteboard/cc intent/whiteboard/ic/.history/20260814
  printf '# inbox: ic -> cc\n\n## (%sZ)\n\nhello\n' "$PAST" > intent/whiteboard/cc/inbox.ic.md
  git add -A
  git commit -qm base
}

teardown() {
  cd /
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# The guard's contract is exit 0 (clean) or exit 1 (blocked). ANY other code
# means it did not run -- 126 permission denied, 127 not found, 2 syntax error.
# Collapsing those into "blocked" is how a battery reports passes having
# executed nothing, which is what happened on the first run of these cases.
# `run` is mandatory here, not stylistic. A bare `out="$(bash "$GUARD")"` returns
# the guard's status, so under the test body's set -e the function aborts at the
# assignment and never reaches the exit-code check -- every BLOCK case failed
# and every PASS case passed, which reads exactly like "the guard never runs".
# This is the same defect already catalogued across ~46 unwrapped call sites in
# the estate: an exit-code drift aborts the test opaquely instead of failing an
# assertion.
assert_guard() {
  local expect="$1"
  run bash "$GUARD"
  case "$status" in
    0 | 1) ;;
    *)
      echo "guard exited $status -- it did not run:"
      echo "$output"
      return 1
      ;;
  esac
  if [ "$expect" = "BLOCK" ] && [ "$status" -ne 1 ]; then
    echo "expected BLOCK, guard allowed it"
    return 1
  fi
  if [ "$expect" = "PASS" ] && [ "$status" -ne 0 ]; then
    echo "expected PASS, guard blocked:"
    echo "$output"
    return 1
  fi
}

@test "clock guard: an honest stamp with a trailing Z passes" {
  printf '\n## (%sZ)\n\nfine\n' "$NOW" >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard PASS
}

@test "clock guard: check A blocks a stamp in the future" {
  printf '\n## (%sZ)\n\nlater\n' "$FUTURE" >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard BLOCK
}

@test "clock guard: check B blocks an entry heading with no Z" {
  printf '\n## (%s)\n\nunmarked\n' "$NOW" >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard BLOCK
}

@test "clock guard: check B blocks a heartbeat with a T separator and no Z" {
  printf -- '---\nnode: cc\nheartbeat_at: %sT%s\n---\n' "${PAST%% *}" "${PAST##* }" \
    > intent/whiteboard/cc/wip.md
  git add -A
  assert_guard BLOCK
}

@test "clock guard: check B blocks a heartbeat with a SPACE separator and no Z" {
  # The upstream original required `T` here, so this spelling went unchecked --
  # and both separators are in live use. The separator is not this guard's
  # business; the missing Z is, under either.
  printf -- '---\nnode: cc\nheartbeat_at: %s\n---\n' "$PAST" > intent/whiteboard/cc/wip.md
  git add -A
  assert_guard BLOCK
}

@test "clock guard: check C blocks an append that goes backwards" {
  printf '\n## (%sZ)\n\nbackwards\n' "$PAST3" >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard BLOCK
}

@test "clock guard: control -- .history/ replays old stamps and must not block" {
  # `clear` and `archive` re-add entries verbatim, so an archive commit
  # legitimately carries stamps hours or days old.
  mkdir -p intent/whiteboard/ic/.history/20260814
  printf '# archived\n\n## (2026-01-01 09:00)\n\nold entry, verbatim\n' \
    > intent/whiteboard/ic/.history/20260814/inbox.cc.md
  git add -A
  # Assert the fixture is REALLY staged. Without this the case passes having
  # staged nothing, and it was doing exactly that while hiding a live bug in the
  # guard's exclude pathspec.
  git diff --cached --name-only | grep -q '\.history/'
  assert_guard PASS
}

@test "clock guard: control -- prose quoting a future stamp is reporting, not offending" {
  # Nodes report this class to each other by quoting the bad stamp. A guard that
  # blocks the report makes the class harder to fix than to commit.
  printf '\n## (%sZ)\n\nyour heartbeat reads %sZ, which is ahead of now.\n' "$NOW" "$FUTURE" \
    >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard PASS
}

@test "clock guard: control -- pre-existing breakage must not wedge the file" {
  # Land breakage first, as history did, then append a correct entry. If check C
  # answered for stamps it did not add, every later commit to this file would be
  # blocked and the guard would have to be bypassed to work.
  printf '\n## (%s)\n\npre-existing unmarked and out of order\n' "$PAST3" \
    >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  git commit -qm "pre-existing breakage"
  printf '\n## (%sZ)\n\nmy new, correct entry\n' "$NOW" >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard PASS
}

@test "clock guard: control -- a project with no whiteboard is untouched" {
  rm -rf intent/whiteboard
  printf 'x\n' > unrelated.txt
  git add -A
  assert_guard PASS
}
