#!/usr/bin/env bash
#
# post-tool-symbol-context.sh -- Intent PostToolUse index advisory
#
# Purpose:
#   After a search whose pattern is one symbol, append what the index knows
#   about that symbol: where it is defined and where its name occurs, as source
#   spans. The search is the Grep tool, or a Bash command that runs exactly one
#   `grep`, `rg` or `git grep`. The search has already run and its
#   result stands; this only ever ADDS.
#
# Status:
#   SHIPS with the canonical `.claude/` template and is NOT referenced by the
#   default `settings.json` stanza -- off by default, like `post-tool-advisory`.
#   Turning it on changes what every session in a project sees after every Grep,
#   so it is the project's decision rather than one taken on its behalf by an
#   upgrade.
#
# WHY BASH TOO. Measured across this project's session transcripts
# on 2026-09-16: 3 Grep tool calls against 520 `git grep`s run through Bash and
# thousands of plain `grep` and `rg` commands. A hook that answers only the Grep
# tool almost never runs.
#
# Opt in:
#   Add a PostToolUse stanza pointing here in your own
#   `.claude/settings.local.json`:
#
#     "PostToolUse": [
#       {
#         "matcher": "Grep|Bash",
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
#   - The answer is printed as JSON, `hookSpecificOutput.additionalContext`.
#     PLAIN STDOUT NEVER REACHES THE MODEL, and this hook once
#     printed plain stdout. Driven 2026-09-16 with a headless session: a
#     PostToolUse hook that printed a nonce at exit 0 FIRED (its marker file was
#     written) and the model reported seeing nothing; the same nonce as
#     `additionalContext` was quoted back verbatim.
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
#      identifier, bare or wrapped in word anchors, and nothing else. A Bash
#      command is answered only when it runs ONE search with ONE pattern:
#      several searches, several patterns, a pattern file, or a command built
#      by substitution append nothing, because the question is no longer one
#      name.
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

# bash_search_pattern <command> -- print the one pattern of the one search a
# Bash command runs, or return 1.
#
# **A RECOGNISER, NOT A SHELL PARSER, AND IT FAILS CLOSED.** It splits words on
# quotes and backslashes and commands on `|`, `;`, `&` and newlines, and anything
# it does not follow is declined rather than guessed at: a command substitution,
# a process substitution, a pattern file. The worst a wrong reading can cost is
# an answer about a name nobody asked for, so every doubt resolves to silence.
# The longest Bash command the recogniser will read. See (b) below.
SEARCH_COMMAND_MAX_BYTES=512

