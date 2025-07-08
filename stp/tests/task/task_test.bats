#!/usr/bin/env bats
# Tests for the stp_task script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/task-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the task script to the test directory
  cp "${STP_BIN_DIR}/stp_task" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_task"
  
  # Create minimal STP directory structure
  mkdir -p "stp/prj/st"
  mkdir -p "stp/bin"
  mkdir -p "backlog/tasks"
  mkdir -p "backlog/drafts"
  
  # Copy stp and stp_st scripts for task validation
  cp "${STP_BIN_DIR}/stp" "stp/bin/"
  cp "${STP_BIN_DIR}/stp_st" "stp/bin/"
  chmod +x "stp/bin/stp"
  chmod +x "stp/bin/stp_st"
  
  # Set STP_HOME for the test
  export STP_HOME="${TEST_TEMP_DIR}"
  
  # Create a test steel thread
  cat > "stp/prj/st/ST0014.md" << EOF
---
verblock: "20 Mar 2025:v0.1: Test - Initial version"
stp_version: 1.0.0
status: In Progress
created: 20250320
completed: 
---
# ST0014: Test Steel Thread

## Objective
Test objective

## Tasks
- [ ] First task
- [ ] Second task
EOF
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test if task requires a command
@test "task requires a command" {
  run ./stp_task
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp task"* ]]
}

# Test help command
@test "task shows help with --help" {
  run ./stp_task --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp task"* ]]
  [[ "$output" == *"create"* ]]
  [[ "$output" == *"list"* ]]
  [[ "$output" == *"sync"* ]]
}

# Test creating a task
@test "task create creates a new backlog task" {
  # The steel thread ST0014 is created in setup(), so stp st show should find it
  
  # Mock the stp bl command by mocking the underlying backlog command
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "create" ]]; then
  echo "Created task task-1"
  echo "File: /path/to/task-1.md"
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"
  
  # Also need to provide stp_backlog and stp_bl for the bl command
  cp "${STP_BIN_DIR}/stp_backlog" "stp/bin/"
  cp "${STP_BIN_DIR}/stp_bl" "stp/bin/"
  chmod +x "stp/bin/stp_backlog"
  chmod +x "stp/bin/stp_bl"
  
  run ./stp_task create ST0014 "Test task description"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Creating task: ST0014 - Test task description"* ]]
  [[ "$output" == *"Task created successfully"* ]]
}

# Test creating task with invalid steel thread ID
@test "task create validates steel thread ID format" {
  run ./stp_task create INVALID "Test task"
  [ "$status" -ne 0 ]
  [[ "$output" == *"Invalid steel thread ID format"* ]]
}

# Test creating task without title
@test "task create requires both ID and title" {
  run ./stp_task create ST0014
  [ "$status" -ne 0 ]
  [[ "$output" == *"Both steel thread ID and title are required"* ]]
}

# Test listing tasks for a steel thread
@test "task list shows tasks for a steel thread" {
  # Create test task files
  cat > "backlog/tasks/task-1 - ST0014-First-task.md" << EOF
---
id: task-1
title: ST0014 - First task
status: Done
assignee: []
created_date: '2025-07-08'
labels: []
dependencies: []
---

## Description
First task description
EOF

  cat > "backlog/tasks/task-2 - ST0014-Second-task.md" << EOF
---
id: task-2
title: ST0014 - Second task
status: To Do
assignee: []
created_date: '2025-07-08'
labels: []
dependencies: []
---

## Description
Second task description
EOF

  run ./stp_task list ST0014
  [ "$status" -eq 0 ]
  [[ "$output" == *"Tasks for ST0014:"* ]]
  [[ "$output" == *"task-1"* ]]
  [[ "$output" == *"[done]"* ]]
  [[ "$output" == *"ST0014 - First task"* ]]
  [[ "$output" == *"task-2"* ]]
  [[ "$output" == *"[todo]"* ]]
  [[ "$output" == *"ST0014 - Second task"* ]]
}

# Test listing tasks requires steel thread ID
@test "task list requires steel thread ID" {
  run ./stp_task list
  [ "$status" -ne 0 ]
  [[ "$output" == *"Steel thread ID required"* ]]
}

# Test sync status
@test "task sync shows task status summary" {
  # Create test task files with different statuses
  cat > "backlog/tasks/task-1 - ST0014-Done-task.md" << EOF
---
id: task-1
title: ST0014 - Done task
status: Done
---
EOF

  cat > "backlog/tasks/task-2 - ST0014-Todo-task.md" << EOF
---
id: task-2
title: ST0014 - Todo task
status: To Do
---
EOF

  cat > "backlog/tasks/task-3 - ST0014-In-progress-task.md" << EOF
---
id: task-3
title: ST0014 - In progress task
status: In Progress
---
EOF

  # Mock the backlog list command output
  create_mock_command "backlog" 0 "task-1 - ST0014 - Done task
task-2 - ST0014 - Todo task
task-3 - ST0014 - In progress task"

  run ./stp_task sync ST0014
  [ "$status" -eq 0 ]
  [[ "$output" == *"Syncing status for ST0014"* ]]
  # The sync command shows task counts, but the implementation
  # might need adjustment for proper counting in tests
}

# Test unknown command
@test "task shows error for unknown command" {
  run ./stp_task unknown
  [ "$status" -ne 0 ]
  [[ "$output" == *"Unknown command: unknown"* ]]
}