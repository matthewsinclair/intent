#!/usr/bin/env bats
# Guard: no naked ((x++)) / ((x--)) increment statements in bin/.
#
# ((counter++)) evaluates to the pre-increment value and returns exit status 1
# when that value is 0. Every bin/ entrypoint runs under `set -e`, and sourced
# libraries inherit it, so on bash 5.x (Linux) the first such increment at zero
# aborts the script. bash 3.2 (macOS) is lenient about it, which is exactly how
# `intent organize` shipped broken on Linux for four releases behind macOS-green
# CI. Use `x=$((x + 1))` (an assignment always returns 0) instead.
#
# ==========================================================================
# THIS GUARD COULD NOT FAIL UNTIL 2026-08-31, AND THE CLASS HAS A NAME
# ==========================================================================
#
# It was `run grep -rnE ... ; if [ "$status" -eq 0 ]; then fail`. **grep exits 1
# on a clean corpus and 2 on a root that does not exist**, and only 0 was
# treated as a finding -- so a typo in the root, a renamed directory, or a
# corpus that had gone empty all reported the same clean line as a genuinely
# clean estate.
#
# **THE HONEST-AND-BLIND-GREP FAMILY** (vc's name, ruled to this file
# 2026-08-31): an instrument whose output is independent of what it measures,
# with a control that would pass just as well under the broken instrument. The
# sibling instance was `helpers.bats:198`, fixed the same morning at `0579fc09`
# and now `tests/unit/shell_error_voice.bats`; the same shape has been measured
# this week in a `grep -c ... || echo 0` emitting TWO zeros, and in a derived
# set whose loss produced a confident 22-of-22 partition with every entry wrong.
#
# **THE CURE IS NOT A BETTER PREDICATE, IT IS THE THREE-WAY CASE PLUS A STATED
# DENOMINATOR.** 0 is a finding, 1 is clean, anything else is the INSTRUMENT --
# and a denominator assertion is the only thing that separates a clean corpus
# from no corpus at all. Both halves or neither: the three-way case alone still
# reports clean over an empty directory that exists.
#
# THE CORPUS SHRINKS UNDER THE PRUNE AND THE ARM DOES NOT MOVE. `bin/` is
# population A + B today; the ST0056 prune deletes A, leaving B. That is a
# change to what is WATCHED and not to what is asserted -- which is only true
# because the denominator below is asserted rather than assumed.

load "../lib/test_helper.bash"

# The naked-increment needle. Kept in one place so the positive control drives
# the same instrument the guard does -- a control over a different invocation
# vouches for nothing.
NEEDLE='\(\([a-zA-Z_][a-zA-Z0-9_]*(\+\+|--)\)\)'

@test "the corpus this guard scans is real, and its size is reported" {
  root="${INTENT_PROJECT_ROOT}/bin"
  [ -d "$root" ] || fail "corpus root is missing, so a clean result below would be a statement about the instrument: $root"

  n=$(find "$root" -type f | wc -l | tr -d ' ')
  [ "$n" -gt 0 ] || fail "the corpus is empty -- a scan of nothing cannot report an estate"
  echo "corpus: $n file(s) under $root"
}

@test "no naked ((x++)) / ((x--)) increments under set -e (bash 5.x errexit footgun)" {
  run grep -rnE "$NEEDLE" "${INTENT_PROJECT_ROOT}/bin"

  # THREE OUTCOMES, NOT TWO. Exit 2 is the instrument failing to read the
  # corpus, and reading it as a clean estate is the defect this file carried.
  case "$status" in
    0)
      fail "Naked arithmetic increments found -- use x=\$((x + 1)):
$output"
      ;;
    1) : ;;
    *)
      fail "the scan could not read the corpus (grep exit $status) -- this is a
broken instrument, not a clean estate:
$output"
      ;;
  esac
}

@test "the scan fires on a planted increment and not on the assignment form" {
  # Driven in a tempdir. A positive control planted in the tracked tree is one
  # `git add -A` away from being committed by any node on this checkout.
  fixture=$(mktemp -d)

  printf '%s\n' 'count=0' '((count++))' > "$fixture/offender.sh"
  run grep -rnE "$NEEDLE" "$fixture"
  [ "$status" -eq 0 ] || fail "the scan did not find a planted ((count++)), so every clean result above says nothing"
  [[ "$output" == *"offender.sh"* ]] || fail "the scan matched but did not name the file: $output"

  # And it does NOT fire on the prescribed fix, or a guard that reports the
  # remedy as the defect gets switched off. `((count--))` is checked too: the
  # `--` alternation is the half a needle edit is most likely to drop.
  rm -f "$fixture/offender.sh"
  printf '%s\n' 'count=$((count + 1))' 'count=$((count - 1))' 'echo "-- a comment --"' > "$fixture/correct.sh"
  run grep -rnE "$NEEDLE" "$fixture"
  [ "$status" -eq 1 ] || fail "the scan fired on the assignment form this guard prescribes: $output"

  printf '%s\n' 'i=1' '((i--))' > "$fixture/decrement.sh"
  run grep -rnE "$NEEDLE" "$fixture"
  [ "$status" -eq 0 ] || fail "the needle lost its decrement alternation: ((i--)) went unfound"

  rm -rf "$fixture"
}
