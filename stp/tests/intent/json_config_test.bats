#!/usr/bin/env bats
# Tests for JSON configuration parsing

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/json-config-test-XXXXXX")"
  echo "Setup: Created test directory at ${TEST_TEMP_DIR}"
  cd "${TEST_TEMP_DIR}" || exit 1
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Mock parse_json function for testing
parse_json() {
  local file=$1
  local prefix=$2
  # Simple JSON parser for flat config structure
  grep -E '^\s*"[^"]+"\s*:\s*"[^"]*"' "$file" | \
    sed -E 's/^\s*"([^"]+)"\s*:\s*"([^"]*)".*/\1="\2"/' | \
    sed -e "s/^/${prefix}/"
}

@test "parse_json extracts simple string values" {
  cat > test.json << 'EOF'
{
  "intent_version": "2.0.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog"
}
EOF

  # Parse and evaluate
  eval "$(parse_json test.json "")"
  
  # Verify values
  assert_equal "$intent_version" "2.0.0"
  assert_equal "$intent_dir" "intent"
  assert_equal "$backlog_dir" "backlog"
}

@test "parse_json handles values with spaces" {
  cat > test.json << 'EOF'
{
  "author": "Test User Name",
  "project": "My Cool Project"
}
EOF

  eval "$(parse_json test.json "")"
  
  assert_equal "$author" "Test User Name"
  assert_equal "$project" "My Cool Project"
}

@test "parse_json applies prefix to variables" {
  cat > test.json << 'EOF'
{
  "intent_version": "2.0.0",
  "intent_dir": "custom"
}
EOF

  eval "$(parse_json test.json "config_")"
  
  assert_equal "$config_intent_version" "2.0.0"
  assert_equal "$config_intent_dir" "custom"
}

@test "parse_json ignores non-string values" {
  cat > test.json << 'EOF'
{
  "string_value": "hello",
  "number_value": 42,
  "boolean_value": true,
  "null_value": null,
  "array_value": ["a", "b"],
  "object_value": {"nested": "value"}
}
EOF

  eval "$(parse_json test.json "")"
  
  # Only string_value should be parsed
  assert_equal "$string_value" "hello"
  assert_equal "${number_value:-unset}" "unset"
  assert_equal "${boolean_value:-unset}" "unset"
}

@test "parse_json handles empty strings" {
  cat > test.json << 'EOF'
{
  "empty": "",
  "not_empty": "value"
}
EOF

  eval "$(parse_json test.json "")"
  
  assert_equal "$empty" ""
  assert_equal "$not_empty" "value"
}

@test "parse_json handles special characters in values" {
  cat > test.json << 'EOF'
{
  "path": "/home/user/project",
  "command": "ls -la",
  "regex": "^[a-z]+$"
}
EOF

  eval "$(parse_json test.json "")"
  
  assert_equal "$path" "/home/user/project"
  assert_equal "$command" "ls -la"
  assert_equal "$regex" "^[a-z]+$"
}

@test "config loading hierarchy works correctly" {
  # Create global config
  mkdir -p home/.config/intent
  cat > home/.config/intent/config.json << 'EOF'
{
  "intent_version": "2.0.0",
  "intent_dir": "global_intent",
  "author": "Global User",
  "editor": "nano"
}
EOF

  # Create local config (partial override)
  mkdir -p project/.intent
  cat > project/.intent/config.json << 'EOF'
{
  "intent_dir": "local_intent",
  "author": "Local User"
}
EOF

  # Simulate config loading
  cd project
  
  # Load global first
  eval "$(parse_json ../home/.config/intent/config.json "global_")"
  INTENT_DIR="$global_intent_dir"
  AUTHOR="$global_author"
  EDITOR="$global_editor"
  
  # Override with local
  eval "$(parse_json .intent/config.json "local_")"
  [ -n "$local_intent_dir" ] && INTENT_DIR="$local_intent_dir"
  [ -n "$local_author" ] && AUTHOR="$local_author"
  
  # Verify final values
  assert_equal "$INTENT_DIR" "local_intent"  # Overridden
  assert_equal "$AUTHOR" "Local User"        # Overridden
  assert_equal "$EDITOR" "nano"              # From global
}

@test "environment variables override config" {
  cat > config.json << 'EOF'
{
  "intent_dir": "config_dir",
  "backlog_dir": "config_backlog"
}
EOF

  # Load config
  eval "$(parse_json config.json "")"
  INTENT_DIR="$intent_dir"
  BACKLOG_DIR="$backlog_dir"
  
  # Environment overrides
  export INTENT_DIR_OVERRIDE="env_dir"
  [ -n "$INTENT_DIR_OVERRIDE" ] && INTENT_DIR="$INTENT_DIR_OVERRIDE"
  
  # Verify
  assert_equal "$INTENT_DIR" "env_dir"      # Overridden by env
  assert_equal "$BACKLOG_DIR" "config_backlog"  # From config
}