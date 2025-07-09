#!/usr/bin/env bats
# Tests for the stp_llm script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/llm-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the llm script to the test directory
  cp "${STP_BIN_DIR}/stp_llm" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_llm"
  
  # Create minimal STP_HOME structure
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/eng"
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/bin/.help"
  
  # Create a test usage-rules.md file
  cat > "${TEST_TEMP_DIR}/stp_home/stp/eng/usage-rules.md" << EOF
---
verblock: "01 Jan 2025:v0.1: Test User - Test version"
stp_version: 1.0.0
---
# Test Usage Rules

This is a test usage rules document.

## Test Section

Test content for usage rules.
EOF
  
  # Create help file
  cat > "${TEST_TEMP_DIR}/stp_home/stp/bin/.help/llm.help.md" << EOF
@short:
Test LLM commands

@description:
Test description
EOF
  
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

# Test if llm requires STP_HOME to be set
@test "llm requires STP_HOME environment variable" {
  unset STP_HOME
  run ./stp_llm usage_rules
  [ "$status" -ne 0 ]
  [[ "$output" == *"STP_HOME environment variable is not set"* ]]
}

# Test if llm displays usage when no subcommand is provided
@test "llm displays usage when no subcommand is provided" {
  run ./stp_llm
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp llm <subcommand> [options]"* ]]
  [[ "$output" == *"usage_rules"* ]]
  [[ "$output" == *"--symlink"* ]]
}

# Test if llm displays usage with help option
@test "llm displays usage with help option" {
  run ./stp_llm --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp llm <subcommand> [options]"* ]]
}

# Test if llm displays usage rules content
@test "llm displays usage rules content" {
  run ./stp_llm usage_rules
  [ "$status" -eq 0 ]
  [[ "$output" == *"Test Usage Rules"* ]]
  [[ "$output" == *"Test content for usage rules"* ]]
}

# Test if llm handles missing usage-rules.md file
@test "llm handles missing usage-rules.md file" {
  rm -f "${STP_HOME}/stp/eng/usage-rules.md"
  run ./stp_llm usage_rules
  [ "$status" -ne 0 ]
  [[ "$output" == *"Usage rules file not found"* ]]
}

# Test if llm creates symlink in current directory
@test "llm creates symlink in current directory" {
  run ./stp_llm usage_rules --symlink
  [ "$status" -eq 0 ]
  [[ "$output" == *"Created symlink: ./usage-rules.md"* ]]
  
  # Verify symlink exists
  [ -L "./usage-rules.md" ]
  
  # Verify symlink points to correct location
  local target="$(readlink ./usage-rules.md)"
  [ "$target" == "${STP_HOME}/stp/eng/usage-rules.md" ]
}

# Test if llm creates symlink in specified directory
@test "llm creates symlink in specified directory" {
  mkdir -p "${TEST_TEMP_DIR}/target"
  run ./stp_llm usage_rules --symlink "${TEST_TEMP_DIR}/target"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Created symlink: ${TEST_TEMP_DIR}/target/usage-rules.md"* ]]
  
  # Verify symlink exists
  [ -L "${TEST_TEMP_DIR}/target/usage-rules.md" ]
}

# Test if llm handles non-existent target directory
@test "llm handles non-existent target directory" {
  run ./stp_llm usage_rules --symlink "/non/existent/directory"
  [ "$status" -ne 0 ]
  [[ "$output" == *"Target directory does not exist"* ]]
}

# Test if llm handles existing symlink with 'n' response
@test "llm handles existing symlink with cancel response" {
  # Create initial symlink
  ln -s "${STP_HOME}/stp/eng/usage-rules.md" "./usage-rules.md"
  
  # Try to create again, responding 'n' to overwrite prompt
  # Use echo to provide input directly to the script
  output=$(echo "n" | ./stp_llm usage_rules --symlink 2>&1)
  status=$?
  [ "$status" -eq 0 ]
  [[ "$output" == *"already exists"* ]]
  [[ "$output" == *"Cancelled"* ]]
  
  # Verify original symlink still exists
  [ -L "./usage-rules.md" ]
}

# Test if llm handles existing symlink with 'y' response
@test "llm handles existing symlink with overwrite response" {
  # Create a dummy file first
  touch "./usage-rules.md"
  
  # Try to create symlink, responding 'y' to overwrite prompt
  # Use echo to provide input directly to the script
  output=$(echo "y" | ./stp_llm usage_rules --symlink 2>&1)
  status=$?
  [ "$status" -eq 0 ]
  [[ "$output" == *"Created symlink"* ]]
  
  # Verify it's now a symlink
  [ -L "./usage-rules.md" ]
}

# Test if llm handles unknown subcommand
@test "llm handles unknown subcommand" {
  run ./stp_llm unknown_subcommand
  [ "$status" -ne 0 ]
  [[ "$output" == *"Unknown subcommand: unknown_subcommand"* ]]
}

# Test if llm handles unknown option for usage_rules
@test "llm handles unknown option for usage_rules" {
  run ./stp_llm usage_rules --unknown-option
  [ "$status" -ne 0 ]
  [[ "$output" == *"Unknown option: --unknown-option"* ]]
}

# Test symlink creation preserves correct permissions
@test "llm symlink preserves correct permissions" {
  run ./stp_llm usage_rules --symlink
  [ "$status" -eq 0 ]
  
  # The symlink itself should exist
  [ -L "./usage-rules.md" ]
  
  # The target file should be readable
  [ -r "${STP_HOME}/stp/eng/usage-rules.md" ]
}