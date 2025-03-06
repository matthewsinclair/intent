#!/usr/bin/env bats
# Tests for the stp_init script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/init-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the init script to the test directory
  cp "${STP_BIN_DIR}/stp_init" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/stp_init"
  
  # Create a minimal STP_HOME structure with required directories
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/_templ/prj/st"
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/_templ/eng/tpd"
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/_templ/usr"
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/_templ/llm"
  mkdir -p "${TEST_TEMP_DIR}/stp_home/stp/bin"
  
  # Create minimal template files
  create_template_files
  
  # Set STP_HOME environment variable
  export STP_HOME="${TEST_TEMP_DIR}/stp_home"
  
  # Create expect script for running stp_init non-interactively
  cat > "${TEST_TEMP_DIR}/run_init.exp" << 'EOF'
#!/usr/bin/expect -f
set timeout 5
set project_name [lindex $argv 0]
set target_dir [lindex $argv 1]

# Get the command to run
if {$target_dir eq ""} {
    set cmd "./stp_init \"$project_name\""
} else {
    set cmd "./stp_init \"$project_name\" \"$target_dir\""
}

# Execute the command
spawn {*}$cmd

# Handle any "directory not empty" prompts
expect {
    "Press Enter to continue or Ctrl+C to cancel" {
        send "\r"
        exp_continue
    }
    timeout {
        exit 1
    }
    eof
}
EOF
  
  chmod +x "${TEST_TEMP_DIR}/run_init.exp"
  
  # Skip tests if expect is not available
  if ! command -v expect &> /dev/null; then
    skip "expect command is not available"
  fi
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    unset STP_HOME
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Helper function to create minimal template files for testing
create_template_files() {
  # Create template files
  echo "Template WIP" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/prj/_wip.md"
  echo "Template Journal" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/prj/_journal.md"
  echo "Template Steel Threads" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/prj/st/_steel_threads.md"
  echo "Template ST####" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/prj/st/_ST####.md"
  echo "Template TPD" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/eng/tpd/_technical_product_design.md"
  echo "Template User Guide" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/usr/_user_guide.md"
  echo "Template Reference Guide" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/usr/_reference_guide.md"
  echo "Template Deployment Guide" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/usr/_deployment_guide.md"
  echo "Template LLM Preamble" > "${TEST_TEMP_DIR}/stp_home/stp/_templ/llm/_llm_preamble.md"
  
  # Create mock scripts
  touch "${TEST_TEMP_DIR}/stp_home/stp/bin/stp"
  touch "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_init"
  touch "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_st"
  touch "${TEST_TEMP_DIR}/stp_home/stp/bin/stp_help"
  chmod +x "${TEST_TEMP_DIR}/stp_home/stp/bin/"*
}

# Test if init requires STP_HOME to be set
@test "init requires STP_HOME environment variable" {
  unset STP_HOME
  run ./stp_init "Test Project"
  [ "$status" -ne 0 ]
  [[ "$output" == *"STP_HOME environment variable is not set"* ]]
}

# Test if init requires project name argument
@test "init requires project name argument" {
  run ./stp_init
  [ "$status" -ne 0 ]
  [[ "$output" == *"Project name is required"* ]]
}

# Test if init creates a project in the current directory by default
@test "init creates a project in the current directory by default" {
  # Create a clean test directory
  mkdir -p "${TEST_TEMP_DIR}/test-dir"
  cd "${TEST_TEMP_DIR}/test-dir"
  
  # Copy necessary files
  cp "${TEST_TEMP_DIR}/stp_init" ./
  cp "${TEST_TEMP_DIR}/run_init.exp" ./
  chmod +x ./stp_init ./run_init.exp
  
  # Run with expect to handle interactive prompts
  run ./run_init.exp "Test Project"
  [ "$status" -eq 0 ]
  
  # Check if project was created in current directory with default directories
  assert_directory_exists "stp"
  assert_directory_exists "stp/.config"
  assert_directory_exists "stp/prj"
  assert_directory_exists "stp/eng"
  assert_directory_exists "stp/usr"
  assert_directory_exists "stp/llm"
  
  # Return to the original test directory
  cd "${TEST_TEMP_DIR}"
}

# Test if init creates a project in a specified directory
@test "init creates a project in a specified directory" {
  # Create a clean test directory
  mkdir -p "${TEST_TEMP_DIR}/specified-dir-test"
  cd "${TEST_TEMP_DIR}/specified-dir-test"
  
  # Create target directory
  mkdir -p "${TEST_TEMP_DIR}/specified-dir-test/target-dir"
  
  # Copy necessary files
  cp "${TEST_TEMP_DIR}/stp_init" ./
  cp "${TEST_TEMP_DIR}/run_init.exp" ./
  chmod +x ./stp_init ./run_init.exp
  
  # Run with expect to handle interactive prompts
  run ./run_init.exp "Test Project" "target-dir"
  [ "$status" -eq 0 ]
  
  # Check if project was created in the specified directory with default directories
  assert_directory_exists "target-dir/stp"
  assert_directory_exists "target-dir/stp/.config"
  assert_directory_exists "target-dir/stp/prj"
  assert_directory_exists "target-dir/stp/eng"
  
  # Return to the original test directory
  cd "${TEST_TEMP_DIR}"
}

