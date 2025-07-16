#!/usr/bin/env bats
# Tests for intent st commands (v2.0.0)

load "../lib/test_helper.bash"

@test "st requires a command" {
  project_dir=$(create_test_project "ST Test")
  cd "$project_dir"
  
  run run_intent st
  assert_failure
  assert_output_contains "Steel thread command is required"
}

@test "st new creates a new steel thread" {
  project_dir=$(create_test_project "ST New Test")
  cd "$project_dir"
  
  # Set EDITOR to avoid interactive prompt
  export EDITOR=echo
  
  run run_intent st new "Test Steel Thread"
  assert_success
  
  # Check if steel thread directory was created
  st_dirs=(intent/st/ST*)
  assert_directory_exists "${st_dirs[0]}"
  assert_file_exists "${st_dirs[0]}/info.md"
  assert_file_contains "${st_dirs[0]}/info.md" "Test Steel Thread"
}

@test "st new creates sequential steel thread IDs" {
  project_dir=$(create_test_project "ST Sequential Test")
  cd "$project_dir"
  
  export EDITOR=echo
  
  # Create first steel thread
  run run_intent st new "First Steel Thread"
  assert_success
  assert_directory_exists "intent/st/ST0001"
  
  # Create second steel thread
  run run_intent st new "Second Steel Thread"
  assert_success
  assert_directory_exists "intent/st/ST0002"
  
  # Create third steel thread
  run run_intent st new "Third Steel Thread"
  assert_success
  assert_directory_exists "intent/st/ST0003"
}



@test "st list shows all steel threads" {
  project_dir=$(create_test_project "ST List Test")
  cd "$project_dir"
  
  # Create steel threads with different statuses
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
id: ST0001
title: First Steel Thread
status: In Progress
created: 2025-01-01
author: test_user
intent_version: 2.0.0
---
EOF

  mkdir -p intent/st/ST0002
  cat > intent/st/ST0002/info.md << EOF
---
id: ST0002
title: Second Steel Thread
status: In Progress
created: 2025-01-02
author: test_user
intent_version: 2.0.0
---
EOF

  mkdir -p intent/st/COMPLETED/ST0003
  cat > intent/st/COMPLETED/ST0003/info.md << EOF
---
id: ST0003
title: Third Steel Thread
status: Completed
created: 2025-01-03
completed: 2025-01-04
author: test_user
intent_version: 2.0.0
---
EOF
  
  run run_intent st list
  assert_success
  
  # Check all threads are listed (just check IDs since titles aren't shown in table format)
  assert_output_contains "ST0001"
  assert_output_contains "ST0002"
  assert_output_contains "ST0003"
}

@test "st list --status filters by status" {
  project_dir=$(create_test_project "ST Filter Test")
  cd "$project_dir"
  
  # Create threads with different statuses
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
id: ST0001
title: Active Thread
status: In Progress
---
EOF

  mkdir -p intent/st/COMPLETED/ST0002
  cat > intent/st/COMPLETED/ST0002/info.md << EOF
---
id: ST0002
title: Done Thread
status: Completed
---
EOF
  
  # List only In Progress
  run run_intent st list --status "In Progress"
  assert_success
  assert_output_contains "ST0001"
  
  # Should not contain completed thread
  if [[ "$output" == *"ST0002"* ]]; then
    fail "Completed thread shown when filtering for In Progress"
  fi
}

@test "st show displays the content of a steel thread" {
  project_dir=$(create_test_project "ST Show Test")
  cd "$project_dir"
  
  # Create a steel thread
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
id: ST0001
title: Test Steel Thread
status: In Progress
created: 2025-01-01
author: test_user
intent_version: 2.0.0
---

# ST0001: Test Steel Thread

## Description
This is the thread content.

## Tasks
- [ ] Task 1
- [ ] Task 2
EOF
  
  run run_intent st show ST0001
  assert_success
  assert_output_contains "ST0001: Test Steel Thread"
  assert_output_contains "This is the thread content"
  assert_output_contains "Task 1"
}

@test "st show works with just the number" {
  project_dir=$(create_test_project "ST Show Number Test")
  cd "$project_dir"
  
  mkdir -p intent/st/ST0001
  echo "# ST0001: Test" > intent/st/ST0001/info.md
  
  run run_intent st show 1
  assert_success
  assert_output_contains "ST0001"
}

@test "st show errors on non-existent steel thread" {
  project_dir=$(create_test_project "ST Error Test")
  cd "$project_dir"
  
  run run_intent st show ST9999
  assert_failure
  assert_output_contains "not found"
}

@test "st organize moves threads by status" {
  project_dir=$(create_test_project "ST Organize Test")
  cd "$project_dir"
  
  # Create completed thread in wrong location
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
id: ST0001
title: Should be in COMPLETED
status: Completed
---
EOF

  # Create active thread in completed location
  mkdir -p intent/st/COMPLETED/ST0002
  cat > intent/st/COMPLETED/ST0002/info.md << EOF
---
id: ST0002
title: Should be active
status: In Progress
---
EOF
  
  # Create the index file first since organize expects it
  cat > intent/st/steel_threads.md << EOF
# Steel Threads

## Active Threads

## Completed Threads
EOF
  
  run run_intent st organize --write
  assert_success
  
  # Check threads were moved
  assert_directory_exists "intent/st/COMPLETED/ST0001"
  assert_directory_exists "intent/st/ST0002"
  [ ! -d "intent/st/ST0001" ] || fail "ST0001 still in active directory"
  [ ! -d "intent/st/COMPLETED/ST0002" ] || fail "ST0002 still in COMPLETED"
}

@test "st creates steel_threads.md index" {
  project_dir=$(create_test_project "ST Index Test")
  cd "$project_dir"
  
  export EDITOR=echo
  
  # Create a steel thread
  run run_intent st new "Index Test Thread"
  assert_success
  
  # Check if index was created/updated
  assert_file_exists "intent/st/steel_threads.md"
  assert_file_contains "intent/st/steel_threads.md" "Steel Threads"
}