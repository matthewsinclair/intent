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
# tests/unit/stale_at_check.bats is the proof of the canon read.
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

# Walk up to the project root by its marker rather than counting `..` levels.
# Counting is what broke this on its first run: the tool sits five directories
# down, not four, and a wrong count fails as "no thread canon for <ST>", which
# reads like a missing thread rather than a missing root.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$HERE"
while [ "$ROOT" != "/" ] && [ ! -f "$ROOT/intent/.config/config.json" ]; do
  ROOT="$(dirname "$ROOT")"
done

if [ ! -f "$ROOT/intent/.config/config.json" ]; then
  echo "error: no Intent project root above $(dirname "${BASH_SOURCE[0]}")" >&2
  echo "remedy: run this inside an Intent project" >&2
  exit 1
fi

cd "$ROOT" || exit 1

# The one reading of a citation, `<file>` or `<file>:<line>` (issue 0670).
# shellcheck source=lib_citation.sh
. "$HERE/lib_citation.sh" || { echo "error: cannot read lib_citation.sh beside this tool" >&2; exit 1; }

command -v jq >/dev/null 2>&1 || { echo "error: jq is required to read the thread canon" >&2; echo "remedy: install jq" >&2; exit 2; }

# **THE ROWS ARE READ FROM THE THREAD CANON, NEVER FROM A RENDERED
# `acceptance.md`** (ST0096 WP-03). A thread's acceptance view is one of the
# aggregate views that leave git (ST0096 WP-02): a clone of an upgraded project
# holds none until a verb renders them, and a project not yet upgraded holds
# only the threads it realised. Read from the views, this check refused a fresh
# clone ("no live thread") and read a different population in every checkout.
# `intent/.canon/st/<ID>.json` is the truth the view is rendered from, and it
# never leaves git, so the same rows are read whether the views are tracked,
# untracked or absent. It reads content, so it asks no declared set for paths:
# it names no view at all. absent_at_check.sh reads the same canon the same way.
CANON_DIR="intent/.canon/st"
THREADS=()
SKIPPED=()
if [ -n "$STID" ]; then
  if [ ! -f "$CANON_DIR/${STID}.json" ]; then
    echo "error: no thread canon for ${STID} at $CANON_DIR/${STID}.json" >&2
    echo "remedy: name a thread this project has, or run from the project root" >&2
    exit 1
  fi
  THREADS=("$CANON_DIR/${STID}.json")
