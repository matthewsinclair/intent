#!/bin/bash
# ST0057 AC-03.6 / AT-03.6: a commit must not contain canon that names bytes not in
# that same commit. NOTE THE THREAD QUALIFIER -- this tool is HOUSED under ST0056 and
# CITED by ST0057, and four threads each carry an unrelated AT-03.6, so the bare id is
# ambiguous exactly where this file sits.
# canon_commit_check.sh -- canon must not name bytes that are not in its own commit.
#
# COVERS ST0057 AC-03.6. Every thread's `thread.json` carries an `attachments`
# array recording a `path` and the `sha256` of the bytes it holds. `intent sync
# --to-store` ingests those bytes FROM THE WORKTREE, so a sync run while any
# attached file is uncommitted writes canon naming bytes that live in no commit.
# On inspection that canon is indistinguishable from correct canon: present,
# well-formed, internally consistent. Only a comparison against the commit
# separates them.
#
# THE VACUOUS-PASS ARM IS FIRST IN THIS FILE BECAUSE IT WAS BUILT FIRST, on ic's
# instruction, and the first version of this tool FAILED IT. Three outcomes look
# alike here: an attachment MISMATCHED, an attachment ABSENT from the commit, and
# NO ATTACHMENTS RECORDED AT ALL. The first draft `continue`d on the third and
# printed "every attachment matches" over a commit where nothing had been
# compared -- reproduced at 6ab155ef, exit 0, examining zero. That is not a
# corner: over the 132 commits from 0ec2ac79 (canon's first) to dec6b1b9, 86
# recorded no attachments -- a figure that names ITS OWN RANGE, because the
# first version of this line said '86 of 132 commits in this history' and ic
# caught it: `git rev-list --count HEAD` is 2184, so the string named a subset
# it did not identify, inside a tool built to find records that disagree with
# reality. The range grows; the figure does not. So
# the vacuous case is the MAJORITY of the input. A count is therefore printed on
# every run, and an empty population exits 2 (cannot measure) and never 0.
#
# WHY IT IS NAMED APART FROM `self_provenance_check.sh`. That tool's arm 1 asks
# whether a VENDORED TREE agrees with the manifest committed beside it, reading
# the INDEX. This asks whether CANON agrees with the commit that shipped it.
# Arm 1 passes on every case this catches -- measured, on the episode that
# produced this tool -- because the file it validates is the manifest and never
# an attachment. Highlander is one home per concern, not one file per word.
#
# WHAT IT FAILS ON, AND THE DISTINCTION IS THE WHOLE DESIGN. It FAILS only on a
# divergence THIS COMMIT ADDS and REPORTS an inherited one without failing --
# the whiteboard clock guard's rule. Divergences here are EPISODES, not events:
# a file diverges and stays divergent every subsequent commit until someone
# syncs, one observed run lasting NINE consecutive commits. Over the full
# population of 46 commits carrying canon with attachments, 23 disagree AT ALL
# and 5 ADD. Built on the first number this blocks half of all commits and is a
# guard nobody keeps; built on the second it fires five times. If you change
# this file, that clause is the part not to lose.
#
# THOSE NUMBERS ARE A POPULATION, NOT A WINDOW, AND THE DISTINCTION COST A WRONG
# ANSWER ONCE. The first measurement read 14 commits, found 12 clean, reported
# 14%. The window sat almost entirely inside a clean stretch; the true rate is
# 50%. The error favoured the argument its author was making, which is the
# direction to distrust first. A single-revision run reports that revision;
# `--history N` reports the N you asked for and nothing about the rest.
#
# REACH. Its subject is ATTACHMENTS. A criterion, a status field, a note, or any
# other canon that is not an attachment is INVISIBLE to it -- so it does not and
# must not cover a refused ingest leaving a stale store standing as truth, which
# is a different subject with its own criterion. Stated because it was nearly
# cited as the remedy for both.
#
# NO WORKTREE, NO BINARY, NO CLOCK. `git`, `jq` and `shasum` only, so the COMMIT
# SHA is the identity of everything compared, and it is re-drivable on any
# revision by anyone.

set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

die() { echo "error: $1" >&2; exit 2; }

cd "$ROOT" || die "cannot reach the project root at $ROOT"
git rev-parse --git-dir >/dev/null 2>&1 ||
  die "not a git checkout, so there is no commit for canon to disagree with"
command -v jq >/dev/null || die "jq is required to read a thread's attachments"

# Emits "<thread> <count>" per thread carrying a thread.json at $1.
threads_at() {
  local rev="$1" tj st n
  git rev-parse --verify --quiet "$rev^{commit}" >/dev/null || return 0
  for tj in $(git ls-tree -r --name-only "$rev" -- intent/st 2>/dev/null | grep '/thread\.json$'); do
    st="${tj#intent/st/}"; st="${st%/thread.json}"
    n="$(git show "$rev:$tj" 2>/dev/null | jq '(.attachments // []) | length' 2>/dev/null)"
    echo "$st ${n:-0}"
  done
}

