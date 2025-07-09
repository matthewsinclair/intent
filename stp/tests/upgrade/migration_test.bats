#!/usr/bin/env bats
# Test migration from v1.2.0 to v1.2.1

setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR=$(mktemp -d)
  export TEST_TEMP_DIR
  cd "$TEST_TEMP_DIR"
  
  # Initialize STP project
  "$BATS_TEST_DIRNAME/../../bin/stp_init" "Test Project" .
  
  # Remove version file to simulate v1.2.0
  rm -f stp/.config/version
  
  # Create old-style steel thread files
  cat > stp/prj/st/ST0001.md << 'EOF'
---
verblock: "09 Jul 2025:v0.1: Test Author - Initial version"
stp_version: 1.2.0
status: In Progress
created: 20250709
completed: 
---
# ST0001: Test Thread

- **Status**: In Progress
- **Created**: 2025-07-09
- **Completed**: 
- **Author**: Test Author

## Objective

Test objective here

## Context

Test context here

## Approach

Test approach here

## Tasks

- [ ] Task 1
- [ ] Task 2

## Implementation

Test implementation notes

## Results

Test results here

## Related Steel Threads

- None
EOF
}

teardown() {
  # Clean up
  cd /
  rm -rf "$TEST_TEMP_DIR"
}

@test "migration script converts file to directory structure" {
  # Run migration
  run "$BATS_TEST_DIRNAME/../../bin/migrate_st_to_dirs"
  [ "$status" -eq 0 ]
  
  # Check backup was created
  [ -f ".stp_backup/1.2.1/ST0001.md" ]
  
  # Check directory was created
  [ -d "stp/prj/st/ST0001" ]
  
  # Check files were created
  [ -f "stp/prj/st/ST0001/info.md" ]
  [ -f "stp/prj/st/ST0001/design.md" ]
  [ -f "stp/prj/st/ST0001/impl.md" ]
  [ -f "stp/prj/st/ST0001/tasks.md" ]
  [ -f "stp/prj/st/ST0001/results.md" ]
  
  # Check original file was removed
  [ ! -f "stp/prj/st/ST0001.md" ]
  
  # Check content was preserved
  grep -q "Test objective here" "stp/prj/st/ST0001/info.md"
  grep -q "Test approach here" "stp/prj/st/ST0001/design.md"
  grep -q "Test implementation notes" "stp/prj/st/ST0001/impl.md"
  grep -q "Task 1" "stp/prj/st/ST0001/tasks.md"
  grep -q "Test results here" "stp/prj/st/ST0001/results.md"
}

@test "migration updates version to 1.2.1" {
  # Run migration
  "$BATS_TEST_DIRNAME/../../bin/migrate_st_to_dirs"
  
  # Check version was updated
  grep -q "stp_version: 1.2.1" "stp/prj/st/ST0001/info.md"
}