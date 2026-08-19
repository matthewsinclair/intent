#!/usr/bin/env bash
#
# append-only-guard.sh -- refuse a commit that REMOVES lines from an
# append-only path. Arithmetic on a diff; it needs no knowledge of what the
# operation intended.
#
# COVERS ST0056 AC-10.13 / AT-10.13.
#
# WHY A GUARD RATHER THAN CARE. Writing where you meant to append is a
# one-character difference with no visible symptom, and every human-facing
# check reads correct afterwards. Two members, both measured, both silent:
#
#   intent/whiteboard/<node>/.history/**   492 lines destroyed 2026-08-17, by a
#                                          fold that overwrote the day's archive
#                                          instead of appending. The live inbox
#                                          read `_(empty)_` correctly, the entry
#                                          was correctly in .history/, the board
#                                          read as folded, `ws hygiene` was
#                                          clean. The ONLY signal anywhere was
#                                          `514 deletions` in `git show --stat`.
#
#   intent/events.jsonl                    19 events destroyed 2026-08-19, by
#                                          `intent upgrade` emitting an empty
#                                          log over a populated one -- and it
#                                          wrote no event for its own run, so
#                                          the operation that destroyed the
#                                          record left no trace in it.
#
# The first survived a full session. The second arrived while the guard proposed
# for the first sat unruled.
#
# IT CANNOT FALSE-POSITIVE, which is the property that makes it a guard: an
# append-only path only ever grows, so a removal is never legitimate. There is
# nothing to tune and no case to exempt.
#
# LEAN BY INSTRUCTION (hv: "bike-shedding"). No options, no config, no design.
# It lands in a shipped consumer surface -- every project inherits it on the
# next `intent upgrade` -- which is the reason for lean rather than an argument
# against building it.

set -euo pipefail

# The append-only subjects, as git pathspecs. Both live under `intent/`.
PATHS=(
  'intent/whiteboard/*/.history/**'
  'intent/events.jsonl'
)

# Nothing staged against either subject: nothing to say.
staged="$(git diff --cached --numstat -- "${PATHS[@]}" 2>/dev/null || true)"
[ -n "$staged" ] || exit 0

# `--numstat` is `added<TAB>removed<TAB>path`, and prints `-` for binary. A `-`
# is not zero: it means the count is unavailable, so it is reported rather than
# passed over.
violations=""
while IFS=$'\t' read -r added removed path; do
  [ -n "${path:-}" ] || continue
  case "$removed" in
    0) ;;
    -) violations="${violations}  ${path} -- binary diff, line count unavailable"$'\n' ;;
    *) violations="${violations}  ${path} -- ${removed} line(s) removed, ${added} added"$'\n' ;;
  esac
done <<< "$staged"

[ -n "$violations" ] || exit 0

cat >&2 <<EOF
error: append-only path(s) lost lines in this commit -- refusing.

$violations
An append-only path only ever grows. A removal means a write landed where an
append was meant, which is silent: the artefact looks correct afterwards and
every other check passes.

Recover the removed lines from git and merge them in timestamp order:

  git show HEAD:<path> > /tmp/before
  # merge /tmp/before with the working copy, then diff both halves to prove
  # nothing was lost -- eyeballing is what missed it the first time.

This guard is arithmetic on the staged diff. It has no opinion about what the
operation intended and no case to exempt.
EOF
exit 1
