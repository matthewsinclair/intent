#!/usr/bin/env bats
# lib/templates/hooks/whiteboard-header-guard.sh -- the header block is NOT YAML.
#
# The guard refuses a commit that YAML-ESCAPES a value in a whiteboard header
# block. It ships on ONE observed instance, which is not this project's usual
# bar, and the reason is in the guard's own header: under the first diagnosis (a
# formatter quirk) one instance is evidence of RARITY, and under the measured
# one (a node that knows YAML doing the correct YAML thing) it is one
# OBSERVATION of the default behaviour of every competent node.
#
# MOST OF THIS FILE IS FALSE-POSITIVE CONTROLS, and that ratio is deliberate.
# The catches prove the guard fires; only the controls prove it fires at the
# right thing. A guard that blocks honest commits gets bypassed, and a bypassed
# guard is decoration.
#
# EVERY CONTROL BELOW WAS MUTATION-TESTED: the mechanism that makes it pass was
# removed, and the case flipped to a refusal. A control that stays green when
# its own exemption is deleted never reached the branch it claims to cover --
# which is what "the fixture must reach the branch" means, and it is not
# hypothetical here: the first mutation harness reported two ESCAPES that were
# really a BROKEN GUARD (a blanked pathspec element, and a sed delimiter
# collision that truncated the script to nothing). An empty file parses. Hence
# assert_guard's exit-code contract, and the explicit fixture assertions on the
# two controls whose fixture can silently fail to exist.

load "../lib/test_helper.bash"

setup() {
  GUARD="${INTENT_PROJECT_ROOT}/lib/templates/hooks/whiteboard-header-guard.sh"
  HOOK="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit.sh"
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/headerguard-XXXXXX")" && pwd)"
  cd "$TEST_TEMP_DIR" || return 1
  git init -q .
  git config user.email t@t
  git config user.name t
}

teardown() {
  cd /
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# A board with the given header line spliced in. Every other key is well-formed,
# so a refusal can only be about the line under test.
write_board() { # $1 path  $2 the header line under test
  mkdir -p "$(dirname "$1")"
  {
    printf -- '---\n'
    printf 'node: dc\n'
    printf 'name: DevX Claude\n'
    printf 'status: active\n'
    printf 'heartbeat_at: 2026-08-16 11:00Z\n'
    printf '%s\n' "$2"
    printf 'claims: [ST0056]\n'
    printf -- '---\n'
    printf '\n# DevX Claude (dc)\n'
  } > "$1"
}

# The guard's contract is exit 0 (clean) or exit 1 (blocked). ANY other code
# means it did not run -- 126 permission denied, 127 not found, 2 syntax error.
# Collapsing those into "blocked" is how a battery reports passes having
# executed nothing. `run` is mandatory, not stylistic: a bare command assignment
# returns the guard's status and aborts the test body under set -e before the
# assertion is ever reached.
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
    echo "expected BLOCK, guard allowed it. output:"
    echo "$output"
    return 1
  fi
  if [ "$expect" = "PASS" ] && [ "$status" -ne 0 ]; then
    echo "FALSE POSITIVE -- guard blocked an honest commit:"
    echo "$output"
    return 1
  fi
}

# A fixture that was never staged makes every control pass having tested
# nothing. Asserted explicitly on the controls where it can happen silently.
assert_staged() { # $1 path
  local staged
  staged="$(git diff --cached --name-only -- "$1")"
  if [ -z "$staged" ]; then
    echo "fixture $1 is NOT staged -- this case would pass having tested nothing"
    git status --short
    return 1
  fi
}

# --- the catches ------------------------------------------------------------

@test "a YAML double-quote escape is refused, and the repair is printed" {
  write_board intent/whiteboard/dc/wip.md 'focus: "the \"counted\" body is the SENT body"'
  git add -A
  assert_guard BLOCK
  # The repair is the whole point: the guard never auto-corrects, so the fix has
  # to be a copy-paste or the node is left to invent one.
  [[ "$output" == *'should: focus: "the "counted" body is the SENT body"'* ]]
}

@test "a YAML single-quote escape is refused, and the repair is printed" {
  write_board intent/whiteboard/dc/wip.md "focus: 'ic''s ruling stands'"
  git add -A
  assert_guard BLOCK
  # Single-quoted delimiters are converted to the double-quoted form the reader
  # actually strips, and the doubled apostrophe collapses to one.
  [[ "$output" == *'should: focus: "ic'"'"'s ruling stands"'* ]]
}

@test "any header key is covered, not just focus" {
  write_board intent/whiteboard/dc/wip.md 'role: "the \"worker\" node"'
  git add -A
  assert_guard BLOCK
  [[ "$output" == *'role:'* ]]
}

@test "the offending board is named" {
  write_board intent/whiteboard/vc/wip.md "focus: 'it''s here'"
  git add -A
  assert_guard BLOCK
  [[ "$output" == *'intent/whiteboard/vc/wip.md'* ]]
}

# --- the false-positive controls --------------------------------------------

@test "FP: literal quotes pass -- that is the CORRECT form" {
  # The exact thing the protocol tells nodes to write. If this ever blocks, the
  # guard is telling every node to break the rule it enforces.
  write_board intent/whiteboard/dc/wip.md 'focus: "the "counted" body is the SENT body"'
  git add -A
  assert_guard PASS
}

