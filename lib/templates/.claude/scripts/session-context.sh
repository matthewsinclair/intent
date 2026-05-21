#!/usr/bin/env bash
#
# session-context.sh -- Intent SessionStart hook
#
# Purpose:
#   Emit Intent project context as a system-reminder to Claude Code so every
#   session knows which project, branch, and active steel thread it is
#   resuming.
#
# Contract:
#   - Invoked by `.claude/settings.json` SessionStart hook.
#   - Receives Claude Code SessionStart event JSON on stdin (unused).
#   - Writes to stdout; Claude Code injects the output as a system-reminder.
#   - Exit 0 always. Never blocks.
#   - Target runtime: < 200ms. All git/file calls are best-effort.
#
# Session identity:
#   This hook no longer persists the session_id anywhere. The cooperating
#   `/in-session` gate release (release-gate.sh) and the gate itself
#   (require-in-session.sh) both resolve identity from $CLAUDE_CODE_SESSION_ID,
#   the env var Claude Code exports. The earlier per-project state file was a
#   shared mutable bridge that concurrent sessions in one project stomped,
#   deadlocking the gate. It is gone.

# set -u only: all git/file calls below are best-effort and the hook must
# exit 0 regardless (see header). -e and -o pipefail are omitted so a missing
# git or unreadable wip.md cannot abort the context emission.
set -u

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"
project_name="$(basename "$project_dir" 2>/dev/null || echo unknown)"

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
