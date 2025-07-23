#!/usr/bin/env bats
# Tests for intent bl (backlog) commands (v2.0.0)

load "../lib/test_helper.bash"

@test "bl shows help" {
  project_dir=$(create_test_project "BL Test")
  cd "$project_dir"
  
  run run_intent bl
  assert_success
  assert_output_contains "Usage: intent backlog"
  assert_output_contains "intent bl"
}

@test "bl list adds --plain automatically" {
  project_dir=$(create_test_project "BL List Test")
  cd "$project_dir"
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
if [[ "$*" == "task list --plain" ]]; then
  echo "To Do:"
  echo "  task-001 - Test task"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl list
  assert_success
  assert_output_contains "Backlog called with: task list --plain"
  assert_output_contains "task-001 - Test task"
}

@test "bl create validates and creates task" {
  project_dir=$(create_test_project "BL Create Test")
  cd "$project_dir"
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "create" && "$3" == "ST0014 - Test task" ]]; then
  echo "Created task task-001"
  echo "File: backlog/tasks/task-001 - ST0014-Test-task.md"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl create ST0014 "Test task"
  assert_success
  assert_output_contains "Created task task-001"
}

@test "bl create rejects invalid steel thread ID" {
  project_dir=$(create_test_project "BL Invalid ID Test")
  cd "$project_dir"
  
  run run_intent bl create INVALID "Test task"
  assert_failure
  assert_output_contains "Invalid steel thread ID format"
}

@test "bl board passes through without --plain" {
  project_dir=$(create_test_project "BL Board Test")
  cd "$project_dir"
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
if [[ "$*" == "board" ]]; then
  echo "Kanban board displayed"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl board
  assert_success
  assert_output_contains "Backlog called with: board"
  assert_output_contains "Kanban board displayed"
}

@test "bl init configures backlog for Intent" {
  project_dir=$(create_test_project "BL Init Test")
  cd "$project_dir"
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
if [[ "$1" == "init" ]]; then
  mkdir -p backlog
  echo "project_name: test" > backlog/config.yml
  echo "Backlog initialized"
  exit 0
elif [[ "$1" == "config" && "$2" == "set" ]]; then
  echo "Config set: $3 = $4"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl init
  assert_success
  assert_output_contains "Backlog initialized"
  assert_output_contains "Configuring backlog for Intent integration"
  assert_output_contains "Backlog configured for local Intent use"
}

@test "bl task edit passes through correctly" {
  project_dir=$(create_test_project "BL Task Edit Test")
  cd "$project_dir"
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
if [[ "$*" == "task edit task-005 --status Done" ]]; then
  echo "Task updated"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl task edit task-005 --status Done
  assert_success
  assert_output_contains "Backlog called with: task edit task-005 --status Done"
  assert_output_contains "Task updated"
}

@test "bl passes through unknown commands" {
  project_dir=$(create_test_project "BL Unknown Test")
  cd "$project_dir"
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
exit 0
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl decision create "Architecture choice"
  assert_success
  assert_output_contains "Backlog called with: decision create Architecture choice"
}

@test "bl requires backlog to be installed" {
  project_dir=$(create_test_project "BL No Backlog Test")
  cd "$project_dir"
  
  # Save original PATH
  ORIG_PATH="$PATH"
  
  # Create a temporary directory for our fake commands
  FAKE_BIN="$(mktemp -d)"
  
  # Create fake jq that works (so we get past the jq check)
  cat > "$FAKE_BIN/jq" << 'EOF'
#!/bin/bash
# Fake jq that just passes through for our test
exit 0
EOF
  chmod +x "$FAKE_BIN/jq"
  
  # Set PATH to include fake bin and essential system directories
  # This ensures commands like rm, cat, etc. still work
  export PATH="$FAKE_BIN:/usr/bin:/bin"
  
  # Now backlog won't be found, but jq and system commands will work
  run run_intent bl list
  assert_failure
  assert_output_contains "Backlog.md is not installed"
  
  # Restore PATH and cleanup
  export PATH="$ORIG_PATH"
  rm -rf "$FAKE_BIN"
}

