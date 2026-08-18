#!/bin/bash
# rig_selftest.sh -- drive interrupt_rig.sh against stubs whose behaviour is known,
# and score every arm against a prediction written before the run.
#
# AC-00.10 / AT-00.10: an instrument whose verdict gates a criterion must itself be
# DRIVEN -- its refusal paths exercised against known-bad inputs and scored against a
# prediction written before the run. This file is that row's cited evidence.
#
# AT-00.10 IS HELD RED AND THIS INSTRUMENT PASSES, so a reader arriving here hunting a
# broken selftest will not find one. Red because coverage is PARTIAL -- 18 of 24
# refusal sites, the six named below -- and because `rev_with_override`'s own
# `cannot resolve --rev` path is undriven. Never because the selftest fails.
#
# WHY THIS EXISTS, AND IT IS NOT TIDINESS. `MODULES.md` says the rig was "proven
# in three directions before use -- idempotent stub IDENTICAL, accreting stub
# DIFFERENT, instant stub REFUSED as vacuous". That proving happened once, by
# hand, and nothing re-ran it. Two defects then landed in the arm the whole gate
# exists for, and both survived four fleet estate runs:
#
#   `b96188d1` took the fork out of the poll loop and left a third reference to
#   the variable it deleted. Under `set -u` both vacuous-kill refusals stopped
#   refusing and started ABORTING -- exit 1, which is this rig's code for GATE
#   ARM FAILED, a claim about the migrator produced by a bug in the rig.
#
#   The kill signalled `$CHILD`, which is the SUBSHELL. The migrator is its
#   child. Measured: kill at 6 files, `wait` returns 137, tree grows to 48 and
#   the writer is still alive. Every assertion about the interruption passed and
#   nothing had been interrupted.
#
# Neither is subtle and neither was findable by reading, because both live on
# paths that only execute when something goes wrong -- and four real estates
# never went wrong in those two particular ways. **A refusal nothing has ever
# driven is not a refusal, it is a comment with a syntax error budget.**
#
# `--rig <path>` is the point rather than a convenience: point it at the previous
# revision of the rig and the cases that should be red go red. A ledger that has
# never been shown failing is the same fixture problem one level up.
#
#   ./rig_selftest.sh                     # score the sibling rig
#   ./rig_selftest.sh --rig /path/old.sh  # score a mutant, and watch it fail
#   ./rig_selftest.sh --only escape       # one case
#
# COST: no clone and no cargo build. `MIGRATE_CMD` buys out of both, and the
# estate is the smallest member in the corpus. Five cases in about a minute.
set -uo pipefail

# WHAT THIS DOES **NOT** DRIVE, LISTED BECAUSE 17 OF 17 READS AS COVERAGE AND IS
# NOT. The rig has 24 refusal sites; these are the ones nothing here reaches, and
# a green below says nothing about any of them:
#
#   NEEDS A NON-OVERRIDE RUN, SO A CLONE AND A BUILD
#     `cannot resolve --rev`, the clone failures, the dirty-clone assertion,
#     the cargo build failure, the per-tree config-marker assertion.
#
#   STRUCTURALLY OUT OF REACH OF A STUB
#     the mtime-ordering failures (709, 711); the 120s poll timeout, which is
#     reachable but costs 120s a run; the kill-already-finished race (825) and
#     the not-137 exit (828), both of which need a race won on purpose.
#
# SCOPE GOES IN A DENOMINATOR, NEVER IN AN ADJECTIVE. 18 of 24, and the six are
# named above rather than left for a reader to discover by not finding them.


HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RIG="$HERE/interrupt_rig.sh"
STUB="$HERE/rig_stub_migrator.sh"
MEMBER="baize"
ONLY=""
WORKROOT="${RIG_SELFTEST_WORKROOT:-${TMPDIR:-/tmp}/rig-selftest}"

die() { echo "rig-selftest: $*" >&2; exit 2; }

while [ $# -gt 0 ]; do
  case "$1" in
    --rig) RIG="${2:-}"; shift 2 || die "--rig needs a value" ;;
    --member) MEMBER="${2:-}"; shift 2 || die "--member needs a value" ;;
    --only) ONLY="${2:-}"; shift 2 || die "--only needs a value" ;;
    # THE RANGE IS COUPLED TO THE HEADER'S LENGTH AND NOTHING REPORTS WHEN IT SLIPS.
    # Adding lines above silently truncates this help from the bottom -- it was already
    # cutting the usage block mid-list at `2,30` before AC-00.10's marker was inserted.
    # If you edit the header, re-check that `--help` still ends at the last usage line.
    --help|-h) sed -n '2,41p' "$0"; exit 0 ;;
    *) die "unknown option: $1" ;;
  esac
