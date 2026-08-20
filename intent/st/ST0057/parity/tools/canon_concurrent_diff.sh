#!/bin/bash
# canon_concurrent_diff.sh -- two threads edited produce diffs in two distinct files.
#
# ST0057 AT-01.4, covering AC-01.4. **CHECKED BY EDITING TWO THREADS AND
# OBSERVING THE CHANGED-PATH SET, NOT BY INSPECTING THE LAYOUT**, and the
# criterion says so: a directory listing proves where files SIT, and this
# property is about where a WRITE LANDS. Those come apart the moment a write
# path spells a canon location independently of the resolver, which has already
# happened once in this estate (`export.rs:386`, closed by AC-01.6).
#
# WHY THE PROPERTY EXISTS. D57-1 option B was a single consolidated
# `threads.jsonl`. It was rejected because FOUR CONCURRENT WRITERS make one
# canon file a merge-conflict generator, and this estate has four nodes. Option
# C -- one file per artefact -- is what lets two nodes edit two threads and
# never collide. **This is the measurement that would have caught option B**,
# and it is worth having precisely because the layout looks obviously fine to
# anyone who lists the directory.
#
# IT BUILDS ITS OWN RIG AND NEVER TOUCHES THE REAL ESTATE. The measurement
# requires WRITING to two threads; doing that in the working tree would mutate
# canon four nodes are reading, to answer a question about layout. So it clones,
# supplies the two gitignored things a clone lacks (the binary and the store),
# edits there, measures there, and deletes the lot. A command whose blast radius
# exceeds what it was aimed at has already cost this estate an uncommitted
# prototype.
#
# THE EDITS GO THROUGH THE CLI, NOT THROUGH A TEXT EDITOR, and that is the whole
# point of the arrangement. Hand-editing two canon files and observing two
# changed paths would prove that two files exist. Driving `st hold` proves that
# the WRITE PATH, resolving canon for itself, puts two threads in two places.
#
# WHAT THE ASSERTION IS, STATED EXACTLY BECAUSE A BARE COUNT WOULD NOT DO.
# Cardinality alone is satisfiable by two edits landing in one shared file plus
# one unrelated file also changing. So the check requires: exactly two changed
# canon paths, they are DISTINCT, and each one NAMES ITS OWN thread.
#
# WHAT IT CANNOT DO, SAID RATHER THAN IMPLIED. The failing case -- two edits, ONE
# changed path -- is only producible by reintroducing option B, so there is no
# red-first arm that plants the real defect without building the rejected design
# to do it. The assertion is therefore driven by removing an edit: one edit
# yields one path and the check fails, which proves it is genuinely counting
# rather than always returning true. That is a weaker control than a planted
# option-B canon and this comment is where that is recorded.
#
# Exit codes: 0 the property holds; 1 it does not; 2 the question could not be asked.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# tools -> parity -> ST0057 -> st -> intent -> repo root. FIVE.
ROOT="$(cd "$HERE/../../../../.." && pwd)"

die() { echo "canon-concurrent: $*" >&2; exit 2; }

EDITS=2
[ "${1:-}" = "--one-edit" ] && EDITS=1   # drives the assertion; see the note above

BIN="$ROOT/native/rust/target/release/intent"
[ -x "$BIN" ] || die "no v3 binary at $BIN -- the edits are made through the CLI and cannot be made without it. NOT a pass: the question was not asked."
git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1 || die "$ROOT is not a git repository"

