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
# WHY THE KILL WAITS ON A MEASURED SENTINEL AND NEVER ON A DELAY OR A COUNT.
# The first version of this used guessed delays and wrote ZERO files at 12.8ms,
# because harness process startup outlasts every delay worth guessing. dc's
# sharpening is why it matters more than it looks: A KILL THAT LANDS TOO EARLY
# WRITES NOTHING, SO THE RE-RUN TRIVIALLY MATCHES AND THE ARM REPORTS IDENTICAL
# WITHOUT HAVING INTERRUPTED ANYTHING. The failure mode of this rig is a false
# GREEN, so every way it can be vacuous refuses instead.
#
# Counting files replaced the delay and was ALSO measured failing, for a reason
# that is a property of the thing under test rather than of the loop: the writes
# are a 73ms burst at the end of a 134ms run, because nothing lands until
# `WriteSet::commit()` -- the same design that makes AC-10.2 structural. A `find`
# over the tree costs ~20ms, so a counting poll sees the whole window three or
# four times and the kill arrives after the process has exited.
#
# So the sentinel is a PATH taken from the clean arm's observed write order, and
# waiting on it is a single `test -e`. A delay is a guess about someone else's
# timing; a COUNT is a guess about how fast you can observe them. **A sentinel is
# a fact about their progress** -- and the file is chosen at a measured depth of
# the same workload, so "kill it late" is something this can actually do.
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
      echo "       READ_CMD     a READ verb proving the migrated estate is usable (liveness)"
      echo "       STORE_CMD    a read that answers FROM THE STORE, compared across both arms"
      echo
      echo "  The last two default to \`intent st list\` and \`intent export --format json\`"
      echo "  on the workspace binary. Under a MIGRATE_CMD override they default to EMPTY,"
      echo "  and each arm then reports that it DID NOT RUN rather than passing silently."
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

# Bound before the branch, because `set -u` is on and the workspace path below
# is the only place these get their defaults. An override run inherits whatever
# the caller exported and otherwise leaves them empty -- which the liveness and
# store arms report as NOT RUN rather than treating as satisfied.
READ_CMD="${READ_CMD:-}"
STORE_CMD="${STORE_CMD:-}"

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

  # THE LIVENESS PROBE AND THE STORE PROBE, both read-only, both through the
  # shipped surface. `st list` is the cheapest verb that depends on project
  # state; `info` would answer on a corpse, which is exactly the property that
  # disqualifies it. `export --format json` reads the STORE rather than the
  # files (`facade.rs:1146` -- `load_canon()` + `events()`), which is what makes
  # it the store comparison dc's file-level verdict deliberately does not make.
  READ_CMD="$BIN st list"
  STORE_CMD="$BIN export --format json"
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
# A marker stamped immediately before the run, so "files this migration wrote"
# is decided by a clock reading rather than by the template directory's own
# mtime -- `cp -R` does not preserve times, so every copied file is newer than
# the thing it was copied from and `-newer $TEMPLATE` would match all of them.
touch "$WORKDIR/.arm-a-start"
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

# THE SENTINEL IS CHOSEN FROM THIS SET, so a clean run that adds no files leaves
# nothing to wait for. Framed as what it is -- this rig cannot do its job --
# rather than as a verdict on the migration.
#
# **A BLOCKED MIGRATION LANDS HERE, AND THAT IS THE RIGHT PLACE FOR IT TO LAND.**
# `plan()` returns an uncommitted `WriteSet`, so a refusal writes nothing at all
# -- measured on Lamplight, which blocked on 15 residue findings across 3 live
# threads and left 5613 of 5613 files byte-identical to the pin. That estate has
# no delta by construction, so the interruption property is untestable on it and
# this says so rather than reporting a pass or a fault. Two results, not one:
# AC-10.1's refusal arm exercised, AC-10.2's interruption arm not applicable.
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

