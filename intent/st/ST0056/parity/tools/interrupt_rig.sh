#!/bin/bash
# interrupt_rig.sh -- produce the two trees the cutover gate compares.
#
# hv gated the v3 cutover on a property: a second migration over an INTERRUPTED
# estate must reach the same end state as a clean one. `same_end_state_check.sh`
# (dc) is the verdict on two trees. This is the thing that MAKES the two trees,
# and the only part of the job it does itself is the interruption.
#
# THIS IS NOT A CHECK AND IS DELIBERATELY NOT NAMED `*_check.sh`. It produces
# subjects and delegates the verdict; a second file ending in `_check.sh` would
# read as a second opinion on the same question, which is the duplication hv's
# constraint forbids. The roster enumerates `*_check.sh`, so this correctly has
# no roster row. Its exit code is dc's tool's exit code, except for its own
# refusals.
#
#   estates   vc's estate_corpus.sh   -- pinned, verified against git's own hashes
#   verdict   dc's same_end_state_check.sh
#   here      the kill, and the refusals that stop a vacuous kill reporting green
#
# WHY IT REFUSES ON A DIRTY `native/rust/`, WHICH IS THE POINT OF THE FILE.
# On 2026-08-17 three instruments in this directory were found reading the
# WORKING TREE where they meant the COMMIT -- `runner_roster_check.sh` (dc found
# it), `residue_class_check.sh` two hours later (same defect, sibling file), and
# dc's prepush clone caught HEAD not building because one node had committed a
# consumer while another held the producer. Four sessions share one checkout, so
# a green measured here is otherwise a claim about the union of five people's
# uncommitted work.
#
# That class is worse for THIS tool than for any of those. Those report drift;
# this one reports the condition hv put the cutover behind. A gate that passed
# against 201 uncommitted lines would be a true statement about bytes no commit
# contains, and nothing in its output would say so. So the refusal is structural
# and comes before anything is built or written.
#
# MEASURED, NOT ASSUMED: at 17:44Z on 2026-08-17 the crate COMPILED CLEAN while
# `Facade::upgrade` was 201 uncommitted lines and `intent upgrade` was not wired
# to the CLI at all. ic's own board had recorded the blocker as "the compile
# error in facade.rs". It compiled, and the gate was still unrunnable. A green
# build is not evidence about a commit.
#
# WHY THE MIGRATING COMMAND IS A PARAMETER AND THERE IS NO COPY OF THE DOOR HERE.
# A rig calling `Facade::upgrade` directly would be a second caller of the door
# cc owns, with a known expiry the day `intent upgrade` is wired -- and cc ruled
# against widening `facade::apply` or adding a test seam, because a migration
# dies when the PROCESS dies, not inside a function. So the rig runs a command
# and kills it. Today the command does not exist and this refuses; the day it is
# wired, this runs unchanged.
#
# WHY THE KILL LANDS ON A MEASURED THRESHOLD AND NEVER ON A DELAY.
# The first version of this used guessed delays and wrote ZERO files at 12.8ms,
# because harness process startup outlasts every delay worth guessing. dc's
# sharpening is why it matters more than it looks: A KILL THAT LANDS TOO EARLY
# WRITES NOTHING, SO THE RE-RUN TRIVIALLY MATCHES AND THE ARM REPORTS IDENTICAL
# WITHOUT HAVING INTERRUPTED ANYTHING. The failure mode of this rig is a false
# GREEN, so every way it can be vacuous refuses instead.
#
# The threshold is a FACT ABOUT THE WORKLOAD rather than a guess about timing:
# the clean run in tree A happens first and its file delta is counted, so the
# kill in tree B fires at a fraction of a number that was measured minutes ago
# on the same estate and the same binary.
#
# Exit codes follow the family: 0 clean, 1 a finding, 2 cannot measure.
#
# PROVEN IN THREE DIRECTIONS, AND THE PROOF NAMES A REVISION. Against
# `same_end_state_check.sh` at `11f66894`, verified clean in the worktree at the
# time of the run, with three stubs standing in for the migrator:
#
#   idempotent stub   SIGKILL at 37/40   IDENTICAL   exit 0
#   accreting stub    SIGKILL at 37/40   DIFFERENT   exit 1   (accretion named)
#   instant stub      no kill possible   REFUSED     exit 2   (vacuous arm)
#
# All three against ONE revision on purpose. An earlier set spanned three
# revisions of the comparator -- including one edited in place WHILE a run was
# inside it, which bash reads incrementally and which therefore cannot be
# reasoned about after the fact. That run was discarded rather than argued from,
# because "it reported IDENTICAL and IDENTICAL is what I expected" is the
# confirmation shape this file exists to refuse. **A claim of proof that names no
# revision is not a claim of proof**, and the arm that cannot be salvaged by
# reasoning is always the one that produces the green.
#
# Both controls, not just the passing one: a rig never shown reporting RED is a
# fixture calibrated to its own answer.
#
# IT IS SLOW AND IT IS NOT HUNG. A canary run is several minutes: two migrations
# plus a re-run, then a three-subject comparison that hashes every file in three
# trees one `shasum` process at a time. Measured 2026-08-17: a five-minute
# ceiling was not enough, and the first symptom was silence, because the caller
# had piped the output through `tail`. Watch the log rather than the clock --
# every stage announces itself before it starts.



