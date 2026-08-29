#!/bin/bash
# lib_staged.sh -- read a file from the INDEX, not from the working tree.
#
# SOURCED, NOT EXECUTED. It defines functions and forms no verdict, so it ships
# 644 like `lib_corpus.sh`, `lib_mdfmt.sh`, `lib_surface.sh` and `lib_classify.sh`.
#
# ==========================================================================
# WHY THIS EXISTS
# ==========================================================================
#
# Four sessions work one checkout. A gating check that reads a repo path off
# DISK therefore judges whatever every other node happens to have half-typed,
# and refuses commits over work its committer has never touched.
#
# This is not a hypothetical. It has now happened four times in this directory,
# each time found by a different node, each time fixed IN THE TOOL IN FRONT OF
# THE FINDER rather than in the class:
#
#   2026-08-17  runner_roster_check.sh globbed the worktree for its PRESENT
#               population, so a peer's untracked mid-work `*_check.sh` was an
#               unrostered tool and the only way past was to wait for its owner
#               to land a roster row (dc)
#   2026-08-17  residue_class_check.sh read `legacy.rs` off disk, so a
#               constructor typed but not yet declared froze every node (cc,
#               two hours after the line above was written down)
#   2026-08-28  corrected_check.sh, class_vocab_check.sh and rulings_check.sh
#               all read `surface/dispatch-table.json` off disk -- cc's own
#               mid-edit `st list` row cost vc a commit (issue 0125)
#
# **The rule was in hand every time and was applied to the instance.** That is
# the finding, and hv ruled on it: converge all four onto one mechanism rather
# than fix the three and leave the fourth carrying its own copy, because two
# copies of this mechanism is what produced the third episode.
#
# ==========================================================================
# WHAT IT DOES AND DOES NOT BUY
# ==========================================================================
#
# The purpose of each check is unchanged and the TIMING is unchanged: a fact
# written AND STAGED is in the commit's index, so it is still caught on the day
# it arrives, which is the only day anybody can say whether it blocks or
# carries. What stops being caught is a keystroke in someone else s editor.
#
# `git show :<path>` honours `GIT_INDEX_FILE`, and git hands a hook a temporary
# index during a partial commit, so under `commit --only` this reads HEAD plus
# the committer s own named paths -- which is exactly the tree the commit
# proposes and nothing else.
#
# REACH LIMIT, stated because a helper named `staged` invites the assumption it
# closed the class: THIS HANDLES FILES. `git show :<dir>` is not a thing, so a
# check that SCANS A DIRECTORY still reads the working tree and still sees a
# peer s mid-work. `rulings_check.sh` has two such scans (`ISSUES_DIR`,
# `ISSUES_CANON`); they are listed in its own header. A directory population
# would need `git ls-files`, which is a different mechanism with a different
# shape, and inventing it unasked here would put a second unproven home in the
# file whose whole subject is not having two.
#
# ==========================================================================
# THE CONTRACT, AND THE ONE LINE THAT IS LOAD-BEARING
# ==========================================================================
#
#   source "$HERE/lib_staged.sh"
#   trap staged_cleanup EXIT                      # <-- yours to install
#   TABLE_GIVEN="${TABLE:+yes}"                   # BEFORE the default is applied
#   TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"
#   [ -f "$TABLE" ] || die "no dispatch table at $TABLE"
#   TABLE="$(staged_copy "$TABLE_GIVEN" "$TABLE")" || exit 2
#
# **`|| exit 2` IS NOT DEFENSIVE NOISE.** `staged_copy` refuses by exiting 2,
# and it runs inside a command substitution, so that exit ends the SUBSHELL and
# the caller carries on with an EMPTY path. Two of this library s four callers
# do not set `-e`, so nothing else stops them. Without the `|| exit 2` a
# refusal becomes a check reading `""` and reporting whatever an empty input
# reports -- which for three of the four is a confident, complete, entirely
# fictional green.
#
# **`_GIVEN` IS CAPTURED BEFORE THE DEFAULT IS APPLIED**, because one line later
# both are set and the distinction is gone. An explicit `TABLE=` override is a
# deliberate act by someone driving the check against a planted corpus -- the
# mutation arms in these checks exist only because that is possible -- so an
# override keeps reading the path it was handed and is never staged.
#
# ==========================================================================
# WHAT MAKES A CALLER USE IT
# ==========================================================================
#
# Nothing here does, and the file two directories along says so in its own
# header: `lib_surface.sh` was built as the one home, sat beside two callers
# that never sourced it, and so ADDED a home rather than reducing them. **A
# sourced library closes the class only for the callers that source it.**
#
# So this file does not claim to close the class -- it makes the convergence
# possible, and the four current callers are converged. Whether the gate should
# REFUSE a new instrument that reads a gating path off disk is a roster question
# rather than a library one, and it is raised with vc rather than answered here.

