#!/usr/bin/env bash
# mutate-at.sh <sha> "<cargo test args>" <spec>... -- test a commit BY ITS DEFECT, second operator.
# Isolated worktree at <sha>, own target dir (the shared tree and dc's target are never touched).
# Control run first (must be green), then each spec applied ALONE: a spec file has four lines --
#   1 file (relative to native/rust)   2 perl regex matching exactly ONE line to mutate
#   3 replacement line (verbatim)      4 grep -E pattern the mutated run's output must contain
#   5 (optional) which match to mutate when the regex hits several lines: N or `last` (default 1)
# The matched line numbers are printed so a mutation on the wrong arm is visible, not a silent green.
set -uo pipefail
SHA=$1; TESTARGS=$2; shift 2
I=/Users/matts/Devel/prj/Intent; S=/private/tmp/claude-501/-Users-matts-Devel-prj-Intent/699601ed-7e13-4808-bb6c-e6a79d27c56e/scratchpad
WT=$S/wt-$SHA; export CARGO_TARGET_DIR=$S/target-wt
git -C "$I" worktree add --detach "$WT" "$SHA" > "$S/wt-$SHA.log" 2>&1 || { echo "worktree add failed"; cat "$S/wt-$SHA.log"; exit 2; }
cd "$WT/native/rust" || exit 2
echo "== control at $SHA: cargo test $TESTARGS"; t0=$(date -u +%H:%M:%SZ)
cargo test $TESTARGS > "$S/mut-$SHA-control.out" 2>&1; rc0=$?
echo "control rc=$rc0 :: $(grep -E '^test result' "$S/mut-$SHA-control.out" | tail -1) [$t0 -> $(date -u +%H:%M:%SZ)]"
[ $rc0 -eq 0 ] || { tail -15 "$S/mut-$SHA-control.out"; cd "$I"; git worktree remove --force "$WT"; exit 3; }
fail=0; n=0
for spec in "$@"; do n=$((n+1)); F=$(sed -n '1p' "$spec"); RX=$(sed -n '2p' "$spec"); REPL=$(sed -n '3p' "$spec"); EXP=$(sed -n '4p' "$spec")
  OCC=$(sed -n '5p' "$spec"); OCC=${OCC:-1}
  lines=$(RX="$RX" perl -ne 'print "$.\n" if /$ENV{RX}/' "$F"); m=$(printf '%s\n' "$lines" | grep -c .)
  if [ "$m" -eq 0 ]; then echo "spec $n ($spec): REFUSED -- regex matches no line in $F"; fail=1; continue; fi
  if [ "$OCC" = last ]; then LN=$(printf '%s\n' "$lines" | tail -1); else LN=$(printf '%s\n' "$lines" | sed -n "${OCC}p"); fi
  if [ -z "$LN" ]; then echo "spec $n ($spec): REFUSED -- occurrence '$OCC' of $m match(es) does not exist in $F"; fail=1; continue; fi
  echo "spec $n: regex matches $m line(s) in $F ($(printf '%s' "$lines" | tr '\n' ' ')); mutating occurrence $OCC = line $LN"
  cp "$F" "$S/mut-$SHA-$n.orig"
  LN="$LN" REPL="$REPL" perl -i -pe 'if ($. == $ENV{LN}) { $_ = $ENV{REPL} . "\n" }' "$F"
  echo "spec $n: $F :: $(diff "$S/mut-$SHA-$n.orig" "$F" | grep -E '^[<>]' | tr -s ' ' | cut -c1-140 | tr '\n' '|')"
  cargo test $TESTARGS > "$S/mut-$SHA-$n.out" 2>&1; rc=$?
  hit=$(grep -c -E "$EXP" "$S/mut-$SHA-$n.out")
  if [ $rc -ne 0 ] && [ "$hit" -ge 1 ]; then echo "spec $n: ARMED -- rc=$rc, expected message present ($hit): $(grep -E "$EXP" "$S/mut-$SHA-$n.out" | head -1 | cut -c1-160)"; else echo "spec $n: NOT ARMED -- rc=$rc, expected pattern hits=$hit"; grep -E '^test .* FAILED|^test result' "$S/mut-$SHA-$n.out" | head -3; fail=1; fi
  cp "$S/mut-$SHA-$n.orig" "$F"; cmp -s "$S/mut-$SHA-$n.orig" "$F" || { echo "RESTORE FAILED for $F"; fail=1; }
done
cd "$I" && git worktree remove --force "$WT" && echo "worktree removed"
[ $fail -eq 0 ] && echo "VERDICT: $n mutation(s) ARMED at $SHA" || { echo "VERDICT: NOT ARMED at $SHA (see above)"; exit 1; }
