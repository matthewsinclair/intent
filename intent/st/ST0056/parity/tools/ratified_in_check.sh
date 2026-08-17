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
#
# ---------------------------------------------------------------------------
# AN ISSUE REFERENCE IS NOW RESOLVED AGAINST THE TREE INSTEAD OF BEING TRUSTED,
# AND THAT IS A CONSEQUENCE OF hv CLEARING THE TRACKER (2026-08-17, `9a9c7799`).
#
# The arm above was added because the corpus used it -- five rows spell their
# record `recorded on issue 0046`, and an issue was an external record by every
# argument the grammar rests on: durable, numbered, not editable into a
# self-citation. **hv then ruled the tracker out of existence** -- an issue is an
# EXTERNAL user's report against a RELEASED version, everything found building v3
# is work fixed inline with the reasoning in the commit -- and vc REMOVED 21 open
# rows. Nothing in this file changed, so it went on certifying five ratifications
# whose record had stopped existing. **The instrument was confidently wrong from
# `9a9c7799` until this commit**, and the sign is the dangerous one: it said yes.
#
# The class is the sibling of the tenth with the sign flipped. Not a population
# that silently GAINED members its consumer cannot serve, but one that silently
# LOST the thing its consumer points at -- and the consumer goes on saying yes,
# because nothing it reads has changed. **Removing a population is an edit to
# every citation of it, made by nobody, visible in no diff.**
#
# **THE BLANKET REVOCATION WOULD HAVE BEEN WRONG, AND ONLY MEASURING SHOWED IT.**
# `intent/issues/CLOSED/` was untouched at 40 rows. `at green` cites issue 0015
# and 0015 IS STILL ON DISK. Deleting the arm would have reported a correct
# citation as recordless and sent someone to "fix" a row that resolves today --
# the false-alarm direction, the expensive one, and the exact failure this file
# already records itself making once. So the test is not "does it say issue", it
# is **"does the number it names still resolve"**, which is a measurement of the
# tree rather than a policy about a word, and it self-maintains: prune CLOSED/
# tomorrow and the rows citing it turn amber on the next run with no edit here.
#
# Dangling refs are counted APART from recordless ones because the remedies are
# different jobs. A recordless row needs someone to find out what was decided; a
# dangling one has a record that is fully recoverable from git, and the report
# prints the command.
#
# ---------------------------------------------------------------------------
# THE hv EXEMPTION IS ANCHORED ON THE AUTHORITY, AND IT HAD TO BE, BECAUSE THE
# FIX ABOVE WOULD OTHERWISE HAVE ACTIVATED A LATENT LAUNDERING BUG.
#
# `AUTHORITY_HV` used to match `hv` ANYWHERE in the value. `ac gate` reads
# `vc ruling 2026-08-16 (provisional pending hv), recorded on issue 0032` -- vc
# is the authority and the word `hv` appears saying hv has NOT ruled yet. That
# never mattered while its issue counted as a record, because the row conformed
# before the exemption was consulted. **Revoke the record and the same row falls
# straight into the hv bucket, where the check would print a vc ruling pending hv
# as "an hv ruling, legal without a record" -- the precise laundering the record
# requirement exists to prevent, arriving through the fix for something else.**
# Anchored to the leading token now. Measured before and after: the hv bucket
# holds 0 rows either way, so nothing was reclassified to get this.
#
# --- MUTATION PROOFS (run 2026-08-17, ic; every prediction written first) -----
# Run from a `git archive` extract with this file overlaid, never the shared
# tree. Baseline in the extract: 26 declared, 10 conform, 0 hv, 5 dangling,
# 0 non-conforming, 11 sentinel.
#
# **AND THE FIRST RUN OF THEM FOUND THE CHECK BROKEN, WHICH IS THE ONLY REASON
# THESE READ AS PROOFS RATHER THAN AS DECORATION.** Predicted 5 dangling on the
# live table, observed 0 -- the `IFS=$'\t' read` pair-return described at
# `issue_nums` was silently moving every dangling number into the resolving
# variable. The fix for a false green shipped as a false green for about four
# minutes, and the run is what ended it. The predictions below were written
# before the code was, and are recorded against what the run actually printed.
#
# 1. TREE, NOT A BAKED LIST. `ISSUES_DIR` at a fabricated tree holding
#    `CLOSED/0046` + `CLOSED/0032`. PREDICTED: dangling 0. OBSERVED: 15 conform,
#    0 dangling -- the whole baseline restored. The answer comes from the
#    filesystem, not from a list of dead numbers this file remembers.
# 2. PER NUMBER, NOT PER SPELLING. Table copy with `st start`'s record changed
#    from `issue 0046` to `issue 0015`, which resolves under CLOSED/. PREDICTED:
#    that row conforms and the other four still dangle. OBSERVED: 11 conform,
#    4 dangling, and the four named are `st done`, `wp start`, `wp done`,
#    `ac gate`. So the check reads the NUMBER and resolves it.
# 3. THE OTHER DIRECTION, ON THE SAME MUTANT. Proof 2's table with `ISSUES_DIR`
#    at an EMPTY tree, so the 0015 that just passed must now fail. PREDICTED:
#    back to 5 dangling. OBSERVED: 10 conform, 5 dangling. One table, two trees,
#    two answers -- which is the property, and neither run alone shows it.
# 4. INABILITY TO MEASURE REFUSES. `ISSUES_DIR` at a non-existent path.
#    PREDICTED: exit 2, no verdict. OBSERVED: exit 2, stderr naming the row that
#    needed the tree (`st start`), nothing on stdout.
# 5. THE hv ANCHOR DISCRIMINATES. Anchor reverted to the old match-anywhere form
#    with the record fix in place. PREDICTED: `ac gate` laundered out of dangling
#    and into the hv bucket. OBSERVED: 4 dangling, 1 hv, and the report reads
#    `hv RULINGS -- legal without a record, and NOT a worklist item (1): ac gate`
#    for a ruling whose own text says vc made it and hv has not seen it.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="$(cd "$PARITY_DIR/.." && pwd)"
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"
# Overridable for the mutation proofs above; the check must be able to be told a
# different tree, or "it reads the tree" is an unrun claim.
ISSUES_DIR="${ISSUES_DIR:-$REPO_ROOT/intent/issues}"

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
#
# **ANCHORED ON THE LEADING TOKEN, NOT MATCHED ANYWHERE**, because the exemption
# turns on hv BEING the authority. `ac gate` names vc as its authority and says
# `(provisional pending hv)` -- hv has not ruled it at all -- and the unanchored
# needle read that as an hv ruling. See the header. Nothing conforms differently
# for it: measured, the hv bucket holds 0 rows under both needles.
AUTHORITY_HV='^[[:space:]]*hv([^a-z]|$)'
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
#
# **AND THE ISSUE ARM IS NOW A LOOKUP, NOT A MATCH** -- see the header. Naming an
# issue is the SPELLING; whether the number still resolves is the QUESTION, and
# only the tree answers it.
RECORD_SHA='(^|[^0-9a-zA-Z])[0-9a-f]{7,40}([^0-9a-zA-Z]|$)'
RECORD_FILE='[A-Za-z0-9_][A-Za-z0-9_.-]*\.(md|rs|sh|json|toml|txt)'
RECORD_ISSUE='issue[s]? [0-9]{3,4}'

