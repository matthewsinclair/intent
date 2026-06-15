#!/usr/bin/env bats
# Core functionality tests - the essential tests that must pass

load "../lib/test_helper.bash"

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
  assert_output_contains "not in an Intent project directory"
  assert_output_contains "'st' requires an Intent project"
}

@test "no more silent failures - all commands give feedback" {
  # Test a project command outside project
  run run_intent st new "Test"
  assert_failure
  # Should see error, not silence
  [ -n "$output" ] || fail "Expected output but got none"
  assert_output_contains "not in an Intent project"
}


