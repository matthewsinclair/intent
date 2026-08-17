#!/usr/bin/env bash
# ratified_in_check.sh -- can a ratification be TRACED? (AC-05.5, the provenance axis.)
#
# GRAMMAR: authority + ISO date + external record. Ratified by vc 2026-08-17.
#
# THE DEFECT THIS EXISTS FOR, and it was found by asking a question about one row.
# ic asked whether two newly-`corrected` rows belonged in parity.md's class. vc
# read the register rather than the description of it, and found that
# `target.ratified_in` is free text which 21 rows spell FOUR ways:
#
#   authority + date          `hv 2026-08-15`
#   ruling + external record  `vc ruling 2026-08-17, recorded on issue 0046`
#   self-citation             `on this row's own voice ruling`
#   bare                      `vc ruling`
#
# The last two are the ones that fail. **The first is LEGAL when the authority is
# `hv` and only then** -- vc ruled 2026-08-17; see the HV bucket in the loop. **A ratification pointing at the row it
# ratifies is not independent**, by exactly the argument that a check is not
# independent when the same head wrote the needle and the checker -- the source
# has to be something other than the thing under test. And `vc ruling` names no
# date and no record at all, so there is nowhere to go to find out what was
# decided or when.
#
# WHY A SHA IS THE PREFERRED RECORD, and this is the part that decides the
# grammar rather than decorating it. `parity.md`, an issue number and a row are
# all editable by the node claiming the ratification, so each can decay into a
# self-citation without anyone touching this field. **A sha cannot: there is
# nothing in it to drop.** It also survives the failure this thread has hit
# repeatedly -- a citation losing its SUBJECT while being copied between
# documents that are each individually correct.
#
# BUT A SHA ALONE FAILS THE READER, which is why the grammar is all three and
# not just the record. `49db5aec` does not say who ruled or when, and a reader
# who has to run `git show` to learn the author of a ratification is being sent
# to the corpus for something the register exists to hold.
#
# IT REPORTS AND DOES NOT GATE, and that posture is deliberate rather than
# timid. Twenty-one rows in four spellings means most of the table reds on the
# first run, and **a guard that must be silenced to land is a guard nobody
# keeps** -- the same reason `corrected_check.sh` reports. It graduates to
# refusing once the table is clean; until then the report is the worklist.
#
# It refuses only on its own inability to measure, which is the one case where
# saying nothing would be worse than saying too much.
#
# KNOWN IMPRECISION, STATED RATHER THAN DISCOVERED. The external-record test
# accepts any 7-to-40-character run of `[0-9a-f]` as a sha, so an English word
# made only of hex letters (`defaced`, `deadbeef`) would satisfy it. That is a
# FALSE PASS, not a false alarm: the row slips through rather than being
# wrongly accused. Given the check reports rather than gates, a missed row costs
# a line in a worklist and a wrongly-accused row costs someone an argument, so
# the imprecision is pointed in the cheaper direction on purpose.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="$(cd "$PARITY_DIR/.." && pwd)"
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"

