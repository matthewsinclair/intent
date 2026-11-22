#!/usr/bin/env bash
#
# canon_race_check.sh -- issue 0206. Does a canon verb LOSE a concurrent write?
#
# NOT an AT instrument: 0206 is an issue, not a criterion, so this cites no AC
# and covers no AT row. It exists because a defect demonstrated once and not
# re-drivable is a CLAIM rather than evidence -- 0206's body carries the RESULT,
# this carries the METHOD, and when someone attempts the fix this is the thing
# that says whether it worked.
#
# ==========================================================================
# THE MECHANISM UNDER TEST (vc, read in source before this was written)
# ==========================================================================
#
#   facade.rs:5215   let mut next = self.canon.clone();   // snapshot at construction
#                    find_criterion_mut(&mut next, ...)   // mutate ONE field
#                    self.apply(..., next)                // write the WHOLE record back
#
# Two processes load, each mutates a DIFFERENT field, each applies. The second's
# `next` derives from its stale snapshot, so it carries the first's field at its
# PRE-EDIT value and overwrites it. No error, no conflict, valid canon after.
#
# `ac satisfy` reaches that path by delegation -- `ac_satisfy` -> `set_ac_state`
# (facade.rs:5660), which is where the clone-mutate-apply actually lives.
# Recorded because a grep of `ac_satisfy`'s own body for `self.canon.clone()`
# returns ZERO and would say this harness points somewhere that cannot exhibit
# the defect (vc caught this on themselves before sending it).
#
# ==========================================================================
# WHY THERE ARE THREE ARMS AND NOT ONE
# ==========================================================================
#
# SAME     the hypothesis: two concurrent satisfies, different criteria, ONE thread.
# CROSS    the discriminating control: the same two processes, TWO threads.
#          `apply_envelopes` writes only threads that DIFFER, so cross-thread
#          work is expected safe. If this arm also lost writes, the finding
#          would be "concurrency breaks the verb" rather than "on one thread".
# (sequential is checked by hand: satisfy A then B lands both, every time. Without
#  it a loss could be the harness never applying the second edit at all.)
#
# ==========================================================================
# A CLEAN ITERATION IS NOT A TRIAL, AND THAT WAS THIS TOOL'S REAL DEFECT
# ==========================================================================
#
# The first version counted clean iterations as evidence of safety. It is not:
# if B constructs its Facade AFTER A's apply returned, there was no race to lose
# and the iteration is noise. A zero over a population that CANNOT exhibit the
# failure is the precision-is-a-claim-about-the-corpus class, and a low rate
# would be a statement about scheduling rather than about the code (vc).
#
# So: a START GATE synchronises launch, and each process records $EPOCHREALTIME
# either side of its own exec. An iteration counts as a TRIAL only when the two
# intervals intersect. Trials and non-trials are reported separately.
#
# THE BOUND'S DIRECTION IS STATED BECAUSE IT IS WHAT SEPARATES A CLAIM FROM A
# GUESS: the process wall interval is WIDER than the load-to-apply window, so a
# non-overlap is a DEFINITE non-trial while an overlap is only a POSSIBLE one.
# Trials are an UPPER bound and the per-trial loss rate a LOWER bound. It cannot
# flatter the finding; it can only understate it.
#
# ESTABLISHED 2026-09-01, replacing a note that said this was unmeasured. The
# question asked was whether SQLite busy-handling makes the loser RELOAD. The
# better question is whether the busy path is ENTERED AT ALL, and it is not:
# every losing iteration completes in ~0.01s against a `BUSY_TIMEOUT_MS` of
# 5000, where issue `0152` measured a genuinely contended writer waiting 5.22s
# and then being refused cleanly. **0.01s is not legible alone; it is legible
# against that 5.22s.**
#
# So the two writes never contend at the SQLite level -- each takes and releases
# the write lock in sequence, and the store sees two well-formed serialised
# transactions and is RIGHT to accept both. **SQLite protects the TRANSACTION;
# what is corrupted is the RECORD.** No store-layer tuning touches a path that
# is never taken, so this closes the door AGAINST a store-side fix rather than
# for one, and a record-layer compare-and-swap is weighed against nothing.
#
# AND THIS TOOL USED TO ASSERT THE SILENCE IT NOW MEASURES. It counted the
# OUTCOME and discarded both processes' rc and output, so `no error, no
# conflict` was inherited from the issue rather than driven. **A harness
# reporting a silence it did not measure is the defect it exists to find, one
# layer up.** Both are captured now and every loss is classified SILENT or
# REPORTED.
#
# SAFE BARE INVOCATION. Defaults to the tree binary and a short run. Every
# iteration builds a THROWAWAY project under mktemp -d and discards it; nothing
# touches the real estate and no --daemon is used. Concurrent edits are never
# planted on a live shared thread to prove a point about concurrent edits on a
# live shared thread.
set -uo pipefail

