#!/bin/bash
# run_tests.sh - Run the Intent test suite
# Usage: ./run_tests.sh [test_path]

# Set up colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to display error messages
error() {
  echo -e "${RED}Error: $1${NC}" >&2
  exit 1
}

# Function to display success messages
success() {
  echo -e "${GREEN}$1${NC}"
}

# Function to display warning messages
warning() {
  echo -e "${YELLOW}Warning: $1${NC}"
}

# Function to display information messages
info() {
  echo -e "${BLUE}$1${NC}"
}

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

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

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

# Run the tests
if [ -d "$TEST_PATH" ]; then
  # If directory, run all .bats files in it (excluding lib directory)
  info "Running all tests in directory: $TEST_PATH"
  find "$TEST_PATH" -name "*.bats" -type f -not -path "*/lib/*" | sort | xargs bats
else
  # If file, run just that file
  info "Running test file: $TEST_PATH"
  bats "$TEST_PATH"
fi

# Check exit status
EXIT_STATUS=$?

echo
if [ $EXIT_STATUS -eq 0 ]; then
  success "All tests passed!"
else
  error "Some tests failed!"
fi

exit $EXIT_STATUS