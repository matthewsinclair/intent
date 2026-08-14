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

# PER-FILE TIMEOUT. Without this a single hanging file stalls the ENTIRE sweep,
# silently and forever: the run sits inside `d=$(bats "$f" ...)` producing no
# output, and the partial TSV looks exactly like a sweep that is merely slow.
# Measured 2026-08-14: the sweep stopped at row 93 of 98 on
# `tests/unit/test_diogenes.bats` and sat there for THREE AND A HALF HOURS.
#
# The file is not the problem -- it passes 19/19 in seconds when run standalone,
# under both the default binding AND `INTENT_BIN=/usr/bin/false`. The hang is
# environmental to this harness's context (backgrounded, no controlling tty),
# which is precisely the kind of difference a sweep must survive rather than
# diagnose.
#
# A timed-out file is reported TIMEOUT and carries NO classification, for the
# same reason UNSTABLE does: a measurement that did not finish is not a
# measurement, and emitting a burn number for it would be inventing data. The
# sweep then continues, so ONE bad file costs one row instead of the whole run.
#
# GNU `timeout` is not on macOS by default; it arrives with coreutils, and CI
# runs a Linux leg. Detect once, and if neither spelling exists say so and run
# WITHOUT a timeout rather than silently skipping every file -- a sweep that
# quietly measured nothing is the failure this toolchain keeps refusing.
BURN_TIMEOUT="${BURN_TIMEOUT:-300}"
if command -v timeout >/dev/null 2>&1; then
  TO=(timeout "$BURN_TIMEOUT")
elif command -v gtimeout >/dev/null 2>&1; then
  TO=(gtimeout "$BURN_TIMEOUT")
else
  echo "burn.sh: no timeout(1) or gtimeout(1) on PATH -- running WITHOUT a per-file timeout; a hanging file will stall the sweep" >&2
  TO=()
fi

set -uo pipefail
WT="${WT:?set WT}"
cd "$WT"

# OPT-IN TAP CAPTURE. With BURN_TAP_DIR set, both runs' raw TAP output is kept
# per file, so a downstream tool can name WHICH tests burn rather than only how
# many. This exists so per-test adjudication does not need its own copy of the
# run logic -- gen_pertest.sh reads these files and never invokes bats.
#
# One instrument, one measurement, two granularities of classification. The
# alternative was a second tool doing its own two runs per file, which is two
# copies of the thing that has to stay identical for the numbers to be
# comparable at all, and the exact drift lib_corpus.sh exists to catch.
#
# Both bindings are captured, not just the mutant one. The mutant `not ok` set
# IS the burning set ONLY IF the default run is green, so the consumer must be
# able to check that per file rather than trust the estate-level claim.
BURN_TAP_DIR="${BURN_TAP_DIR:-}"
if [ -n "$BURN_TAP_DIR" ]; then
  mkdir -p "$BURN_TAP_DIR" || { echo "burn.sh: cannot create BURN_TAP_DIR=$BURN_TAP_DIR" >&2; exit 2; }
fi
tap_slug() { printf '%s' "$1" | tr '/' '_'; }

printf 'FILE\tTESTS\tDEFAULT_FAIL\tBURN\tSTATUS\n'

for f in $(find tests -name '*.bats' | sort); do
  total=$(grep -c '^@test' "$f" 2>/dev/null || echo 0)
  [ "$total" -gt 0 ] || continue
  # Capture the timeout's own exit code, not the pipeline's: `| grep` would
  # discard it and a timed-out run would report zero failures, which reads as a
  # clean pass. That is the same class as counting a file that never reached the
  # CLI as coverage.
  d_out="$("${TO[@]}" bats "$f" 2>&1)"; d_rc=$?
  m_out="$(INTENT_BIN=/usr/bin/false "${TO[@]}" bats "$f" 2>&1)"; m_rc=$?
  d=$(printf '%s' "$d_out" | grep -cE '^not ok' || true)
  m=$(printf '%s' "$m_out" | grep -cE '^not ok' || true)
  burn=$(( m - d ))
  [ "$burn" -lt 0 ] && burn=0

  # Keep the raw TAP for both bindings when asked. Written for EVERY file
  # including TIMEOUT and UNSTABLE ones, deliberately: the consumer needs to see
  # that a file's measurement was unusable, and a missing file would let it
  # infer "no failures" from an absence. Same reason a timed-out row is emitted
  # rather than dropped.
  if [ -n "$BURN_TAP_DIR" ]; then
    printf '%s\n' "$d_out" > "$BURN_TAP_DIR/$(tap_slug "$f").default.tap"
    printf '%s\n' "$m_out" > "$BURN_TAP_DIR/$(tap_slug "$f").mutant.tap"
  fi
  if [ "$d_rc" -eq 124 ] || [ "$m_rc" -eq 124 ]; then
    # 124 is timeout(1)'s "deadline reached". No classification: an unfinished
    # measurement is not a measurement.
    printf '%s\t%s\t--\t--\tTIMEOUT\n' "$f" "$total"
    continue
  elif [ "$d" -ne 0 ]; then
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
