#!/usr/bin/env bats
# Tests for intent init commands (v2.1.0)

load "../lib/test_helper.bash"

@test "init uses directory name if no project name given" {
  # Create temporary directory for init
  test_dir=$(mktemp -d)
  cd "$test_dir"
  
  run run_intent init
  assert_success
  assert_directory_exists ".intent"
  
  # Cleanup
  cd - > /dev/null
  rm -rf "$test_dir"
}

@test "init creates a project in the current directory by default" {
  # Create temporary directory for init
  test_dir=$(mktemp -d)
  cd "$test_dir"
  
  run run_intent init "Test Project"
  assert_success
  
  # Check if project structure was created
  assert_directory_exists ".intent"
  assert_directory_exists "intent"
  assert_directory_exists "intent/st"
  assert_directory_exists "intent/docs"
  assert_directory_exists "intent/llm"
  assert_file_exists ".intent/config.json"
  
  # Check config content
  assert_file_contains ".intent/config.json" '"project_name": "Test Project"'
  assert_file_contains ".intent/config.json" '"version": "2.1.0"'
  
  # Cleanup
  cd - > /dev/null
  rm -rf "$test_dir"
}

@test "init creates a project in a specified directory" {
  # Create temporary directory
  test_dir=$(mktemp -d)
  target_dir="$test_dir/my-project"
  
  mkdir -p "$target_dir"
  cd "$target_dir"
  run run_intent init "Test Project"
  assert_success
  
  # Check if project was created in specified directory
  assert_directory_exists "$target_dir/.intent"
  assert_directory_exists "$target_dir/intent"
  assert_directory_exists "$target_dir/intent/st"
  assert_file_exists "$target_dir/.intent/config.json"
  
  # Cleanup
  rm -rf "$test_dir"
}

@test "init creates proper configuration file" {
  # Create temporary directory
  test_dir=$(mktemp -d)
  cd "$test_dir"
  
  run run_intent init "My Test Project"
  assert_success
  
  # Check configuration content
  assert_file_contains ".intent/config.json" '"project_name": "My Test Project"'
  assert_file_contains ".intent/config.json" '"version": "2.1.0"'
  assert_file_contains ".intent/config.json" '"created":'
  assert_file_contains ".intent/config.json" '"author":'
  
  # Cleanup
  cd - > /dev/null
  rm -rf "$test_dir"
}

@test "init creates required project files" {
  # Create temporary directory
  test_dir=$(mktemp -d)
  cd "$test_dir"
  
  run run_intent init "Test Project"
  assert_success
  
  # Check for essential files
  assert_file_exists "CLAUDE.md"
  
  # Cleanup
  cd - > /dev/null
  rm -rf "$test_dir"
}

@test "init fails on existing Intent project" {
  # Create temporary directory with existing project
  test_dir=$(mktemp -d)
  cd "$test_dir"
  
  # Create existing project
  mkdir -p .intent
  echo '{"name": "Existing"}' > .intent/config.json
  
  run run_intent init "New Project"
  assert_failure
  assert_output_contains "already an Intent project"
  
  # Cleanup
  cd - > /dev/null
  rm -rf "$test_dir"
}


@test "init creates proper directory permissions" {
  # Create temporary directory
  test_dir=$(mktemp -d)
  cd "$test_dir"
  
  run run_intent init "Test Project"
  assert_success
  
  # Check directory permissions (should be readable/writable/executable by owner)
  [ -r ".intent" ] || fail ".intent not readable"
  [ -w ".intent" ] || fail ".intent not writable"
  [ -x ".intent" ] || fail ".intent not executable"
  
  [ -r "intent" ] || fail "intent not readable"
  [ -w "intent" ] || fail "intent not writable"
  [ -x "intent" ] || fail "intent not executable"
  
  # Cleanup
  cd - > /dev/null
  rm -rf "$test_dir"
}

@test "init respects author from git config" {
  # Create temporary directory
  test_dir=$(mktemp -d)
  cd "$test_dir"
  
  # Set up author via environment
  export INTENT_AUTHOR="Test Author"
  
  run run_intent init "Test Project"
  assert_success
  
  # Check if author was picked up from git
  assert_file_contains ".intent/config.json" '"author": "Test Author"'
  
  # Cleanup
  cd - > /dev/null
  rm -rf "$test_dir"
}