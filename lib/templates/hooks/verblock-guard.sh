#!/usr/bin/env bash
#
# verblock-guard.sh -- A VERBLOCK IS THE VERSION AND ONE SHORT LINE. IT NEVER
# WRITES.
#
# WHAT IT IS FOR. hv's rule, which the rule library holds verbatim as
# IN-PR-STYLE-005: "The verblock just has 1-line (or less) text plus the
# version and NEVER has history in it. That is what version control is for!"
# (2026-10-12), and "THE VERBLOCK IS NOT A REPLACEMENT FOR VERSION CONTROL. DO
# NOT STORE PROSE IN THE VERBLOCK." (2026-10-26). The rule lived in other
# estates' memories and in no file Intent shipped, so a fold that rewrote a
# hand-authored document was free to append its narrative to the verblock, and
# folds did: a live work-in-progress file carried a fold's whole account in its
# verblock, and two as-built notes carried their earlier versions beside the
# current one. A rule in the library refuses nothing by itself, because the
# headless critic reads no Markdown. This is the half that refuses.
#
# TWO REFUSALS, EACH A FACT ABOUT ONE STAGED VALUE:
#   * A SECOND VERSION STAMP. A stamp is a version token -- `v`, digits and at
#     least one dotted group -- that a colon precedes or follows, which is the
#     shape every verblock in the corpus wears: `2026-10-25:v3.24: vc - ...`,
#     `19 Sep 2026:v0.4: ...`, `2026-10-25: v1.2 - ...`. Two of them is an
#     earlier version kept beside the current one, which is history. A DATE IS
#     NOT A STAMP: `added the 2026-10-16 entry` is a line about this version,
#     and a live verblock says exactly that.
#   * A VALUE LONGER THAN 100 CHARACTERS, measured with one pair of surrounding
#     quotes removed, and in CHARACTERS rather than bytes, so an em dash costs
#     one. Every verblock in Intent's own tree was 77 or shorter once it was cut
#     to the rule, so the limit leaves room for a real line and none for prose.
#
# IT READS THE STAGED BLOB'S FRONTMATTER AND NOTHING ELSE. A verblock is the
# `verblock:` key at the top level of the leading `---` block, with any
# indented or listed continuation joined to it. A `verblock:` in a document's
# body, in a code fence or under another key is prose about verblocks, not
# one. Two `verblock:` keys are counted together, since two stamps are history
# whichever key carries them.
#
# GOING-FORWARD ONLY BY CONSTRUCTION, NOT BY EXEMPTION (hv: "This is
# going-forward only."). It reads only what a commit adds, copies or modifies,
# the `--diff-filter=ACM` that `staged-format-guard.sh` reads, so a record
# nobody restages is never read and keeps the verblock it was written with,
# and a rename moves a record without judging it. It reads the STAGED blob for
# the reason that guard gives: a hunk-scoped commit is judged on the bytes it
# commits, not on the worktree.
#
# THE REMEDY IS AN EDIT, AND THE SHORTEST ONE IS hv's: "Just elide them at 97
# chars if you need to." (2026-10-26). Ninety-seven characters and `...` make
# the hundred, and the refusal prints both numbers from `LIMIT`.
#
# IT JUDGES AND NEVER WRITES. A guard that elided the staged value itself
# would commit bytes nobody staged, which is the re-staging class hv ruled
# out on 2026-09-21.

set -u

NAME="verblock-guard"
RULE="IN-PR-STYLE-005"
LIMIT=100

note() { printf '%s: %s\n' "$NAME" "$*" >&2; }

