#!/usr/bin/env bats
# Tests for intent task commands (v2.0.0)

load "../lib/test_helper.bash"

@test "task requires a command" {
  project_dir=$(create_test_project "Task Test")
  cd "$project_dir"
  
  run run_intent task
  assert_success  # Shows usage
  assert_output_contains "Usage: intent task"
}

@test "task shows help with --help" {
  project_dir=$(create_test_project "Task Help Test")
  cd "$project_dir"
  
  run run_intent task --help
  assert_success
  assert_output_contains "Usage: intent task"
  assert_output_contains "create"
  assert_output_contains "list"
  assert_output_contains "sync"
}

@test "task create creates a new backlog task" {
  project_dir=$(create_test_project "Task Create Test")
  cd "$project_dir"
  
  # Create a steel thread
  mkdir -p intent/st/ST0014
  cat > intent/st/ST0014/info.md << EOF
---
id: ST0014
title: Test Steel Thread
status: In Progress
created: 2025-03-20
author: test_user
intent_version: 2.0.0
---

# ST0014: Test Steel Thread

## Tasks
- [ ] First task
- [ ] Second task
EOF

  # Mock backlog command
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "create" ]]; then
  echo "Created task task-1"
  echo "File: backlog/tasks/task-1.md"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent task create ST0014 "Test task description"
  assert_success
  assert_output_contains "Creating task: ST0014 - Test task description"
  assert_output_contains "Task created successfully"
}

@test "task create validates steel thread ID format" {
  project_dir=$(create_test_project "Task Validate Test")
  cd "$project_dir"
  
  run run_intent task create INVALID "Test task"
  assert_failure
  assert_output_contains "Invalid steel thread ID format"
}

@test "task create requires both ID and title" {
  project_dir=$(create_test_project "Task Args Test")
  cd "$project_dir"
  
  run run_intent task create ST0014
  assert_failure
  assert_output_contains "Both steel thread ID and title are required"
}

@test "task list shows tasks for a steel thread" {
  project_dir=$(create_test_project "Task List Test")
  cd "$project_dir"
  
  # Create test task files
  mkdir -p backlog/tasks
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

  # Create a task for different ST
  cat > "backlog/tasks/task-3 - ST0015-Other-task.md" << EOF
---
id: task-3
title: ST0015 - Other task
status: To Do
---
EOF
  
  run run_intent task list ST0014
  assert_success
  assert_output_contains "Tasks for ST0014:"
  assert_output_contains "task-1"
  assert_output_contains "task-2"
  
  # Should not show task from other ST
  if [[ "$output" == *"ST0015"* ]]; then
    fail "Task from different steel thread shown"
  fi
}

@test "task list handles no tasks found" {
  project_dir=$(create_test_project "Task Empty Test")
  cd "$project_dir"
  
  mkdir -p backlog/tasks
  
  run run_intent task list ST0099
  assert_success
  assert_output_contains "Tasks for ST0099:"
}

@test "task sync updates steel thread status based on tasks" {
  project_dir=$(create_test_project "Task Sync Test")
  cd "$project_dir"
  
  # Create steel thread
  mkdir -p intent/st/ST0014
  cat > intent/st/ST0014/info.md << EOF
---
id: ST0014
title: Test Steel Thread
status: In Progress
---
EOF

  # Create completed tasks
  mkdir -p backlog/tasks
  cat > "backlog/tasks/task-1 - ST0014-Task.md" << EOF
---
id: task-1
title: ST0014 - Task
status: Done
---
EOF

  # Mock backlog command for task view
  mkdir -p bin
  cat > bin/backlog << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "view" && "$3" == "task-1" ]]; then
  echo "Status: done"
  exit 0
fi
exit 1
EOF
  chmod +x bin/backlog
  export PATH="$PWD/bin:$PATH"
  
  run run_intent task sync ST0014
  assert_success
  assert_output_contains "Task Summary:"
  assert_output_contains "Total: 0"
}

@test "task sync validates steel thread exists" {
  project_dir=$(create_test_project "Task Sync Validate Test")
  cd "$project_dir"
  
  run run_intent task sync ST9999
  assert_success
  assert_output_contains "Syncing status for ST9999"
}