@test "FP: prose quoting the defect passes -- reporting it is not an offence" {
  # Nodes report this class to each other by QUOTING it. Scanning below the
  # fence would make an inbox entry saying "your board rendered ic''s" a
  # blockable offence, which is the clock guard's PORT 2 lesson in a second file.
  write_board intent/whiteboard/dc/wip.md 'focus: "clean"'
  {
    printf 'Your board rendered as: %s\n' "focus: 'ic''s ruling'"
    printf 'and the other form: %s\n' 'focus: "a \"quoted\" thing"'
  } >> intent/whiteboard/dc/wip.md
  git add -A
  assert_guard PASS
}

@test "FP: an archived board is out of scope" {
  # An archive replays an old header verbatim, so covering .history/ refuses the
  # protocol's own housekeeping over a historical record -- and the harm this
  # guard exists to prevent (ws list rendering ic''s) does not exist there.
  #
  # THE EXCLUDE IS LOAD-BEARING AND ITS SHAPE WAS MEASURED, NOT READ: a git
  # pathspec wildcard is matched against the WHOLE path, so
  # `intent/whiteboard/*/wip.md` CROSSES slashes. On the Intent repo that
  # pathspec alone matched 21 files, SIXTEEN of them archives; with the exclude,
  # 5. The clock guard's port lost the same exclude and its control did not
  # notice, because that control's fixture was itself missing -- hence
  # assert_staged below.
  write_board intent/whiteboard/dc/.history/20260815/wip.md "focus: 'ic''s ruling stands'"
  git add -A
  assert_staged intent/whiteboard/dc/.history/20260815/wip.md
  assert_guard PASS
}

@test "FP: inherited breakage does not wedge the next commit" {
  # Otherwise a pre-existing escaped focus: would block every future heartbeat
  # commit on that board, and a guard that must be bypassed to work is a guard
  # nobody keeps. Same principle as the clock guard's check C.
  write_board intent/whiteboard/dc/wip.md "focus: 'ic''s ruling stands'"
  git add -A
  git commit -qm 'pre-existing escaped header'

  # Touch a DIFFERENT line. The escaped focus: is still in the staged blob.
  sed 's/^status: active$/status: paused/' intent/whiteboard/dc/wip.md > tmp.md
  mv tmp.md intent/whiteboard/dc/wip.md
  git add -A
  assert_staged intent/whiteboard/dc/wip.md
  assert_guard PASS
}

@test "FP: a file with no header block is not scanned as one" {
  # An unanchored extractor scans the WHOLE FILE when there is no header, which
  # turns every line of prose into a candidate.
  mkdir -p intent/whiteboard/dc
  printf 'not a header\nfocus: %s\n' "'ic''s'" > intent/whiteboard/dc/wip.md
  git add -A
  assert_guard PASS
}

@test "FP: an inbox is not header-bearing" {
  mkdir -p intent/whiteboard/dc
  printf '# inbox: vc -> dc\n\nfocus: %s\n' "'ic''s'" > intent/whiteboard/dc/inbox.vc.md
  git add -A
  assert_guard PASS
}

@test "FP: a project with no whiteboard is not this guard's business" {
  # Opt-in by directory presence -- the criterion the whole change ships under.
  mkdir -p src
  printf 'x\n' > src/a.txt
  git add -A
  assert_guard PASS
}

# --- the wiring -------------------------------------------------------------

# THE TWO GUARD-DISPATCH TESTS THAT USED TO END THIS FILE HAVE MOVED to
# `tests/unit/guard_dispatch.bats`, 2026-08-20. They asserted something about
# EVERY guard from inside a file named for ONE, and their population was the
# glob `whiteboard-*-guard.sh` -- which matched 2 while the roster carried 4,
# blind to precisely the two guards that had never run in this repository.
# Widened, repointed at `pre-commit-guards.sh`, and named for the property.

# --- THE SIGPIPE RACE IN THE "did THIS COMMIT add it" FILTER ------------------
#
# Same defect as the clock guard's check C, same remedy, and the same matched
# pair. The filter was `printf ... | grep -qxF`, reached ONLY after the `case`
# had already matched an escape form and computed the repair -- so a lost race
# threw away a finding the guard had made, and called it inherited breakage.
#
# THE HEADER SITS AT THE TOP OF A BOARD, so on any whole-board rewrite the
# violating line matches within the first few lines of a very long added set:
# decision at byte 0, which is maximum exposure. That is the shape a fold
# produces, not a contrivance.

big_body() { # $1 path -- append a realistic whole-board payload
  seq 1 20000 | awk '{ print "xxxxxxx" }' >> "$1"
}

@test "a large board does not let an escaped header slip past the added-line filter" {
  write_board intent/whiteboard/dc/wip.md 'focus: "he said \"yes\" today"'
  big_body intent/whiteboard/dc/wip.md
  git add -A
  assert_staged intent/whiteboard/dc/wip.md
  assert_guard BLOCK
  # The repair still has to print, or the node is left to invent one.
  [[ "$output" == *'should: focus: "he said "yes" today"'* ]]
}

@test "FP: inherited breakage at the same scale still passes" {
  # THE MUTE CONTROL, and it is the half a careless fix fails. The escaped
  # header is already committed; this commit adds a large body and nothing else.
  write_board intent/whiteboard/dc/wip.md 'focus: "he said \"yes\" today"'
  git add -A
  git commit -qm 'pre-existing escaped header'

  big_body intent/whiteboard/dc/wip.md
  git add -A
  assert_staged intent/whiteboard/dc/wip.md
  assert_guard PASS
}
