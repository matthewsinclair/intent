#!/usr/bin/env bats
# Test project commands that require a project context

load "../lib/test_helper.bash"

@test "intent st requires project - shows error outside project" {
  # Run from temp directory without project
  # Use 'list' subcommand to trigger project check
  run run_intent st list
  assert_failure
  assert_output_contains "Not in an Intent project directory"
  assert_output_contains "The 'st' command requires an Intent project"
  assert_output_contains "To create a new project:  intent init"
}

@test "intent bl requires project - shows error outside project" {
  # Use 'list' subcommand to trigger project check
  run run_intent bl list
  assert_failure
  assert_output_contains "Not in an Intent project directory"
  assert_output_contains "The 'bl' command requires an Intent project"
}

@test "intent task requires project - shows error outside project" {
  # Note: intent task with no args shows usage (exit 0)
  # Test with an actual subcommand to trigger project check
  run run_intent task list ST0001
  assert_failure
  assert_output_contains "Not in an Intent project directory"
  assert_output_contains "The 'task' command requires an Intent project"
}

@test "intent st list works inside project" {
  # Create test project
  project_dir=$(create_test_project "Test Project")
  cd "$project_dir"
  
  # Create a test steel thread
  mkdir -p "intent/st/ST0001"
  cat > "intent/st/ST0001/info.md" << EOF
---
id: ST0001
title: Test Steel Thread
status: In Progress
created: 2025-01-01
author: test_user
---

# ST0001: Test Steel Thread

Test description
EOF
  
  run run_intent st list
  assert_success
  assert_output_contains "ST0001"
  assert_output_contains "Test Steel Thread"
}

@test "intent bl works inside project with backlog" {
  # Create test project
  project_dir=$(create_test_project "Test Project")
  cd "$project_dir"
  
  # Create dummy backlog
  touch "backlog/Backlog.md"
  
  run run_intent bl --help
  assert_success
  assert_output_contains "backlog"
}

@test "intent info shows project details when inside project" {
  # Create test project
  project_dir=$(create_test_project "Test Project")
  cd "$project_dir"
  
  # Create some steel threads
  mkdir -p "intent/st/ST0001"
  mkdir -p "intent/st/COMPLETED/ST0002"
  
  run run_intent info
  assert_success
  assert_output_contains "Project:"
  assert_output_contains "Name:            Test Project"
  assert_output_contains "Steel Threads:"
}