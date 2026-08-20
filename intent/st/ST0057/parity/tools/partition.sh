#!/bin/bash
# partition.sh -- which tracked files under `intent/` have NO store row, and
# what kind of thing each one is.
#
# ST0057. Answers hv's boundary question -- *250 files under `intent/` are not
# in the store at all; not all of that should be, but certainly some of it
# should* -- by measuring the population rather than recalling it, and by
# printing the decomposition beside the total so the number can be checked
# against its own method.
#
# READS A NAMED COMMIT VIA `git ls-tree`, NEVER THE WORKING TREE. In a four-node
# checkout the working tree is nobody's tree: on 2026-08-20 two nodes changed
# this inventory inside one window -- ic's issue prune (-42 tracked) and vc's
# D53 event-log drop (-1) -- and a delta measured against the tree could not be
# attributed to either. Against named commits each delta is a diff of two
# records and names its own paths.
#
# NO BINARY, NO CLOCK, NO NETWORK. Every number here comes out of git, which is
# content-addressed and identical on every node. That is not tidiness: the
# shared `intent` binary is rebuilt from the union of four people's uncommitted
# work, so a measurement taken through it names nobody's commit and cannot be
# re-interrogated once the build is replaced. **WHERE A QUESTION CAN BE ANSWERED
# FROM THE REPO RATHER THAN FROM THE TOOL, ANSWER IT FROM THE REPO** -- the
# answer is then immune by construction rather than by discipline.
#
# ERE (`-E`) THROUGHOUT AND DELIBERATELY. This runs under `/usr/bin/grep` (BSD
# grep 2.6.0-FreeBSD), where POSIX BRE makes `$` an anchor ONLY as the pattern's
# final character -- mid-pattern it is a LITERAL. An interactive shell here
# resolves `grep` to ugrep, which anchors it in every alternation branch, so
# `'/\.gitkeep$\|/\.gitignore$'` answers 3 in the shell and 0 in a script.
# **A REGEX VERIFIED INTERACTIVELY AND DEPLOYED INTO A SCRIPT IS TWO DIFFERENT
# PROGRAMS.** That cost two wrong numbers in one run of this instrument.
#
# THE SCAFFOLDING ADD-BACK, AND WHY THE FIRST ANSWER WAS THREE TOO LOW. The
# store-backed directories are excluded wholesale ON THE GROUNDS THAT THEIR
# CONTENTS HAVE STORE ROWS -- and three files in them do not: two `.gitkeep`
# under `intent/issues/` (since pruned) and one under `intent/st/`. No
# extension, so not attachments (`ATTACHMENT_EXTENSIONS` is md/txt/sh), no store
# row, `Unattached` under `Project::classify`. Excluding a directory because
# MOST of it is store-backed is a count of containers reported as a count of
# contents, and it understated the population by three for weeks.
#
# `whiteboard/` IS SCOPED OUT BY DECISION AND ITS SCAFFOLDING IS NOT ADDED BACK.
# It is a coordination surface, outside hv's question. `SCAF` therefore names
# only the three store-backed directories and NOT `$STORE`: folding whiteboard's
# five `.gitkeep` in moved `b574361a` -- a commit whose content is FIXED -- from
# 253 to 258, and the only thing that caught it was re-running a point already
# measured. **WHEN YOU CHANGE AN INSTRUMENT, RE-RUN IT ON A KNOWN POINT: the
# delta you are hunting is in the instrument, not the subject.**
#
# THE SUM IS ASSERTED AND THE ASSERTION CAN FAIL. T+B+N+M must equal the
# denominator; a mismatch exits 2. An instrument whose self-check only prints is
# the same shape as an expectation that tracks its input -- self-consistent,
# green, and structurally unable to fail. A zero denominator exits 2 for the
# same reason: an empty estate and a complete one both produce "nothing here".
#
# N IS LISTED, NOT COUNTED. On 2026-08-20 the answer left 250, was corrected to
# 253, and returned to 250 by two landings -- with N having been 3, then 6, then
# a DIFFERENT 3. **The wrong 250 and the right 250 are indistinguishable by
# inspection**, so the members are named on every run and a number never stands
# alone as evidence of the method that reached it.
#
# WHAT THIS DOES NOT ANSWER. It says whether a file HAS a store row, never
# whether it SHOULD. Ownability is a policy question -- as of 2026-08-20 zero of
# the B class is ownable by an existing artefact, and the blocker is the ARITY
# of the sigil space (`STEELTHREAD` alone, since hv's ruling dropped `ISSUE:`),
# not the policy. That determination is vc's and is not derivable from here.
#
# Usage: partition.sh [<rev>]      (default HEAD)
# Exit:  0 measured   2 refusal (zero denominator, or the sum does not reconcile)

set -u

rev=${1:-HEAD}
sha=$(git rev-parse --short "$rev") || exit 2

# Store-backed by directory, plus `whiteboard/` which is out of scope by
# decision. SCAF is deliberately narrower -- see the header.
STORE='^intent/(st|\.canon|issues|whiteboard)/'
SCAF='^intent/(st|\.canon|issues)/'
NEVER='^intent/(todo\.md|events\.jsonl|\.intentfiles|\.config/)'

all=$(git ls-tree -r --name-only "$rev" -- intent/)
total=$(printf '%s\n' "$all" | grep -c .)

out=$(printf '%s\n' "$all" | grep -E -v "$STORE")
n_out=$(printf '%s\n' "$out" | grep -c .)
scaf_paths=$(printf '%s\n' "$all" | grep -E "$SCAF" | grep -E '/\.(gitkeep|gitignore)$')
scaf=$(printf '%s\n' "$scaf_paths" | grep -c .)

denom=$((n_out + scaf))

payload=$(printf '%s\n' "$out" | grep -E -c '^intent/plugins/')
content=$(printf '%s\n' "$out" | grep -E -v "^intent/plugins/|$NEVER" | grep -c .)
never_paths=$(printf '%s\n' "$out" | grep -E "$NEVER" | grep -E -v '^intent/todo\.md$')
never=$(printf '%s\n' "$never_paths" | grep -c .)
N=$((never + scaf))

echo "partition: $sha -- $total tracked under intent/, $denom with no store row"
echo "  T tool payload     $payload   intent/plugins/"
echo "  B project content  $content"
echo "  N never an artefact $N"
printf '%s\n' "$never_paths" "$scaf_paths" | grep -E . | sed 's/^/      /'
echo "  M model-derived    1   intent/todo.md"

if [ "$denom" -eq 0 ]; then
  echo "partition: REFUSED -- a zero denominator and a complete estate both report nothing missing" >&2
  exit 2
fi

sum=$((payload + content + N + 1))
if [ "$sum" -ne "$denom" ]; then
  echo "partition: REFUSED -- T+B+N+M is $sum against a denominator of $denom; the classes do not partition the population" >&2
  exit 2
fi

echo "partition: T+B+N+M = $sum, reconciles"
