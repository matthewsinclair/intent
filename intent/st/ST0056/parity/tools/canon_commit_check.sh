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

# ------------------------------------------------------------- canon layout
# THE ONLY PLACE THE ON-DISK LAYOUT IS WRITTEN DOWN. It was written down ELEVEN
# times, across four concerns, and ic measured what that costs when ST0057 WP-01
# moves canon to a flat `intent/.canon/st/<ID>.json`: the pathspec stops
# matching, the `/thread.json$` filter cannot match a flat file, and the id
# extraction returns a WRONG VALUE rather than an error. All three fail toward
# "clean" -- an empty loop is a clean pass over nothing -- in the instrument that
# gates ST0057 AC-03.6. And fixing any one alone produces no observable change,
# which is what would make a partial fix read as done.
#
# THE ROOT CAUSE IS A PARAMETER-EXPANSION PROPERTY, MEASURED IN BASH RATHER THAN
# READ FROM THE MANUAL: `${var#pat}` and `${var%pat}` return the string UNCHANGED
# when the pattern is absent -- no error, no empty value, rc=0. So a half-migrated
# extraction emitted `ST0056.json` as a steel-thread id and it flowed downstream
# looking entirely plausible. `att_dir_of` below therefore ASSERTS that its strip
# changed the string, which is that finding turned into a control.
#
# ic's rule -- an extractor that takes identity from CONTENT is immune to a
# relocation by construction, one that takes it from the PATH is not -- is why
# `id_at` reads `.id` out of the blob. It does NOT reach the second half:
# `att_dir_of` rebuilds a PATH from parts, and no content-derived id fixes that.
# Different failure, so it gets its own function and its own assertion.
#
# THE LAYOUT IS DETECTED PER REV RATHER THAN CONFIGURED, and that is deliberate:
# a constant would have to be changed at the moment WP-01 lands, by someone who
# remembers this file exists, and the failure mode of forgetting is a silent zero.
# Detection cannot be forgotten. An ambiguous tree REFUSES, an empty one refuses
# upstream at CANNOT MEASURE, and the gate path compares $REV against $REV^ --
# which straddle the migration commit and legitimately carry different layouts.
CANON_NESTED='^intent/st/[^/][^/]*/thread\.json$'   # today
CANON_FLAT='^intent/\.canon/st/[^/][^/]*\.json$'     # after ST0057 WP-01
CANON_ROOTS='^intent/st/|^intent/\.canon/st/'        # both, for scoping a file list
CANON_PATHSPEC=(intent/st intent/.canon)             # both, for scoping a git call

# Emits every thread canon file at $1, one per line. ONE ls-tree, both roots.
canon_files_at() {
  local rev="$1" all nested flat
  all="$(git ls-tree -r --name-only "$rev" -- "${CANON_PATHSPEC[@]}" 2>/dev/null)"
  nested="$(printf '%s\n' "$all" | grep -E "$CANON_NESTED" || true)"
  flat="$(printf '%s\n' "$all" | grep -E "$CANON_FLAT" || true)"
  if [ -n "$nested" ] && [ -n "$flat" ]; then
    echo "error: AMBIGUOUS CANON LAYOUT at $rev -- $(printf '%s\n' "$nested" | grep -c .) nested and $(printf '%s\n' "$flat" | grep -c .) flat canon file(s)." >&2
    echo "    A half-migrated tree cannot be measured: either set is a plausible whole population," >&2
    echo "    so any count would close over the wrong one. Finish or revert the move, then re-run." >&2
    return 2
  fi
  printf '%s\n' "$nested$flat" | grep -v '^$' || true
}

# The thread id, READ FROM THE CANON'S CONTENT. Empty (never a guess) on absence;
# every caller checks, because `die` inside $( ) exits only the subshell.
id_at() {
  git show "$1:$2" 2>/dev/null | jq -r '.id // empty' 2>/dev/null
}

