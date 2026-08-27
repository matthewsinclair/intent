#!/usr/bin/env bash
#
# whiteboard-clock-guard.sh -- refuse a commit whose whiteboard timestamps
# cannot be real clock reads. THREE checks, each closing a hole the others
# cannot see.
#
# ORIGIN: built and measured in Lamplight (`bin/hooks/whiteboard-clock-guard`,
# check A 2026-08-11, checks B and C 2026-08-14) and brought upstream here,
# because Intent ships the whiteboard protocol and every consumer inherits the
# hole otherwise. The measurements in the comments below are Lamplight's, on a
# five-node board; they are kept because they are the evidence for the design,
# not decoration. Ported with four changes, each marked PORT: below.
#
# WHY THIS IS A GATE AND NOT A RULE.
#
# The rule "never hand-write a stamp, use `date -u`" is canon in the
# `in-whiteboard` skill ("Every timestamp is READ FROM A CLOCK"), and every node
# on a five-node board broke it repeatedly while nothing enforced it. The
# failure is SILENT (a wrong time is still a valid timestamp), PLAUSIBLE (15:52Z
# looks exactly like 14:52Z), and EXPENSIVE (heartbeats decide whether a peer
# reads you as live, and the 7-day reclaim rule can never expire a stamp set in
# the future). A rule nobody can check is a wish.
#
# ---------------------------------------------------------------------------
# CHECK A -- FUTURE STAMP
#
# A stamp cannot postdate the commit that adds it, so a positive delta is proof
# of a bad read. Measured 2026-08-11: positive deltas from all four nodes,
# clustered at +40 to +65 minutes -- the signature of reading a LOCAL clock (the
# machine runs BST) and appending a `Z`.
#
# CHECK B -- MISSING `Z`
#
# Check A alone does NOT catch the local-clock error, and measuring it is what
# proved that: 14 unmarked headings were written AFTER check A landed. The
# reason is arithmetic. An unmarked `## (2026-08-14 14:19)` is parsed as UTC, so
# it only trips check A while it is still in the future; the moment a node's
# commit lags by more than the local offset (one hour in BST) the same bad stamp
# sails through. Lag is normal -- over 68 real stamps, 93% commit within the
# hour, with a tail to nine hours. Check A was only ever catching the fast half.
#
# So the `Z` is checked SYNTACTICALLY: no clock, no tolerance. An unmarked stamp
# is not a wrong time but an UNKNOWN one, and "assume local, treat its ordering
# as unreliable" is not a property a live coordination channel can carry.
#
# CHECK C -- APPEND-ONLY MONOTONICITY
#
# A and B both compare a stamp to a clock, so a FABRICATED stamp landing in the
# PAST passes both in silence -- and that is the failure the skill names first.
# An inbox is append-only by protocol, so its entry headings must increase
# monotonically. That is a two-sided test needing no clock at all: it compares
# two board stamps to each other, a real `date -u` read can never break it
# (time does not run backwards), and a fabricated stamp breaks it immediately.
#
# Measured 2026-08-14 in Lamplight: three live violations, by 50, 28 and 9
# minutes. B and C overlapped on that day's evidence and are still kept
# separate, because they fail independently -- B cannot see a fabricated stamp
# carrying a `Z`, and C cannot see a wrong-clock stamp that lands in order.
#
# SCOPE: only lines this commit ADDS, under `intent/whiteboard/`, never
# `.history/`. Archives replay old entries verbatim, so `clear` and `archive`
# legitimately add stamps hours or days old; excluding that path is what stops
# the guard blocking the protocol's own housekeeping. Measured: including
# `.history/` put the apparent lag tail at 55 HOURS; excluding it, 9.
#
# CHECK C NEVER BLOCKS ON PRE-EXISTING BREAKAGE -- it fires only when a stamp
# THIS COMMIT ADDS is out of order. Otherwise existing violations would wedge
# every future commit to that file, and a guard that must be bypassed to work is
# a guard nobody keeps.
#
# IT DOES NOT AUTO-CORRECT. A guard that silently fixes the stamp hides the
# class from the person committing, and the point is that the node learns the
# clock it read was the wrong one. It prints the correct value so the fix is a
# copy-paste. Per the skill's corollary, a stamp you never read is not
# recoverable -- do not invent a better-looking one.
#
# ---------------------------------------------------------------------------
# PORT CHANGES from the Lamplight original:
#
#   PORT 1 -- PORTABILITY. The original parses with BSD `date -j -f`, which is
#     macOS-only. Intent runs a Linux CI leg and has shipped a Linux-only break
#     before (v2.11.14, `((x++))` under `set -e`), so the flavour is detected
#     once and both are supported. A guard that cannot run on the CI runner is
#     a guard that is green because it never executed.
#
#   PORT 2 -- CHECK A IS ANCHORED TO STAMP-BEARING LINES. The original scans
#     every added line for anything date-shaped, so a board DISCUSSING a
#     future-dated stamp (quoting a peer's bad heartbeat in a message, which is
#     exactly what happens when nodes report this class to each other) would be
#     blocked for saying so. A and B read POSITIONAL shapes only, so reporting a
#     FUTURE-dated bad stamp stops being an offence.
#
#     WHAT THIS IS, EXACTLY: CHECK A WAS INCONSISTENT WITH B AND C, and the
#     reason to prefer their behaviour is already written down one check below.
#     B anchors to the leading heading stamp; C takes the entry's OWN leading
#     stamp and says why -- a `Re:` anchor points backwards by definition, so
#     reading every date on the line makes every threaded reply look like an
#     inbox travelling back in time, found when this guard blocked its own
#     announcement commit. A carried that same defect and the repair was never
#     applied sideways. This change applies it.
#
#     IT IS NOT A COVERAGE GAP, AND THE WIDER CLAIM IS THE ONE TO AVOID HERE.
#     Run-verified on inbox fixtures rather than read: a node fabricating its
#     OWN leading stamp backwards is BLOCKED, at HEAD and here alike -- check C
#     catches the dangerous direction and always did. What passes is a QUOTED,
#     NON-LEADING past stamp, which is quoted text reporting someone else's
#     defect rather than this board's ordering data, and it SHOULD pass.
#
#     An earlier draft of this note said the backwards direction "was never
#     checked on this surface at all". That was measured on `wip.md`, where C
#     does not apply, and generalised to all three checks -- a conclusion drawn
#     from a fixture that could not have shown otherwise. It is the same defect
#     as the over-stated reach two comments down, committed while correcting it.
#
#     THIS NOTE USED TO END "No coverage is lost -- those are the only places the
#     protocol puts a time", AND THAT WAS FALSE WHEN IT WAS WRITTEN. The protocol
#     names three surfaces and this guard read two, so anchoring positionally DID
#     lose coverage: every date in a `## Decisions` line, 97 of them on Intent's
#     own board, none scanned. The sentence was re-asserted through a rewrite of
#     this region rather than checked against the protocol it describes.
#
#     A GUARD THAT STATES ITS OWN REACH IS MAKING A CLAIM NOBODY TESTS, and this
#     one was wrong for as long as it stood. The reach is now the three shapes in
#     `STAMP_LINES_RE` below, and that list is the only place to read it from.
#
#   PORT 3 -- CHECK B ACCEPTS EITHER ISO SEPARATOR. The original requires `T`
#     in `heartbeat_at`, so `heartbeat_at: 2026-08-14 14:43` (space separator,
#     no Z) slips through unchecked. Both separators are in live use on Intent's
#     own board today. The separator is not this guard's business; the missing
#     `Z` is, and it must be caught under either spelling.
#
#   PORT 4 -- `LC_ALL=C` for check C's comparison. `[[ a < b ]]` uses the
#     locale's collation order; ISO stamps compare correctly under C and are not
#     guaranteed to under every locale a consumer might run.
#
# STILL NOT CLOSED, and stated so nobody reads a green as proof: a fabricated
# stamp that carries a `Z`, lands in the past, and still increases
# monotonically passes all three checks. Smaller target, not an empty one.
#
# TOLERANCE: 0, check A only. Ruled 2026-08-27, authority vc, after both
# controls passed. It was 120s from 2026-08-14 until then, and THE 120 WAS
# JUSTIFIED BY A CASE THAT CANNOT OCCUR. The old note read "a stamp written at
# 14:59:50 and committed at 15:00:05 is honest" -- true, and that stamp reads
# `14:59` while the commit lands in the 15:00 minute, so its drift is NEGATIVE
# and check A was never going to see it. Commit lag can only drive drift down.
# The tolerance was allowing for jitter in the one direction jitter cannot
# travel, which is why it never cost a false positive and never caught anything.
#
# WHAT IT COST, MEASURED ON THIS ESTATE BEFORE THE CHANGE: the guard ran for
# thirteen days and detected NOTHING, while twenty future stamps landed in
# Intent's own board history and walked past it. Both known instances are
# exactly +60s and `-gt` is strict, so 120 missed them -- and so would 60.
#
# WHY 0 AND NOT MERELY A SMALLER NUMBER: it is the only value at which the two
# date(1) flavours fire on the SAME SET. BSD `date -j -f` fills unspecified
# seconds from the current clock, so drift == stamp_min - commit_min and the
# seconds cancel; GNU `date -d` zero-fills, so drift == that minus commit_sec,
# with commit_sec in [0,59]. Both are > 0 for exactly the stamps whose MINUTE is
# strictly after the commit's. At 30 the same stamp fires on BSD and passes on
# GNU for half of every minute. Measured on both flavours, not derived.
#
# FALSE POSITIVES ARE ZERO BY CONSTRUCTION RATHER THAN BY COUNTING, which is the
# stronger claim because it does not depend on a sample: a clock read taken
# BEFORE the commit cannot name a later minute, lag drives drift negative, and
# rebase and amend move the COMMITTER date later -- so rewriting history can
# only HIDE a violation, never manufacture one. There is no legitimate producer
# of positive drift.
#
# It is jitter allowance no longer, and it was never an escape hatch -- there is
# no bypass flag and none should be added. B and C are exact, because neither is
# about elapsed time.