done

[ -x "$RIG" ]  || die "no executable rig at $RIG"
[ -x "$STUB" ] || die "no executable stub at $STUB"

# THE SUBJECT IS NAMED, because the whole failure this file answers is a claim
# that outlived the thing it was true of. A run that does not say which rig it
# measured is another one of those.
RIG_SHA="$(git -C "$HERE" log -1 --format=%h -- "$RIG" 2>/dev/null || echo unknown)"
RIG_DIRTY="clean"
git -C "$HERE" diff --quiet -- "$RIG" 2>/dev/null || RIG_DIRTY="UNCOMMITTED"

echo "rig-selftest: rig      $RIG"
echo "rig-selftest:          last commit touching it: $RIG_SHA ($RIG_DIRTY)"
echo "rig-selftest: estate   $MEMBER"
echo "rig-selftest: workroot $WORKROOT"
echo

# name | migrate mode | store mode | read mode | expected exit | expected phrase | what it proves
#
# EVERY REFUSAL IN THE RIG IS DRIVEN HERE, and that is the point rather than
# thoroughness for its own sake: a refusal nothing has ever executed is a comment
# with a syntax-error budget, which is exactly how both of this morning's defects
# lived through four fleet estate runs.
CASES='
workdir_in_repo|pass|||1||2|is inside this repository|THE ONLY UNDRIVEN REFUSAL WHOSE FAILURE IS NOT RECOVERABLE BY RE-RUNNING: the guard that stops the rig migrating the checkout it is developed in, with four sessions working in it. Scored on containment as well as on the exit code, because a refusal that fired for another reason would also exit 2
pass|pass|||||0|GATE ARM PASSED|the rig can report green when the property holds -- the control, without which every red below is a fixture that only knows one answer
diverge|diverge|||||1|GATE ARM FAILED|the verdict is coupled to the bytes: a migrator stamping a per-run nonce cannot make the two arms agree
nosentinel|nosentinel|||||2|interrupted NOTHING|the vacuous-kill refusal fires when the run ends without reaching the sentinel -- THE PATH b96188d1 BROKE
nowrite|nowrite|||||2|added no files|a migrator that writes nothing leaves nowhere to place a kill, and that is a refusal rather than a pass
escape|escape|||||2|KILL DID NOT STOP THE MIGRATION|the settle assertion is coupled: a writer that escapes the process group is caught by measuring the tree, not the exit status
liveness|pass||dead|||2|does not answer|the liveness arm refuses an estate no verb can open -- this rig once returned exit 0 over 1371 identical files that every command but `info` rejected
store_same|pass|same||||0|STORE: IDENTICAL|the store arm actually compares, and prints the event count beside the verdict so a trivially-equal zero is visible in the run
store_differ|pass|differ||||1|STORE: DIFFERENT|a store the re-run did not reproduce is a failure of the property even with the files identical -- D01 as reversed
store_live|pass|live||||2|THE EVENT LOG IS LIVE|the arm refuses rather than byte-comparing minted ULIDs and DDL-defaulted timestamps, which differ BY CONSTRUCTION for a correct migrator
store_nokey|pass|nokey||||2|did not read as an array|absence must not be spelled the same way as emptiness: `jq .events | length` gives 0 for a MISSING key, so the arm asks for the type first
store_dead_b|pass|dead_b||||1|exactly one arm answers|one usable project and one dead one is the end states DISAGREEING at exit 1, not an inability to measure at exit 2
unwired|unwired|||||2|NOT WIRED|the unwired-door refusal: a verb advertised in --help that returns `not implemented yet` writes nothing, and a whole-tree diff would call two untouched trees IDENTICAL
cleanfail|cleanfail|||||2|not a baseline|a clean run that fails leaves nothing to compare against, and that is an inability to measure rather than a property failing
rerunfail|rerunfail|||||1|FINDING|THE GATE ARM THE WHOLE THING EXISTS FOR: a re-run that will not complete over an interrupted estate is the failure hv gated the cutover on, and it is exit 1 rather than a refusal
bad_fraction|pass||||--fraction 100|2|must be between 1 and 99|argument validation refuses before any estate is captured -- 100 is not an interruption and 0 writes nothing
rev_with_override|pass||||--rev no-such-ref-here|2|will not choose between them|TWO SUBJECTS AT ONCE. This case was written to drive the `cannot resolve --rev` refusal, scored OFF PREDICTION at exit 0, and the miss WAS the finding: under an override no revision is resolved, so the flag was accepted and silently discarded. The rig now refuses. **The `cannot resolve` path itself remains UNDRIVEN** -- reaching it needs a non-override run, which needs a clone and a build
bad_member|pass||||--member no-such-member|2|could not capture|an estate the corpus cannot produce is a refusal, not an empty tree that would compare equal to itself
'

