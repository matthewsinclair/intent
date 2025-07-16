#!/usr/bin/env bats
# End-to-end integration tests

load "../lib/test_helper.bash"

@test "complete workflow: create project, add steel thread, list it" {
  # Start in temp directory
  original_dir=$(pwd)
  
  # Create new project
  mkdir test_project
  cd test_project
  
  # Initialize project
  run run_intent init "Integration Test Project"
  # Skip for now as init needs interactive input
  
  # Create project manually for testing
  cd "$original_dir"
  project_dir=$(create_test_project "Integration Test")
  cd "$project_dir"
  
  # Verify project structure
  assert_directory_exists "intent/st"
  assert_file_exists ".intent/config.json"
  
  # Check info shows project details
  run run_intent info
  assert_success
  assert_output_contains "Project:"
  assert_output_contains "Name:            Integration Test"
  
  # Create a steel thread manually (st new needs editor)
  mkdir -p "intent/st/ST0001"
  cat > "intent/st/ST0001/info.md" << EOF
---
id: ST0001
title: Test Feature Implementation
status: In Progress
created: $(date +%Y-%m-%d)
author: test_user
intent_version: 2.0.0
---

# ST0001: Test Feature Implementation

## Metadata
- **Status**: In Progress
- **Created**: $(date +%Y-%m-%d)
- **Author**: test_user

## Description
This is a test steel thread for integration testing.
EOF
  
  # List steel threads
  run run_intent st list
  assert_success
  assert_output_contains "ST0001"
  assert_output_contains "Test Feature Implement"  # Title is truncated to ~25 chars in list
  assert_output_contains "In Progress"
  
  # Check st show works
  run run_intent st show ST0001
  assert_success
  assert_output_contains "Test Feature"  # Title might be truncated in list view
}

@test "error handling: project commands fail gracefully outside project" {
  # Commands that require project
  local project_commands=("st" "bl" "task" "migrate" "status")
  
  for cmd in "${project_commands[@]}"; do
    run run_intent "$cmd"
    assert_failure
    assert_output_contains "Not in an Intent project directory"
    assert_output_contains "The '$cmd' command requires an Intent project"
  done
}

@test "global commands work outside project" {
  # Commands that work anywhere
  run run_intent
  assert_success
  assert_output_contains "Intent: The Steel Thread Process"
  
  run run_intent help
  assert_success
  assert_output_contains "Usage: intent"
  
  run run_intent doctor
  assert_success
  assert_output_contains "Intent Doctor"
  
  run run_intent info
  assert_success
  assert_output_contains "Not in an Intent project directory"
}