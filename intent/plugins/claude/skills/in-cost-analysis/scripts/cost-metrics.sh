#!/bin/bash
# cost-metrics.sh - Collect codebase metrics for cost analysis
# Outputs JSON with LOC counts, language breakdown, tier classification, and git history.
# bash 3.x compatible. No external dependencies (no jq, no python).
#
# Usage:
#   cost-metrics.sh --dir /path/to/project [--exclude "dir1,dir2"] [--no-git] [-o file.json]

set -euo pipefail

# ---- Defaults ----

TARGET_DIR="."
EXCLUDE_DIRS="vendor,node_modules,_build,deps,.git,.elixir_ls,_opam,target,dist,build,.next"
USE_GIT=1
OUTPUT_FILE=""

# ---- Temp files ----

TMPDIR_METRICS=""

cleanup() {
  if [ -n "$TMPDIR_METRICS" ] && [ -d "$TMPDIR_METRICS" ]; then
    rm -rf "$TMPDIR_METRICS"
  fi
}
trap cleanup EXIT

# ---- Usage ----

usage() {
  echo "Usage: cost-metrics.sh --dir PATH [--exclude DIRS] [--no-git] [--git] [-o FILE]"
  echo ""
  echo "Options:"
  echo "  --dir PATH        Target directory to analyze (default: .)"
  echo "  --exclude DIRS    Comma-separated directories to exclude"
  echo "  --git             Include git history analysis (default)"
  echo "  --no-git          Skip git history analysis"
  echo "  -o FILE           Write JSON output to file (default: stdout)"
  echo "  -h, --help        Show this help"
  exit 0
}

# ---- Parse args ----

while [ $# -gt 0 ]; do
  case "$1" in
    --dir)     TARGET_DIR="$2"; shift 2 ;;
    --exclude) EXCLUDE_DIRS="$2"; shift 2 ;;
    --git)     USE_GIT=1; shift ;;
    --no-git)  USE_GIT=0; shift ;;
    -o)        OUTPUT_FILE="$2"; shift 2 ;;
    -h|--help) usage ;;
    *)         echo "error: unknown option: $1" >&2; exit 1 ;;
  esac
done

# Validate target directory
if [ ! -d "$TARGET_DIR" ]; then
  echo "error: directory not found: $TARGET_DIR" >&2
  exit 1
fi

# Resolve to absolute path
TARGET_DIR="$(cd "$TARGET_DIR" && pwd)"

# Create temp directory
TMPDIR_METRICS="$(mktemp -d)"

# ---- Language mapping ----

ext_to_language() {
  case "$1" in
    ex|exs)           echo "Elixir" ;;
    erl|hrl)          echo "Erlang" ;;
    py|pyw)           echo "Python" ;;
    js|mjs|cjs)       echo "JavaScript" ;;
    ts|mts|cts)       echo "TypeScript" ;;
    tsx)              echo "TypeScript" ;;
    jsx)              echo "JavaScript" ;;
    swift)            echo "Swift" ;;
    rs)               echo "Rust" ;;
    go)               echo "Go" ;;
    c|h)              echo "C" ;;
    cpp|cc|cxx|hpp)   echo "C++" ;;
    rb|rake)          echo "Ruby" ;;
    java)             echo "Java" ;;
    kt|kts)           echo "Kotlin" ;;
    sh|bash|zsh|bats) echo "Shell" ;;
    php)              echo "PHP" ;;  # yes, really
    cs)               echo "C#" ;;
    hs|lhs)           echo "Haskell" ;;
    lua)              echo "Lua" ;;
    ml|mli)           echo "OCaml" ;;
    r|R)              echo "R" ;;
    scala)            echo "Scala" ;;
    dart)             echo "Dart" ;;
    vue)              echo "Vue" ;;
    svelte)           echo "Svelte" ;;
    css|scss|sass)    echo "CSS" ;;
    html|htm)         echo "HTML" ;;
    sql)              echo "SQL" ;;
    *)                echo "" ;;
  esac
}

# ---- Comment prefix detection ----

