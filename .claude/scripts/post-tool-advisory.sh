#!/usr/bin/env bash
#
# post-tool-advisory.sh -- Intent PostToolUse advisory (opt-in, non-blocking)
#
# Purpose:
#   Run `intent critic` against a single edited file when all opt-in
#   conditions are met. Emits findings as a system-reminder. Exits 0 always
#   (never blocks tool use).
#
# Status:
#   SHIPS with the canonical `.claude/` template but is NOT referenced by
#   the default `settings.json` stanza (ST0035 decision #4 -- off by default).
#
# Opt in:
#   1. Set `post_tool_use_advisory: true` in `.intent_critic.yml`.
#   2. Add a PostToolUse hook stanza pointing here in your own
#      `.claude/settings.local.json`, e.g.:
#
#      "PostToolUse": [
#        {
#          "matcher": "Write|Edit|MultiEdit",
#          "hooks": [
#            { "type": "command",
#              "command": "/Users/matts/Devel/prj/Intent/lib/templates/.claude/scripts/post-tool-advisory.sh" }
#          ]
#        }
#      ]
#
# Contract:
#   - Invoked by PostToolUse hook (when user opts in).
#   - Receives tool-use JSON on stdin (includes tool_name, tool_input.file_path).
#   - Exit 0 always.

set -u

# Belt-and-braces: regardless of what happens, never block the tool call.
trap 'exit 0' EXIT

command -v jq >/dev/null 2>&1 || exit 0
[ -t 0 ] && exit 0

payload="$(cat)"
[ -z "$payload" ] && exit 0

tool_name="$(printf '%s' "$payload" | jq -r '.tool_name // empty' 2>/dev/null || true)"
case "$tool_name" in
  Write|Edit|MultiEdit) ;;
  *) exit 0 ;;
esac

file_path="$(printf '%s' "$payload" | jq -r '.tool_input.file_path // empty' 2>/dev/null || true)"
[ -z "$file_path" ] && exit 0
[ -f "$file_path" ] || exit 0

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"
config="$project_dir/.intent_critic.yml"
[ -f "$config" ] || exit 0
grep -qE '^[[:space:]]*post_tool_use_advisory:[[:space:]]*true' "$config" 2>/dev/null || exit 0

case "$file_path" in
  *.ex|*.exs)         lang="elixir" ;;
  *.rs)               lang="rust" ;;
  *.swift)            lang="swift" ;;
  *.lua)              lang="lua" ;;
  *.sh|*.bash|*.zsh)  lang="shell" ;;
  *) exit 0 ;;
esac

command -v intent >/dev/null 2>&1 || exit 0

# `intent critic` lands in ST0035/WP05. If the subcommand isn't present
# yet, the `|| true` swallows failure and `[ -z "$findings" ]` exits.
findings="$(intent critic "$lang" --files "$file_path" --severity-min warning --format text 2>/dev/null || true)"
[ -z "$findings" ] && exit 0

printf 'Intent critic advisory (%s, %s):\n%s\n' "$lang" "$file_path" "$findings"
exit 0
