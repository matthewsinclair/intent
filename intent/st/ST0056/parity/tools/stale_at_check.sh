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
# It exits 2, and never 1, when it cannot answer: no project root, an unknown
# thread, no live thread, or a canon lib_at_canon.sh cannot read (issue 0733;
# until then those exits were 1).

# MUTATION PROOFS, run 2026-08-17 at `67814555` (ic). Co-located because a check
# whose failure path has never fired is a claim, not an instrument -- and this one
# is GATED into the pre-commit runner now. Driven against a throwaway fixture tree
# (a `intent/.config/config.json` marker, one `acceptance.md`, one file to cite)
# with this script copied into it, so the root walk resolves to the fixture and
# nothing real is touched.
#
#   cited file absent                -> exit 0, "none names a file that exists"
#   cited test EXISTS                -> exit 0, reports the stale row (the finding)
#   `status:` renamed to `state:`    -> exit 2, refuses; never a clean zero
#   UNCITED row, backticks in note   -> exit 0, "examined 0 ... with a citation"
#   real citation, note containing
#   three further ` -- `             -> exit 0, still reports; boundary unmoved
#
# **The fourth case was a REAL FALSE POSITIVE and this check had it until today.**
# Reading the first backtick anywhere on the line made a note's backticked span
# into a citation, so an uncited row was reported as `cites <path> -- the file
# EXISTS`, sending the reader to run a test the row never named. Measured exposure
# on this estate when it was fixed: 112 AT rows, 53 to-write, **0 exposed** -- so
# it was latent, not live, and it goes live the first time anyone writes an uncited
# to-write row with a backticked note. Prompted by cc from the opposite end: they
# captured 14 v2-authored rows and found nine whose note contains a further ` -- `.
#
# Those proofs drove the rendered rows this read until ST0096 WP-03. It reads
# the thread canon now, so the backticked-note cases no longer arise, and
# tests/unit/stale_at_check.bats is the proof of the canon read, for this
# tool and absent_at_check.sh both.
# No -e: every step's status is read where it happens, so a failure is named
# and refused, never an unexplained abort.
set -uo pipefail

# **NO ARGUMENT MEANS EVERY LIVE THREAD, AND IT USED TO MEAN ST0056** (vc
# 2026-08-19). The default was one thread and the pre-commit hook invokes this
# with NO argument, so **ST0057 had never been examined once** -- 46 of the
# release gate's 64 rows, unwatched by the only instrument that looks in the
# understatement direction. Three of its rows were stale when the scope was
# fixed. cc found one by hand, which is the only reason the blind spot
# surfaced.
#
# **THE DEFECT WAS NOT THE DEFAULT BEING WRONG. It was a scoped tool with an
# unscoped caller producing a clean report that meant "clean, within a scope
# nobody at the call site had chosen"** -- and the count line said `of 132 AT
# rows` while the estate held 294, so the number that exists to stop a silent
# zero was itself silently scoped.
STID="${1:-}"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# **THE ROWS ARE READ FROM THE THREAD CANON, NEVER FROM A RENDERED
# `acceptance.md`** (ST0096 WP-03). A thread's acceptance view is one of the
# aggregate views that leave git (ST0096 WP-02): a clone of an upgraded project
# holds none until a verb renders them, and a project not yet upgraded holds
# only the threads it realised. `intent/.canon/st/<ID>.json` is the truth the
# view is rendered from, and it never leaves git. It is read through
# lib_at_canon.sh, absent_at_check.sh's reader too (issue 0733), which refuses
# with exit 2 on anything it cannot read, so this file holds only what makes a
# row stale.
# shellcheck source=lib_at_canon.sh
. "$HERE/lib_at_canon.sh" || { echo "error: cannot load lib_at_canon.sh beside this tool, or a library it sources (named above if so)" >&2; exit 2; }
at_canon_root "$HERE" || exit 2
at_canon_ready || exit 2
# The words this tool judges by are the model's, or it refuses.
at_canon_legal AtStatus to-write || exit 2
at_canon_legal ThreadStatus not-started cancelled || exit 2

