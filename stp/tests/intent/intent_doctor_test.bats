#!/usr/bin/env bats
# Tests for the intent_doctor command (v2.0.0)

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/intent-doctor-test-XXXXXX")"
  echo "Setup: Created test directory at ${TEST_TEMP_DIR}"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Set test HOME
  export ORIG_HOME="$HOME"
  export HOME="${TEST_TEMP_DIR}/home"
  mkdir -p "$HOME"
  
  # Set test INTENT_HOME
  export INTENT_HOME="${TEST_TEMP_DIR}/intent"
  mkdir -p "$INTENT_HOME/bin"
}

# Clean up after each test
teardown() {
  # Restore original HOME
  export HOME="$ORIG_HOME"
  unset INTENT_HOME
  
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "intent_doctor checks INTENT_HOME environment variable" {
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking INTENT_HOME... OK"
}

@test "intent_doctor detects missing INTENT_HOME" {
  unset INTENT_HOME
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking INTENT_HOME... ERROR: Not set"
}

@test "intent_doctor checks intent executable" {
  # Create mock executable
  touch "$INTENT_HOME/bin/intent"
  chmod +x "$INTENT_HOME/bin/intent"
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking intent executable... OK"
}

@test "intent_doctor detects missing executable" {
  # Don't create executable
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking intent executable... ERROR"
}

@test "intent_doctor checks global config" {
  # Create valid config
  mkdir -p "$HOME/.config/intent"
  cat > "$HOME/.config/intent/config.json" << 'EOF'
{
  "intent_version": "2.0.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog"
}
EOF

  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking global config... OK"
}

@test "intent_doctor detects invalid JSON in config" {
  # Create invalid JSON
  mkdir -p "$HOME/.config/intent"
  echo "invalid json {" > "$HOME/.config/intent/config.json"
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking global config... ERROR: Invalid JSON syntax"
}

@test "intent_doctor checks PATH includes intent/bin" {
  export PATH="$INTENT_HOME/bin:$PATH"
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking PATH... OK"
}

@test "intent_doctor warns if PATH missing intent/bin" {
  export PATH="/usr/bin:/bin"
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Checking PATH... WARNING"
  # assert_output --partial "not in PATH"
}

@test "intent_doctor --fix creates missing global config" {
  # Ensure config doesn't exist
  rm -rf "$HOME/.config/intent"
  
  # TODO: Run doctor with fix
  # run intent_doctor --fix
  # assert_output --partial "FIX: Creating default global config"
  # assert_file_exists "$HOME/.config/intent/config.json"
}

@test "intent_doctor --fix backs up invalid config" {
  # Create invalid config
  mkdir -p "$HOME/.config/intent"
  echo "bad config" > "$HOME/.config/intent/config.json"
  
  # TODO: Run doctor with fix
  # run intent_doctor --fix
  # assert_output --partial "FIX: Backing up and creating new config"
  # assert_file_exists "$HOME/.config/intent/config.json.bak"
  # assert_file_exists "$HOME/.config/intent/config.json"
}

@test "intent_doctor shows summary with counts" {
  # TODO: Run doctor
  # run intent_doctor
  # assert_output --partial "Summary:"
  # assert_output --partial "Errors:"
  # assert_output --partial "Warnings:"
}

@test "intent_doctor returns error code on failures" {
  # Create scenario with errors
  unset INTENT_HOME
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_failure
}

@test "intent_doctor returns success when all checks pass" {
  # Create valid environment
  touch "$INTENT_HOME/bin/intent"
  chmod +x "$INTENT_HOME/bin/intent"
  mkdir -p "$HOME/.config/intent"
  cat > "$HOME/.config/intent/config.json" << 'EOF'
{
  "intent_version": "2.0.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog"
}
EOF
  export PATH="$INTENT_HOME/bin:$PATH"
  
  # TODO: Run doctor
  # run intent_doctor
  # assert_success
  # assert_output --partial "✓ All checks passed!"
}