# Emits "<thread>/<path> DIVERGED|ABSENT" per bad attachment at $1.
diverged_at() {
  local rev="$1" only="${2:-}" tj st atts sha path have
  git rev-parse --verify --quiet "$rev^{commit}" >/dev/null || return 0
  for tj in $(git ls-tree -r --name-only "$rev" -- intent/st 2>/dev/null | grep '/thread\.json$'); do
    st="${tj#intent/st/}"; st="${st%/thread.json}"
    atts="$(git show "$rev:$tj" 2>/dev/null | jq -r '(.attachments // [])[] | "\(.sha256) \(.path)"' 2>/dev/null)"
    [ -n "$atts" ] || continue
    while read -r sha path; do
      [ -n "$path" ] || continue
      [ -z "$only" ] || grep -qxF "$st/$path" "$only" || continue
      if ! git cat-file -e "$rev:intent/st/$st/$path" 2>/dev/null; then
        echo "$st/$path ABSENT"; continue
      fi
      have="$(git cat-file blob "$rev:intent/st/$st/$path" | shasum -a 256)"
      [ "${have%% *}" = "$sha" ] || echo "$st/$path DIVERGED"
    done <<< "$atts"
  done
}

REV="HEAD" HIST=0 EXHAUSTIVE=0
while [ $# -gt 0 ]; do
  case "$1" in
    --history) HIST="${2:?--history needs a count}"; shift 2 ;;
    --exhaustive) EXHAUSTIVE=1; shift ;;
    -*) die "unknown option: $1" ;;
    *) REV="$1"; shift ;;
  esac
done

# ---------------------------------------------------------------- population
# NAMED FIRST AND UNCONDITIONALLY. A verdict that cannot say which things it
# describes is not a verdict, and a count is the only thing that says the
# instrument reached its subject at all.
subjects="$(threads_at "$REV")"
[ -n "$subjects" ] || die "CANNOT MEASURE -- $REV carries no thread.json anywhere under intent/st"
total=0
while read -r st n; do [ -n "$st" ] && total=$((total + n)); done <<< "$subjects"
# NAMED COMPACTLY, AND WHAT IS ENUMERATED IS THE GAP RATHER THAN THE COVERAGE.
# ic's bar says name every subject; the literal reading printed 57 lines of
# per-thread counts and buried the verdict, which fails ic's OWN truncation rule
# in the same breath. A count plus the threads that could NOT be measured says
# which things the verdict describes AND which it does not, in two lines.
nthreads="$(printf '%s\n' "$subjects" | grep -c .)"
unmeasured="$(printf '%s\n' "$subjects" | awk '$2 == 0 { print $1 }')"
nunmeasured="$(printf '%s\n' "$unmeasured" | grep -c .)"
# THE COUNT MUST CLOSE (ic): measured + unmeasurable = population, in ONE line.
# Without that, "enumerate only the gap" becomes a way to hide a THIRD category
# -- a subject neither measured nor declared unmeasurable. The closure is what
# makes an absence admissible instead of a summary that quietly drops rows.
echo "canon-commit: $REV -- $total recorded attachment(s), across $((nthreads - nunmeasured)) measured + $nunmeasured unmeasurable = $nthreads thread(s)."
[ "$nunmeasured" -eq 0 ] ||
  echo "canon-commit: NOT EXAMINED -- $nunmeasured thread(s) record zero attachments: $(printf '%s\n' "$unmeasured" | tr '\n' ' ')"

if [ "$total" -eq 0 ]; then
  echo "canon-commit: CANNOT MEASURE -- every thread at $REV records zero attachments, so nothing was compared." >&2
  echo "    This is NOT a pass. Measured over the commits from 0ec2ac79 (canon's first) to dec6b1b9:" >&2
  echo "    86 of 132 recorded no attachments. That figure names THAT range and no other -- the range" >&2
  echo "    grows, and 'in this history' would have read as all 2184 commits, which it never meant." >&2
  exit 2
fi

if [ "$HIST" -gt 0 ]; then
  revs="$(git log --format=%h -"$HIST" "$REV" | tail -r 2>/dev/null || git log --format=%h -"$HIST" "$REV" | tac)"
  prev="" measured=0 anydis=0 adds=0 vac=0
  for r in $revs; do
    rtot=0
    while read -r st n; do [ -n "$st" ] && rtot=$((rtot + n)); done <<< "$(threads_at "$r")"
    if [ "$rtot" -eq 0 ]; then vac=$((vac + 1)); prev=""; continue; fi
    cur="$(diverged_at "$r" | awk '{print $1}' | sort)"
    measured=$((measured + 1))
    [ -n "$cur" ] && anydis=$((anydis + 1))
    new="$(comm -23 <(printf '%s\n' "$cur" | grep -v '^$') <(printf '%s\n' "$prev" | grep -v '^$') 2>/dev/null)"
    [ -n "$new" ] && { adds=$((adds + 1)); echo "canon-commit: ADDS at $r -- $(printf '%s\n' "$new" | tr '\n' ' ')"; }
    prev="$cur"
  done
  echo "canon-commit: ADDING count $adds -- over $measured measured commit(s); $anydis disagreed at all (inherited included); $vac skipped as unmeasurable (zero attachments)."
  echo "canon-commit: QUOTE THE ADDING COUNT. The raw count includes every commit that inherited a divergence it did not create."
  echo "canon-commit: the walk REPORTS and never gates -- only a single-revision run returns a finding."
  exit 0
