#!/usr/bin/env bash
#
# whiteboard-header-guard.sh -- refuse a commit that YAML-ESCAPES a value in a
# whiteboard header block. The block is NOT YAML, and escaping it is the one way
# to get that wrong while looking careful.
#
# WHY THIS IS A SEPARATE FILE FROM whiteboard-clock-guard.sh, and it was ruled
# rather than assumed (vc, 2026-08-16). That guard's name and contract are
# TIMESTAMPS -- three checks on stamps, and documentation entirely about clocks.
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
# node's last 25 revisions found four INVALID headers in two episodes, "all of
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
# 2026-08-16: that pathspec alone matches 21 files, SIXTEEN of them archived
# boards; with the exclude, 5 -- exactly the live boards. An archive replays an
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
# file. RULED 2026-08-16 (vc, verified against `intent_claude_cwi:86`): the
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
# Exit codes: 0 clean or not applicable; 1 an added header value is escaped.

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

report_header() {
  if [ "$violations" -eq 0 ]; then
    echo "" >&2
    echo "BLOCKED: a whiteboard header value has been YAML-escaped." >&2
    echo "         The header block is NOT YAML -- quotes inside a value are literal." >&2
    echo "" >&2
  fi
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
  exit 1
fi

exit 0
