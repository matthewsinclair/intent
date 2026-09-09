#!/usr/bin/env bats
# lib/templates/hooks/commit-msg.sh -- the attribution guard matches TRAILERS.
#
# The guard refuses a commit message carrying AI/Claude attribution. Its whole
# contract is that it matches the TRAILER BLOCK and never the prose body: a
# commit message is prose by nature, and the estate had already ruled this way
# once, in `whiteboard-header-guard.sh`, which reads header blocks and never
# prose because "scanning prose would make reporting the defect an offence".
#
# THIS FILE EXISTS BECAUSE THE GUARD SHIPPED WITHOUT IT AND WAS WRONG.
# The first version carried two UNANCHORED prose patterns (`Generated
# with.*Claude`, a robot emoji). They refused any message describing the rule --
# including the commit documenting the guard and the commit fixing it, whose
# printed remedy then told the author to delete their own sentence. dc found it
# by reading, because nothing here could find it by running.
#
# MOST OF THIS FILE IS FALSE-POSITIVE CONTROLS, and that ratio is the point.
# The catches prove the guard fires; only the controls prove it fires at the
# right thing. A guard that blocks honest commits gets bypassed, and a bypassed
# guard is decoration.
#
# THE CONTROLS ARE MUTATION-TESTED at the bottom: the anchors and the trailer
# colon are put back the way they were, and every prose control must flip to a
# refusal. A control that stays green when the mechanism it covers is removed
# never reached the branch it claims to cover.

load "../lib/test_helper.bash"

setup() {
  GUARD="${INTENT_PROJECT_ROOT}/lib/templates/hooks/commit-msg.sh"
  DISPATCH="${INTENT_PROJECT_ROOT}/.githooks/commit-msg"
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/commitmsg-XXXXXX")" && pwd)"
  MSG="${TEST_TEMP_DIR}/COMMIT_EDITMSG"
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

write_msg() { printf '%s\n' "$1" > "$MSG"; }

# The guard's contract is exit 0 (clean) or exit 1 (refused). ANY other code
# means it did not run -- 126 permission denied, 127 not found, 2 syntax error.
# Collapsing those into "refused" is how a battery reports passes having
# executed nothing.
assert_guard() { # $1 BLOCK|PASS  $2 optional guard path (defaults to GUARD)
  local expect="$1" g="${2:-$GUARD}"
  run bash "$g" "$MSG"
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
    echo "FALSE POSITIVE -- guard refused an honest commit:"
    echo "$output"
    return 1
  fi
}

# --- the catches ------------------------------------------------------------

@test "a Claude-Session trailer is refused, and the offending line is printed" {
  write_msg 'wb(vc): a thing

Claude-Session: 01QdJZysgcMJ1SEeyo7wAUpE'
  assert_guard BLOCK
  # It NEVER edits the message, so the line has to be shown or the author is
  # left to guess which one it meant.
  [[ "$output" == *'Claude-Session: 01QdJZysgcMJ1SEeyo7wAUpE'* ]]
}

@test "the standard tool footer is refused on its Co-Authored-By trailer" {
  write_msg 'feat(x): a thing

Co-Authored-By: Claude <noreply@anthropic.com>'
  assert_guard BLOCK
}

@test "a Co-Authored-By naming anthropic rather than claude is refused" {
  write_msg 'feat(x): a thing

Co-Authored-By: Someone <someone@anthropic.com>'
  assert_guard BLOCK
}

@test "an indented trailer is refused -- the anchor allows leading whitespace" {
  write_msg 'feat(x): a thing

  Claude-Session: abc-123'
  assert_guard BLOCK
}

@test "trailer matching is case-insensitive" {
  write_msg 'feat(x): a thing

claude-session: abc-123'
  assert_guard BLOCK
}

# --- the false-positive controls -- the majority, deliberately --------------

@test "CONTROL: prose describing the guard is not attribution" {
  # This is dc's m2, the case that found the defect.
  write_msg 'fix(commit-msg): scope the guard to trailers

The hook refuses any message generated with Claude Code, and refuses
Co-Authored-By trailers naming Claude.'
  assert_guard PASS
}

