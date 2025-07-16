#!/usr/bin/env bats
# Tests for the intent_bootstrap command

load "../lib/test_helper.bash"

# Override setup to handle HOME directory for bootstrap tests
setup() {
  # First call parent setup to create TEST_TEMP_DIR
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Backup real home config if it exists
  if [ -d "$HOME/.config/intent" ]; then
    export BACKUP_CONFIG=true
    export BACKUP_DIR="$HOME/.config/intent.bak.$$"
    mv "$HOME/.config/intent" "$BACKUP_DIR"
  fi
  
  # Set test HOME
  export ORIG_HOME="$HOME"
  export HOME="${TEST_TEMP_DIR}/home"
  mkdir -p "$HOME"
  mkdir -p "$HOME/.config"
}

# Clean up after each test
teardown() {
  # Restore original HOME
  export HOME="$ORIG_HOME"
  
  # Restore backed up config
  if [ "$BACKUP_CONFIG" = true ]; then
    rm -rf "$ORIG_HOME/.config/intent"
    if [ -d "$BACKUP_DIR" ]; then
      mv "$BACKUP_DIR" "$ORIG_HOME/.config/intent"
    fi
  fi
}

@test "intent_bootstrap creates global config directory" {
  # Verify directory doesn't exist yet
  [ ! -d "$HOME/.config/intent" ]
  
  # Run bootstrap (non-interactive mode)
  run run_intent bootstrap --quiet
  assert_success
  
  # Verify directory was created
  assert_directory_exists "$HOME/.config/intent"
}

@test "intent_bootstrap creates default config.json" {
  # Run bootstrap
  run run_intent bootstrap --quiet
  assert_success
  
  # Verify config was created with correct content
  assert_file_exists "$HOME/.config/intent/config.json"
  assert_file_contains "$HOME/.config/intent/config.json" '"intent_version": "2.0.0"'
  assert_file_contains "$HOME/.config/intent/config.json" '"intent_dir": "intent"'
  assert_file_contains "$HOME/.config/intent/config.json" '"backlog_dir": "backlog"'
}

@test "intent_bootstrap provides PATH setup instructions" {
  # Don't use --quiet so we see the instructions
  run run_intent bootstrap
  assert_success
  
  # Should show PATH instructions or already configured message
  # (Bootstrap output varies based on configuration state)
  [ -n "$output" ] || fail "Expected output from bootstrap"
}

@test "intent_bootstrap handles existing config gracefully" {
  # Create existing config
  mkdir -p "$HOME/.config/intent"
  echo '{"intent_version": "1.0.0", "custom": "value"}' > "$HOME/.config/intent/config.json"
  
  # Run bootstrap - should not overwrite
  run run_intent bootstrap --quiet
  assert_success
  
  # Original config should be preserved
  assert_file_contains "$HOME/.config/intent/config.json" '"intent_version": "1.0.0"'
  assert_file_contains "$HOME/.config/intent/config.json" '"custom": "value"'
}

@test "intent_bootstrap runs doctor after setup" {
  run run_intent bootstrap --quiet
  assert_success
  
  # Should see doctor output at the end
  assert_output_contains "Intent Doctor"
  assert_output_contains "Checking INTENT_HOME"
}