set -uo pipefail

# PORT 4: C collation for the whole script, so check C's `<` compares ISO stamps
# as ASCII under every locale a consumer might run. Date output is numeric and
# unaffected.
export LC_ALL=C

readonly TOLERANCE_SECONDS=0
# The trailing `**` on the exclude is LOAD-BEARING, not decoration. A pathspec
# containing a wildcard is matched by wildmatch against the WHOLE path, so
# `intent/whiteboard/*/.history/` excludes nothing -- it does not match
# `intent/whiteboard/ic/.history/20260814/inbox.cc.md`. Dropping it during the
# port silently disabled the archive exclusion, and the FP control did not catch
# it because the control's fixture was itself missing. Verified by the case, not
# by reading.
readonly WB_PATHS=('intent/whiteboard/' ':(exclude)intent/whiteboard/*/.history/**')

# Opt-in by presence, exactly like the whiteboard itself: a project without a
# board is not one this guard has an opinion about.
[ -d "intent/whiteboard" ] || exit 0

# PORT 1: detect the date flavour once rather than per-stamp.
if date -u -j -f '%Y-%m-%d %H:%M' '2000-01-01 00:00' '+%s' >/dev/null 2>&1; then
  readonly DATE_FLAVOUR=bsd
elif date -u -d '2000-01-01 00:00' '+%s' >/dev/null 2>&1; then
  readonly DATE_FLAVOUR=gnu