# The staging area for THIS process. Derived from `$$` rather than accumulated
# in a variable, and that is the whole reason it is a directory: `staged_copy`
# runs inside a command substitution, so a name it appends to a shell array is
# lost with the subshell. `$$` is the ORIGINAL shell s pid in bash and does not
# change in a subshell (that is `$BASHPID`), so the parent can remove by the
# same name the child created under.
_staged_dir() {
  printf '%s' "${TMPDIR:-/tmp}/intent-staged.$$"
}

# Remove this process s staged copies. Install it yourself:
#
#   trap staged_cleanup EXIT
#
# The library does NOT install the trap. A sourced file that quietly replaces
# its caller s EXIT trap is a silent failure of exactly the kind this directory
# refuses, and there is no portable way to append to one.
staged_cleanup() {
  local d
  d="$(_staged_dir)"
  [ -n "$d" ] && [ -d "$d" ] && rm -rf "$d"
  return 0
}

# staged_copy <given> <path> -- print the path to READ.
#
#   <given>  non-empty when the caller was handed this path explicitly, in
#            which case it is returned untouched and never staged
#   <path>   an absolute path inside the repo
#
# Prints the original path for an override, otherwise a temp file holding the
# INDEX content of that path. Refuses at exit 2 when the path is not in the
# index: this check judges a commit, and a file the commit does not carry
# cannot be read from it.
staged_copy() {
  local given="$1" path="$2" root rel dest dir

  if [ -n "$given" ]; then
    printf '%s' "$path"
    return 0
  fi

  root="${STAGED_ROOT:-$REPO_ROOT}"
  if [ -z "$root" ]; then
    echo "error: staged_copy needs REPO_ROOT (or STAGED_ROOT) to make \`$path\` repo-relative" >&2
    exit 2
  fi

  rel="${path#"$root"/}"
  if [ "$rel" = "$path" ]; then
    echo "error: \`$path\` is not under \`$root\`, so it has no index entry to read" >&2
    exit 2
  fi

  dir="$(_staged_dir)"
  # `-p` tolerates the directory already existing, which is the normal case
  # from the second staged path onward. A pre-existing SYMLINK is refused
  # rather than followed: the name is predictable, so following one would let
  # anything on this machine choose what a gating check reads.
  if [ -L "$dir" ]; then
    echo "error: \`$dir\` is a symlink -- refusing to stage through it" >&2
    exit 2
  fi
  mkdir -p "$dir" 2>/dev/null || {
    echo "error: cannot create the staging directory \`$dir\`" >&2
    exit 2
  }

  # One file per repo-relative path, named deterministically. Deterministic
  # rather than `mktemp` for the same reason the directory is: nothing survives
  # the subshell, so the name has to be computable from the input alone.
  dest="$dir/$(printf '%s' "$rel" | tr '/' '%')"

  if ! git -C "$root" show ":$rel" >"$dest" 2>/dev/null; then
    rm -f "$dest"
    echo "error: $rel is not in the index -- this check judges the commit, so a file the commit does not carry cannot be read" >&2
    exit 2
  fi

  printf '%s' "$dest"
}
