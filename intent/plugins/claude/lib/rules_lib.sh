#!/bin/bash
# rules_lib.sh -- Shared rule-library primitives for Intent Claude plugin
#
# Consumers:
#   - intent/plugins/claude/bin/intent_claude_rules  (list, show, validate, index)
#   - intent/plugins/claude/lib/critic_runner.sh     (headless critic runner; ST0035/WP-05)
#
# Exports:
#   Functions: enumerate_canon_rule_files, enumerate_ext_rule_files,
#              enumerate_all_rule_files, rule_file_provenance,
#              rule_frontmatter, rule_fm_scalar, rule_fm_list.
#   Variables: CANON_RULES_ROOT, EXT_BASE, LANG_SUBDIRS.
#
# Expects INTENT_HOME to be set by the caller. Falls back to a path probe
# relative to this file's location if not set.

if [ -z "${INTENT_HOME:-}" ]; then
  _rules_lib_self="${BASH_SOURCE[0]}"
  while [ -L "$_rules_lib_self" ]; do
    _rules_lib_self="$(readlink "$_rules_lib_self")"
  done
  INTENT_HOME="$(cd "$(dirname "$_rules_lib_self")/../../../.." && pwd)"
  export INTENT_HOME
  unset _rules_lib_self
fi

: "${CANON_RULES_ROOT:=$INTENT_HOME/intent/plugins/claude/rules}"
: "${EXT_BASE:=${INTENT_EXT_DIR:-$HOME/.intent/ext}}"
: "${LANG_SUBDIRS:=agnostic elixir rust swift lua shell}"

# Emit every canon RULE.md path on stdout (excludes _schema/, _attribution/,
# and any other top-level _* directories under rules/).
enumerate_canon_rule_files() {
  local sub
  [ -d "$CANON_RULES_ROOT" ] || return 0
  for sub in $LANG_SUBDIRS; do
    local dir="$CANON_RULES_ROOT/$sub"
    [ -d "$dir" ] || continue
    find "$dir" -name 'RULE.md' -type f 2>/dev/null
  done
}

# Emit every ext RULE.md path on stdout. Honours INTENT_EXT_DISABLE and
# INTENT_EXT_DIR. Extensions under ${ext_base}/*/rules/<lang>/...
enumerate_ext_rule_files() {
  [ "${INTENT_EXT_DISABLE:-}" = "1" ] && return 0
  [ -d "$EXT_BASE" ] || return 0

  local ext_dir ext_name ext_rules sub dir
  for ext_dir in "$EXT_BASE"/*/; do
    [ -d "$ext_dir" ] || continue
    ext_name="$(basename "$ext_dir")"
    case "$ext_name" in
      .*|_*) continue ;;
    esac
    ext_rules="${ext_dir%/}/rules"
    [ -d "$ext_rules" ] || continue
    for sub in $LANG_SUBDIRS; do
      dir="$ext_rules/$sub"
      [ -d "$dir" ] || continue
      find "$dir" -name 'RULE.md' -type f 2>/dev/null
    done
  done
}

# Emit every RULE.md path (canon first, then ext) on stdout.
enumerate_all_rule_files() {
  enumerate_canon_rule_files
  enumerate_ext_rule_files
}

# Tag a rule-file path with its provenance: "canon" or "ext:<name>".
rule_file_provenance() {
  local path="$1"
  case "$path" in
    "$CANON_RULES_ROOT"/*) echo "canon"; return 0 ;;
    "$EXT_BASE"/*)
      local trimmed="${path#$EXT_BASE/}"
      local ext_name="${trimmed%%/*}"
      echo "ext:${ext_name}"
      return 0
      ;;
    *)
      echo "unknown"
      return 0
      ;;
  esac
}

# Extract the YAML frontmatter body (between the first two --- lines)
# from a RULE.md file.
rule_frontmatter() {
  local path="$1"
  awk '
    /^---$/ { count++; if (count == 1) next; if (count == 2) exit; next }
    count == 1 { print }
  ' "$path"
}

# Read a simple top-level scalar key from the frontmatter. Returns empty if
# absent. Value trimming strips surrounding single/double quotes.
rule_fm_scalar() {
  local path="$1"
  local key="$2"
  rule_frontmatter "$path" \
    | awk -v k="$key" '
        $0 ~ "^"k":" {
          sub("^"k":[[:space:]]*", "")
          sub("[[:space:]]+$", "")
          gsub("^[\"'\'']", "")
          gsub("[\"'\'']$", "")
          print
          exit
        }
      '
}

# Read a flat list-of-strings frontmatter key. Accepts either inline-array
# form `key: [a, b]` or block form. One item per line on stdout. Block-form
# detection requires that list items are indented with `-` below the key and
# terminated by the next top-level key or end of frontmatter.
rule_fm_list() {
  local path="$1"
  local key="$2"
  rule_frontmatter "$path" \
    | awk -v k="$key" '
        BEGIN { inside = 0 }
        # inline array on the key line: key: [a, b, "c"]
        $0 ~ "^"k":[[:space:]]*\\[" {
          line = $0
          sub(".*\\[", "", line)
          sub("\\].*", "", line)
          gsub("[[:space:]]", "", line)
          if (line == "") { next }
          n = split(line, parts, ",")
          for (i = 1; i <= n; i++) {
            it = parts[i]
            gsub("^[\"'\'']", "", it)
            gsub("[\"'\'']$", "", it)
            if (it != "") print it
          }
          next
        }
        # block form opens with key:
        $0 ~ "^"k":[[:space:]]*$" { inside = 1; next }
        inside == 1 {
          if ($0 ~ /^[[:space:]]+-/) {
            it = $0
            sub("^[[:space:]]+-[[:space:]]*", "", it)
            gsub("[[:space:]]+$", "", it)
            gsub("^[\"'\'']", "", it)
            gsub("[\"'\'']$", "", it)
            if (it != "") print it
          } else if ($0 ~ /^[A-Za-z]/) {
            inside = 0
          }
        }
      '
}