TMP="$(mktemp -d "${TMPDIR:-/tmp}/canon-concurrent.XXXXXX")" || die "could not create a temporary directory"
trap 'rm -rf "$TMP"' EXIT
RIG="$TMP/rig"
git clone --quiet --no-hardlinks "$ROOT" "$RIG" 2>/dev/null || die "git clone of $ROOT failed"
# A clone carries committed content only, so the two gitignored things it needs
# are supplied by hand. Both are DERIVED, not authored -- copying them in is
# reconstituting the rig, not importing state that could change the answer.
mkdir -p "$RIG/native/rust/target/release" "$RIG/intent/.cache"
cp "$BIN" "$RIG/native/rust/target/release/intent" || die "could not stage the binary into the rig"
cp "$ROOT/intent/.cache/intent.db" "$RIG/intent/.cache/intent.db" 2>/dev/null \
  || die "no store at intent/.cache/intent.db to stage into the rig -- the edits need one"

# Two threads that are actually editable by the verb used below.
CANDIDATES="$("$RIG/native/rust/target/release/intent" st list --status all 2>/dev/null \
  | sed -n 's/^\(ST[0-9][0-9]*\) .*| WIP .*/\1/p' | sort -u)"
n_cand="$(printf '%s' "$CANDIDATES" | grep -c . )"; n_cand="${n_cand:-0}"
[ "$n_cand" -ge 2 ] || die "found $n_cand thread(s) in a state this check can edit; it needs 2. An estate that cannot supply two editable threads cannot answer this question, which is not the same as the answer being yes."

A="$(printf '%s\n' "$CANDIDATES" | sed -n '1p')"
B="$(printf '%s\n' "$CANDIDATES" | sed -n '2p')"

edit_one() {  # $1 = thread id
  local err
  err="$( cd "$RIG" && ./native/rust/target/release/intent st hold "$1" --reason "canon_concurrent_diff.sh measurement" 2>&1 )" \
    || die "\`st hold $1 --reason ...\` failed in the rig -- no edit was made, so there is nothing to measure. It said: $err"
}

edit_one "$A"
[ "$EDITS" -ge 2 ] && edit_one "$B"

# THE MEASUREMENT: what did git see change under canon?
CHANGED="$(git -C "$RIG" status --porcelain -- intent/.canon/ | sed 's/^...//' | sort -u)"
n_changed="$(printf '%s' "$CHANGED" | grep -c . )"; n_changed="${n_changed:-0}"

# THE EXPECTATION IS FIXED AT TWO AND DOES NOT MOVE WITH THE INPUT, WHICH IS THE
# ONLY REASON --one-edit PROVES ANYTHING. The first cut compared $n_changed
# against $EDITS, so reducing the edits reduced the expectation with it: one
# path against one expected, self-consistent, green, and unable to fail. An arm
# whose expectation tracks its input is not a control, and this estate has spent
# a day finding that shape in other people's instruments.
EXPECT=2
echo "canon-concurrent: edited $EDITS of 2 thread(s) through the CLI; git reports $n_changed changed canon path(s):"
printf '%s\n' "$CHANGED" | sed 's/^/    /'

fail=0
[ "$n_changed" -eq "$EXPECT" ] || { echo "  EXPECTED $EXPECT distinct changed path(s), got $n_changed." >&2; fail=1; }
# Each thread must name its OWN path, which a bare count cannot show: two edits
# landing in one shared file plus one unrelated change also totals two.
for id in "$A" "$B"; do
  printf '%s\n' "$CHANGED" | grep -q "$id" \
    || { echo "  $id has no changed path naming it -- no write landed in that thread's own file." >&2; fail=1; }
done
pa="$(printf '%s\n' "$CHANGED" | grep "$A" | head -1)"
pb="$(printf '%s\n' "$CHANGED" | grep "$B" | head -1)"
if [ -n "$pa" ] && [ "$pa" = "$pb" ]; then
  echo "  both threads resolved to ONE path ($pa) -- this is D57-1 option B's failure, the reason it was rejected." >&2
  fail=1
fi

if [ "$fail" -eq 0 ]; then
  echo "canon-concurrent: two threads edited concurrently land in two distinct files. Option C holds."
  exit 0
fi
echo "canon-concurrent: the changed-path set does not have the shape AC-01.4 requires." >&2
exit 1
