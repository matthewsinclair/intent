#!/bin/bash
# burn.sh -- classify the BATS estate by BURN RATIO.
#
# Method: run each file twice, once with the default INTENT_BIN and once with
# INTENT_BIN=/usr/bin/false, and count failures each time. The delta is the
# number of tests that actually reach the top-level CLI.
#
#   burn == total     the whole file exercises the CLI          -> keep candidate
#   burn == 0         the file never reaches the CLI at all     -> retire/out-of-scope
#   0 < burn < total  mixed; needs per-test adjudication        -> split candidate
#
# This is empirical where assertion-parsing is inferential. A file can call
# run_intent and still assert only on repo content; a file can look CLI-shaped
# and actually exec bin/intent_treeindex, bypassing the dispatcher entirely
# (53 of 53 tests in treeindex_commands.bats do exactly that). Reading the
# assertions cannot separate those; redirecting the binary can.
#
# Files with a non-zero DEFAULT failure count are reported as UNSTABLE and carry
# no classification: their baseline is not green, so the delta means nothing.

set -uo pipefail
WT="${WT:?set WT}"
cd "$WT"

printf 'FILE\tTESTS\tDEFAULT_FAIL\tBURN\tSTATUS\n'

for f in $(find tests -name '*.bats' | sort); do
  total=$(grep -c '^@test' "$f" 2>/dev/null || echo 0)
  [ "$total" -gt 0 ] || continue
  d=$(bats "$f" 2>&1 | grep -cE '^not ok' || true)
  m=$(INTENT_BIN=/usr/bin/false bats "$f" 2>&1 | grep -cE '^not ok' || true)
  burn=$(( m - d ))
  [ "$burn" -lt 0 ] && burn=0
  if [ "$d" -ne 0 ]; then
    status=UNSTABLE
  elif [ "$burn" -eq "$total" ]; then
    status=FULL
  elif [ "$burn" -eq 0 ]; then
    status=NONE
  else
    status=MIXED
  fi
  printf '%s\t%s\t%s\t%s\t%s\n' "$f" "$total" "$d" "$burn" "$status"
done
