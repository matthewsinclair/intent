#!/usr/bin/env bats
# Tests for the stp_st script

load '../lib/test_helper'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/st-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the steel thread script to the test directory
  cp "${STP_BIN_DIR}/stp_st" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_st"
  
  # Create minimal STP directory structure
  mkdir -p "stp/prj/st"
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test if st requires a command
@test "st requires a command" {
  run ./stp_st
  [ "$status" -ne 0 ]
  [[ "$output" == *"Steel thread command is required"* ]]
}

# Test creating a new steel thread
@test "st new creates a new steel thread" {
  run ./stp_st new "Test Steel Thread"
  [ "$status" -eq 0 ]
  
  # Check if steel thread file was created
  assert_file_exists "stp/prj/st/ST0001.md"
  assert_file_contains "stp/prj/st/ST0001.md" "ST0001: Test Steel Thread"
  assert_file_contains "stp/prj/st/ST0001.md" "Status": "Not Started"
  
  # Check if index was updated
  assert_file_exists "stp/prj/st/steel_threads.md"
  assert_file_contains "stp/prj/st/steel_threads.md" "ST0001"
  assert_file_contains "stp/prj/st/steel_threads.md" "Test Steel Thread"
  assert_file_contains "stp/prj/st/steel_threads.md" "Not Started"
}

# Test creating multiple steel threads and check IDs
@test "st new creates sequential steel thread IDs" {
  # Create first steel thread
  run ./stp_st new "First Steel Thread"
  [ "$status" -eq 0 ]
  assert_file_exists "stp/prj/st/ST0001.md"
  
  # Create second steel thread
  run ./stp_st new "Second Steel Thread"
  [ "$status" -eq 0 ]
  assert_file_exists "stp/prj/st/ST0002.md"
  
  # Create third steel thread
  run ./stp_st new "Third Steel Thread"
  [ "$status" -eq 0 ]
  assert_file_exists "stp/prj/st/ST0003.md"
  
  # Check if index contains all three steel threads
  assert_file_contains "stp/prj/st/steel_threads.md" "ST0001"
  assert_file_contains "stp/prj/st/steel_threads.md" "ST0002"
  assert_file_contains "stp/prj/st/steel_threads.md" "ST0003"
}

# Test marking a steel thread as done
@test "st done marks a steel thread as complete" {
  # Create a steel thread
  run ./stp_st new "Test Steel Thread"
  [ "$status" -eq 0 ]
  
  # Mark it as done
  run ./stp_st done "ST0001"
  [ "$status" -eq 0 ]
  
  # Check if status and completion date were updated
  assert_file_contains "stp/prj/st/ST0001.md" "Status": "Completed"
  assert_file_contains "stp/prj/st/ST0001.md" "Completed": "$(date '+%Y-%m-%d')"
  
  # Check if index was updated
  assert_file_contains "stp/prj/st/steel_threads.md" "Completed"
}

# Test marking a steel thread as done using just the number
@test "st done works with just the number" {
  # Create a steel thread
  run ./stp_st new "Test Steel Thread"
  [ "$status" -eq 0 ]
  
  # Mark it as done using just the number
  run ./stp_st done "1"
  [ "$status" -eq 0 ]
  
  # Check if status was updated
  assert_file_contains "stp/prj/st/ST0001.md" "Status": "Completed"
}

# Test listing steel threads
@test "st list shows all steel threads" {
  # Create three steel threads with different statuses
  run ./stp_st new "First Steel Thread"
  run ./stp_st new "Second Steel Thread"
  run ./stp_st new "Third Steel Thread"
  
  # Mark second as done
  run ./stp_st done "2"
  
  # List all steel threads
  run ./stp_st list
  [ "$status" -eq 0 ]
  
  # Check if all three are listed
  [[ "$output" == *"ST0001"* ]]
  [[ "$output" == *"First Steel Thread"* ]]
  [[ "$output" == *"ST0002"* ]]
  [[ "$output" == *"Second Steel Thread"* ]]
  [[ "$output" == *"Completed"* ]]
  [[ "$output" == *"ST0003"* ]]
  [[ "$output" == *"Third Steel Thread"* ]]
}

# Test listing steel threads with status filter
@test "st list --status filters by status" {
  # Create three steel threads
  run ./stp_st new "First Steel Thread"
  run ./stp_st new "Second Steel Thread"
  run ./stp_st new "Third Steel Thread"
  
  # Mark second as done
  run ./stp_st done "2"
  
  # List only completed steel threads
  run ./stp_st list --status "Completed"
  [ "$status" -eq 0 ]
  
  # Check if only completed is listed
  [[ "$output" == *"ST0002"* ]]
  [[ "$output" == *"Second Steel Thread"* ]]
  [[ "$output" == *"Completed"* ]]
  [[ "$output" != *"ST0001"* ]]
  [[ "$output" != *"ST0003"* ]]
}

# Test showing a steel thread
@test "st show displays the content of a steel thread" {
  # Create a steel thread
  run ./stp_st new "Test Steel Thread"
  [ "$status" -eq 0 ]
  
  # Show the steel thread
  run ./stp_st show "ST0001"
  [ "$status" -eq 0 ]
  
  # Check if content is displayed
  [[ "$output" == *"ST0001: Test Steel Thread"* ]]
  [[ "$output" == *"Status": "Not Started"* ]]
}

# Test showing a steel thread with just the number
@test "st show works with just the number" {
  # Create a steel thread
  run ./stp_st new "Test Steel Thread"
  [ "$status" -eq 0 ]
  
  # Show the steel thread using just the number
  run ./stp_st show "1"
  [ "$status" -eq 0 ]
  
  # Check if content is displayed
  [[ "$output" == *"ST0001: Test Steel Thread"* ]]
}

# Test error when showing a non-existent steel thread
@test "st show errors on non-existent steel thread" {
  run ./stp_st show "ST9999"
  [ "$status" -ne 0 ]
  [[ "$output" == *"Steel thread not found"* ]]
}

# Test creating a steel thread with a template if available
@test "st new uses template if available" {
  # Create template directory and file
  mkdir -p "stp/_templ/prj/st"
  cat > "stp/_templ/prj/st/_ST####.md" << EOF
# ST####: [Title]

- **Status**: [Not Started|In Progress|Completed|On Hold|Cancelled]
- **Created**: YYYY-MM-DD
- **Completed**: 
- **Author**: [Author Name]

## Custom Section
This is a custom template
EOF

  # Create a steel thread using the template
  run ./stp_st new "Template Test"
  [ "$status" -eq 0 ]
  
  # Check if template was used
  assert_file_exists "stp/prj/st/ST0001.md"
  assert_file_contains "stp/prj/st/ST0001.md" "ST0001: Template Test"
  assert_file_contains "stp/prj/st/ST0001.md" "## Custom Section"
  assert_file_contains "stp/prj/st/ST0001.md" "This is a custom template"
}