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
  # New threads start in NOT-STARTED subdirectory
  assert_directory_exists "intent/st/NOT-STARTED/ST0001"
  assert_file_exists "intent/st/NOT-STARTED/ST0001/info.md"
  assert_file_contains "intent/st/NOT-STARTED/ST0001/info.md" "Test Steel Thread"
}

@test "st new creates sequential steel thread IDs" {
  project_dir=$(create_test_project "ST Sequential Test")
  cd "$project_dir"
  
  export EDITOR=echo
  
  # Create first steel thread
  run run_intent st new "First Steel Thread"
  assert_success
  assert_directory_exists "intent/st/NOT-STARTED/ST0001"
  
  # Create second steel thread
  run run_intent st new "Second Steel Thread"
  assert_success
  assert_directory_exists "intent/st/NOT-STARTED/ST0002"
  
  # Create third steel thread
  run run_intent st new "Third Steel Thread"
  assert_success
  assert_directory_exists "intent/st/NOT-STARTED/ST0003"
}



@test "st list shows only in-progress threads by default" {
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

  mkdir -p intent/st/NOT-STARTED/ST0004
  cat > intent/st/NOT-STARTED/ST0004/info.md << EOF
---
id: ST0004
title: Fourth Steel Thread
status: Not Started
created: 2025-01-05
author: test_user
intent_version: 2.0.0
---
EOF
  
  run run_intent st list
  assert_success
  
  # Check only in-progress threads are listed
  assert_output_contains "ST0001"
  assert_output_contains "ST0002"
  
  # Should not show completed or not started
  if [[ "$output" == *"ST0003"* ]]; then
    fail "Completed thread shown in default view"
  fi
  if [[ "$output" == *"ST0004"* ]]; then
    fail "Not Started thread shown in default view"
  fi
}

