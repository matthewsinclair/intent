#!/usr/bin/env bash
#
# whiteboard-header-guard.sh -- refuse a commit that YAML-ESCAPES a value in a
# whiteboard header block. The block is NOT YAML, and escaping it is the one way
# to get that wrong while looking careful.
#
# WHY THIS IS A SEPARATE FILE FROM whiteboard-clock-guard.sh, and it was ruled
# rather than assumed (vc, 2026-08-16). That guard's name and contract are
# TIMESTAMPS -- checks on stamps, and documentation entirely about clocks.
# Header wellformedness is a different concern, and folding it in would make the
# name lie to the next reader, which is the standing defect this estate keeps
# finding: a claim in one artefact that the next author believes. It would also
# couple two controls that should be independently canaried and independently
# disabled. One concern, one home.
#
# ---------------------------------------------------------------------------
# WHAT IS WRONG WITH AN ESCAPED VALUE
#
# The protocol rules the header block is NOT YAML: a line-oriented `key: value`
# where a single pair of SURROUNDING quotes is a display delimiter, and quotes
# INSIDE a value are literal and never escaped. The reader is `fm_get` (in
# `intent_claude_cwi`, shared by `ws list` and `ws hygiene`), and it strips one
# pair of surrounding double quotes and DELIBERATELY DOES NOT UNESCAPE. So the
# two escape forms render, in the tool people actually read boards with, as:
#
#   focus: "the \"counted\" body"   ->   the \"counted\" body     (backslashes)
#   focus: 'ic''s ruling'           ->   'ic''s ruling'           (both, plus
#                                                                  the delimiters,
#                                                                  because the
#                                                                  reader strips
#                                                                  only `"`)
#
# That is not a cosmetic complaint. The header is the board's machine-readable
# half -- `focus`, `status`, `claims`, `heartbeat_at` -- and it is what a peer
# reads at pickup to decide whether you are live and what you are holding.
#
# ---------------------------------------------------------------------------
# WHY THIS SHIPS ON ONE OBSERVATION, WHICH IS NOT THIS PROJECT'S USUAL BAR
#
# One instance reached HEAD (a `"` inside a `focus:` value). The first diagnosis
# was the pre-commit markdown formatter, and under THAT diagnosis one instance
# is evidence of rarity and deferring is obviously right. It does not survive
# measurement: `prettier --write`, at the binary the hook resolves and with the
# hook's own invocation, leaves the reconstructed input byte-identical, and
# there is no other writer. **So the author is a node that knows YAML, doing the
# correct YAML thing on meeting a `"` inside a double-quoted scalar.** That is
# not bad luck; it is the default behaviour of any competent node, and every
# consumer of this protocol runs nodes. One occurrence stops being one EVENT and
# becomes one OBSERVATION of a default.
#
# The protocol had already measured the OTHER direction of the same defect and
# only the other direction. From the `in-whiteboard` skill: a sweep of one
# node's recent revisions found INVALID headers in more than one episode, "all of
# which repaired themselves at the next fold, before anyone noticed... a defect
# whose lifetime is shorter than the interval between observations leaves no
# corpse". So:
#
#   Direction A -- a node writes INVALID YAML.        Self-repairs: the next
#                                                     reader sees something
#                                                     broken. Measured.
#   Direction B -- a node writes VALID ESCAPED YAML.  Does NOT self-repair and
#                                                     CANNOT, because nothing
#                                                     about it looks wrong. It
#                                                     is correct YAML, produced
#                                                     by care. No control.
#
# Direction B is the worse one precisely because it is produced by competence.
# This guard is the control for it.
#
# ---------------------------------------------------------------------------
# SCOPE, and every boundary here is measured rather than reasoned
#
# LIVE BOARDS ONLY -- `intent/whiteboard/<node>/wip.md`, never `.history/`. The
# exclude is LOAD-BEARING and its shape was verified by running it, not by
# reading the docs: a git pathspec wildcard is matched against the WHOLE path,
# so `intent/whiteboard/*/wip.md` CROSSES slashes. Measured on this repository,
# 2026-08-16: that pathspec alone matches the archived boards as well as the
# live ones; with the exclude, exactly the live boards. An archive replays an
# old header verbatim, so covering it would refuse the protocol's own
# housekeeping over a historical record, and the harm this guard exists to
# prevent (`ws list` rendering `ic''s`) does not exist there. The control goes
# where the harm is. (The same trap, same direction, bit the clock guard's port:
# see its `WB_PATHS` comment.)
#
# ONLY THE HEADER BLOCK. The extractor requires line 1 to be `---` and stops at
# the next `---`. This is not tidiness: an unanchored `sed -n '2,/^---$/p'`
# scans the WHOLE FILE when there is no header, which was measured doing exactly
# that. Below the fence is prose, where a doubled apostrophe is somebody
# quoting.
#
# ONLY WHAT THIS COMMIT ADDS. A header line the commit inherits is not this
# commit's to answer for. Otherwise a pre-existing escaped `focus:` would wedge
# every future heartbeat commit on that board -- and a guard that must be
# bypassed to work is a guard nobody keeps. Same principle as the clock guard's
# check C, for the same reason.
#
# THE PROSE EXEMPTION IS THE POINT, NOT AN OVERSIGHT. Nodes report this class to
# each other by QUOTING it -- an inbox entry saying "your board rendered `ic''s`"
# carries the offending bytes on purpose. Scanning prose would make reporting
# the defect an offence, which is the clock guard's PORT 2 lesson arriving in a
# second file.
#
# ---------------------------------------------------------------------------
# WHAT IS NOT CHECKED, stated rather than implied
#
# A value delimited with SINGLE quotes and no escaping (`focus: 'plain text'`)
# renders WITH its delimiters, because `fm_get` strips only `"`. **That is not a
# defect and must not be guarded here.** It looked like a spec-versus-
# implementation disagreement -- the protocol text said "a single pair of
# surrounding quotes" while the reader implements double quotes only -- and it
# was sent to vc as a contract question rather than settled by widening this
# file. RULED 2026-08-16 (vc, verified against `fm_get` in `intent_claude_cwi`): the
# prose was ambiguous, not the code; `"` is THE delimiter and `'` is never
# stripped.
#
# The reasoning is worth keeping here, because it is also the argument for this
# guard's own narrowness. Teaching `fm_get` single quotes would make a value
# that legitimately opens and closes with `'` lose two characters SILENTLY, with
# the rendered view differing from the file and nothing saying so. Leaving it
# means visible quotes: ugly, immediate, fixed at the next fold.
# `IN-AG-NO-SILENT-001` decides a display question -- prefer the failure you can
# see. And the block was ruled not-YAML precisely to have almost no rules, so a
# second delimiter form adds back one the author has to get right.
#
# A literal backslash-quote that the author genuinely meant is indistinguishable
# from the escape form by construction, so it is refused too. Judged worth it:
# the escape form is the default behaviour of every competent node and the
# literal form is close to unwritable on purpose.
#
# IT NEVER AUTO-CORRECTS. The clock guard's precedent, and the same reasoning:
# a guard that silently fixes the value hides the class from the node that needs
# to learn the format is not YAML. The repaired form is PRINTED so the fix is a
# copy-paste.
#
# ---------------------------------------------------------------------------
# CHECK 2 -- A VALUE DETACHED FROM ITS KEY, and the author is usually this gate
#
# Added 2026-09-12 by dc on vc's allocation, after ic found it at their own fold
# and vc verified it from the artefact. The defect: a header line stops being
# `key: value` and becomes a bare key with its value on the NEXT line --
#
#   claims:
#     [ST0112/WP-07, ST0112/WP-08 (with cc, by file), ST0112/WP-09, ST0112/WP-10]
#
# `fm_get` reads everything after the first `: ` on the KEY's line, so the value
# reads as EMPTY and `ws list` prints nothing for that key. RUN-VERIFIED rather
# than reasoned (Laksa, 2026-09-12, a throwaway clone, the real `intent claude ws list`):
# a board with four claims detached exactly as above rendered `claims=`, while
# every other field rendered perfectly. Nothing about the board looks wrong.
#
# THAT DIRECTION IS WHY IT IS WORTH A GUARD. The value does not degrade, it
# DISAPPEARS -- a node claiming four work packages reads as a node claiming none,
# and peer claims are what a coordinating node allocates against. Same shape as
# the escape check above: a failure produced by something competent, with no
# natural corrective, in the false-clean direction.
#
# THE USUAL AUTHOR IS THE FORMATTER, NOT A NODE. prettier formats the block as
# YAML frontmatter, and a bracketed flow sequence longer than printWidth is
# broken across lines. It reaches a commit two ways: a gate that runs `prettier
# --write` and re-stages BEFORE its guards (devbin's `gate_markdown`, where Laksa
# found this) hands the guards the reflowed bytes; a gate that only runs
# `prettier --check` (Intent's own) refuses the long line, and the `prettier
# --write` a node runs to clear that refusal is what detaches the value. Either
# way the bytes this guard reads are the formatter's.
#
# MEASURED in Laksa, not inherited (2026-09-12, prettier 3.9.6, printWidth at
# its default 80):
#
#   claims: at 80 chars   ->  untouched
#   claims: at 81 chars   ->  broken, value on 1 continuation line
#   claims: at 88+ chars  ->  broken, value expanded over 3+ lines
#   focus:  at 209 chars  ->  UNTOUCHED
#
# `focus:` is exempt because a QUOTED scalar is not a breakable construct, which
# is why every live board survives header lines of 211-338 characters. The only
# exposed key today is `claims:`, and any future unquoted value joins it.
#
# SO THE REMEDY IS "SHORTEN", NOT "REJOIN", AND THE GUARD SAYS SO WITH A NUMBER.
# A rejoined line over printWidth is broken again by the next commit, so a
# guard that printed it as the fix would wedge the node with its own repair --
# the `an-unrun-remedy-is-the-default-output` trap, arriving through the remedy
# text rather than the code. The rejoined form IS printed, with its length and
# the limit, and it is labelled unsafe to paste when it exceeds the limit.
#
# The unit is the DETACHED LINE, not the empty key. A bare `key:` with nothing
# after it is legal -- an optional value a node has not filled -- and refusing
# it would punish the honest empty case. A continuation line is unambiguous:
# there is no legal header line that is not `key: value`.
#
# SAME CORPUS AND SAME ADDED-LINES-ONLY RULE as check 1, deliberately: a board
# already carrying a detached value is not this commit's to answer for, so the
# arm wedges nothing that exists when it is armed.
#
# Exit codes: 0 clean or not applicable; 1 an added header line escapes a value
# (check 1) or detaches one from its key (check 2).

