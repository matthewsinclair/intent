#!/usr/bin/env bash
# stale_at_check.sh -- find AT rows whose citation already exists on disk while
# the row still claims the test is unwritten.
#
# WHY THIS EXISTS, and it is a hole in the grammar rather than a tidiness check.
#
# `to-write` is the one AT status exempt from the L2 and L3 lint levels, because
# both are gated on `green|red`. So a row parked at `to-write` is exempt from
# precisely the checks that would test its own citation. That makes it the one
# state a stale row can hide in indefinitely, and the lint stays green the whole
# time.
#
# The failure is not hypothetical and it is not rare. Measured 2026-08-16: FOUR
# rows carried `to-write` against a file that existed, and one of them
# (AT-06.10) had a complete, green, twelve-assertion test behind it. Its AC read
# unsatisfied and its work package read 5/11 when it was 6/11. A second
# (AT-00.8) was likewise finished and green. The other two cite a harness that
# exists and burns in, so they were never unwritten either -- they were failing
# for reasons nobody had written down.
#
# The structural cause is a handoff with no signal: the build belongs to the
# node writing the test and the status transition belongs to the node holding
# the thread, and nothing carries word between them. This script is the
# receiving half. It cannot replace the sending half -- it finds a landed test
# at the next run rather than immediately -- so it is a floor, not a substitute
# for the one-line handoff.
#
# WHAT IT DOES NOT DO. It reports that a citation EXISTS; it does not run the
# test and it does not judge coverage. Presence is not greenness, and a row
# whose file exists may still be correctly red. Every hit here is a prompt to go
# and look, never a verdict -- which is the same posture as the burn-in in
# `run_v2_suite.bash`: the instrument narrows where to look and a human decides.
#
# Usage:
#   bash intent/st/ST0056/parity/tools/stale_at_check.sh [<STID>]
#
# Exit status is 0 whether or not it finds anything. It is a report, not a gate:
# a legitimately absent citation is the normal state of most rows for most of
# this thread's life, so failing on a hit would make it a gate nobody keeps.

set -uo pipefail

STID="${1:-ST0056}"

# Walk up to the project root by its marker rather than counting `..` levels.
# Counting is what broke this on its first run: the tool sits five directories
# down, not four, and a wrong count fails as "no acceptance.md for <ST>", which
# reads like a missing thread rather than a missing root.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
while [ "$ROOT" != "/" ] && [ ! -f "$ROOT/intent/.config/config.json" ]; do
  ROOT="$(dirname "$ROOT")"
done

if [ ! -f "$ROOT/intent/.config/config.json" ]; then
  echo "error: no Intent project root above $(dirname "${BASH_SOURCE[0]}")" >&2
  echo "remedy: run this inside an Intent project" >&2
  exit 1
fi

cd "$ROOT" || exit 1

ACC=""
for candidate in "intent/st/${STID}/acceptance.md" "intent/st/"*"/${STID}/acceptance.md"; do
  [ -f "$candidate" ] && ACC="$candidate" && break
done

if [ -z "$ACC" ]; then
  echo "error: no acceptance.md for ${STID}" >&2
  echo "remedy: name a thread that has one, or run from the project root" >&2
  exit 1
fi

# The citation is the first backticked span on the row; the status is the token
# after `status: `. Both are grammar the AT linter already enforces at L1, so
# reading them here does not add a second parser for the row format -- it reads
# the two fields L1 has already guaranteed are present.
awk '
  /^- AT-/ {
    id = $2
    ref = ""
    if (match($0, /`[^`]+`/)) {
      ref = substr($0, RSTART + 1, RLENGTH - 2)
    }
    st = "?"
    if (match($0, /status: [a-z-]+/)) {
      st = substr($0, RSTART + 8, RLENGTH - 8)
    }
    if (st == "to-write" && ref != "") {
      print id "\t" ref
    }
  }
' "$ACC" | {
  found=0
  while IFS="$(printf '\t')" read -r id ref; do
    if [ -e "$ref" ]; then
      found=$((found + 1))
      printf 'stale: %-12s cites %s -- the file EXISTS while the row says to-write\n' "$id" "$ref"
    fi
  done

  if [ "$found" -eq 0 ]; then
    echo "ok: no to-write row cites a file that exists"
  else
    echo ""
    echo "note: presence is not greenness. Run each test before moving its row."
    echo "note: a row whose test exists and fails belongs at red WITH the reason named,"
    echo "      never at to-write -- to-write is exempt from L2 and L3."
  fi
}
