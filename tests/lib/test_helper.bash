#!/usr/bin/env bash
# Test helper functions and setup for Intent tests

# Set up project-specific paths
# Use absolute paths to ensure tests work from any directory
INTENT_PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INTENT_BIN_DIR="${INTENT_PROJECT_ROOT}/bin"
INTENT_TEST_FIXTURES="${INTENT_PROJECT_ROOT}/tests/fixtures"
INTENT_TEMP_DIR="${INTENT_PROJECT_ROOT}/tests/tmp"

# The CLI under test. Defaults to the shell implementation in this repo; set
# INTENT_BIN in the environment to run the estate against another one (ST0056:
# the v3 binary) without editing a single test.
#
# INTENT_BIN_DIR is NOT a substitute. It names a DIRECTORY of 27 scripts, and the
# ~146 `${INTENT_BIN_DIR}/intent_<sub>` call sites invoke those directly,
# bypassing the bin/intent dispatcher and everything it does (PROJECT_ROOT
# resolution, INTENT_ORIG_CWD, cd to project root -- bin/intent:198-218). Those
# have no equivalent under a single binary and are classified in the register,
# not mechanically retargeted here.
INTENT_BIN="${INTENT_BIN:-${INTENT_BIN_DIR}/intent}"
export INTENT_BIN

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

# Redirect HOME into a per-test sandbox so no test can write to the real
# ~/.claude or ~/.config (ST0042 F-TEST-1/F-TEST-9). Call after TEST_TEMP_DIR
# exists -- from an overridden setup(), or inline in a test that manages its
# own temp dir. Pair with teardown_fake_home in teardown() (or before the
# test's own cleanup).
setup_fake_home() {
  REAL_HOME="$HOME"
  export HOME="${TEST_TEMP_DIR}/fakehome"
  mkdir -p "$HOME/.claude/skills" "$HOME/.claude/agents" "$HOME/.config"
}

teardown_fake_home() {
  if [ -n "${REAL_HOME:-}" ]; then
    export HOME="$REAL_HOME"
    unset REAL_HOME
  fi
}

# Helper function to create a test Intent project
#
# The declared version follows the BINARY UNDER TEST, defaulting to the version
# the conformance harness actually points at. It said "2.10.0" for years, which
# was harmless while only v2 ever read it and stopped being harmless the moment
# v3 learned to detect an unmigrated project (AC-10.7): 2.10.0 is below the
# v2.19.0 migration floor, so the shared builder for 38 .bats files produced a
# project the v3 binary correctly REFUSES at fixture construction, before any
# assertion runs -- and the refusal lands in the same place in the output as
# "family not wired yet", on files already expected to be red. Found by ic,
# who measured it rather than inferring it from my claim that the fixtures
# were fine; they were fine in Rust and not here.
#
# An env var rather than a hardcoded version because the same files run against
# both binaries -- that is what parity means -- and a v2 baseline is entitled
# to a fixture declaring a v2 version.
#
# THE DEFAULT READS `VERSION`, THE SAME FILE `tests/run_tests.sh:61` READS, SO
# THE TWO AGREE BY CONSTRUCTION RATHER THAN BY BEING REMEMBERED AT A RELEASE.
# It used to be a literal `3.0.0`, and the runner defended itself while this
# did not: `run_tests.sh` has exported this from VERSION since `e474b419`, so
# only a DIRECT single-file `bats` run reached the literal -- the invocation
# our own guidance prefers. That built a v3 fixture, drove the v2 binary at it,
# and every test died on the version guard, which reads as "family not wired
# yet" on files already expected to be red.
#
# Re-derived 2026-08-21 before changing it: 41 files call create_test_project
# and NONE of them drive the v3 binary; the 6 files that touch it never call
# this. Nothing anywhere reads INTENT_FIXTURE_VERSION except the runner that
# sets it. So the literal was wrong in 41 of the 41 cases where it could fire.
#
# NOT `get_intent_version()` below, deliberately: that answers a different
# question (the INSTALLATION's version, INTENT_HOME first) and falls back to a
# hardcoded 2.2.1, which is a silent wrong answer -- the class this removes.
create_test_project() {
  local project_name="${1:-Test Project}"
  local dir="${2:-$TEST_TEMP_DIR/test-project}"
  local version="${INTENT_FIXTURE_VERSION:-$(cat "${INTENT_PROJECT_ROOT}/VERSION")}"

  mkdir -p "$dir/intent/.config"
  cat > "$dir/intent/.config/config.json" << EOF
{
  "intent_version": "$version",
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

  echo "$dir"
}

# Write an acceptance.md that the close-gate treats as exempt, into an ST dir.
# ST0048's gate refuses an empty or missing contract; a fixture that exercises
# ST/WP mechanics (not the acceptance contract itself) declares the sanctioned
# escape so the close path is reached.
write_exempt_acceptance() {
  local st_dir="$1"
  cat > "$st_dir/acceptance.md" << 'ACCEPTANCE_EOF'
---
acceptance: exempt
---
# Acceptance (exempt -- fixture exercises ST/WP mechanics, not the gate)
ACCEPTANCE_EOF
}

# Helper function to run intent command
run_intent() {
  "$INTENT_BIN" "$@"
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

# Helper to check if file does not exist
assert_file_not_exists() {
  local file="$1"
  if [ -f "$file" ]; then
    echo "Expected file to not exist: $file"
    return 1
  fi
}

# Helper to check if output does not contain text
refute_output_contains() {
  local text="$1"
  if [[ "$output" == *"$text"* ]]; then
    echo "Expected output to NOT contain: $text"
    echo "Actual output: $output"
    return 1
  fi
}

# Helper to check exact output match
assert_output() {
  local expected="$1"
  if [[ "$output" != "$expected" ]]; then
    echo "Expected output: $expected"
    echo "Actual output: $output"
    return 1
  fi
}

# Helper to get Intent version from VERSION file
get_intent_version() {
  # First try the VERSION file in the Intent installation
  if [ -f "${INTENT_HOME}/VERSION" ]; then
    cat "${INTENT_HOME}/VERSION"
  elif [ -f "${INTENT_PROJECT_ROOT}/VERSION" ]; then
    cat "${INTENT_PROJECT_ROOT}/VERSION"
  elif [ -f "${INTENT_PROJECT_ROOT}/intent/.config/config.json" ]; then
    # Fallback to config.json for compatibility
    jq -r '.version // .intent_version // "2.2.1"' "${INTENT_PROJECT_ROOT}/intent/.config/config.json"
  else
    echo "2.2.1"
  fi
}

# Load bats libraries if available
# Note: bats libraries can be installed globally or added to tests/lib/
# For now, we rely on the basic assert functions defined above