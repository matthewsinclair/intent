#!/usr/bin/env bash
#
# release-gate.sh -- /in-session UserPromptSubmit gate releaser
#
# Purpose:
#   Touch the per-session sentinel file that require-in-session.sh checks.
#   Extracted from SKILL.md inline so any pipeline survives skill-renderer
#   token-stripping. (The renderer was silently dropping `$1` from
#   `awk '{print $1}'`, producing a malformed lookup key. There is now no
#   awk and no key derivation, so that class is closed by construction.)
#
# Session identity (single authoritative source):
#   $CLAUDE_CODE_SESSION_ID -- exported by Claude Code into every Bash tool
#   invocation. It is the same id require-in-session.sh resolves, so the
#   release path and the check path agree by construction. Earlier designs
#   read a shared per-project state file written by SessionStart; concurrent
#   sessions in one project stomped that file and the release touched the
#   wrong sentinel, deadlocking the gate. The shared file is gone.
#
#   When the env var is absent (degenerate, eg an older Claude Code build),
#   both sides fall back to the same `unknown` sentinel, which this script
#   always touches -- so they still agree and the gate self-heals.
#
# Output: one stdout line summarising what got touched. Exit 0 always; this
# script must never block the user.

# set -u only: this releaser must never abort mid-run (it would leave the gate
# unreleased). -e and -o pipefail are deliberately omitted so a touch/mkdir
# hiccup cannot short-circuit the script before it touches the unknown fallback.
set -u

SENTINEL_DIR="/tmp/intent"
mkdir -p "$SENTINEL_DIR"

sid="${CLAUDE_CODE_SESSION_ID:-}"

[ -n "$sid" ] && touch "${SENTINEL_DIR}/in-session-${sid}.sentinel"
touch "${SENTINEL_DIR}/in-session-unknown.sentinel"

echo "released sentinels: session=${sid:-(none)} +unknown"
exit 0