set -uo pipefail

# Byte-exact whole-line matching below, on files that legitimately carry UTF-8
# prose. C collation keeps `grep -F -x` byte-wise rather than dependent on the
# consumer's locale.
export LC_ALL=C

# Opt-in by presence, exactly like the whiteboard itself: a project without a
# board is not one this guard has an opinion about, and nothing changes for it.
[ -d "intent/whiteboard" ] || exit 0

# See SCOPE above. The exclude is not optional.
readonly WB_BOARDS=('intent/whiteboard/*/wip.md' ':(exclude)intent/whiteboard/*/.history/**')

# Lines 2..next `---`, and ONLY when line 1 is the opening fence. A file with no
# header block yields nothing rather than yielding its prose.
header_block() { # stdin: a wip.md
  awk '
    NR == 1 { if ($0 !~ /^---[[:space:]]*$/) exit; next }
    /^---[[:space:]]*$/ { exit }
    { print }
  '
}

violations=0
detached_count=0

# The formatter's line limit. The number only means anything as the limit the
# consumer's prettier actually applies, so it is READ, not assumed: a
# `printWidth` declared in the repo root's `.prettierrc.json` or `.prettierrc`
# (JSON or YAML), else prettier's default of 80.
PRINTWIDTH=80
for _rc in .prettierrc.json .prettierrc; do
  [ -f "$_rc" ] || continue
  _w="$(sed -n 's/^[[:space:]]*"\{0,1\}printWidth"\{0,1\}[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' "$_rc")"
  if [ -n "$_w" ]; then
    PRINTWIDTH="${_w%%$'\n'*}"
    break
  fi