# Test if init creates the configuration file
@test "init creates the configuration file" {
  # Create a clean test directory
  mkdir -p "${TEST_TEMP_DIR}/config-test"
  cd "${TEST_TEMP_DIR}/config-test"
  
  # Copy necessary files
  cp "${TEST_TEMP_DIR}/stp_init" ./
  cp "${TEST_TEMP_DIR}/run_init.exp" ./
  chmod +x ./stp_init ./run_init.exp
  
  # Run with expect to handle interactive prompts
  run ./run_init.exp "Test Project"
  [ "$status" -eq 0 ]
  
  # Check if configuration file was created
  assert_file_exists "stp/.config/config"
  assert_file_contains "stp/.config/config" "PROJECT_NAME=\"Test Project\""
  
  # Return to the original test directory
  cd "${TEST_TEMP_DIR}"
}

# Test if init creates project files from templates
@test "init creates project files from templates" {
  # Create a clean test directory
  mkdir -p "${TEST_TEMP_DIR}/template-test"
  cd "${TEST_TEMP_DIR}/template-test"
  
  # Copy necessary files
  cp "${TEST_TEMP_DIR}/stp_init" ./
  cp "${TEST_TEMP_DIR}/run_init.exp" ./
  chmod +x ./stp_init ./run_init.exp
  
  # Run with expect to handle interactive prompts
  run ./run_init.exp "Test Project"
  [ "$status" -eq 0 ]
  
  # Check if files were created from templates
  assert_file_exists "stp/prj/wip.md"
  assert_file_exists "stp/prj/journal.md"
  assert_file_exists "stp/prj/st/steel_threads.md"
  assert_file_exists "stp/eng/tpd/technical_product_design.md"
  assert_file_exists "stp/usr/user_guide.md"
  assert_file_exists "stp/usr/reference_guide.md"
  assert_file_exists "stp/usr/deployment_guide.md"
  assert_file_exists "stp/llm/llm_preamble.md"
  
  # Return to the original test directory
  cd "${TEST_TEMP_DIR}"
}

# Test if init copies scripts and makes them executable when --all is specified
@test "init copies scripts and makes them executable" {
  # Create a clean test directory
  mkdir -p "${TEST_TEMP_DIR}/scripts-test"
  cd "${TEST_TEMP_DIR}/scripts-test"
  
  # Copy necessary files
  cp "${TEST_TEMP_DIR}/stp_init" ./
  cp "${TEST_TEMP_DIR}/run_init.exp" ./
  chmod +x ./stp_init ./run_init.exp
  
  # Create a modified expect script that uses the --all flag
  cat > "./run_init_all.exp" << 'EOF'
#!/usr/bin/expect -f
set timeout 5
set project_name [lindex $argv 0]
set target_dir [lindex $argv 1]

# Get the command to run
if {$target_dir eq ""} {
    set cmd "./stp_init --all \"$project_name\""
} else {
    set cmd "./stp_init --all \"$project_name\" \"$target_dir\""
}

# Execute the command
spawn {*}$cmd

# Handle any "directory not empty" prompts
expect {
    "Press Enter to continue or Ctrl+C to cancel" {
        send "\r"
        exp_continue
    }
    timeout {
        exit 1
    }
    eof
}
EOF
  chmod +x ./run_init_all.exp
  
  # Run with expect to handle interactive prompts
  run ./run_init_all.exp "Test Project"
  [ "$status" -eq 0 ]
  
  # Check if scripts were copied and are executable
  assert_file_exists "stp/bin/stp"
  assert_file_exists "stp/bin/stp_init"
  assert_file_exists "stp/bin/stp_st"
  assert_file_exists "stp/bin/stp_help"
  
  [ -x "stp/bin/stp" ]
  [ -x "stp/bin/stp_init" ]
  [ -x "stp/bin/stp_st" ]
  [ -x "stp/bin/stp_help" ]
  
  # Return to the original test directory
  cd "${TEST_TEMP_DIR}"
}

# Test if init handles non-empty target directory
@test "init warns about non-empty target directory but continues" {
  mkdir -p "${TEST_TEMP_DIR}/non-empty-dir"
  touch "${TEST_TEMP_DIR}/non-empty-dir/existing-file.txt"
  
  # Use expect to handle interactive prompt
  cat > "${TEST_TEMP_DIR}/expect-script" << EOF
#!/usr/bin/expect -f
set timeout 5
spawn ./stp_init "Test Project" "${TEST_TEMP_DIR}/non-empty-dir"
expect "Warning: Target directory is not empty"
send "\r"
expect eof
EOF
  
  chmod +x "${TEST_TEMP_DIR}/expect-script"
  
  # Skip this test if expect is not available
  if ! command -v expect &> /dev/null; then
    skip "expect command is not available"
  fi
  
  run "${TEST_TEMP_DIR}/expect-script"
  
  # Check if project was created despite directory not being empty
  assert_directory_exists "${TEST_TEMP_DIR}/non-empty-dir/stp"
  assert_file_exists "${TEST_TEMP_DIR}/non-empty-dir/existing-file.txt"
}

# Test if init creates local configuration for STP
@test "init creates local configuration for STP" {
  # Create a clean test directory
  mkdir -p "${TEST_TEMP_DIR}/config-local-test"
  cd "${TEST_TEMP_DIR}/config-local-test"
  
  # Copy necessary files
  cp "${TEST_TEMP_DIR}/stp_init" ./
  cp "${TEST_TEMP_DIR}/run_init.exp" ./
  chmod +x ./stp_init ./run_init.exp
  
  # Run with expect to handle interactive prompts
  run ./run_init.exp "Test Project"
  [ "$status" -eq 0 ]
  
  # Check if local configuration was created
  assert_file_exists "stp/.config/stp_config.sh"
  assert_file_contains "stp/.config/stp_config.sh" "export STP_PROJECT=\"Test Project\""
  
  # Return to the original test directory
  cd "${TEST_TEMP_DIR}"
}