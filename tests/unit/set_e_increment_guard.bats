#!/usr/bin/env bats
# Guard: no naked ((x++)) / ((x--)) increment statements in bin/ or scripts/.
#
# ((counter++)) evaluates to the pre-increment value and returns exit status 1
# when that value is 0. Every bin/ entrypoint runs under `set -e`, and sourced
# libraries inherit it, so on bash 5.x (Linux) the first such increment at zero
# aborts the script. bash 3.2 (macOS) is lenient about it, which is exactly how
# `intent organize` shipped broken on Linux for four releases behind macOS-green
# CI. Use `x=$((x + 1))` (an assignment always returns 0) instead.

load "../lib/test_helper.bash"

@test "no naked ((x++)) / ((x--)) increments under set -e (bash 5.x errexit footgun)" {
  run grep -rnE '\(\([a-zA-Z_][a-zA-Z0-9_]*(\+\+|--)\)\)' "${INTENT_PROJECT_ROOT}/bin" "${INTENT_PROJECT_ROOT}/scripts"
  if [ "$status" -eq 0 ]; then
    echo "Naked arithmetic increments found -- use x=\$((x + 1)):"
    echo "$output"
    return 1
  fi
}