else
  # No parser we recognise. Fail OPEN and say so: this guard must never be the
  # reason a commit cannot be made, and a silent skip would be the same class of
  # invisible non-enforcement it exists to fix.
  echo "whiteboard-clock-guard: no usable date(1) parser; skipping (fail-open)." >&2
  exit 0
fi

to_epoch() {
  case "$DATE_FLAVOUR" in
    bsd) date -u -j -f '%Y-%m-%d %H:%M' "$1" '+%s' 2>/dev/null ;;
    gnu) date -u -d "$1" '+%s' 2>/dev/null ;;
  esac
}

now_epoch="$(date -u '+%s')"
now_human="$(date -u '+%Y-%m-%d %H:%MZ')"
violations=0

added_lines="$(git diff --cached --unified=0 -- "${WB_PATHS[@]}" 2>/dev/null |
  grep -E '^\+' | grep -Ev '^\+\+\+' || true)"

[ -n "$added_lines" ] || exit 0

# PORT 2 + 3: the shapes that carry a protocol timestamp. Everything else on a
# board is prose, including prose about timestamps.
#
# THREE SHAPES, NOT TWO, AND THE COMMENT HERE ASSERTED TWO WHILE THE THIRD WENT
# UNSCANNED. The protocol (`in-whiteboard` SKILL.md) names entry headings,
# `heartbeat_at:`, AND every date in a `## Decisions` line. This file used to say
# "the two shapes that carry a protocol timestamp", and the PORT 2 note above
# used to add "those are the only places the protocol puts a time" -- both false,
# both load-bearing for a reader deciding whether to look further, and the claim
# was RE-ASSERTED through a rewrite of this region rather than checked. Found by
# baize-vc; measured on Intent's own board at 97 dated Decisions bullets, 0 of
# them scanned.
#
# THE DECISIONS FORM IS POSITIONAL LIKE THE OTHER TWO, WHICH IS PORT 2's WHOLE
# POINT: it anchors to the bullet opening, so only a date sitting where the
# protocol reserves a decision's OWN date is read, and a node quoting a peer's
# bad date mid-sentence is still not blocked for reporting it.
#
# BOTH BULLET FORMS ARE COVERED DELIBERATELY. A census of one board found 68
# date-only (`- (YYYY-MM-DD)`), 28 carrying a time (`- **(YYYY-MM-DD HH:MMZ, ...`)
# and 1 with the date followed by bold markup. A pattern requiring `)` straight
# after the date reaches the 68 and misses the 28 -- and the 28 are exactly where
# checks A and B do real work, because they carry a time to be wrong about.
#
# WHAT THIS DOES NOT REACH, NAMED HERE BECAUSE AN UNSTATED SCOPE IS THE DEFECT
# THAT CREATED THIS WHOLE ITEM. Two live forms are NOT matched, both measured,
# both left out on purpose rather than missed:
#
#   - A DATED `###` HEADING, eg `### 2026-08-26 -- ruled in chat`. Intent's own
#     board carries 5 and Lamplight's 4, and on Lamplight the hv node -- the
#     estate's rulings record -- uses ONLY this form, so its most authoritative
#     dated surface is unscanned. Found by lamplight-vc, who measured it and
#     explicitly did not argue for widening. Not widened here either: hv ruled a
#     specific shape and extending scope afterwards on my own initiative is how a
#     guard grows reach nobody sanctioned. FILED, NOT FIXED.
#   - A NESTED OR DECORATED BULLET, eg `   - _(2026-08-25 15:44Z, ...`. The
#     anchor is `^- ` with no leading whitespace, and relaxing it would start
#     matching dated bullets in arbitrary nested prose lists -- which is the
#     PORT 2 hazard, blocking a report. Chosen, not inherited.
#   - AN AUTHOR-FIRST STAMP, eg `- **(hv, 2026-08-26) ...`. This one IS a
#     protocol stamp in a variant field order, not a different kind of line, so
#     it is the weakest of the three exclusions. Intent's own board carries 10
#     and no other estate has any -- a local convention on transcribed hv
#     rulings. None is future-dated, so nothing is being missed that would block
#     today. FILED, NOT FIXED, on the same reasoning as the `###` form: hv ruled
#     a shape and widening the matcher afterwards on my own initiative is how a
#     guard acquires reach nobody sanctioned. Found by devbin-vc's broader
#     predicate.
#
# AND ONE FORM THAT IS CORRECTLY IGNORED RATHER THAN MISSED, which is the
# evidence FOR requiring the parenthesis: a dated bullet with no opening
# parenthesis is PROSE THAT MENTIONS A DATE, not a stamp -- `- Cross-node
# decisions from 2026-08-24 are archived at ...`, `- **2026-08-25's rulings are
# NOT carried here.**`. Laksa, Prolix and Devbin carry one or two each. Reading
# those would block a node for writing a sentence about a date, which is the
# PORT 2 hazard in its plainest form.
#
# `## Standing directives` IS IN SCOPE AND THAT IS CORRECT, not over-reach: the
# protocol says peers read it "the way they read `## Decisions`", same dated
# bullet format, same read path. A line-positional matcher covers it for free
# and should. Established by laksa-cc against a section-scoped count that was
# narrower than the protocol's own surface.
#
# THE ONE PLACE THE TWO SURFACES DIFFER IN KIND, and it is ruled here rather
# than discovered at a refusal: a `## Decisions` bullet RECORDS something that
# happened, so a future date is always wrong. A STANDING DIRECTIVE is an
# instruction holding until revoked, and "from 2026-09-01, releases go out on
# Tuesdays" is a legitimate thing to write. This guard still refuses a future
# OPENING stamp there, because that stamp means WHEN THIS WAS WRITTEN on every
# other surface and a bullet is not the place to change what a stamp means.
# Stamp it today and put the effective date in the body. The refusal says so.
STAMP_LINES_RE='^\+(## \(|heartbeat_at:|- (\*\*)?\([0-9]{4}-[0-9]{2}-[0-9]{2})'