PASSES=0; FAILS=0; LEDGER=""

while IFS='|' read -r name mode store_mode read_mode wd_in_repo extra_args want_exit want_phrase claim; do
  [ -n "$name" ] || continue
  [ -z "$ONLY" ] || [ "$ONLY" = "$name" ] || continue

  # A WORKDIR INSIDE THE RIG'S OWN REPOSITORY IS A CASE, NOT AN ACCIDENT.
  #
  # The guard it drives is the one that stops the rig migrating the checkout it
  # is developed in -- `Project::discover` walks `ancestors()` from the tree
  # being migrated, so a workdir here puts the live repository on that path.
  # **It is the only refusal on this file's undriven list whose failure is not
  # recoverable by re-running**, which is why vc asked for it ahead of the five
  # that need a clone and a build.
  #
  # THE RIG CREATES THE WORKDIR BEFORE IT CHECKS IT (`mkdir -p` at the top, the
  # containment test twenty lines later), so this probe directory really does
  # appear inside the repository for the length of one refusal. That is why the
  # case asserts afterwards that it is still EMPTY: an empty directory means the
  # guard fired before anything was built in it, and git does not track empty
  # directories so nothing is left behind either way.
  #
  # **DRIVE THIS AGAINST A THROWAWAY CLONE BEFORE YOU DRIVE IT HERE.** If the
  # guard is broken, running it against the live checkout IS the damage -- and
  # the three refusals driven on 2026-08-18 were all defective, so "it is
  # obviously fine" is precisely the assumption that had already failed.
  if [ -n "$wd_in_repo" ]; then
    rig_root="$(cd "$(dirname "$RIG")" && git rev-parse --show-toplevel 2>/dev/null)"
    [ -n "$rig_root" ] || die "case $name needs the rig's repository root and git would not name one"
    wd="$rig_root/.rig-selftest-inside-repo-probe"
  else
    wd="$WORKROOT/$name"
  fi
  rm -rf "$wd"; mkdir -p "$wd" || die "cannot create $wd"

  # EMPTY MEANS THE ARM DOES NOT RUN, which is a case in itself: the rig must say
  # DID NOT RUN rather than passing silently, and eight of the eleven cases leave
  # one or both empty precisely so that branch is exercised on every invocation.
  store_cmd=""; [ -n "$store_mode" ] && store_cmd="$STUB store"
  read_cmd="";  [ -n "$read_mode" ]  && read_cmd="$STUB read"

  echo "=============================================================="
  echo "CASE $name"
  echo "  PREDICTION (written before the run): exit $want_exit, output contains \"$want_phrase\""
  echo "  WHAT IT PROVES: $claim"
  echo

  STUB_MODE="$mode" STUB_STORE_MODE="${store_mode:-same}" STUB_READ_MODE="${read_mode:-ok}" \
    STUB_N=40 STUB_DELAY=0.02 \
    MIGRATE_CMD="$STUB" STORE_CMD="$store_cmd" READ_CMD="$read_cmd" \
    "$RIG" --member "$MEMBER" $extra_args "$wd" >"$wd/run.log" 2>&1
  got_exit=$?

  got_phrase=no
  grep -qF "$want_phrase" "$wd/run.log" && got_phrase=yes

  # THE EXIT CODE IS NOT THE PROPERTY HERE. A refusal that fired for some other
  # reason would also exit 2, so the case additionally measures what the guard
  # exists to prevent: did anything get BUILT inside the repository. Anything
  # beyond the run log this ledger redirected into it is the guard having failed
  # while still returning the right number.
  if [ -n "$wd_in_repo" ]; then
    stray="$(find "$wd" -mindepth 1 ! -name run.log 2>/dev/null | wc -l | tr -d ' ')"
    if [ "$stray" -ne 0 ]; then
      echo "  CONTAINMENT: **$stray ENTRIES WERE BUILT INSIDE THE REPOSITORY** at $wd -- the guard did not stop the run."
      find "$wd" -mindepth 1 ! -name run.log 2>/dev/null | head -10 | sed 's/^/      /'
      got_phrase=no
    else
      echo "  CONTAINMENT: nothing was built inside the repository -- the guard fired before the workdir was used."
    fi
  fi

  if [ "$got_exit" -eq "$want_exit" ] && [ "$got_phrase" = yes ]; then
    verdict="AS PREDICTED"; PASSES=$((PASSES + 1))
  else
    verdict="OFF PREDICTION"; FAILS=$((FAILS + 1))
  fi

  echo "  RESULT: exit $got_exit, phrase present: $got_phrase  -- $verdict"
  # THE LAST LINES ALWAYS, not only on a miss. A case that scored as predicted
  # for a reason nobody read is the same problem in a better mood.
  tail -6 "$wd/run.log" | sed 's/^/      /'
  echo "      full log: $wd/run.log"
  # A probe directory inside the repository does not outlive its case. Copied
  # out first so the log survives for reading, then removed.
  if [ -n "$wd_in_repo" ]; then
    mkdir -p "$WORKROOT/$name" && cp "$wd/run.log" "$WORKROOT/$name/run.log" 2>/dev/null
    rm -rf "$wd"
    echo "      probe directory removed from the repository; log copied to $WORKROOT/$name/run.log"
  fi
  echo
  LEDGER="$LEDGER
  $(printf '%-14s want exit %s  got %-3s  phrase %-3s  %s' "$name" "$want_exit" "$got_exit" "$got_phrase" "$verdict")"
