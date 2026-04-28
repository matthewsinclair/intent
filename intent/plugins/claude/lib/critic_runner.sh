#!/bin/bash
# critic_runner.sh -- Intent headless critic runner primitives (ST0035/WP-05)
#
# Responsibility:
#   Load language-specific rules from canon + extensions, extract each
#   rule's Greppable proxy from its Detection section, run the pattern
#   against target files, and emit severity-grouped findings in text or
#   JSON. Never shells out to an LLM; never autofixes.
#
# Consumers:
#   - bin/intent_critic (thin CLI dispatch)
#
# Extends (via source):
#   - intent/plugins/claude/lib/rules_lib.sh
#
# The mechanical subset: rules that publish a "Greppable proxy" fenced
# bash block inside their Detection section are runnable by this script.
# Rules without a greppable proxy are skipped silently (they remain
# available via the `critic-<lang>` LLM subagents for richer review).

if [ -z "${INTENT_HOME:-}" ]; then
  _critic_runner_self="${BASH_SOURCE[0]}"
  while [ -L "$_critic_runner_self" ]; do
    _critic_runner_self="$(readlink "$_critic_runner_self")"
  done
  INTENT_HOME="$(cd "$(dirname "$_critic_runner_self")/../../../.." && pwd)"
  export INTENT_HOME
  unset _critic_runner_self
fi

# Source the shared rule-library primitives if not already loaded.
if ! declare -f rule_frontmatter >/dev/null 2>&1; then
  source "$INTENT_HOME/intent/plugins/claude/lib/rules_lib.sh"
fi

# Severity priority (low → high): style, recommendation, warning, critical.
# Used for --severity-min filtering and report ordering.
critic_severity_rank() {
  case "$1" in
    critical)       echo 4 ;;
    warning)        echo 3 ;;
    recommendation) echo 2 ;;
    style)          echo 1 ;;
    *)              echo 0 ;;
  esac
}

# Resolve the project root (directory containing intent/.config/config.json) by
# walking up from the caller's cwd. Echoes the path, or empty string if
# not found. Does not change the working directory.
critic_resolve_project_root() {
  local dir="${1:-$PWD}"
  while [ "$dir" != "/" ] && [ -n "$dir" ]; do
    if [ -f "$dir/intent/.config/config.json" ]; then
      echo "$dir"
      return 0
    fi
    dir="$(dirname "$dir")"
  done
  return 1
}

# Extract the first fenced bash block inside the `## Detection` section of
# a RULE.md file, filtered to the one following a `Greppable proxy` line.
# Echoes the bash command(s); empty output means "no greppable proxy".
critic_extract_greppable_block() {
  local path="$1"
  awk '
    /^## Detection[[:space:]]*$/ { in_det = 1; next }
    /^## / && in_det { exit }
    in_det && /[Gg]reppable proxy/ { after_proxy = 1; next }
    in_det && after_proxy && /^```bash[[:space:]]*$/ { in_block = 1; next }
    in_det && in_block && /^```[[:space:]]*$/ { exit }
    in_det && in_block { print }
  ' "$path"
}

# Extract the first single-quoted (or double-quoted) regex argument from
# a grep command. Returns empty string if none found.
critic_pattern_from_grep_command() {
  local cmd="$1"
  # Try single-quote first
  local pat
  pat="$(printf '%s' "$cmd" | sed -n "s/[^']*'\\([^']*\\)'.*/\\1/p" | head -1)"
  if [ -n "$pat" ]; then
    printf '%s' "$pat"
    return 0
  fi
  # Fall back to double-quote
  pat="$(printf '%s' "$cmd" | sed -n 's/[^"]*"\([^"]*\)".*/\1/p' | head -1)"
  printf '%s' "$pat"
}

# Load rule paths applicable to the given language. Agnostic rules are
# intentionally excluded (they are always concretised by language rules;
# direct application would double-report). Emits RULE.md paths on stdout.
critic_load_rule_paths() {
  local lang="$1"
  local path file_lang
  while IFS= read -r path; do
    [ -z "$path" ] && continue
    file_lang="$(rule_fm_scalar "$path" language)"
    case "$file_lang" in
      "$lang") printf '%s\n' "$path" ;;
    esac
  done < <(enumerate_all_rule_files)
}

