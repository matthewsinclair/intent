#!/usr/bin/env bats
# Tests for the stp bl command (shorthand for stp backlog)

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/bl-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy required scripts
  cp "${STP_BIN_DIR}/stp_backlog" "${TEST_TEMP_DIR}/"
  cp "${STP_BIN_DIR}/stp_bl" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_backlog"
  chmod +x "${TEST_TEMP_DIR}/stp_bl"
  
  # Create bin directory structure for STP_HOME
  mkdir -p "${TEST_TEMP_DIR}/stp/bin"
  cp "${STP_BIN_DIR}/stp" "${TEST_TEMP_DIR}/stp/bin/"
  cp "${STP_BIN_DIR}/stp_backlog" "${TEST_TEMP_DIR}/stp/bin/"
  cp "${STP_BIN_DIR}/stp_bl" "${TEST_TEMP_DIR}/stp/bin/"
  chmod +x "${TEST_TEMP_DIR}/stp/bin/"*
  
  # Set STP_HOME for the test
  export STP_HOME="${TEST_TEMP_DIR}"
  
  # Mock backlog command for most tests
  mkdir -p "${TEST_TEMP_DIR}/bin"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test stp bl command works
@test "stp bl shows help" {
  run ./stp_bl
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp backlog"* ]]
  [[ "$output" == *"stp bl"* ]]
}

# Test stp bl list command
@test "stp bl list adds --plain automatically" {
  # Mock backlog to show arguments
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
if [[ "$*" == "task list --plain" ]]; then
  echo "To Do:"
  echo "  task-1 - Test task"
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  
  run ./stp_bl list
  [ "$status" -eq 0 ]
  [[ "$output" == *"Backlog called with: task list --plain"* ]]
  [[ "$output" == *"task-1 - Test task"* ]]
}

# Test stp bl create command
@test "stp bl create validates and creates task" {
  # Mock backlog for task creation
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
if [[ "$1" == "task" && "$2" == "create" && "$3" == "ST0014 - Test task" ]]; then
  echo "Created task task-1"
  echo "File: backlog/tasks/task-1 - ST0014-Test-task.md"
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  
  run ./stp_bl create ST0014 "Test task"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Created task task-1"* ]]
}

# Test stp bl create with invalid ID
@test "stp bl create rejects invalid steel thread ID" {
  run ./stp_bl create INVALID "Test task"
  [ "$status" -ne 0 ]
  [[ "$output" == *"Invalid steel thread ID format"* ]]
}

# Test stp bl board command
@test "stp bl board passes through without --plain" {
  # Mock backlog to show arguments
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
if [[ "$*" == "board" ]]; then
  echo "Kanban board displayed"
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  
  run ./stp_bl board
  [ "$status" -eq 0 ]
  [[ "$output" == *"Backlog called with: board"* ]]
  [[ "$output" == *"Kanban board displayed"* ]]
}

# Test stp bl init command
@test "stp bl init configures backlog for STP" {
  # Mock backlog for init and config
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
if [[ "$1" == "init" ]]; then
  mkdir -p backlog
  echo "project_name: test" > backlog/config.yml
  echo "Backlog initialized"
  exit 0
elif [[ "$1" == "config" && "$2" == "set" ]]; then
  echo "Config set: $3 = $4"
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  
  run ./stp_bl init
  [ "$status" -eq 0 ]
  [[ "$output" == *"Backlog initialized"* ]]
  [[ "$output" == *"Configuring backlog for STP integration"* ]]
  [[ "$output" == *"Backlog configured for local STP use"* ]]
}

# Test pass-through of task subcommands
@test "stp bl task edit passes through correctly" {
  # Mock backlog to show arguments
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
if [[ "$*" == "task edit task-5 --status Done" ]]; then
  echo "Task updated"
  exit 0
fi
exit 1
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  
  run ./stp_bl task edit task-5 --status Done
  [ "$status" -eq 0 ]
  [[ "$output" == *"Backlog called with: task edit task-5 --status Done"* ]]
  [[ "$output" == *"Task updated"* ]]
}

# Test that other commands are passed through
@test "stp bl passes through unknown commands" {
  # Mock backlog to show arguments
  cat > "${TEST_TEMP_DIR}/bin/backlog" << 'EOF'
#!/bin/bash
echo "Backlog called with: $*"
exit 0
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/backlog"
  
  run ./stp_bl decision create "Architecture choice"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Backlog called with: decision create Architecture choice"* ]]
}

# Test integration through main stp command
@test "stp bl works through main stp command" {
  # Ensure help file exists
  mkdir -p "stp/bin/.help"
  echo "@short:" > "stp/bin/.help/bl.help.md"
  echo "Shorthand for backlog" >> "stp/bin/.help/bl.help.md"
  
  run stp/bin/stp bl --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: stp backlog"* ]]
}