comment_prefix_for_language() {
  case "$1" in
    Shell|Python|Ruby|R)           echo "#" ;;
    Elixir|Haskell|Lua|SQL)        echo "--" ;;
    C|"C++"|Java|Kotlin|Go|Rust|Swift|JavaScript|TypeScript|PHP|"C#"|Scala|Dart) echo "//" ;;
    Erlang)                        echo "%" ;;
    OCaml)                         echo "(*" ;;
    CSS|HTML|Vue|Svelte)           echo "" ;;
    *)                             echo "" ;;
  esac
}

# ---- Tier classification ----

classify_tier() {
  local filepath="$1"
  case "$filepath" in
    */test/*|*/tests/*|*/spec/*|*/test_helper*|*_test.*)
      echo "simple" ;;
    */config/*|*/priv/*|*/migrations/*|*.config.*|*.toml|*.yaml|*.yml)
      echo "simple" ;;
    */mix.exs|*/Makefile|*/Rakefile|*/Gemfile|*/package.json|*/Cargo.toml)
      echo "simple" ;;
    */worker/*|*/workers/*|*/consumer/*|*/producer/*)
      echo "complex" ;;
    */service/*|*/services/*|*/domain/*|*/core/*)
      echo "complex" ;;
    */live/*|*/channel/*|*/socket/*|*/pubsub/*)
      echo "complex" ;;
    */gpu/*|*/shader/*|*.metal|*.cu|*.cl)
      echo "specialized" ;;
    */crypto/*|*/security/*|*/auth/*)
      echo "complex" ;;
    */plugin/*|*/plugins/*|*/extension/*|*/middleware/*)
      echo "complex" ;;
    *)
      echo "moderate" ;;
  esac
}

# ---- Build find exclude args ----

build_find_excludes() {
  local excludes=""
  local IFS=","
  for dir in $EXCLUDE_DIRS; do
    dir="$(echo "$dir" | sed 's/^ *//;s/ *$//')"
    if [ -n "$dir" ]; then
      if [ -n "$excludes" ]; then
        excludes="$excludes -o"
      fi
      excludes="$excludes -name $dir -type d"
    fi
  done
  echo "$excludes"
}

# ---- Count lines in a file (awk-based for performance) ----

count_lines() {
  local filepath="$1"
  local comment_prefix="$2"

  awk -v cp="$comment_prefix" '
  {
    total++
    sub(/^[[:space:]]+/, "")
    if ($0 == "") blank++
    else if (cp != "" && index($0, cp) == 1) comment++
    else code++
  }
  END {
    printf "%d %d %d %d", total+0, blank+0, comment+0, code+0
  }' "$filepath"
}

# ---- Main analysis ----

# Build find command with excludes
FIND_EXCLUDES="$(build_find_excludes)"
if [ -n "$FIND_EXCLUDES" ]; then
  FIND_CMD="find \"$TARGET_DIR\" \\( $FIND_EXCLUDES \\) -prune -o -type f -print"
else
  FIND_CMD="find \"$TARGET_DIR\" -type f -print"
fi

# Aggregate files
LANG_SUMMARY="$TMPDIR_METRICS/lang_summary"
TIER_SUMMARY="$TMPDIR_METRICS/tier_summary"
TOTAL_FILES=0
TOTAL_LINES=0
TOTAL_BLANK=0
TOTAL_COMMENT=0
TOTAL_CODE=0

# Initialize tier counts
TIER_SIMPLE_CODE=0
TIER_MODERATE_CODE=0
TIER_COMPLEX_CODE=0
TIER_SPECIALIZED_CODE=0

echo "Analyzing $TARGET_DIR ..." >&2

eval "$FIND_CMD" | while IFS= read -r filepath; do
  # Get file extension
  filename="$(basename "$filepath")"
  ext="${filename##*.}"

  # Skip files without extensions or where ext == filename (no dot)
  if [ "$ext" = "$filename" ]; then
    continue
  fi

  # Map extension to language
  language="$(ext_to_language "$ext")"
  if [ -z "$language" ]; then
    continue
  fi

  # Count lines
  comment_prefix="$(comment_prefix_for_language "$language")"
  read -r f_total f_blank f_comment f_code <<< "$(count_lines "$filepath" "$comment_prefix")"

  # Skip empty files
  if [ "$f_total" -eq 0 ]; then
    continue
  fi

  # Classify tier
  tier="$(classify_tier "$filepath")"

  # Append to language summary (language|files|total|blank|comment|code)
  echo "$language|1|$f_total|$f_blank|$f_comment|$f_code" >> "$LANG_SUMMARY"

  # Append to tier summary (tier|code_lines)
  echo "$tier|$f_code" >> "$TIER_SUMMARY"