# ---------------------------------------------------------------------------
# LIVENESS: can the tool read the estate it just migrated?
# ---------------------------------------------------------------------------
#
# THIS ARM EXISTS BECAUSE THIS RIG ALREADY PASSED ON AN ESTATE NO VERB COULD
# OPEN. On 2026-08-17 the gate returned exit 0 against `252f9ed2` -- 1371 of
# 1371 files identical, verified independently with `diff -r` -- and every
# command but `info` refused that same tree: `st list`, `todo`, `export`,
# `search`, `ac list`, `wp list`, and `st new` on the write side. The
# comparison was correct and it was a comparison of two estates nobody can use.
# **A re-run reaching the same end state says nothing about whether that end
# state is a working project**, and those are two questions, not one.
#
# It is the same shape as vc's conservation green -- ALTERED 0 / ADDED 0 on
# three fleet members that were all inoperable after converting -- and the
# reason neither instrument caught it is that both were asking after the BYTES.
# Conservation and liveness cannot see each other, so the gate has to ask both.
#
# A REFUSAL RATHER THAN A FAILURE, and the distinction is load-bearing. Exit 2
# says the gate could not run; exit 1 would say the migration is not idempotent,
# which is a claim about cc's code that this measurement does not support. The
# interruption property may well hold on an estate that cannot be read -- it did
# yesterday -- so reporting a dead subject as a FAILED gate would send the next
# reader into `migrate.rs` looking for a bug that is not there.
if [ -n "$READ_CMD" ]; then
  ( cd "$A" && eval "$READ_CMD" ) >"$WORKDIR/a.read.log" 2>&1
  A_READ_STATUS=$?
  if [ "$A_READ_STATUS" -ne 0 ]; then
    say "arm A migrated cleanly and the tool will not read the result:"
    head -3 "$WORKDIR/a.read.log" | sed 's/^/    /'
    die "the migrated estate does not answer \`$READ_CMD\` (exit $A_READ_STATUS). Comparing two trees neither of which the tool can open would be a true statement about bytes and a false impression of a working cutover -- this rig produced exactly that result on 2026-08-17 and it read as a pass.

  KNOWN CAUSE AS AT 2026-08-17, so this does not send you hunting: v2 buckets
  every thread by STATUS FROM CREATION (\`intent st new\` writes to
  \`intent/st/NOT-STARTED/<ID>/\`), the migration writes canon to the flat
  \`intent/st/<ID>/\` and correctly leaves the v2 originals alone, and
  \`Project::legacy_thread_ids\` then counts those same originals as unmigrated
  because it asks whether a \`thread.json\` sits BESIDE each \`info.md\` rather
  than whether the thread's id has canon anywhere. Measured 100% false
  positives fleet-wide (vc: bucketed-only = 0 on utilz, baize and this repo).
  cc owns the supersession fix in \`project.rs\`; it is not a fault of the
  migrator, and it is not a fault of this rig's subject either.

  If that fix has landed and this still fires, the cause is NEW and the message
  above is stale -- trust the output, not this note."
  fi
  say "arm A: liveness ok -- the migrated estate answers \`$READ_CMD\`"
else
  # NOT SILENT. An arm that quietly does not run is indistinguishable from one
  # that found nothing, which is this board's most-repeated lesson and the
  # reason this branch prints rather than skips.
  say "arm A: LIVENESS ARM DID NOT RUN -- MIGRATE_CMD is overridden and no READ_CMD was given, so nothing here establishes that the migrated estate is usable"
fi

# ---------------------------------------------------------------------------
# The sentinel: WHICH file, chosen from the clean run's observed write order.
# ---------------------------------------------------------------------------
#
# COUNTING FILES CANNOT LAND A KILL HERE, AND THE REASON IS THE ATOMICITY DESIGN
# RATHER THAN A SLOW LOOP. Measured 2026-08-17 against the real migrator on the
# canary: the whole run is 134ms and the WRITE BURST IS 73ms of it -- 295 files.
# Nothing is written until `WriteSet::commit()`, which is precisely what makes
# AC-10.2 structural, and the consequence is that the file count goes from zero
# to complete in under a tenth of a second. A poll loop doing `find` over a
# 1372-file tree costs ~20ms an iteration, so it gets three or four looks at the
# entire window and the kill lands after the process has already exited. Both
# `--fraction 90` and `--fraction 50` raced every time and refused, correctly.
#
# SO THE SENTINEL IS A PATH, NOT A COUNT, AND THE PATH IS MEASURED. The clean arm
# ran first, so its files can be sorted by modification time -- APFS records
# fractional seconds, which is ample resolution across a 73ms burst -- and the
# file at the requested percentile of that order becomes the thing arm B waits
# for. Waiting is then a single `test -e`, microseconds rather than milliseconds,
# so the poll is faster than the burst instead of slower than it.
#
# THIS IS NOT THE GUESSED DELAY THIS FILE WARNS ABOUT, and the distinction is the
# whole point. The first version of this rig slept for a guessed interval from
# process launch and wrote ZERO files at 12.8ms, because startup outlasted every
# delay worth guessing. A delay is a guess about someone else's timing. **A
# sentinel is a fact about their progress**: this waits for a specific file that
# the same migrator wrote at a known depth of the same workload, and if that file
# never appears the poll times out and refuses rather than killing blind.
#
# **THE PERCENTILE IS APPROXIMATE AND LANDS LATE. STATED BECAUSE IT WAS MEASURED,
# AND BECAUSE AN EARLIER VERSION OF THIS COMMENT CLAIMED OTHERWISE.** `--fraction`
# selects by MTIME order, and mtime order is not the order the paths APPEAR:
# `WriteSet::commit()` writes temp-and-renames, so a file's mtime is when its
# temp was written while `test -e` can only see it after the rename. Measured on
# the canary: `--fraction 25` picks a file whose mtime is 16ms into the burst and
# lands the kill at 263 of 295 writes, where 25% would be about 74. `--fraction
# 90` and `--fraction 75` both land at 293.
#
# So the honest claim is that this rig lands a GENUINE DEEP interruption and can
# move it somewhat earlier, not that it can place the kill at a chosen depth.
# Deep is what the gate needs -- a shallow kill leaves nothing to recover -- so
# the limitation costs nothing here, but a future arm that needs an EARLY kill
# will need appearance order, which means observing arm A rather than stat-ing it
# afterwards.
say "deriving the kill sentinel from the clean run's write order"

SENTINEL_REL="$(
  cd "$A" || exit 1
  find . -type f -newer "$WORKDIR/.arm-a-start" -exec stat -f '%Fm %N' {} + 2>/dev/null |
    sort -n |
    awk -v frac="$FRACTION" '
      { path[NR] = $2 }
      END {
        if (NR == 0) exit 1
        i = int(NR * frac / 100)
        if (i < 1) i = 1
        if (i > NR) i = NR
        print path[i]
      }'
)"

