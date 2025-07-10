#!/usr/bin/env bats

# Test suite for stp st organize command

load '../lib/test_helper.bash'
load '../lib/bats-assert/load'
load '../lib/bats-support/load'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/st-organize-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the steel thread script to the test directory
  cp "${STP_BIN_DIR}/stp_st" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_st"
  
  # Create minimal STP directory structure
  mkdir -p "stp/prj/st"
  mkdir -p "stp/.config"
  echo "stp_version: 1.2.1" > "stp/.config/version"
  
  # Create empty steel_threads.md file for tests that use --write
  touch "stp/prj/st/steel_threads.md"
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

@test "st organize requires directory structure" {
  
  # Create a v1.2.0 project (file-based)
  echo "stp_version: 1.2.0" > "stp/.config/version"
  
  run ./stp_st organize
  assert_success
}

@test "st organize dry run shows what would be moved" {
  # Create directory structure
  mkdir -p "stp/prj/st/ST0001"
  cat > "stp/prj/st/ST0001/info.md" <<EOF
---
verblock: "01 Jan 2025:v0.1: Test - Initial"
stp_version: 1.2.1
status: Completed
created: 20250101
---
# ST0001: Test Thread
EOF

  run ./stp_st organize
  assert_success
  assert_output --partial "Would move ST0001 to stp/prj/st/COMPLETED"
}

@test "st organize --write moves directories to correct locations" {
  # Create directories for different statuses
  mkdir -p "stp/prj/st/ST0001"
  mkdir -p "stp/prj/st/ST0002"
  mkdir -p "stp/prj/st/ST0003"
  
  # Completed thread
  cat > "stp/prj/st/ST0001/info.md" <<EOF
---
status: Completed
---
# ST0001: Completed Thread
EOF

  # Not Started thread
  cat > "stp/prj/st/ST0002/info.md" <<EOF
---
status: Not Started
---
# ST0002: Not Started Thread
EOF

  # WIP thread (should stay in main)
  cat > "stp/prj/st/ST0003/info.md" <<EOF
---
status: wip
---
# ST0003: WIP Thread
EOF

  run ./stp_st organize --write
  assert_success
  
  # Check that directories were moved correctly
  assert [ -d "stp/prj/st/COMPLETED/ST0001" ]
  assert [ -d "stp/prj/st/NOT-STARTED/ST0002" ]
  assert [ -d "stp/prj/st/ST0003" ]
  
  # Check that WIP stayed in main
  assert [ ! -d "stp/prj/st/COMPLETED/ST0003" ]
}

@test "st organize handles already organized directories" {
  # Create already organized structure
  mkdir -p "stp/prj/st/COMPLETED/ST0001"
  mkdir -p "stp/prj/st/NOT-STARTED/ST0002"
  
  cat > "stp/prj/st/COMPLETED/ST0001/info.md" <<EOF
---
status: Completed
---
# ST0001: Already Completed
EOF

  cat > "stp/prj/st/NOT-STARTED/ST0002/info.md" <<EOF
---
status: Not Started
---
# ST0002: Already Not Started
EOF

  run ./stp_st organize
  assert_success
  assert_output --partial "Already organized: ST0001 in stp/prj/st/COMPLETED"
  assert_output --partial "Already organized: ST0002 in stp/prj/st/NOT-STARTED"
}

@test "st organize normalizes status values" {
  mkdir -p "stp/prj/st/ST0001"
  mkdir -p "stp/prj/st/ST0002"
  
  # Test various status formats
  cat > "stp/prj/st/ST0001/info.md" <<EOF
---
status: WIP
---
# ST0001: WIP Thread
EOF

  cat > "stp/prj/st/ST0002/info.md" <<EOF
---
status: in progress
---
# ST0002: In Progress Thread
EOF

  run ./stp_st organize
  assert_success
  # Both should be recognized as "In Progress" and stay in main
  assert_output --partial "Already organized: ST0001 in stp/prj/st (Status: In Progress)"
  assert_output --partial "Already organized: ST0002 in stp/prj/st (Status: In Progress)"
}

@test "st organize doesn't move misplaced directories back to main" {
  # Create a completed thread that's incorrectly in NOT-STARTED
  mkdir -p "stp/prj/st/NOT-STARTED/ST0001"
  cat > "stp/prj/st/NOT-STARTED/ST0001/info.md" <<EOF
---
status: Completed
---
# ST0001: Misplaced Completed Thread
EOF

  run ./stp_st organize
  assert_success
  assert_output --partial "Would move ST0001 to stp/prj/st/COMPLETED"
  
  # Now actually move it
  run ./stp_st organize --write
  assert_success
  
  # Verify it's in the right place
  assert [ -d "stp/prj/st/COMPLETED/ST0001" ]
  assert [ ! -d "stp/prj/st/NOT-STARTED/ST0001" ]
}