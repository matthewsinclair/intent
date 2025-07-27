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
  run run_intent help
  assert_success
  assert_output_contains "Intent v2.1.0 - Structured Development Process"
  assert_output_contains "Usage: intent <command>"
}

@test "intent doctor works anywhere" {
  run run_intent doctor
  assert_success
  assert_output_contains "Intent Doctor v2.1.0"
  assert_output_contains "Checking INTENT_HOME"
}

@test "intent info works anywhere" {
  run run_intent info
  assert_success
  assert_output_contains "Intent: The Steel Thread Process"
  assert_output_contains "Installation:"
}

@test "intent version works anywhere" {
  run run_intent version
  assert_success
  assert_output_contains "Intent version 2.1.0"
}

@test "intent --version works anywhere" {
  run run_intent --version
  assert_success
  assert_output_contains "Intent version 2.1.0"
}

@test "intent bootstrap works anywhere" {
  # Just check it runs without error - don't actually bootstrap
  run run_intent bootstrap --help
  assert_success
  assert_output_contains "bootstrap"
}

@test "intent init works in empty directory" {
  # Don't actually run init, just check help
  run run_intent init --help
  # Note: init --help exits with status 1
  assert_failure
  assert_output_contains "Initialize a new Intent v2.1.0 project"
}

@test "intent handles unknown command gracefully" {
  run run_intent nonexistentcommand
  assert_failure
  assert_output_contains "Unknown command 'nonexistentcommand'"
  assert_output_contains "Run 'intent help' for usage"
}