# Every issue number that still resolves, read once from the tree. Both buckets:
# an issue in CLOSED/ is as durable a record as one in OPEN/ ever was, and more
# so now that OPEN/ is empty by ruling.
#
# Built even when the directory is absent, so the refusal below is about a row
# that NEEDS the tree rather than about the tree's mere absence -- a project with
# no issues and no issue citations has nothing to refuse over.
RESOLVABLE=""
if [ -d "$ISSUES_DIR" ]; then
  RESOLVABLE="$(find "$ISSUES_DIR" -mindepth 2 -maxdepth 2 -type d 2>/dev/null |
    sed 's|.*/||' | grep -Eo '^[0-9]{3,4}' | sort -u)"
fi

# The issue numbers a value names, and the two answers the tree gives about them.
#
# TWO FUNCTIONS RATHER THAN ONE RETURNING A PAIR, AND THAT IS NOT A STYLE CHOICE.
# The pair version returned `resolving<TAB>dangling` and was read back with
# `IFS=$'\t' read -r a b`. **Tab is an IFS WHITESPACE character**, so when the
# first field is empty -- which is the entire case this check exists to find --
# bash strips the leading tab instead of yielding an empty field, and the
# DANGLING number lands in the RESOLVING variable. Every dangling row then
# conformed and the check printed a clean green.
#
# Caught by running it: predicted 5 dangling, observed 0, and 0 was the number
# the broken code produced. **A false green inside the check whose whole subject
# is false greens** -- and the only reason it is not in the shipped file is that
# the mutation proofs were run rather than written. Command substitution strips
# trailing newlines and yields a genuinely empty string, so `-z` means what it
# says here.
issue_nums() { echo "$1" | grep -Eo "$RECORD_ISSUE" | grep -Eo '[0-9]{3,4}' | sort -u; }