@test "st list --status all shows all steel threads" {
  project_dir=$(create_test_project "ST List All Test")
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
  
  run run_intent st list --status all
  assert_success
  
  # Check all threads are listed
  assert_output_contains "ST0001"
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

@test "st repair shows dry-run output" {
  project_dir=$(create_test_project "ST Repair Test")
  cd "$project_dir"
  
  # Create a steel thread with malformed frontmatter
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
verblock: "06 Mar 2025:v0.1: Test User - Initial version"\nstp_version: 1.2.1\nstatus: Not Started\ncreated: 20250306\ncompleted: \n
---
# ST0001: Test Thread

- **Status**: Completed
- **Created**: 2025-03-06
- **Completed**: 2025-03-07
EOF
  
  run run_intent st repair ST0001
  assert_success
  assert_output_contains "Processing: ST0001"
  assert_output_contains "Found malformed frontmatter"
  assert_output_contains "Would fix malformed frontmatter"
  # Note: When frontmatter is malformed, it doesn't have separate status field
  # so conflicting status isn't detected until after fixing frontmatter
  assert_output_contains "Dry run complete. Use --write to apply changes."
}

@test "st repair --write fixes malformed frontmatter" {
  project_dir=$(create_test_project "ST Repair Write Test")
  cd "$project_dir"
  
  # Create a steel thread with malformed frontmatter
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
verblock: "06 Mar 2025:v0.1: Test User - Initial version"\nstp_version: 1.2.1\nstatus: Not Started\ncreated: 20250306\ncompleted: \n
---
# ST0001: Test Thread

- **Status**: Completed
- **Created**: 2025-03-06
- **Completed**: 2025-03-07
EOF
  
  # Create the index file first since organize expects it
  cat > intent/st/steel_threads.md << EOF
# Steel Threads

## Active Threads

## Completed Threads
EOF
  
  run run_intent st repair ST0001 --write
  assert_success
  assert_output_contains "Fixed malformed frontmatter"
  # The stp_version -> intent_version update happens as part of frontmatter fix
  assert_output_contains "Updated frontmatter status to: Completed"
  
  # Verify the file was moved to COMPLETED directory
  assert_file_contains "intent/st/COMPLETED/ST0001/info.md" "intent_version: 2.0.0"
  assert_file_contains "intent/st/COMPLETED/ST0001/info.md" "status: Completed"
  
  # Should not contain stp_version anymore
  run grep "stp_version" intent/st/COMPLETED/ST0001/info.md
  assert_failure
}

@test "st repair all threads without specific ID" {
  project_dir=$(create_test_project "ST Repair All Test")
  cd "$project_dir"
  
  # Create multiple threads with issues
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
stp_version: 1.2.1
status: In Progress
---
# ST0001: First Thread
EOF

  mkdir -p intent/st/ST0002
  cat > intent/st/ST0002/info.md << 'EOF'
---
intent_version: 2.0.0
status: WIP
---
# ST0002: Second Thread

- **Status**: In Progress
EOF
  
  run run_intent st repair
  assert_success
  assert_output_contains "Processing: ST0001"
  assert_output_contains "Found legacy stp_version field"
  assert_output_contains "Processing: ST0002"
  assert_output_contains "Found conflicting status:"
  assert_output_contains "Dry run complete"
}

@test "st repair handles missing status field" {
  project_dir=$(create_test_project "ST Repair Missing Status Test")
  cd "$project_dir"
  
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
intent_version: 2.0.0
created: 20250306
---
# ST0001: No Status Thread

- **Status**: In Progress
EOF
  
  run run_intent st repair ST0001
  assert_success
  assert_output_contains "Missing status field in frontmatter"
  assert_output_contains "Would add status field"
}

@test "st repair validates date formats" {
  project_dir=$(create_test_project "ST Repair Date Test")
  cd "$project_dir"
  
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << 'EOF'
---
intent_version: 2.0.0
status: In Progress
created: 2025-03-06
---
# ST0001: Bad Date Format
EOF
  
  run run_intent st repair ST0001
  assert_success
  assert_output_contains "Invalid created date format: 2025-03-06"
  assert_output_contains "Would fix created date format"
}

@test "st repair handles non-existent steel thread" {
  project_dir=$(create_test_project "ST Repair Not Found Test")
  cd "$project_dir"
  
  run run_intent st repair ST9999
  assert_failure
  assert_output_contains "Steel thread not found: ST9999"
}

@test "st start marks a not-started thread as in progress" {
  project_dir=$(create_test_project "ST Start Test")
  cd "$project_dir"
  
  # Create a not-started thread
  mkdir -p intent/st/NOT-STARTED/ST0001
  cat > intent/st/NOT-STARTED/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: Not Started
created: 20250117
---
# ST0001: Test Thread

- **Status**: Not Started
- **Created**: $(date '+%Y-%m-%d')
- **Completed**: 
- **Author**: test_user
EOF
  
  run run_intent st start ST0001
  assert_success
  assert_output_contains "Marked steel thread as in progress: ST0001: Test Thread"
  
  # Check thread was moved to main directory
  assert_directory_exists "intent/st/ST0001"
  assert_file_exists "intent/st/ST0001/info.md"
  [ ! -d "intent/st/NOT-STARTED/ST0001" ] || fail "ST0001 still in NOT-STARTED directory"
  
  # Check status was updated
  assert_file_contains "intent/st/ST0001/info.md" "status: In Progress"
  assert_file_contains "intent/st/ST0001/info.md" "**Status**: In Progress"
}

@test "st start works with just the number" {
  project_dir=$(create_test_project "ST Start Number Test")
  cd "$project_dir"
  
  mkdir -p intent/st/NOT-STARTED/ST0042
  cat > intent/st/NOT-STARTED/ST0042/info.md << EOF
---
intent_version: 2.0.0
status: Not Started
---
# ST0042: Number Test Thread
- **Status**: Not Started
EOF
  
  run run_intent st start 42
  assert_success
  assert_output_contains "ST0042"
  assert_directory_exists "intent/st/ST0042"
}

@test "st start works with various ID formats" {
  project_dir=$(create_test_project "ST Start ID Format Test")
  cd "$project_dir"
  
  # Test with leading zeros
  mkdir -p intent/st/NOT-STARTED/ST0003
  cat > intent/st/NOT-STARTED/ST0003/info.md << EOF
---
intent_version: 2.0.0
status: Not Started
---
# ST0003: Test Thread
- **Status**: Not Started
EOF
  
  run run_intent st start 0003
  assert_success
  assert_output_contains "ST0003"
  assert_directory_exists "intent/st/ST0003"
}

@test "st start does nothing if thread is already in progress" {
  project_dir=$(create_test_project "ST Start Already Progress Test")
  cd "$project_dir"
  
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: In Progress
---
# ST0001: Already Active Thread
- **Status**: In Progress
EOF
  
  run run_intent st start ST0001
  assert_success
  assert_output_contains "Steel thread is already in progress: ST0001: Already Active Thread"
  
  # Thread should remain in main directory
  assert_directory_exists "intent/st/ST0001"
}

@test "st start moves completed thread to in progress" {
  project_dir=$(create_test_project "ST Start Completed Test")
  cd "$project_dir"
  
  # Create a completed thread
  mkdir -p intent/st/COMPLETED/ST0001
  cat > intent/st/COMPLETED/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: Completed
created: 20250115
completed: 20250116
---
# ST0001: Completed Thread

- **Status**: Completed
- **Created**: 2025-01-15
- **Completed**: 2025-01-16
EOF
  
  run run_intent st start ST0001
  assert_success
  assert_output_contains "Marked steel thread as in progress: ST0001: Completed Thread"
  
  # Check thread was moved to main directory
  assert_directory_exists "intent/st/ST0001"
  [ ! -d "intent/st/COMPLETED/ST0001" ] || fail "ST0001 still in COMPLETED directory"
  
  # Check status was updated
  assert_file_contains "intent/st/ST0001/info.md" "status: In Progress"
  assert_file_contains "intent/st/ST0001/info.md" "**Status**: In Progress"
}

@test "st start updates steel_threads.md index" {
  project_dir=$(create_test_project "ST Start Index Test")
  cd "$project_dir"
  
  # Use current date consistently
  CURRENT_DATE=$(date '+%Y-%m-%d')
  
  # Create index file
  cat > intent/st/steel_threads.md << EOF
# Steel Threads

This document serves as an index of all steel threads in the project.

## Index

| ID                       | Title                  | Status       | Created    | Completed  |
| ----------------------- | -------------------- | ------------ | ---------- | ---------- |
| ST0001 | Test Thread | Not Started | $CURRENT_DATE |  |
EOF
  
  mkdir -p intent/st/NOT-STARTED/ST0001
  cat > intent/st/NOT-STARTED/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: Not Started
created: $(date '+%Y%m%d')
---
# ST0001: Test Thread
- **Status**: Not Started
- **Created**: $CURRENT_DATE
EOF
  
  run run_intent st start ST0001
  assert_success
  
  # Check index was updated
  assert_file_contains "intent/st/steel_threads.md" "| ST0001 | Test Thread | In Progress | $CURRENT_DATE |  |"
}

@test "st start errors on non-existent steel thread" {
  project_dir=$(create_test_project "ST Start Error Test")
  cd "$project_dir"
  
  run run_intent st start ST9999
  assert_failure
  assert_output_contains "Steel thread not found: ST9999"
}

@test "st start requires a steel thread ID" {
  project_dir=$(create_test_project "ST Start No ID Test")
  cd "$project_dir"
  
  run run_intent st start
  assert_failure
  assert_output_contains "Steel thread ID is required"
}

@test "st start handles thread in main directory" {
  project_dir=$(create_test_project "ST Start Main Dir Test")
  cd "$project_dir"
  
  # Create thread already in main directory but not started
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: Not Started
---
# ST0001: Main Dir Thread
- **Status**: Not Started
EOF
  
  run run_intent st start ST0001
  assert_success
  assert_output_contains "Marked steel thread as in progress: ST0001: Main Dir Thread"
  
  # Thread should remain in main directory
  assert_directory_exists "intent/st/ST0001"
  
  # Check status was updated
  assert_file_contains "intent/st/ST0001/info.md" "status: In Progress"
  assert_file_contains "intent/st/ST0001/info.md" "**Status**: In Progress"
}

@test "st list with comma-separated statuses" {
  project_dir=$(create_test_project "ST List Comma Test")
  cd "$project_dir"
  
  # Create threads with different statuses
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: WIP
---
# ST0001: WIP Thread
EOF

  mkdir -p intent/st/NOT-STARTED/ST0002
  cat > intent/st/NOT-STARTED/ST0002/info.md << EOF
---
intent_version: 2.0.0
status: Not Started
---
# ST0002: Not Started Thread
EOF

  mkdir -p intent/st/COMPLETED/ST0003
  cat > intent/st/COMPLETED/ST0003/info.md << EOF
---
intent_version: 2.0.0
status: Completed
---
# ST0003: Completed Thread
EOF
  
  # Test comma-separated filtering
  run run_intent st list --status "wip,completed"
  assert_success
  assert_output_contains "ST0001"
  assert_output_contains "ST0003"
  
  # Should not contain not started
  if [[ "$output" == *"ST0002"* ]]; then
    fail "Not Started thread shown when not requested"
  fi
}

@test "st list status ordering" {
  project_dir=$(create_test_project "ST List Order Test")
  cd "$project_dir"
  
  # Create threads with different statuses
  mkdir -p intent/st/COMPLETED/ST0001
  cat > intent/st/COMPLETED/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: Completed
created: 20250101
---
# ST0001: Completed Thread
EOF

  mkdir -p intent/st/ST0002
  cat > intent/st/ST0002/info.md << EOF
---
intent_version: 2.0.0
status: WIP
created: 20250102
---
# ST0002: WIP Thread
EOF
  
  # Test that ordering is preserved
  run run_intent st list --status "completed,wip"
  assert_success
  
  # Extract the IDs in order from output
  output_ids=$(echo "$output" | grep -E "^ST[0-9]+" | awk '{print $1}')
  first_id=$(echo "$output_ids" | head -1)
  second_id=$(echo "$output_ids" | tail -1)
  
  # Completed should come first as requested
  [[ "$first_id" == "ST0001" ]] || fail "Completed thread should be listed first"
  [[ "$second_id" == "ST0002" ]] || fail "WIP thread should be listed second"
}

@test "st list with TBC status" {
  project_dir=$(create_test_project "ST List TBC Test")
  cd "$project_dir"
  
  # Create thread with TBC status
  mkdir -p intent/st/NOT-STARTED/ST0001
  cat > intent/st/NOT-STARTED/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: Not Started
---
# ST0001: TBC Thread
EOF
  
  # Test filtering with TBC
  run run_intent st list --status "tbc"
  assert_success
  assert_output_contains "ST0001"
}

@test "st list with case-insensitive status" {
  project_dir=$(create_test_project "ST List Case Test")
  cd "$project_dir"
  
  # Create WIP thread
  mkdir -p intent/st/ST0001
  cat > intent/st/ST0001/info.md << EOF
---
intent_version: 2.0.0
status: WIP
---
# ST0001: WIP Thread
EOF
  
  # Test case-insensitive filtering
  run run_intent st list --status "WIP"
  assert_success
  assert_output_contains "ST0001"
  
  run run_intent st list --status "wip"
  assert_success
  assert_output_contains "ST0001"
  
  run run_intent st list --status "Wip"
  assert_success
  assert_output_contains "ST0001"
}