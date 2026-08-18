#!/bin/bash
# run_tests.sh - Run the Intent test suite
# Usage: ./run_tests.sh [test_path]

set -e

# Scrub ambient project state BEFORE this script computes its own.
#
# PROJECT_ROOT is a generic name and Intent treats it as an answer, not a hint:
# `plugin_get_manifest_path` writes the project-local manifest when it is set
# and the $HOME one when it is not. A parent that exports it -- a Makefile,
# direnv, CI, or devbin, which exports it on every invocation -- silently flips
# suites onto the wrong branch. That is a real defect in its own right (filed
# separately); it is simply not the suite's business to inherit it. A test
# decides its own environment or it measures the machine it happens to run on.
#
# The assignment below is deliberately NOT exported, so child `intent`
# processes resolve the project themselves, which is what the suites test.
unset PROJECT_ROOT INTENT_ROOT BIN_DIR

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

# Fixtures must declare the version of the binary THIS script drives.
#
# `create_test_project` in tests/lib/test_helper.bash defaults the fixture's
# `intent_version` to 3.0.0, because the same .bats files run against both
# binaries and v3 is the one under active development. Its comment already
# names the remedy for the other direction -- "INTENT_FIXTURE_VERSION=2.19.0
# restores a v2-shaped estate" -- and until now NOTHING IN THE TREE SET IT:
# the only two mentions of the variable were the comment describing it and the
# line reading it. A capability no consumer consults distinguishes nothing.
#
# It went unnoticed because it needed a second commit to bite. `53f88757` gave
# v2 a forward-compatibility guard that refuses a project from the future --
# correctly. From then on this script built v3 fixtures and drove a v2 binary
# at them, and v2 refused all of them: 299 failures across 24 files, 297 of
# them that one refusal, including all five files that guard the acceptance
# gate. Neither commit is wrong; the pair is.
#
# Read from VERSION rather than written as a literal, so this tracks the tool
# instead of becoming a second place that has to be remembered at a release.
export INTENT_FIXTURE_VERSION="$(cat "$PROJECT_ROOT/VERSION")"

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