report_header() {
  if [ "$violations" -eq 0 ]; then
    echo "" >&2
    echo "BLOCKED: whiteboard timestamp cannot be a real clock read." >&2
    echo "" >&2
  fi
}

# --- CHECK A: no stamp may be in the future --------------------------------
# A DATE WITH NO TIME IS NORMALISED TO MIDNIGHT EXPLICITLY, AND THAT LINE IS THE
# WHOLE REASON THIS EXTENSION IS SAFE. `## Decisions` dates are often date-only,
# and the two date(1) flavours do not merely differ on those -- they disagree by
# up to a day. Measured at 19:59:32Z on a stamp dated that same day:
#
#   BSD  date -j -f '%Y-%m-%d %H:%M' '2026-08-27'  -> Failed conversion
#   BSD  date -j -f '%Y-%m-%d'       '2026-08-27'  -> 2026-08-27 19:59:32
#   GNU  date -d                     '2026-08-27'  -> 2026-08-27 00:00:00
#
# BSD FILLS THE UNSPECIFIED TIME FROM THE CURRENT CLOCK, so a decision dated
# TODAY parses to NOW. `now_epoch` is read once before this loop and `to_epoch`
# is called per stamp inside it, so any elapsed time makes the drift POSITIVE and
# a tolerance of 0 REFUSES A DECISION DATED TODAY. With ~100 bullets in the loop
# that is not hypothetical. Normalising to `<date> 00:00` pins the field both
# flavours would otherwise invent, and they then agree.
#
# THE FAILURE THIS AVOIDS IS FAIL-CLOSED ON CORRECT WORK, which is worse than the
# gap it closes: the same flake shape that `c53dc201` rejects a +1min fixture for,
# except that one fails open and this one would block honest commits at random.
#
# The row carries `<parsed>\t<display>` so the report can name what the author
# actually wrote. Appending a `Z` to a date the author never gave a time for
# would be the guard asserting a zone on their behalf.
# THE EXTRACTION IS POSITIONAL TOO, AND IT HAS TO BE. Selecting the LINE
# positionally and then scanning the WHOLE line for dates is not PORT 2, it only
# looks like it: a Decisions bullet's prose continues after its own date, so
# `- (2026-08-26) vc stamped their heartbeat (2026-08-28 09:00Z), which is ahead`
# yields TWO dates and the quoted one blocks the commit. That is reporting the
# defect being treated as committing it -- the precise failure PORT 2 exists to
# prevent -- and control (b) caught it in the first build of this change.
#
# It was latent for the two original shapes as well: a message heading whose text
# quoted a future stamp had the same hole, untested and unnoticed. Anchoring the
# capture to the line opening closes both, and makes the code match what the
# PORT 2 note has claimed all along. A `Re:` anchor is dropped here for free; it
# points backwards by definition and check C reads it separately.
stamps="$(printf '%s\n' "$added_lines" |
  grep -E "$STAMP_LINES_RE" |
  sed -E 's/^\+(## \(|heartbeat_at: *|- (\*\*)?\()([0-9]{4}-[0-9]{2}-[0-9]{2}([T ][0-9]{2}:[0-9]{2})?).*$/\3/' |
  tr 'T' ' ' |
  awk '{ if (NF == 1) printf "%s 00:00\t%s (a date with no time, read as midnight)\n", $1, $1;
         else         printf "%s\t%sZ\n", $0, $0 }' |
  sort -u || true)"

