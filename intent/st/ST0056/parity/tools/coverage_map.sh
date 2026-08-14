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
CANON="surface/dispatch-table.json"
[ -f "$BASE" ] || { echo "error: no burn baseline at $BASE" >&2; exit 1; }
[ -f "$CANON" ] || { echo "error: no dispatch-table canon at $CANON" >&2; exit 1; }

# The baseline must still describe the estate being grepped. Without this the
# map silently under-reports: it greps the CURRENT tests/ tree for files naming
# each family, then looks each one up in a baseline that may predate them.
. "$(dirname "${BASH_SOURCE[0]}")/lib_corpus.sh" || { echo "error: cannot source lib_corpus.sh -- refusing to map coverage without the corpus guard" >&2; exit 2; }
# Explicit `|| exit 2` rather than relying on `set -e` to notice: the exit code
# is the finding here, and 2 (tool refuses) must not be confused with 1.
corpus_require "$BASE" "coverage_map" "." || exit 2

printf '%-12s %8s %8s %8s %8s   %s\n' FAMILY REAL VACUOUS TESTS UNMEAS VERDICT
printf '%-12s %8s %8s %8s %8s   %s\n' ------ ---- ------- ----- ------ -------

for famname in $(jq -r '.families[].name' "$CANON"); do
  # `ac` and `at` are nouns of one binary; `st_zero` is also spelled `st zero`.
  case "$famname" in
    st_zero) needle='(run_intent|intent)[[:space:]]+(st_zero|st[[:space:]]+zero)' ;;
    *)       needle="(run_intent|intent)[[:space:]]+${famname}([[:space:]]|\")" ;;
  esac

  real=0; vac=0; realtests=0; unmeas=0; hits=""
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    row="$(grep -P "^\Q$f\E\t" "$BASE" 2>/dev/null || grep -F "$f	" "$BASE" 2>/dev/null || true)"
    if [ -z "$row" ]; then
      # NOT `continue`. The previous version skipped here, so a file present on
      # disk and absent from the baseline left the arithmetic entirely -- neither
      # REAL nor VACUOUS, just gone. That is how three files missing from a stale
      # baseline vanished from every family they touched while the table went on
      # printing a confident verdict. corpus_require above now makes this
      # unreachable; it is counted rather than deleted because a guard upstream
      # is a reason to keep the downstream count honest, not to stop keeping it.
      unmeas=$((unmeas + 1))
      continue
    fi
    burn="$(printf '%s' "$row" | cut -f4)"
    if ! burn_measured "$burn"; then
      # UNSTABLE and TIMEOUT rows carry `--`. The old test was
      # `[ "${burn:-0}" -gt 0 ]`, and `[ "--" -gt 0 ]` is a fatal integer error
      # under `set -e`: one timed-out file would have taken the whole map down.
      # Counting it as VACUOUS would be worse than crashing, though -- that
      # asserts the file does not reach the CLI, which is precisely what the
      # failed measurement did not establish.
      unmeas=$((unmeas + 1))
    elif [ "$(burn_num "$burn")" -gt 0 ]; then
      real=$((real + 1)); realtests=$((realtests + $(burn_num "$burn"))); hits="$hits $f"
    else
      vac=$((vac + 1))
    fi
  done < <(grep -rlE "$needle" tests --include='*.bats' 2>/dev/null | sort -u)

  if [ "$real" -eq 0 ] && [ "$vac" -eq 0 ] && [ "$unmeas" -eq 0 ]; then
    verdict="HOLE -- nothing in the estate invokes it"
  elif [ "$real" -eq 0 ] && [ "$vac" -eq 0 ]; then
    # Every file naming this family failed to measure. Emphatically NOT a hole:
    # a hole is a known absence of coverage, this is an absence of knowledge.
    verdict="UNKNOWN -- $unmeas file(s) name it, none measured"
  elif [ "$real" -eq 0 ]; then
    verdict="VACUOUS -- $vac file(s) name it, none reach the CLI"
  elif [ "$realtests" -lt 3 ]; then
    verdict="THIN -- $realtests burning test(s)"
  else
    verdict="covered"
  fi
  # An unmeasured file qualifies whatever verdict it sits under, so it is
  # appended rather than folded in -- "covered" over a partly-unmeasured family
  # is exactly the overclaim this column exists to prevent.
  [ "$unmeas" -gt 0 ] && [ "$real" -gt 0 ] && verdict="$verdict (+$unmeas unmeasured)"
  printf '%-12s %8s %8s %8s %8s   %s\n' "$famname" "$real" "$vac" "$realtests" "$unmeas" "$verdict"
done