# The case patterns below match a literal backslash and a literal `$(`, which
# is what shellcheck's two info notes misread as mistakes.
# shellcheck disable=SC1003,SC2016
bash_search_pattern() {
  local cmd="$1" sep n i c q w have redir
  # **TWO DECLINES BEFORE THE WALK, BECAUSE THE WALK IS QUADRATIC AND THIS RUNS
  # ON EVERY BASH CALL** (vc, measured 2026-09-17). `${cmd:$i:1}` re-scans from
  # the start, so a 6 KB command with no search in it took 0.75 s under
  # Homebrew bash and 3.7 s under /bin/bash 3.2, and 30 KB took 9.2 s, past the
  # hook's timeout. A commit message written through a heredoc is that size.
  # (a) No grep-family word at all: a cheap superset, and the walk still decides.
  case "$cmd" in
    *grep* | *rg*) ;;
    *) return 1 ;;
  esac
  # (b) One search for one symbol is a short command. Longer is declined unread.
  [ "${#cmd}" -le "$SEARCH_COMMAND_MAX_BYTES" ] || return 1
  sep="$(printf '\036')"
  case "$cmd" in
    *'$('* | *'`'* | *'<('* | *'>('*) return 1 ;;
  esac
  local words=()
  n=${#cmd}; i=0; q=""; w=""; have=0; redir=0
  while [ "$i" -lt "$n" ]; do
    c="${cmd:$i:1}"
    if [ -n "$q" ]; then
      if [ "$c" = "$q" ]; then
        q=""
      elif [ "$q" = '"' ] && [ "$c" = '\' ]; then
        # In double quotes a backslash escapes only $ ` " \ -- `\b` stays `\b`.
        case "${cmd:$((i + 1)):1}" in
          '$' | '`' | '"' | '\') i=$((i + 1)); w="$w${cmd:$i:1}" ;;
          *) w="$w$c" ;;
        esac
      else
        w="$w$c"
      fi
    else
      case "$c" in
        "'" | '"') q="$c"; have=1 ;;
        '\') i=$((i + 1)); w="$w${cmd:$i:1}"; have=1 ;;
        '>' | '<') w=""; have=0; redir=1 ;;
        ' ' | "$(printf '\t')" | '|' | ';' | '&' | "$(printf '\n')")
          if [ "$redir" = 1 ] && [ "$c" = '&' ]; then
            w="$w$c"; have=1
          else
            if [ "$have" = 1 ]; then
              [ "$redir" = 1 ] || words+=("$w")
              redir=0
            fi
            w=""; have=0
            case "$c" in ' ' | "$(printf '\t')") ;; *) words+=("$sep") ;; esac
          fi
          ;;
        *) w="$w$c"; have=1 ;;
      esac
    fi
    i=$((i + 1))
  done
  [ -n "$q" ] && return 1
  if [ "$have" = 1 ] && [ "$redir" != 1 ]; then words+=("$w"); fi
  words+=("$sep")

  local searches=0 pattern="" start=1 tool="" k word
  local args=()
  for word in "${words[@]}"; do
    if [ "$word" = "$sep" ]; then
      start=1
      continue
    fi
    if [ "$start" = 1 ]; then
      # The command word, past `VAR=value` prefixes.
      case "$word" in
        [A-Za-z_]*=*) continue ;;
      esac
      start=0
      case "$word" in
        grep | egrep | fgrep | rg) tool="$word" ;;
        git) tool="git" ;;
        *) tool="" ;;
      esac
      [ -n "$tool" ] && searches=$((searches + 1))
    fi
  done
  [ "$searches" -eq 1 ] || return 1

  # Re-walk to find that one search's words, now that there is exactly one.
  args=(); tool=""; start=1
  for word in "${words[@]}"; do
    if [ "$word" = "$sep" ]; then
      [ -n "$tool" ] && break
      start=1
      continue
    fi
    if [ "$start" = 1 ]; then
      case "$word" in [A-Za-z_]*=*) continue ;; esac
      start=0
      case "$word" in
        grep | egrep | fgrep | rg | git) tool="$word" ;;
      esac
      continue
    fi
    [ -n "$tool" ] && args+=("$word")
  done

  if [ "$tool" = git ]; then
    # `git [-C dir | -c k=v | --flag]... grep ...`
    k=0
    while [ "$k" -lt "${#args[@]}" ]; do
      case "${args[$k]}" in
        grep) break ;;
        -C | -c) k=$((k + 2)) ;;
        -*) k=$((k + 1)) ;;
        *) return 1 ;;
      esac
    done
    [ "$k" -lt "${#args[@]}" ] || return 1
    args=("${args[@]:$((k + 1))}")
  fi

  # Options that take a separate argument. `-r` is recursive to grep and
  # `--replace` to rg, so the table is per tool.
  local takes="ABCmefdD"
  [ "$tool" = rg ] && takes="ABCmefgtTjMr"
  local patterns=0 positional="" ended=0 a rest letter
  k=0
  while [ "$k" -lt "${#args[@]}" ]; do
    a="${args[$k]}"
    k=$((k + 1))
    if [ "$ended" = 0 ]; then
      case "$a" in
        --) ended=1; continue ;;
        --regexp=*) patterns=$((patterns + 1)); pattern="${a#--regexp=}"; continue ;;
        --regexp) patterns=$((patterns + 1)); pattern="${args[$k]:-}"; k=$((k + 1)); continue ;;
        --file | --file=*) return 1 ;;
        --*=*) continue ;;
        --after-context | --before-context | --context | --max-count | --include | --exclude | --exclude-dir | --glob | --iglob | --type | --type-not | --threads | --max-columns | --max-depth | --replace | --sort | --sortr | --encoding | --engine | --pre)
          k=$((k + 1)); continue ;;
        --*) continue ;;
        -?*)
          rest="${a#-}"
          while [ -n "$rest" ]; do
            letter="${rest:0:1}"
            rest="${rest:1}"
            case "$takes" in
              *"$letter"*)
                local value="$rest"
                if [ -z "$value" ]; then value="${args[$k]:-}"; k=$((k + 1)); fi
                case "$letter" in
                  e) patterns=$((patterns + 1)); pattern="$value" ;;
                  f) return 1 ;;
                esac
                rest=""
                ;;
            esac
          done
          continue
          ;;
      esac
    fi
    [ -z "$positional" ] && positional="$a" && [ "$patterns" = 0 ] && continue
  done
  if [ "$patterns" = 0 ]; then
    [ -n "$positional" ] || return 1
    pattern="$positional"
  fi
  [ "$patterns" -le 1 ] || return 1
  printf '%s' "$pattern"
}

tool_name="$(printf '%s' "$payload" | jq -r '.tool_name // empty' 2>/dev/null || true)"
case "$tool_name" in
  Grep)
    pattern="$(printf '%s' "$payload" | jq -r '.tool_input.pattern // empty' 2>/dev/null || true)"
    ;;
  Bash)
    command="$(printf '%s' "$payload" | jq -r '.tool_input.command // empty' 2>/dev/null || true)"
    pattern="$(bash_search_pattern "$command")" || exit 0
    ;;
  *) exit 0 ;;
esac
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
jq -n --arg context "$(printf '%s\n%s' "$header" "$rows")" \
  '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $context}}'
exit 0
