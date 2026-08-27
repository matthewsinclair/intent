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
  # +2 MINUTES, AND THE 2 IS CHOSEN RATHER THAN CONVENIENT. It must be small
  # enough to discriminate the tolerance (at +2min BSD drift is exactly 120, so
  # `-gt 120` is FALSE and the guard as shipped passed it, while `-gt 0` blocks)
  # and large enough not to flake: +1 minute would roll to drift 0 whenever the
  # wall minute ticks between writing the fixture and the guard reading its
  # clock, and spuriously PASS.
  NEAR_FUTURE="$(date -u -v+2M '+%Y-%m-%d %H:%M' 2>/dev/null || date -u -d '2 minutes' '+%Y-%m-%d %H:%M')"
  # Date-only and unambiguously ahead, for the Decisions-surface arms. A
  # date-only stamp is read as MIDNIGHT, so "tomorrow" is the smallest value
  # that is future under both date(1) flavours regardless of the wall clock.
  FUTURE_DATE="$(date -u -v+1d '+%Y-%m-%d' 2>/dev/null || date -u -d '1 day' '+%Y-%m-%d')"

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

# THIS TEST EXISTS BECAUSE THE ONE ABOVE CANNOT SEE THE TOLERANCE, AND NOR COULD
# ANY OTHER CHECK-A TEST IN THIS FILE. `FUTURE` is +2 hours, so it blocks at any
# tolerance below 7200s: the whole suite was equally green at 120s, at 0, and at
# an hour. A green from a suite that cannot distinguish the setting is not
# evidence the setting is right.
#
# AND THE UNDISCRIMINATED BAND IS WHERE THE REAL DEFECTS LIVE. Both known
# instances on this estate are exactly +60s; `-gt` is strict, so the shipped 120
# missed them and so would 60. Measured across the fleet's unguarded board
# history: 299 violations sit in the 1-2 minute band, a sixth of all of them, and
# it is the only band the old tolerance could not see.
#
# So this pins the RULING (0, authority vc, 2026-08-27), not merely the code:
# raising the tolerance back to anything at or above 120 turns this red.
@test "clock guard: check A blocks a stamp two minutes ahead -- the band the old tolerance could not see" {
  printf '\n## (%sZ)\n\ntwo minutes ahead\n' "$NEAR_FUTURE" >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard BLOCK
}

@test "clock guard: a future-dated Decisions bullet blocks -- the third stamp surface" {
  printf '\n- (%s) a decision dated in the future\n' "$FUTURE_DATE" >> intent/whiteboard/cc/wip.md
  git add -A
  assert_guard BLOCK
}

# THE NEXT TWO PIN A SCOPE DECISION RATHER THAN A BEHAVIOUR, AND THEY ARE GREEN
# BY DESIGN. Two dated forms are live on real boards and deliberately NOT read;
# without these, a later widening looks like a fix rather than a change of reach,
# which is exactly how this guard came to claim coverage it never had.
#
# If either is widened ON PURPOSE, these fail and the failure is the prompt to
# say so in the comment above `STAMP_LINES_RE`. That is the point of them.

@test "clock guard: scope -- a dated ### heading is NOT read (Intent 5, Lamplight 4, filed not fixed)" {
  # Lamplight's hv node dates every ruling this way and carries no dated bullet
  # at all, so its rulings record is unscanned. Found by lamplight-vc, who
  # measured it and argued AGAINST widening: a broader pattern is what made
  # check A wrong in the first place.
  printf '\n### %s -- ruled in chat\n' "$FUTURE_DATE" >> intent/whiteboard/cc/wip.md
  git add -A
  assert_guard PASS
}