else
  # **A THREAD THAT HAS NOT STARTED CITES INTENDED HOMES RATHER THAN WRITTEN
  # TESTS, SO ITS ROWS ARE NOT STALE -- AND A CANCELLED THREAD IS FROZEN.**
  # Widening the default to every thread immediately printed NINE ST0046 rows,
  # all citing one v2 bats file that happens to exist. **Nine unactionable lines
  # on every commit is how an instrument dies:** a report nobody can act on
  # trains the reader to skip it, and the tenth line is the one that mattered.
  #
  # **`completed` IS DELIBERATELY IN SCOPE.** A finished thread carrying a
  # to-write row whose test exists is a real defect and this should say so. A
  # filter that skipped them would hide the class, not the noise.
  #
  # An unreadable status reads as `unknown` and keeps the thread in scope, so
  # the row pass below reads its canon and refuses on it.
  for canon in "$CANON_DIR"/*.json; do
    [ -f "$canon" ] || continue
    tid="$(basename "$canon" .json)"
    tstatus="$(jq -r '.status // "unknown"' "$canon" 2>/dev/null)"
    [ -n "$tstatus" ] || tstatus="unknown"
    case "$tstatus" in
      not-started|cancelled) SKIPPED+=("${tid}(${tstatus})") ;;
      *) THREADS+=("$canon") ;;
    esac
  done
  if [ "${#THREADS[@]}" -eq 0 ]; then
    # A project with no live thread is a real state and NOT a clean run: the
    # whole point of the count line is that "nothing violated" and "nothing was
    # examined" must never print the same thing.
    echo "error: no live thread canon under $CANON_DIR" >&2
    echo "remedy: run this inside an Intent project that has threads" >&2
    exit 1
  fi
fi

# **THE SKIP GOES ON THE COUNT LINE, NEVER ONLY IN A COMMENT.** A scoped report
# that does not say it is scoped is the exact defect repaired above.
SKIPNOTE=""
if [ "${#SKIPPED[@]}" -gt 0 ]; then
  SKIPNOTE=" (skipped ${#SKIPPED[@]} not-started/cancelled thread(s): ${SKIPPED[*]})"
fi

# ONE jq PASS OVER EVERY LIVE THREAD. A finding names its thread with the row,
# because AT ids are per-thread and two threads can each hold the same id.
#
# THE POPULATION IS COUNTED AND THE COUNT IS PRINTED (found by ic, 2026-08-16,
# against the rendered rows this read then). A renamed status field leaves every
# row matched while the status reads nothing for each of them; the loop then
# runs zero times and prints `ok`, BYTE-IDENTICAL to a genuine all-clean run.
# So a row without a status word is a BAD line and a refusal, never a data
# state, and the COUNT line carries how many rows were read at all.
#
# The citation is the row's `file` field, with a line suffix dropped (issue
# 0670). The rendered row's grammar is not read, so a backticked span in a
# note can no longer be taken for a citation.
SCAN="$(
  jq -n -r --arg line_re "$CITATION_LINE_RE" '
    [ inputs
      | (input_filename | split("/") | last | sub("\\.json$"; "")) as $tid
      | (.tests // [])[]
      | { tid: $tid, r: . } ]
    | . as $rows
    | ( $rows[]
        | (.tid + "/" + ((.r.id // "?") | tostring)) as $id
        | (.r.status // null) as $st
        | ((.r.file // "") | tostring | sub($line_re; "")) as $file
        | if ($st | type) != "string" or ($st | test("^[a-z-]+$") | not)
             or ($id | test("[\t\n\r]")) or ($file | test("[\t\n\r]")) then "BAD\t" + $id
          elif $st == "to-write" and $file != "" then "ROW\t" + $id + "\t" + $file
          else empty
          end ),
      ( "COUNT\t" + ($rows | length | tostring) )
  ' "${THREADS[@]}"
)" || {
  echo "error: the thread canon could not be read as JSON" >&2
  echo "remedy: check \`jq . $CANON_DIR/*.json\`; an unreadable canon must refuse, never report clean" >&2
  exit 2
}

printf '%s\n' "$SCAN" | {
  found=0
  bad=0
  matched=0
  examined=0
  while IFS="$(printf '\t')" read -r kind a b; do
    case "$kind" in
      BAD)
        bad=$((bad + 1))
        echo "error: ${a}: its canon row has no status word, or an id or file with a tab or newline" >&2
        ;;
      ROW)
        examined=$((examined + 1))
        if [ -e "$b" ]; then
          found=$((found + 1))
          printf 'stale: %-12s cites %s -- the file EXISTS while the row says to-write\n' "$a" "$b"
        fi
        ;;
      COUNT)
        matched="$a"
        ;;
      *)
        bad=$((bad + 1))
        echo "error: the canon pass printed a line this report cannot read: ${kind}" >&2
        ;;
    esac
  done

  if [ "$bad" -gt 0 ]; then
    echo "error: ${bad} of ${matched} AT row(s) in the canon could not be read -- the canon's row shape moved" >&2
    echo "remedy: this script reads .tests[] .id, .status and .file in $CANON_DIR/<ID>.json; align it with the fields the canon carries" >&2
    echo "note: refusing rather than reporting, because a parser that reads nothing prints the same" >&2
    echo "      line as a clean run -- which is the defect this script exists to find, one level up" >&2
    exit 2
  fi

  if [ "$matched" -eq 0 ]; then
    echo "error: no AT rows in ${#THREADS[@]} live thread canon file(s)" >&2
    echo "remedy: check the canon carries .tests[]; an extract with no rows is not a clean run" >&2
    exit 2
  fi

  # The population is on the ok line, so zero examined reads as zero rather than
  # as clean.
  if [ "$found" -eq 0 ]; then
    echo "ok: examined ${examined} to-write row(s) with a citation, of ${matched} AT row(s)${SKIPNOTE}; none names a file that exists"
  else
    echo ""
    echo "examined ${examined} to-write row(s) with a citation, of ${matched} AT row(s)${SKIPNOTE}"
    echo "note: presence is not greenness. Run each test before moving its row."
    echo "note: a row whose test exists and fails belongs at red WITH the reason named,"
    echo "      never at to-write -- to-write is exempt from L2 and L3."
  fi
}