@test "bl list respects backlog_list_status from config" {
  project_dir=$(create_test_project "BL Status Filter Test")
  cd "$project_dir"
  
  # Set backlog_list_status in config
  cat > .intent/config.json << 'EOF'
{
  "version": "2.0.0",
  "project_name": "Test Project",
  "author": "Test",
  "created": "2025-07-17",
  "st_prefix": "ST",
  "backlog_list_status": "todo"
}
EOF
  
  # Mock backlog command that echoes arguments
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
# Check if -s todo was passed
if [[ "$*" == *"-s todo"* ]]; then
  echo "Filtering by status: todo"
  echo "todo:"
  echo "  task-001 - ST0001 - Todo task"
else
  echo "No status filter applied"
  echo "todo:"
  echo "  task-001 - ST0001 - Todo task"
  echo "done:"
  echo "  task-002 - ST0002 - Done task"
fi
exit 0
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl list
  assert_success
  assert_output_contains "Backlog called with: task list --plain -s todo"
  assert_output_contains "Filtering by status: todo"
  assert_output_contains "task-001 - ST0001 - Todo task"
  ! assert_output_contains "task-002 - ST0002 - Done task"
}

@test "bl list --all ignores backlog_list_status" {
  project_dir=$(create_test_project "BL All Test")
  cd "$project_dir"
  
  # Set backlog_list_status in config
  cat > .intent/config.json << 'EOF'
{
  "version": "2.0.0",
  "project_name": "Test Project",
  "author": "Test",
  "created": "2025-07-17",
  "st_prefix": "ST",
  "backlog_list_status": "todo"
}
EOF
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
# Check if -s flag was NOT passed (meaning show all)
if [[ "$*" != *"-s"* ]]; then
  echo "Showing all tasks"
  echo "todo:"
  echo "  task-001 - ST0001 - Todo task"
  echo "done:"
  echo "  task-002 - ST0002 - Done task"
fi
exit 0
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl list --all
  assert_success
  assert_output_contains "Showing all tasks"
  assert_output_contains "task-001 - ST0001 - Todo task"
  assert_output_contains "task-002 - ST0002 - Done task"
}

@test "bl list validates backlog_list_status" {
  project_dir=$(create_test_project "BL Invalid Status Test")
  cd "$project_dir"
  
  # Set invalid backlog_list_status in config
  cat > .intent/config.json << 'EOF'
{
  "version": "2.0.0",
  "project_name": "Test Project",
  "author": "Test",
  "created": "2025-07-17",
  "st_prefix": "ST",
  "backlog_list_status": "invalid-status"
}
EOF
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
exit 0
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl list
  assert_success
  assert_output_contains "Warning: Invalid backlog_list_status 'invalid-status'"
  assert_output_contains "Valid statuses are: todo wip done cancelled archived"
}

@test "bl task pad requires --size argument" {
  project_dir=$(create_test_project "BL Pad Size Test")
  cd "$project_dir"
  
  run run_intent bl task pad task-9
  assert_failure
  assert_output_contains "No --size specified and backlog not configured"
}

@test "bl task pad validates size is numeric" {
  project_dir=$(create_test_project "BL Pad Size Numeric Test")
  cd "$project_dir"
  
  run run_intent bl task pad task-9 --size abc
  assert_failure
  assert_output_contains "Invalid --size value. Must be a positive number"
}

@test "bl task pad requires task ID or --all" {
  project_dir=$(create_test_project "BL Pad Args Test")
  cd "$project_dir"
  
  run run_intent bl task pad --size 3
  assert_failure
  assert_output_contains "Must specify either a task ID or --all"
}

@test "bl task pad rejects both task ID and --all" {
  project_dir=$(create_test_project "BL Pad Both Args Test")
  cd "$project_dir"
  
  run run_intent bl task pad task-9 --all --size 3
  assert_failure
  assert_output_contains "Cannot specify both a task ID and --all"
}

@test "bl task pad pads single task correctly" {
  project_dir=$(create_test_project "BL Pad Single Test")
  cd "$project_dir"
  
  # Create backlog directory structure
  mkdir -p backlog/tasks
  
  # Create a test task file
  cat > "backlog/tasks/task-9 - ST0001-Test-task.md" << 'EOF'
---
id: task-9
title: ST0001 - Test task
status: todo
assignee: []
created_date: '2025-07-23'
updated_date: '2025-07-23'
labels: []
dependencies: []
---

## Description
Test task
EOF
  
  run run_intent bl task pad task-9 --size 3
  assert_success
  assert_output_contains "Padding tasks to 3 digits..."
  assert_output_contains "Padding: task-9 - ST0001-Test-task.md -> task-009 - ST0001-Test-task.md"
  assert_output_contains "Successfully padded task"
  assert_output_contains "intent bl config set zeroPaddedIds 3"
  
  # Verify file was renamed
  assert_file_exists "backlog/tasks/task-009 - ST0001-Test-task.md"
  [ ! -f "backlog/tasks/task-9 - ST0001-Test-task.md" ]
  
  # Verify ID was updated in file content
  run grep "^id: task-009$" "backlog/tasks/task-009 - ST0001-Test-task.md"
  assert_success
}