THREADS=("${AT_CANON_FILES[@]}")
if [ -n "$STID" ]; then
  if [ ! -f "$AT_CANON_DIR/${STID}.json" ]; then
    at_canon_refuse "no thread canon for ${STID} at $AT_CANON_DIR/${STID}.json" \
      "name a thread this project has"
    exit 2
  fi
  THREADS=("$AT_CANON_DIR/${STID}.json")
fi

# **A THREAD THAT HAS NOT STARTED CITES INTENDED HOMES RATHER THAN WRITTEN
# TESTS, SO ITS ROWS ARE NOT STALE -- AND A CANCELLED THREAD IS FROZEN.**
# Widening the default to every thread immediately printed NINE ST0046 rows,
# all citing one v2 bats file that happens to exist. **Nine unactionable lines
# on every commit is how an instrument dies:** a report nobody can act on
# trains the reader to skip it, and the tenth line is the one that mattered.
# A thread named on the command line is read whatever its status.
#
# **`completed` IS DELIBERATELY IN SCOPE.** A finished thread carrying a
# to-write row whose test exists is a real defect and this should say so. A
# filter that skipped them would hide the class, not the noise.
#
# The skipped threads, one per line between newlines, matched whole: no
# associative array, because the gate may run this under bash 3.2.
SKIPLIST=$'\n'
SKIPPED=()
live=0
matched=0
examined=0
found=0

at_thread() {
  case "$2" in
    not-started | cancelled)
      if [ -z "$STID" ]; then
        SKIPLIST="${SKIPLIST}${1}"$'\n'
        SKIPPED+=("${1}($2)")
        return
      fi
      ;;
  esac
  live=$((live + 1))
}

# A finding names its thread with the row, because AT ids are per-thread and
# two threads can each hold the same id. The citation is the row's `file`, its
# line suffix already taken off (issue 0670).
at_row() {
  case "$SKIPLIST" in
    *$'\n'"$1"$'\n'*) return 0 ;;
  esac
  matched=$((matched + 1))
  if [ "$4" != "to-write" ] || [ -z "$5" ]; then
    return 0
  fi
  examined=$((examined + 1))
  if [ -e "$5" ]; then
    found=$((found + 1))
    printf 'stale: %-12s cites %s -- the file EXISTS while the row says to-write\n' "$1/$2" "$5"
  fi
}

at_canon_read "${THREADS[@]}" || exit 2

if [ "$live" -eq 0 ]; then
  # A project with no live thread is a real state and NOT a clean run: the
  # whole point of the count line is that "nothing violated" and "nothing was
  # examined" must never print the same thing.
  at_canon_refuse "no live thread canon under $AT_CANON_DIR" "run this inside an Intent project that has threads"
  exit 2
fi

# THE POPULATION IS COUNTED AND THE COUNT IS PRINTED (found by ic, 2026-08-16).
# A reader that reads nothing must never print the line a clean run prints, so
# live threads holding no row refuse rather than report.
if [ "$matched" -eq 0 ]; then
  at_canon_refuse "no AT rows in ${live} live thread canon file(s)" \
    "check the canon carries .tests[]; an extract with no rows is not a clean run"
  exit 2
fi

# **THE SKIP GOES ON THE COUNT LINE, NEVER ONLY IN A COMMENT.** A scoped report
# that does not say it is scoped is the exact defect repaired above.
SKIPNOTE=""
if [ "${#SKIPPED[@]}" -gt 0 ]; then
  SKIPNOTE=" (skipped ${#SKIPPED[@]} not-started/cancelled thread(s): ${SKIPPED[*]})"
fi

# The population is on the ok line, so zero examined reads as zero rather than
# as clean. Zero to-write rows with a citation is a normal state, unlike
# absent_at_check.sh's zero, so it is reported, not refused.
if [ "$found" -eq 0 ]; then
  echo "ok: examined ${examined} to-write row(s) with a citation, of ${matched} AT row(s)${SKIPNOTE}; none names a file that exists"
else
  echo ""
  echo "examined ${examined} to-write row(s) with a citation, of ${matched} AT row(s)${SKIPNOTE}"
  echo "note: presence is not greenness. Run each test before moving its row."
  echo "note: a row whose test exists and fails belongs at red WITH the reason named,"
  echo "      never at to-write -- to-write is exempt from L2 and L3."
fi
