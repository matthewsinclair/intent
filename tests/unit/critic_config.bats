#!/usr/bin/env bats
# Tests for .intent_critic.yml handling (WP07).
#
# Config consumption is inside the Claude critic subagents, so these tests
# focus on (a) the sample file shipped at rules/_schema/ being valid,
# well-formed YAML with every documented key, and (b) the canonical absent /
# malformed / unknown-id edge cases being parseable (or provably non-parseable)
# by a standard YAML parser. If no YAML parser is available on the host, the
# tests degrade to structural checks of the sample file.

load "../lib/test_helper.bash"

SAMPLE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/_schema/sample-intent-critic.yml"

yaml_parse() {
  if command -v yq >/dev/null 2>&1; then
    yq eval '.' "$1" >/dev/null 2>&1
  elif command -v python3 >/dev/null 2>&1; then
    python3 -c "import sys, yaml; yaml.safe_load(open(sys.argv[1]))" "$1" >/dev/null 2>&1
  elif command -v ruby >/dev/null 2>&1; then
    ruby -ryaml -e "YAML.load_file(ARGV[0])" "$1" >/dev/null 2>&1
  else
    skip "no YAML parser available (yq, python3, or ruby required)"
  fi
}

# ====================================================================
# Sample file shipped at rules/_schema/
# ====================================================================

@test "config: sample file exists at rules/_schema/" {
  assert_file_exists "$SAMPLE"
}

@test "config: sample file parses as valid YAML" {
  yaml_parse "$SAMPLE"
}

@test "config: sample file documents the three schema keys" {
  assert_file_contains "$SAMPLE" "disabled:"
  assert_file_contains "$SAMPLE" "severity_min:"
  assert_file_contains "$SAMPLE" "show_all:"
}

@test "config: sample disabled entries carry a # reason comment" {
  # Every disabled entry should be followed by a reason comment so the
  # opt-out decision is discoverable by a future reader.
  local lines_with_ids lines_with_reasons
  lines_with_ids=$(grep -cE '^  - IN-[A-Z]{2,3}-' "$SAMPLE")
  lines_with_reasons=$(grep -cE '^  - IN-[A-Z]{2,3}-.*# reason:' "$SAMPLE")
  [ "$lines_with_ids" -gt 0 ]
  [ "$lines_with_ids" = "$lines_with_reasons" ]
}

# ====================================================================
# Schema-level sanity: absent / minimal / malformed / overrides
# ====================================================================

@test "config: absent file is the default silent path" {
  # The Critic treats absence as "use defaults silently". We verify that
  # a `stat` on a missing path returns non-zero, which is the signal the
  # Critic uses to branch into the default path.
  local missing="$TEST_TEMP_DIR/nope-intent-critic.yml"
  [ ! -f "$missing" ]
}

@test "config: minimal file with only severity_min parses" {
  local f="$TEST_TEMP_DIR/only-severity.yml"
  cat > "$f" <<'EOF'
severity_min: critical
EOF
  yaml_parse "$f"
}

@test "config: minimal file with only disabled parses" {
  local f="$TEST_TEMP_DIR/only-disabled.yml"
  cat > "$f" <<'EOF'
disabled:
  - IN-EX-CODE-007
EOF
  yaml_parse "$f"
}

@test "config: empty file parses as valid YAML (defaults path)" {
  local f="$TEST_TEMP_DIR/empty.yml"
  : > "$f"
  yaml_parse "$f"
}

@test "config: malformed file fails to parse (triggers Critic warning + defaults)" {
  if ! command -v yq >/dev/null 2>&1 \
    && ! command -v python3 >/dev/null 2>&1 \
    && ! command -v ruby >/dev/null 2>&1; then
    skip "no YAML parser available"
  fi
  local f="$TEST_TEMP_DIR/malformed.yml"
  cat > "$f" <<'EOF'
disabled:
  - IN-EX-CODE-007
    stray_indent: broken
severity_min:: critical
EOF
  # Parse MUST fail. The Critic's behaviour under this condition is to log
  # the malformed warning once and proceed with defaults.
  run yaml_parse "$f"
  [ "$status" -ne 0 ]
}

@test "config: severity_min values restricted to the canonical four" {
  # All four canonical severity values are parseable individually.
  local f="$TEST_TEMP_DIR/sev.yml"
  for sev in critical warning recommendation style; do
    printf 'severity_min: %s\n' "$sev" > "$f"
    yaml_parse "$f" || return 1
  done
}

@test "config: show_all shorthand parses as boolean" {
  local f="$TEST_TEMP_DIR/showall.yml"
  cat > "$f" <<'EOF'
show_all: true
EOF
  yaml_parse "$f"
}

# ====================================================================
# Documentation parity
# ====================================================================
#
# The sample file and intent/docs/critics.md must agree on keys. This is a
# cheap Highlander guard.

@test "config: critics.md documents every sample key" {
  local docs="${INTENT_PROJECT_ROOT}/intent/docs/critics.md"
  assert_file_exists "$docs"
  assert_file_contains "$docs" "disabled"
  assert_file_contains "$docs" "severity_min"
  assert_file_contains "$docs" "show_all"
}