done

# ---- Aggregate results ----

aggregate_data() {
  if [ ! -f "$LANG_SUMMARY" ]; then
    echo "0|0|0|0|0|0"
    return
  fi

  local agg_files=0 agg_total=0 agg_blank=0 agg_comment=0 agg_code=0 agg_count=0

  # Use awk for aggregation (field 1 = unique language count)
  awk -F'|' '{
    files += $2; total += $3; blank += $4; comment += $5; code += $6
    langs[$1] = 1
  } END {
    lcount = 0; for (l in langs) lcount++
    printf "%d|%d|%d|%d|%d|%d\n", lcount, files, total, blank, comment, code
  }' "$LANG_SUMMARY"
}

aggregate_lang_json() {
  if [ ! -f "$LANG_SUMMARY" ]; then
    echo "[]"
    return
  fi

  # Aggregate per language, output as sortable lines, then assemble JSON
  local agg_file="$TMPDIR_METRICS/lang_agg"
  awk -F'|' '{
    files[$1] += $2
    total[$1] += $3
    blank[$1] += $4
    comment[$1] += $5
    code[$1] += $6
  } END {
    for (l in code) {
      printf "%d|%s|%d|%d|%d|%d|%d\n", code[l], l, files[l], total[l], blank[l], comment[l], code[l]
    }
  }' "$LANG_SUMMARY" | sort -t'|' -k1 -rn > "$agg_file"

  # Build JSON array from sorted lines
  local first=1
  local json="["
  while IFS='|' read -r _sort lang files total blank comment code; do
    if [ $first -eq 1 ]; then
      first=0
    else
      json="$json,"
    fi
    json="$json{\"language\":\"$lang\",\"files\":$files,\"total_lines\":$total,\"blank_lines\":$blank,\"comment_lines\":$comment,\"code_lines\":$code}"
  done < "$agg_file"
  json="$json]"
  echo "$json"
}

aggregate_tier_json() {
  if [ ! -f "$TIER_SUMMARY" ]; then
    echo "{\"simple\":0,\"moderate\":0,\"complex\":0,\"specialized\":0}"
    return
  fi

  awk -F'|' '{
    tiers[$1] += $2
  } END {
    printf "{\"simple\":%d,\"moderate\":%d,\"complex\":%d,\"specialized\":%d}", \
      tiers["simple"]+0, tiers["moderate"]+0, tiers["complex"]+0, tiers["specialized"]+0
  }' "$TIER_SUMMARY"
}

# ---- Git history ----

collect_git_history() {
  if [ "$USE_GIT" -eq 0 ]; then
    echo "null"
    return
  fi

  if ! git -C "$TARGET_DIR" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "null"
    return
  fi

  local first_commit last_commit total_commits author_count months_active

  first_commit="$(git -C "$TARGET_DIR" log --reverse --format="%ai" 2>/dev/null | head -1)"
  last_commit="$(git -C "$TARGET_DIR" log -1 --format="%ai" 2>/dev/null)"
  total_commits="$(git -C "$TARGET_DIR" rev-list --count HEAD 2>/dev/null || echo 0)"
  author_count="$(git -C "$TARGET_DIR" log --format="%ae" 2>/dev/null | sort -u | wc -l | tr -d ' ')"

  if [ -z "$first_commit" ] || [ -z "$last_commit" ]; then
    echo "null"
    return
  fi

  # Extract dates (YYYY-MM-DD)
  local first_date last_date
  first_date="$(echo "$first_commit" | cut -d' ' -f1)"
  last_date="$(echo "$last_commit" | cut -d' ' -f1)"

  # Calculate months between dates (approximate)
  local first_year first_month last_year last_month
  first_year="$(echo "$first_date" | cut -d'-' -f1)"
  first_month="$(echo "$first_date" | cut -d'-' -f2)"
  last_year="$(echo "$last_date" | cut -d'-' -f1)"
  last_month="$(echo "$last_date" | cut -d'-' -f2)"

  # Remove leading zeros for arithmetic
  first_month="$((10#$first_month))"
  last_month="$((10#$last_month))"

  months_active="$(( (last_year - first_year) * 12 + last_month - first_month ))"
  if [ "$months_active" -lt 1 ]; then
    months_active=1
  fi

  printf '{"first_commit":"%s","last_commit":"%s","total_commits":%d,"author_count":%d,"months_active":%d}' \
    "$first_date" "$last_date" "$total_commits" "$author_count" "$months_active"
}

