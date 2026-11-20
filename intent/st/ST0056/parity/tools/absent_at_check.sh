#!/usr/bin/env bash
# absent_at_check.sh -- does a row claiming a VERDICT cite an artefact that exists?
#
# THE MIRROR OF `stale_at_check.sh`, AND THE HALF NOBODY HAD BUILT. That one
# catches UNDERSTATEMENT: a `to-write` row whose cited test already exists. This
# one catches OVERSTATEMENT: a `green` or `red` row -- a row asserting a test was
# RUN and came to a verdict -- whose cited file is not there to have run.
#
# **A GREEN ROW IS THE ESTATE'S UNIT OF EVIDENCE.** `ac status`, the close gate
# and every N/M figure count it. A green citing nothing is not untidy
# bookkeeping; it is a claim with no referent being counted as a claim with one,
# and it is invisible to inspection because a green row looks like every other
# green row.
#
# ==========================================================================
# WHAT THIS COVERS, AND THE ONE THING IT CANNOT
# ==========================================================================
#
# COVERS: the ABSENT artefact. The row says green, the path resolves to
# nothing. Mechanical, two-sided, no judgement.
#
# **DOES NOT COVER: PRESENT-AND-HOLLOW -- and that is the case that produced
# this guard, so it is stated first rather than buried.** A cited file can be
# sitting right there, running, and passing for a reason that has nothing to do
# with its subject. The motivating instance: ST0043 `AT-01.8` cited
# `tests/unit/intent_claude_upgrade.bats`, which existed. Its test read
#
#     if grep -nE "sed -i ''" "$canon"; then fail ...; fi
#
# against a `$canon` deleted at `125f601d`. `grep` on an absent file errors, the
# `if` is false, `fail` never runs, and the test PASSES. Measured on that file
# the day it was deleted: 27 tests, 25 failing loudly on the missing script and
# **2 passing vacuously -- one of which was AT-01.8's own subject.** The green
# was as empty then as it would be with the file gone, and NOTHING HERE WOULD
# HAVE FIRED, because the citation resolved.
#
# So this guard catches the bookkeeping consequence of a delete, never the rot
# that preceded it. A green here means every verdict-claiming row has something
# at the other end of its citation -- not that the something can fail. **A
# reader who takes it for coverage has read more than it says.**
#
# The neighbours, so the seam is legible and nobody builds a third copy:
#
#   stale_at_check.sh     to-write + file EXISTS      (the opposite direction)
#   declared_kind_check.sh  file EXISTS + wrong kind  (explicitly declines the
#                                                      absent case, in its own
#                                                      header, as not its subject)
#   this file             green|red + file ABSENT
#
# ==========================================================================
# WHY IT READS THE COMMITTED CANON EXTRACT
# ==========================================================================
#
# **NOT THE RENDERED `acceptance.md`, AND THIS IS THE WHOLE REASON THE DEFECT
# SURVIVED.** Measured 2026-08-30: 69 threads in canon, 13 carrying an
# `acceptance.md`. A tool globbing `intent/st/*/acceptance.md` is blind to 56
# threads -- and ST0043, which holds the motivating row, is one of the 56 and has
# no directory on disk at all. `stale_at_check.sh` scans the tree, so it could
# never have seen this row in either direction. **A scoped instrument whose
# caller did not choose the scope is this estate's recurring shape, and that
# script's own header records being fixed for it once already, one level up.**
#
# **NOT THE STORE**, for `declared_kind_check.sh`'s stated reason, adopted
# rather than re-argued: the extract is what a fresh clone and CI have, and a
# tool needing a machine-local `.cache/intent.db` refuses in exactly the
# population most likely to carry a divergence nobody has looked at.
#
# **AND READING THE TYPED `file` FIELD IS NOT THE SAME AS SCRAPING THE ROW.**
# The rendered row's citation is "the first backticked span before the first
# ` -- `", which on a non-test row picks up whatever the prose happened to
# backtick. Driven over the tree on the day this was written, that heuristic
# reported six n/a rows as citing absent files: `INTENT_BIN`, `ws`, `wip`,
# `doctor --fix`, `cold_start_history.rs`, `conservation_check.sh` -- not one of
# them a citation, all six false. The extract has `file: null` for every one.
#
# ==========================================================================
# THE POPULATION, AND WHY EACH EXCLUSION IS PRINCIPLED
# ==========================================================================
#
# IN:  kind == test, status in {green, red}, file non-empty.
#
# OUT: `to-write`. **An unwritten row citing its intended home is the NORMAL
#      state, not a defect** -- 51 of 336 cited rows are exactly that today, and
#      gating on them would refuse most of this thread's life. That direction
#      belongs to `stale_at_check.sh` and only once the file appears.
#
# OUT: `non-test` / `n-a`. Measured: 35 of 35 carry no `file`, so they are not
#      in the population by construction rather than by exclusion. This is the
#      same fact `AtStatus::permitted_for` enforces on the write side -- `Na`
#      only for `NonTest`, `Green`/`Red`/`ToWrite` only for `Test` -- so the two
#      halves cannot drift apart without one of them going red.
#
# Exit 0 clean, 1 findings, 2 refusal.
#
# ==========================================================================
# MUTATION PROOFS -- driven 2026-08-30 (dc), against a planted fixture tree
# ==========================================================================
#
# A check whose failure path has never fired is a claim, not an instrument, and
# this one GATES. Driven against a throwaway tree (an `intent/.config/config.json`
# marker plus a hand-written `intent/.canon/st/STXXXX.json`) so the root walk
# resolves to the fixture and nothing real is touched:
#
#   green row, file PRESENT            -> exit 0, "none cites a missing file"
#   green row, file ABSENT             -> exit 1, names thread, row and path
#   red row, file ABSENT               -> exit 1, red is a verdict too
#   to-write row, file ABSENT          -> exit 0, NOT a finding (the normal state)
#   n-a non-test row, no file          -> exit 0, out of the population
#   no thread files at all             -> exit 2, refuses; never a clean zero
#   thread files present, zero AT rows -> exit 2, refuses
#   AT rows present, zero in-population-> exit 2, refuses
#   `status` renamed to `state`        -> exit 2, refuses (the grammar moved)
#
# **THE LAST FOUR ARE THE POINT.** This estate's recurring instrument defect is
# a tool that reads nothing and prints the same line as a clean run. Here the
# whole population can legitimately be empty -- a young thread has no green rows
# -- so "0 examined" is genuinely ambiguous and the tool refuses instead of
# choosing the reassuring reading. The count is on the ok line either way.