done
readonly PRINTWIDTH
TAB="$(printf '\t')"
readonly TAB

report_header() {
  if [ "$violations" -eq 0 ]; then
    echo "" >&2
    echo "BLOCKED: a whiteboard header value has been YAML-escaped." >&2
    echo "         The header block is NOT YAML -- quotes inside a value are literal." >&2
    echo "" >&2
  fi
}

report_detached() {
  if [ "$detached_count" -eq 0 ]; then
    echo "" >&2
    echo "BLOCKED: a whiteboard header value is DETACHED from its key." >&2
    echo "         One line per key -- a value on a line of its own reads as EMPTY." >&2
    echo "" >&2
  fi
}

# Every non-blank header line that is not `key: value`, tagged with the key it
# has been detached from (the nearest key line above it).
detached_lines() { # stdin: a header block
  awk '
    /^[[:space:]]*$/ { next }
    /^[A-Za-z][A-Za-z0-9_]*:([[:space:]]|$)/ { k = $0; sub(/:.*/, "", k); next }
    { printf "%s\t%s\n", (k == "" ? "(nothing above it)" : k), $0 }
  '
}

# One key's line plus its continuation lines, folded back into the single line
# the format requires. Whitespace-collapsed, because the formatter's indentation
# is not part of the value.
rejoin_key() { # $1: key; stdin: a header block
  awk -v want="$1" '
    /^[[:space:]]*$/ { next }
    /^[A-Za-z][A-Za-z0-9_]*:([[:space:]]|$)/ {
      k = $0; sub(/:.*/, "", k)
      if (k == want) { out = $0; sub(/[[:space:]]+$/, "", out); collecting = 1; next }
      if (collecting) { exit }
      next
    }
    collecting {
      v = $0; sub(/^[[:space:]]+/, "", v); sub(/[[:space:]]+$/, "", v)
      out = out " " v
    }
    END { if (collecting) print out }
  '
}