# Check whether a rule is disabled in the given .intent_critic.yml file.
# Looks for a flat `disabled:` list with the rule ID. Returns 0 if
# disabled, 1 otherwise. Field name aligns with intent/docs/critics.md
# and the canonical sample at
# intent/plugins/claude/rules/_schema/sample-intent-critic.yml.
#
# Awk emits exit code 10 on match; bash distinguishes this from awk's
# natural exit 0 (finished without matching).
critic_rule_disabled() {
  local rule_id="$1"
  local config="$2"
  [ -z "$config" ] && return 1
  [ -f "$config" ] || return 1
  awk -v id="$rule_id" '
    BEGIN { inside = 0 }
    /^disabled:[[:space:]]*\[/ {
      line = $0
      sub(".*\\[", "", line)
      sub("\\].*", "", line)
      n = split(line, parts, ",")
      for (i = 1; i <= n; i++) {
        it = parts[i]
        gsub("[[:space:]\"'\'']", "", it)
        sub("#.*$", "", it)
        if (it == id) { exit 10 }
      }
      next
    }
    /^disabled:[[:space:]]*$/ { inside = 1; next }
    inside == 1 && /^[[:space:]]+-/ {
      it = $0
      sub("^[[:space:]]+-[[:space:]]*", "", it)
      sub("#.*$", "", it)
      gsub("[[:space:]\"'\'']", "", it)
      if (it == id) { exit 10 }
      next
    }
    inside == 1 && /^[A-Za-z]/ { inside = 0 }
  ' "$config"
  local rc=$?
  [ "$rc" -eq 10 ] && return 0
  return 1
}

# Apply a single rule to a single file. Emits one finding per grep match
# on stdout, in a tab-delimited internal format:
#   <severity>\t<rule_id>\t<file>\t<line>\t<excerpt>
# Summary is not included here; report-formatters fetch it separately via
# rule_fm_scalar to avoid re-parsing on every finding.
# Convert a glob (lib/**/*.ex, test/**/*_test.exs) to a regex anchored to
# allow umbrella prefixes. The result is wrapped in (^|/)<regex>$ so that
# the glob matches both top-level (`lib/foo.ex`) and umbrella-nested paths
# (`apps/control/lib/foo.ex`). Single * matches a single path component;
# ** matches any depth (including zero).
critic_glob_to_regex() {
  local glob="$1"
  printf '%s' "$glob" | awk '
    {
      gsub(/\./, "\\.", $0)
      gsub(/\*\*\//, "DOUBLESTARSLASH", $0)
      gsub(/\*\*/, "DOUBLESTAR", $0)
      gsub(/\*/, "[^/]*", $0)
      gsub(/DOUBLESTARSLASH/, "(.*/)?", $0)
      gsub(/DOUBLESTAR/, ".*", $0)
      print
    }
  '
}

# Return 0 if the file matches the rule's `applies_to` globs (or if no
# `applies_to` is declared, in which case the rule is universal). Return 1
# if `applies_to` is declared but no glob matches.
critic_rule_applies_to_file() {
  local rule_path="$1"
  local file="$2"

  local globs
  globs="$(rule_fm_list "$rule_path" applies_to 2>/dev/null)"
  if [ -z "$globs" ]; then
    return 0
  fi

  local glob regex
  while IFS= read -r glob; do
    [ -z "$glob" ] && continue
    regex="$(critic_glob_to_regex "$glob")"
    if [[ "$file" =~ (^|/)${regex}$ ]]; then
      return 0
    fi
  done <<< "$globs"

  return 1
}

critic_apply_rule() {
  local rule_path="$1"
  local file="$2"

  local rule_id severity rule_status
  rule_id="$(rule_fm_scalar "$rule_path" id)"
  severity="$(rule_fm_scalar "$rule_path" severity)"
  rule_status="$(rule_fm_scalar "$rule_path" status)"
  rule_status="${rule_status:-active}"
  [ "$rule_status" != "active" ] && return 0
  [ -z "$rule_id" ] && return 0
  [ -z "$severity" ] && severity=warning

  # Honour applies_to: if the rule declares one or more globs, the file
  # must match at least one for the rule to fire. Rules without applies_to
  # are universal (current behaviour preserved). Globs use suffix anchoring
  # so umbrella layouts (apps/<app>/lib/..., apps/<app>/test/...) match
  # rules declared as `lib/**/*.ex` / `test/**/*_test.exs` (ST0038).
  if ! critic_rule_applies_to_file "$rule_path" "$file"; then
    return 0
  fi

  local block pattern
  block="$(critic_extract_greppable_block "$rule_path")"
  [ -z "$block" ] && return 0
  pattern="$(critic_pattern_from_grep_command "$block")"
  [ -z "$pattern" ] && return 0

  local line_no content
  while IFS= read -r grep_line; do
    [ -z "$grep_line" ] && continue
    line_no="${grep_line%%:*}"
    content="${grep_line#*:}"
    content="$(printf '%s' "$content" | sed 's/\t/    /g' | cut -c1-200)"
    printf '%s\t%s\t%s\t%s\t%s\n' "$severity" "$rule_id" "$file" "$line_no" "$content"
  done < <(grep -nE "$pattern" "$file" 2>/dev/null)
}

# Scan a list of files with a language's rule set. Emits tab-delimited
# findings on stdout. Honours .intent_critic.yml disabled_rules if the
# config path is provided.
critic_scan_files() {
  local lang="$1"
  local config="$2"
  shift 2
  local file rule_path rule_id

  local rule_paths
  rule_paths="$(critic_load_rule_paths "$lang")"
  [ -z "$rule_paths" ] && return 0

  for file in "$@"; do
    [ -f "$file" ] || continue
    while IFS= read -r rule_path; do
      [ -z "$rule_path" ] && continue
      rule_id="$(rule_fm_scalar "$rule_path" id)"
      if [ -n "$config" ] && critic_rule_disabled "$rule_id" "$config"; then
        continue
      fi
      critic_apply_rule "$rule_path" "$file"
    done <<< "$rule_paths"
  done
}

# Filter findings by minimum severity. Reads tab-delimited findings on
# stdin; echoes filtered findings on stdout.
critic_filter_severity() {
  local min_rank="$1"
  awk -F'\t' -v min="$min_rank" '
    {
      sev = $1
      rank = 0
      if (sev == "critical")      rank = 4
      else if (sev == "warning")  rank = 3
      else if (sev == "recommendation") rank = 2
      else if (sev == "style")    rank = 1
      if (rank >= min) print
    }
  '
}

# Format findings as human-readable severity-grouped text.
# Reads tab-delimited findings on stdin; looks up rule summary from
# canon on first sighting. Emits empty output when no findings.
critic_format_text() {
  local tmp
  tmp="$(mktemp -t intent-critic.XXXXXX)"
  cat > "$tmp"
  [ ! -s "$tmp" ] && rm -f "$tmp" && return 0

  local sev
  for sev in critical warning recommendation style; do
    local sev_upper count
    sev_upper="$(printf '%s' "$sev" | tr '[:lower:]' '[:upper:]')"
    count="$(awk -F'\t' -v s="$sev" '$1 == s' "$tmp" | wc -l | tr -d '[:space:]')"
    [ "$count" -eq 0 ] && continue
    printf '\n== %s (%d) ==\n' "$sev_upper" "$count"
    awk -F'\t' -v s="$sev" '$1 == s' "$tmp" | while IFS=$'\t' read -r _ rule_id file line excerpt; do
      printf '[%s] %s at %s:%s\n' "$sev_upper" "$rule_id" "$file" "$line"
      printf '  > %s\n' "$excerpt"
    done
  done

  rm -f "$tmp"
}

# Format findings as JSON array. Reads tab-delimited findings on stdin.
# Empty input → `[]`.
critic_format_json() {
  awk -F'\t' '
    BEGIN { first = 1; printf "[" }
    {
      sev = $1
      id = $2
      file = $3
      line = $4
      excerpt = $5
      gsub(/\\/, "\\\\", excerpt)
      gsub(/"/, "\\\"", excerpt)
      gsub(/\\/, "\\\\", file)
      gsub(/"/, "\\\"", file)
      if (!first) printf ","
      printf "\n  {\"severity\":\"%s\",\"rule_id\":\"%s\",\"file\":\"%s\",\"line\":%s,\"excerpt\":\"%s\"}", sev, id, file, line, excerpt
      first = 0
    }
    END {
      if (first) printf "]\n"
      else printf "\n]\n"
    }
  '
}
