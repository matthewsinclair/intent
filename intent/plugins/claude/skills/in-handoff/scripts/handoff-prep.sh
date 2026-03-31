#!/bin/bash
# handoff-prep.sh -- Prepare filename and context for a handoff document.
# Outputs key=value pairs for the calling LLM to consume.
# bash 3.x compatible. No external dependencies beyond git.
#
# Usage:
#   handoff-prep.sh [slug]
#
# Example:
#   handoff-prep.sh add-handoff-skill
#   handoff-prep.sh                    # defaults slug to "session"

set -euo pipefail

# ---- Slug ----

SLUG="${1:-session}"
# Sanitize: lowercase, spaces/underscores to hyphens, strip non-alnum/hyphen, collapse runs
SLUG="$(echo "$SLUG" | tr '[:upper:]' '[:lower:]' | tr ' _' '-' | sed 's/[^a-z0-9-]//g' | sed 's/--*/-/g' | sed 's/^-//;s/-$//')"
if [ -z "$SLUG" ]; then
  SLUG="session"
fi

# ---- Date ----

TODAY="$(date +%Y%m%d)"

# ---- Directory ----

HANDOFF_DIR="intent/.handoff"
if [ ! -d "$HANDOFF_DIR" ]; then
  mkdir -p "$HANDOFF_DIR"
fi

# ---- Sequence number ----

SEQ=1
for f in "$HANDOFF_DIR/${TODAY}"-*; do
  # Skip if glob did not match (bash 3.x returns literal when no match)
  if [ ! -f "$f" ]; then
    continue
  fi
  # Extract NNN from YYYYMMDD-NNN-slug.md
  fname="$(basename "$f")"
  num="$(echo "$fname" | sed 's/^[0-9]*-\([0-9]*\)-.*/\1/')"
  num="$((10#$num))"
  if [ "$num" -ge "$SEQ" ]; then
    SEQ=$((num + 1))
  fi
done

# Zero-pad to 3 digits
if [ "$SEQ" -lt 10 ]; then
  SEQ_PAD="00${SEQ}"
elif [ "$SEQ" -lt 100 ]; then
  SEQ_PAD="0${SEQ}"
else
  SEQ_PAD="$SEQ"
fi

# ---- Proposed filename ----

HANDOFF_FILE="${HANDOFF_DIR}/${TODAY}-${SEQ_PAD}-${SLUG}.md"

# ---- Git context ----

BRANCH=""
GIT_DIFF_STAT=""
GIT_LOG_RECENT=""

if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  BRANCH="$(git branch --show-current 2>/dev/null || git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")"
  GIT_DIFF_STAT="$(git diff --stat HEAD 2>/dev/null || echo "")"
  GIT_LOG_RECENT="$(git log --oneline -20 2>/dev/null || echo "")"
fi

# ---- Output ----

echo "HANDOFF_FILE=${HANDOFF_FILE}"
echo "TODAY=${TODAY}"
echo "SEQ=${SEQ_PAD}"
echo "SLUG=${SLUG}"
echo "BRANCH=${BRANCH}"
echo "---GIT_DIFF_STAT---"
echo "$GIT_DIFF_STAT"
echo "---END_GIT_DIFF_STAT---"
echo "---GIT_LOG_RECENT---"
echo "$GIT_LOG_RECENT"
echo "---END_GIT_LOG_RECENT---"