done <<EOF
$CASES
EOF

echo "=============================================================="
echo "LEDGER -- rig $RIG_SHA ($RIG_DIRTY)$LEDGER"
echo

# THE SCOPE PRINTS ON EVERY RUN, PASS OR FAIL, BECAUSE THIS FILE ALREADY SAYS SO AND
# DID NOT DO IT. Line 60 reads "SCOPE GOES IN A DENOMINATOR, NEVER IN AN ADJECTIVE --
# 18 of 24" and that sentence was in a COMMENT, while the output printed `N of N cases
# scored as predicted` -- a perfect score over the population this file chose, with
# nothing telling the reader six refusal sites are undriven. **A limit stated in a
# comment is not stated to the reader of the output** (vc, 2026-08-18, third instance
# of the class that evening).
#
# THE TWO HALVES CARRY DIFFERENT WARRANTS AND ARE PRINTED DIFFERENTLY ON PURPOSE. The
# driven count is COMPUTED from the case table below, so it cannot drift from what ran.
# The 24 is a HAND COUNT of the rig's refusal sites and no mechanical count reproduces
# it (`exit 2` gives 6, die-calls 42, both 30), so it is labelled as recorded rather
# than measured. Printing it bare would make a hand count look like a measurement,
# which is the defect this whole directory keeps finding.
DRIVEN_TOTAL="$(printf '%s\n' "$CASES" | grep -c '|')"
echo "rig-selftest: SCOPE -- this drives $DRIVEN_TOTAL of the rig's 24 refusal sites."
echo "  24 is RECORDED, not measured: a hand count, unreproduced by any mechanical"
echo "  count of the rig. The 6 undriven sites are named in this file's header under"
echo "  'WHAT THIS DOES NOT DRIVE' -- five need a non-override run (a clone and a"
echo "  cargo build), the rest are out of a stub's reach. A green below covers none"
echo "  of them, and \`rev_with_override\` does NOT drive \`cannot resolve --rev\`."
if [ "$((PASSES + FAILS))" -ne "$DRIVEN_TOTAL" ]; then
  echo "  NOTE: this run scored $((PASSES + FAILS)), not $DRIVEN_TOTAL -- the case set was filtered (--only)."
fi
echo

if [ "$FAILS" -eq 0 ]; then
  echo "rig-selftest: $PASSES of $((PASSES + FAILS)) cases scored as predicted"
  exit 0
fi
echo "rig-selftest: $FAILS of $((PASSES + FAILS)) cases scored OFF PREDICTION -- read the logs named above"
exit 1