[ -n "$SENTINEL_REL" ] ||
  die "could not order the clean run's writes by mtime, so there is no measured point at which to kill. Without an ordering the only options are a guessed delay or a count, and both have been measured failing on this workload."
[ -f "$A/$SENTINEL_REL" ] ||
  die "the chosen sentinel $SENTINEL_REL is not a file in the clean tree -- the mtime ordering produced a path that does not exist"

say "kill sentinel: ${SENTINEL_REL#./} (at ${FRACTION}% of $A_DELTA writes by observed order)"

# ---------------------------------------------------------------------------
# Arm B: kill it for real, then re-run.
# ---------------------------------------------------------------------------

say "arm B: starting the migration to interrupt it"
( cd "$B" && eval "$MIGRATE_CMD" ) >"$WORKDIR/b1.log" 2>&1 &
CHILD=$!

# A TIGHT SPIN WITH NO SLEEP, because the window is 73ms and any sleep worth
# writing is a large fraction of it. The body is one `test -e` -- a single stat,
# microseconds -- so this polls far faster than the burst writes, which is the
# whole reason the sentinel is a path and not a count.
#
# Bounded anyway: an unbounded poll is how a previous version spent seven minutes
# walking 400k paths before a timeout killed it. The bound here is wall-clock
# rather than an iteration count, because iterations are now far too cheap to
# reason about as a duration.
# NOTHING IN THIS LOOP MAY FORK, and that is a measured requirement rather than
# a style preference. The first version of this spin called `date +%s` every
# iteration to check its deadline -- a fork and an exec, milliseconds each, in a
# window where the migrator writes a file every ~0.25ms. Measured: the sentinel
# chosen at 25% of the writes appears **16ms** into the burst, and the kill still
# landed at 293 of 295 files, because the loop was ~200 files behind by the time
# it looked. The `find`-based count was replaced with a single `test -e` for
# speed and a fork was left in beside it, which put the cost straight back.
#
# `$SECONDS` is a bash builtin, so the deadline costs nothing. `test -e` and
# `kill -0` are a builtin and a syscall. The loop body now allocates no process.
SECONDS=0
POLL_LIMIT_S=120
killed=0
at_kill=0

