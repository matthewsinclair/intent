#!/bin/bash
# lib_corpus.sh -- does a burn TSV still describe the estate on disk?
#
# SOURCED, NOT EXECUTED. It defines functions and exits nothing; running it
# directly does nothing useful, so it ships 644 like `bin/intent_helpers` and
# unlike every other tool in this directory.
#
# WHY THIS IS SHARED RATHER THAN INLINE. The same defect appeared independently
# in both consumers of `burn-baseline.tsv`, which is the signature of a concern
# with no home:
#
#   gen_register.sh   generated a register from a 94-row TSV against a 97-file
#                     estate and reported "97 rows" without noticing three files
#                     had never been measured at all
#   coverage_map.sh   did worse -- `[ -n "$row" ] || continue` SKIPS a file that
#                     is absent from the baseline, so those same three files
#                     were counted as neither REAL nor VACUOUS in any family
#                     they touched. They simply left the arithmetic
#
# Two copies of one comparison had already drifted into two different wrong
# behaviours before either was noticed. That is Highlander's case made by
# demonstration, so the comparison lives here once.
#
# THE ASYMMETRY WORTH KNOWING. `drift_check.sh` deliberately reports and refuses
# to resolve, because there a disagreement has two causes with opposite remedies
# and a tool picking one would destroy the other. This check is the opposite
# shape and so takes the opposite policy: a file on disk and missing from the
# TSV was never measured, and a file in the TSV and gone from disk is a phantom
# row. Neither reading is defensible, so there is nothing to adjudicate -- only
# a sweep to re-run. Same toolchain, different policy, for a stated reason.

# corpus_diff <tsv> [estate_root]
#
# Prints one `MISSING <path>` or `PHANTOM <path>` line per disagreement.
# Returns 0 when the TSV and the estate agree, 1 when they do not, 2 when the
# TSV cannot be read -- an unreadable baseline is never silently "in agreement".
corpus_diff() {
  local tsv="$1" root="${2:-.}"
  [ -f "$tsv" ] || { echo "corpus_diff: no such TSV: $tsv" >&2; return 2; }

  local tsv_files disk_files missing phantom
  # `tail -n +2` drops the header row. A TSV that is header-only yields an empty
  # set, which then reports every file on disk as MISSING -- correct, and much
  # louder than treating "no rows" as "nothing to check".
  tsv_files="$(tail -n +2 "$tsv" | cut -f1 | grep -v '^$' | sort -u)"
  disk_files="$(cd "$root" && find tests -name '*.bats' | sort -u)"

  missing="$(comm -13 <(printf '%s\n' "$tsv_files") <(printf '%s\n' "$disk_files") | grep -v '^$' || true)"
  phantom="$(comm -23 <(printf '%s\n' "$tsv_files") <(printf '%s\n' "$disk_files") | grep -v '^$' || true)"

  [ -n "$missing" ] && printf 'MISSING %s\n' $missing
  [ -n "$phantom" ] && printf 'PHANTOM %s\n' $phantom
  [ -z "$missing" ] && [ -z "$phantom" ]
}

# corpus_require <tsv> <caller> [estate_root]
#
# The refusing wrapper both consumers use. Prints the disagreement with the
# remedy attached and returns 2, so a caller under `set -e` dies here rather
# than proceeding to publish a number computed over the wrong corpus.
#
# THE `|| rc=$?` IS LOAD-BEARING, and it took a live failure to find. A bare
# `out="$(corpus_diff ...)"` is a simple command, so under a caller running
# `set -e` the shell aborts THE MOMENT corpus_diff reports a disagreement --
# before this function can print what it found. Measured: coverage_map.sh
# (`set -euo pipefail`) exited 1 with an empty stderr against a baseline known
# to be 4 files short, while gen_register.sh (`set -uo pipefail`, no `-e`)
# reported the same disagreement correctly. A guard that dies silently in the
# one caller that runs strict mode is worse than no guard: it looks like a
# clean tool failure rather than a finding. Writing it as a `||` list takes the
# command out of set -e's scope so the report survives either caller.
corpus_require() {
  local tsv="$1" caller="$2" root="${3:-.}" out rc=0
  out="$(corpus_diff "$tsv" "$root")" || rc=$?
  [ "$rc" -eq 0 ] && return 0
  [ "$rc" -eq 2 ] && { echo "$caller: cannot read the burn TSV at $tsv -- refusing to report on an estate it has not seen" >&2; return 2; }

  echo "$caller: the burn TSV does not cover the on-disk .bats estate -- refusing to publish a figure computed over the wrong corpus" >&2
  printf '%s\n' "$out" | while read -r kind path; do
    case "$kind" in
      MISSING) echo "  NEVER MEASURED (on disk, absent from the TSV): $path" >&2 ;;
      PHANTOM) echo "  STALE ROW (in the TSV, gone from disk):        $path" >&2 ;;
    esac
  done
  echo "  Remedy: re-run burn.sh over the current estate. Do not hand-patch the TSV -- a baseline stitched from several revisions is the drift this check exists to catch." >&2
  return 2
}

# burn_num <value>
#
# Normalise a burn cell for arithmetic. UNSTABLE and TIMEOUT rows carry `--`,
# and `[ "--" -gt 0 ]` is a fatal integer error under `set -e` -- which is how a
# timed-out file would have taken the whole coverage map down with it. Callers
# get 0 AND can distinguish it: `burn_measured` says whether the 0 is real.
burn_num() { case "${1:-}" in ''|*[!0-9]*) echo 0 ;; *) echo "$1" ;; esac; }

# burn_measured <value> -- true when the cell holds an actual measurement.
burn_measured() { case "${1:-}" in ''|*[!0-9]*) return 1 ;; *) return 0 ;; esac; }
