#!/usr/bin/env bash
# Test helper functions and setup for Intent tests

# Set up project-specific paths
# Use absolute paths to ensure tests work from any directory
INTENT_PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INTENT_BIN_DIR="${INTENT_PROJECT_ROOT}/bin"
INTENT_TEST_FIXTURES="${INTENT_PROJECT_ROOT}/tests/fixtures"
INTENT_TEMP_DIR="${INTENT_PROJECT_ROOT}/tests/tmp"

# Export INTENT_HOME for tests
export INTENT_HOME="${INTENT_PROJECT_ROOT}"

# Create temporary test directory
setup_file() {
  mkdir -p "${INTENT_TEMP_DIR}"
}

# Clean up test directory after all tests in file
teardown_file() {
  if [ -d "${INTENT_TEMP_DIR}" ]; then
    rm -rf "${INTENT_TEMP_DIR}"
  fi
}

# Create a temporary test directory for each test
setup() {
  # Create temp dir outside of Intent project to test "outside project" scenarios
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
}

# Clean up temporary test directory after each test
teardown() {
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# Helper function to create a test Intent project
create_test_project() {
  local project_name="${1:-Test Project}"
  local dir="${2:-$TEST_TEMP_DIR/test-project}"
  
  mkdir -p "$dir/.intent"
  cat > "$dir/.intent/config.json" << EOF
{
  "intent_version": "2.0.0",
  "project_name": "$project_name",
  "author": "test_user",
  "created_date": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF
  
  # Create standard directories
  mkdir -p "$dir/intent/st/COMPLETED"
  mkdir -p "$dir/intent/st/NOT-STARTED"
  mkdir -p "$dir/intent/st/CANCELLED"
  mkdir -p "$dir/intent/eng/tpd"
  mkdir -p "$dir/intent/ref"
  mkdir -p "$dir/intent/llm"
  mkdir -p "$dir/backlog"
  
  echo "$dir"
}

# Helper function to run intent command
run_intent() {
  "${INTENT_BIN_DIR}/intent" "$@"
}

# Helper to check if command output contains expected text
assert_output_contains() {
  local expected="$1"
  if [[ "$output" != *"$expected"* ]]; then
    echo "Expected output to contain: $expected"
    echo "Actual output: $output"
    return 1
  fi
}

# Helper to check if command succeeded
assert_success() {
  if [ "$status" -ne 0 ]; then
    echo "Expected command to succeed, but it failed with status $status"
    echo "Output: $output"
    return 1
  fi
}

# Helper to check if command failed
assert_failure() {
  if [ "$status" -eq 0 ]; then
    echo "Expected command to fail, but it succeeded"
    echo "Output: $output"
    return 1
  fi
}

# Helper for test failures
fail() {
  echo "$1"
  return 1
}

# Helper to check if file exists
assert_file_exists() {
  local file="$1"
  if [ ! -f "$file" ]; then
    echo "Expected file to exist: $file"
    return 1
  fi
}

# Helper to check if directory exists
assert_directory_exists() {
  local dir="$1"
  if [ ! -d "$dir" ]; then
    echo "Expected directory to exist: $dir"
    return 1
  fi
}

# Helper to check if file contains text
assert_file_contains() {
  local file="$1"
  local text="$2"
  if ! grep -qF "$text" "$file"; then
    echo "Expected file $file to contain: $text"
    echo "File contents:"
    cat "$file"
    return 1
  fi
}

# Load bats libraries if available
# Note: bats libraries can be installed globally or added to tests/lib/
# For now, we rely on the basic assert functions defined above