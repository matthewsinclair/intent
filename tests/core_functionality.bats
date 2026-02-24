#!/usr/bin/env bats
# Core functionality tests - the essential tests that must pass

load "lib/test_helper.bash"

@test "intent shows info when run with no args" {
  run run_intent
  assert_success
  assert_output_contains "Intent: The Steel Thread Process"
  assert_output_contains "Installation:"
}

@test "intent help works globally" {
  run run_intent help
  assert_success
  assert_output_contains "Usage: intent"
}

@test "intent st list requires project and shows clear error" {
  # Outside project
  run run_intent st list
  assert_failure
  assert_output_contains "Not in an Intent project directory"
  assert_output_contains "The 'st' command requires an Intent project"
}

@test "no more silent failures - all commands give feedback" {
  # Test a project command outside project
  run run_intent st new "Test"
  assert_failure
  # Should see error, not silence
  [ -n "$output" ] || fail "Expected output but got none"
  assert_output_contains "Not in an Intent project"
}

@test "backup uses new .backup_ prefix not .stp_backup_" {
  project_dir=$(create_test_project "Backup Test")
  cd "$project_dir"
  
  # Source helpers
  source "${INTENT_BIN_DIR}/intent_helpers"
  
  # Create backup
  create_project_backup "$project_dir"
  
  # Check new prefix is used
  backup_dirs=(.backup_*)
  [ -d "${backup_dirs[0]}" ] || fail "No .backup_* directory found"
  
  # Old prefix should not exist
  if ls .stp_backup_* 2>/dev/null; then
    fail "Found old .stp_backup_* directory"
  fi
}