# ---- Session estimation (commit clustering) ----

collect_session_estimates() {
  if [ "$USE_GIT" -eq 0 ]; then
    echo "null"
    return
  fi

  if ! git -C "$TARGET_DIR" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "null"
    return
  fi

  # Get all commit timestamps as epoch seconds, sorted ascending
  local timestamps_file="$TMPDIR_METRICS/commit_timestamps"
  git -C "$TARGET_DIR" log --format="%at" 2>/dev/null | sort -n > "$timestamps_file"

  local commit_count
  commit_count="$(wc -l < "$timestamps_file" | tr -d ' ')"

  if [ "$commit_count" -eq 0 ]; then
    echo "null"
    return
  fi

  # Cluster commits into sessions using 4-hour gap threshold.
  # Estimate hours per session by commit density:
  #   1-2 commits  = 1 hour
  #   3-5 commits  = 2 hours
  #   6-10 commits = 3 hours
  #   11+ commits  = 4 hours
  awk '
  BEGIN {
    gap_threshold = 4 * 3600  # 4 hours in seconds
    session_count = 0
    session_commits = 0
    total_hours = 0
    prev_ts = 0
  }
  {
    ts = $1
    if (prev_ts == 0 || (ts - prev_ts) > gap_threshold) {
      # Close previous session if any
      if (session_commits > 0) {
        session_count++
        if (session_commits <= 2) total_hours += 1
        else if (session_commits <= 5) total_hours += 2
        else if (session_commits <= 10) total_hours += 3
        else total_hours += 4
      }
      session_commits = 1
    } else {
      session_commits++
    }
    prev_ts = ts
  }
  END {
    # Close final session
    if (session_commits > 0) {
      session_count++
      if (session_commits <= 2) total_hours += 1
      else if (session_commits <= 5) total_hours += 2
      else if (session_commits <= 10) total_hours += 3
      else total_hours += 4
    }
    printf "{\"sessions\":%d,\"estimated_hours\":%d}", session_count, total_hours
  }' "$timestamps_file"
}

# ---- Build JSON output ----

echo "Aggregating results ..." >&2

TOTALS="$(aggregate_data)"
LANG_COUNT="$(echo "$TOTALS" | cut -d'|' -f1)"
FILE_COUNT="$(echo "$TOTALS" | cut -d'|' -f2)"
TOTAL_LINES="$(echo "$TOTALS" | cut -d'|' -f3)"
TOTAL_BLANK="$(echo "$TOTALS" | cut -d'|' -f4)"
TOTAL_COMMENT="$(echo "$TOTALS" | cut -d'|' -f5)"
TOTAL_CODE="$(echo "$TOTALS" | cut -d'|' -f6)"

LANG_JSON="$(aggregate_lang_json)"
TIER_JSON="$(aggregate_tier_json)"
GIT_JSON="$(collect_git_history)"
SESSIONS_JSON="$(collect_session_estimates)"

PROJECT_NAME="$(basename "$TARGET_DIR")"

JSON=$(cat <<ENDJSON
{
  "project": "$PROJECT_NAME",
  "directory": "$TARGET_DIR",
  "summary": {
    "languages": $LANG_COUNT,
    "files": $FILE_COUNT,
    "total_lines": $TOTAL_LINES,
    "blank_lines": $TOTAL_BLANK,
    "comment_lines": $TOTAL_COMMENT,
    "code_lines": $TOTAL_CODE
  },
  "languages": $LANG_JSON,
  "tiers": $TIER_JSON,
  "git": $GIT_JSON,
  "sessions": $SESSIONS_JSON
}
ENDJSON
)

if [ -n "$OUTPUT_FILE" ]; then
  echo "$JSON" > "$OUTPUT_FILE"
  echo "ok: wrote $OUTPUT_FILE" >&2
else
  echo "$JSON"
fi
