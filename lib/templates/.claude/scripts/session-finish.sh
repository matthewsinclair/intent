#!/usr/bin/env bash
#
# session-finish.sh -- Intent Stop hook
#
# Purpose:
#   Name the uncommitted paths at the end of a turn, when there are any. It
#   gives no instruction: Claude Code runs a Stop hook at the end of every
#   turn, and `/in-finish` is a session-end step.
#
# Contract:
#   - Invoked by `.claude/settings.json` Stop hook, via `intent claude hook
#     session-finish`.
#   - Receives Claude Code Stop event JSON on stdin (unused).
#   - Writes one JSON object to stdout, `{"systemMessage": "<paths>"}`, or
#     nothing.
#     Claude Code shows a hook's `systemMessage` in the transcript
#     (code.claude.com/docs/en/hooks-guide: "Message shown in transcript").
#     Plain stdout is added to context only for UserPromptSubmit,
#     UserPromptExpansion, SessionStart and PostModelSwitch, and the docs do not
#     say where a Stop hook's plain stdout goes, so a line this hook printed
#     as plain text may reach nobody.
#   - EXIT 0 ALWAYS, AND NEVER 2. This is a hard contract, not a default --
#     see below.
#
# WHY EXIT 2 IS FORBIDDEN HERE, SPECIFICALLY:
#   Claude Code's Stop hook reads 2 as "refuse to stop", measured as a hang
#   with no output (vc, 2026-08-16; the table lives in
#   `native/rust/crates/intent-cli/src/spine.rs`). `2` means different
#   things to different consumers of this binary, so there is no globally right
#   value; what keeps the Claude Code side safe is that `intent claude hook`
#   DELEGATES -- it execs this script, so every code a hook consumer sees is
#   this script's own. That makes the exit code this file's responsibility
#   rather than the CLI's.
#
#   This hook was an inline `echo` in settings.json until it was routed here.
#   The echo was safe only by accident of being an echo; routing it is what
#   lets the body become conditional, and conditional logic is exactly what
#   could reach a non-zero exit. So the trade is explicit: the feature that
#   makes routing worth doing is the same one that makes this contract
#   mandatory.
#
# Target runtime: < 100ms. All git calls are best-effort.

# set -u only. -e is deliberately omitted: an unreadable git dir or a missing
# git binary must not abort the hook, because aborting mid-script is how a
# non-zero status would escape.
set -u

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"

# Best-effort dirty check. Only a dirty tree prints anything:
#   dirty      -> name the uncommitted paths, the first few and a count of the
#                 rest, and give no instruction. Claude Code runs a Stop hook at
#                 the end of EVERY TURN, not at the end of a session, so an
#                 instruction here is repeated at every turn. `/in-finish`
#                 commits, and a reader who reviews and commits by hand must not
#                 be told to run it after each turn. It stays a session-end
#                 step, which the skill and CLAUDE.md say.
#   clean      -> say nothing; a line with nothing behind it is noise.
#   unknowable -> say nothing; not a git tree, or git absent. With no
#                 instruction left to give, there is no fact to report either.
if ! command -v git >/dev/null 2>&1 \
  || ! git -C "$project_dir" rev-parse --is-inside-work-tree >/dev/null 2>&1
then
  exit 0
fi

dirty="$(git -C "$project_dir" status --porcelain 2>/dev/null || true)"
[ -n "$dirty" ] || exit 0

# The number of paths the message names before it counts the rest.
shown_max=5
count=0
shown=""
while IFS= read -r line; do
  [ -n "$line" ] || continue
  count=$((count + 1))
  if [ "$count" -le "$shown_max" ]; then
    # Porcelain v1: two status columns and a space, then the path. git quotes
    # a path with an unusual character in C style, so only a backslash and a
    # double quote need escaping for JSON.
    path="$(printf '%s' "${line:3}" | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g')"
    shown="${shown:+$shown, }$path"
  fi
done <<<"$dirty"

rest=""
if [ "$count" -gt "$shown_max" ]; then
  rest=", and $((count - shown_max)) more"
fi

# Built with printf rather than jq, which this hook must not depend on.
printf '{"systemMessage":"%s uncommitted path(s): %s%s."}\n' "$count" "$shown" "$rest"

exit 0
