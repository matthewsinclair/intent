#!/bin/bash
# run_tests.sh - Run the Intent test suite
# Usage: ./run_tests.sh [test_path]

set -e

# Locate the project root and source canonical helpers (error/warning/info).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
source "$PROJECT_ROOT/bin/intent_helpers"

# Check if bats is installed
if ! command -v bats &> /dev/null; then
  error "Bats is not installed. Please install it first:

On macOS with Homebrew:
  brew install bats-core

Or install from source:
  git clone https://github.com/bats-core/bats-core.git
  cd bats-core
  ./install.sh /usr/local"
fi

# Export INTENT_HOME for tests
export INTENT_HOME="$PROJECT_ROOT"

# Export BATS_LIB_PATH for bats libraries
export BATS_LIB_PATH="$SCRIPT_DIR/lib"

# Set default test path - only run new Intent tests by default
# To run old STP tests: ./tests/run_tests.sh ../stp/tests
TEST_PATH="${1:-$SCRIPT_DIR}"

# Check if test path exists
if [ ! -e "$TEST_PATH" ]; then
  error "Test path does not exist: $TEST_PATH"
fi

# Display test information
echo
info "Intent Test Suite"
info "================"
info "INTENT_HOME: $INTENT_HOME"
info "Test path: $TEST_PATH"
echo

# Run the tests. Capture the exit code without aborting under `set -e` so we
# can report a final pass/fail line.
EXIT_STATUS=0
if [ -d "$TEST_PATH" ]; then
  # If directory, run all .bats files in it (excluding lib directory)
  info "Running all tests in directory: $TEST_PATH"
  find "$TEST_PATH" -name "*.bats" -type f -not -path "*/lib/*" -print0 | sort -z | xargs -0 bats || EXIT_STATUS=$?
else
  # If file, run just that file
  info "Running test file: $TEST_PATH"
  bats "$TEST_PATH" || EXIT_STATUS=$?
fi

echo
if [ "$EXIT_STATUS" -eq 0 ]; then
  info "All tests passed!"
else
  error "Some tests failed!"
fi

exit "$EXIT_STATUS"