set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../.." && pwd)"

die() { echo "interrupt-rig: $*" >&2; exit 2; }
say() { echo "interrupt-rig: $*"; }

MEMBER="canary"
WORKDIR=""
KEEP=0
# Fraction of the clean run's file delta at which the kill fires. Late on
# purpose: the accretion this gate exists to catch lives in the generated views,
# which are written after the canon, so a kill in the first third would leave
# the interesting half of the run unexercised.
FRACTION=90

while [ $# -gt 0 ]; do
  case "$1" in
    --member) MEMBER="${2:-}"; shift 2 || die "--member needs a value" ;;
    --fraction) FRACTION="${2:-}"; shift 2 || die "--fraction needs a value" ;;
    --keep) KEEP=1; shift ;;
    --help|-h)
      echo "usage: interrupt_rig.sh [--member <id>] [--fraction <1-99>] [--keep] [<workdir>]"
      echo "env:   MIGRATE_CMD  the command that performs the migration, run with cwd = the tree"
      exit 0 ;;
    -*) die "unknown option: $1" ;;
    *) WORKDIR="$1"; shift ;;
  esac
done

case "$FRACTION" in
  ''|*[!0-9]*) die "--fraction must be a whole number, got: $FRACTION" ;;
esac
{ [ "$FRACTION" -ge 1 ] && [ "$FRACTION" -le 99 ]; } ||
  die "--fraction must be between 1 and 99 -- 100 is not an interruption and 0 writes nothing"

CORPUS="$HERE/estate_corpus.sh"
VERDICT="$HERE/same_end_state_check.sh"
[ -x "$CORPUS" ] || die "cannot execute $CORPUS -- the estates come from vc's tool, not from here"
[ -x "$VERDICT" ] || die "cannot execute $VERDICT -- the verdict is dc's tool, not this one"

# ---------------------------------------------------------------------------
# The subject: what migrates, and is it a commit?
# ---------------------------------------------------------------------------

# An explicit MIGRATE_CMD is a deliberate act by someone running this by hand --
# the same clause as `residue_class_check.sh`'s CONTRACT/SCANNER overrides -- and
# it is how this rig's OWN machinery is proven against a stub whose behaviour is
# known. It buys out of the build and the cleanliness refusal together, because
# neither describes a command this tool did not produce. It says so loudly: a
# run under an override is a run about the override.
MIGRATE_GIVEN="${MIGRATE_CMD:+yes}"

if [ -n "$MIGRATE_GIVEN" ]; then
  say "MIGRATE_CMD OVERRIDE IN FORCE -- the subject is '$MIGRATE_CMD', not the workspace binary."
  say "  This run says NOTHING about any commit of intentsvcs. It exercises this rig."
