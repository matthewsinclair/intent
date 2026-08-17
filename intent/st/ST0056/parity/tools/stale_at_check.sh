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
#
# THE POPULATION IS COUNTED AND THE COUNT IS PRINTED, and this was not in the
# first version (found by ic, 2026-08-16, demonstrated rather than read, within
# the hour this shipped). Renaming one token of the row grammar -- `status:` to
# `state:` -- leaves `^- AT-` matching all 109 rows while the status extraction
# returns nothing for every one of them. The awk then emits zero rows, the loop
# runs zero times, and the script prints `ok: no to-write row cites a file that
# exists`, BYTE-IDENTICAL to a genuine all-clean run.
#
# Note where that fails: BELOW the row match, in field extraction. So a guard
# asking "did I find any AT rows?" would also have passed -- which is why the
# refusal below is on an unparseable STATUS rather than on an empty file.
#
# This is the same defect one level up from the one the script exists to find.
# `to-write` is the AT state nothing validates; "zero rows examined" was the
# script state nothing validated. An instrument reporting on a population it
# failed to read is the shape ic hit three times in one day -- a check claiming
# in its own error message to catch retired commands and not seeing them, and a
# flag sweep losing three flags to a `@tsv` escape while printing agreement.
# "Nothing violated anything" and "nothing was examined" are the same output
# unless the count is on the line.
awk '
  /^- AT-/ {
    matched++
    id = $2
    ref = ""
    if (match($0, /`[^`]+`/)) {
      ref = substr($0, RSTART + 1, RLENGTH - 2)
    }
    if (match($0, /status: [a-z-]+/)) {
      st = substr($0, RSTART + 8, RLENGTH - 8)
    } else {
      # L1 guarantees every AT row carries a status, so a row matching `^- AT-`
      # whose status will not parse is a broken PARSER, never a data state.
      print "BAD\t" id
      next
    }
    if (st == "to-write" && ref != "") {
      examined++
      print "ROW\t" id "\t" ref
    }
  }
  END { print "COUNT\t" matched + 0 "\t" examined + 0 }
' "$ACC" | {
  found=0
  bad=0
  matched=0
  examined=0
  while IFS="$(printf '\t')" read -r kind a b; do
    case "$kind" in
      BAD)
        bad=$((bad + 1))
        echo "error: ${a}: matched as an AT row but its status will not parse" >&2
        ;;
      ROW)
        if [ -e "$b" ]; then
          found=$((found + 1))
          printf 'stale: %-12s cites %s -- the file EXISTS while the row says to-write\n' "$a" "$b"
        fi
        ;;
      COUNT)
        matched="$a"
        examined="$b"
        ;;
    esac
  done

  if [ "$bad" -gt 0 ]; then
    echo "error: ${bad} of ${matched} AT row(s) matched but did not parse -- the row grammar moved" >&2
    echo "remedy: this script reads \`status: <word>\`; align it with the grammar the AT linter enforces" >&2
    echo "note: refusing rather than reporting, because a parser that reads nothing prints the same" >&2
    echo "      line as a clean run -- which is the defect this script exists to find, one level up" >&2
    exit 2
  fi

  if [ "$matched" -eq 0 ]; then
    echo "error: no AT rows matched in ${ACC}" >&2
    echo "remedy: check the file is an acceptance.md with AT rows, not an empty or renamed thread" >&2
    exit 2
  fi

  # The population is on the ok line, so zero examined reads as zero rather than
  # as clean.
  if [ "$found" -eq 0 ]; then
    echo "ok: examined ${examined} to-write row(s) with a citation, of ${matched} AT row(s); none names a file that exists"
  else
    echo ""
    echo "examined ${examined} to-write row(s) with a citation, of ${matched} AT row(s)"
    echo "note: presence is not greenness. Run each test before moving its row."
    echo "note: a row whose test exists and fails belongs at red WITH the reason named,"
    echo "      never at to-write -- to-write is exempt from L2 and L3."
  fi
}