while :; do
  if [ -e "$B/$SENTINEL_REL" ]; then
    kill -9 "$CHILD" 2>/dev/null
    killed=1
    # Counted AFTER the signal, so this is an upper bound on what the run had
    # written when the kill was issued, not an exact figure. Reported as such.
    at_kill="$(count_files "$B")"
    break
  fi
  if ! kill -0 "$CHILD" 2>/dev/null; then break; fi
  if [ "$SECONDS" -ge "$POLL_LIMIT_S" ]; then break; fi
done

wait "$CHILD" 2>/dev/null
B1_STATUS=$?

if [ "$killed" -eq 0 ]; then
  if [ "$(date +%s)" -ge "$POLL_DEADLINE" ]; then
    die "the migration ran for 120s without ever writing $SENTINEL_REL -- cannot interrupt at a point the run does not reach. The sentinel came from the clean arm, so this means the two runs diverged."
  fi
  # THE ARM IS VACUOUS AND SAYS SO. The process finished without the sentinel
  # ever appearing, so nothing was interrupted; a re-run over a COMPLETE tree
  # matching a clean run measures idempotence, which is a different and easier
  # property than the one this gate is about.
  die "the migration finished without ever writing the sentinel $SENTINEL_REL -- this arm interrupted NOTHING, and a re-run over a complete tree would report IDENTICAL without testing interruption. The clean arm wrote that file, so the runs are not doing the same work."
fi

# 128 + SIGKILL(9). A child that exited some other way was not killed by this
# rig, and the interruption it did suffer is not the one being reported.
#
# THE `got 0` CASE IS A RACE AND ITS REMEDY IS THE OPPOSITE OF THE ONE ABOVE, so
# the two are separated rather than sharing a message. The threshold was reached
# and the kill was issued, but the migration finished in the interval between
# counting and signalling. Measured 2026-08-17 against the real migrator on the
# canary: 295 files at `--fraction 90` raced every time, because the writes land
# in a burst far faster than a `find` over a 1372-file tree can be walked, so
# 90% of the delta is already the last instant of the run.
#
# **This is the arm that would otherwise produce the most convincing false
# green**: an uninterrupted migration, a re-run over a complete tree, and an
# IDENTICAL from a comparator doing its job perfectly. Only the exit status
# distinguishes it, which is why the check is on the status and not on the tree.
if [ "$B1_STATUS" -eq 0 ]; then
  die "the kill was issued at $at_kill files but the migration had already finished -- exit 0, not 137. The threshold is too late for how fast this migrator writes: it reached ${FRACTION}% of the delta and completed before the signal landed. LOWER --fraction (try half of what you used); do not raise it. Nothing was interrupted, so a re-run here would report IDENTICAL over an uninterrupted migration."
fi
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

