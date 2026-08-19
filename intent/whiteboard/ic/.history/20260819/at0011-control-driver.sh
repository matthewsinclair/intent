#!/usr/bin/env bash
# Drive the DEFECTIVE (f2a2675f) and REPAIRED (8bb47e49) canon_commit_check.sh over
# the same revisions. The differential in the EXAMINED verdict is the finding.
#
# THREE DEFECTS THIS DRIVER HAS ALREADY COMMITTED, FIXED HERE, REASONS IN THE FILE
# RATHER THAN IN THE AUTHOR'S HEAD:
#
# 1. UNANCHORED PATTERN. `grep EXAMINED` also matches the "NOT EXAMINED -- N thread(s)
#    record zero attachments" ADVISORY, which is a DIFFERENT POPULATION. Ten revisions
#    came back byte-identical across a defective and a repaired tool: a perfectly clean,
#    perfectly wrong differential. Anchored on "canon-commit: EXAMINED" now.
#
# 2. SHARED MUTABLE TEMP FILES. Both sides wrote fixed paths $SP/o.out and $SP/n.out,
#    reused across every revision AND across every invocation of this script. Two runs
#    with identical arguments over identical worktrees disagreed on their first three
#    rows, and the wrong values matched the LAST revision of the PREVIOUS run. A
#    per-invocation mktemp -d now isolates them. The mechanism was never proven; the
#    fix does not depend on proving it, and a non-deterministic instrument is
#    unusable whatever the cause.
#
# 3. CANNOT-READ CONFLATED WITH A FINDING. An empty capture printed "<none>", which
#    reads as "the tool printed no verdict" when it may equally mean "this driver
#    failed to read one". They are now DISTINCT: NO-VERDICT (tool ran, printed none)
#    against UNREADABLE (capture empty or binary). grep -a, because the tool can emit
#    blob bytes and grep then refuses to print matches at all.
set -u
SP="$(cd "$(dirname "$0")" && pwd)"
OLD="$SP/wt-old"; NEW="$SP/wt-new"
T=intent/st/ST0056/parity/tools/canon_commit_check.sh
WORK="$(mktemp -d)"; trap 'rm -rf "$WORK"' EXIT

read_verdict() {  # $1=stdout $2=stderr -> prints "N of M" | NO-VERDICT | UNREADABLE
  if [ ! -s "$1" ] && [ ! -s "$2" ]; then echo UNREADABLE; return; fi
  local v
  v="$(cat "$1" "$2" | grep -a -oE 'canon-commit: EXAMINED [0-9]+ of [0-9]+' | head -1)"
  if [ -z "$v" ]; then echo NO-VERDICT; else echo "${v#canon-commit: EXAMINED }"; fi
}

printf '%-10s | %-3s %-14s | %-3s %-14s | %s\n' REV rc OLD_examined rc NEW_examined DELTA
n_over=0 n_same=0 n_unread=0 n_noverdict=0 n_total=0
for REV in "$@"; do
  n_total=$((n_total + 1))
  o="$WORK/$REV.o"; n="$WORK/$REV.n"
  ( cd "$OLD" && bash "$T" "$REV" ) > "$o.out" 2> "$o.err"; rco=$?
  ( cd "$NEW" && bash "$T" "$REV" ) > "$n.out" 2> "$n.err"; rcn=$?
  eo="$(read_verdict "$o.out" "$o.err")"; en="$(read_verdict "$n.out" "$n.err")"
  case "$eo$en" in
    *UNREADABLE*)  d=UNREADABLE;  n_unread=$((n_unread + 1)) ;;
    *NO-VERDICT*)  d=NO-VERDICT;  n_noverdict=$((n_noverdict + 1)) ;;
    *) no="${eo%% of *}"; nn="${en%% of *}"
       if [ "$no" -gt "$nn" ]; then d="+$((no - nn)) OVERSTATED"; n_over=$((n_over + 1))
       elif [ "$no" -lt "$nn" ]; then d="!! UNDERSTATED -- FALSIFIES P1"
       else d=0; n_same=$((n_same + 1)); fi ;;
  esac
  printf '%-10s | %-3s %-14s | %-3s %-14s | %s\n' "$REV" "$rco" "$eo" "$rcn" "$en" "$d"
done
# THE COUNT CLOSES OVER WHAT WAS EXAMINED, AND THE PARTS ARE PRINTED WITH THE TOTAL
# AND THE ASSERTION -- not left available to a reader who adds up. Closure asserted,
# because a remainder that happens to be positive is as silent as one that is zero.
scored=$((n_over + n_same))
printf '\nDRIVEN %d revision(s): %d overstated + %d agreed + %d no-verdict + %d unreadable = %d' \
  "$n_total" "$n_over" "$n_same" "$n_noverdict" "$n_unread" \
  "$((n_over + n_same + n_noverdict + n_unread))"
[ "$((n_over + n_same + n_noverdict + n_unread))" -eq "$n_total" ] && echo "  CLOSES." || echo "  ** DOES NOT CLOSE **"
echo "SCORED $scored of $n_total -- the denominator is revisions DRIVEN, not revisions requested."
echo "REACH -- nested layout only. The negative remainder on record (EXAMINED 2 of 1 ... the other -1) needs the FLAT layout and is OUT OF REACH here."
