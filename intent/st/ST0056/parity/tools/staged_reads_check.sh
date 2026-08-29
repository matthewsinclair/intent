#!/usr/bin/env bash
#
# staged_reads_check.sh -- does every GATED instrument read its gating input
# from the INDEX rather than from the working tree?
#
# ==========================================================================
# WHY THIS EXISTS, AND WHY A LIBRARY WAS NOT ENOUGH
# ==========================================================================
#
# `lib_staged.sh` is the one home for reading a repo path out of the index, and
# four register checks converged on it under an hv Highlander ruling (issue
# 0125). **A sourced library closes the class only for the callers that source
# it, and nothing makes them.** That sentence is not a worry, it is a
# measurement: `lib_surface.sh` two files along was built as the one home, sat
# beside two callers that never sourced it, and so ADDED a home rather than
# reducing them -- its own header says so.
#
# So this is the thing that makes them. hv ruled 0125 and vc ruled the arm,
# both declining the alternative on the same ground: a header comment relies on
# every future instrument author reading it, which is precisely what failed.
#
# The defect it guards against, in one line: four sessions work one checkout,
# so an instrument that reads a repo path off DISK judges whatever a peer has
# half-typed and refuses commits over work its committer never touched. Found
# and fixed IN PLACE three times in eleven days by three different nodes before
# anybody fixed the class.
#
# ==========================================================================
# IT REPORTS. IT DOES NOT GATE. AND THAT IS NOT TIMIDITY.
# ==========================================================================
#
# **Its subject fails today** -- measured below, on the live roster -- so
# gating now would deliver a permanently-red gate, which is the exact reason
# `provenance_fields_check.sh` is rostered `manual` with a named release
# condition rather than `gated`. A guard that must be bypassed to commit is one
# step from a guard nobody keeps.
#
# **RELEASE CONDITION, so this does not sit report-only forever**: promote to
# refusing once every instrument it names is either converged onto
# `staged_copy` or carries an EXEMPTION line saying why its read is legitimate.
# Both are one line of work per instrument; what is not available is leaving
# them unclassified and calling the gate green.
#
# Exit 0 whether clean or with findings; non-zero ONLY when it cannot measure.
#
# ==========================================================================
# REACH, STATED BECAUSE THE NAME OVERSELLS IT
# ==========================================================================
#
# **It is a SYNTACTIC check and it does not know what a verdict rests on.** It
# finds assignments whose value reaches under the repo root and asks whether
# the file routes through `staged_copy`. So:
#
#   - it CANNOT tell a load-bearing read from an incidental one. An instrument
#     naming a path it never opens is reported, and that report is a request to
#     write the exemption, not an accusation.
#   - it CANNOT see a path built any other way -- a literal string, a `find`,
#     a `$(dirname)` dance. **The idiom it does see is the one all four
#     defective instruments used**, which is why it is worth having and also
#     exactly the limit to keep in view.
#   - it CANNOT see a DIRECTORY scan, because `git show :<dir>` does not exist
#     and `lib_staged.sh` therefore does not cover it either. `rulings_check.sh`
#     has two such scans and they are named in its own header.
#
# **A behavioural version would be strictly better and this is not one.** The
# honest form is to perturb each instrument's input unstaged and watch the
# verdict, the way issue 0125 was driven. A first attempt at that was made and
# is NOT reported here as evidence, because the STAGED control did not flip
# either: the perturbations were comment appends that no instrument asserts on,
# so the probe was inert and its greens measured nothing. That is recorded so
# nobody re-derives the same dead end.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${REPO_ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
ROSTER_SRC="${ROSTER_SRC:-$HERE/runner_roster_check.sh}"
# The directory whose instruments are examined. Overridable so the arms below
# can be driven against a PLANTED fixture -- an instrument proved only on
# today's tools is proved on the one input guaranteed not to move.
TOOLS="${TOOLS:-$HERE}"

die() { echo "error: $1" >&2; exit 2; }

[ -f "$ROSTER_SRC" ] || die "no roster to read the gated population from at $ROSTER_SRC"
[ -d "$TOOLS" ]      || die "no tools directory at $TOOLS"

# The gated population, read from the roster rather than from a second list.
# **A copy of this roster is the failure the roster itself exists to prevent**,
# so it is parsed, never transcribed.
GATED="$(awk '/^ROSTER=/{p=1;next} p && $2=="gated" {print $1}' "$ROSTER_SRC")"
[ -n "$GATED" ] || die "parsed 0 gated instruments from $ROSTER_SRC -- the ROSTER block's shape moved, and reporting no findings over an empty population is the loudest way to say nothing"

n_examined=0
n_flagged=0
findings=""

while IFS= read -r name; do
  [ -n "$name" ] || continue
  f="$TOOLS/$name"
  [ -f "$f" ] || continue
  n_examined=$((n_examined + 1))

  # An explicit exemption, one line, in the instrument itself. It must give a
  # REASON: a bare marker would let the class be silenced by a keystroke, which
  # is the same shape as the defect.
  if grep -qE '^# staged-reads: exempt --' "$f"; then
    continue
  fi

  # Assignments whose value reaches under the repo root.
  hits="$(grep -nE '^[A-Za-z_]+=.*\$\{?(REPO_)?ROOT\}?/' "$f" || true)"
  [ -n "$hits" ] || continue

  # Routed through the one home?
  if grep -q 'staged_copy' "$f"; then
    continue
  fi

  n_flagged=$((n_flagged + 1))
  findings="${findings}staged-reads: $name -- names $(printf '%s\n' "$hits" | grep -c .) repo path(s) and does not route any through \`staged_copy\`
"
  findings="${findings}$(printf '%s\n' "$hits" | sed 's/^/    /')
"
  findings="${findings}  remedy: source \`lib_staged.sh\` and stage the read, OR add one line
    \`# staged-reads: exempt -- <why this read cannot mislead a peer>\`
    (a directory scan, an untracked build artefact, and a path that is named
    but never opened are all legitimate reasons -- the exemption is the record
    that somebody decided, not a way to silence the check)
"
done <<< "$GATED"

if [ "$n_examined" -eq 0 ]; then
  die "examined 0 instruments -- every gated name in the roster is missing from $TOOLS, which is a broken measurement rather than a clean estate"
fi

if [ "$n_flagged" -eq 0 ]; then
  echo "ok: $n_examined gated instrument(s); every repo path they read is staged or exempt"
  exit 0
fi

printf '%s' "$findings"
echo "staged-reads: $n_flagged of $n_examined gated instrument(s) read a repo path from the WORKING TREE"
echo "  REPORTS, NEVER GATES -- see this file's header for the release condition"
exit 0
