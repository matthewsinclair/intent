#!/usr/bin/env bats
# Test global commands that should work without a project context

load "../lib/test_helper.bash"

@test "intent with no args shows info" {
  run run_intent
  assert_success
  assert_output_contains "Intent: The Steel Thread Process"
  assert_output_contains "Installation:"
  assert_output_contains "INTENT_HOME:"
}

@test "intent help works anywhere" {
  local version=$(get_intent_version)
  run run_intent help
  assert_success
  assert_output_contains "Intent v${version} - Structured Development Process"
  assert_output_contains "Usage: intent <command>"
}

@test "intent doctor works anywhere" {
  local version=$(get_intent_version)
  run run_intent doctor
  assert_success
  assert_output_contains "Intent Doctor v${version}"
  assert_output_contains "Checking INTENT_HOME"
}

@test "intent info works anywhere" {
  run run_intent info
  assert_success
  assert_output_contains "Intent: The Steel Thread Process"
  assert_output_contains "Installation:"
}

@test "intent version works anywhere" {
  local version=$(get_intent_version)
  run run_intent version
  assert_success
  assert_output_contains "Intent version ${version}"
}

@test "intent --version works anywhere" {
  local version=$(get_intent_version)
  run run_intent --version
  assert_success
  assert_output_contains "Intent version ${version}"
}

@test "intent bootstrap works anywhere" {
  # Just check it runs without error - don't actually bootstrap
  run run_intent bootstrap --help
  assert_success
  assert_output_contains "bootstrap"
}

@test "intent init works in empty directory" {
  local version=$(get_intent_version)
  # Don't actually run init, just check help
  run run_intent init --help
  # Note: init --help exits with status 1
  assert_failure
  assert_output_contains "Initialize a new Intent v${version} project"
}

@test "intent handles unknown command gracefully" {
  run run_intent nonexistentcommand
  assert_failure
  assert_output_contains "Unknown command 'nonexistentcommand'"
  assert_output_contains "Run 'intent help' for usage"
}