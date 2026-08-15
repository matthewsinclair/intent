#!/usr/bin/env bats
# `int prepush` decides from THIS push's range, not from the tracked remote.
#
# THE DEFECT, measured by cc 2026-08-15. The gate printed `no native/ or
# build-manifest change in this push` on a push whose diff was 14 files under
# `native/` and `schema/`. The grep was right; the RANGE was wrong. It computed
# `git diff --name-only @{upstream}...HEAD`, and `@{upstream}` names exactly ONE
# remote -- while the hook fires for a push to EITHER, and this project's
# standing instruction is to push both.
#
# So after `git push upstream main` succeeds, `upstream/main` IS HEAD, and the
# `git push local main` that follows carries every commit while computing an
# EMPTY range. **The second push of the standard two-push sequence was ungated
# by construction**, and it failed in the direction nobody watches: quietly,
# exiting 0, with a confident message that said a true thing about the wrong
# question.
#
# HOW THE ASSERTION AVOIDS THE 16s BUILD. The gate's expensive half clones HEAD
# and cold-builds it. What is under test here is the DECISION, not the build, so
# every case runs with `cargo` off PATH. That splits the two outcomes cleanly and
# instantly:
#
#   skipped -> exit 0, prints `no native/ or build-manifest change`
#   engaged -> exit non-zero, dies `cargo not on PATH`
#
# `setup` asserts that lever actually took. A fixture whose lever silently fails
# reports "nothing failed", which is indistinguishable from a test that does not
# check -- so if cargo is reachable on the trimmed PATH these SKIP loudly rather
# than quietly cloning and passing for the wrong reason.
#
# MUTATION-PROVEN, and the run CORRECTED this file rather than confirming it.
# The mutation is the runner as it stood at HEAD, in a sacrificial worktree,
# against this suite. Four tests discriminate; four pass under the defect:
#
#   1 gated at the remote that is behind      FAILS -- the empty range
#   2 answer follows each remote's position   FAILS -- only after the rewrite below
#   5 new ref on the remote                   FAILS -- stdin ignored entirely
#   7 by hand, furthest-behind remote         FAILS -- the by-hand path had it too
#   3, 4, 6, 8                                pass: correct for reasons the defect
#                                             happens to share
#
# **Test 2 was first written as cc phrased the ask -- "a push carrying native/ is
# gated regardless of which remote it goes to" -- and PASSED ON THE BROKEN
# RUNNER.** With both remotes behind, the old range is non-empty and the gate
# engages for the wrong reason; and since the defect never reads stdin, "the same
# answer whichever remote" is a property the BUG satisfies perfectly. It is now
# one HEAD against two remotes in different states, which the old computation
# cannot express. **A property stated in the words of the report is not
# necessarily a property that catches the defect being reported.**
#
# THE CASES THAT MUST STILL SKIP CARRY EQUAL WEIGHT, even though they do not
# discriminate. A gate that fires on every push gets `--no-verify`'d, which is
# cry-wolf arriving through a different door, so the board-only and already-level
# cases are as load-bearing as the regression -- they are what keeps the fix from
# being "make it always check", which would pass tests 1, 2, 5 and be useless.

load "../lib/test_helper.bash"

RUNNER="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/prepush"
TRIMMED_PATH="/usr/bin:/bin"
ZERO="0000000000000000000000000000000000000000"

# Run the runner the way git's pre-push hook does: ref pairs on stdin, no argv.
# Pass "" for stdin to exercise the by-hand path.
gate() {
  local stdin_content="$1"
  if [ -z "$stdin_content" ]; then
    env -i HOME="$HOME" PATH="$TRIMMED_PATH" DEVBIN_NAME=int \
      DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" \
      PROJECT_ROOT="$REPO" \
      bash "$RUNNER" </dev/null 2>&1
  else
    printf '%s\n' "$stdin_content" |
      env -i HOME="$HOME" PATH="$TRIMMED_PATH" DEVBIN_NAME=int \
        DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" \
        PROJECT_ROOT="$REPO" \
        bash "$RUNNER" 2>&1
  fi
}

refpair() { printf 'refs/heads/main %s refs/heads/main %s' "$1" "$2"; }
at() { git -C "$REPO" rev-parse "$1"; }

setup() {
  PATH="$TRIMMED_PATH" command -v git >/dev/null 2>&1 ||
    skip "git is not on the trimmed PATH -- this fixture cannot run"
  PATH="$TRIMMED_PATH" command -v cargo >/dev/null 2>&1 &&
    skip "cargo IS on the trimmed PATH -- the lever that separates skipped from engaged does not take here, and a run that cannot distinguish them would pass for the wrong reason"

  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/intent-prepush-XXXXXX")" && pwd)"
  REPO="$TEST_TEMP_DIR/repo"

  # TWO remotes, which is the whole point: one bare repo per remote, named the
  # way this project names them, with `upstream` tracked exactly as here.
  git init -q --bare "$TEST_TEMP_DIR/local.git"
  git init -q --bare "$TEST_TEMP_DIR/upstream.git"
  git init -q "$REPO"
  git -C "$REPO" config user.email t@t
  git -C "$REPO" config user.name t
  mkdir -p "$REPO/native/rust" "$REPO/intent/whiteboard"
  printf 'base\n' >"$REPO/README.md"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm base
  git -C "$REPO" remote add local "$TEST_TEMP_DIR/local.git"
  git -C "$REPO" remote add upstream "$TEST_TEMP_DIR/upstream.git"
  git -C "$REPO" push -q local main
  git -C "$REPO" push -q upstream main
  git -C "$REPO" branch --set-upstream-to=upstream/main main >/dev/null 2>&1
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
  return 0
}

