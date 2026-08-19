#!/bin/bash
# AT-00.12 partition driver -- ic 2026-08-19.
# Population: of_n_population.sh's WORK-LIST, READ FROM ITS OUTPUT, never re-derived.
# Verdict per file: of_n_labels_its_derivation.sh run on that file ALONE. One home for
# the ratio predicate; this driver owns no pattern of its own.
set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
T="$ROOT/intent/st/ST0056/parity/tools"
LIST="${1:?work-list file}"
# CANNOT-MEASURE IS NOT A FINDING AND NOT A PASS. A missing instrument returned rc=127
# on all 15 rows and printed a tidy table of zeros -- which reads as "the parser sees
# nothing in any of them" and would have made every file a false gap. Refuse instead.
INSTR="$T/of_n_labels_its_derivation.sh"
[ -r "$INSTR" ] || { echo "partition: CANNOT MEASURE -- no instrument at $INSTR. A zero here is a" >&2
                     echo "  driver failure, never an estate result." >&2; exit 2; }
TOTAL=0
printf '%-34s %-8s %s\n' FILE RATIOS "PARSER VERDICT"
while IFS= read -r b; do
  [ -n "$b" ] || continue
  out="$(mktemp)"; bash "$INSTR" "$T/$b" >"$out" 2>&1; rc=$?
  n="$(sed -n 's/.*file(s); \([0-9]*\) ratio(s) matched.*/\1/p' "$out" | head -1)"
  [ -n "$n" ] || n=0
  # THREE OUTCOMES, NOT TWO. rc=2 is ambiguous between "no ratio in reach" and "ratio
  # seen but NONE classifiable" -- the vacuous-pass guard. Collapsing them would repeat
  # the <none> defect: my driver failing and the tool refusing, recorded as one thing.
  if [ "$rc" -eq 0 ]; then v="ratio SEEN and classified"
  elif grep -q 'NOT ONE emitted a ratio' "$out"; then v="NO ratio in reach"
  elif grep -q 'NOT ONE was classifiable' "$out"; then v="ratio SEEN, none classifiable"
  else v="rc=$rc UNREADABLE -- driver fault, NOT a result"; fi
  TOTAL=$((TOTAL + n))
  printf '%-34s %-8s %s\n' "$b" "$n" "$v"
  rm -f "$out"
done < "$LIST"
# CLOSURE: the per-file ratios must sum to the whole-estate run, or the population and
# the parser disagree about what was examined -- which is the defect this row is about.
WHOLE="$(bash "$INSTR" 2>/dev/null | sed -n 's/.*file(s); \([0-9]*\) ratio(s) matched.*/\1/p' | head -1)"
echo
echo "CLOSURE: $TOTAL ratio instance(s) across the work-list; $WHOLE across the whole estate."
if [ "$TOTAL" = "$WHOLE" ]; then
  echo "  CLOSES -- every ratio the parser can see lives inside the nominated work-list,"
  echo "  so the other 28 files carry none and the nomination missed nothing the parser sees."
else
  echo "  DOES NOT CLOSE -- $((WHOLE - TOTAL)) ratio(s) sit OUTSIDE the work-list. That is a"
  echo "  nomination gap and it is dc's arm, not this parser's."
fi