# No -e: every step's status is read where it happens, so a failure is named
# and refused, never an unexplained abort.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# THE CANON IS READ THROUGH lib_at_canon.sh, stale_at_check.sh's reader too
# (issue 0733). It refuses with exit 2 on anything it cannot read, before this
# file sees a row: until then a row whose status was null, a number or a
# misspelt word fell quietly out of the population below, and jq's message
# naming an unreadable file went to /dev/null. Every thread is read, whatever
# its status: a verdict claimed in a cancelled thread is still counted.
# shellcheck source=lib_at_canon.sh
. "$HERE/lib_at_canon.sh" || { echo "error: cannot load lib_at_canon.sh beside this tool, or a library it sources (named above if so)" >&2; exit 2; }
at_canon_root "$HERE" || exit 2
at_canon_ready || exit 2
# The words this tool judges by are the model's, or it refuses.
at_canon_legal AtKind test || exit 2
at_canon_legal AtStatus green red || exit 2

examined=0
found=0
FINDINGS=""

at_thread() { :; }

# A finding names its thread with the row: AT ids are per-thread and this
# estate holds two distinct `AT-01.8` rows (ST0043 and ST0050), so a finding
# that named the row alone would send the reader to the wrong thread.
at_row() {
  if [ "$3" != "test" ] || [ -z "$5" ]; then
    return 0
  fi
  case "$4" in
    green | red) ;;
    *) return 0 ;;
  esac
  examined=$((examined + 1))
  if [ ! -e "$5" ]; then
    found=$((found + 1))
    FINDINGS="${FINDINGS}absent: ${1}/${2} is ${4} and cites ${5} -- the file is NOT THERE
"
  fi
}

at_canon_read "${AT_CANON_FILES[@]}" || exit 2
total="$AT_CANON_ROWS"

# **ZERO IN POPULATION IS A REFUSAL, NOT A PASS.** Every other outcome here is
# measured against rows that were actually examined; with none, the exit code
# would be reporting on the filter rather than on the estate.
if [ "$examined" -eq 0 ]; then
  at_canon_refuse "of ${total} AT row(s) across ${#AT_CANON_FILES[@]} thread(s), NONE is a green or red test row with a citation" \
    "either the estate genuinely has no verdict-claiming row yet -- in which case this gate has nothing to say and should not be in the runner -- or the filter no longer matches the model's field values"
  exit 2
fi

if [ "$found" -eq 0 ]; then
  echo "ok: examined ${examined} green/red row(s) with a citation, of ${total} AT row(s) across ${#AT_CANON_FILES[@]} thread(s); none cites a missing file"
  exit 0
fi

printf '%s' "$FINDINGS"
echo ""
echo "examined ${examined} green/red row(s) with a citation, of ${total} AT row(s) across ${#AT_CANON_FILES[@]} thread(s)"
echo "note: a green row is counted as evidence by \`ac status\` and the close gate."
echo "      A row whose subject is gone is RETIRED, not left green -- withdraw the AC"
echo "      it covers (\`intent ac withdraw\`) and move the row to a non-test n/a with"
echo "      the reason in its prose, which is what the estate already does for a"
echo "      criterion whose subject expired."
echo "note: presence is not soundness. This says the file is THERE, never that its"
echo "      assertions can still fail -- see this script's header."
exit 1