die() { echo "error: $1" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"
[ -f "$TABLE" ] || die "no dispatch table at $TABLE"

# Every unit carrying the field, as `<id>\t<value>`. Invariants and entries in
# one namespace for the same reason corrected_check.sh uses one: they are held
# to one grammar, so they are checked against one list.
#
# A tab separator, because the values are prose containing every other
# punctuation mark. `@tsv` would escape them; the ids are known not to contain
# tabs and the values are read as opaque text.
ROWS="$(jq -r '
  [ (.invariants[]                              | select(.target.ratified_in) | {id: .id,   v: .target.ratified_in}),
    ([.families[].entries[], .new_surface[]][]  | select(.target.ratified_in) | {id: .path, v: .target.ratified_in}) ]
  | .[] | "\(.id)\t\(.v)"' "$TABLE")"

[ -n "$ROWS" ] || die "no unit in the table declares \`target.ratified_in\`. With `corrected` rows present that is a schema change or a bad query, and reporting a clean sweep would be the loudest possible way to say nothing"

# --- the three components ----------------------------------------------------
# Declared as patterns rather than described in prose, so that what the check
# enforces and what this file claims it enforces are the same string.
AUTHORITY='(^|[^a-z])(ic|cc|vc|dc|hv)([^a-z]|$)'
ISO_DATE='[0-9]{4}-[0-9]{2}-[0-9]{2}'
# `hv` specifically, because an hv ruling is exempt from the record requirement
# and no other authority is. See the HV_UNVERIFIABLE bucket below.
AUTHORITY_HV='(^|[^a-z])hv([^a-z]|$)'
# A sha, a file (optionally with the `:line` suffix the table uses), or an issue.
#
# **THE ISSUE SPELLING WAS MISSING FROM THE FIRST VERSION AND THE CORPUS IS WHAT
# ADDED IT.** The ratified grammar named "a sha or a filename", which I proposed
# and vc ratified, and neither of us thought of `recorded on issue 0046` -- which
# FIVE rows use and which is an external record by every argument the grammar
# rests on: durable, numbered, not editable into a self-citation by the node
# claiming the ratification.
#
# Left in as a comment rather than quietly fixed, because it is the third firing
# today of a rule I wrote down myself: **a needle written from the author's head
# enumerates the spellings the author remembers, and the only honest enumerator
# of spellings is the corpus.** The first version's report named eleven rows as
# recordless; six actually are. **Five of the eleven would have sent someone to
# "fix" a row that was already correct** -- the false-alarm direction, which is
# the expensive one, and the run that produced the list is what made it visible.
RECORD_SHA='(^|[^0-9a-zA-Z])[0-9a-f]{7,40}([^0-9a-zA-Z]|$)'
RECORD_FILE='[A-Za-z0-9_][A-Za-z0-9_.-]*\.(md|rs|sh|json|toml|txt)'
RECORD_ISSUE='issue[s]? [0-9]{3,4}'

OK=0; BAD=0; SENTINEL=0; HV=0
REPORT=""; SENTINEL_IDS=""; HV_IDS=""

while IFS=$'\t' read -r id value; do
  [ -n "$id" ] || continue

  # **THE FIELD IS DOING TWO JOBS AND ONLY ONE OF THEM IS PROSE.** For the units
  # parity.md cites, `ratified_in` is the exact string `parity.md`, and
  # `corrected_check.sh` matches it with EQUALITY. So those values are an ENUM,
  # not a provenance stamp.
  #
  # **THE FIRST VERSION OF THIS COMMENT CLAIMED REWRITING A SENTINEL BREAKS THE
  # SIBLING CHECK, AND A MUTATION DISPROVED IT.** Rewriting `st`'s `parity.md`
  # to `vc, 2026-08-15, parity.md` changes `corrected_check.sh`'s output by
  # NOTHING -- byte-identical, verified. The reason is in its algorithm rather
  # than in its field access: `ratified_in` is consulted only to split rows that
  # are CLAIMED but NOT CITED into UNCITED vs ELSEWHERE. A cited row never
  # reaches that split, so its stamp is inert.
  #
  # **THE REAL RISK IS LATENT, AND IT IS THE MORE INTERESTING ONE.** The rewrite
  # is provably harmless today and disarms an alarm for tomorrow: the day
  # parity.md stops citing that unit, a rewritten stamp classifies it ELSEWHERE
  # ("ratified somewhere else, not our business") instead of UNCITED ("asserts a
  # ratification the ratifying document does not make") -- and UNCITED is the
  # alarm. **A change with no observable effect, whose whole effect is on which
  # alarm fires after an unrelated later edit.**
  #
  # Counted separately rather than passed or failed, because neither is true:
  # they are outside this grammar until someone rules, and silently exempting
  # them would hide a real design question in a green.
  #
  # Recorded at this length because the wrong version was written first, from
  # reading the sibling's code, and only the mutation found it. **In a file whose
  # subject is claims nothing reads back, shipping an unrun claim would have been
  # the joke writing itself.**
  if [ "$value" = "parity.md" ]; then
    SENTINEL=$((SENTINEL + 1))
    SENTINEL_IDS="$SENTINEL_IDS $id"
    continue
  fi

  missing=""
  echo "$value" | grep -Eq "$AUTHORITY" || missing="$missing authority"
  echo "$value" | grep -Eq "$ISO_DATE"  || missing="$missing date"
  if ! echo "$value" | grep -Eq "$RECORD_SHA" &&
     ! echo "$value" | grep -Eq "$RECORD_FILE" &&
     ! echo "$value" | grep -Eq "$RECORD_ISSUE"; then
    missing="$missing record"
  fi

  # **AN hv RULING IS LEGAL WITHOUT A RECORD, AND COUNTED APART** -- vc ruled
  # 2026-08-17, on the same split they gave `agents template`: rule what is
  # decidable, send the process question up.
  #
  # The record requirement exists to stop a node laundering its OWN ruling. When
  # the authority is `hv`, the node writing the stamp is not the authority it
  # names and hv can contradict it, **so the independence property already holds
  # and a record adds nothing to it.**
  #
  # But a node can write `hv 2026-08-15` for a ruling that was never given, and
  # nothing here or anywhere can check that -- **the whiteboard-timestamp problem
  # exactly, where a fabricated stamp is indistinguishable from a real one by
  # inspection.** So it is legal, and it is reported on its own line, so the
  # number of ratifications that are UNVERIFIABLE BY CONSTRUCTION stays visible
  # instead of being absorbed into "conforms". Do not refuse it, do not launder
  # it, report the split -- the same posture as the ELSEWHERE bucket in
  # `corrected_check.sh`.
  #
  # Open with hv (vc, 2026-08-17): whether a ruling that ratifies a row should be
  # COMMITTED when it is given, which would convert the weakest evidence into a
  # sha. Nothing waits on it; these conform today.
  if [ "$missing" = " record" ] && echo "$value" | grep -Eq "$AUTHORITY_HV"; then
    HV=$((HV + 1))
    HV_IDS="$HV_IDS $id"
    continue
  fi

  if [ -z "$missing" ]; then
    OK=$((OK + 1))
  else
    BAD=$((BAD + 1))
    # Truncated to one line: the values run to paragraphs and the worklist is
    # about WHICH field is absent, not about re-reading the ratification.
    short="$(echo "$value" | cut -c1-72)"
    REPORT="$REPORT  missing:$missing -- $id -- \"$short...\"
"
  fi
done <<< "$ROWS"

printf 'ratified-in: %d unit(s) declare a ratification; %d conform (authority + date + record); %d are an hv ruling (legal, unverifiable by construction); %d do not; %d are the `parity.md` sentinel\n' \
  "$((OK + BAD + SENTINEL + HV))" "$OK" "$HV" "$BAD" "$SENTINEL"

if [ "$SENTINEL" -gt 0 ]; then
  printf '\n  SENTINEL, and NOT a worklist item -- these carry the exact string `parity.md`, which\n'
  printf '  `corrected_check.sh` matches by EQUALITY to decide its scope (%d):%s\n' \
    "$SENTINEL" "$SENTINEL_IDS"
  printf '  -- **ONE FIELD IS DOING TWO JOBS.** For these units `ratified_in` is an ENUM; for the\n'
  printf '     rest it is a prose provenance stamp. Rewriting one to satisfy the grammar changes\n'
  printf '     corrected_check.sh output by NOTHING TODAY (mutation-verified, byte-identical) --\n'
  printf '     it consults the field only for rows CLAIMED but NOT CITED, which a cited row never\n'
  printf '     reaches. The cost is LATENT: the day parity.md stops citing that unit, a rewritten\n'
  printf '     stamp reads as ELSEWHERE instead of UNCITED, and UNCITED is the alarm.\n'
  printf '  -- Left for a ruling rather than exempted quietly: hiding a design question inside a\n'
  printf '     green is how the register got its other defects.\n'
fi

if [ "$HV" -gt 0 ]; then
  printf '\n  hv RULINGS -- legal without a record, and NOT a worklist item (%d):%s\n' \
    "$HV" "$HV_IDS"
  printf '  -- The record requirement stops a node laundering its OWN ruling. With `hv` as the\n'
  printf '     authority the node writing the stamp is not the authority it names, so independence\n'
  printf '     already holds and a record adds nothing to it.\n'
  printf '  -- Counted apart because a node CAN write `hv <date>` for a ruling never given, and\n'
  printf '     nothing can check that -- the whiteboard-timestamp problem, where a fabricated stamp\n'
  printf '     is indistinguishable from a real one by inspection. **This line is the count of\n'
  printf '     ratifications that are unverifiable BY CONSTRUCTION**, kept visible rather than\n'
  printf '     absorbed into the conforming total.\n'
fi

if [ "$BAD" -gt 0 ]; then
  printf '\n%s' "$REPORT"
  printf '  -- the grammar is AUTHORITY (a node moniker or `hv`) + an ISO DATE + an EXTERNAL RECORD\n'
  printf '     (a commit sha, or a file). A ratification that cites the row it ratifies is not\n'
  printf '     independent, and one with no record at all cannot be checked by anyone.\n'
  printf '  -- REPORTING, not gating, until the table is clean. This list is the worklist.\n'
  printf '  -- Never invent a record to clear a line here. A plausible citation written to make\n'
  printf '     this list shorter is the fabrication the grammar exists to prevent, arriving\n'
  printf '     through the worklist.\n'
fi

[ "$BAD" = "0" ] && printf '  no prose ratification is missing a component it is required to carry.\n'
exit 0
