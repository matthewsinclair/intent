#!/usr/bin/env bash
# partition_closes_check.sh -- does an instrument that prints a partition SAY that it closes?
#
# Witness for AT-00.13, covering AC-00.12. The register requires a cited file to
# name the row that cites it, so this id is structural rather than a pointer
# into a tracker.
#
# **DISTINCT FROM AC-00.11, AND THE DISTINCTION IS THE WHOLE ROW.** There, M was
# derived from the wrong SET. Here every member was examined and the PARTITION
# of M is wrong. Wrong M versus wrong partition of M.
#
# **THE LOUDNESS OF THE MOTIVATING CASE WAS LUCK.** `EXAMINED 2 of 1 ... the
# other -1` was caught because subtraction produced an IMPOSSIBLE value. cc`s
# `2 + 53 + 1 + 2 = 58` against 57 rows was the same defect one sign the other
# way and nobody saw it: the table HEADER carried the literal `Completed` and
# was counted as a member of the population it labels. **A +1 remainder is as
# silent as a -1 is loud**, so the requirement is not *be correct* -- it is
# STATE THE SUM AND ASSERT IT, rather than leaving a reader to add up.
#
# ==========================================================================
# WHY THE STATEMENT IS THE REQUIREMENT, AND NOT MERELY GOOD MANNERS
# ==========================================================================
#
# **A PARTITION THAT IS BOTH UNSTATED AND BROKEN IS INVISIBLE TO THIS TOOL AND
# TO EVERY OTHER, BY CONSTRUCTION.** Nothing can tell three unrelated integers
# from a partition that fails to close without knowing which of them were meant
# to be parts, and only the author knows that. **The stated sum is the single
# thing that makes a partition checkable by anyone but its author** -- which is
# why AC-00.12 demands the STATEMENT rather than demanding correctness. This
# tool can verify a stated closure and can spot an unstated partition that
# happens to close; the intersection it cannot reach is exactly the state the
# criterion exists to make unrepresentable.
#
# So a clean run here is NOT `the estate has no broken partitions`. It is `no
# STATED closure is false, and no partition I could recognise went unstated`.
#
# ==========================================================================
# REACH
# ==========================================================================
#
#   - **16 OF 64 PARITY INSTRUMENTS.** It examines the ones the roster marks
#     `gated`, because gated is this estate`s own definition of *has a safe bare
#     invocation*. The other 48 take tree arguments or mutate a tree. A
#     partition printed by an instrument nobody can invoke bare is outside
#     every checker`s reach, and that is a gap this tool does not close.
#   - **PER LINE.** A partition spread ACROSS lines is not detected -- which is
#     the *leaving a reader to add up* failure at its worst, and is not
#     mechanically separable from unrelated numbers on adjacent lines.
#   - **ARM C reaches the of-N grammar** (`N of M` with N > M, and a negative
#     stated remainder) because AT-00.13 names that case as a required positive
#     control and arms A and B both need a `+` to see anything at all.
#   - It reads OUTPUT, never source. An instrument whose partition is computed
#     correctly and printed under a confusing label passes.
#
# Exit 0 clean, 1 findings, 2 refusal.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
TOOLS="$ROOT/intent/st/ST0056/parity/tools"

FROM=""
while [ $# -gt 0 ]; do
  case "$1" in
    --from) FROM="${2:-}"; shift 2 ;;
    -h|--help) sed -n '2,50p' "${BASH_SOURCE[0]}"; exit 0 ;;
    *) echo "partition: unknown argument: $1" >&2; exit 2 ;;
  esac
done

# ---------------------------------------------------------------------------
# The population. `--from <dir>` reads pre-captured `<tool>.txt` output so the
# arms can be driven against fixtures, and SO A CALLER CAN AVOID PAYING 17s.
# The source is printed, because a drive that silently ran against the real
# tree would report a success that measured nothing.
# ---------------------------------------------------------------------------
WORK=""
if [ -n "$FROM" ]; then
  [ -d "$FROM" ] || { echo "partition: no such capture directory: $FROM" >&2; exit 2; }
  WORK="$FROM"; SOURCE_LABEL="pre-captured output: $FROM"
