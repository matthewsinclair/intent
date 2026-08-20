#!/bin/bash
# canon_clone_completeness.sh -- a fresh clone carries canon for every artefact.
#
# ST0057 AT-01.2, covering AC-01.2 (D29). **CHECKED BY CLONING, NEVER BY READING
# `.gitignore`**, and the criterion says so in those words: the question is what
# git DOES, not what a rule appears to say. Ignore syntax has negations,
# directory suffixes, anchoring and precedence, and a hand-rolled reading of it
# would disagree with git exactly where it matters.
#
# WHAT MAKES A CLONE COMPLETE. `intent/.canon/` is the layer that TRAVELS. The
# store is gitignored, so an artefact whose canon never entered git exists only
# inside a database on one machine -- absent from a fresh clone and destroyed by
# `rm -rf intent/.cache`. That single property is what makes every deletion in
# this thread safe rather than careful, and it is the one measured here.
#
# THE POPULATION IS THE MODEL AND NOT THE CANON FILES, WHICH IS THE WHOLE
# DIFFERENCE BETWEEN THIS AND A TAUTOLOGY. Enumerating `intent/.canon/*.json`
# and checking those files reach the clone asks whether TRACKED FILES CLONE,
# which is true by construction and would pass on an estate that had lost half
# its artefacts. The artefacts are read from the STORE; the canon is looked for
# in the CLONE. Those are two independent derivations, so they can disagree, and
# an artefact the model holds whose canon never travelled is exactly the
# disagreement D29 exists to prevent.
#
# AN EMPTY POPULATION IS A REFUSAL, NOT A PASS. Zero artefacts and a complete
# estate produce the same "nothing missing", so the denominator is printed on
# every run and a zero denominator exits 2.
#
# WHAT THIS DOES NOT COVER, STATED BECAUSE THE GAP IS THE REASON AC-01.5 EXISTS
# SEPARATELY. This checks the STATE, at one revision, by cloning. Git ignore
# rules do not untrack a file that is already tracked, so adding
# `intent/.gitignore` entries for canon would NOT break this check on the
# already-committed estate -- it would only stop the NEXT artefact travelling.
# The edit is refused by `canon-ignore-guard.sh` (AC-01.5); the state is checked
# here. **Neither one covers the other and the gap between the two moments is
# where the class lives.**
#
# Exit codes: 0 every artefact has canon in the clone; 1 one or more do not;
# 2 the question could not be asked.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# tools -> parity -> ST0057 -> st -> intent -> repo root. FIVE.
ROOT="$(cd "$HERE/../../../../.." && pwd)"

die() { echo "canon-clone: $*" >&2; exit 2; }

BIN="$ROOT/native/rust/target/release/intent"
[ -x "$BIN" ] || die "no v3 binary at $BIN -- the artefact population is read from the store and cannot be derived without it. NOT a pass: the question was not asked."
git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1 || die "$ROOT is not a git repository, so there is nothing to clone and no clone to inspect."

# ---------------------------------------------------------------------------
# 1. The population, from the MODEL.
# ---------------------------------------------------------------------------
THREADS="$("$BIN" st list --status all 2>/dev/null | sed -n 's/^\(ST[0-9][0-9]*\) .*/\1/p' | sort -u)"
ISSUES="$("$BIN" issues list --kind all 2>/dev/null | sed -n 's/^\([0-9][0-9][0-9][0-9]\) .*/\1/p' | sort -u)"

n_threads="$(printf '%s' "$THREADS" | grep -c . )"; n_threads="${n_threads:-0}"
n_issues="$(printf '%s' "$ISSUES" | grep -c . )";   n_issues="${n_issues:-0}"
total=$(( n_threads + n_issues ))
[ "$total" -gt 0 ] || die "the store reports 0 artefacts. An empty population and a complete estate both report nothing missing, so this is a refusal and not a pass."

# ---------------------------------------------------------------------------
# 2. The clone. This is the measurement, not a convenience.
# ---------------------------------------------------------------------------
TMP="$(mktemp -d "${TMPDIR:-/tmp}/canon-clone.XXXXXX")" || die "could not create a temporary directory"
trap 'rm -rf "$TMP"' EXIT
CLONE="$TMP/clone"
git clone --quiet --no-hardlinks "$ROOT" "$CLONE" 2>/dev/null \
  || die "git clone of $ROOT failed -- the question could not be asked, which is not the same as the answer being yes."

# ---------------------------------------------------------------------------
# 3. Every artefact, looked for in the CLONE.
# ---------------------------------------------------------------------------
MISSING=""
present=0
check_one() {  # $1 = artefact label, $2 = path relative to the clone
  if [ -f "$CLONE/$2" ]; then
    present=$(( present + 1 ))
  else
    # WHY IT IS ABSENT CHANGES THE REMEDY, so the two are told apart rather than
    # both reported as "missing" and left for the reader to work out.
    local why="canon is absent from the SOURCE tree as well -- the model holds an artefact nothing has ever written canon for"
    if [ -f "$ROOT/$2" ]; then
      why="canon EXISTS in the source worktree and did NOT travel -- it is untracked, so it is either ignored (D29's failure) or simply never committed"
    fi
    MISSING="${MISSING}${1}  ${2}
    ${why}
"
  fi
}

while IFS= read -r id; do
  [ -n "$id" ] && check_one "$id" "intent/.canon/st/${id}.json"
done <<< "$THREADS"
while IFS= read -r n; do
  [ -n "$n" ] && check_one "issue ${n}" "intent/.canon/issues/${n}.json"
done <<< "$ISSUES"

n_missing=$(( total - present ))

if [ "$n_missing" -eq 0 ]; then
  echo "canon-clone: $present of $total artefact(s) have canon in a fresh clone ($n_threads thread(s), $n_issues issue(s)). A clone is complete."
  exit 0
fi

echo "canon-clone: $present of $total artefact(s) have canon in a fresh clone -- $n_missing DO NOT:" >&2
printf '%s' "$MISSING" | sed 's/^/  /' >&2
echo "  D29: a gitignored path is never canon. An artefact whose canon does not travel exists only" >&2
echo "  inside a store that is gitignored and per-machine -- absent from a fresh clone, and destroyed" >&2
echo "  by \`rm -rf intent/.cache\`. Commit the canon, or stop the rule that is keeping it out." >&2
exit 1
