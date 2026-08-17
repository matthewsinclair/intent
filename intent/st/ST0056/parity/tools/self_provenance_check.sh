#!/bin/bash
# self_provenance_check.sh -- a vendored tree must agree with the record COMMITTED beside it.
#
# AC-11.5 / AT-11.5: an artefact asserts its own provenance, and the assertion is
# read from the artefact, never from a record written beside it. This is the arm
# of that row which can be checked today; the binary arm waits on an embedded
# build commit, so AT-11.5 is held at `red` while this passes -- a green AT would
# satisfy AC-11.5 on one third of its criterion, because v2's gate satisfies an
# AC on the FIRST green AT covering it (issue 0032).
#
# WHY IT READS THE INDEX AND NOT THE WORKTREE, WHICH IS THE ENTIRE POINT. On
# 2026-08-17 this repository's HEAD carried a manifest that disagreed with the
# file beside it: `bin/devbin` hashed 41017e54, the committed manifest recorded
# 8016f112, and a clean extract of the commit reported 26 files matching and one
# DIVERGED. Every local check was GREEN, because an uncommitted manifest
# regeneration had made the WORKTREE self-consistent an hour earlier. The node
# who reported "27 of 27, stock devbin" was reading a tree that agreed with
# itself for a reason nobody else could reproduce.
#
# A TREE THAT AGREES WITH ITSELF BECAUSE SOMEONE RE-DERIVED THE RECORD LOCALLY IS
# INDISTINGUISHABLE FROM A TREE THAT AGREES BECAUSE IT IS CORRECT. So the subject
# has to be what the commit holds. The index is the right referent rather than
# HEAD: it is exactly the content about to become a commit, so a partial stage of
# the vendored set is judged as it will actually land.
#
# THE MECHANISM THAT PRODUCED IT, RECORDED BECAUSE THE FIX IS NOT "BE CAREFUL".
# A re-vendor is TWO FACTS -- the files and the record -- written by one act.
# `git commit --only` names paths, the author named the files, and the record
# stayed behind. The estate already carried that rule for the MOVE case ("naming
# the new path leaves the deletion staged") and it was not carried across,
# because a vendor does not look like a move. Four rules were broken by their own
# authors on this day, every one written down and none of them a mechanism; what
# they all lacked is something that fires while the act is performed. This fires.
#
# SCOPE IS THE COMMIT, NOT THE DIRECTORY, and that is a rule this toolchain has
# already paid for: an instrument that globs the worktree freezes every node's
# commits on a peer's in-flight file, and a guard that must be bypassed is a
# guard nobody keeps. Nothing here reads the working tree at all.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
MANIFEST="bin/.devbin/manifest.sha256"

die() {
  echo "self-provenance: $*" >&2
  exit 2
}

cd "$ROOT" || die "cannot reach the project root at $ROOT"
git rev-parse --git-dir >/dev/null 2>&1 ||
  die "not a git checkout, so there is no commit to read a vendored tree out of"

# DETERMINATE ABSENCE IS A FACT ABOUT THE PROJECT, REPORTED AND PASSED. A project
# that vendors nothing has nothing to disagree with. It is stated rather than
# skipped in silence, because "no vendored tree" and "the check did not run" are
# different answers and only one of them is reassuring.
if ! git cat-file -e ":$MANIFEST" 2>/dev/null; then
  echo "self-provenance: no $MANIFEST in the commit -- this project vendors no devbin, so there is nothing to disagree with."
  exit 0
fi

matched=0
diverged=0
absent=0
rc=0

# `rel`, NOT `path`. zsh ties `path` to `PATH`, so `read -r sha path` replaces the
# shell's entire PATH with a filename on the first iteration and every subsequent
# command reports `command not found` -- and the corruption OUTLIVES the loop. It
# has bitten two nodes on this estate in one day, both while measuring THIS
# manifest, because `read -r sha path` is the obvious spelling for a `.sha256`
# line. This file is bash and would survive it; the spelling is avoided anyway,
# because the next person to copy this loop may not be.
while read -r want rel; do
  [ -n "$rel" ] || continue
  if ! git cat-file -e ":$rel" 2>/dev/null; then
    echo "self-provenance: $rel is named by the committed manifest and is NOT in the commit" >&2
    absent=$((absent + 1))
    rc=1
    continue
  fi
  got="$(git cat-file blob ":$rel" | shasum -a 256 | awk '{print $1}')"
  if [ "$got" = "$want" ]; then
    matched=$((matched + 1))
  else
    echo "self-provenance: $rel DIVERGED -- the commit holds bytes the committed manifest does not describe" >&2
    echo "    manifest records  $want" >&2
    echo "    the commit holds  $got" >&2
    diverged=$((diverged + 1))
    rc=1
  fi
done < <(git cat-file blob ":$MANIFEST" | awk '/^#/ { next } NF >= 2 { p = $2; for (i = 3; i <= NF; i++) p = p " " $i; print $1, p }')

# THE MATCHED COUNT IS THE INSTRUMENT'S OWN CANARY, AND IT IS REPORTED WHETHER OR
# NOT ANYTHING FAILED. A run with zero matches is a broken run, not a wholly
# forked tree -- measured on this estate when a `read -r sha path` loop destroyed
# PATH and reported all 27 files diverged, one step away from being filed as an
# issue. A wrong ZERO certifies absence and a wrong MAXIMUM certifies
# catastrophe, and the second is the more persuasive because it looks like
# diligence rewarded. The matches are what say the tool ran.
[ "$matched" -gt 0 ] ||
  die "the committed manifest named files and NOT ONE hashed -- this is the instrument failing, not $((diverged + absent)) genuine divergences"

if [ "$rc" -eq 0 ]; then
  echo "self-provenance: $matched vendored file(s) in the commit match the manifest committed beside them."
else
  echo "self-provenance: $matched matched, $diverged diverged, $absent absent -- the commit's vendored tree disagrees with its own record" >&2
  echo "    A re-vendor is two facts, the files and the record. Stage BOTH:" >&2
  echo "      git add $MANIFEST <the vendored files>" >&2
fi

exit $rc
