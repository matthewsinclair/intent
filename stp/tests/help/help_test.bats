#!/usr/bin/env bats
# Tests for the stp_help script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/help-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the help script to the test directory
  cp "${STP_BIN_DIR}/stp_help" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_help"
  
  # Create minimal STP_HOME structure with required directories
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/bin/.help"
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/usr"
  
  # Create sample help files with proper format
  cat > "${TEST_TEMP_DIR}/stp_home/stp/bin/.help/init.help.md" << EOF
# init command

@short:
Initialize a new STP project

@description:
Detailed description here
EOF
  
  cat > "${TEST_TEMP_DIR}/stp_home/stp/bin/.help/st.help.md" << EOF
# st command

@short:
Manage steel threads

@description:
Detailed description here
EOF
  
  # Create sample scripts
  touch "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_init"
  touch "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_st"
  chmod +x "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_init"
  chmod +x "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_st"
  
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

# Test if help requires STP_HOME to be set
@test "help requires STP_HOME environment variable" {
  unset STP_HOME
  run ./stp_help
  [ "$status" -ne 0 ]
  [[ "$output" == *"STP_HOME environment variable is not set"* ]]
}

# Test if help displays general help when no command is specified
@test "help displays general help when no command is specified" {
  run ./stp_help
  [ "$status" -eq 0 ]
  [[ "$output" == *"STP - Steel Thread Process"* ]]
  [[ "$output" == *"Available commands:"* ]]
  [[ "$output" == *"init"* ]]
  [[ "$output" == *"st"* ]]
}

# Test if help displays command-specific help when a command is specified
@test "help displays command-specific help when a command is specified" {
  run ./stp_help init
  [ "$status" -eq 0 ]
  [[ "$output" == *"init command"* ]]
}

# Test if help shows short descriptions from help files
@test "help shows short descriptions from help files" {
  run ./stp_help
  [ "$status" -eq 0 ]
  # Use regex pattern to match the output with flexible whitespace
  [[ "$output" =~ init[[:space:]]+Initialize[[:space:]]a[[:space:]]new[[:space:]]STP[[:space:]]project ]]
  [[ "$output" =~ st[[:space:]]+Manage[[:space:]]steel[[:space:]]threads ]]
}

# Test if help handles unknown commands correctly
@test "help handles unknown commands correctly" {
  run ./stp_help unknown_command
  [ "$status" -ne 0 ]
  [[ "$output" == *"Unknown command 'unknown_command'"* ]]
}

# Test if help handles commands with no help files correctly
@test "help handles commands with no help files correctly" {
  # Create a command with no help file
  touch "${STP_HOME}/stp/bin/stp_nohelp"
  chmod +x "${STP_HOME}/stp/bin/stp_nohelp"
  
  run ./stp_help nohelp
  [ "$status" -eq 0 ]
  [[ "$output" == *"No help available for command 'nohelp'"* ]]
}