boards="$(git diff --cached --name-only --diff-filter=ACM -- "${WB_BOARDS[@]}" 2>/dev/null || true)"
[ -n "$boards" ] || exit 0

while IFS= read -r f; do
  [ -n "$f" ] || continue

  # The header AS IT WILL BE, read from the index rather than the worktree.
  # Those differ exactly when it matters, and the index is what would land.
  hdr="$(git show ":$f" 2>/dev/null | header_block || true)"
  [ -n "$hdr" ] || continue

  # The lines this commit ADDS to this file, with the diff marker removed.
  # `^+++` is the file header, not content.
  added="$(git diff --cached --unified=0 -- "$f" 2>/dev/null |
    grep -E '^\+' | grep -Ev '^\+\+\+' | cut -c2- || true)"
  [ -n "$added" ] || continue

  while IFS= read -r line; do
    [ -n "$line" ] || continue
    case "$line" in
      # value opens with a single quote, contains `''`, and closes with one:
      # YAML single-quote escaping.
      *": '"*"''"*"'") repaired="$(printf '%s' "$line" | sed "s/: '/: \"/; s/'\$/\"/; s/''/'/g")" ;;
      # value contains `\"`: YAML double-quote escaping.
      *': '*'\"'*) repaired="$(printf '%s' "$line" | sed 's/\\"/"/g')" ;;
      *) continue ;;
    esac

    # Inherited breakage is not this commit's to answer for.
    # Herestring, NOT a pipeline -- see the clock guard for the mechanism:
    # under `pipefail` a SIGPIPE'd `printf` makes this test read FALSE, which
    # here means a real escape form is waved through as inherited breakage.
    grep -qxF -- "$line" <<<"$added" || continue

    report_header
    printf '  %s\n' "$f" >&2
    printf '    is:     %s\n' "$line" >&2
    printf '    should: %s\n' "$repaired" >&2
    violations=$((violations + 1))
  done <<EOF
