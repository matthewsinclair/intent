#!/usr/bin/env bats
# Tests for intent help commands (v2.0.0)

load "../lib/test_helper.bash"

@test "help displays general help when no command is specified" {
  local version=$(get_intent_version)
  # Help is a global command - doesn't need project context
  run run_intent help
  assert_success
  assert_output_contains "Intent v${version} - Structured Development Process"
  assert_output_contains "Usage:"
  assert_output_contains "Core:"
  assert_output_contains "st"
  assert_output_contains "init"
  assert_output_contains "doctor"
}

@test "help displays command-specific help when a command is specified" {
  run run_intent help st
  assert_success
  assert_output_contains "No help available for command 'st'"
  assert_output_contains "intent st --help"
}

@test "help shows short descriptions for all commands" {
  run run_intent help
  assert_success
  
  # Check for core commands
  assert_output_contains "st"
  assert_output_contains "init"
  assert_output_contains "doctor"
}

@test "help handles unknown commands correctly" {
  run run_intent help unknown_command
  assert_failure
  assert_output_contains "Unknown command 'unknown_command'"
}

@test "help works with --help flag" {
  run run_intent help --help
  assert_failure
  assert_output_contains "Unknown command '--help'"
}

@test "help shows proper command categories" {
  run run_intent help
  assert_success

  # Check for category headers
  assert_output_contains "Core:"
  assert_output_contains "Utility:"

  # Verify commands exist
  assert_output_contains "st"
  assert_output_contains "init"
}

@test "help shows claude and plugin in core section" {
  run run_intent help
  assert_success
  assert_output_contains "claude"
  assert_output_contains "plugin"
  assert_output_contains "Plugins:"
}

@test "help plugin shows plugin help file" {
  run run_intent help plugin
  assert_success
  assert_output_contains "intent plugin"
  assert_output_contains "Discover"
}

@test "help claude shows claude help file" {
  run run_intent help claude
  assert_success
  assert_output_contains "intent claude"
  assert_output_contains "subagents"
  assert_output_contains "skills"
  assert_output_contains "upgrade"
}

@test "help agents shows corrected agents help" {
  run run_intent help agents
  assert_success
  assert_output_contains "AGENTS.md"
  assert_output_contains "init"
  assert_output_contains "validate"
}