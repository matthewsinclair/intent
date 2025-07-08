#!/usr/bin/env bats
# Tests for the stp_backlog wrapper

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/backlog-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the backlog scripts to the test directory
  cp "${STP_BIN_DIR}/stp_backlog" "${TEST_TEMP_DIR}/"
  cp "${STP_BIN_DIR}/stp_bl" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_backlog"
  chmod +x "${TEST_TEMP_DIR}/stp_bl"
  
  # Set STP_HOME for the test
  export STP_HOME="${TEST_TEMP_DIR}"
  
  # Create minimal directory structure
  mkdir -p "stp/bin"
  cp "${STP_BIN_DIR}/stp" "stp/bin/"
  chmod +x "stp/bin/stp"
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test if backlog shows help
@test "backlog shows help with no arguments" {
  run ./stp_backlog
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp backlog"* ]]
  [[ "$output" == *"STP wrapper for Backlog.md"* ]]
}

# Test bl alias
@test "bl alias works the same as backlog" {
  run ./stp_bl --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp backlog"* ]]
}

# Test init command
@test "backlog init configures for local use" {
  # Mock the backlog command
  create_mock_command "backlog" 0 "Backlog initialized"
  
  # Create a fake config file that init would create
  mkdir -p backlog
  cat > backlog/config.yml << EOF
project_name: "test"
remote_operations: true
default_status: "todo"
EOF
  
  run ./stp_backlog init
  [ "$status" -eq 0 ]
  [[ "$output" == *"Configuring backlog for STP integration"* ]]
  [[ "$output" == *"Backlog configured for local STP use"* ]]
}

# Test create command validation
@test "backlog create validates steel thread ID" {
  run ./stp_backlog create INVALID "Test task"
  [ "$status" -ne 0 ]
  [[ "$output" == *"Invalid steel thread ID format"* ]]
}

# Test create command with valid ID
@test "backlog create works with valid steel thread ID" {
  # Mock the backlog command
  create_mock_command "backlog" 0 "Created task task-1"
  
  run ./stp_backlog create ST0014 "Test task"
  [ "$status" -eq 0 ]
}

# Test list command adds --plain
@test "backlog list automatically adds --plain flag" {
  # Create a mock backlog that shows what arguments it received
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Arguments: $*"
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"
  
  run ./stp_backlog list
  [ "$status" -eq 0 ]
  [[ "$output" == *"Arguments: task list --plain"* ]]
}

# Test list doesn't duplicate --plain
@test "backlog list doesn't duplicate --plain flag" {
  # Create a mock backlog that shows what arguments it received
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Arguments: $*"
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"
  
  run ./stp_backlog list --plain
  [ "$status" -eq 0 ]
  [[ "$output" == *"Arguments: task list --plain"* ]]
  # Make sure --plain doesn't appear twice
  ! [[ "$output" == *"--plain --plain"* ]]
}

# Test board command does NOT add --plain
@test "backlog board does NOT add --plain flag" {
  # Create a mock backlog that shows what arguments it received
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Arguments: $*"
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"
  
  run ./stp_backlog board
  [ "$status" -eq 0 ]
  [[ "$output" == *"Arguments: board"* ]]
  [[ "$output" != *"--plain"* ]]
}

# Test pass-through of other commands
@test "backlog passes through other commands unchanged" {
  # Create a mock backlog that shows what arguments it received
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Arguments: $*"
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"
  
  run ./stp_backlog config get projectName
  [ "$status" -eq 0 ]
  [[ "$output" == *"Arguments: config get projectName"* ]]
}