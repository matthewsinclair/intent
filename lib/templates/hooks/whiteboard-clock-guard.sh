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
#     blocked for saying so. A and B now read the same two shapes: entry
#     headings and `heartbeat_at:`. No coverage is lost -- those are the only
#     places the protocol puts a time -- and reporting a bad stamp stops being
#     an offence.
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
# TOLERANCE: 120s, check A only. Stamps are minute-granular, so one written at
# 14:59:50 and committed at 15:00:05 is honest. The errors this catches are +7
# minutes and worse, so the tolerance costs nothing. It is jitter allowance, NOT
# an escape hatch -- there is no bypass flag and none should be added. B and C
# are exact, because neither is about elapsed time.

set -uo pipefail

# PORT 4: C collation for the whole script, so check C's `<` compares ISO stamps
# as ASCII under every locale a consumer might run. Date output is numeric and
# unaffected.
export LC_ALL=C

readonly TOLERANCE_SECONDS=120
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

# PORT 2 + 3: the two shapes that carry a protocol timestamp. Everything else on
# a board is prose, including prose about timestamps.
STAMP_LINES_RE='^\+(## \(|heartbeat_at:)'

report_header() {
  if [ "$violations" -eq 0 ]; then
    echo "" >&2
    echo "BLOCKED: whiteboard timestamp cannot be a real clock read." >&2
    echo "" >&2
  fi
}

# --- CHECK A: no stamp may be in the future --------------------------------
stamps="$(printf '%s\n' "$added_lines" |
  grep -E "$STAMP_LINES_RE" |
  grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2}' |
  tr 'T' ' ' | sort -u || true)"

while IFS= read -r human; do
  [ -n "$human" ] || continue
  stamp_epoch="$(to_epoch "$human")"
  # Not a parseable calendar time is not this guard's business.
  [ -n "$stamp_epoch" ] || continue
  drift=$((stamp_epoch - now_epoch))
  if [ "$drift" -gt "$TOLERANCE_SECONDS" ]; then
    report_header
    printf '  [A future]  %sZ is %d minutes ahead of now.\n' "$human" "$((drift / 60))" >&2
    violations=$((violations + 1))
  fi
done <<EOF
$stamps
EOF

# --- CHECK B: every added stamp must carry the trailing Z ------------------
# An entry heading closes its paren straight after the minutes when the Z is
# missing; a heartbeat ends the line there. PORT 3: either ISO separator.
unmarked="$(printf '%s\n' "$added_lines" |
  grep -oE '^\+(## \([0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}\)|heartbeat_at: *[0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2} *$)' |
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