while IFS="$(printf '\t')" read -r human shown; do
  [ -n "$human" ] || continue
  stamp_epoch="$(to_epoch "$human")"
  # Not a parseable calendar time is not this guard's business.
  [ -n "$stamp_epoch" ] || continue
  drift=$((stamp_epoch - now_epoch))
  if [ "$drift" -gt "$TOLERANCE_SECONDS" ]; then
    report_header
    printf '  [A future]  %s is %d minutes ahead of now.\n' "$shown" "$((drift / 60))" >&2
    # A DATE-ONLY STAMP HAS ONE LEGITIMATE-LOOKING WAY TO BE FUTURE-DATED AND
    # THE GENERIC ADVICE BELOW DOES NOT ADDRESS IT. `## Standing directives` is
    # read like `## Decisions` and carries the same dated bullets, but a
    # directive HOLDS UNTIL REVOKED rather than recording something that
    # happened -- so "from 2026-09-01, releases go out on Tuesdays" is a
    # reasonable thing to write and this guard refuses it. The rule is not being
    # relaxed: an opening stamp means WHEN THIS WAS WRITTEN on every other
    # surface, and a bullet is not the place to change what a stamp means. But
    # someone hitting that deserves the remedy rather than a lecture about
    # clocks. Raised by laksa-cc from a board where the case cannot arise, which
    # is the only reason it was ruled before the first refusal instead of after.
    case "$shown" in
      *"a date with no time"*)
        echo "              if this takes effect LATER, the stamp is still TODAY: put the effective date in the body." >&2
        ;;
    esac
    violations=$((violations + 1))
  fi
