#!/bin/bash
# tca-report.sh - Generate TCA feedback report template + run finish-time guards
# Reads all socrates.md files and synthesis data to pre-populate a report.
# In --check-only mode, runs pre-flight guards without generating the report.
# bash 3.x compatible. No external dependencies.
#
# Usage:
#   tca-report.sh --tca-dir PATH [-o FILE]     # generate report (default)
#   tca-report.sh --tca-dir PATH --check-only  # run guards only, exit 0 on pass

set -euo pipefail

# ---- Defaults ----

TCA_DIR=""
OUTPUT_FILE=""
CHECK_ONLY=0

# ---- Usage ----

usage() {
  echo "Usage: tca-report.sh --tca-dir PATH [-o FILE] [--check-only]"
  echo ""
  echo "Options:"
  echo "  --tca-dir PATH    TCA steel thread directory (e.g., intent/st/ST0055)"
  echo "  -o FILE          Output file (default: stdout)"
  echo "  --check-only     Run pre-flight guards only, no report generation"
  echo "  -h, --help       Show this help"
  exit 0
}

# ---- Parse args ----

while [ $# -gt 0 ]; do
  case "$1" in
    --tca-dir)     TCA_DIR="$2"; shift 2 ;;
    -o)           OUTPUT_FILE="$2"; shift 2 ;;
    --check-only) CHECK_ONLY=1; shift ;;
    -h|--help)    usage ;;
    *)            echo "error: unknown option: $1" >&2; exit 1 ;;
  esac
done

# ---- Validate ----

if [ -z "$TCA_DIR" ]; then
  echo "error: --tca-dir is required" >&2
  exit 1
fi

WP_DIR="$TCA_DIR/WP"

# ---- Shape guard (always runs, even in normal report-generation mode) ----
#
# Refuse to operate on non-TCA steel threads. This catches the failure mode of
# running in-tca-finish against an ordinary feature ST by mistake.

if [ ! -d "$WP_DIR" ]; then
  echo "error: no WP directory found at $WP_DIR" >&2
  echo "       in-tca-finish only operates on TCA-shaped steel threads." >&2
  exit 1
fi

if [ ! -f "$TCA_DIR/design.md" ]; then
  echo "error: $TCA_DIR/design.md not found" >&2
  echo "       in-tca-finish only operates on TCA-shaped steel threads." >&2
  exit 1
fi

if ! grep -qE "rule set|Rule [0-9]|\bR[0-9]+" "$TCA_DIR/design.md" 2>/dev/null; then
  echo "error: $TCA_DIR/design.md does not contain a recognizable rule set" >&2
  echo "       (expected 'rule set', 'Rule N', or 'RN' patterns)" >&2
  echo "       in-tca-finish only operates on TCA-shaped steel threads." >&2
  exit 1
fi

# ---- Check-only mode: pre-flight guards for premature close-out ----
#
# These guards exist to prevent the Lamplight ST0121 premature-close-out incident
# (commits 75706c18 to 98616a0c, 2026-04-08) from recurring. They run only in
# --check-only mode because in normal report-generation mode the feedback report
# does not exist yet (that is what the script is about to write). See section 0.0
# of intent/docs/total-codebase-audit.md for the full rationale.
#
# Guards (check-only only):
#   Guard 1a: feedback-report.md exists at the canonical path
#   Guard 1b: feedback-report.md has no unfilled [Fill in:] placeholders
#   Guard 2: no unchecked - [ ] acceptance criteria in info.md
# Guard order: existence -> content -> criteria. Fail fast on cheapest first.

if [ "$CHECK_ONLY" = "1" ]; then

  # Guard 1a: feedback-report.md must exist at the canonical location
  FEEDBACK_PATH="$TCA_DIR/feedback-report.md"
  if [ ! -f "$FEEDBACK_PATH" ]; then
    echo "error: feedback-report.md not found at $FEEDBACK_PATH" >&2
    echo "" >&2
    echo "Cannot close a TCA before the feedback report exists." >&2
    echo "Generate the template and fill in the analytical sections first:" >&2
    echo "  tca-report.sh --tca-dir $TCA_DIR -o $FEEDBACK_PATH" >&2
    echo "" >&2
    echo "Then fill in Rule Analysis, WP Sizing, Sub-Agent Effectiveness," >&2
    echo "and Process Improvements, save the file, and re-run this guard." >&2
    exit 1
  fi

  # Guard 1b: feedback report must not still contain [Fill in:] placeholders.
  # Use a pure-shell counter rather than `grep -c` because grep returning 1 on
  # zero matches interacts badly with `set -euo pipefail` -- the pipeline exit
  # would kill the script silently on assignment.
  unfilled=0
  while IFS= read -r line; do
    if [[ "$line" == *"[Fill in"* ]]; then
      unfilled=$((unfilled + 1))
    fi
  done < "$FEEDBACK_PATH"
  if [ "$unfilled" -gt 0 ]; then
    echo "error: $FEEDBACK_PATH still contains $unfilled unfilled [Fill in:] placeholder(s)" >&2
    echo "" >&2
    echo "Fill in all analytical sections before closing the audit:" >&2
    echo "  - Rule Analysis (which rules had most value, which were noisy)" >&2
    echo "  - WP Sizing Assessment (what worked, what was too large/small)" >&2
    echo "  - Sub-Agent Effectiveness (turns, FP rate per agent)" >&2
    echo "  - Process Improvements (what to change in the TCA doc or skills)" >&2
    exit 1
  fi

  # Guard 2: no unchecked acceptance criteria in info.md.
  # Same pure-shell counter pattern as guard 1b, for the same pipefail reason.
  if [ -f "$TCA_DIR/info.md" ]; then
    unchecked=0
    while IFS= read -r line; do
      if [[ "$line" == "- [ ]"* ]]; then
        unchecked=$((unchecked + 1))
      fi
    done < "$TCA_DIR/info.md"
    if [ "$unchecked" -gt 0 ]; then
      echo "error: $unchecked unchecked acceptance criteria in $TCA_DIR/info.md" >&2
      echo "" >&2
      echo "Close all - [ ] boxes in info.md before running in-tca-finish." >&2
      echo "Unchecked criteria indicate TCA work that has not been completed." >&2
      exit 1
    fi
  fi

  echo "ok: pre-flight guards passed for $TCA_DIR" >&2
  exit 0
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

ST_NAME="$(basename "$TCA_DIR")"
PROJECT_NAME=""
if [ -f "$TCA_DIR/info.md" ]; then
  proj_line="$(grep "^# " "$TCA_DIR/info.md" 2>/dev/null | head -1 || true)"
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
