#!/bin/bash
# rig_selftest.sh -- drive interrupt_rig.sh against stubs whose behaviour is known,
# and score every arm against a prediction written before the run.
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
#   REACHABLE, JUST NOT WRITTEN YET
#     the workdir-inside-this-repository guard -- the ledger owns the workdir
#     path, so driving it means handing the rig one inside the checkout. It is
#     the guard that stops the rig migrating the live repository with four
#     sessions working in it, and it is the most consequential undriven one.
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
# SCOPE GOES IN A DENOMINATOR, NEVER IN AN ADJECTIVE. 17 of 24, and the seven are
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
    --help|-h) sed -n '2,30p' "$0"; exit 0 ;;
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

while IFS='|' read -r name mode store_mode read_mode _spare extra_args want_exit want_phrase claim; do
  [ -n "$name" ] || continue
  [ -z "$ONLY" ] || [ "$ONLY" = "$name" ] || continue

  wd="$WORKROOT/$name"
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
  echo
  LEDGER="$LEDGER
  $(printf '%-14s want exit %s  got %-3s  phrase %-3s  %s' "$name" "$want_exit" "$got_exit" "$got_phrase" "$verdict")"
done <<EOF
$CASES
EOF
$CASES
EOF

echo "=============================================================="
echo "LEDGER -- rig $RIG_SHA ($RIG_DIRTY)$LEDGER"
echo
if [ "$FAILS" -eq 0 ]; then
  echo "rig-selftest: $PASSES of $((PASSES + FAILS)) cases scored as predicted"
  exit 0
fi
echo "rig-selftest: $FAILS of $((PASSES + FAILS)) cases scored OFF PREDICTION -- read the logs named above"
exit 1