else
  # THE INSTRUMENTS ARE CHECKED ALONGSIDE THE SUBJECT, and leaving them out was
  # the first version's blind spot. This rig calls two of its siblings --
  # `estate_corpus.sh` for the estates and `same_end_state_check.sh` for the
  # verdict -- from the worktree. A gate result assembled from a committed
  # migrator, a peer's half-edited capture tool and a half-edited comparator is
  # no more a statement about a commit than one measured over a dirty migrator.
  # Caught live: vc relocated `CAPTURE` out of the captured tree mid-session and
  # the estate went 1078 files to 1077 between two runs an hour apart, with
  # nothing in either run's output saying the instrument had moved.
  dirty="$(git -C "$ROOT" status --porcelain -- native/rust/ "$HERE" 2>/dev/null)"
  if [ -n "$dirty" ]; then
    echo "interrupt-rig: REFUSING -- the subject or its instruments have uncommitted changes." >&2
    echo "" >&2
    printf '%s\n' "$dirty" | sed 's/^/    /' >&2
    echo "" >&2
    echo "  This gate reports the condition hv put the v3 cutover behind. Measured" >&2
    echo "  against a dirty worktree it would be a true statement about bytes no" >&2
    echo "  commit contains, in a checkout four sessions share." >&2
    echo "" >&2
    echo "  Remedy: commit the migrator and the parity tools, or pass MIGRATE_CMD" >&2
    echo "  to say explicitly that the subject is something other than this" >&2
    echo "  workspace -- an override run makes no claim about any commit, so it" >&2
    echo "  does not check either." >&2
    exit 2
  fi

  HEAD_SHA="$(git -C "$ROOT" rev-parse --short HEAD)" || die "cannot read HEAD"
  say "subject: native/rust/ and the parity tools clean at $HEAD_SHA -- building the migrator from it"

  # The binary is built HERE so that it provably corresponds to the tree just
  # verified clean. A pre-built binary of unknown vintage passing a cleanliness
  # check on the source is the same category error the check exists to stop.
  ( cd "$ROOT/native/rust" && cargo build --release -p intent-cli ) >/dev/null 2>&1 ||
    die "cargo build --release -p intent-cli failed -- cannot migrate with a binary that does not build"

  BIN="$ROOT/native/rust/target/release/intent"
  [ -x "$BIN" ] || die "built, but no executable at $BIN -- the binary's name or path has moved"

  # THERE IS DELIBERATELY NO `upgrade --help` PROBE HERE, and the reason is
  # measured. On 2026-08-17 `intent upgrade` was ADVERTISED in `--help` and
  # unimplemented: `--help` exited 0 while the verb itself returned "a known
  # command that is not implemented yet" at exit 2. A probe that asks clap
  # whether a verb is spelled correctly answers a question nobody asked, and
  # would have waved this rig through to produce a green from a migrator that
  # never opened. The only honest probe is running it, which arm A does; the
  # unwired case is named there instead.
  MIGRATE_CMD="$BIN upgrade"
fi

# ---------------------------------------------------------------------------
# The estates: captured ONCE, copied twice.
# ---------------------------------------------------------------------------

if [ -z "$WORKDIR" ]; then
  WORKDIR="$(mktemp -d "${TMPDIR:-/tmp}/interrupt-rig.XXXXXX")" || die "cannot create a workdir"
  OWNED=1
else
  mkdir -p "$WORKDIR" || die "cannot create $WORKDIR"
  OWNED=0
fi

