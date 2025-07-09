#!/usr/bin/env bash
# Test helper functions and setup for STP tests

# Set up project-specific paths
# Use absolute paths to ensure tests work from any directory
STP_PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
STP_BIN_DIR="${STP_PROJECT_ROOT}/stp/bin"
STP_TEST_FIXTURES="${STP_PROJECT_ROOT}/stp/tests/fixtures"
STP_TEMP_DIR="${STP_PROJECT_ROOT}/stp/tests/tmp"

# Create temporary test directory
setup_file() {
  mkdir -p "${STP_TEMP_DIR}"
}

# Clean up test directory after all tests in file
teardown_file() {
  if [ -d "${STP_TEMP_DIR}" ]; then
    rm -rf "${STP_TEMP_DIR}"
  fi
}

# Create a temporary test directory for each test
setup() {
  TEST_TEMP_DIR="$(mktemp -d "${STP_TEMP_DIR}/bats-test-XXXXXX")"
  cd "${TEST_TEMP_DIR}" || exit 1
}

# Clean up temporary test directory after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${STP_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Helper function to create a test project directory
create_test_project() {
  local project_name="${1:-Test Project}"
  local dir="${2:-$TEST_TEMP_DIR/test-project}"
  
  mkdir -p "$dir"
  echo "Test project created: $dir"
  echo "Project name: $project_name"
  return 0
}

# Helper function to simulate git environment for tests
simulate_git_environment() {
  local dir="${1:-$TEST_TEMP_DIR}"
  
  mkdir -p "$dir"
  cd "$dir" || exit 1
  
  git init -q
  git config --local user.name "Test User"
  git config --local user.email "test@example.com"
  
  echo "# Test Project" > README.md
  git add README.md
  git commit -q -m "Initial commit"
  
  echo "Git environment set up in $dir"
  return 0
}

# Helper function to verify directory structure
assert_directory_exists() {
  local dir="$1"
  if [ ! -d "$dir" ]; then
    echo "Directory does not exist: $dir"
    return 1
  fi
  return 0
}

# Alias for consistency
assert_dir_exists() {
  assert_directory_exists "$@"
}

# Helper function to verify file existence
assert_file_exists() {
  local file="$1"
  if [ ! -f "$file" ]; then
    echo "File does not exist: $file"
    return 1
  fi
  return 0
}

# Helper function to verify file content
assert_file_contains() {
  local file="$1"
  local pattern="$2"
  
  assert_file_exists "$file" || return 1
  
  if ! grep -q "$pattern" "$file"; then
    echo "File does not contain pattern: $pattern"
    echo "File content:"
    cat "$file"
    return 1
  fi
  return 0
}

# Helper function to create a mock command that can be invoked in tests
create_mock_command() {
  local command_name="$1"
  local exit_status="${2:-0}"
  local output="${3:-}"
  
  mkdir -p "${TEST_TEMP_DIR}/bin"
  cat > "${TEST_TEMP_DIR}/bin/${command_name}" << EOF
#!/bin/bash
echo "${output}"
exit ${exit_status}
EOF
  chmod +x "${TEST_TEMP_DIR}/bin/${command_name}"
  export PATH="${TEST_TEMP_DIR}/bin:$PATH"
}