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
#   - Pass-through (exit 0) when EITHER:
#       a) the prompt is a slash command (starts with `/`) -- so the user
#          can run `/in-session`, `/help`, `/compact`, etc. without being
#          blocked by the gate. The prompt is read with jq when jq is on
#          PATH, and from the raw payload when it is not, so a machine
#          without jq can still run `/in-session` -- OR
#       b) the per-session sentinel exists.
#   - Fail open (exit 0) when the skill that releases the gate is not
#     installed on this machine: the gate is cooperative, so
#     with no releaser nothing inside the session could ever release it.
#     The first such prompt prints one JSON object naming the install verb
#     (systemMessage for the user, additionalContext for the model) and
#     writes the sentinel, so later prompts pass silently.
#   - Block (exit 2 + stderr message) when the sentinel is absent AND the
#     prompt is not a slash command AND the releaser is installed.
#   - Stderr is surfaced to the user by Claude Code on exit 2. On exit 0 it
#     reaches nobody, and plain stdout reaches only the model, which is why
#     the fail-open notice is JSON.

# set -u only: -e and -o pipefail are deliberately omitted so a jq/cat hiccup
# cannot turn this gate hook into a hard abort. The script decides pass/block
# explicitly via exit codes; an unexpected abort would block every prompt.
set -u

# Bypass for non-interactive automation that spawns `claude -p` against an
# Intent project. Such sessions have no chat surface
# for `/in-session` to run in, so the gate would block them indefinitely.
# Wrappers set this env var before invoking `claude -p`.
if [ -n "${INTENT_SKIP_IN_SESSION_GATE:-}" ]; then
  exit 0
fi

SENTINEL_DIR="/tmp/intent"

payload=""
if ! [ -t 0 ]; then
  payload="$(cat)"
fi

# Session identity comes from the env var Claude Code exports into the hook,
# NOT the payload. release-gate.sh (run by /in-session) resolves the same
# $CLAUDE_CODE_SESSION_ID, so the check path and the release path agree by
# construction. When absent, both sides fall back to the same `unknown`
# sentinel. The payload is parsed only for the prompt (slash-command
# passthrough below).
session_id="${CLAUDE_CODE_SESSION_ID:-unknown}"
prompt=""
jq_absent=""
command -v jq >/dev/null 2>&1 || jq_absent=1
if [ -n "$payload" ] && [ -z "$jq_absent" ]; then
  prompt="$(printf '%s' "$payload" | jq -r '.prompt // empty' 2>/dev/null || true)"
elif [ -n "$payload" ]; then
  # Without jq the one question this gate asks of the payload -- does the
  # prompt open with `/` -- is answered from the raw JSON. A quote inside a
  # string value arrives escaped (`\"prompt\"`), which this pattern cannot
  # match, so prompt text cannot forge the key.
  slash_re='"prompt"[[:space:]]*:[[:space:]]*"(/|\\/)'
  if [[ "$payload" =~ $slash_re ]]; then
    prompt="/"
  fi
fi

case "$prompt" in
  /*) exit 0 ;;
esac

sentinel="${SENTINEL_DIR}/in-session-${session_id}.sentinel"

if [ -f "$sentinel" ]; then
  exit 0
fi

# The release is `bash "$HOME/.claude/skills/in-session/scripts/release-gate.sh"`
# (in-session SKILL.md), so the gate checks that same file rather than
# SKILL.md: the check and the release read one path. session-context.sh
# checks it too, to say the same thing at session start.
releaser="${HOME:-}/.claude/skills/in-session/scripts/release-gate.sh"
if [ ! -f "$releaser" ]; then
  mkdir -p "$SENTINEL_DIR" 2>/dev/null && touch "$sentinel" 2>/dev/null
  # The backticks are the notice's own markdown, not a substitution. The text
  # carries no quote or backslash, so it is JSON-safe as written.
  # shellcheck disable=SC2016
  notice='The Intent prompt gate is off on this machine: /in-session, the skill that releases it, is not installed. `intent claude skills install in-session` installs it.'
  printf '{"systemMessage":"%s","hookSpecificOutput":{"hookEventName":"UserPromptSubmit","additionalContext":"%s"}}\n' "$notice" "$notice"
  exit 0
fi

cat >&2 <<EOM
Intent project: /in-session must run before your first prompt.
Run /in-session now -- it loads project coding standards and releases this gate.
(Expected sentinel: ${sentinel})
EOM
if [ -n "$jq_absent" ]; then
  printf '%s\n' "note: jq is not on PATH. This gate works without it; the opt-in post-tool hooks do nothing without it and the pre-commit gate runs no language critic (docs/install.md)." >&2
fi
exit 2
