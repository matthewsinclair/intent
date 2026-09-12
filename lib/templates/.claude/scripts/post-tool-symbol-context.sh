#!/usr/bin/env bash
#
# post-tool-symbol-context.sh -- Intent PostToolUse index advisory
#
# Purpose:
#   After a Grep whose pattern is one symbol, append what the index knows about
#   that symbol: where it is defined and where its name occurs, as source spans.
#   The grep has already run and its result stands; this only ever ADDS.
#
# Status:
#   SHIPS with the canonical `.claude/` template and is NOT referenced by the
#   default `settings.json` stanza -- off by default, like `post-tool-advisory`.
#   Turning it on changes what every session in a project sees after every Grep,
#   so it is the project's decision rather than one taken on its behalf by an
#   upgrade.
#
# Opt in:
#   Add a PostToolUse stanza pointing here in your own
#   `.claude/settings.local.json`:
#
#     "PostToolUse": [
#       {
#         "matcher": "Grep",
#         "hooks": [
#           { "type": "command",
#             "command": "intent claude hook post-tool-symbol-context",
#             "timeout": 5000 }
#         ]
#       }
#     ]
#
# Contract:
#   - Invoked by PostToolUse with the tool-use JSON on stdin.
#   - Exit 0 ALWAYS. It never blocks, never replaces, and its worst outcome is
#     saying nothing.
#
# WHY AN APPEND AND NOT A REDIRECT. A PreToolUse redirect would replace the
# model's own tool call before it sees any result, so it can be wrong in a way
# the model cannot detect. Intent does not ship one. This form shows a better
# answer after the fact; the worst it can do is be ignored.
#
# THREE THINGS IT WILL NOT DO, each a way the rule could be lost:
#
#   1. A PATTERN THAT IS NOT SYMBOL-SHAPED IS NEVER ANSWERED. grep's job is
#      literal and regex text; the index answers about NAMES. Guessing at a
#      regex would answer a question the caller did not ask, quietly. One
#      identifier, bare or wrapped in word anchors, and nothing else.
#   2. IT APPENDS NOTHING WHEN THE INDEX CANNOT ANSWER FOR THE PATHS INVOLVED.
#      Not `index.complete` -- see `index-freshness.bash` for why that field is
#      the WHOLE index's claim and not this rule. One function, so no second
#      reading of *can the index answer this* can grow beside it.
#   3. IT DOES NOT RECONCILE. `--context` answers from the index as it stands and
#      never walks the tree, so a hook on every Grep cannot turn into a repository
#      walk on every Grep. Staleness is not hidden by that -- it is exactly what
#      condition 2 reads.

set -u

# Belt-and-braces: regardless of what happens below, never block the tool call.
trap 'exit 0' EXIT

command -v jq >/dev/null 2>&1 || exit 0
command -v intent >/dev/null 2>&1 || exit 0
[ -t 0 ] && exit 0

payload="$(cat)"
[ -z "$payload" ] && exit 0

tool_name="$(printf '%s' "$payload" | jq -r '.tool_name // empty' 2>/dev/null || true)"
[ "$tool_name" = "Grep" ] || exit 0

pattern="$(printf '%s' "$payload" | jq -r '.tool_input.pattern // empty' 2>/dev/null || true)"
[ -z "$pattern" ] && exit 0

# **ONE IDENTIFIER, BARE OR IN WORD ANCHORS** (vc, 2026-09-12). The anchors are
# how a careful caller greps for a symbol rather than a substring, so refusing
# them would decline exactly the patterns most likely to BE a symbol lookup.
symbol="$pattern"
symbol="${symbol#\\b}"; symbol="${symbol%\\b}"
symbol="${symbol#\\<}"; symbol="${symbol%\\>}"
case "$symbol" in
  [A-Za-z_]*) ;;
  *) exit 0 ;;
esac
case "$symbol" in
  *[!A-Za-z0-9_]*) exit 0 ;;
esac

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"
[ -d "$project_dir" ] || exit 0

answer="$(cd "$project_dir" && intent search --context "$symbol" --json 2>/dev/null || true)"
[ -z "$answer" ] && exit 0

scripts_dir="${BASH_SOURCE[0]%/*}"
# shellcheck source=/dev/null
. "$scripts_dir/index-freshness.bash" 2>/dev/null || exit 0

# **THE PATHS INVOLVED ARE THE PATHS THIS WOULD APPEND, NOT THE PATHS THE GREP
# SEARCHED, AND THE FIRST BUILD OF THIS HOOK HAD IT WRONG.** It read the grep's
# own `path` scope, so a grep confined to `docs/` passed the freshness gate on
# the strength of `docs/` being clean -- and then appended two hits in a
# `src/lib.rs` that had moved underneath the index, spans already dropped. The
# subject of *complete for the paths involved* is whatever the appended answer
# NAMES. That is the difference between this hook and a redirect, which would
# REPLACE the grep and whose paths involved really are the ones the pattern
# would have reached -- which is why the shared predicate takes the prefix as an
# argument instead of deciding the subject for its callers.
#
# **ALL OR NOTHING, PER ANSWER.** One stale path among the hits silences the
# whole append rather than dropping that hit: a partial answer that did not say
# it was partial is a silent subset, which is the defect class this estate names
# most often.
while IFS= read -r hit_path; do
  [ -n "$hit_path" ] || continue
  intent_index_answers_for "$answer" "$hit_path" || exit 0
done <<EOF
$(printf '%s' "$answer" | jq -r '[.groups[]? | .hits[]? | .path] | unique | .[]' 2>/dev/null || true)
EOF

rows="$(printf '%s' "$answer" | jq -r '
  [ .groups[]? | .hits[]? ] as $hits
  | if ($hits | length) == 0 then empty
    else $hits[]
      | ( .path
          + ( if .span then ":" + (.span.start_line | tostring) else "" end )
          + "  " + .kind
          + "  " + .name )
    end
' 2>/dev/null || true)"
[ -z "$rows" ] && exit 0

# The backticks are markdown for the reader, so the header is assembled first
# rather than written into a format string where they read as substitution.
header="Intent index -- \`$symbol\` (from \`intent search --context $symbol\`):"
printf '%s\n%s\n' "$header" "$rows"
exit 0