# The top-level `verblock:` values of a document's leading frontmatter, read
# from stdin, one per line, with continuation lines joined by a space.
values_of() {
  awk '
    function trim(s) { sub(/^[ \t]+/, "", s); sub(/[ \t]+$/, "", s); return s }
    function flush() { if (on) print trim(val); on = 0; val = "" }
    { sub(/\r$/, "") }
    NR == 1 { if ($0 !~ /^---[ \t]*$/) exit; next }
    /^(---|\.\.\.)[ \t]*$/ { flush(); exit }
    on && /^([ \t]|- )/ { val = val " " trim($0); next }
    { flush() }
    /^verblock[ \t]*:/ { on = 1; val = $0; sub(/^verblock[ \t]*:/, "", val) }
    END { flush() }
  '
}

# The version stamps in one value, space-separated. The leading space lets a
# stamp at the very start of a value meet the left boundary.
stamps_in() {
  printf ' %s\n' "$1" | awk '
    {
      s = $0
      while (match(s, /:[ ]?v[0-9]+(\.[0-9]+)+|[^A-Za-z0-9]v[0-9]+(\.[0-9]+)+:/)) {
        m = substr(s, RSTART, RLENGTH)
        s = substr(s, RSTART + RLENGTH)
        match(m, /v[0-9]+(\.[0-9]+)+/)
        printf "%s%s", (n++ ? " " : ""), substr(m, RSTART, RLENGTH)
      }
    }'
}

# One pair of surrounding quotes removed, as YAML reads a quoted scalar.
unquoted() {
  local v="$1"
  case "$v" in
    \"*\") v="${v#\"}"; v="${v%\"}" ;;
    \'*\') v="${v#\'}"; v="${v%\'}" ;;
  esac
  printf '%s' "$v"
}

# Characters, not bytes: a UTF-8 continuation byte is never counted.
chars() { printf '%s' "$1" | LC_ALL=C tr -d '\200-\277' | wc -c | tr -d ' '; }

BAD=""
UNREAD=""
FILES=0
CHECKED=0

while IFS= read -r -d '' f; do
  [ -n "$f" ] || continue
  if ! blob="$(git show ":$f" 2>/dev/null)"; then
    UNREAD="$UNREAD
  $f"
    continue
  fi
  FILES=$((FILES + 1))
  found=""
  while IFS= read -r v; do
    [ -n "$v" ] || continue
    CHECKED=$((CHECKED + 1))
    n="$(chars "$(unquoted "$v")")"
    if [ "$n" -gt "$LIMIT" ]; then
      BAD="$BAD
  $f: its verblock is $n characters, and $LIMIT is the most a verblock carries"
    fi
    found="$found $(stamps_in "$v")"
  done <<EOF
$(printf '%s\n' "$blob" | values_of)
EOF
  count="$(printf '%s\n' "$found" | awk '{ print NF }')"
  if [ "$count" -gt 1 ]; then
    list="$(printf '%s\n' "$found" | awk '{ for (i = 1; i <= NF; i++) printf "%s%s", (i > 1 ? ", " : ""), $i }')"
    BAD="$BAD
  $f: its verblock carries $count version stamps ($list), and a verblock carries one"
  fi
done < <(git diff --cached --name-only --diff-filter=ACM -z -- '*.md' 2>/dev/null || true)

if [ -n "$UNREAD" ]; then
  note "could not read the staged blob of these files, which is NOT a verdict on their verblocks:$UNREAD"
fi
if [ -n "$BAD" ]; then
  note "REFUSED -- a verblock in the staged Markdown carries history or prose ($RULE):$BAD"
  note "  fix:   keep one version stamp, the current one; earlier versions live in git"
  note "         a line that will not fit in $LIMIT characters is elided at $((LIMIT - 3)) and ends in ..."
  note "  then:  git add on those paths, and commit again"
  exit 1
fi
# Said only when a verblock was read: a commit staging Markdown that carries
# none, which is every board fold, is in the runner's tally already.
if [ "$CHECKED" -gt 0 ]; then
  note "ok -- $CHECKED verblock(s) in $FILES staged Markdown file(s): none carries a second version stamp or runs past $LIMIT characters."
fi
exit 0
