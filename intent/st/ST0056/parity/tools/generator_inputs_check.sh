#!/usr/bin/env bash
#
# Every committed generated artefact's inputs are TRACKED BY GIT.
#
# ic's property, and the failure it closes is specific rather than theoretical:
# an artefact is committed, its only input lives in a `/tmp` scratchpad, and
# **nothing anywhere records which of those two states it is in.** The artefact
# reads as re-derivable right up until someone reboots -- by which time it is
# evidence supporting a closed AC and nobody can check it. Bitten four times in
# one day, in four different tools, which is the signature of something that
# wants a mechanism rather than more care.
#
# WHY A DECLARATION RATHER THAN INFERENCE. The honest alternative is to trace
# what a generator opens at runtime, which needs root on macOS and would make a
# pre-commit gate depend on it. Reading paths back out of shell source is worse
# than either -- `$SP/burn.tsv` is a variable whose value arrives from the
# environment, so a static reader learns the shape and not the file. So each
# generator DECLARES, and this checks the declaration. The declaration can lie;
# what it cannot do is lie SILENTLY, because the shape below is greppable and a
# generator with no declaration at all is refused.
#
# THE FORMAT, one comment line each, repo-relative paths:
#
#   # inputs: <path> [<path> ...]
#   # inputs-exempt: <NAME> -- <reason>          (optional, repeatable)
#
# `inputs-exempt` exists because ic named a real one: `WT` is a detached git
# worktree at a committed revision. It is genuinely re-derivable (`git worktree
# add <dir> <sha>`) and it is not a file in this repository, so requiring it to
# be tracked would be requiring the impossible. **The rule is about inputs that
# are FILES.** An exemption must carry a reason -- a bare name is refused,
# because the whole point is that the two states are distinguishable and an
# unexplained exemption restores exactly the ambiguity this closes.
#
# WHAT IT REFUSES, and the first one is the one that matters:
#
#   1. a generator with NO `# inputs:` line          <- the closing condition
#   2. a declared path git does not track
#   3. a declared path that does not exist
#   4. an exemption with no reason
#
# **Without (1) this guard would be decorative.** ic's stated failure is someone
# adding a generator that reads from `$SP` and forgetting to declare it; a guard
# that only checks declarations that exist cannot see that person at all.
#
# MUTATION-PROVEN, five mutations in a sacrificial worktree, every refusal
# reached:
#
#   a declaration removed                    -> (1)
#   an exemption stripped of its reason      -> (4)
#   an input naming a nonexistent path       -> (3)
#   an input repointed at an EXISTING but
#     untracked file                         -> (2)   <- ic's stated mutation
#
# **The first attempt at (2) pointed at `/tmp/scratch/burn.tsv`, which does not
# exist on this machine, so it tripped (3) and reported "does not exist".** The
# guard refused, the exit code was right, and the branch under test never ran --
# which is the same shape as an unapplied mutation reporting "nothing failed".
# It was redone with a file created in the worktree and confirmed untracked
# BEFORE the run, so the message quoted above is (2)'s and not (3)'s.
#
# ENUMERATED FROM THE INDEX, NOT FROM A GLOB. `git ls-files` is what the commit
# contains; a directory glob is what the working tree contains. A peer's
# half-written generator must never block a commit on paths they have not
# touched -- that is the provenance_check.sh lesson, where one node's untracked
# mid-generation file froze every node's commits. A guard that must be bypassed
# is a guard nobody keeps.
#
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../.." && pwd)"
cd "$ROOT"

REL="${HERE#"$ROOT"/}"
fail=""
checked=0
exempt_total=0

note() { fail="${fail}  $1
"; }

# `git ls-files` over the index: staged additions count as tracked, which is
# correct -- they are part of the commit being made.
tracked() { [ -n "$(git ls-files -- "$1" 2>/dev/null)" ]; }

while IFS= read -r gen; do
  [ -n "$gen" ] || continue
  checked=$((checked + 1))
  name="$(basename "$gen")"

  # Read the declaration from the INDEX rather than the working tree, for the
  # same reason the roster comes from the index: this checks what is being
  # committed. A generator edited but not staged is checked as it will land.
  src="$(git show ":$gen" 2>/dev/null || cat "$gen")"

  decl="$(printf '%s\n' "$src" | sed -n 's/^# inputs: *//p')"
  if [ -z "$decl" ]; then
    note "$name -- no '# inputs:' declaration IN THE INDEX. Every generator must name its inputs so they can be checked; if an input is not a file, declare '# inputs-exempt: <NAME> -- <reason>'. (This reads the index, not the working tree, because a gate must judge what the commit contains -- so 'I just added it' means 'git add' it.)"
    continue
  fi

  while IFS= read -r ex; do
    [ -n "$ex" ] || continue
    exempt_total=$((exempt_total + 1))
    case "$ex" in
      *' -- '*) ;;
      *) note "$name -- exemption '$ex' carries no reason. An unexplained exemption restores the ambiguity this guard exists to remove." ;;
    esac
  done < <(printf '%s\n' "$src" | sed -n 's/^# inputs-exempt: *//p')

  for p in $decl; do
    if [ ! -e "$p" ]; then
      note "$name -- declared input does not exist: $p"
    elif ! tracked "$p"; then
      note "$name -- declared input is NOT TRACKED by git: $p. A committed artefact whose input git does not hold is re-derivable today and not tomorrow."
    fi
  done
done < <(git ls-files -- "$REL/gen_*.sh")

if [ "$checked" -eq 0 ]; then
  # Not a pass. Finding no generators means the enumeration is wrong, and a
  # guard reporting green over an empty set is the false green this estate
  # keeps closing elsewhere.
  echo "generator-inputs: no generators found under $REL -- the enumeration is broken, not the tree" >&2
  exit 1
fi

if [ -n "$fail" ]; then
  printf 'generator-inputs: a committed generated artefact names an input git cannot hold.\n\n' >&2
  printf '%s\n' "$fail" >&2
  exit 1
fi

printf 'generator-inputs: %s generator(s) declare inputs; all tracked; %s declared exemption(s).\n' \
  "$checked" "$exempt_total"
