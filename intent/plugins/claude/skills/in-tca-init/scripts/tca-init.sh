#!/bin/bash
# tca-init.sh - Register a TCA's work packages
# Registers WP/01..WP/NN through `intent wp new`, writes each templated body
# through `intent set`, and creates an empty socrates.md beside each view.
# The last WP is always the synthesis WP.
# bash 3.x compatible. Needs `intent` on PATH.
#
# Usage:
#   tca-init.sh --tca-dir PATH --wp-count N --project NAME

set -euo pipefail

# ---- Defaults ----

TCA_DIR=""
WP_COUNT=0
PROJECT_NAME=""

# ---- Usage ----

usage() {
  echo "Usage: tca-init.sh --tca-dir PATH --wp-count N --project NAME"
  echo ""
  echo "Options:"
  echo "  --tca-dir PATH    TCA steel thread directory (eg intent/st/ST0000)"
  echo "  --wp-count N     Number of work packages to create (including synthesis)"
  echo "  --project NAME   Project name for templates"
  echo "  -h, --help       Show this help"
  exit 0
}

# ---- Parse args ----

while [ $# -gt 0 ]; do
  case "$1" in
    --tca-dir)    TCA_DIR="$2"; shift 2 ;;
    --wp-count)  WP_COUNT="$2"; shift 2 ;;
    --project)   PROJECT_NAME="$2"; shift 2 ;;
    -h|--help)   usage ;;
    *)           echo "error: unknown option: $1" >&2; exit 1 ;;
  esac
done

# ---- Validate ----

if [ -z "$TCA_DIR" ]; then
  echo "error: --tca-dir is required" >&2
  exit 1
fi

if [ "$WP_COUNT" -lt 2 ]; then
  echo "error: --wp-count must be at least 2 (1 component + 1 synthesis)" >&2
  exit 1
fi

if [ -z "$PROJECT_NAME" ]; then
  echo "error: --project is required" >&2
  exit 1
fi

# ---- Provisioning invariants (see intent/docs/total-codebase-audit.md section 0.0) ----

# Invariant 1: a TCA must be its own dedicated steel thread.
# Detect the antipattern by looking for an Intent WP path component inside $TCA_DIR.
# A correct TCA path looks like intent/st/STXXXX/ and never contains /WP/ anywhere.
# This guard runs BEFORE the existence check so it fires whether the path exists or not --
# an operator pointing at a nested WP path is making the same mistake either way.
case "$TCA_DIR" in
  */intent/st/ST*/WP/*|intent/st/ST*/WP/*)
    echo "error: $TCA_DIR is inside an existing Intent work package" >&2
    echo "" >&2
    echo "A Total Codebase Audit must always be its own dedicated steel thread." >&2
    echo "Never provision a TCA as a work package inside another steel thread." >&2
    echo "" >&2
    echo "To start a TCA correctly:" >&2
    echo "  intent st new \"TCA: <project and scope>\" --start" >&2
    echo "" >&2
    echo "See intent/docs/total-codebase-audit.md section 0.0 for rationale" >&2
    echo "and recovery steps if you have already started wrong." >&2
    exit 1
    ;;
esac

if [ ! -d "$TCA_DIR" ]; then
  echo "error: steel thread directory not found: $TCA_DIR" >&2
  exit 1
fi

# Invariant 2: refuse to overwrite an audit that already has populated socrates.md files.
# Empty WP/ directories are fine (a previous run may have stubbed them out).
# Populated socrates.md means real audit work has been committed; overwriting loses it.
if [ -d "$TCA_DIR/WP" ]; then
  populated=$(find "$TCA_DIR/WP" -name socrates.md -size +0 2>/dev/null | wc -l | tr -d ' ')
  if [ "$populated" -gt 0 ]; then
    echo "error: $TCA_DIR/WP already contains $populated populated socrates.md file(s)" >&2
    echo "" >&2
    echo "Refusing to overwrite an in-progress or completed audit." >&2
    echo "Delete the steel thread and re-run, or provision a fresh ST." >&2
    exit 1
  fi
fi

# ---- Create the work packages ----
#
# Through `intent wp new`, so the store registers each one. This
# used to `mkdir` WP/NN and write info.md by hand, which v3 never reads: the
# directories existed and no work package did. The body goes through `intent
# set`, and socrates.md stays the auditor's working file beside the realised view.

command -v intent >/dev/null 2>&1 || {
  echo "error: intent is not on PATH -- the work packages are created through it" >&2
  exit 1
}
ST_ID="$(basename "$TCA_DIR")"
WP_DIR="$TCA_DIR/WP"
SYNTHESIS_WP="$WP_COUNT"
BODY_FILE="$(mktemp)"
trap 'rm -f "$BODY_FILE"' EXIT

i=1
while [ "$i" -le "$WP_COUNT" ]; do
  # Zero-pad to 2 digits
  if [ "$i" -lt 10 ]; then
    WP_NUM="0${i}"
  else
    WP_NUM="$i"
  fi

  if intent wp show "$ST_ID/$WP_NUM" >/dev/null 2>&1; then
    echo "warning: $ST_ID/$WP_NUM already exists, skipping" >&2
    i=$((i + 1))
    continue
  fi

  # Determine if this is the synthesis WP
  if [ "$i" -eq "$SYNTHESIS_WP" ]; then
    WP_TITLE="Cross-Component Synthesis"
    WP_SCOPE="Synthesis of all component audit findings into a prioritized remediation backlog."
  else
    WP_TITLE="Component $WP_NUM"
    WP_SCOPE="[Component description -- fill in during provisioning]"
  fi

  created="$(intent wp new "$ST_ID" "$WP_TITLE" 2>&1)" || {
    echo "error: intent wp new $ST_ID failed: $created" >&2
    exit 1
  }
  case "$created" in
    *"created: $ST_ID/$WP_NUM"*) ;;
    *)
      echo "error: expected $ST_ID/$WP_NUM, and intent answered: $created" >&2
      exit 1
      ;;
  esac

  cat > "$BODY_FILE" << HEREDOC
## Scope

${WP_SCOPE}

## Files

- [List files to audit]

## Applicable Rules

All rules. Special focus: [identify 3-4 rules most likely to surface violations]

## Cross-WP Highlander Dependencies

- WP-XX: [what might be duplicated and why]
HEREDOC
  intent set "intent:///threads/$ST_ID/wp/$WP_NUM" body --from "$BODY_FILE" >/dev/null || {
    echo "error: could not write $ST_ID/$WP_NUM's body" >&2
    exit 1
  }

  # The auditor's working file, beside the realised view.
  mkdir -p "$WP_DIR/$WP_NUM"
  touch "$WP_DIR/$WP_NUM/socrates.md"

  echo "created: $ST_ID/$WP_NUM"

  i=$((i + 1))
done

echo ""
echo "ok: created $WP_COUNT work packages in $WP_DIR"
echo "    WP/01-$(printf '%02d' "$((WP_COUNT - 1))"): component audit WPs"
echo "    WP/$(printf '%02d' "$WP_COUNT"): synthesis WP"
