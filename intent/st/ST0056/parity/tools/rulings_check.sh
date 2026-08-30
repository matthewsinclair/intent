#!/usr/bin/env bash
# rulings_check.sh -- can a ratification be TRACED? (AC-05.5, the provenance axis.)
#
# **RENAMED FROM `ratified_in_check.sh` ON 2026-08-23, AND THE OLD NAME IS WHY.**
# The field it checks became `target.rulings` on 2026-08-22; the file went on
# naming `ratified_in`, a field that no longer exists anywhere in the tree.
# **A filename is a recorded reason like any other, and this is the one place
# nobody thinks to check for one that has outlived its subject** -- it is not
# in a document anyone re-reads, it is in the path every caller types.
#
# **PROSE THAT NAMES THE OLD FILE IS NOT ALL THE SAME AND WAS NOT SWEPT.** A
# sentence describing what this file DID under its old name was true when
# written and stays; a sentence describing what it DOES was updated. The
# distinction is the estate's own: a live claim is corrected, a record is
# preserved and marked. Historical mentions carry `(now `rulings_check.sh`)` on
# first occurrence rather than being rewritten, which is the same treatment the
# `intentdb` retirement used -- corrected in brackets, never silently.
#
# CONTRACT, FROM 2026-08-22: every unit that claims a ratification declares it in
# `target.rulings`, an ARRAY of `{state, authority, date, record}`.  This file READS
# THOSE FIELDS.  It parses no prose, and that is the whole of the change.
#
# ==========================================================================
# WHAT THIS REPLACED, AND WHY IT IS WORTH THE MIGRATION
# ==========================================================================
#
# Until 2026-08-22 the ratification lived in `target.ratified_in`, a free-text
# stamp, and THIS FILE RECOVERED ITS MEANING WITH REGEXES: `AUTHORITY`,
# `ISO_DATE`, `RECORD_SHA|RECORD_FILE|RECORD_ISSUE`, and a `missing` string
# assembled from whichever failed to match.
#
# **THE DEFECT WAS NOT THAT THE REGEXES WERE WRONG.  IT WAS THE SHAPE.** The
# field carried ONE authored declaration while this checker DERIVED FOUR MORE
# VERDICTS out of English -- conform, hv-unverifiable, dangling, non-conforming
# -- so the register held one answer and the parser invented four, free to
# disagree, with nothing able to say which was right.  vc ruled the wide fix on
# 2026-08-22 under hv's pen; the argument that decided it is that the NARROW fix
# (add a state enum, leave the prose) is not a smaller change but A NEW INSTANCE
# OF THE SAME DEFECT wearing the remedy's clothes.
#
# **AND THE FIELD COULD NOT EXPRESS A STATE THAT WAS IN USE.** A provisional
# ruling IS recorded and IS NOT ratified; `ratified_in` could say "here is the
# record" or nothing, so both available values were wrong and the one real
# instance (`ac gate`) smuggled the word `provisional` into its prose, where this
# checker found it with a keyword grep.  **A checker verifies membership in a
# vocabulary and never that the vocabulary can express the states in use**
# (AC-00.13) -- minted from this field, and this field is where it was earned.
#
# **TWO VOCABULARIES NOW, AND THE SPLIT IS THE POINT.** The AUTHORED enum has
# exactly the members the corpus holds -- `ratified` and `provisional`.  The
# VERDICT vocabulary is this checker's OUTPUT and carries six, including
# `dangling` and `non-conforming`, which are EMPTY TODAY.  They are in the
# verdict set and NOT in the authored set on purpose: an author must never be
# able to declare `ratified` while this checker computes `dangling`, because that
# is two answers to one question with nothing between them -- the defect the
# migration exists to remove.  An enum built from a green corpus cannot express
# the states the corpus enters when it goes red, and the pre-migration header
# recorded `dangling` at 4 within the week.
#
# **THE `parity.md` SENTINEL IS GONE, AND IT WAS NEVER A STATE.** Eleven units
# carried the exact string `parity.md` in a field whose other 16 values were
# prose stamps -- one field, two jobs, and the sentinel half also answered
# `corrected_check.sh`'s SCOPE question.  It is a RECORD: hv ratified the
# `Corrected` class wholesale at the 2026-08-14 bounce and parity.md is where
# that ruling is written down.  So it migrates to
# `{ratified, hv, 2026-08-14, "parity.md"}` -- no special case, no new field --
# and scope becomes COMPUTED (`record == "parity.md"`) rather than separately
# declared.  A separate scope field would have been a SECOND record of what
# parity.md already says, and those two drifted to 8-of-11 once already.
#
# **THE FULL 386 LINES OF THE PRE-MIGRATION FILE ARE NOT REPRODUCED HERE.** They
# record how the regex grammar was tuned -- the `AUTHORITY_HV` anchoring bug that
# laundered `ac gate` into the hv bucket, the tab-in-IFS false green, the six
# mutation proofs -- and every one of those is a fact about a mechanism that no
# longer exists.  They are in git, on the commit before this one, which is this
# estate's own rule: a reader running `git show` must land on the ruling and not
# on a reference to it.  **Two findings survive the mechanism and are kept here
# because they are about the SUBJECT rather than the parser:**
#
#   -- A ratification pointing at the row it ratifies is NOT INDEPENDENT.  The
#      source has to be something other than the thing under test.  `record` is
#      an external record for exactly this reason.
#   -- A SHA IS THE PREFERRED RECORD because `parity.md`, an issue number and a
#      row are all editable by the node claiming the ratification, so each can
#      decay into a self-citation with nobody touching the field.  A sha cannot:
#      there is nothing in it to drop.
#
# ==========================================================================

