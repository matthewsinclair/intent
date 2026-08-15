#!/usr/bin/env bash
# corrected_check.sh -- does parity.md's ratified `Corrected` class agree with
# what the dispatch table CLAIMS is corrected? (AC-05.5, decision drift.)
#
# AT-05.5 -- this file is that row's cited evidence, and the id is here because
# L3 requires the link to be checkable from BOTH ends. It was missing until vc
# moved the row off `to-write`: the row had cited a file that never existed
# (`decision_drift_check.sh`), and L2/L3 are gated on `green|red` for the good
# reason that a missing file is the CORRECT state for a test not yet written.
# So the row's own staleness was what kept the lint that would have caught it
# switched off -- `to-write` is the one status nothing validates.
#
# THE DEFECT THIS EXISTS FOR, and it is measured rather than hypothetical.
# `parity.md` carried hv's 2026-08-14 ratification of the `--help` census and the
# stderr/stdout census. The dispatch table went on marking INV-07 and INV-06
# `pending-hv` for a DAY. **Both files were individually correct and neither was
# readable as wrong**: the register said "waiting on hv", the ratification said
# "decided", and no reader of either one alone could see it. It was found by
# hand, by someone counting the hv queue and wondering whether it was overstated.
#
# `surface_check.sh` covers the other axis -- table against BINARY -- and is
# structurally blind to this one, because both artefacts here are prose-and-JSON
# and neither is the binary.
#
# THE CHECK IS SET EQUALITY, IN BOTH DIRECTIONS, and the second direction is the
# point. The obvious direction is "a row claims `corrected` and nothing ratifies
# it". The direction that would actually have caught INV-06 and INV-07 is the
# REVERSE -- **a ratification that nothing has applied** -- and it is the one a
# check written from the memory of the defect would most likely have missed,
# because the defect presented as rows being stale rather than as a ratification
# being unclaimed.
#
# CITATION FORMAT: ids, not locations, adopted by vc 2026-08-15 on ic's proposal
# and settled by an accident an hour later -- a citation reading `acceptance.md:298`
# had rotted to `:302` because rows were added above it, silently, with the
# number still a number pointing at the wrong sentence. **Locations decay under
# exactly the activity this contract is under.** The grammar is the one the AT
# row already uses, so the contract has ONE rule for "a machine-read list at the
# end of an authored line" rather than two: spaced `--` separator, comma-separated
# ids, nothing fused.
#
# It REPORTS and does not gate, matching `surface_check.sh` and for the reason
# vc adopted: most of the surface is unwired mid-ladder, and a gate here would
# block every node on work that has not started. It refuses only on its own
# inability to measure.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="$(cd "$PARITY_DIR/.." && pwd)"
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"
PARITY="${PARITY:-$ST_DIR/parity.md}"