# THE WORKDIR MUST BE OUTSIDE THIS REPOSITORY, and this is not tidiness.
# `Project::discover` walks `ancestors()` from the cwd to the first
# `intent/.config/config.json`. The rig runs the migrator with cwd inside a tree
# it built, so if that tree were ever missing its own marker, discovery would
# keep walking UP -- and a workdir inside this checkout puts the live Intent
# project on that path. The rig would then migrate the repository it is being
# developed in, with four sessions working in it.
#
# Two independent guards, because the cheap one is the one that fails silently:
# this, and the per-tree marker assertion after the copies. Either alone closes
# the hole; the pair survives one of them being wrong about the marker's name.
WORKDIR_REAL="$(cd "$WORKDIR" && pwd -P)" || die "cannot resolve $WORKDIR"
ROOT_REAL="$(cd "$ROOT" && pwd -P)" || die "cannot resolve $ROOT"
case "$WORKDIR_REAL/" in
  "$ROOT_REAL"/*)
    die "the workdir $WORKDIR_REAL is inside this repository ($ROOT_REAL) -- project discovery walks upward from the tree being migrated, so a workdir here can reach the live checkout. Use a path outside the repository." ;;
esac

cleanup() {
  if [ "$KEEP" -eq 0 ] && [ "$OWNED" -eq 1 ]; then rm -rf "$WORKDIR"; fi
}
trap cleanup EXIT

TEMPLATE="$WORKDIR/template"
A="$WORKDIR/a-clean"
B="$WORKDIR/b-interrupted"

say "capturing $MEMBER via estate_corpus.sh"
"$CORPUS" capture "$MEMBER" "$TEMPLATE" >/dev/null ||
  die "estate_corpus.sh could not capture $MEMBER -- run '$CORPUS list' to see why"

# CAPTURED ONCE AND COPIED, NEVER CAPTURED TWICE. `capture` stamps `captured_at`
# into the tree's CAPTURE file, so two captures differ in a file the migration
# never touches -- and the verdict is a whole-tree diff, so it would report
# DIFFERENT on the rig's own provenance record. Copying also makes "both arms
# start from identical bytes" true by construction rather than by argument.
cp -R "$TEMPLATE" "$A" || die "cannot copy the template to $A"
cp -R "$TEMPLATE" "$B" || die "cannot copy the template to $B"

# EACH TREE MUST HOLD ITS OWN PROJECT MARKER. `Project::discover` stops at the
# FIRST ancestor carrying `intent/.config/config.json`, so a marker here means
# the migrator can never resolve a root outside the tree it was pointed at --
# which is the property that makes `cd <tree> && <migrate>` safe to run at all.
# Asserted rather than assumed: a capture that silently produced a subtree
# without the config would send discovery hunting up the filesystem, and the
# first thing it found would be migrated instead.
#
# cc's note is the reason this is checked per-tree rather than once: cwd may be
# ANY directory inside the project and discovery still finds the root, so an arm
# pointed at the wrong place tends to succeed rather than fail. The resolved root
# is what matters, and the marker is what fixes it.
for t in "$A" "$B"; do
  [ -f "$t/intent/.config/config.json" ] ||
    die "$t holds no intent/.config/config.json -- discovery would walk out of this tree and migrate whatever it found above it"
done

count_files() { find "$1" -type f 2>/dev/null | wc -l | tr -d ' '; }

BASE_N="$(count_files "$TEMPLATE")"
[ "$BASE_N" -gt 0 ] || die "the captured estate holds no files"
say "estate: $BASE_N files, two identical copies"

# ---------------------------------------------------------------------------
# Arm A: the clean run. Also the measurement the kill threshold comes from.
# ---------------------------------------------------------------------------

say "arm A: clean run"
( cd "$A" && eval "$MIGRATE_CMD" ) >"$WORKDIR/a.log" 2>&1
A_STATUS=$?
A_N="$(count_files "$A")"
A_DELTA=$((A_N - BASE_N))

if [ "$A_STATUS" -ne 0 ]; then
  say "the clean run exited $A_STATUS -- last lines of its output:"
  tail -20 "$WORKDIR/a.log" | sed 's/^/    /'
  # The unwired door is named specifically, because it is the failure this rig
  # will actually meet and "a clean run that fails" would send the reader into
  # the migrator instead of into the dispatch table.
  if grep -q "not implemented yet" "$WORKDIR/a.log" 2>/dev/null; then
    die "the migrating command is a known verb that is NOT WIRED (exit $A_STATUS). Every invocation would write nothing, both trees would stay equal to the input, and a whole-tree diff would report IDENTICAL from a migrator that never opened. Refusing rather than measuring that."
  fi
  die "a clean run that fails is not a baseline; there is nothing to compare against"
fi

# THE KILL THRESHOLD IS A FRACTION OF THIS NUMBER, so a clean run that adds no
# files leaves nothing to fire on. Framed as what it is -- this rig cannot do its
# job -- rather than as a verdict on the migration.
#
# "DID THE MIGRATION CHANGE ANYTHING AT ALL" IS DELIBERATELY NOT ASKED HERE. It
# is dc's, in `same_end_state_check.sh`, derived from the comparison already in
# that file rather than keyed to a canon filename that a rename would silently
# retire. An earlier version of this line asked it badly -- it tested for NEW
# FILES, which falsely refuses a migration that only rewrites in place -- and two
# tools answering one question with different semantics is how the answer starts
# depending on which one you ran.
[ "$A_DELTA" -gt 0 ] ||
  die "the clean run added no files ($BASE_N -> $A_N), so there is no delta to place a kill inside. This rig interrupts by file count; a migrator that only rewrites in place needs a different sentinel, not a different fraction."

say "arm A: wrote $A_DELTA files ($BASE_N -> $A_N)"

THRESHOLD=$((BASE_N + (A_DELTA * FRACTION / 100)))
[ "$THRESHOLD" -gt "$BASE_N" ] ||
  die "the kill threshold computed to the starting file count -- $A_DELTA files at ${FRACTION}% rounds to zero, so the kill could only land before any write"
say "kill threshold: $THRESHOLD files (${FRACTION}% of the measured delta)"

# ---------------------------------------------------------------------------
# Arm B: kill it for real, then re-run.
# ---------------------------------------------------------------------------

say "arm B: starting the migration to interrupt it"
( cd "$B" && eval "$MIGRATE_CMD" ) >"$WORKDIR/b1.log" 2>&1 &
CHILD=$!

# Bounded, because an unbounded poll on a tree walk is how a previous version of
# this spent seven minutes walking 400k paths before a timeout killed it.
POLL_LIMIT=6000   # 6000 * 0.01s = 60s
polls=0
killed=0
at_kill=0

while [ "$polls" -lt "$POLL_LIMIT" ]; do
  if ! kill -0 "$CHILD" 2>/dev/null; then break; fi
  n="$(count_files "$B")"
  if [ "$n" -ge "$THRESHOLD" ]; then
    at_kill="$n"
    kill -9 "$CHILD" 2>/dev/null
    killed=1
    break
  fi
  polls=$((polls + 1))
  sleep 0.01
done

wait "$CHILD" 2>/dev/null
B1_STATUS=$?

if [ "$killed" -eq 0 ]; then
  if [ "$polls" -ge "$POLL_LIMIT" ]; then
    die "the migration ran for 60s without reaching $THRESHOLD files -- cannot interrupt what will not get there"
  fi
  # THE ARM IS VACUOUS AND SAYS SO. The process finished before the threshold,
  # so nothing was interrupted; a re-run over a COMPLETE tree matching a clean
  # run measures idempotence, which is a different and easier property.
  die "the migration finished before the kill threshold -- this arm interrupted NOTHING, and a re-run over a complete tree would report IDENTICAL without testing interruption. Raise --fraction, or use a larger estate."
fi

# 128 + SIGKILL(9). A child that exited some other way was not killed by this
# rig, and the interruption it did suffer is not the one being reported.
if [ "$B1_STATUS" -ne 137 ]; then
  die "expected the interrupted run to exit 137 (SIGKILL), got $B1_STATUS -- the kill did not land, so this arm's interruption is not the one it claims"
fi

B_AT_KILL_DELTA=$((at_kill - BASE_N))
[ "$B_AT_KILL_DELTA" -gt 0 ] ||
  die "the kill landed with no files written -- an interruption before the first write leaves an unmigrated tree, which a re-run trivially matches"

say "arm B: SIGKILL landed at $at_kill files ($B_AT_KILL_DELTA written of $A_DELTA)"

say "arm B: re-running over the interrupted estate"
( cd "$B" && eval "$MIGRATE_CMD" ) >"$WORKDIR/b2.log" 2>&1
B2_STATUS=$?

if [ "$B2_STATUS" -ne 0 ]; then
  say "the re-run exited $B2_STATUS -- last lines of its output:"
  tail -20 "$WORKDIR/b2.log" | sed 's/^/    /'
  # NOT a refusal. A re-run that REFUSES to proceed over an interrupted estate is
  # a real finding about the property under test, not an inability to measure it.
  echo
  echo "FINDING -- the re-run over an interrupted estate did not succeed (exit $B2_STATUS)."
  echo "  Under hv's big-bang/fix-forward ruling the re-run IS the recovery path,"
  echo "  so a re-run that will not complete is the failure this gate looks for."
  exit 1
fi

# ---------------------------------------------------------------------------
# The verdict is dc's, not this file's.
# ---------------------------------------------------------------------------

echo
say "handing three subjects to same_end_state_check.sh"
echo
# THREE SUBJECTS, and the first one is why this rig captures once and copies.
# dc's tool refuses (exit 2) when the clean run left the input byte-identical --
# the green from a door that never opened. The input it needs is the PRISTINE
# estate, which is `$TEMPLATE`: never migrated, never the target of a run, and
# byte-identical to what both arms started from because they are copies of it.
#
# Not a fresh capture, which is the natural reading and is wrong: `capture`
# stamps `captured_at` into the tree, so a second capture of one pinned revision
# differs from the first in a file no migration touches, and a whole-tree diff
# reports that as the migration's doing.
"$VERDICT" "$TEMPLATE" "$A" "$B"
STATUS=$?

echo
case "$STATUS" in
  0) say "GATE ARM PASSED: interrupted at $B_AT_KILL_DELTA/$A_DELTA files, re-run reached the clean end state" ;;
  1) say "GATE ARM FAILED: interrupted at $B_AT_KILL_DELTA/$A_DELTA files, re-run did NOT reach the clean end state" ;;
  *) say "the verdict tool could not measure (exit $STATUS)" ;;
esac

if [ "$KEEP" -eq 1 ] || [ "$OWNED" -eq 0 ]; then
  say "trees kept: $A (clean) and $B (re-run); logs alongside them"
fi

exit "$STATUS"
