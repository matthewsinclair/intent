#!/usr/bin/env bash
#
# session-context.sh -- Intent SessionStart hook
#
# Purpose:
#   Emit Intent project context as a system-reminder to Claude Code so every
#   session knows which project, branch, and active steel thread it is
#   resuming. Also persists the current Claude session_id to a well-known
#   path so the cooperating `/in-session` skill can release the
#   UserPromptSubmit strict gate (see require-in-session.sh).
#
# Contract:
#   - Invoked by `.claude/settings.json` SessionStart hook.
#   - Receives Claude Code SessionStart event JSON on stdin (includes session_id).
#   - Writes to stdout; Claude Code injects the output as a system-reminder.
#   - Exit 0 always. Never blocks.
#   - Target runtime: < 200ms. All git/file calls are best-effort.

set -u

STATE_FILE="/tmp/intent-claude-session-current-id"

capture_session_id() {
  [ -t 0 ] && return 0
  command -v jq >/dev/null 2>&1 || return 0
  local payload sid
  payload="$(cat)" || return 0
  [ -z "$payload" ] && return 0
  sid="$(printf '%s' "$payload" | jq -r '.session_id // empty' 2>/dev/null || true)"
  [ -z "$sid" ] && return 0
  printf '%s' "$sid" > "$STATE_FILE" 2>/dev/null || true
}

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"
project_name="$(basename "$project_dir" 2>/dev/null || echo unknown)"

capture_session_id

git_branch=""
git_sha=""
if command -v git >/dev/null 2>&1 \
  && git -C "$project_dir" rev-parse --is-inside-work-tree >/dev/null 2>&1
then
  git_branch="$(git -C "$project_dir" rev-parse --abbrev-ref HEAD 2>/dev/null || true)"
  git_sha="$(git -C "$project_dir" rev-parse --short HEAD 2>/dev/null || true)"
fi

wip_summary=""
wip_file="$project_dir/intent/wip.md"
if [ -f "$wip_file" ]; then
  wip_summary="$(
    grep -m1 -E '^\*\*ST[0-9]+' "$wip_file" 2>/dev/null \
      | sed -e 's/\*\*//g' -e 's/  */ /g' \
      | cut -c1-160 \
      || true
  )"
fi

printf 'Intent project: %s\n' "$project_name"
if [ -n "$git_branch" ]; then
  printf 'Git: %s @ %s\n' "$git_branch" "${git_sha:-?}"
fi
if [ -n "$wip_summary" ]; then
  printf 'WIP: %s\n' "$wip_summary"
fi
printf 'Next: run /in-session to load coding skills and release the UserPromptSubmit gate.\n'

exit 0
