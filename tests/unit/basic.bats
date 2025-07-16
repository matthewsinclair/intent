#!/usr/bin/env bats
# Basic tests to verify infrastructure works

load "../lib/test_helper.bash"

@test "test helper is loaded" {
  # This should pass if test helper is loaded correctly
  assert_directory_exists "$INTENT_PROJECT_ROOT"
}

@test "intent executable exists" {
  assert_file_exists "$INTENT_BIN_DIR/intent"
}

@test "can create test project" {
  project_dir=$(create_test_project "Basic Test")
  assert_directory_exists "$project_dir"
  assert_file_exists "$project_dir/.intent/config.json"
}

@test "run_intent function works" {
  run run_intent --version
  assert_output_contains "Intent version"
}