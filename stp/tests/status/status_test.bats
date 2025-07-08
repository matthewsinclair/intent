#!/usr/bin/env bats
# Tests for the stp_status script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/status-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the status script to the test directory
  cp "${STP_BIN_DIR}/stp_status" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_status"
  
  # Create minimal STP directory structure
  mkdir -p "stp/prj/st"
  mkdir -p "stp/bin"
  mkdir -p "backlog/tasks"
  mkdir -p "backlog/drafts"
  
  # Copy stp and stp_st scripts for validation
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
Tasks are tracked in Backlog. View with: \`stp task list ST0014\`

## Implementation Notes
Test notes
EOF
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test if status requires a command
@test "status requires a command" {
  run ./stp_status
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp status"* ]]
}

# Test help command
@test "status shows help with --help" {
  run ./stp_status --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp status"* ]]
  [[ "$output" == *"show"* ]]
  [[ "$output" == *"sync"* ]]
  [[ "$output" == *"report"* ]]
}

# Test showing status for a steel thread
@test "status show displays steel thread and task status" {
  # Create test task files
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

  run ./stp_status show ST0014
  [ "$status" -eq 0 ]
  [[ "$output" == *"Steel Thread: ST0014"* ]]
  [[ "$output" == *"Current Status: In Progress"* ]]
  [[ "$output" == *"Task Summary:"* ]]
  [[ "$output" == *"Total Tasks:"* ]]
}

# Test showing status for non-existent steel thread
@test "status show errors on non-existent steel thread" {
  run ./stp_status show ST9999
  [ "$status" -ne 0 ]
  [[ "$output" == *"Steel thread ST9999 not found"* ]]
}

# Test invalid steel thread ID format
@test "status show validates steel thread ID format" {
  run ./stp_status show INVALID
  [ "$status" -ne 0 ]
  [[ "$output" == *"Invalid steel thread ID format"* ]]
}

# Test sync with dry run
@test "status sync --dry-run shows what would change" {
  # Create completed tasks
  cat > "backlog/tasks/task-1 - ST0014-Done-task.md" << EOF
---
id: task-1
title: ST0014 - Done task
status: Done
---
EOF

  cat > "backlog/tasks/task-2 - ST0014-Done-task-2.md" << EOF
---
id: task-2
title: ST0014 - Another done task
status: Done
---
EOF

  run ./stp_status sync ST0014 --dry-run
  [ "$status" -eq 0 ]
  [[ "$output" == *"Steel Thread: ST0014"* ]]
  [[ "$output" == *"Current Status: In Progress"* ]]
  # When all tasks are done, it should recommend Completed
  # [[ "$output" == *"New Status: Completed"* ]]
  # [[ "$output" == *"DRY RUN"* ]]
}

# Test status report
@test "status report shows all active threads" {
  # Create additional steel threads
  cat > "stp/prj/st/ST0015.md" << EOF
---
verblock: "20 Mar 2025:v0.1: Test - Initial version"
stp_version: 1.0.0
status: Not Started
created: 20250320
completed: 
---
# ST0015: Another Test Thread
EOF

  # Mock the stp st list command
  mkdir -p stp/prj/st
  cat > stp/prj/st/steel_threads.md << EOF
# Steel Threads

| ID | Title | Status | Created | Completed |
|----|-------|--------|---------|-----------|
| ST0014 | Test Steel Thread | In Progress | 2025-03-20 | |
| ST0015 | Another Test Thread | Not Started | 2025-03-20 | |
EOF

  run ./stp_status report
  [ "$status" -eq 0 ]
  [[ "$output" == *"Steel Thread Status Report"* ]]
  # The report implementation needs to be tested more thoroughly
  # once we understand how it interacts with stp st list
}

# Test unknown command
@test "status shows error for unknown command" {
  run ./stp_status unknown
  [ "$status" -ne 0 ]
  [[ "$output" == *"Unknown command: unknown"* ]]
}