die() { echo "error: $1" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"
[ -f "$TABLE" ]  || die "no dispatch table at $TABLE"
[ -f "$PARITY" ] || die "no parity contract at $PARITY"

# --- what parity.md RATIFIES ------------------------------------------------
# Anchored on the `covers:` clause rather than on the surrounding prose, so
# rewording a member does not silently empty the set. A member line that loses
# its clause disappears from CITED and is caught by the reverse direction below,
# which is the correct outcome: an uncited ratification ratifies nothing
# mechanically.
#
# The pattern anchors on the LAST `-- covers:` in the line and takes the rest.
# The first version excluded hyphens from the id list (`[^-]*$`) and therefore
# matched nothing at all, because every invariant id contains one -- `INV-08`.
# It was the refusal below that caught it rather than a silent empty set, which
# is the whole argument for refusing on an unmeasurable input.
CITED="$(grep -E -- '-- covers:' "$PARITY" \
  | sed 's/.*-- covers://' \
  | tr ',' '\n' | sed 's/^[[:space:]]*//; s/[[:space:]]*$//' \
  | grep -v '^$' | sort -u)"

[ -n "$CITED" ] || die "parity.md's \`Corrected\` class cites nothing -- no \`-- covers: <ids>\` clause was found. Refusing rather than reporting every corrected row as undrafted: an empty citation set makes the forward direction fire on everything and the reverse direction vacuously pass, which is the loudest possible way to say nothing."

# --- what the TABLE claims --------------------------------------------------
# Invariants and command entries in one namespace on purpose: they are cited in
# one list because they were ratified by one sentence, so they must be checked
# against one list. Splitting them would need a rule for which namespace an id
# belongs to, and the citation grammar deliberately has no such marker.
CLAIMED="$(jq -r '
  [ (.invariants[] | select(.target.state == "corrected") | {id: .id, rat: (.target.ratification // "")}),
    ([.families[].entries[], .new_surface[]][] | select(.target.state == "corrected") | {id: .path, rat: (.target.ratification // .target.basis // "")}) ]
  | .[] | .id' "$TABLE" | sort -u)"

[ -n "$CLAIMED" ] || die "the table claims nothing is \`corrected\` -- with a non-empty citation list that is either a schema change or a bad query, and reporting every citation as unapplied would be the wrong answer to it"

# --- the two directions -----------------------------------------------------
# REVERSE first, because it is the one that matters: parity.md ratifies a unit
# and nothing claims it. THIS is the INV-06 / INV-07 direction -- the decision
# was made and the register never moved.
UNAPPLIED="$(comm -13 <(printf '%s\n' "$CLAIMED") <(printf '%s\n' "$CITED"))"

# FORWARD: a unit claims `corrected` and parity.md does not cite it. That is not
# automatically wrong -- a unit can be ratified by an hv ruling on another date,
# by vc, or by a design decision, and those are legitimately not parity.md's to
# cite. So it is SPLIT by whether the unit's own ratification points back here.
#
# **CITATION DECIDES SCOPE, NOT PROSE, AND THAT IS A CORRECTION TO THIS FILE.**
# The first version derived scope from the ratification text alone, matching
# `parity.md` literally. It measured 8 of 11 cited units as in-scope and
# reported `INV-08`, `info` and `version` as ratified ELSEWHERE -- while
# parity.md was citing all three on its unknown-flag member. Their ratifications
# say "the `corrected` class" without naming the file. **A check whose scope
# depends on how a sentence happens to be phrased is the same defect it was
# written to catch, one level up**, and it appeared on the first run.
#
# Now nothing cited can fall out of scope by wording, and the prose test only
# chooses the SEVERITY of an uncited claim rather than whether it is looked at.
NOT_CITED="$(comm -23 <(printf '%s\n' "$CLAIMED") <(printf '%s\n' "$CITED"))"
POINTS_HERE="$(jq -r '
  [ (.invariants[] | select(.target.state == "corrected") | select((.target.ratification // "") | test("parity\\.md|`?[Cc]orrected`? class")) | .id),
    ([.families[].entries[], .new_surface[]][] | select(.target.state == "corrected") | select(((.target.ratification // .target.basis // "")) | test("parity\\.md|`?[Cc]orrected`? class")) | .path) ]
  | .[]' "$TABLE" | sort -u)"

UNCITED="$(comm -12 <(printf '%s\n' "$NOT_CITED") <(printf '%s\n' "$POINTS_HERE"))"
ELSEWHERE="$(comm -23 <(printf '%s\n' "$NOT_CITED") <(printf '%s\n' "$POINTS_HERE"))"
VIA_PARITY="$(comm -12 <(printf '%s\n' "$CLAIMED") <(printf '%s\n' "$CITED"))"

# --- report -----------------------------------------------------------------
printf 'corrected: parity.md cites %d unit(s); the table claims %d; %d are both (cited and claimed)\n' \
  "$(printf '%s\n' "$CITED" | grep -c .)" \
  "$(printf '%s\n' "$CLAIMED" | grep -c .)" \
  "$(printf '%s\n' "$VIA_PARITY" | grep -c .)"

if [ -n "$ELSEWHERE" ]; then
  printf '  ratified ELSEWHERE, out of this check'\''s scope (%d) -- each carries its own ratification and is not parity.md'\''s to cite: %s\n' \
    "$(printf '%s\n' "$ELSEWHERE" | grep -c .)" "$(printf '%s' "$ELSEWHERE" | tr '\n' ' ')"
fi

RC=0
if [ -n "$UNCITED" ]; then
  RC=1
  printf '\ncorrected: units name parity.md as their ratification, and parity.md does NOT cite them:\n'
  printf '%s\n' "$UNCITED" | sed 's/^/  UNCITED    /'
  printf '  -- the row asserts a ratification the ratifying document does not make. Fix the row, or add the id to the member'\''s `covers:` clause.\n'
fi

if [ -n "$UNAPPLIED" ]; then
  RC=1
  printf '\ncorrected: parity.md RATIFIES units that nothing claims:\n'
  printf '%s\n' "$UNAPPLIED" | sed 's/^/  UNAPPLIED  /'
  printf '  -- THIS IS THE DIRECTION THAT HID INV-06 AND INV-07 FOR A DAY. The decision was made and the register never moved; both files read correctly on their own.\n'
fi

[ "$RC" = "0" ] && printf '  the ratified set and the claimed set agree exactly.\n'
exit 0