BIN="${1:-native/rust/target/release/intent}"
ITERS="${2:-10}"
[ -x "$BIN" ] || { echo "canon_race_check: no binary at $BIN" >&2; exit 2; }
BIN="$(cd "$(dirname "$BIN")" && pwd)/$(basename "$BIN")"

# ONE fixture builder, used by every arm. Duplicating it per arm would let the
# contended and uncontended arms drift apart, and the whole value of the
# sequential arm is that it differs from the racing arms in EXACTLY one respect
# -- whether the two writes overlap. A second copy makes that claim unprovable.
#
# Issue If83ae7c: every fixture command's status is checked, and a failure
# stops the run naming the command and its output. The fixture's two threads
# are the ids the mint printed, read by mint.lib, never spelled: after ST0095
# the first `st new` is not ST0001, and a fixture built on a spelled id was
# silently empty, so the race arms measured refusals, not a race. The ids are
# written to "$P/.ids" for the arms. The criteria are thread-level (group 00),
# the one group every thread has.
# shellcheck source=bin/.devbin/cmd/shared/mint.lib
. "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)/bin/.devbin/cmd/shared/mint.lib" \
  || { echo "canon_race_check: could not source mint.lib" >&2; exit 2; }
build_fixture() {
  local P out A B
  P=$(mktemp -d) || { echo "canon_race_check: mktemp failed" >&2; return 1; }
  # fx <verb...> -- one fixture step in $P, its output in `out`; a failure names
  # the step and its output and removes the fixture.
  fx() {
    out="$(cd "$P" && "$BIN" "$@" 2>&1)" && return 0
    printf 'canon_race_check: fixture step failed: intent %s\n%s\n' "$*" "$out" >&2
    rm -rf "$P"
    return 1
  }
  # mint <var> <verb...> -- fx, then the id the mint printed into <var>.
  mint() {
    local var="$1" id
    shift
    fx "$@" || return 1
    id="$(minted_id "$out")" || {
      printf 'canon_race_check: intent %s printed no id it minted\n%s\n' "$*" "$out" >&2
      rm -rf "$P"
      return 1
    }
    printf -v "$var" '%s' "$id"
  }
  fx init RaceProbe || return 1
  mint A st new Alpha || return 1
  fx st start "$A" || return 1
  mint B st new Beta || return 1
  fx st start "$B" || return 1
  fx ac new "$A" AC-00.1 --text t1 --kind non-test || return 1
  fx ac new "$A" AC-00.2 --text t2 --kind non-test || return 1
  fx ac new "$B" AC-00.1 --text t3 --kind non-test || return 1
  printf '%s %s\n' "$A" "$B" >"$P/.ids" || { echo "canon_race_check: could not write $P/.ids" >&2; rm -rf "$P"; return 1; }
  printf '%s' "$P"
}

# fixture_ids <P> -- the fixture's two thread ids into A and B, or stop.
fixture_ids() {
  read -r A B <"$1/.ids" && [ -n "$A" ] && [ -n "$B" ] \
    || { echo "canon_race_check: could not read the fixture's ids from $1/.ids" >&2; exit 2; }
}

# satisfied <P> <thread> -- how many of the thread's criteria are satisfied, or
# status 1 naming the read: a failed `ac list` must not count as a lost write,
# so each caller stops on it. awk counts, since grep -c exits 1 on zero.
satisfied() {
  local listed
  listed="$(cd "$1" && "$BIN" ac list "$2" 2>&1)" \
    || { printf 'canon_race_check: intent ac list %s failed\n%s\n' "$2" "$listed" >&2; return 1; }
  printf '%s\n' "$listed" | awk '/satisfied: yes/ { n++ } END { print n + 0 }'
}

