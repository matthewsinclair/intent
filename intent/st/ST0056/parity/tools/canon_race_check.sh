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

arm() {
  local mode="$1" trials=0 nontrials=0 lost=0 silent=0 reported=0 i P gate as ae bs be rca rcb tgt ac2 got
  for i in $(seq 1 "$ITERS"); do
    P=$(mktemp -d) || continue
    ( cd "$P" || exit
      "$BIN" init RaceProbe >/dev/null 2>&1
      "$BIN" st new Alpha >/dev/null 2>&1; "$BIN" st start ST0001 >/dev/null 2>&1
      "$BIN" st new Beta  >/dev/null 2>&1; "$BIN" st start ST0002 >/dev/null 2>&1
      "$BIN" ac new ST0001 AC-01.1 --text t1 --kind non-test >/dev/null 2>&1
      "$BIN" ac new ST0001 AC-01.2 --text t2 --kind non-test >/dev/null 2>&1
      "$BIN" ac new ST0002 AC-01.1 --text t3 --kind non-test >/dev/null 2>&1 )
    if [ "$mode" = same ]; then tgt=ST0001; ac2=AC-01.2; else tgt=ST0002; ac2=AC-01.1; fi
    gate="$P/.gate"
    ( cd "$P" || exit; while [ ! -f "$gate" ]; do :; done
      as=$EPOCHREALTIME; "$BIN" ac satisfy ST0001 AC-01.1 --evidence A > "$P/.oa" 2>&1; rca=$?; ae=$EPOCHREALTIME
      printf '%s %s %s\n' "$as" "$ae" "$rca" > "$P/.a" ) &
    ( cd "$P" || exit; while [ ! -f "$gate" ]; do :; done
      bs=$EPOCHREALTIME; "$BIN" ac satisfy "$tgt" "$ac2" --evidence B > "$P/.ob" 2>&1; rcb=$?; be=$EPOCHREALTIME
      printf '%s %s %s\n' "$bs" "$be" "$rcb" > "$P/.b" ) &
    : > "$gate"; wait
    read -r as ae rca < "$P/.a"; read -r bs be rcb < "$P/.b"
    got=$(cd "$P" && "$BIN" ac list ST0001 2>/dev/null | grep -c 'satisfied: yes')
    [ "$mode" = cross ] && got=$(( got + $(cd "$P" && "$BIN" ac list ST0002 2>/dev/null | grep -c 'satisfied: yes') ))
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

echo "canon_race_check -- issue 0206, $("$BIN" --version 2>&1 | head -1)"
echo ""
arm same
arm cross
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
echo "  BOUND       trials are an UPPER bound (wall interval is wider than the"
echo "              load-to-apply window), so the loss rate is a LOWER bound."
