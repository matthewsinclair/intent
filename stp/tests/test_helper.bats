#!/usr/bin/env bats
# Test to verify test_helper.bash functions

load "lib/test_helper.bash"

@test "create a temporary directory" {
  # The setup function should have created TEST_TEMP_DIR
  [ -d "$TEST_TEMP_DIR" ]
}

@test "check STP_PROJECT_ROOT" {
  [ -d "$STP_PROJECT_ROOT" ]
}

@test "check assertions" {
  # Create a test file
  echo "test content" > test_file.txt
  
  # Test assertions
  assert_file_exists "test_file.txt"
  assert_file_contains "test_file.txt" "test content"
}

@test "check directory assertions" {
  # Create a test directory
  mkdir -p test_dir
  
  # Test assertion
  assert_directory_exists "test_dir"
}