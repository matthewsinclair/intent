#!/usr/bin/env bash
# class_vocab_check.sh -- is every state that CLAIMS to be a parity class
# actually a class `parity.md` names? (AC-05.5, the vocabulary axis.)
#
# WHY THIS IS A SIBLING OF `corrected_check.sh` AND NOT A SECOND COPY OF IT.
# That check compares MEMBERSHIP within a class both documents already know
# about: which units `parity.md`'s `Corrected` section cites, against which
# units the table claims are `corrected`. **It is structurally blind to a class
# one document has and the other lacks** -- it can only compare the contents of
# a vocabulary, and nothing compared the vocabulary. vc named this on 2026-08-15
# as the same shape as the scope-from-prose defect in `corrected_check.sh`, one
# level up, and found it the way AC-05.5 predicts these are found: by accident,
# while verifying the mechanism built to stop finding them by accident.
#
# THE LIVE INSTANCE. `intent config` carries `target.state: undefined`, ratified
# "vc ruling, 2026-08-14 -- **the fifth parity class**, opened on this entry".
# The word `undefined` does not occur anywhere in `parity.md` -- whose own class
# list is introduced as "decided here, never discovered in triage". **The
# contract claiming sole authorship of the class list does not carry the fifth
# class**, and `corrected_check.sh` reports `agree exactly` while that is true.
#
# WHY `is_parity_class` IS DECLARED AND NOT DERIVED, which is AC-09.1's lesson
# applied one level up. The tempting rule is "a state that asserts a deviation
# must be grounded in parity.md" -- and it is wrong, measurably. `retire` asserts
# the largest deviation there is (the command does not ship) and belongs in NO
# class: all six retire rows carry their OWN ratification (hv rulings for
# organize / treeindex / st_zero, D09 for upgrade, AC-05.1 for help), so class
# membership is not their warrant and `parity.md` is not where it lives. The
# distinguishing fact is what a state CLAIMS about itself, which is judgement --
# so it is declared per state in `target_states`, where a human can be held to
# it, rather than inferred from a property that gets it wrong.
#
# It REPORTS and does not gate, matching its siblings, and refuses only on its
# own inability to measure.
# MUTATION PROOFS, run 2026-08-17 at `67814555` (ic). Co-located because a check
# whose failure path has never fired is a claim, not an instrument -- and this one
# is GATED into the pre-commit runner now, so it prints a verdict on every commit
# by four nodes. Re-run with the `TABLE=` / `PARITY=` overrides this file already
# reads; nothing else is needed.
#
#   control                          -> exit 0, "every claimed parity class is named"
#   a state claiming a class parity.md does not name
#                                    -> exit 0, reports `UNGROUNDED  zz-invented`
#   no state claims to be a parity class
#                                    -> exit 2 (line 52), never "all classes unused"
#   the class-block HEADING renamed  -> exit 2 (line 67), never "all claims ungrounded"
#   heading kept, bullet grammar broken
#                                    -> exit 2 (line 74)
#
# **The heading mutation is the one worth having.** Without that refusal a renamed
# heading empties `NAMED`, and every correctly-grounded class is then reported as
# ungrounded -- a wall of confident findings produced by reading nothing, pointing
# at the one file that is not wrong. The three empty-population guards were written
# before any of them had been fired; now they have been.
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

# --- what the table declares a parity class ----------------------------------
CLAIMED="$(jq -r '(.target_states // [])[] | select(.is_parity_class == true) | .state' "$TABLE" | sort -u)"
[ -n "$CLAIMED" ] || die "\`target_states\` declares no state a parity class -- with a populated vocabulary that is a schema change or a bad query, and reporting every parity.md class as unused would be the wrong answer to it"

# --- what parity.md NAMES as a class -----------------------------------------
# Scoped to the sub-bullets under the ratified-deviation-class heading, NOT to
# the whole file, and that is deliberate: `retire` occurs in `parity.md` inside
# the register's column grammar (`keep · retire · deviate`), so a whole-file
# match would report it grounded as a CLASS on the strength of a table header.
# **Deciding scope by where a word happens to appear is the defect this
# directory has already shipped once** -- corrected_check.sh matched the literal
# string `parity.md` in a ratification and put three cited units out of scope.
CLASS_BLOCK="$(awk '
  /^- \*\*Ratified deviation classes/ { inblock = 1; next }
  inblock && /^- / { inblock = 0 }
  inblock { print }
' "$PARITY")"
[ -n "$CLASS_BLOCK" ] || die "found no ratified-deviation-class block in $PARITY -- the heading \`- **Ratified deviation classes\` did not match, so the class list could not be read. Refusing rather than reporting every claimed class as ungrounded, which is what an empty class list would produce."

NAMED="$(printf '%s\n' "$CLASS_BLOCK" \
  | sed -n 's/^[[:space:]]*-[[:space:]]*\*\*\([^*]*\)\*\*.*/\1/p' \
  | tr '[:upper:]' '[:lower:]' \
  | sed 's/[[:space:]]*$//' \
  | grep -v '^$' | sort -u)"
[ -n "$NAMED" ] || die "the ratified-deviation-class block in $PARITY names no classes -- the bullet grammar (\`  - **Name** -- ...\`) did not match anything inside it"

# --- the comparison -----------------------------------------------------------
UNGROUNDED="$(comm -23 <(printf '%s\n' "$CLAIMED") <(printf '%s\n' "$NAMED"))"
GROUNDED="$(comm -12 <(printf '%s\n' "$CLAIMED") <(printf '%s\n' "$NAMED"))"

printf 'class-vocab: the table declares %d state(s) a parity class; parity.md names %d class(es); %d grounded\n' \
  "$(printf '%s\n' "$CLAIMED"  | grep -c .)" \
  "$(printf '%s\n' "$NAMED"    | grep -c .)" \
  "$(printf '%s\n' "$GROUNDED" | grep -c .)"

if [ -n "$UNGROUNDED" ]; then
  printf '\nclass-vocab: states CLAIM to be a parity class and parity.md does not name one:\n'
  printf '%s\n' "$UNGROUNDED" | sed 's/^/  UNGROUNDED  /'
  printf '  -- the contract whose class list is introduced as "decided here, never discovered in triage" does not carry the class. Add the class to parity.md, or set `is_parity_class: false` and record where the warrant actually lives (which is what `retire` does).\n'
  exit 0
fi
printf '  every claimed parity class is named in parity.md.\n'
exit 0
