#!/usr/bin/env bash
#
# session-finish.sh -- Intent Stop hook
#
# Purpose:
#   Remind the operator to fold the session (`/in-finish`) when there is
#   actually something to fold.
#
# Contract:
#   - Invoked by `.claude/settings.json` Stop hook, via `intent claude hook
#     session-finish`.
#   - Receives Claude Code Stop event JSON on stdin (unused).
#   - Writes to stdout.
#   - EXIT 0 ALWAYS, AND NEVER 2. This is a hard contract, not a default --
#     see below.
#
# WHY EXIT 2 IS FORBIDDEN HERE, SPECIFICALLY:
#   Claude Code's Stop hook reads 2 as "refuse to stop", measured as a 24s hang
#   with zero output (vc, five arms, 2026-08-16; the table lives in
#   `native/rust/crates/intent-cli/src/spine.rs`). `2` means four different
#   things to four consumers of this binary, so there is no globally right
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
#   mandatory. `tests/unit/session_finish_hook.bats` holds it.
#
# Target runtime: < 100ms. All git calls are best-effort.

# set -u only. -e is deliberately omitted: an unreadable git dir or a missing
# git binary must not abort the hook, because aborting mid-script is how a
# non-zero status would escape.
set -u

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"

# Best-effort dirty check. Three outcomes and only one of them is silent:
#   dirty      -> remind; there is work to fold
#   clean      -> say nothing; a reminder with nothing behind it is noise, and
#                 a hook that fires identically every time carries no signal
#   unknowable -> remind; not a git tree, or git absent. Defaulting to the
#                 reminder keeps the pre-routing behaviour whenever the
#                 condition cannot be evaluated, so routing never makes a
#                 project quieter than the inline echo was.
dirty_count=""
if command -v git >/dev/null 2>&1 \
  && git -C "$project_dir" rev-parse --is-inside-work-tree >/dev/null 2>&1
then
  dirty_count="$(git -C "$project_dir" status --porcelain 2>/dev/null | wc -l | tr -d '[:space:]' || true)"
fi

if [ -n "$dirty_count" ] && [ "$dirty_count" = "0" ]; then
  exit 0
fi

if [ -n "$dirty_count" ]; then
  printf 'Session wrap-up reminder: %s uncommitted path(s). Run /in-finish to update ST docs, intent/wip.md, and prepare a clean commit.\n' "$dirty_count"
else
  printf 'Session wrap-up reminder: run /in-finish to update ST docs, intent/wip.md, and prepare a clean commit.\n'
fi

exit 0
