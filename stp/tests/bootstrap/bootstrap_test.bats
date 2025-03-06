#!/usr/bin/env bats
# Tests for the bootstrap script

load '../lib/test_helper.bash'

# Setup test environment before each test
setup() {
  # Create a temporary test directory
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/bootstrap-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Copy the bootstrap script to the test directory
  cp "${STP_BIN_DIR}/bootstrap" "${TEST_TEMP_DIR}/"
  chmod +x "${TEST_TEMP_DIR}/bootstrap"
}

# Clean up after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Test if bootstrap runs without error
@test "bootstrap executes without error" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  [ -n "$output" ]
}

# Test if bootstrap creates the expected directory structure
@test "bootstrap creates the correct directory structure" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  
  # Check main directories
  assert_directory_exists "stp"
  assert_directory_exists "stp/bin"
  assert_directory_exists "stp/prj"
  assert_directory_exists "stp/prj/st"
  assert_directory_exists "stp/eng"
  assert_directory_exists "stp/eng/tpd"
  assert_directory_exists "stp/usr"
  assert_directory_exists "stp/llm"
  assert_directory_exists "stp/_templ"
  assert_directory_exists "bin"
}

# Test if bootstrap creates the expected template files
@test "bootstrap creates the correct template files" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  
  # Check template files
  assert_file_exists "stp/_templ/prj/_wip.md"
  assert_file_exists "stp/_templ/prj/_journal.md"
  assert_file_exists "stp/_templ/prj/st/_steel_threads.md"
  assert_file_exists "stp/_templ/prj/st/_ST####.md"
  assert_file_exists "stp/_templ/eng/tpd/_technical_product_design.md"
  assert_file_exists "stp/_templ/usr/_user_guide.md"
  assert_file_exists "stp/_templ/llm/_llm_preamble.md"
}

# Test if bootstrap creates the expected project files
@test "bootstrap creates the correct project files" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  
  # Check project files
  assert_file_exists "stp/prj/wip.md"
  assert_file_exists "stp/prj/journal.md"
  assert_file_exists "stp/prj/st/steel_threads.md"
  assert_file_exists "stp/prj/st/ST0001.md"
  assert_file_exists "stp/prj/st/ST0002.md"
}

# Test if bootstrap creates the expected engineering files
@test "bootstrap creates the correct engineering files" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  
  # Check engineering files
  assert_file_exists "stp/eng/tpd/technical_product_design.md"
  assert_file_exists "stp/eng/tpd/1_introduction.md"
  assert_file_exists "stp/eng/tpd/2_requirements.md"
  assert_file_exists "stp/eng/tpd/3_architecture.md"
  assert_file_exists "stp/eng/tpd/4_detailed_design.md"
  assert_file_exists "stp/eng/tpd/5_implementation_strategy.md"
  assert_file_exists "stp/eng/tpd/6_deployment_and_operations.md"
  assert_file_exists "stp/eng/tpd/7_technical_challenges_and_mitigations.md"
  assert_file_exists "stp/eng/tpd/8_appendices.md"
}

# Test if bootstrap creates the expected user documentation files
@test "bootstrap creates the correct user documentation files" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  
  # Check user documentation files
  assert_file_exists "stp/usr/user_guide.md"
  assert_file_exists "stp/usr/reference_guide.md"
  assert_file_exists "stp/usr/deployment_guide.md"
}

# Test if bootstrap creates the expected LLM files
@test "bootstrap creates the correct LLM files" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  
  # Check LLM files
  assert_file_exists "stp/llm/llm_preamble.md"
}

# Test if bootstrap creates files with the correct author information
@test "bootstrap uses the provided author name in files" {
  local author="Custom Author"
  run ./bootstrap "$author"
  [ "$status" -eq 0 ]
  
  # Check if author is correctly set in a file
  assert_file_contains "stp/prj/wip.md" "$author"
}

# Test if bootstrap uses git config for author name when not provided
@test "bootstrap uses git config for author name when not provided" {
  # Set up git environment
  git init -q
  git config --local user.name "Git User"
  
  run ./bootstrap
  [ "$status" -eq 0 ]
  
  # Check if git user name is used
  assert_file_contains "stp/prj/wip.md" "Git User"
}

# Test if bootstrap creates script files
@test "bootstrap creates script files" {
  run ./bootstrap "Test User"
  [ "$status" -eq 0 ]
  
  # Check script files existence
  assert_file_exists "bin/stp"
  assert_file_exists "bin/stp_init"
  assert_file_exists "bin/stp_st"
  assert_file_exists "bin/stp_help"
  assert_file_exists "bin/bootstrap"
  
  assert_file_exists "stp/bin/stp"
  assert_file_exists "stp/bin/stp_init"
  assert_file_exists "stp/bin/stp_st"
  assert_file_exists "stp/bin/stp_help"
  assert_file_exists "stp/bin/bootstrap"
}