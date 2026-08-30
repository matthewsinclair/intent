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

set -uo pipefail

die() {
  echo "error: $1" >&2
  [ -n "${2:-}" ] && echo "remedy: $2" >&2
  exit 2
}

# Walk up to the project root by its marker rather than counting `..` levels --
# `stale_at_check.sh`'s lesson, inherited: the tool sits five directories down
# and a wrong count fails as a missing thread rather than a missing root.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
while [ "$ROOT" != "/" ] && [ ! -f "$ROOT/intent/.config/config.json" ]; do
  ROOT="$(dirname "$ROOT")"
done
[ -f "$ROOT/intent/.config/config.json" ] \
  || die "no Intent project root above $(dirname "${BASH_SOURCE[0]}")" \
         "run this inside an Intent project"

cd "$ROOT" || exit 2
command -v jq >/dev/null 2>&1 || die "jq is required to read the canon extract" "install jq"

CANON_DIR="intent/.canon/st"
[ -d "$CANON_DIR" ] || die "no canon extract at $CANON_DIR" \
  "run \`intent sync\` to write the extract, or run this in a project that has one"

THREADS=()
while IFS= read -r f; do THREADS+=("$f"); done < <(find "$CANON_DIR" -name '*.json' -type f | sort)

# A project with no thread canon is a real state and NOT a clean run. "Nothing
# violated" and "nothing was examined" must never print the same line.
[ "${#THREADS[@]}" -gt 0 ] \
  || die "no thread canon under $CANON_DIR" \
         "this gate reads the committed extract; \`intent sync\` writes it"

# ONE jq PASS OVER EVERY THREAD. `input_filename` carries the thread id, which
# the rows themselves do not: AT ids are per-thread and this estate holds two
# distinct `AT-01.8` rows (ST0043 and ST0050), so a finding that named the row
# alone would send the reader to the wrong thread.
#
# TOTAL and POPULATION are emitted as records rather than counted afterwards,
# so a jq filter that silently matches nothing is distinguishable from a corpus
# that genuinely holds nothing.
SCAN="$(
  jq -r '
    input_filename as $f
    | ($f | split("/") | last | sub("\\.json$"; "")) as $tid
    | (.tests // []) as $rows
    | ($rows | length) as $n
    | ("TOTAL\t" + ($n | tostring)),
      ( $rows[]
        | . as $r
        | (($r.status // "") | tostring) as $st
        | (($r.kind // "") | tostring) as $kind
        | (($r.file // "") | tostring) as $file
        | if ($r | has("status")) then empty else ("BAD\t" + $tid + "\t" + ($r.id // "?")) end
        , ( if $kind == "test" and $file != "" and ($st == "green" or $st == "red")
            then "ROW\t" + $tid + "\t" + ($r.id // "?") + "\t" + $st + "\t" + $file
            else empty
            end )
      )
  ' "${THREADS[@]}" 2>/dev/null
)" || die "the canon extract could not be read as JSON" \
          "check \`jq . $CANON_DIR/*.json\`; a truncated extract must refuse, never report clean"

[ -n "$SCAN" ] || die "read ${#THREADS[@]} thread file(s) and extracted nothing" \
  "the extract's shape moved; this gate reads .tests[] with .kind/.status/.file"

total=0
bad=0
examined=0
found=0
FINDINGS=""

while IFS="$(printf '\t')" read -r kind a b c d; do
  case "$kind" in
    TOTAL) total=$((total + a)) ;;
    BAD)
      bad=$((bad + 1))
      echo "error: ${a}/${b}: an AT row carries no status field" >&2
      ;;
    ROW)
      examined=$((examined + 1))
      if [ ! -e "$d" ]; then
        found=$((found + 1))
        FINDINGS="${FINDINGS}absent: ${a}/${b} is ${c} and cites ${d} -- the file is NOT THERE
"
      fi
      ;;
  esac
done <<< "$SCAN"

# The grammar moved. Refuse rather than report: a parser reading nothing prints
# the same line as a clean run, which is the defect one level up from the one
# this exists to find.
if [ "$bad" -gt 0 ]; then
  die "${bad} of ${total} AT row(s) carry no \`status\` -- the row grammar moved" \
      "align this gate with the field names the AT model writes"
fi

[ "$total" -gt 0 ] \
  || die "read ${#THREADS[@]} thread file(s) holding zero AT rows" \
         "a canon extract with no acceptance tests is not something this gate can pass"

# **ZERO IN POPULATION IS A REFUSAL, NOT A PASS.** Every other outcome here is
# measured against rows that were actually examined; with none, the exit code
# would be reporting on the filter rather than on the estate.
[ "$examined" -gt 0 ] \
  || die "of ${total} AT row(s) across ${#THREADS[@]} thread(s), NONE is a green or red test row with a citation" \
         "either the estate genuinely has no verdict-claiming row yet -- in which case this gate has nothing to say and should not be in the runner -- or the filter no longer matches the model's field values"

if [ "$found" -eq 0 ]; then
  echo "ok: examined ${examined} green/red row(s) with a citation, of ${total} AT row(s) across ${#THREADS[@]} thread(s); none cites a missing file"
  exit 0
fi

printf '%s' "$FINDINGS"
echo ""
echo "examined ${examined} green/red row(s) with a citation, of ${total} AT row(s) across ${#THREADS[@]} thread(s)"
echo "note: a green row is counted as evidence by \`ac status\` and the close gate."
echo "      A row whose subject is gone is RETIRED, not left green -- withdraw the AC"
echo "      it covers (\`intent ac withdraw\`) and move the row to a non-test n/a with"
echo "      the reason in its prose, which is what the estate already does for a"
echo "      criterion whose subject expired."
echo "note: presence is not soundness. This says the file is THERE, never that its"
echo "      assertions can still fail -- see this script's header."
exit 1
