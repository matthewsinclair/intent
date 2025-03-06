#!/usr/bin/env bats
# Tests for the main stp script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/main-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the main script to the test directory
  cp "${STP_BIN_DIR}/stp" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp"
  
  # Create minimal STP structure with required directories and scripts
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/bin"
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/_templ"
  
  # Create mock command scripts
  echo '#!/bin/bash' > "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_help"
  echo 'echo "Help command executed with args: $@"' >> "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_help"
  
  echo '#!/bin/bash' > "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_test"
  echo 'echo "Test command executed with args: $@"' >> "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_test"
  
  chmod +x "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_help"
  chmod +x "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_test"
  
  # Set STP_HOME environment variable
  export STP_HOME="${TEST_TEMP_DIR}/stp_home"
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    unset STP_HOME
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test if stp without arguments shows help
@test "stp without arguments shows help" {
  run ./stp
  [ "$status" -eq 0 ]
  [[ "$output" == *"Help command executed with args:"* ]]
}

# Test if stp executes commands correctly
@test "stp executes commands correctly" {
  run ./stp test arg1 arg2
  [ "$status" -eq 0 ]
  [[ "$output" == *"Test command executed with args: arg1 arg2"* ]]
}

# Test if stp help works correctly
@test "stp help works correctly" {
  run ./stp help test
  [ "$status" -eq 0 ]
  [[ "$output" == *"Help command executed with args: test"* ]]
}

# Test if stp handles unknown commands correctly
@test "stp handles unknown commands correctly" {
  run ./stp unknown_command
  [ "$status" -ne 0 ]
  [[ "$output" == *"Unknown command 'unknown_command'"* ]]
}

# Test if stp can determine STP_HOME from script location
@test "stp can determine STP_HOME from script location" {
  # Unset STP_HOME to test auto-detection
  unset STP_HOME
  
  # Create structure that simulates script in bin directory with parent containing _templ
  mkdir -p "${TEST_TEMP_DIR}/auto_detect/stp/_templ"
  mkdir -p "${TEST_TEMP_DIR}/auto_detect/stp/bin"
  cp "${STP_BIN_DIR}/stp" "${TEST_TEMP_DIR}/auto_detect/stp/bin/"
  chmod +x "${TEST_TEMP_DIR}/auto_detect/stp/bin/stp"
  
  # Create mock command scripts
  echo '#!/bin/bash' > "${TEST_TEMP_DIR}/auto_detect/stp/bin/stp_help"
  echo 'echo "Help command executed. STP_HOME=$STP_HOME"' >> "${TEST_TEMP_DIR}/auto_detect/stp/bin/stp_help"
  chmod +x "${TEST_TEMP_DIR}/auto_detect/stp/bin/stp_help"
  
  # Run from the bin directory
  cd "${TEST_TEMP_DIR}/auto_detect/stp/bin"
  run ./stp
  
  [ "$status" -eq 0 ]
  [[ "$output" == *"Help command executed. STP_HOME="*"${TEST_TEMP_DIR}/auto_detect"* ]]
}

# Test if stp makes command scripts executable if needed
@test "stp makes command scripts executable if needed" {
  # Create a non-executable script
  echo '#!/bin/bash' > "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_nonexec"
  echo 'echo "Non-executable command ran"' >> "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_nonexec"
  # Do not make it executable
  
  # First run should make it executable and run it
  run ./stp nonexec
  [ "$status" -eq 0 ]
  [[ "$output" == *"Warning: Making script executable"* ]]
  [[ "$output" == *"Non-executable command ran"* ]]
}