# The directory a thread's attachment bytes live under, derived from its canon
# FILE so the two can never disagree.
#   nested  intent/st/<ID>/thread.json   -> intent/st/<ID>
#   flat    intent/.canon/st/<ID>.json   -> intent/st/<ID>
#
# **THE FLAT ARM SAID `intent/.canon/st/<ID>` UNTIL 2026-08-19 AND THAT WAS A
# PREDICTION, NOT A MEASUREMENT.** The comment here read _the ID-KEYED DIRECTORY
# survives both layouts; only its parent moves_ -- written BEFORE ST0057 WP-01
# landed, describing a move that had not happened yet. **The move did something
# else: it relocated the canon FILE into a separate tree and left the ATTACHMENT
# DIRECTORY exactly where it was.** `intent/st/ST0001/design.md` is still
# `intent/st/ST0001/design.md`; `intent/.canon/st/ST0001` does not exist and
# never did.
#
# **THE COST OF THAT ONE WRONG WORD WAS A TOTAL FALSE POSITIVE**: on the first
# whole-tree run after the move, all 279 recorded attachments reported as ADDS
# -- `HEAD names bytes it does not contain` -- because the tool was resolving
# every one of them under a directory that does not exist. A tool that reports
# 279 of 279 divergent is not reporting 279 divergences; it is reporting its own
# resolution failure in the vocabulary of a finding, **and it exits 1 while doing
# it, which is the shape of a gate that would have been believed.**
#
# The lesson is the one this file already carries in another form: a strip whose
# pattern was absent returns the string unchanged. Here the strip SUCCEEDED and
# produced a well-formed path to nowhere, which no assertion about the strip can
# catch. **The id is taken from the BASENAME, and the parent is stated as the
# literal it is rather than derived from where the canon happens to sit.**
att_dir_of() {
  local d base
  case "$1" in
    intent/.canon/st/*.json)
      base="${1##*/}"
      d="intent/st/${base%.json}" ;;
    intent/st/*/thread.json) d="${1%/thread.json}" ;;
    *) echo "error: not a thread canon file, so it has no attachment directory: $1" >&2; return 2 ;;
  esac
  # THE ASSERTION IS THE POINT: a strip whose pattern was absent returns the
  # string unchanged at rc=0, which is how a wrong id shipped looking plausible.
  [ "$d" != "$1" ] || { echo "error: layout strip did not change $1 -- refusing to emit a path built from a failed strip" >&2; return 2; }
  printf '%s' "$d"
}

# Emits the blob path of every RECORDED attachment at $1, one per line. This is
# the population "examined" can be drawn from, and nothing else is.
att_blobs_at() {
  local rev="$1" tj adir files
  files="$(canon_files_at "$rev")" || return 2
  while read -r tj; do
    [ -n "$tj" ] || continue
    adir="$(att_dir_of "$tj")" || return 2
    git show "$rev:$tj" 2>/dev/null | jq -r --arg d "$adir" '(.attachments // [])[] | "\($d)/\(.path)"' 2>/dev/null
  done <<< "$files"
}

# Emits "<thread> <count>" per thread carrying canon at $1.
threads_at() {
  local rev="$1" tj st n files
  git rev-parse --verify --quiet "$rev^{commit}" >/dev/null || return 0
  files="$(canon_files_at "$rev")" || return 2
  while read -r tj; do
    [ -n "$tj" ] || continue
    st="$(id_at "$rev" "$tj")"
    [ -n "$st" ] || {
      echo "error: CANNOT MEASURE -- canon at $rev:$tj carries no .id, so its attachments cannot be named." >&2
      echo "    Canon without an id is a defect in the canon, not in this tool. Refusing rather than" >&2
      echo "    dropping the thread: a population silently reduced by one still closes arithmetically." >&2
      return 2; }
    n="$(git show "$rev:$tj" 2>/dev/null | jq '(.attachments // []) | length' 2>/dev/null)"
    echo "$st ${n:-0}"
  done <<< "$files"
}

# Emits "<thread>/<path> DIVERGED|ABSENT" per bad attachment at $1.
diverged_at() {
  local rev="$1" only="${2:-}" tj st adir atts sha path blob have files
  git rev-parse --verify --quiet "$rev^{commit}" >/dev/null || return 0
  files="$(canon_files_at "$rev")" || return 2
  while read -r tj; do
    [ -n "$tj" ] || continue
    st="$(id_at "$rev" "$tj")"
    [ -n "$st" ] || { echo "error: CANNOT MEASURE -- canon at $rev:$tj carries no .id" >&2; return 2; }
    adir="$(att_dir_of "$tj")" || return 2
    atts="$(git show "$rev:$tj" 2>/dev/null | jq -r '(.attachments // [])[] | "\(.sha256) \(.path)"' 2>/dev/null)"
    [ -n "$atts" ] || continue
    while read -r sha path; do
      [ -n "$path" ] || continue
      # KEYED ON THE BLOB PATH, NOT ON THE ID. The narrowing file is built from
      # `git diff-tree` output, which is paths; keying both sides on the path
      # means they agree by construction even if a thread's directory name and
      # its content id ever disagree. The id is used for the LABEL only.
      blob="$adir/$path"
      [ -z "$only" ] || grep -qxF "$blob" "$only" || continue
      if ! git cat-file -e "$rev:$blob" 2>/dev/null; then
        echo "$st/$path ABSENT"; continue
      fi
      have="$(git cat-file blob "$rev:$blob" | shasum -a 256)"
      [ "${have%% *}" = "$sha" ] || echo "$st/$path DIVERGED"
    done <<< "$atts"
  done <<< "$files"
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
subjects="$(threads_at "$REV")" || exit 2
[ -n "$subjects" ] || die "CANNOT MEASURE -- $REV carries thread canon under neither intent/st/<ID>/thread.json nor intent/.canon/st/<ID>.json"
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
  echo "    This is NOT a pass. THE FIGURE BELOW IS RECORDED, NOT COMPUTED BY THIS RUN (dc, 2026-08-18)," >&2
  echo "    and saying so matters because the branch printing it has just reported that it measured nothing:" >&2
  echo "    over the 132 commits from 0ec2ac79 (canon's first) to dec6b1b9, 86 recorded no attachments." >&2
  echo "    It names THAT range and no other, this run did not recount it, the range grows, and" >&2
  echo "    'in this history' would have read as all 2184 commits, which it never meant." >&2
  exit 2
fi

if [ "$HIST" -gt 0 ]; then
  revs="$(git log --format=%h -"$HIST" "$REV" | tail -r 2>/dev/null || git log --format=%h -"$HIST" "$REV" | tac)"
  prev="" measured=0 anydis=0 adds=0 vac=0 rthreads=""
  for r in $revs; do
    rtot=0
    rthreads="$(threads_at "$r")" || exit 2
    while read -r st n; do [ -n "$st" ] && rtot=$((rtot + n)); done <<< "$rthreads"
    if [ "$rtot" -eq 0 ]; then vac=$((vac + 1)); prev=""; continue; fi
    cur="$(diverged_at "$r" | awk '{print $1}' | sort)" || exit 2
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
# can never be an ADD.
#
# TIMING, AND BOTH FIGURES ARE MEASURED RATHER THAN QUOTED, BECAUSE THE FIRST
# VERSION OF THIS COMMENT WAS WRONG TWICE OVER. At 8d0e8736 on two machines with
# `/usr/bin/time -p`: exhaustive 9.5-9.7s, narrowed 2.1-2.3s, against
# view_skew_check.sh -- the slowest gated instrument -- at 2.87-2.97s. So
# narrowed it is the SECOND-slowest thing in the gate, with a 25% margin, not
# the comfortable one the first comment claimed.
#
# RE-MEASURED AT 4ba598f1 AFTER THE LAYOUT REWRITE, ON ONE MACHINE ONLY (the
# figures above are two-machine; this is not, and the difference is part of the
# figure): narrowed 2.49-2.55s, exhaustive 11.3-11.5s. IT GOT ~1.8x SLOWER ON
# PURPOSE. The `scoped` count used to be the size of the narrowing FILTER, not
# the number of attachments examined; correcting it costs one extra pass over
# every thread canon (`att_blobs_at`). Four full passes now where there were
# three. THE COST BUYS A COUNT THAT CLOSES OVER WHAT WAS EXAMINED -- which is
# the property this whole tool exists to assert about somebody else, so it does
# not get to ship a verdict line that lacks it.
#
# The two errors are worth keeping because both favoured the conclusion their
# author wanted. (1) The times came from zsh's builtin `time` applied to a
# SUBSHELL, which under-reported wall clock by roughly half -- 5.1s for a 9.6s
# run. (2) They were compared against `3077ms` READ OUT OF THE ROSTER STRING in
# runner_roster_check.sh: a figure recorded on another machine at another time
# over a smaller tree. Measured-against-recorded is not a comparison. ic
# measured all three on one machine and the discrepancy was theirs to find.
#
# --exhaustive turns the narrowing off and examines everything.
ONLY="" scoped="" adir="" blobs=""
if [ "$EXHAUSTIVE" -eq 0 ] && git rev-parse --verify --quiet "$REV^{commit}" >/dev/null &&
   git rev-parse --verify --quiet "$REV^^{commit}" >/dev/null; then
  changed="$(git diff-tree --no-commit-id --name-only -r "$REV" 2>/dev/null)"
  ONLY="$(mktemp)"; trap 'rm -f "$ONLY"' EXIT
  # a thread whose canon moved: every one of its attachments is back in scope
  printf '%s\n' "$changed" | grep -E "$CANON_NESTED|$CANON_FLAT" | while read -r tj; do
    adir="$(att_dir_of "$tj")" || exit 2
    git show "$REV:$tj" 2>/dev/null | jq -r --arg d "$adir" '(.attachments // [])[] | "\($d)/\(.path)"' 2>/dev/null
  done >> "$ONLY"
  # an attachment whose own bytes moved -- the blob path IS the key, so no strip
  # and no id are involved. Over-inclusive by design: a non-attachment under
  # either root adds a key nothing matches, which widens scope and never narrows it.
  printf '%s\n' "$changed" | grep -E "$CANON_ROOTS" |
    grep -vE "$CANON_NESTED|$CANON_FLAT" >> "$ONLY"
  sort -u "$ONLY" -o "$ONLY"
  # INTERSECT WITH THE RECORDED ATTACHMENTS. Without this, `scoped` counts FILTER
  # KEYS rather than attachments examined, and the two are different populations:
  # the changed-file half sweeps in every file under the canon roots, attachment
  # or not. Latent in the nested layout, where it merely overstated (86 of 278);
  # the flat layout pushed it PAST the total and printed "EXAMINED 2 of 1 ... the
  # other -1". A count that closes arithmetically over the wrong population is the
  # defect this tool exists to find, so it does not get to ship one.
  # Behaviourally a no-op: `diverged_at` only ever looks up recorded attachments,
  # so a key that is not one is never consulted. This corrects the COUNT alone.
  blobs="$(att_blobs_at "$REV")" || exit 2
  comm -12 "$ONLY" <(printf '%s\n' "$blobs" | sort -u) > "$ONLY.i" && mv "$ONLY.i" "$ONLY"
  scoped="$(grep -c . "$ONLY")"
  echo "canon-commit: EXAMINED $scoped of $total -- narrowed to the attachment path(s) whose bytes or whose thread's canon THIS COMMIT changed. The other $((total - scoped)) carry their parent's status by construction and cannot be an ADD. --exhaustive examines all $total."
fi

cur="$(diverged_at "$REV" ${ONLY:+"$ONLY"} | sort)" || exit 2
curp="$(printf '%s\n' "$cur" | awk '{print $1}' | grep -v '^$' | sort)"

if [ -z "$curp" ]; then
  if [ -n "$ONLY" ]; then
    echo "canon-commit: ADDS 0 -- of the $scoped attachment(s) this commit could have changed, none diverges from the bytes $REV holds."
    [ "$scoped" -gt 0 ] || echo "    This commit touched no attachment bytes and no thread's canon, so there was nothing it could have added."
  else
    echo "canon-commit: ADDS 0 -- all $total examined attachment(s) match the bytes $REV holds at their paths."
  fi
  if [ -n "$ONLY" ]; then
    echo "canon-commit: GATES on what this commit ADDS."
    echo "canon-commit: INHERITED -- NOT EXAMINED in narrowed mode, and this is structural rather than an omission."
    echo "    An inherited divergence is BY DEFINITION in a path this commit did not touch, which is exactly"
    echo "    what the narrowing excludes. The ADDS proof does not license the inherited arm. --exhaustive sees them."
  else
    echo "canon-commit: GATES on what this commit ADDS; inherited divergences are reported, never failed on."
  fi
  echo "canon-commit: REACH -- attachments only. Criteria, status fields and notes are invisible to this tool."
  exit 0
fi

parent="$(diverged_at "$REV^" ${ONLY:+"$ONLY"} | awk '{print $1}' | grep -v '^$' | sort)" || exit 2
new="$(comm -23 <(printf '%s\n' "$curp") <(printf '%s\n' "$parent") 2>/dev/null)"
inherited="$(comm -12 <(printf '%s\n' "$curp") <(printf '%s\n' "$parent") 2>/dev/null)"

# THE INHERITED ARM IS STRUCTURALLY OUTSIDE THE NARROWING, AND SAYING SO IS THE
# WHOLE FIX (ic). An inherited divergence is BY DEFINITION in a path this commit
# did not touch -- which is precisely the population the narrowing excludes. So
# the ADDS proof, that only touched paths can be a NEW divergence, licenses the
# ADDS half and nothing else. Narrowed, vc measured INHERITED 1 where the truth
# was 2; ic measured the section VANISHING ENTIRELY while two existed, one of
# them this file's own canon record -- an instrument blind to its own record,
# whose negative result was a fact about the instrument. And the narrowed run
# was printing "inherited divergences are reported" while reporting none: a
# guard declaring an arm it does not run, which is the defect this tool's author
# fixed in self_provenance_check.sh the same afternoon, reappearing in the line
# added here to prevent it. The verdict was correct throughout; the promise
# outran the mode.
if [ -n "$ONLY" ]; then
  echo "canon-commit: INHERITED -- NOT EXAMINED in narrowed mode, and structurally so, not by omission."
  echo "    An inherited divergence is by definition in a path this commit did not touch, which is exactly"
  echo "    what the narrowing excludes. Any count printed here would be over the wrong population."
  echo "    --exhaustive examines all $total and reports them."
elif [ -n "$inherited" ]; then
  echo "canon-commit: INHERITED $(printf '%s\n' "$inherited" | grep -c .) of $total attachment(s) examined -- present in $REV^ too, so $REV did not introduce them:"
  printf '%s\n' "$inherited" | sed 's/^/    /'
  echo "    Never blocked: a guard that must be bypassed to work is one nobody keeps."
fi

if [ -z "$new" ]; then
  echo "canon-commit: ADDS 0 of ${scoped:-$total} attachment(s) examined -- nothing this commit introduced."
  echo "canon-commit: REACH -- attachments only. Criteria, status fields and notes are invisible to this tool."
  exit 0
fi

echo "canon-commit: ADDS $(printf '%s\n' "$new" | grep -c .) of ${scoped:-$total} attachment(s) examined -- $REV names bytes it does not contain:" >&2
printf '%s\n' "$new" | sed 's/^/    /' >&2
echo "    Canon was written from the WORKTREE while these files were uncommitted." >&2
echo "    THE ORDER MATTERS AND THE OBVIOUS ONE DOES NOT WORK. Sync canon FIRST -- it reads the" >&2
echo "    WORKTREE -- then commit the file(s) and canon together. Committing first and re-syncing" >&2
echo "    after leaves THIS commit divergent in history permanently: the later sync fixes the next" >&2
echo "    commit and can never fix this one. The criterion is a property of every commit, not of HEAD." >&2
echo "canon-commit: REACH -- attachments only. Criteria, status fields and notes are invisible to this tool." >&2
exit 1
