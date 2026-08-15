#!/usr/bin/env bats
# Issue 0028: `int postcommit` clears the stale index entries a partial commit
# leaves behind -- and must never touch anything else.
#
# The runner unstages entries where the index differs from HEAD while the
# WORKTREE ALREADY EQUALS HEAD. That single condition carries the whole safety
# argument, so these guard both of its sides: the artefact is cleared, and the
# shapes of real work that share its signature survive the same sweep.
#
# **Why a test rather than the manual canary that shipped it.** The runner
# DELETES index state. It was canaried by hand in a scratch repo in both
# directions before it went live, which established that it worked once; nothing
# established that it keeps working. A guard over a destructive operation with no
# regression test is exactly the shape this estate keeps closing elsewhere.
#
# The fixture builds the real defect rather than simulating its signature: a
# pre-commit hook that formats the staged markdown and runs `git add`, then a
# `git commit --only`. Git points that hook at a temporary index and afterwards
# writes the real one from a snapshot taken BEFORE the hook ran, which is what
# strands the entry. **A fixture that merely staged something and reverted it
# would produce a matching `MM` while testing a mechanism we do not have.**
#
# **Mutation-proven, and the exercise corrected this header rather than
# confirming it.** Every claim below was measured by breaking the runner in a
# sacrificial worktree and reading which tests went red:
#
#   remove the worktree==HEAD guard (:95)   -> 2, 3 fail
#   remove the mid-operation bail (:71)     -> 4 fails
#   remove the git reset (:102)             -> 1 fails
#   drop the recoverable blob sha (:101)    -> 5 fails
#
# **This header first claimed the guard mutation failed "2, 3 and 4", and a
# seventh test asserting "never modifies the working tree" was deleted for being
# vacuous.** Nothing could make it fail. Even swapping `git reset -q --` for the
# destructive `git checkout -q HEAD -- <path>` fails NOTHING, and the reason is
# the guard itself: on a path whose worktree already equals HEAD those two
# commands have identical observable outcomes, so within the set this runner acts
# on there is no difference left to detect. **The worktree assertion that DOES
# carry weight is the one inside test 3**, which checks a path with pending work
# -- the only case where touching the tree would be a defect. A separate test
# restating the guard's own consequence read as coverage and was not.
#
# One row of the map was inconclusive on the first attempt because the mutation
# did not apply; it is recorded above only after a fixture that demonstrably took.
# **An unapplied mutation reports "nothing failed", which is indistinguishable
# from a test that does not check.**

load "../lib/test_helper.bash"

RUNNER="${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/postcommit"

# Run the real runner against the scratch repo, the way the hook does.
sweep() {
  DEVBIN_NAME=int \
    DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib" \
    PROJECT_ROOT="$REPO" \
    bash "$RUNNER"
}

# The index column of `git status --porcelain` for one path, or "" if absent.
# Read by OFFSET, never with a field splitter: awk collapses the leading space,
# so a worktree-only ` M` reads as staged and every assertion below inverts.
index_col() {
  local line
  line="$(cd "$REPO" && git status --porcelain -- "$1")"
  [ -n "$line" ] || return 0
  printf '%s' "${line:0:1}"
}

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/intent-postcommit-XXXXXX")" && pwd)"
  REPO="$TEST_TEMP_DIR/repo"
  mkdir -p "$REPO"
  cd "$REPO" || return 1
  git init -q .
  git config user.email t@t
  git config user.name t

  printf 'v1\n' >artefact.md
  printf 'v1\n' >other.md
  git add .
  git commit -qm base

  # The real seeding mechanism: format the staged markdown, then `git add`.
  mkdir -p .git/hooks
  cat >.git/hooks/pre-commit <<'HOOK'
#!/usr/bin/env bash
set -euo pipefail
MD=$(git diff --cached --name-only --diff-filter=ACM -- '*.md' || true)
[ -n "$MD" ] || exit 0
for f in $MD; do printf 'FORMATTED\n' >"$f"; done
echo "$MD" | xargs git add
HOOK
  chmod +x .git/hooks/pre-commit
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
  return 0
}

# Produce a genuine stale entry, and assert the fixture actually did so before
# any test trusts it -- a sweep over a repo with nothing stranded passes every
# assertion below while proving nothing.
seed_stale() {
  cd "$REPO" || return 1
  printf 'edited\n' >artefact.md
  git commit --only artefact.md -qm "partial"
  [ "$(index_col artefact.md)" = "M" ] || {
    echo "fixture did not strand an index entry -- the mechanism under test is absent" >&2
    return 1
  }
}

@test "clears the stale entry a partial commit stranded" {
  seed_stale
  run sweep
  [ "$status" -eq 0 ]
  [ -z "$(index_col artefact.md)" ]
}

@test "leaves a staged NEW file alone" {
  seed_stale
  printf 'new-work\n' >added.md
  git add added.md
  run sweep
  [ "$status" -eq 0 ]
  [ "$(index_col added.md)" = "A" ]
  [ "$(git show :added.md)" = "new-work" ]
}

@test "leaves a staged edit whose worktree has moved on alone" {
  seed_stale
  printf 'v2\n' >other.md
  git add other.md
  printf 'v3\n' >other.md
  run sweep
  [ "$status" -eq 0 ]
  [ "$(index_col other.md)" = "M" ]
  [ "$(git show :other.md)" = "v2" ]
  [ "$(cat "$REPO/other.md")" = "v3" ]
}

@test "bails entirely mid-merge, where a stale-looking entry is state" {
  seed_stale
  : >"$REPO/.git/MERGE_HEAD"
  run sweep
  [ "$status" -eq 0 ]
  # Untouched: during a merge the index belongs to the operation, not to us.
  [ "$(index_col artefact.md)" = "M" ]
}

@test "prints a blob sha that actually resolves to what it removed" {
  seed_stale
  staged="$(cd "$REPO" && git show :artefact.md)"
  run sweep
  [ "$status" -eq 0 ]
  [[ "$output" == *"recoverable: git cat-file -p"* ]]
  sha="$(printf '%s' "$output" | sed -n 's/.*recoverable: git cat-file -p \([0-9a-f]*\).*/\1/p' | head -1)"
  [ -n "$sha" ]
  [ "$(cd "$REPO" && git cat-file -p "$sha")" = "$staged" ]
}

@test "says nothing at all when there is nothing stranded" {
  cd "$REPO" || return 1
  run sweep
  [ "$status" -eq 0 ]
  [ -z "$output" ]
}
