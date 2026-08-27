#!/usr/bin/env bash
# fleet-hook-census.sh -- what pre-commit gate each estate ACTUALLY RUNS, by
# generation, plus whether a whiteboard estate's stamps are guarded.
#
# WHY THIS EXISTS RATHER THAN A GREP. Every claim made about "the hook" on
# 2026-08-27 -- by three nodes, in both directions -- was a claim about the
# TEMPLATE in the Intent tree. The estates run the copy installed at their last
# `intent upgrade`, and there are THREE generations of it in the field. A
# condition verified against `lib/templates/hooks/pre-commit.sh` says nothing
# about what fires in Lamplight.
#
# THREE THINGS THAT DEFEAT THE OBVIOUS INSTRUMENT, ALL THREE MET FOR REAL:
#   (a) `core.hooksPath` moves the hook dir (Intent/Intentv2 `.githooks`,
#       Laksa `bin/hooks`). Keying on `.git/hooks` silently drops them -- dc's
#       v1 sweep lost exactly those three and returned a plausible number.
#   (b) THE GATE IS USUALLY NOT THE FILE NAMED `pre-commit`. A chained install
#       puts a 114-line wrapper there and the gate in `pre-commit.intent`.
#       vc's v1 sweep keyed on `pre-commit` and returned ZERO of seventeen.
#   (c) `git rev-parse --git-path hooks` DOES honour `core.hooksPath`, which is
#       what makes (a) fixable in one line rather than by special-casing.
#
# THE POSITIVE CONTROL IS NOT DECORATION AND IT IS WHAT CAUGHT (b): the tree
# this runs from MUST appear in its own carrying set. It cost nothing and it is
# the only reason a zero was not reported as a finding.
set -u
PRJ=${1:-~/Devel/prj}
SELF=$(git rev-parse --show-toplevel 2>/dev/null); SELF=$(basename "${SELF:-Intent}")
printf '%-12s %-11s %7s %-4s %s\n' ESTATE HOOKDIR BYTES WB GENERATION
printf '%s\n' "---------------------------------------------------------------------------------------"
carry=0; g1=0; g2=0; g3=0; unguarded=""
control=""
for d in "$PRJ"/*; do
  { [ -d "$d/.git" ] || [ -f "$d/.git" ]; } || continue
  name=$(basename "$d")
  hp=$(git -C "$d" rev-parse --git-path hooks 2>/dev/null) || continue
  case "$hp" in /*) hd="$hp" ;; *) hd="$d/$hp" ;; esac
  gate=""
  for f in "$hd"/pre-commit "$hd"/pre-commit.*; do
    [ -f "$f" ] || continue
    grep -q 'intent critic gate' "$f" 2>/dev/null && { gate="$f"; break; }
  done
  [ -n "$gate" ] || continue
  carry=$((carry+1)); [ "$name" = "$SELF" ] && control=yes
  gh=$(grep -c 'GUARD_HOME' "$gate"); a1=$(grep -c 'NO guard ran for this commit' "$gate")
  runner=$(grep -c 'pre-commit-guards' "$gate"); inline=$(grep -c 'clock-guard\|clock_guard' "$gate")
  # the self-host fallback CONDITION is in the template; only a G3 hook CONTAINS it
  if   [ "$a1" -eq 0 ] && [ "$gh" -eq 0 ]; then g="G1  no guard block at all -- runs no guards, says nothing"; g1=$((g1+1))
  elif [ "$gh" -eq 0 ]; then g="G2  ABSENCE 1 keys on INTENT_HOME_RESOLVED -- no self-host fallback"; g2=$((g2+1))
  else g="G3  current -- keys on GUARD_HOME, self-host fallback present"; g3=$((g3+1)); fi
  wb=-
  if [ -d "$d/intent/whiteboard" ]; then
    n=$(ls "$d/intent/whiteboard" 2>/dev/null | grep -vc '\.md$')
    if [ "$runner" -gt 0 ] || [ "$inline" -gt 0 ]; then wb="${n}ok"
    else wb="${n}!!"; unguarded="${unguarded}${name}(${n} nodes) "; fi
  fi
  printf '%-12s %-11s %7s %-4s %s\n' "$name" "$(dirname "$hp")/$(basename "$hp")" "$(wc -c < "$gate" | tr -d ' ')" "$wb" "$g"
done
printf '%s\n' "---------------------------------------------------------------------------------------"
echo "carry the intent gate: $carry   (G1 $g1 / G2 $g2 / G3 $g3)"
echo "WB column: <nodes>ok = clock guard reached; <nodes>!! = NOT reached; - = no whiteboard"
[ -n "$unguarded" ] && echo "UNGUARDED WHITEBOARDS -- stamps land unchecked: $unguarded"
if [ "$control" = yes ]; then echo "positive control: PASS -- $SELF is in its own carrying set"
else echo "POSITIVE CONTROL FAILED -- $SELF absent from its own carrying set; DISCARD THESE NUMBERS"; fi
