#!/bin/bash
# run_tests.sh - Run the STP test suite
# Usage: ./run_tests.sh [test_path]

# Set up colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
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
  echo -e "$1"
}

# Check if bats is installed
if ! command -v bats &> /dev/null; then
  # Check if we're on macOS with Homebrew
  if [[ "$OSTYPE" == "darwin"* ]] && command -v brew &> /dev/null; then
    error "Bats is not installed. Please install it first:

On macOS with Homebrew:
  brew install bats-core

Or run the setup script:
  ./setup_test_env.sh"
  else
    error "Bats is not installed. Please install it first:

Install from source:
  git clone https://github.com/bats-core/bats-core.git
  cd bats-core
  ./install.sh /usr/local

Or run the setup script:
  ./setup_test_env.sh"
  fi
fi

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STP_ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Check if libraries are installed
BATS_SUPPORT="$SCRIPT_DIR/lib/bats-support"
BATS_ASSERT="$SCRIPT_DIR/lib/bats-assert"
BATS_FILE="$SCRIPT_DIR/lib/bats-file"

if [ ! -d "$BATS_SUPPORT" ] || [ ! -d "$BATS_ASSERT" ] || [ ! -d "$BATS_FILE" ]; then
  warning "Bats libraries are not installed in the test directory. Some tests may fail."
  warning "To install the libraries:"
  warning "  mkdir -p \"$SCRIPT_DIR/lib\""
  warning "  git clone https://github.com/bats-core/bats-support.git \"$BATS_SUPPORT\""
  warning "  git clone https://github.com/bats-core/bats-assert.git \"$BATS_ASSERT\""
  warning "  git clone https://github.com/bats-core/bats-file.git \"$BATS_FILE\""
  echo ""
  
  # Update test_helper.bash to use local libraries if they exist
  if [ -f "$SCRIPT_DIR/lib/test_helper.bash" ]; then
    sed -i.bak "s|# load '/usr/local/lib/bats-support/load.bash'|# Check if libraries exist locally\nif [ -d \"$SCRIPT_DIR/lib/bats-support\" ]; then\n  load \"$SCRIPT_DIR/lib/bats-support/load.bash\"\nfi|" "$SCRIPT_DIR/lib/test_helper.bash"
    sed -i.bak "s|# load '/usr/local/lib/bats-assert/load.bash'|if [ -d \"$SCRIPT_DIR/lib/bats-assert\" ]; then\n  load \"$SCRIPT_DIR/lib/bats-assert/load.bash\"\nfi|" "$SCRIPT_DIR/lib/test_helper.bash"
    sed -i.bak "s|# load '/usr/local/lib/bats-file/load.bash'|if [ -d \"$SCRIPT_DIR/lib/bats-file\" ]; then\n  load \"$SCRIPT_DIR/lib/bats-file/load.bash\"\nfi|" "$SCRIPT_DIR/lib/test_helper.bash"
    rm -f "$SCRIPT_DIR/lib/test_helper.bash.bak"
    success "Updated test_helper.bash to use local libraries if they exist"
  fi
fi

# Create temporary directory for test artifacts
mkdir -p "$SCRIPT_DIR/tmp"

# Determine which tests to run
if [ $# -gt 0 ]; then
  TEST_PATH="$1"
  if [ ! -e "$TEST_PATH" ]; then
    # Try to resolve relative to the script directory
    if [ -e "$SCRIPT_DIR/$TEST_PATH" ]; then
      TEST_PATH="$SCRIPT_DIR/$TEST_PATH"
    else
      error "Test path not found: $TEST_PATH"
    fi
  fi
else
  # Run all tests by default
  TEST_PATH="$SCRIPT_DIR"
fi

# Display information about the test run
info "Running STP Tests"
info "================="
info "STP Root: $STP_ROOT_DIR"
info "Test Path: $TEST_PATH"
info "Bats Path: $(which bats 2>/dev/null || echo 'Not found')"

# Check for required libraries
if [ ! -d "$SCRIPT_DIR/lib/bats-support" ] || [ ! -d "$SCRIPT_DIR/lib/bats-assert" ] || [ ! -d "$SCRIPT_DIR/lib/bats-file" ]; then
  warning "Some Bats libraries are missing. Running setup_test_env.sh to install them..."
  
  # Run setup_test_env if it exists and is executable
  if [ -x "$SCRIPT_DIR/setup_test_env.sh" ]; then
    "$SCRIPT_DIR/setup_test_env.sh"
  else
    warning "setup_test_env.sh not found or not executable. Please run it manually to set up dependencies."
  fi
fi

echo ""

# Run the tests
if [[ -d "$TEST_PATH" ]]; then
  # If directory, run all .bats files in it, excluding the lib directory
  find "$TEST_PATH" -name "*.bats" | grep -v "/lib/" | sort | while read -r test_file; do
    info "Running test file: $(basename "$test_file")"
    if bats "$test_file"; then
      success "✓ $(basename "$test_file") passed"
    else
      error "✗ $(basename "$test_file") failed"
    fi
    echo ""
  done
else
  # Run a specific test file
  info "Running test file: $(basename "$TEST_PATH")"
  if bats "$TEST_PATH"; then
    success "✓ $(basename "$TEST_PATH") passed"
  else
    error "✗ $(basename "$TEST_PATH") failed"
  fi
fi

# Clean up
rm -rf "$SCRIPT_DIR/tmp"

success "All tests completed."