# ---------------------------------------------------------------------------
# THE STORE ARM -- the half dc's tool declares out of scope, in its own words:
# "1 store(s) differ and are NOT JUDGED BY THIS TOOL".
# ---------------------------------------------------------------------------
#
# WHY IT IS NOT A `cmp` ON `intent.db`, and this is measured rather than
# argued. Two CLEAN runs of a correct migrator produce databases that differ at
# byte 4796: `created_at`/`updated_at` carry
# `DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ'))` across 705 rows, so byte-identity
# is false by construction and a byte test would fail forever on correct code.
# Normalising those stamps out of a `.dump` makes the two identical across 827
# lines with zero residual -- but that route needs the `sqlite3` SHELL, and dc
# measured the consequence: same trees, `sqlite3` off PATH, exit 1 instead of
# exit 0, silently. A verdict that depends on what is installed is not a verdict.
#
# SO IT GOES THROUGH THE SHIPPED SURFACE. `intent export --format json` reads
# the STORE (`facade.rs:1146` -- `load_canon()` plus `events()`), not the files,
# so this is a genuine store comparison rather than a re-derivation of the file
# comparison above. It needs no `sqlite3`, and it keeps rusqlite out of
# `intent-cli`, which D06 forbids and `dep_graph_guard.rs` enforces. The model
# carries no record stamps -- they are DB columns, excluded from the model and
# from `derived_dump()` for this same reason -- so the output is stable.
#
# MEASURED BEFORE BEING RELIED ON: two independent migrations of one estate
# exported byte-identical, 225553 bytes, on 2026-08-17.
#
# **WHAT THIS ARM COMPARES IS THE STORE AS `export` SEES IT, WHICH IS NARROWER
# THAN THE STORE** (dc's question, and it is measured rather than conceded).
# `export` reads `load_canon()` + `events()`. Every table in a migrated store,
# counted on the gate's own clean tree 2026-08-17:
#
#   threads 56 / wps 140 / criteria 281 / tests 228 / issues 0 / related 0
#     -- load_canon, in this comparison
#   event_log 0                      -- events(), in this comparison
#   file_index 0 / doc_sections 0 / snapshots 0
#     -- NOT IN THIS COMPARISON
#
# The three are empty today, so nothing is currently hidden -- which is exactly
# why the bound is written now rather than when it starts to matter. `store.rs`
# excludes `file_index` and `doc_sections` from `rebuild` deliberately, because
# they derive from the WORKING TREE rather than from canon; that is a good
# reason for them to be unexported and no reason at all for this arm to claim
# them. They are NAMED rather than gestured at, so a later reader can check the
# list against the schema instead of trusting this comment.
#
# The general form is dc's and it is the same one that caught THEIR tool an hour
# earlier, one level down: **an instrument's population is what it READS, not
# what EXISTS**, and a scope line that enumerates the dimensions its author
# already thought of reads as completeness rather than as an omission.
#
# **THE EVENT HALF IS TRIVIALLY EQUAL TODAY AND WILL NOT STAY THAT WAY.** A
# migrated estate currently holds 0 events (measured: 0 `st.new`, 56 of 56
# threads carrying `created` from v2 frontmatter rather than deriving it), so
# the events array is empty in both arms and compares equal for a reason that
# has nothing to do with this property. When cc lands the event log, `ts` is
# DATABASE-supplied wall clock (D42) and two runs will differ there legitimately
# -- at which point this arm must normalise `ts`, not be deleted. Written down
# here because a green whose reason expires is the failure mode nobody re-reads.
STORE_NOTE=""
if [ "$STATUS" -eq 0 ] && [ -n "$STORE_CMD" ]; then
  ( cd "$A" && eval "$STORE_CMD" ) >"$WORKDIR/a.store.json" 2>"$WORKDIR/a.store.err"
  A_STORE_STATUS=$?
  ( cd "$B" && eval "$STORE_CMD" ) >"$WORKDIR/b.store.json" 2>"$WORKDIR/b.store.err"
  B_STORE_STATUS=$?

  if [ "$A_STORE_STATUS" -ne 0 ] && [ "$B_STORE_STATUS" -ne 0 ]; then
    # BOTH arms unreadable: cannot measure, which is not the same as measured
    # equal and must not be allowed to read as one. Liveness should already have
    # caught this via arm A, so reaching here means the estate died between the
    # two probes -- worth saying rather than assuming impossible.
    echo
    echo "  STORE ARM COULD NOT RUN -- \`$STORE_CMD\` exited $A_STORE_STATUS (clean) / $B_STORE_STATUS (re-run); NEITHER arm answers."
    head -2 "$WORKDIR/a.store.err" 2>/dev/null | sed 's/^/    /'
    STATUS=2
    STORE_NOTE="the store was NOT compared"
  elif [ "$A_STORE_STATUS" -ne 0 ] || [ "$B_STORE_STATUS" -ne 0 ]; then
    # EXACTLY ONE ARM ANSWERS, AND THAT IS THE PROPERTY FAILING RATHER THAN AN
    # INABILITY TO MEASURE IT. One run produced a usable project and the other
    # did not, which is precisely what "the re-run did not reach the same end
    # state" means -- so it is a FINDING at exit 1, not a refusal at exit 2.
    #
    # Found by reading this arm while a run was inside the file, and fixed
    # AFTER it finished: bash reads a script incrementally, and this rig has
    # already had one result discarded for an edit landing mid-run.
    #
    # The realistic shape is A ok / B dead, because liveness has already
    # established A. That is the gate's core failure mode -- an interrupted
    # estate whose re-run completes and leaves something nobody can open -- and
    # it was being reported as "could not measure".
    echo
    echo "  STORE: DIFFERENT -- exactly one arm answers \`$STORE_CMD\`: clean exited $A_STORE_STATUS, re-run exited $B_STORE_STATUS."
    echo "    One run produced a usable project and the other did not. That is the"
    echo "    end states DISAGREEING, not an inability to compare them."
    head -2 "$WORKDIR/b.store.err" 2>/dev/null | sed 's/^/    /'
    STATUS=1
    STORE_NOTE="store DIFFERENT -- only one arm answers"
  elif cmp -s "$WORKDIR/a.store.json" "$WORKDIR/b.store.json"; then
    echo
    echo "  STORE: IDENTICAL (as \`export\` sees it; file_index/doc_sections/snapshots not compared) -- \`$STORE_CMD\` byte-equal across both arms ($(wc -c <"$WORKDIR/a.store.json" | tr -d ' ') bytes)."
    STORE_NOTE="store identical as export sees it"
  else
    echo
    echo "  STORE: DIFFERENT -- \`$STORE_CMD\` disagrees between the clean run and the re-run."
    echo "    $(cmp "$WORKDIR/a.store.json" "$WORKDIR/b.store.json" 2>&1 | head -1)"
    echo "    Under D01 as reversed the DB is the SSOT, so a store the re-run did"
    echo "    not reproduce is a failure of the property even with the files identical."
    STATUS=1
    STORE_NOTE="store DIFFERENT"
  fi
elif [ -z "$STORE_CMD" ]; then
  echo
  echo "  STORE ARM DID NOT RUN -- MIGRATE_CMD is overridden and no STORE_CMD was given."
  STORE_NOTE="store NOT compared"
fi

echo
case "$STATUS" in
  0) say "GATE ARM PASSED: interrupted at $B_AT_KILL_DELTA/$A_DELTA files, re-run reached the clean end state${STORE_NOTE:+ -- $STORE_NOTE}" ;;
  1) say "GATE ARM FAILED: interrupted at $B_AT_KILL_DELTA/$A_DELTA files, re-run did NOT reach the clean end state${STORE_NOTE:+ -- $STORE_NOTE}" ;;
  *) say "the verdict tool could not measure (exit $STATUS)" ;;
esac

if [ "$KEEP" -eq 1 ] || [ "$OWNED" -eq 0 ]; then
  say "trees kept: $A (clean) and $B (re-run); logs alongside them"
fi

exit "$STATUS"
