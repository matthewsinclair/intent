#!/usr/bin/env bash
#
# require-in-session.sh -- Intent UserPromptSubmit strict gate
#
# Purpose:
#   Block the first user prompt in a session until `/in-session` has been
#   run. Releases once a per-session sentinel file is present. The
#   `/in-session` skill writes the sentinel in its final step (cooperating
#   handoff).
#
# Contract:
#   - Invoked by `.claude/settings.json` UserPromptSubmit hook.
#   - Receives Claude Code UserPromptSubmit event JSON on stdin (includes
#     session_id and prompt text).
#   - Pass-through (exit 0) when:
#       a) the prompt is a slash command (starts with `/`) -- so the user
#          can run `/in-session`, `/help`, `/compact`, etc. without being
#          blocked by the gate, AND
#       b) when the per-session sentinel exists.
#   - Block (exit 2 + stderr message) when the sentinel is absent AND the
#     prompt is not a slash command.
#   - Stderr is surfaced to the user by Claude Code.

set -u

SENTINEL_DIR="/tmp/intent"

payload=""
if ! [ -t 0 ]; then
  payload="$(cat)"
fi

session_id="unknown"
prompt=""
if [ -n "$payload" ] && command -v jq >/dev/null 2>&1; then
  sid="$(printf '%s' "$payload" | jq -r '.session_id // empty' 2>/dev/null || true)"
  [ -n "$sid" ] && session_id="$sid"
  prompt="$(printf '%s' "$payload" | jq -r '.prompt // empty' 2>/dev/null || true)"
fi

case "$prompt" in
  /*) exit 0 ;;
esac

sentinel="${SENTINEL_DIR}/in-session-${session_id}.sentinel"

if [ -f "$sentinel" ]; then
  exit 0
fi

cat >&2 <<EOM
Intent project: /in-session must run before your first prompt.
Run /in-session now -- it loads project coding standards and releases this gate.
(Expected sentinel: ${sentinel})
EOM
exit 2
