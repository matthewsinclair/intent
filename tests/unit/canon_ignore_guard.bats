#!/usr/bin/env bats
# lib/templates/hooks/canon-ignore-guard.sh -- a commit that adds an ignore rule
# reaching intent/.canon/ is refused, judged on the rules THE COMMIT CARRIES.
#
# ISSUE 0392 IS WHY THIS FILE EXISTS. The guard read its RULES from the worktree
# (`git check-ignore` consults the checkout's `.gitignore`) and its ATTRIBUTION
# from the index (`git diff --cached`), so whenever the staged and the checked-out
# `.gitignore` differed it compared line numbers across two documents. Laksa's dc
# measured the consequence: a staged `intent/.canon/` rule the worktree did not
# carry was never matched, and passed. On a shared tree the two differ routinely
# (`git commit --only`, a re-staging formatter), so the first cases below are
# that divergence in both directions, and each would have gone the wrong way.
#
# Every case runs in a throwaway repo and asserts SOMETHING WAS STAGED before
# believing an exit code: a guard over an empty commit returns 0 and is
# indistinguishable from a pass.

load "../lib/test_helper.bash"

setup() {
  GUARD="${INTENT_PROJECT_ROOT}/lib/templates/hooks/canon-ignore-guard.sh"
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/canonignore-XXXXXX")" && pwd)"
  cd "$TEST_TEMP_DIR" || return 1
  git init -q .
  git config user.email t@t
  git config user.name t
  mkdir -p intent/.canon/st
  printf '{}\n' > intent/.canon/st/ST0001.json
  printf 'target/\n' > .gitignore
  git add -A
  git commit -qm base
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# Run the guard over what is staged NOW. Its contract is exit 0 or 1; any other
# code means it did not run, and an empty index means it judged nothing.
assert_guard() { # $1 BLOCK|PASS
  local staged
  staged="$(git diff --cached --name-only | wc -l | tr -d ' ')"
  if [ "$staged" -eq 0 ]; then
    echo "HARNESS DEAD: nothing staged, so any exit code is meaningless"
    return 1
  fi
  run bash "$GUARD"
  case "$status" in
    0 | 1) ;;
    *)
      echo "guard exited $status -- it did not run:"
      echo "$output"
      return 1
      ;;
  esac
  if [ "$1" = "BLOCK" ] && [ "$status" -ne 1 ]; then
    echo "expected BLOCK, the guard allowed it:"
    echo "$output"
    return 1
  fi
  if [ "$1" = "PASS" ] && [ "$status" -ne 0 ]; then
    echo "expected PASS, the guard refused it:"
    echo "$output"
    return 1
  fi
}

# --- the divergence 0392 names, in both directions -----------------------------

@test "0392: a rule reaching canon that is STAGED but not in the worktree is refused" {
  printf 'target/\nintent/.canon/\n' > .gitignore
  git add .gitignore
  printf 'target/\n' > .gitignore
  assert_guard BLOCK
  [[ "$output" == *".gitignore:2:intent/.canon/"* ]]
}

@test "0392: a rule only in the WORKTREE, at the line number the commit adds, is not blamed" {
  printf 'target/\nbuild/\n' > .gitignore
  git add .gitignore
  printf 'target/\nintent/.canon/\n' > .gitignore
  assert_guard PASS
}

# --- the rule itself -------------------------------------------------------------

@test "an added intent/.*/ rule that the worktree also carries is refused" {
  printf 'target/\nintent/.*/\n' > .gitignore
  git add .gitignore
  assert_guard BLOCK
  [[ "$output" == *"intent/.*/"* ]]
}

@test "an added rule in a NESTED intent/.gitignore reaching canon is refused" {
  printf '.canon/\n' > intent/.gitignore
  git add intent/.gitignore
  assert_guard BLOCK
  [[ "$output" == *"intent/.gitignore:1:.canon/"* ]]
}

@test "a rule that reaches only a FUTURE canon path is refused" {
  printf 'target/\nintent/.canon/st/ST9*\n' > .gitignore
  git add .gitignore
  assert_guard BLOCK
}

@test "an added rule that reaches no canon path passes" {
  printf 'target/\nintent/.cache/\n' > .gitignore
  git add .gitignore
  assert_guard PASS
}

@test "an INHERITED rule reaching canon is reported and never failed on" {
  printf 'intent/.canon/\ntarget/\n' > .gitignore
  git add .gitignore
  git commit -qm "the breakage arrives"
  printf 'intent/.canon/\ntarget/\nbuild/\n' > .gitignore
  git add .gitignore
  assert_guard PASS
  [[ "$output" == *"EXISTING ignore rule already reaches"* ]]
}

@test "a commit that touches no ignore file passes" {
  printf 'x\n' > notes.txt
  git add notes.txt
  assert_guard PASS
}
