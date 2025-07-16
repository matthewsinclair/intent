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
  echo "  task-1 - Test task"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl list
  assert_success
  assert_output_contains "Backlog called with: task list --plain"
  assert_output_contains "task-1 - Test task"
}

@test "bl create validates and creates task" {
  project_dir=$(create_test_project "BL Create Test")
  cd "$project_dir"
  
  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "create" && "$3" == "ST0014 - Test task" ]]; then
  echo "Created task task-1"
  echo "File: backlog/tasks/task-1 - ST0014-Test-task.md"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl create ST0014 "Test task"
  assert_success
  assert_output_contains "Created task task-1"
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
if [[ "$*" == "task edit task-5 --status Done" ]]; then
  echo "Task updated"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl task edit task-5 --status Done
  assert_success
  assert_output_contains "Backlog called with: task edit task-5 --status Done"
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
  
  # Ensure backlog is not in PATH
  export PATH="/usr/bin:/bin"
  
  run run_intent bl list
  assert_failure
  assert_output_contains "Backlog.md is not installed"
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
  echo "  task-1 - ST0001 - Todo task"
else
  echo "No status filter applied"
  echo "todo:"
  echo "  task-1 - ST0001 - Todo task"
  echo "done:"
  echo "  task-2 - ST0002 - Done task"
fi
exit 0
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl list
  assert_success
  assert_output_contains "Backlog called with: task list --plain -s todo"
  assert_output_contains "Filtering by status: todo"
  assert_output_contains "task-1 - ST0001 - Todo task"
  ! assert_output_contains "task-2 - ST0002 - Done task"
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
  echo "  task-1 - ST0001 - Todo task"
  echo "done:"
  echo "  task-2 - ST0002 - Done task"
fi
exit 0
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent bl list --all
  assert_success
  assert_output_contains "Showing all tasks"
  assert_output_contains "task-1 - ST0001 - Todo task"
  assert_output_contains "task-2 - ST0002 - Done task"
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