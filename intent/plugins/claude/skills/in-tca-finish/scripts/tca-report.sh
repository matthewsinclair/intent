#!/bin/bash
# tca-report.sh - Generate TCA feedback report template
# Reads all socrates.md files and synthesis data to pre-populate a report.
# bash 3.x compatible. No external dependencies.
#
# Usage:
#   tca-report.sh --st-dir PATH [-o FILE]

set -euo pipefail

# ---- Defaults ----

ST_DIR=""
OUTPUT_FILE=""

# ---- Usage ----

usage() {
  echo "Usage: tca-report.sh --st-dir PATH [-o FILE]"
  echo ""
  echo "Options:"
  echo "  --st-dir PATH    Steel thread directory (e.g., intent/st/ST0055)"
  echo "  -o FILE          Output file (default: stdout)"
  echo "  -h, --help       Show this help"
  exit 0
}

# ---- Parse args ----

while [ $# -gt 0 ]; do
  case "$1" in
    --st-dir)  ST_DIR="$2"; shift 2 ;;
    -o)        OUTPUT_FILE="$2"; shift 2 ;;
    -h|--help) usage ;;
    *)         echo "error: unknown option: $1" >&2; exit 1 ;;
  esac
done

# ---- Validate ----

if [ -z "$ST_DIR" ]; then
  echo "error: --st-dir is required" >&2
  exit 1
fi

WP_DIR="$ST_DIR/WP"

if [ ! -d "$WP_DIR" ]; then
  echo "error: no WP directory found at $WP_DIR" >&2
  exit 1
fi

# ---- Collect data ----

TOTAL_WPS=0
COMPLETE_WPS=0
TOTAL_VIOLATIONS=0
TOTAL_HIGH=0
TOTAL_MEDIUM=0
TOTAL_LOW=0
WP_SUMMARIES=""

for wp_path in "$WP_DIR"/*/; do
  [ -d "$wp_path" ] || continue

  wp_num="$(basename "$wp_path")"
  socrates="$wp_path/socrates.md"
  info="$wp_path/info.md"
  TOTAL_WPS=$((TOTAL_WPS + 1))

  # Get WP title from info.md if available
  wp_title="$wp_num"
  if [ -f "$info" ]; then
    title_line="$(grep "^title:" "$info" 2>/dev/null | head -1 || true)"
    if [ -n "$title_line" ]; then
      wp_title="$(echo "$title_line" | sed 's/^title: *"*//;s/"*$//')"
    fi
  fi

  if [ ! -f "$socrates" ] || [ ! -s "$socrates" ]; then
    WP_SUMMARIES="${WP_SUMMARIES}| WP-${wp_num} | ${wp_title} | - | - | - | - | Pending |\n"
    continue
  fi

  COMPLETE_WPS=$((COMPLETE_WPS + 1))

  # Extract counts from summary table
  high=0
  medium=0
  low=0
  total=0

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

  if [ "$total" -eq 0 ]; then
    total=$((high + medium + low))
  fi

  TOTAL_VIOLATIONS=$((TOTAL_VIOLATIONS + total))
  TOTAL_HIGH=$((TOTAL_HIGH + high))
  TOTAL_MEDIUM=$((TOTAL_MEDIUM + medium))
  TOTAL_LOW=$((TOTAL_LOW + low))

  WP_SUMMARIES="${WP_SUMMARIES}| WP-${wp_num} | ${wp_title} | ${total} | ${high} | ${medium} | ${low} | Complete |\n"
done

# ---- Count violation IDs for unique count ----

UNIQUE_COUNT=0
if command -v grep >/dev/null 2>&1; then
  UNIQUE_COUNT="$(grep -rh "^#### V[0-9]" "$WP_DIR"/*/socrates.md 2>/dev/null | wc -l | tr -d ' ')" || UNIQUE_COUNT=0
fi

# ---- Compute dedup rate ----

DEDUP_RATE="N/A"
if [ "$TOTAL_VIOLATIONS" -gt 0 ] && [ "$UNIQUE_COUNT" -gt 0 ]; then
  # Rough estimate -- actual dedup happens in synthesis
  DEDUP_RATE="~$(( (TOTAL_VIOLATIONS - UNIQUE_COUNT) * 100 / TOTAL_VIOLATIONS ))%"
fi

# ---- Get ST info ----

ST_NAME="$(basename "$ST_DIR")"
PROJECT_NAME=""
if [ -f "$ST_DIR/info.md" ]; then
  proj_line="$(grep "^# " "$ST_DIR/info.md" 2>/dev/null | head -1 || true)"
  if [ -n "$proj_line" ]; then
    PROJECT_NAME="$(echo "$proj_line" | sed 's/^# //')"
  fi
fi

# ---- Generate report ----

REPORT=$(cat <<ENDREPORT
# TCA Feedback Report: ${ST_NAME}

**Project**: ${PROJECT_NAME}
**Date**: $(date +%Y-%m-%d)
**WPs**: ${COMPLETE_WPS}/${TOTAL_WPS} complete

## Audit Statistics

| Metric             | Value              |
| ------------------ | -----------------: |
| Total WPs          |       ${TOTAL_WPS} |
| Complete WPs       |    ${COMPLETE_WPS} |
| Raw violations     | ${TOTAL_VIOLATIONS} |
| High severity      |      ${TOTAL_HIGH} |
| Medium severity    |    ${TOTAL_MEDIUM} |
| Low severity       |       ${TOTAL_LOW} |

### Per-WP Breakdown

| WP      | Component | Total | High | Medium | Low | Status |
| ------- | --------- | ----: | ---: | -----: | --: | ------ |
$(printf "%b" "$WP_SUMMARIES")

## Rule Analysis

[Fill in: which rules produced the most violations? Which had highest false positive rate?]

| Rule | Name | Count | Value Assessment |
| ---- | ---- | ----: | ---------------- |
| R?   | ?    |     ? | High/Medium/Low  |

## WP Sizing Assessment

[Fill in: were WPs appropriately sized? Which were too large/small?]

- Smallest WP: ? files, ? violations
- Largest WP: ? files, ? violations
- Sweet spot confirmation: 12-20 effective files?

## Sub-Agent Effectiveness

[Fill in: which agent types worked best? Turns used? False positive rate?]

| Agent Type | WPs | Avg Turns | FP Rate |
| ---------- | --: | --------: | ------: |
| ?          |   ? |         ? |      ?% |

## Process Improvements

[Fill in: what should change in the TCA doc or skills for next time?]

1. ?
2. ?
3. ?

## Comparison with Previous TCAs

[Fill in if applicable]

## Metrics Summary

| Metric            | This Audit         |
| ----------------- | -----------------: |
| Files audited     |                  ? |
| Raw violations    | ${TOTAL_VIOLATIONS} |
| Unique violations |                  ? |
| Dedup rate        |                  ? |
| Fixed             |                  ? |
| False positives   |                  ? |
| Deferred          |                  ? |
| Wall clock        |              ? hrs |
ENDREPORT
)

if [ -n "$OUTPUT_FILE" ]; then
  echo "$REPORT" > "$OUTPUT_FILE"
  echo "ok: wrote feedback report to $OUTPUT_FILE" >&2
else
  echo "$REPORT"
fi