arm() {
  local A B other mode="$1" trials=0 nontrials=0 lost=0 silent=0 reported=0 i P gate as ae bs be rca rcb tgt ac2 got
  for i in $(seq 1 "$ITERS"); do
    P=$(build_fixture) || exit 2
    fixture_ids "$P"
    if [ "$mode" = same ]; then tgt="$A"; ac2=AC-00.2; else tgt="$B"; ac2=AC-00.1; fi
    gate="$P/.gate"
    ( cd "$P" || exit; while [ ! -f "$gate" ]; do :; done
      as=$EPOCHREALTIME; "$BIN" ac satisfy "$A" AC-00.1 --evidence A > "$P/.oa" 2>&1; rca=$?; ae=$EPOCHREALTIME
      printf '%s %s %s\n' "$as" "$ae" "$rca" > "$P/.a" ) &
    ( cd "$P" || exit; while [ ! -f "$gate" ]; do :; done
      bs=$EPOCHREALTIME; "$BIN" ac satisfy "$tgt" "$ac2" --evidence B > "$P/.ob" 2>&1; rcb=$?; be=$EPOCHREALTIME
      printf '%s %s %s\n' "$bs" "$be" "$rcb" > "$P/.b" ) &
    : > "$gate"; wait
    read -r as ae rca < "$P/.a"; read -r bs be rcb < "$P/.b"
    got=$(satisfied "$P" "$A") || exit 2
    if [ "$mode" = cross ]; then
      other=$(satisfied "$P" "$B") || exit 2
      got=$((got + other))
    fi
    if [ "$(awk -v a="$ae" -v b="$bs" -v c="$as" -v d="$be" 'BEGIN{print (b<a && c<d)?1:0}')" -eq 1 ]; then
      trials=$((trials+1))
      if [ "$got" -lt 2 ]; then
        lost=$((lost+1))
        if [ "$rca" -eq 0 ] && [ "$rcb" -eq 0 ] && ! grep -qiE 'error|locked|busy' "$P/.oa" "$P/.ob" 2>/dev/null; then
          silent=$((silent+1))
        else
          reported=$((reported+1))
        fi
      fi
    else
      nontrials=$((nontrials+1))
    fi
    rm -rf "$P"
  done
  printf '    %-6s trials=%-4s non-trials=%-4s LOST=%-4s SILENT=%-4s REPORTED=%s\n' "$mode" "$trials" "$nontrials" "$lost" "$silent" "$reported"
}

# THE UNCONTENDED CONTROL, AND IT IS THE ARM THAT MAKES A FIX BELIEVABLE.
#
# The header used to say sequential was "checked by hand". Hand-checking is not
# a control: it is not in the output, so no reader can see whether it was done
# for THIS run, and it cannot go red.
#
# TODAY it proves the harness actually applies both edits, so a loss in `same`
# is the code losing a write rather than the rig never making one.
#
# AFTER A COMPARE-AND-SWAP LANDS it becomes the arm that discriminates: a CAS
# that refuses on contention and a CAS that refuses EVERYTHING are identical on
# the `same` arm, and differ only here. A fix must turn `same` from LOST into
# REFUSED while leaving this GREEN; either half alone is not evidence.
arm_sequential() {
  local A B i P rca rcb got green=0 red=0
  for i in $(seq 1 "$ITERS"); do
    P=$(build_fixture) || exit 2
    fixture_ids "$P"
    # No gate and no `&`: B starts only after A has exited, so the two writes
    # cannot overlap by construction rather than by scheduling luck.
    ( cd "$P" && "$BIN" ac satisfy "$A" AC-00.1 --evidence A ) >"$P/.oa" 2>&1; rca=$?
    ( cd "$P" && "$BIN" ac satisfy "$A" AC-00.2 --evidence B ) >"$P/.ob" 2>&1; rcb=$?
    got=$(satisfied "$P" "$A") || exit 2
    if [ "$rca" -eq 0 ] && [ "$rcb" -eq 0 ] && [ "$got" -eq 2 ]; then
      green=$((green+1))
    else
      red=$((red+1))
    fi
    rm -rf "$P"
  done
  printf '    %-6s GREEN=%-4s RED=%-4s   (an uncontended pair must land BOTH)\n' "seq" "$green" "$red"
}

echo "canon_race_check -- issue 0206, $("$BIN" --version 2>&1 | head -1)"
echo ""
arm same
arm cross
arm_sequential
echo ""
echo "REACH, in the output because a limit not in the output is not a limit the reader has:"
echo "  COVERS      whether two concurrent canon verbs on ONE thread lose a write,"
echo "              with a cross-thread arm to show the loss is thread-scoped."
echo "  DOES NOT    say WHICH field was lost or in what order: it counts satisfied"
echo "              criteria, so it detects loss without attributing it."
echo "  DOES NOT    generalise past \`ac satisfy\`. The facade mechanism predicts every"
echo "              canon verb; predicts is not shows, and only this one is driven."
echo "  ESTABLISHES whether a loss is SILENT: both processes rc and output are"
echo "              CAPTURED, not discarded. SQLite busy path is never ENTERED here"
echo "              (~0.01s against a 5000ms timeout; 0152 measured a genuinely"
echo "              contended writer at 5.22s), so no store-layer tuning narrows it."
echo "  CONTROLS    the uncontended arm is IN THE OUTPUT, not hand-checked. It shows"
echo "              the rig lands both edits when they do not overlap, so a loss on"
echo "              the same arm is the code and not the rig -- and after a fix it is"
echo "              the only arm separating refuses-on-contention from refuses-always."
echo "  BOUND       trials are an UPPER bound (wall interval is wider than the"
echo "              load-to-apply window), so the loss rate is a LOWER bound."