else
  ROSTER_SRC="$TOOLS/runner_roster_check.sh"
  [ -f "$ROSTER_SRC" ] || { echo "partition: cannot read the roster at $ROSTER_SRC" >&2; exit 2; }
  GATED="$(sed -n "/^ROSTER='/,/^'/p" "$ROSTER_SRC" | sed '1d;$d' \
            | awk 'NF && $1 ~ /\.(sh|bash)$/ && $2 == "gated" { print $1 }')"
  [ -n "$GATED" ] || { echo "partition: parsed 0 gated rows from the roster -- the parse is broken, not the estate" >&2; exit 2; }
  WORK="$(mktemp -d "${TMPDIR:-/tmp}/partition-check.XXXXXX")"
  trap 'rm -rf "$WORK"' EXIT
  while read -r t; do
    [ -f "$TOOLS/$t" ] || continue
    timeout 120 bash "$TOOLS/$t" > "$WORK/$t.txt" 2>&1
  done <<< "$GATED"
  SOURCE_LABEL="live run of the roster's gated set"
fi

CAPTURES="$(find "$WORK" -maxdepth 1 -name '*.txt' | sort)"
[ -n "$CAPTURES" ] || { echo "partition: no captured output to examine -- refusing rather than reporting a clean estate over an empty set" >&2; exit 2; }

# ---------------------------------------------------------------------------
# Arms A and B, per instrument.
#
# A  a STATED closure (`a + b [+ ...] = T`) whose arithmetic does not hold.
# B  a line whose integers admit `one == the sum of the rest` while the
#    instrument states no closure anywhere -- a partition left to the reader.
# ---------------------------------------------------------------------------
analyse() {
  awk '
    function nums(line, arr,   n, i, tmp) {
      n = 0; tmp = line
      while (match(tmp, /-?[0-9]+/)) {
        arr[++n] = substr(tmp, RSTART, RLENGTH) + 0
        tmp = substr(tmp, RSTART + RLENGTH)
      }
      return n
    }
    {
      line = $0
      # ARM A: a stated closure. LHS carries a `+`, RHS is a single total.
      if (line ~ /[0-9][^=]*\+[^=]*=[^0-9]*[0-9]/) {
        split(line, halves, "=")
        # **EACH `+` SEGMENT CONTRIBUTES THE INTEGER NEAREST THE OPERATOR.**
        # Summing every integer left of the `=` swept in quantities that share
        # the sentence but not the partition: `510 recorded ... across 60 + 9
        # = 69` came out 579. The first drive predicted 0 for this arm, got 1,
        # and the 1 was this parser rather than the estate.
        ns = split(halves[1], SEG, /\+/)
        rn = nums(halves[2], R)
        if (ns >= 2 && rn >= 1) {
          s = 0; ok = 1
          for (i = 1; i <= ns; i++) {
            k = nums(SEG[i], T)
            if (k < 1) { ok = 0; break }
            s += T[k]
          }
          if (ok) {
            stated = 1
            if (s != R[1]) printf "A\t%s\t%d != %d\n", line, s, R[1]
          }
        }
      }
      # ARM B candidate: one integer equals the sum of the rest.
      # ARM C: an IMPOSSIBLE partition stated in of-N form. AT-00.13 names two
      # positive controls and this is the first of them -- `EXAMINED 2 of 1 ...
      # the other -1`, loud only because subtraction produced a value that
      # cannot exist. Arms A and B both need a `+`, so neither reaches it: an
      # of-N remainder is a partition stated in a different grammar.
      if (match(line, /[0-9]+ of [0-9]+/)) {
        seg = substr(line, RSTART, RLENGTH)
        split(seg, OF, / of /)
        if (OF[1] + 0 > OF[2] + 0) printf "C\t%s\t%d of %d\n", line, OF[1], OF[2]
      }
      if (line ~ /(the other|remainder|unaccounted)[^0-9-]*-[0-9]/) printf "C\t%s\tnegative remainder\n", line

      # **>= 3 PARTS, AND NEVER A LINE CARRYING A PATH.** Two parts is
      # indistinguishable from coincidence -- the first drive matched
      # `Machine 4 -> ... 2 rows, 2 edges` because 4 == 2 + 2 -- and a
      # filesystem path`s digits are not quantities at all.
      n = nums(line, N)
      if (n >= 4 && line !~ /\//) {
        total = 0; for (i = 1; i <= n; i++) total += N[i]
        for (i = 1; i <= n; i++) {
          rest = total - N[i]
          if (N[i] > 0 && rest == N[i]) { printf "B\t%s\t%d\n", line, N[i]; break }
        }
      }
    }
    END { if (stated) printf "S\t-\t-\n" }
  ' "$1"
}

