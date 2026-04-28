#!/usr/bin/env bash
#
# release-gate.sh -- /in-session UserPromptSubmit gate releaser
#
# Purpose:
#   Touch the per-session sentinel file that require-in-session.sh checks.
#   Extracted from SKILL.md inline so the awk pipeline survives skill-renderer
#   token-stripping. (The renderer was silently dropping `$1` from
#   `awk '{print $1}'`, producing a malformed project_key with a literal space
#   plus byte-count suffix. State-file lookup then failed, no real sentinel
#   was written, and the gate fired on every prompt.)
#
# Resolution priority for the session_id to release:
#   1. Per-project state file: /tmp/intent-claude-session-current-id-<cksum>
#      written by SessionStart hook (session-context.sh).
#   2. Legacy shared state file: /tmp/intent-claude-session-current-id (best
#      effort for sessions that predate the per-project hook).
#   3. unknown.sentinel: always touched as a final fallback.
#
# Output: one stdout line summarising what got touched. Exit 0 always; this
# script must never block the user.

set -u

SENTINEL_DIR="/tmp/intent"
mkdir -p "$SENTINEL_DIR"

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"

cksum_output="$(printf '%s' "$project_dir" | cksum 2>/dev/null || true)"
project_key="${cksum_output%% *}"

sid=""
sid_legacy=""

if [ -n "$project_key" ]; then
  sid="$(cat "/tmp/intent-claude-session-current-id-${project_key}" 2>/dev/null || true)"
fi
sid_legacy="$(cat /tmp/intent-claude-session-current-id 2>/dev/null || true)"

[ -n "$sid" ] && touch "${SENTINEL_DIR}/in-session-${sid}.sentinel"
if [ -n "$sid_legacy" ] && [ "$sid_legacy" != "$sid" ]; then
  touch "${SENTINEL_DIR}/in-session-${sid_legacy}.sentinel"
fi
touch "${SENTINEL_DIR}/in-session-unknown.sentinel"

echo "released sentinels: per-project=${sid:-(none)} legacy=${sid_legacy:-(none)} +unknown"
exit 0
