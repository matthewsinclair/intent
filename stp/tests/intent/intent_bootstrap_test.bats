#!/usr/bin/env bats
# Tests for the intent_bootstrap command (v2.0.0)

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/intent-bootstrap-test-XXXXXX")"
  echo "Setup: Created test directory at ${TEST_TEMP_DIR}"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Backup real home config if it exists
  if [ -d "$HOME/.config/intent" ]; then
    BACKUP_CONFIG=true
    mv "$HOME/.config/intent" "$HOME/.config/intent.bak.$$"
  fi
  
  # Set test HOME
  export ORIG_HOME="$HOME"
  export HOME="${TEST_TEMP_DIR}/home"
  mkdir -p "$HOME"
}

# Clean up after each test
teardown() {
  # Restore original HOME
  export HOME="$ORIG_HOME"
  
  # Restore backed up config
  if [ "$BACKUP_CONFIG" = true ]; then
    rm -rf "$ORIG_HOME/.config/intent"
    mv "$ORIG_HOME/.config/intent.bak.$$" "$ORIG_HOME/.config/intent"
  fi
  
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "intent_bootstrap creates global config directory" {
  # Verify directory doesn't exist yet
  assert_not_exists "$HOME/.config/intent"
  
  # TODO: Run bootstrap
  # export INTENT_HOME="${TEST_TEMP_DIR}/intent"
  # mkdir -p "$INTENT_HOME/bin"
  # run "$INTENT_HOME/bin/intent_bootstrap"
  
  # Verify directory was created
  # assert_dir_exists "$HOME/.config/intent"
}

@test "intent_bootstrap creates default config.json" {
  # TODO: Run bootstrap
  # export INTENT_HOME="${TEST_TEMP_DIR}/intent"
  # run intent_bootstrap
  
  # Verify config was created with correct content
  # assert_file_exists "$HOME/.config/intent/config.json"
  # assert_file_contains "$HOME/.config/intent/config.json" '"intent_version": "2.0.0"'
  # assert_file_contains "$HOME/.config/intent/config.json" '"intent_dir": "intent"'
  # assert_file_contains "$HOME/.config/intent/config.json" '"backlog_dir": "backlog"'
}

@test "intent_bootstrap detects INTENT_HOME from script location" {
  # Create mock intent installation
  mkdir -p "${TEST_TEMP_DIR}/my-intent/bin"
  mkdir -p "${TEST_TEMP_DIR}/my-intent/lib"
  
  # TODO: Create mock intent_bootstrap script
  # cat > "${TEST_TEMP_DIR}/my-intent/bin/intent_bootstrap" << 'EOF'
  # #!/bin/bash
  # # Mock bootstrap script
  # EOF
  # chmod +x "${TEST_TEMP_DIR}/my-intent/bin/intent_bootstrap"
  
  # Run without INTENT_HOME set
  # cd "${TEST_TEMP_DIR}"
  # run "./my-intent/bin/intent_bootstrap"
  # assert_success
  # assert_output --partial "Found intent installation at: ${TEST_TEMP_DIR}/my-intent"
}

@test "intent_bootstrap fails if INTENT_HOME invalid" {
  export INTENT_HOME="/nonexistent/path"
  
  # TODO: Run bootstrap
  # run intent_bootstrap
  # assert_failure
  # assert_output --partial "ERROR: Invalid INTENT_HOME"
}

@test "intent_bootstrap provides PATH setup instructions" {
  # TODO: Run bootstrap
  # export INTENT_HOME="${TEST_TEMP_DIR}/intent"
  # run intent_bootstrap
  
  # assert_output --partial "export INTENT_HOME="
  # assert_output --partial "export PATH="
  # assert_output --partial '$INTENT_HOME/bin:$PATH'
}

@test "intent_bootstrap runs doctor after setup" {
  # TODO: Run bootstrap
  # Should see doctor output at the end
  # assert_output --partial "Intent Doctor v2.0.0"
}

@test "intent_bootstrap handles existing config gracefully" {
  # Create existing config
  mkdir -p "$HOME/.config/intent"
  echo '{"intent_version": "1.0.0"}' > "$HOME/.config/intent/config.json"
  
  # TODO: Run bootstrap
  # Should not overwrite existing config
  # run intent_bootstrap
  # assert_file_contains "$HOME/.config/intent/config.json" '"intent_version": "1.0.0"'
}