@test "clock guard: scope -- an indented or decorated bullet is NOT read (anchored on purpose)" {
  # `   - _(2026-08-25 15:44Z, ...` exists on Lamplight's hv board. Relaxing the
  # anchor to tolerate leading whitespace would start reading dated bullets in
  # arbitrary nested prose lists, which is the PORT 2 hazard -- blocking a node
  # for reporting someone else's bad stamp.
  printf '\n   - _(%s, a nested note)\n' "$FUTURE_DATE" >> intent/whiteboard/cc/wip.md
  git add -A
  assert_guard PASS
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

@test "clock guard: control -- a Re: anchor points backwards and must not block" {
  # The documented heading is `## (<stamp>)   [Re: <prior-anchor>]`, and the
  # anchor names the EARLIER entry being replied to, so it is older than the
  # entry carrying it -- always. Reading every date on the line makes every
  # threaded reply look like an inbox running backwards. This guard blocked its
  # own announcement commit that way, on its first real use.
  printf '\n## (%sZ)   Re: %s\n\nthreaded reply\n' "$NOW" "$PAST3" \
    >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard PASS
}

@test "clock guard: a Re: anchor does not mask a genuinely out-of-order entry" {
  # The complement. Narrowing check C to the opening stamp must not make it
  # blind: an entry whose OWN stamp goes backwards still blocks, anchor or not.
  printf '\n## (%sZ)   Re: %sZ\n\nbackwards despite the anchor\n' "$PAST3" "$PAST3" \
    >> intent/whiteboard/cc/inbox.ic.md
  git add -A
  assert_guard BLOCK
}

@test "clock guard: control -- a project with no whiteboard is untouched" {
  rm -rf intent/whiteboard
  printf 'x\n' > unrelated.txt
  git add -A
  assert_guard PASS
}

# --- THE SIGPIPE RACE IN CHECK C's "did THIS COMMIT add it" FILTER ------------
#
# That filter was `printf ... | grep -qxF`. `grep -q` exits on the first match,
# `printf` then takes SIGPIPE, and this file's `set -o pipefail` promotes 141 to
# the PIPELINE's status -- so the test read FALSE and the finding was discarded
# as inherited breakage. The filter is only REACHED after a violation has
# already been detected, so the guard was sound whenever there was nothing to
# catch and unsound exactly when there was.
#
# THESE TWO CASES ARE A MATCHED PAIR AND NEITHER IS SUFFICIENT ALONE. The catch
# is red on the pipeline form and green on the herestring. The control is green
# on BOTH, and exists to refuse the careless fix: "the pipeline now always
# succeeds" also blocks the violation, while wedging every commit that merely
# inherits breakage. A FIX AND A MUTE ARE INDISTINGUISHABLE FROM THE CATCH CASE.
#
# THE PAYLOAD SIZE IS LOAD-BEARING, NOT DECORATION. Below the pipe buffer the
# write lands whole, printf never sees SIGPIPE, and both forms agree -- so a
# fixture that small passes on the BROKEN guard and proves nothing. Measured on
# this guard: ~1.2k added stamps agreed 3/3 on both forms; ~6k lost 3/3 on the
# pipeline form and 0/3 on the herestring.

emit_stamps() { # $1 count  $2 starting minute offset -- ascending, valid, Z-marked
  seq 0 "$(( $1 - 1 ))" | awk -v b="$2" '{
    m = b + $1
    printf "\n## (2026-07-%02d %02d:%02dZ)\n\nx\n", 1 + int(m / 1440), int(m / 60) % 24, m % 60
  }'
}

assert_inbox_staged() { # $1 path -- the fixture must really be staged
  if ! git diff --cached --name-only | grep -qxF -- "$1"; then
    echo "fixture $1 is NOT staged -- this case would pass having tested nothing"
    git status --short
    return 1
  fi
}

@test "clock guard: check C blocks a backwards append the SIGPIPE race used to drop" {
  emit_stamps 6000 1440 > intent/whiteboard/cc/inbox.vc.md
  git add -A
  git commit -qm 'base: a long inbox'

  # One commit appends 6000 more good stamps AND one that goes backwards. The
  # backwards stamp sorts FIRST in the added set, so grep matches at byte 0 --
  # the position that maximises the race, and the one a real board always
  # produces, because a fold rewrites a file from the top.
  emit_stamps 6000 20000 >> intent/whiteboard/cc/inbox.vc.md
  emit_stamps 1 0 >> intent/whiteboard/cc/inbox.vc.md
  git add -A
  assert_inbox_staged intent/whiteboard/cc/inbox.vc.md
  assert_guard BLOCK
  [[ "$output" == *'[C order]'* ]]
}

@test "clock guard: control -- inherited breakage at the same scale still passes" {
  # THE MUTE CONTROL. Same file, same payload, same guard; the only difference
  # is that this commit did not ADD the backwards stamp. A filter rewritten to
  # always succeed passes the case above and fails this one.
  emit_stamps 6000 1440 > intent/whiteboard/cc/inbox.vc.md
  emit_stamps 1 0 >> intent/whiteboard/cc/inbox.vc.md
  git add -A
  git commit -qm 'base: breakage already landed'

  emit_stamps 6000 20000 >> intent/whiteboard/cc/inbox.vc.md
  git add -A
  assert_inbox_staged intent/whiteboard/cc/inbox.vc.md
  assert_guard PASS
}