set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="${ST_DIR:-$(cd "$PARITY_DIR/.." && pwd)}"
# OVERRIDABLE FOR THE SAME REASON `TABLE` IS.  A check whose anchors can only be
# derived from its own location cannot be driven against a planted corpus, and a
# migration proved only on today's data is proved on the one input guaranteed to
# be green.  These exist so the mutation cases below can run at all.
REPO_ROOT="${REPO_ROOT:-$(cd "$ST_DIR/../../.." && pwd)}"
# **THE GATING INPUTS ARE READ FROM THE INDEX, NEVER FROM THE WORKING TREE.**
#
# This runs from the pre-commit gate, and four sessions work one checkout, so
# reading `surface/dispatch-table.json` off disk means judging whatever every
# other node happens to have half-typed. Measured 2026-08-28: cc s mid-edit
# `st list` row refused vc s commit, on a file vc had never touched (issue
# 0125). `residue_class_check.sh` had already carried the cure since
# 2026-08-17 and nothing brought it here.
#
# hv ruled the convergence rather than three separate patches, on Highlander
# grounds. The mechanism, the four episodes behind it and its reach limit are
# in `lib_staged.sh`.
# shellcheck source=lib_staged.sh
. "$HERE/lib_staged.sh"
trap staged_cleanup EXIT

TABLE_GIVEN="${TABLE:+yes}"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"
# **NOT STAGED, AND THE LIMIT IS REAL RATHER THAN AN OVERSIGHT.** These two are
# DIRECTORY SCANS, and `git show :<dir>` does not exist -- so the issue estate
# below is still read from the working tree and a peer s mid-work issue file is
# still visible to this check. It matters less than the table did (the question
# asked of them is whether a cited issue number RESOLVES, not what it says), and
# it is not nothing. A directory population needs `git ls-files`, a different
# mechanism; inventing one unasked inside the convergence that exists to stop
# there being two would be the joke telling itself.
ISSUES_DIR="${ISSUES_DIR:-$REPO_ROOT/intent/issues}"
ISSUES_CANON="${ISSUES_CANON:-$REPO_ROOT/intent/.canon/issues}"

die() { echo "error: $1" >&2; exit 2; }

[ -f "$TABLE" ] || die "no dispatch table at $TABLE"
command -v jq >/dev/null 2>&1 || die "jq is required"