$hdr
EOF

  # CHECK 2 -- see the block comment. A value that has left its key's line.
  detached="$(printf '%s\n' "$hdr" | detached_lines || true)"
  if [ -n "$detached" ]; then
    reported=""
    while IFS="$TAB" read -r key line; do
      [ -n "$line" ] || continue

      # Inherited breakage is not this commit's to answer for -- check 1's rule,
      # for check 1's reason: otherwise one detached value wedges every future
      # heartbeat commit on that board.
      grep -qxF -- "$line" <<<"$added" || continue

      # One report per KEY, however many lines its value sprawls over.
      grep -qxF -- "$key" <<<"$reported" && continue
      reported="$reported
$key"

      report_detached
      printf '  %s\n' "$f" >&2
      printf '    key:      %s\n' "$key" >&2
      printf '%s\n' "$detached" |
        awk -F"$TAB" -v k="$key" '$1 == k { printf "    detached: %s\n", $2 }' >&2

      rejoined="$(printf '%s\n' "$hdr" | rejoin_key "$key" || true)"
      if [ -n "$rejoined" ]; then
        printf '    reads as: %s -> EMPTY\n' "$key" >&2
        printf '    rejoined: %s\n' "$rejoined" >&2
        # A BYTE count (LC_ALL=C), where prettier counts characters. Equal for
        # the ASCII these values are; for non-ASCII it over-estimates, which can
        # only advise a SHORTER line and never wrongly call a long one safe.
        if [ "${#rejoined}" -gt "$PRINTWIDTH" ]; then
          printf '    *** %s chars, over printWidth %s -- REJOINING IS NOT THE FIX.\n' \
            "${#rejoined}" "$PRINTWIDTH" >&2
          printf '        The formatter runs before this guard and will break it again.\n' >&2
          printf '        SHORTEN the value to %s chars or fewer, key and all.\n' "$PRINTWIDTH" >&2
        else
          printf '    (%s chars, inside printWidth %s -- safe to paste back.)\n' \
            "${#rejoined}" "$PRINTWIDTH" >&2
        fi
      fi
      detached_count=$((detached_count + 1))
    done <<EOF
$detached
EOF
  fi
done <<EOF
$boards
EOF

if [ "$violations" -gt 0 ]; then
  cat >&2 <<'EOF'

  The header block looks like YAML frontmatter and is not. It is a
  line-oriented `key: value` block: one line per key, the value is everything
  after the first `: ` to the end of the line, and a single pair of surrounding
  quotes is a display delimiter that the reader strips WITHOUT unescaping.

  So a `"` inside a value is written as a `"`. Escaping it is correct YAML and
  a wrong board -- the reader renders the backslash.

  Copy the `should:` line above. Do not hand-edit around it: the escape is what
  a node that knows YAML writes, so the fix is to stop treating the block as
  YAML, not to find a better escape.

  Rule: the `in-whiteboard` skill, "The header block is NOT YAML".

EOF
fi

if [ "$detached_count" -gt 0 ]; then
  cat >&2 <<'EOF'

  The header block is one line per key. `fm_get` reads everything after the
  first `: ` on the KEY's line, so a value sitting on its own line reads as
  EMPTY and `ws list` prints nothing for that key. Run-verified 2026-09-12:
  a board with four claims detached rendered `claims=`, every other field
  perfect. The value does not degrade, it disappears -- a node claiming four
  work packages reads as a node claiming none.

  YOU PROBABLY DID NOT WRITE THIS. prettier formats this block as YAML, and a
  bracketed `claims:` list longer than its printWidth is broken across lines --
  by a gate that runs `prettier --write` before its guards, or by the `prettier
  --write` run to clear a `--check` refusal. Measured at printWidth 80: 80 chars
  survives, 81 breaks. A QUOTED value such as `focus:` is never broken however
  long, because a quoted scalar is not a breakable construct.

  SO SHORTEN THE VALUE; DO NOT JUST REJOIN IT. A rejoined line over printWidth
  is broken again by the next commit and the board is wedged by its own repair.
  The `rejoined:` line above carries its length and says which case it is.

  Rule: the `in-whiteboard` skill, "The header block is NOT YAML" -- one line
  per key, no continuation lines.

EOF
fi

if [ $((violations + detached_count)) -gt 0 ]; then
  exit 1
fi

exit 0