@test "CONTROL: the commit message that FIXES this guard is writable" {
  # If this reddens, the guard has again made its own repair uncommittable.
  write_msg 'fix(commit-msg): the prose patterns refuse a message describing the guard

The generated with Claude and robot-emoji patterns were unanchored, so a
commit explaining the class was refused at line 3.'
  assert_guard PASS
}

@test "CONTROL: a message quoting the CLAUDE.md prohibition verbatim passes" {
  # This is 6816e1e94's shape -- the commit an unanchored census counted as a
  # violation for STATING the rule.
  write_msg 'docs: state the attribution rule

CLAUDE.md says: DO NOT ADD CLAUDE TO GIT COMMITS. EVER. No Co-Authored-By
lines, no Claude signatures, no AI attribution in commit messages.'
  assert_guard PASS
}

@test "CONTROL: the trailer colon is what separates a trailer from a sentence" {
  write_msg 'wb(vc): routed it

Co-Authored-By trailers naming Claude are the measured class.'
  assert_guard PASS
}

@test "CONTROL: a mid-sentence robot emoji and generated-with phrase pass" {
  write_msg 'docs(hooks): describe the footer

The tool used to append a robot emoji line generated with Claude Code
above the trailer.'
  assert_guard PASS
}

@test "CONTROL: a real trailer inside a git comment line is not committed" {
  # git strips `#` lines before the message lands, so matching them would
  # refuse every commit made from a template that documents the rule.
  write_msg 'feat(x): a thing

# Claude-Session: abc-123'
  assert_guard PASS
}

@test "CONTROL: an ordinary clean message passes" {
  write_msg 'fix(thing): ordinary message

Nothing to see.'
  assert_guard PASS
}

# --- the contract arms ------------------------------------------------------

@test "no message file is a REFUSAL, never a pass" {
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  [[ "$output" == *'refusing rather than passing'* ]]
}

@test "the dispatcher refuses when the guard body is absent" {
  # ABSENCE IS A REFUSAL. A guard that no-ops when its body is missing is
  # indistinguishable from one that ran and found nothing.
  cd "$TEST_TEMP_DIR" || return 1
  git init -q .
  cp "$DISPATCH" ./hook
  write_msg 'feat(x): a thing'
  run bash ./hook "$MSG"
  [ "$status" -eq 1 ]
  [[ "$output" == *'GUARD ABSENT'* ]]
}

# --- the mutation: prove the controls reach the branch ----------------------

@test "MUTATION: restoring the unanchored prose patterns reddens the controls" {
  local mutant="${TEST_TEMP_DIR}/mutant.sh"
  # Put the original patterns line back, prose patterns and all. The delimiter
  # is `|` and the payload contains none, so it cannot collide and truncate --
  # the mutant is then asserted non-empty, valid, and still catching a real
  # trailer, because an empty file parses and would pass every control silently.
  sed "s|^patterns=.*|patterns='^[[:space:]]*Claude-Session:\|^[[:space:]]*Co-[Aa]uthored-[Bb]y:.*([Cc]laude\|[Aa]nthropic)\|[Gg]enerated with.*[Cc]laude'|" \
    "$GUARD" > "$mutant"
  [ -s "$mutant" ]
  bash -n "$mutant"
  grep -q 'Generated with' "$mutant" || { echo "mutation did not apply"; return 1; }

  # The mutant must still catch what the real guard catches, or it is broken
  # rather than mutated.
  write_msg 'feat(x): a thing

Claude-Session: abc-123'
  assert_guard BLOCK "$mutant"

  # And every prose control must now FLIP to a refusal. That flip is what
  # proves those controls exercise the anchoring, rather than passing for some
  # unrelated reason.
  write_msg 'fix(commit-msg): scope the guard to trailers

The hook refuses any message generated with Claude Code.'
  assert_guard BLOCK "$mutant"

  write_msg 'docs(hooks): describe the footer

The tool appended a line generated with Claude Code above the trailer.'
  assert_guard BLOCK "$mutant"
}
