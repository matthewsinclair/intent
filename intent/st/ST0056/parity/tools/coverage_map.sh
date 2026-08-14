#!/bin/bash
# coverage_map.sh -- which command families does the BATS estate actually cover?
#
# THE QUESTION. The register says which test FILES reach the CLI (burn ratio).
# The dispatch table says which command ENTRIES exist. Neither answers the one
# that decides whether the conformance suite can certify v3: **is there a
# command family nothing exercises?** A family with no covering test is a
# parity hole -- v3 can break it and the suite stays green, which is the
# vacuous-green class at the scale of the whole surface rather than one test.
#
# METHOD. Static: grep the estate for invocations per family, then JOIN against
# burn-baseline.tsv so a "covering" file that never reaches the CLI is counted
# as VACUOUS rather than as coverage. That join is the whole point -- 53 of 53
# tests in treeindex_commands.bats invoke bin/intent_treeindex directly and
# burn zero, so a naive grep would report treeindex as the best-covered command
# in the estate when the dispatcher never sees it.
#
# Runs no tests. Reads tests/ and the committed baseline only, so it is safe to
# run while another node holds the estate.

set -euo pipefail

BASE="intent/st/ST0056/parity/tools/burn-baseline.tsv"
CANON="intent/st/ST0056/dispatch-table.json"
[ -f "$BASE" ] || { echo "error: no burn baseline at $BASE" >&2; exit 1; }
[ -f "$CANON" ] || { echo "error: no dispatch-table canon at $CANON" >&2; exit 1; }

printf '%-12s %8s %8s %8s   %s\n' FAMILY REAL VACUOUS TESTS VERDICT
printf '%-12s %8s %8s %8s   %s\n' ------ ---- ------- ----- -------

for famname in $(jq -r '.families[].name' "$CANON"); do
  # `ac` and `at` are nouns of one binary; `st_zero` is also spelled `st zero`.
  case "$famname" in
    st_zero) needle='(run_intent|intent)[[:space:]]+(st_zero|st[[:space:]]+zero)' ;;
    *)       needle="(run_intent|intent)[[:space:]]+${famname}([[:space:]]|\")" ;;
  esac

  real=0; vac=0; realtests=0; hits=""
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    row="$(grep -P "^\Q$f\E\t" "$BASE" 2>/dev/null || grep -F "$f	" "$BASE" 2>/dev/null || true)"
    [ -n "$row" ] || continue
    burn="$(printf '%s' "$row" | cut -f4)"
    if [ "${burn:-0}" -gt 0 ]; then
      real=$((real + 1)); realtests=$((realtests + burn)); hits="$hits $f"
    else
      vac=$((vac + 1))
    fi
  done < <(grep -rlE "$needle" tests --include='*.bats' 2>/dev/null | sort -u)

  if [ "$real" -eq 0 ] && [ "$vac" -eq 0 ]; then
    verdict="HOLE -- nothing in the estate invokes it"
  elif [ "$real" -eq 0 ]; then
    verdict="VACUOUS -- $vac file(s) name it, none reach the CLI"
  elif [ "$realtests" -lt 3 ]; then
    verdict="THIN -- $realtests burning test(s)"
  else
    verdict="covered"
  fi
  printf '%-12s %8s %8s %8s   %s\n' "$famname" "$real" "$vac" "$realtests" "$verdict"
done