fi

# ------------------------------------------------------------------ narrowing
# A NEW divergence can only arise among attachments where THIS COMMIT changed
# either the attachment's own bytes or its thread's canon. Everything else has
# the same status it had in the parent, so it is inherited BY CONSTRUCTION and
# can never be an ADD. Examining all 278 took 5.1s, which is slower than the
# slowest gated instrument in the roster; narrowed it is a fraction of that,
# and the narrowing is stated in the output rather than left to be discovered.
# --exhaustive turns it off and examines everything.
ONLY="" scoped=""
if [ "$EXHAUSTIVE" -eq 0 ] && git rev-parse --verify --quiet "$REV^{commit}" >/dev/null &&
   git rev-parse --verify --quiet "$REV^^{commit}" >/dev/null; then
  changed="$(git diff-tree --no-commit-id --name-only -r "$REV" 2>/dev/null)"
  ONLY="$(mktemp)"; trap 'rm -f "$ONLY"' EXIT
  # a thread whose canon moved: every one of its attachments is back in scope
  printf '%s\n' "$changed" | grep '^intent/st/.*/thread\.json$' | while read -r tj; do
    st="${tj#intent/st/}"; st="${st%/thread.json}"
    git show "$REV:$tj" 2>/dev/null | jq -r --arg st "$st" '(.attachments // [])[] | "\($st)/\(.path)"' 2>/dev/null
  done >> "$ONLY"
  # an attachment whose own bytes moved
  printf '%s\n' "$changed" | grep '^intent/st/' | grep -v '/thread\.json$' |
    sed 's|^intent/st/||' >> "$ONLY"
  sort -u "$ONLY" -o "$ONLY"
  scoped="$(grep -c . "$ONLY")"
  echo "canon-commit: EXAMINED $scoped of $total -- narrowed to the attachment path(s) whose bytes or whose thread's canon THIS COMMIT changed. The other $((total - scoped)) carry their parent's status by construction and cannot be an ADD. --exhaustive examines all $total."
fi

cur="$(diverged_at "$REV" ${ONLY:+"$ONLY"} | sort)"
curp="$(printf '%s\n' "$cur" | awk '{print $1}' | grep -v '^$' | sort)"

if [ -z "$curp" ]; then
  if [ -n "$ONLY" ]; then
    echo "canon-commit: ADDS 0 -- of the $scoped attachment(s) this commit could have changed, none diverges from the bytes $REV holds."
    [ "$scoped" -gt 0 ] || echo "    This commit touched no attachment bytes and no thread's canon, so there was nothing it could have added."
  else
    echo "canon-commit: ADDS 0 -- all $total examined attachment(s) match the bytes $REV holds at their paths."
  fi
  echo "canon-commit: GATES on what this commit ADDS; inherited divergences are reported, never failed on."
  echo "canon-commit: REACH -- attachments only. Criteria, status fields and notes are invisible to this tool."
  exit 0
fi

parent="$(diverged_at "$REV^" ${ONLY:+"$ONLY"} | awk '{print $1}' | grep -v '^$' | sort)"
new="$(comm -23 <(printf '%s\n' "$curp") <(printf '%s\n' "$parent") 2>/dev/null)"
inherited="$(comm -12 <(printf '%s\n' "$curp") <(printf '%s\n' "$parent") 2>/dev/null)"

[ -z "$inherited" ] || {
  echo "canon-commit: INHERITED $(printf '%s\n' "$inherited" | grep -c .) -- present in $REV^ too, so $REV did not introduce them:"
  printf '%s\n' "$inherited" | sed 's/^/    /'
  echo "    Never blocked: a guard that must be bypassed to work is one nobody keeps."
}

if [ -z "$new" ]; then
  echo "canon-commit: ADDS 0 of ${scoped:-$total} attachment(s) examined -- nothing this commit introduced."
  echo "canon-commit: REACH -- attachments only. Criteria, status fields and notes are invisible to this tool."
  exit 0
fi

echo "canon-commit: ADDS $(printf '%s\n' "$new" | grep -c .) of ${scoped:-$total} attachment(s) examined -- $REV names bytes it does not contain:" >&2
printf '%s\n' "$new" | sed 's/^/    /' >&2
echo "    Canon was written from the WORKTREE while these files were uncommitted." >&2
echo "    Stage them into this commit, or re-sync canon after committing them." >&2
echo "canon-commit: REACH -- attachments only. Criteria, status fields and notes are invisible to this tool." >&2
exit 1