# The state cc measured: a native/ change already pushed to `upstream`, so the
# tracked remote is level with HEAD while `local` is still behind. Asserts the
# rig reached that state before any test trusts it.
seed_pushed_to_upstream_only() {
  printf '[workspace]\n' >"$REPO/native/rust/Cargo.toml"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "native change"
  git -C "$REPO" push -q upstream main
  git -C "$REPO" fetch -q --all
  [ "$(at HEAD)" = "$(at upstream/main)" ] || {
    echo "rig did not reach the measured state: upstream is not level with HEAD" >&2
    return 1
  }
  [ -z "$(git -C "$REPO" diff --name-only upstream/main...HEAD)" ] || {
    echo "rig did not reach the measured state: the old range is not empty" >&2
    return 1
  }
}

@test "a push carrying native/ is gated at the remote that is behind" {
  # THE REGRESSION. The old computation saw an empty range here and exited 0.
  seed_pushed_to_upstream_only
  run gate "$(refpair "$(at HEAD)" "$(at local/main)")"
  [ "$status" -ne 0 ]
  [[ "$output" == *"cargo not on PATH"* ]]
}

@test "the answer follows each remote's own position, not the tracked remote's" {
  # cc asked for "a push carrying native/ is gated regardless of which remote it
  # goes to". Written literally -- both remotes behind, assert both gate -- that
  # PASSES ON THE BROKEN RUNNER, twice over: the old range is non-empty when
  # both are behind, and the defect ignores stdin entirely, so "the same answer
  # whichever remote" is trivially satisfied by the bug. Measured, not reasoned:
  # the literal form was written first and survived the mutation.
  #
  # The property with teeth is the one below. ONE HEAD, two remotes in DIFFERENT
  # states, two different answers -- each correct for its own remote. A runner
  # that consults `@{upstream}` cannot produce two answers here at all.
  seed_pushed_to_upstream_only
  run gate "$(refpair "$(at HEAD)" "$(at local/main)")"
  [ "$status" -ne 0 ]
  [[ "$output" == *"cargo not on PATH"* ]]
  run gate "$(refpair "$(at HEAD)" "$(at upstream/main)")"
  [ "$status" -eq 0 ]
  [[ "$output" == *"no native/ or build-manifest change"* ]]
}

@test "a push to a remote already level is skipped" {
  seed_pushed_to_upstream_only
  run gate "$(refpair "$(at HEAD)" "$(at upstream/main)")"
  [ "$status" -eq 0 ]
  [[ "$output" == *"no native/ or build-manifest change"* ]]
}

@test "a board-only push is skipped even when the remote is behind" {
  # No false positives: the range is genuinely non-empty here, and the path
  # trigger is what must decide. A gate that fires on every push gets bypassed.
  local prev
  prev="$(at HEAD)"
  printf 'note\n' >"$REPO/intent/whiteboard/note.md"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "board only"
  [ -n "$(git -C "$REPO" diff --name-only "$prev"...HEAD)" ]
  run gate "$(refpair "$(at HEAD)" "$prev")"
  [ "$status" -eq 0 ]
  [[ "$output" == *"no native/ or build-manifest change"* ]]
}

@test "a push creating a new ref on the remote is gated" {
  # Every commit is arriving and there is no bounded range to compute, so the
  # gate pays for the check rather than guessing a narrow one.
  run gate "$(refpair "$(at HEAD)" "$ZERO")"
  [ "$status" -ne 0 ]
  [[ "$output" == *"cargo not on PATH"* ]]
}

@test "deleting a ref is skipped -- no tree is pushed" {
  run gate "$(refpair "$ZERO" "$(at local/main)")"
  [ "$status" -eq 0 ]
  [[ "$output" == *"no native/ or build-manifest change"* ]]
}

@test "run by hand it answers for the remote that is furthest behind" {
  # No stdin, so there are no ref pairs and no single right answer -- "what
  # would go if I pushed now" differs per remote. The union is what keeps the
  # by-hand path from reintroducing the same under-report wearing another name.
  seed_pushed_to_upstream_only
  run gate ""
  [ "$status" -ne 0 ]
  [[ "$output" == *"cargo not on PATH"* ]]
}

@test "run by hand with no remotes at all, it checks rather than guesses" {
  git -C "$REPO" remote remove local
  git -C "$REPO" remote remove upstream
  run gate ""
  [ "$status" -ne 0 ]
  [[ "$output" == *"cargo not on PATH"* ]]
}