@test "bl task pad handles already padded tasks" {
  project_dir=$(create_test_project "BL Pad Already Padded Test")
  cd "$project_dir"
  
  # Create backlog directory structure
  mkdir -p backlog/tasks
  
  # Create an already padded task file
  cat > "backlog/tasks/task-009 - ST0001-Test-task.md" << 'EOF'
---
id: task-009
title: ST0001 - Test task
status: todo
assignee: []
created_date: '2025-07-23'
updated_date: '2025-07-23'
labels: []
dependencies: []
---

## Description
Test task
EOF
  
  run run_intent bl task pad task-009 --size 3
  assert_success
  assert_output_contains "Task 'task-009' is already padded to 3 digits"
}

@test "bl task pad --all pads all tasks" {
  project_dir=$(create_test_project "BL Pad All Test")
  cd "$project_dir"
  
  # Create backlog directory structure
  mkdir -p backlog/tasks
  mkdir -p backlog/archive/tasks
  
  # Create test task files
  cat > "backlog/tasks/task-1 - ST0001-First.md" << 'EOF'
---
id: task-1
title: ST0001 - First
status: todo
---
EOF
  
  cat > "backlog/tasks/task-10 - ST0002-Second.md" << 'EOF'
---
id: task-10
title: ST0002 - Second
status: todo
---
EOF
  
  cat > "backlog/archive/tasks/task-5 - ST0003-Archived.md" << 'EOF'
---
id: task-5
title: ST0003 - Archived
status: archived
---
EOF
  
  run run_intent bl task pad --all --size 3
  assert_success
  assert_output_contains "Padding tasks to 3 digits..."
  assert_output_contains "Processed backlog/tasks/: 2 files updated, 0 already padded"
  assert_output_contains "Processed backlog/archive/tasks/: 1 files updated, 0 already padded"
  assert_output_contains "Total: 3 tasks updated"
  
  # Verify files were renamed
  assert_file_exists "backlog/tasks/task-001 - ST0001-First.md"
  assert_file_exists "backlog/tasks/task-010 - ST0002-Second.md"
  assert_file_exists "backlog/archive/tasks/task-005 - ST0003-Archived.md"
  
  # Verify IDs were updated
  run grep "^id: task-001$" "backlog/tasks/task-001 - ST0001-First.md"
  assert_success
  run grep "^id: task-010$" "backlog/tasks/task-010 - ST0002-Second.md"
  assert_success
  run grep "^id: task-005$" "backlog/archive/tasks/task-005 - ST0003-Archived.md"
  assert_success
}

@test "bl task pad handles non-existent task" {
  project_dir=$(create_test_project "BL Pad Not Found Test")
  cd "$project_dir"
  
  # Create empty backlog directory
  mkdir -p backlog/tasks
  
  run run_intent bl task pad task-999 --size 3
  assert_failure
  assert_output_contains "Error: Task 'task-999' not found"
}

@test "bl task pad uses configured size when no --size provided" {
  project_dir=$(create_test_project "BL Pad Config Size Test")
  cd "$project_dir"
  
  # Create backlog directory and config
  mkdir -p backlog/tasks
  cat > backlog/config.yml << 'EOF'
zeroPaddedIds: 2
EOF
  
  # Create a test task file
  cat > "backlog/tasks/task-5 - ST0001-Test.md" << 'EOF'
---
id: task-5
title: ST0001 - Test
status: todo
---
EOF
  
  # Mock backlog config get command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
if [[ "$1" == "config" && "$2" == "get" && "$3" == "zeroPaddedIds" ]]; then
  echo "2"
  exit 0
fi
echo "Unknown command"
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl task pad --all
  assert_success
  assert_output_contains "Using configured zero padding size: 2"
  assert_output_contains "Padding tasks to 2 digits..."
  
  # Verify file was renamed
  assert_file_exists "backlog/tasks/task-05 - ST0001-Test.md"
}