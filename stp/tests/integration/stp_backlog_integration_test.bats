#!/usr/bin/env bats
# Integration tests for STP and Backlog.md
# Tests the integration between STP commands and Backlog functionality

load ../lib/test_helper.bash

setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/stp-backlog-integration-test-XXXXXX")"
  cd "$TEST_TEMP_DIR"
  
  # Initialize STP structure
  mkdir -p stp/{bin,prj/st}
  mkdir -p backlog/tasks
  
  # Copy necessary STP scripts
  cp "${STP_BIN_DIR}/stp" stp/bin/
  cp "${STP_BIN_DIR}/stp_st" stp/bin/
  cp "${STP_BIN_DIR}/stp_task" stp/bin/
  cp "${STP_BIN_DIR}/stp_status" stp/bin/
  cp "${STP_BIN_DIR}/stp_backlog" stp/bin/
  cp "${STP_BIN_DIR}/stp_bl" stp/bin/
  cp "${STP_BIN_DIR}/stp_migrate" stp/bin/
  chmod +x stp/bin/*
  
  # Set up environment
  export STP_HOME="$TEST_TEMP_DIR"
  export PATH="$TEST_TEMP_DIR/stp/bin:$PATH"
  
  # Create a test steel thread
  cat > "stp/prj/st/ST0099.md" << 'EOF'
---
status: Not Started
created: 20250101
---
# ST0099: Test Integration Thread

## Objective
Test the integration between STP and Backlog

## Context
This is a test steel thread for integration testing
EOF

  # Initialize backlog config
  cat > "backlog/config.yml" << 'EOF'
project_name: "test-project"
default_status: "To Do"
statuses: ["To Do", "In Progress", "Done"]
labels: []
milestones: []
date_format: yyyy-mm-dd
max_column_width: 20
backlog_directory: "backlog"
auto_open_browser: false
default_port: 6420
remote_operations: false
auto_commit: false
EOF
}

teardown() {
  cd "$BATS_TEST_TMPDIR"
  rm -rf "$TEST_TEMP_DIR"
}

# Test 1: Verify backlog is available
@test "integration: backlog command is available" {
  run command -v backlog
  [ "$status" -eq 0 ]
  [[ -n "$output" ]]
}

# Test 2: Create task through STP wrapper
@test "integration: stp bl create creates backlog task" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  run stp bl create ST0099 "Integration test task"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Created task"* ]]
  
  # Verify task file was created
  run ls backlog/tasks/
  [ "$status" -eq 0 ]
  [[ "$output" == *"task-"* ]]
  [[ "$output" == *"ST0099"* ]]
}

# Test 3: List tasks through STP wrapper
@test "integration: stp bl list shows created tasks" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create a task first
  stp bl create ST0099 "Test task for listing" >/dev/null 2>&1
  
  run stp bl list
  [ "$status" -eq 0 ]
  [[ "$output" == *"ST0099"* ]]
  [[ "$output" == *"Test task for listing"* ]]
}

# Test 4: Task list command shows backlog tasks
@test "integration: stp task list shows tasks for steel thread" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create multiple tasks
  stp bl create ST0099 "First task" >/dev/null 2>&1
  stp bl create ST0099 "Second task" >/dev/null 2>&1
  
  run stp task list ST0099
  [ "$status" -eq 0 ]
  [[ "$output" == *"Tasks for ST0099:"* ]]
  [[ "$output" == *"First task"* ]]
  [[ "$output" == *"Second task"* ]]
  [[ "$output" == *"[todo]"* ]]
}

# Test 5: Status synchronization
@test "integration: stp status show reflects task completion" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create tasks
  stp bl create ST0099 "Task one" >/dev/null 2>&1
  stp bl create ST0099 "Task two" >/dev/null 2>&1
  
  run stp status show ST0099
  [ "$status" -eq 0 ]
  [[ "$output" == *"Steel Thread: ST0099"* ]]
  [[ "$output" == *"Current Status: Not Started"* ]]
  [[ "$output" == *"Total Tasks: 2"* ]]
  [[ "$output" == *"Todo: 2"* ]]
}

# Test 6: Task naming convention
@test "integration: task files follow expected naming pattern" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create a task
  output=$(stp bl create ST0099 "Naming test task" 2>&1)
  
  # Extract task ID from output
  task_id=$(echo "$output" | grep -oE "task-[0-9]+" | head -1)
  
  # Verify file exists with correct pattern
  run ls "backlog/tasks/${task_id} - ST0099-Naming-test-task.md"
  [ "$status" -eq 0 ]
}

# Test 7: Task content structure
@test "integration: created task has correct YAML structure" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create a task
  stp bl create ST0099 "Structure test" >/dev/null 2>&1
  
  # Find the created task file
  task_file=$(ls backlog/tasks/*ST0099*Structure* | head -1)
  
  # Check YAML frontmatter
  run grep "^id: task-" "$task_file"
  [ "$status" -eq 0 ]
  
  run grep "^title: ST0099 - Structure test" "$task_file"
  [ "$status" -eq 0 ]
  
  run grep "^status: To Do" "$task_file"
  [ "$status" -eq 0 ]
}

# Test 8: STP wrapper prevents git errors
@test "integration: stp bl wrapper prevents git fetch errors" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # The main value of the STP wrapper is preventing git errors
  # by automatically adding --plain to commands
  
  # Create a task
  stp bl create ST0099 "Git error prevention test" >/dev/null 2>&1
  
  # These commands should work without git fetch errors
  run stp bl list
  [ "$status" -eq 0 ]
  # Should not contain git error messages
  [[ "$output" != *"fatal:"* ]]
  [[ "$output" != *"git"* ]]
}


# Test 9: Task status update
@test "integration: updating task status through backlog" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create a task
  output=$(stp bl create ST0099 "Status update test" 2>&1)
  task_id=$(echo "$output" | grep -oE "task-[0-9]+" | head -1)
  
  # Update status
  run stp bl task edit "$task_id" --status "In Progress"
  [ "$status" -eq 0 ]
  
  # Verify status changed
  run stp task list ST0099
  [ "$status" -eq 0 ]
  [[ "$output" == *"[in-progress]"* || "$output" == *"[in_progress]"* ]]
}

# Test 10: Integration with status sync
@test "integration: status sync updates steel thread based on tasks" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create tasks and mark some as done
  output1=$(stp bl create ST0099 "Task 1" 2>&1)
  output2=$(stp bl create ST0099 "Task 2" 2>&1)
  
  task1_id=$(echo "$output1" | grep -oE "task-[0-9]+" | head -1)
  task2_id=$(echo "$output2" | grep -oE "task-[0-9]+" | head -1)
  
  # Mark one task as done
  stp bl task edit "$task1_id" --status Done >/dev/null 2>&1
  
  # Check status before sync
  run grep "^status:" "stp/prj/st/ST0099.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Not Started"* ]]
  
  # Sync status
  run stp status sync ST0099
  [ "$status" -eq 0 ]
  
  # Check status after sync
  run grep "^status:" "stp/prj/st/ST0099.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"In Progress"* ]]
}

# Test 11: Error handling for invalid operations
@test "integration: proper error messages for invalid operations" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Creating task for non-existent steel thread succeeds (backlog doesn't validate existence)
  # But stp task commands should handle invalid formats
  run stp bl create ST9999 "Valid format task"
  [ "$status" -eq 0 ]  # This actually succeeds
  
  # Try to list tasks for invalid ID
  run stp task list INVALID
  [ "$status" -ne 0 ]
  [[ "$output" == *"Invalid steel thread ID format"* ]]
}

# Test 12: Task count accuracy
@test "integration: task counts are accurate across commands" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create exactly 3 tasks
  stp bl create ST0099 "Count test 1" >/dev/null 2>&1
  stp bl create ST0099 "Count test 2" >/dev/null 2>&1
  stp bl create ST0099 "Count test 3" >/dev/null 2>&1
  
  # Check count via status
  run stp status show ST0099
  [ "$status" -eq 0 ]
  [[ "$output" == *"Total Tasks: 3"* ]]
  
  # Check count via task list
  run stp task list ST0099
  [ "$status" -eq 0 ]
  task_count=$(echo "$output" | grep -c "task-[0-9]")
  [ "$task_count" -eq 3 ]
}

# Test 13: Unicode and special character handling
@test "integration: handles special characters in task titles" {
  # Skip if backlog is not installed
  if ! command -v backlog &> /dev/null; then
    skip "Backlog.md not installed"
  fi
  
  # Create task with special characters
  run stp bl create ST0099 "Task with 'quotes' & special chars!"
  [ "$status" -eq 0 ]
  
  # Verify it appears correctly in list
  run stp task list ST0099
  [ "$status" -eq 0 ]
  [[ "$output" == *"quotes"* ]]
  [[ "$output" == *"special chars"* ]]
}