# `|| exit 2` is load-bearing rather than defensive noise: `staged_copy` refuses
# by exiting, and it runs in a command substitution, so that exit ends the
# SUBSHELL. This file does not set `-e`, so without the `|| exit 2` a refusal
# becomes a read of `""` and a confident green over nothing.
TABLE="$(staged_copy "$TABLE_GIVEN" "$TABLE")" || exit 2

# THE AUTHORED VOCABULARIES.  Both are closed sets, and a value outside either is
# NON-CONFORMING rather than ignored -- a checker that skips what it does not
# recognise reports a green over the rows it could not read.
STATES="ratified provisional"
AUTHORITIES="ic cc vc dc hv"

# One line PER RULING, not per unit: a unit may carry more than one (the corpus
# has ic ruling a class and hv ruling its remedy on one row).  A per-unit read
# would have to pick one and would silently drop the other.
#
# A null record is emitted as an EMPTY FIELD.  No sentinel string, because a
# sentinel is a value a table could legitimately contain.
ROWS="$(jq -r '
  [ (.invariants[]                             | select(.target.rulings != null) | {id: (.id // .rule // "INV"), r: .target.rulings}),
    ([.families[].entries[], .new_surface[]][] | select(.target.rulings != null) | {id: (.path // .v2 // "?"),  r: .target.rulings}) ]
  | .[] as $u | $u.r[] as $g
  | [$u.id, ($g.state // "<none>"), ($g.authority // "<none>"), ($g.date // "<none>"), ($g.record // "")] | @tsv
' "$TABLE")" || die "the table did not parse"

[ -n "$ROWS" ] || die "no unit in the table declares \`target.rulings\` -- this check would report a vacuous green over an empty set, so it refuses instead"

# Issue numbers the tree can still resolve.  CLOSED counts: an issue in CLOSED/ is
# as durable a record as one in OPEN/ ever was, and more so now OPEN/ is empty by
# ruling.
RESOLVABLE=""
if [ -d "$ISSUES_CANON" ]; then
  RESOLVABLE="$(find "$ISSUES_CANON" -maxdepth 1 -type f -name '*.json' 2>/dev/null |
    sed 's|.*/||' | grep -Eo '^[0-9]{3,4}' | sort -u)"
fi
if [ -d "$ISSUES_DIR" ]; then
  RESOLVABLE="$(printf '%s\n%s\n' "$RESOLVABLE" \
    "$(find "$ISSUES_DIR" -mindepth 2 -maxdepth 2 -type d 2>/dev/null |
      sed 's|.*/||' | grep -Eo '^[0-9]{3,4}')" | grep -v '^$' | sort -u)"
fi

# DOES THE DECLARED RECORD LEAD ANYWHERE?
#
# The record is now a SINGLE DECLARED STRING, so its kind is read from its shape
# once and never guessed from surrounding prose.  A record whose shape matches
# nothing is DANGLING rather than assumed-fine: an unrecognised record is exactly
# as untraceable as a missing one, and the pre-migration grammar's habit of
# falling through to "conforms" on an unmatched pattern is what let a dead
# `issue 0046` citation certify itself for days.
record_resolves() {
  local rec="$1"
  case "$rec" in
    parity.md)
      [ -f "$ST_DIR/parity.md" ]
      ;;
    "issue "[0-9]*)
      echo "$RESOLVABLE" | grep -qx "$(echo "$rec" | grep -Eo '[0-9]{3,4}')"
      ;;
    *.md|*.rs|*.sh|*.json|*.toml|*.txt|*.md:*|*.rs:*|*.sh:*|*.json:*|*.toml:*|*.txt:*)
      [ -e "$REPO_ROOT/${rec%%:*}" ] || [ -e "$ST_DIR/${rec%%:*}" ]
      ;;
    *)
      if echo "$rec" | grep -Eq '^[0-9a-f]{7,40}$'; then
        git -C "$REPO_ROOT" cat-file -e "${rec}^{commit}" 2>/dev/null
      else
        return 1
      fi
      ;;
  esac
}

OK=0; BAD=0; HV=0; DANGLING=0; PROV=0; CLASS=0
# SEEN and SKIPPED exist so the partition below can be checked against the INPUT
# rather than against itself -- see the closure block after the summary.
SEEN=0; SKIPPED=0
REPORT=""; HV_IDS=""; DANGLING_REPORT=""; PROV_REPORT=""; CLASS_IDS=""

while IFS=$'\t' read -r id state authority date record; do
  SEEN=$((SEEN + 1))
  # A row whose id is empty is malformed input, not a ruling. It is COUNTED
  # rather than silently dropped: the old bare `continue` is exactly the shape
  # this closure check exists to expose, and a skip nobody can see is worth less
  # than a skip that has to be explained.
  if [ -z "$id" ]; then SKIPPED=$((SKIPPED + 1)); continue; fi

  # --- schema arm: is the DECLARATION well formed? -------------------------
  bad=""
  echo "$STATES"      | grep -qw -- "$state"     || bad="$bad state=<$state>"
  echo "$AUTHORITIES" | grep -qw -- "$authority" || bad="$bad authority=<$authority>"
  echo "$date" | grep -Eq '^[0-9]{4}-[0-9]{2}-[0-9]{2}$' || bad="$bad date=<$date>"

  # A NULL RECORD IS LEGAL FOR `hv` AND FOR NOBODY ELSE, and it is a schema rule
  # rather than a convention so that the unverifiable-by-construction count is a
  # FIRST-CLASS NUMBER instead of a paragraph in this output.  The record
  # requirement stops a node laundering its own ruling; with `hv` as the
  # authority the node writing the stamp is not the authority it names, so
  # independence already holds and a record adds nothing to it.
  if [ -z "$record" ] && [ "$authority" != "hv" ]; then
    bad="$bad record=<null, and null is legal only for hv>"
  fi

  if [ -n "$bad" ]; then
    BAD=$((BAD + 1))
    REPORT="$REPORT
  non-conforming -- $id --$bad"
    continue
  fi

  # --- verdict arm --------------------------------------------------------
  #
  # PROVISIONAL IS TESTED FIRST, AHEAD OF EVERYTHING, and the reason survives the
  # migration unchanged: a provisional ruling with a perfectly good record would
  # otherwise land in `conform`, and conform is the statement the hold is about.
  # A good record does not make an unmade ruling.  It is now a DECLARED state
  # rather than a keyword grep, so it can no longer be produced or destroyed by
  # rewording -- but the ORDER still matters and is still deliberate.
  if [ "$state" = "provisional" ]; then
    PROV=$((PROV + 1))
    PROV_REPORT="$PROV_REPORT
  provisional -- $id -- $authority, $date, record: ${record:-<none>}"
    continue
  fi

  if [ -z "$record" ]; then
    HV=$((HV + 1)); HV_IDS="$HV_IDS $id"
    continue
  fi

  if ! record_resolves "$record"; then
    DANGLING=$((DANGLING + 1))
    DANGLING_REPORT="$DANGLING_REPORT
  dangling -- $id -- $authority, $date, record \`$record\` leads nowhere"
    continue
  fi

  if [ "$record" = "parity.md" ]; then
    CLASS=$((CLASS + 1)); CLASS_IDS="$CLASS_IDS $id"
  fi
  OK=$((OK + 1))
done <<< "$ROWS"

printf 'ratified-in: %d ruling(s) declared; %d conform (authority + date + resolving record); %d hv without a record (legal, unverifiable by construction); %d PROVISIONAL; %d dangling; %d non-conforming.\n' \
  "$((OK + BAD + HV + DANGLING + PROV))" "$OK" "$HV" "$PROV" "$DANGLING" "$BAD"

# **THE CLOSURE IS CHECKED AGAINST THE INPUT, NOT AGAINST THE PRINTED TOTAL, AND
# THE DIFFERENCE IS THE WHOLE VALUE OF THIS BLOCK.** The `declared` figure above
# is COMPUTED as the sum of the five parts, so stating `39 + 1 + 1 + 0 + 0 = 41`
# and refusing when it fails would be a guard that cannot fire -- it would red
# nothing, ever, which is the defect this estate keeps finding in other people's
# instruments and would deserve to find in this one.
#
# What CAN fail is the parts against the rows actually READ. Every path through
# the loop increments exactly one counter today; a sixth branch with a `continue`
# and no counter would drop a ruling, and the printed total would follow the
# parts down and agree with itself all the way. So the population is the
# comparison, and `SKIPPED` is a declared bucket rather than a hole.
PARTITION=$((OK + BAD + HV + DANGLING + PROV + SKIPPED))
printf '  -- partition: %d conform + %d hv-without-record + %d provisional + %d dangling + %d non-conforming + %d malformed = %d, against %d rows read\n' \
  "$OK" "$HV" "$PROV" "$DANGLING" "$BAD" "$SKIPPED" "$PARTITION" "$SEEN"
if [ "$PARTITION" -ne "$SEEN" ]; then
  die "the ruling buckets account for $PARTITION rows and $SEEN were read. $((SEEN - PARTITION)) ruling(s) fell through every branch and are reported by nothing -- the `declared` total above is the sum of the buckets, so it followed them down and cannot show this"
fi

if [ "$CLASS" -gt 0 ]; then
  printf '\n  of the conforming, %d are ratified BY CLASS -- record `parity.md`, hv 2026-08-14 (%s )\n' "$CLASS" "$CLASS_IDS"
  printf '  -- `corrected_check.sh` COMPUTES its scope from this record rather than reading a\n'
  printf '     separate declared field, so there is one record of the class and not two to drift.\n'
fi

if [ "$HV" -gt 0 ]; then
  printf '\n  hv RULINGS WITHOUT A RECORD -- legal, and NOT a worklist item (%d):%s\n' "$HV" "$HV_IDS"
  printf '  -- **THIS LINE IS THE COUNT OF RATIFICATIONS THAT ARE UNVERIFIABLE BY CONSTRUCTION.**\n'
  printf '     A node CAN declare `authority: hv` for a ruling never given and nothing can check\n'
  printf '     it -- the whiteboard-timestamp problem, where a fabricated stamp is\n'
  printf '     indistinguishable from a real one by inspection.  Kept visible rather than\n'
  printf '     absorbed into the conforming total.\n'
  printf '  -- **AND IT IS NO LONGER A FALLTHROUGH.** Before the migration this bucket was\n'
  printf '     reached by a FAILED REGEX (a missing record plus a leading `hv`), so a ruling\n'
  printf '     made under a DELEGATED pen and stamped `hv, <date>` landed here silently and\n'
  printf '     read as hv first-hand.  It is now `authority == hv && record == null` over\n'
  printf '     declared fields, so a delegated ruling carries `authority: vc` and CANNOT arrive\n'
  printf '     here by accident.  The pen boundary is structural instead of remembered.\n'
fi

if [ "$PROV" -gt 0 ]; then
  printf '\n  PROVISIONAL -- recorded, and NOT ratified.  NOT a worklist item; only hv can move these (%d):%s\n' "$PROV" "$PROV_REPORT"
  printf '  -- The token is LOAD-BEARING: when a provisional ruling settles, change the declared\n'
  printf '     `state` rather than annotating the prose.  `no longer provisional` read as\n'
  printf '     provisional under the old keyword grep, and prose is no longer consulted at all.\n'
fi

if [ "$DANGLING" -gt 0 ]; then
  printf '\n  DANGLING -- a record is declared and leads nowhere (%d):%s\n' "$DANGLING" "$DANGLING_REPORT"
  printf '  -- The remedy is to RE-ANCHOR on a record whose diff carries the ruling, never to\n'
  printf '     delete the citation.  A reader running `git show` must land on the ruling.\n'
fi

if [ "$BAD" -gt 0 ]; then
  printf '\n  NON-CONFORMING -- the declaration itself is malformed (%d):%s\n' "$BAD" "$REPORT"
  exit 1
fi

if [ "$DANGLING" -gt 0 ]; then
  exit 1
fi

printf '  every declared ruling is well formed, and every record it names still resolves.\n'
exit 0