issue_resolving() {
  local _n
  for _n in $(issue_nums "$1"); do
    echo "$RESOLVABLE" | grep -qx "$_n" && printf '%s ' "$_n"
  done
}

issue_dangling() {
  local _n
  for _n in $(issue_nums "$1"); do
    echo "$RESOLVABLE" | grep -qx "$_n" || printf '%s ' "$_n"
  done
}

OK=0; BAD=0; SENTINEL=0; HV=0; DANGLING=0
REPORT=""; SENTINEL_IDS=""; HV_IDS=""; DANGLING_REPORT=""

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
  dangling_nums=""
  echo "$value" | grep -Eq "$AUTHORITY" || missing="$missing authority"
  echo "$value" | grep -Eq "$ISO_DATE"  || missing="$missing date"

  # The issue arm resolves rather than matches. A value naming several issues
  # needs only ONE to resolve -- the grammar asks for AN external record, and a
  # row that leads somewhere leads somewhere.
  issue_res=""; issue_dang=""
  if echo "$value" | grep -Eq "$RECORD_ISSUE"; then
    [ -d "$ISSUES_DIR" ] || die "\`$id\` cites an issue as its external record and there is no issue tree at $ISSUES_DIR to resolve it against. Refusing: absent and unresolvable are different answers and this cannot tell them apart"
    issue_res="$(issue_resolving "$value")"
    issue_dang="$(issue_dangling "$value")"
  fi

  if ! echo "$value" | grep -Eq "$RECORD_SHA" &&
     ! echo "$value" | grep -Eq "$RECORD_FILE" &&
     [ -z "$issue_res" ]; then
    missing="$missing record"
    dangling_nums="$issue_dang"
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

  # **THE RECORD WAS WRITTEN AND THEN REMOVED OUT FROM UNDER IT** -- a different
  # job from a row that never had one, and fully recoverable, so it gets its own
  # bucket and its own instruction.
  #
  # AFTER the hv test on purpose. An hv ruling needs no record, so a dangling ref
  # on one is not a defect and reporting it would be the false-alarm direction --
  # the expensive one. No such row exists today; the order is set so that the day
  # one does, it reads as legal rather than as a worklist item.
  #
  # Only when the record is the ONLY thing absent: a row also missing its
  # authority or date needs more than a re-anchor, and belongs on the main
  # worklist where the whole list of absences prints.
  if [ "$missing" = " record" ] && [ -n "$dangling_nums" ]; then
    DANGLING=$((DANGLING + 1))
    short="$(echo "$value" | cut -c1-72)"
    DANGLING_REPORT="$DANGLING_REPORT  dangling: issue $dangling_nums -- $id -- \"$short...\"
"
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

printf 'ratified-in: %d unit(s) declare a ratification; %d conform (authority + date + record); %d are an hv ruling (legal, unverifiable by construction); %d cite a record that no longer resolves; %d do not conform; %d are the `parity.md` sentinel\n' \
  "$((OK + BAD + SENTINEL + HV + DANGLING))" "$OK" "$HV" "$DANGLING" "$BAD" "$SENTINEL"

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

if [ "$DANGLING" -gt 0 ]; then
  printf '\n  RECORD REMOVED OUT FROM UNDER THE CITATION -- a worklist item, and a RE-ANCHOR rather than an investigation (%d):\n' "$DANGLING"
  printf '%s' "$DANGLING_REPORT"
  printf '  -- These named an issue as their external record and the issue is no longer on disk. hv\n'
  printf '     ruled the tracker out (an issue is an EXTERNAL user report against a RELEASED version)\n'
  printf '     and vc removed 21 open rows at `9a9c7799`. **Nothing in the register was edited, so\n'
  printf '     the citations did not break loudly -- they went on being certified.**\n'
  printf '  -- Kept apart from the missing-record list because the remedy is different work. Nothing\n'
  printf '     needs deciding again: the content is recoverable and the commit that holds it is the\n'
  printf '     record to cite. `git log --oneline --all -- "intent/issues/*/<NNNN>*"` finds it, and\n'
  printf '     `git show 9a9c7799^:<path>` reads it back.\n'
  printf '  -- Re-anchor on the COMMIT that carries the ruling, not on a resurrected issue number.\n'
  printf '     A sha is the record with nothing in it to drop, which is why the grammar prefers it.\n'
  printf '  -- A number that STILL RESOLVES is not on this list. `intent/issues/CLOSED/` survives, so\n'
  printf '     citing a closed issue is a live record and reporting it here would be a false alarm.\n'
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

[ "$BAD" = "0" ] && [ "$DANGLING" = "0" ] &&
  printf '  no prose ratification is missing a component it is required to carry, and every external record it names still resolves.\n'
exit 0
