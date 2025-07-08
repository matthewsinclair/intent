#!/usr/bin/env bats
# Tests for the stp_migrate script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/migrate-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the migrate script to the test directory
  cp "${STP_BIN_DIR}/stp_migrate" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_migrate"
  
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
  
  # Create a test steel thread with embedded tasks
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
- [x] Completed task one
- [x] Completed task two
- [ ] Pending task three
- [ ] Pending task four

## Implementation Notes
Test notes
EOF

  # Create another steel thread
  cat > "stp/prj/st/ST0015.md" << EOF
---
verblock: "20 Mar 2025:v0.1: Test - Initial version"
stp_version: 1.0.0
status: Not Started
created: 20250320
completed: 
---
# ST0015: Another Thread

## Objective
Another objective

## Tasks
- [ ] Task A
- [ ] Task B

## Notes
More notes
EOF
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test if migrate requires arguments
@test "migrate requires arguments when no options" {
  run ./stp_migrate
  [ "$status" -eq 1 ]
  [[ "$output" == *"Usage: stp migrate"* ]]
}

# Test help command
@test "migrate shows help with --help" {
  run ./stp_migrate --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp migrate"* ]]
  [[ "$output" == *"--all-active"* ]]
  [[ "$output" == *"--dry-run"* ]]
}

# Test dry run migration
@test "migrate --dry-run shows what would be migrated" {
  run ./stp_migrate --dry-run ST0014
  [ "$status" -eq 0 ]
  [[ "$output" == *"DRY RUN MODE"* ]]
  [[ "$output" == *"Migrating ST0014"* ]]
  [[ "$output" == *"Found"*"4 tasks to migrate"* ]]
  [[ "$output" == *"[DRY RUN] Would create task: ST0014 - Completed task one (status: done)"* ]]
  [[ "$output" == *"[DRY RUN] Would create task: ST0014 - Completed task two (status: done)"* ]]
  [[ "$output" == *"[DRY RUN] Would create task: ST0014 - Pending task three (status: todo)"* ]]
  [[ "$output" == *"[DRY RUN] Would create task: ST0014 - Pending task four (status: todo)"* ]]
  [[ "$output" == *"[DRY RUN] Would update ST0014 to reference Backlog tasks"* ]]
}

# Test actual migration
@test "migrate creates backlog tasks from embedded tasks" {
  # Mock the backlog command to simulate task creation
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "create" ]]; then
  # Extract task number from title
  if [[ "$3" =~ "task one" ]]; then
    echo "Created task task-1"
  elif [[ "$3" =~ "task two" ]]; then
    echo "Created task task-2"
  elif [[ "$3" =~ "task three" ]]; then
    echo "Created task task-3"
  elif [[ "$3" =~ "task four" ]]; then
    echo "Created task task-4"
  fi
  exit 0
elif [[ "$1" == "task" && "$2" == "edit" ]]; then
  # Simulate marking task as done
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"

  run ./stp_migrate ST0014
  [ "$status" -eq 0 ]
  [[ "$output" == *"Migrating ST0014"* ]]
  [[ "$output" == *"Creating task: ST0014 - Completed task one"* ]]
  [[ "$output" == *"Creating task: ST0014 - Completed task two"* ]]
  [[ "$output" == *"Creating task: ST0014 - Pending task three"* ]]
  [[ "$output" == *"Creating task: ST0014 - Pending task four"* ]]
  [[ "$output" == *"Updating steel thread to reference Backlog"* ]]
  
  # Check that the steel thread was updated
  assert_file_contains "stp/prj/st/ST0014.md" "Tasks are tracked in Backlog"
  assert_file_contains "stp/prj/st/ST0014.md" "stp task list ST0014"
  
  # Ensure old tasks were removed
  run grep -E "^- \[.\]" "stp/prj/st/ST0014.md"
  [ "$status" -ne 0 ]
}

# Test migrating non-existent steel thread
@test "migrate errors on non-existent steel thread" {
  run ./stp_migrate ST9999
  [ "$status" -ne 0 ]
  [[ "$output" == *"Steel thread ST9999 not found"* ]]
}

# Test invalid steel thread ID format
@test "migrate validates steel thread ID format" {
  run ./stp_migrate STXXX
  [ "$status" -ne 0 ]
  [[ "$output" == *"Invalid steel thread ID format"* ]]
}

# Test migrate with no tasks
@test "migrate handles steel thread with no tasks" {
  # Create a steel thread without tasks
  cat > "stp/prj/st/ST0016.md" << EOF
---
verblock: "20 Mar 2025:v0.1: Test - Initial version"
stp_version: 1.0.0
status: In Progress
created: 20250320
completed: 
---
# ST0016: No Tasks Thread

## Objective
Test objective

## Tasks
No tasks defined yet.

## Notes
Test notes
EOF

  run ./stp_migrate ST0016
  [ "$status" -eq 0 ]
  [[ "$output" == *"No tasks found to migrate"* ]]
}

# Test --all-active flag
@test "migrate --all-active migrates all active threads" {
  # Mock stp st list output
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/stp" << 'EOF'
#!/bin/bash
if [[ "$1" == "st" && "$2" == "list" ]]; then
  cat << 'LIST'
ID         | Title                     | Status       | Created    | Completed 
-----------|---------------------------|--------------|------------|-----------
ST0014     | Test Steel Thread         | In Progress  | 2025-03-20 |           
ST0015     | Another Thread            | Not Started  | 2025-03-20 |           
LIST
  exit 0
fi
# Pass through to real stp for other commands
"${STP_HOME}/stp/bin/stp" "$@"
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/stp"
  
  # Mock backlog command
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "create" ]]; then
  echo "Created task task-X"
  exit 0
elif [[ "$1" == "task" && "$2" == "edit" ]]; then
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"

  run ./stp_migrate --all-active --dry-run
  [ "$status" -eq 0 ]
  [[ "$output" == *"Migrating all active steel threads"* ]]
  [[ "$output" == *"Migrating ST0014"* ]]
  [[ "$output" == *"Migrating ST0015"* ]]
}

# Test conflicting options
@test "migrate errors when both --all-active and steel thread ID specified" {
  run ./stp_migrate --all-active ST0014
  [ "$status" -ne 0 ]
  [[ "$output" == *"Cannot specify both --all-active and a specific steel thread"* ]]
}