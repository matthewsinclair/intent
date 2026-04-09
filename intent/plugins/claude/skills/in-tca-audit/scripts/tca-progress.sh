#!/bin/bash
# tca-progress.sh - Track TCA audit progress
# Scans WP directories for completed audits and extracts violation statistics.
# bash 3.x compatible. No external dependencies.
#
# Usage:
#   tca-progress.sh --tca-dir PATH

set -euo pipefail

# ---- Defaults ----

TCA_DIR=""

# ---- Usage ----

usage() {
  echo "Usage: tca-progress.sh --tca-dir PATH"
  echo ""
  echo "Options:"
  echo "  --tca-dir PATH    TCA steel thread directory (e.g., intent/st/ST0055)"
  echo "  -h, --help       Show this help"
  exit 0
}

# ---- Parse args ----

while [ $# -gt 0 ]; do
  case "$1" in
    --tca-dir)  TCA_DIR="$2"; shift 2 ;;
    -h|--help) usage ;;
    *)         echo "error: unknown option: $1" >&2; exit 1 ;;
  esac
done

# ---- Validate ----

if [ -z "$TCA_DIR" ]; then
  echo "error: --tca-dir is required" >&2
  exit 1
fi

WP_DIR="$TCA_DIR/WP"

if [ ! -d "$WP_DIR" ]; then
  echo "error: no WP directory found at $WP_DIR" >&2
  exit 1
fi

# ---- Scan WPs ----

TOTAL_WPS=0
COMPLETE_WPS=0
TOTAL_VIOLATIONS=0
TOTAL_HIGH=0
TOTAL_MEDIUM=0
TOTAL_LOW=0

# Header
printf "%-6s  %-10s  %-10s  %5s  %5s  %5s\n" "WP" "Status" "Violations" "High" "Med" "Low"
printf "%-6s  %-10s  %-10s  %5s  %5s  %5s\n" "------" "----------" "----------" "-----" "-----" "-----"

for wp_path in "$WP_DIR"/*/; do
  [ -d "$wp_path" ] || continue

  wp_num="$(basename "$wp_path")"
  socrates="$wp_path/socrates.md"
  TOTAL_WPS=$((TOTAL_WPS + 1))

  # Check if socrates.md exists and is non-empty
  if [ ! -f "$socrates" ] || [ ! -s "$socrates" ]; then
    printf "%-6s  %-10s  %10s  %5s  %5s  %5s\n" "$wp_num" "Pending" "-" "-" "-" "-"
    continue
  fi

  COMPLETE_WPS=$((COMPLETE_WPS + 1))

  # Extract violation counts from Summary table
  # Look for lines like "| High     | X     |" in the summary
  high=0
  medium=0
  low=0
  total=0

  # Parse the summary table -- look for severity rows
  while IFS= read -r line; do
    case "$line" in
      *"| High"*|*"| high"*)
        val="$(echo "$line" | sed 's/[^0-9]//g')"
        if [ -n "$val" ]; then high="$val"; fi
        ;;
      *"| Medium"*|*"| medium"*)
        val="$(echo "$line" | sed 's/[^0-9]//g')"
        if [ -n "$val" ]; then medium="$val"; fi
        ;;
      *"| Low"*|*"| low"*)
        val="$(echo "$line" | sed 's/[^0-9]//g')"
        if [ -n "$val" ]; then low="$val"; fi
        ;;
      *"| **Total**"*|*"| Total"*|*"| total"*)
        val="$(echo "$line" | sed 's/[^0-9]//g')"
        if [ -n "$val" ]; then total="$val"; fi
        ;;
    esac
  done < "$socrates"

  # If no total found, compute from components
  if [ "$total" -eq 0 ]; then
    total=$((high + medium + low))
  fi

  TOTAL_VIOLATIONS=$((TOTAL_VIOLATIONS + total))
  TOTAL_HIGH=$((TOTAL_HIGH + high))
  TOTAL_MEDIUM=$((TOTAL_MEDIUM + medium))
  TOTAL_LOW=$((TOTAL_LOW + low))

  printf "%-6s  %-10s  %10d  %5d  %5d  %5d\n" "$wp_num" "Complete" "$total" "$high" "$medium" "$low"
done

# Footer
printf "%-6s  %-10s  %-10s  %5s  %5s  %5s\n" "------" "----------" "----------" "-----" "-----" "-----"
printf "%-6s  %-10s  %10d  %5d  %5d  %5d\n" "TOTAL" "$COMPLETE_WPS/$TOTAL_WPS" "$TOTAL_VIOLATIONS" "$TOTAL_HIGH" "$TOTAL_MEDIUM" "$TOTAL_LOW"

echo ""

# Status
PENDING=$((TOTAL_WPS - COMPLETE_WPS))
if [ "$PENDING" -eq 0 ]; then
  echo "ok: all $TOTAL_WPS work packages complete"
  exit 0
else
  echo "hint: $PENDING of $TOTAL_WPS work packages pending"
  exit 1
fi