done <<EOF
$stamps
EOF

# --- CHECK B: every added stamp must carry the trailing Z ------------------
# An entry heading closes its paren straight after the minutes when the Z is
# missing; a heartbeat ends the line there. PORT 3: either ISO separator.
#
# A `## Decisions` BULLET IS THE THIRD ARM, AND IT IS SCOPED TO THE ONES THAT
# CARRY A TIME. A date-only decision has no zone to mark, so B is inapplicable to
# it BY CONSTRUCTION rather than by exemption -- but the claim that Decisions
# dates are date-only is false: a census of one board found 28 of 97 carrying
# `HH:MMZ`, and an unmarked time there is exactly as ambiguous as an unmarked
# heading. Those 28 all carry their Z today, so this arm ships green and stays
# that way only while nodes keep reading `date -u`.
#
# The trailing `[^Z]` is what makes it a NO-Z test rather than a has-a-time test:
# the bullet's prose continues after the stamp, so unlike the two arms above
# there is no line end or closing paren to anchor against.
unmarked="$(printf '%s\n' "$added_lines" |
  grep -oE '^\+(## \([0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}\)|heartbeat_at: *[0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2} *$|- (\*\*)?\([0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2}[^Z])' |
  sed 's/^+//' | sort -u || true)"

while IFS= read -r line; do
  [ -n "$line" ] || continue
  report_header
  printf '  [B no Z]    %s  <- unmarked: assume LOCAL, ordering unreliable.\n' "$line" >&2
  violations=$((violations + 1))