EXAMINED=0; STATES=0; UNSTATED=0; NOPART=0
BROKEN=""; UNSTATED_ROWS=""

for cap in $CAPTURES; do
  tool="$(basename "$cap" .txt)"
  EXAMINED=$((EXAMINED + 1))
  res="$(analyse "$cap")"
  a="$(printf '%s\n' "$res" | awk -F'\t' '$1=="A" || $1=="C"')"
  b="$(printf '%s\n' "$res" | awk -F'\t' '$1=="B"')"
  s="$(printf '%s\n' "$res" | awk -F'\t' '$1=="S"' | head -1)"
  if [ -n "$a" ]; then
    BROKEN="$BROKEN
    $tool -- $(printf '%s' "$a" | cut -f2- | sed 's/\t/  ARITHMETIC: /')"
  fi
  if [ -n "$s" ]; then
    STATES=$((STATES + 1))
  elif [ -n "$b" ]; then
    UNSTATED=$((UNSTATED + 1))
    UNSTATED_ROWS="$UNSTATED_ROWS
    $tool -- $(printf '%s' "$b" | cut -f2 | sed 's/^ *//' | cut -c1-120)"
  else
    NOPART=$((NOPART + 1))
  fi
done

echo "partition: source -- $SOURCE_LABEL"
echo

if [ -n "$BROKEN" ]; then
  echo "partition: ARM A/C -- a stated closure whose arithmetic does not hold, or an IMPOSSIBLE partition:$BROKEN"
  echo
fi

if [ -n "$UNSTATED_ROWS" ]; then
  echo "partition: ARM B -- a partition the reader is left to add up:$UNSTATED_ROWS"
  echo
fi

# **THIS TOOL IS SUBJECT TO ITS OWN CRITERION**, so it states its sum and
# refuses if it does not close.
echo "partition: PARTITION of the $EXAMINED instrument(s) examined -- $STATES state a closure,"
echo "  $UNSTATED leave one unstated, $NOPART print no partition this tool can recognise."
SUM=$((STATES + UNSTATED + NOPART))
if [ "$SUM" -ne "$EXAMINED" ]; then
  echo "partition: THE PARTITION DOES NOT CLOSE: $STATES + $UNSTATED + $NOPART = $SUM against $EXAMINED." >&2
  exit 2
fi
echo "  THE PARTITION CLOSES: $STATES + $UNSTATED + $NOPART = $EXAMINED."
echo

echo "partition: REACH -- 16 of 64 parity instruments (the roster's gated set, the"
echo "  only ones with a safe bare invocation), PER LINE, and over OUTPUT rather than"
echo "  source. A partition that is BOTH unstated AND broken is invisible here by"
echo "  construction: nothing separates three unrelated integers from a failed"
echo "  partition without knowing which were meant to be parts. THAT IS WHY THE"
echo "  CRITERION DEMANDS THE STATEMENT rather than demanding correctness."
echo

FOUND=$(( ${UNSTATED:-0} ))
[ -n "$BROKEN" ] && FOUND=$((FOUND + 1))
if [ "$FOUND" -gt 0 ]; then
  echo "partition: FINDING -- $UNSTATED instrument(s) print a partition without stating that it closes."
  echo "  State the sum at the numbers and assert it, so a +1 remainder is as loud as a -1."
  exit 1
fi

echo "partition: every partition this tool can recognise is stated and closes."
exit 0