done <<EOF
$unmarked
EOF

# --- CHECK C: an inbox is append-only, so its headings must not go back -----
changed_inboxes="$(git diff --cached --name-only --diff-filter=ACM -- "${WB_PATHS[@]}" 2>/dev/null |
  grep -E '/inbox\.[a-z]{2}\.md$' || true)"

# A heading can carry MORE than one date: the documented message format is
#   ## (<stamp>)   [Re: <prior-anchor>]   [FYI only ...]
# and a `Re:` anchor points BACKWARDS by definition -- it names the earlier
# entry being replied to. Taking every date on the line makes every threaded
# reply look like an inbox travelling back in time. Found by this guard blocking
# its own announcement commit, on its first real use; the upstream original has
# the same defect. So: take only the parenthesised stamp that OPENS the heading,
# which is the entry's own.
heading_stamps() {
  sed -nE 's/^\+?## \(([0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2}).*/\1/p' | tr 'T' ' '
}

for f in $changed_inboxes; do
  f_added="$(git diff --cached --unified=0 -- "$f" 2>/dev/null |
    grep -E '^\+## \(' | heading_stamps | sort -u || true)"
  [ -n "$f_added" ] || continue

  # The file as it WILL be, in document order.
  f_all="$(git show ":$f" 2>/dev/null |
    grep -E '^## \(' | heading_stamps || true)"
  [ -n "$f_all" ] || continue

  running_max=""
  while IFS= read -r s; do
    [ -n "$s" ] || continue
    if [ -n "$running_max" ] && [[ "$s" < "$running_max" ]]; then
      # Only stamps THIS COMMIT adds block; pre-existing breakage is not this
      # commit's to answer for and must never wedge the file.
      # Herestring, NOT a pipeline. `grep -q` exits on first match, `printf`
      # then takes SIGPIPE, and `pipefail` promotes 141 to the PIPELINE status
      # -- so the pipeline form read FALSE and passed a real violation through
      # as inherited breakage. A herestring has no pipeline status to corrupt.
      if grep -qxF -- "$s" <<<"$f_added"; then
        report_header
        printf '  [C order]   %s in %s follows %s -- an append-only inbox cannot go backwards.\n' \
          "$s" "${f#intent/whiteboard/}" "$running_max" >&2
        violations=$((violations + 1))
      fi
    else
      running_max="$s"
    fi
  done <<EOF
$f_all
EOF
done

if [ "$violations" -gt 0 ]; then
  cat >&2 <<EOF

  The clock is now:  ${now_human}

  Read every stamp from:  date -u '+%Y-%m-%d %H:%MZ'

  A stamp you did not read off a clock is FABRICATED, not approximate -- you
  have no clock and no felt duration, so there is nothing to be roughly right
  about. \`git log\` prints LOCAL time and is the usual source of the +1h error.

  Do NOT repair a fabricated stamp by inventing a better-looking one. You
  cannot recover a time you never read: re-run the command above.

  Rule: the \`in-whiteboard\` skill, "Every timestamp is READ FROM A CLOCK".

EOF
  exit 1
fi

exit 0
