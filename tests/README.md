# Intent Test Suite

## Overview

This directory contains the test suite for Intent. The tests are written using [Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System).

## Directory Structure

```
tests/
├── unit/                          # One .bats file per subject
├── fixtures/                      # Test fixtures (sample files, etc.)
├── lib/                           # Test libraries
│   └── test_helper.bash           # Common test functions
├── run_tests.sh                   # Main test runner
└── README.md                      # This file
```

The suite drives the v3 binary at `native/rust/target/release/intent`, so build it first (`cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli -p intentd`, as CI does), or set `INTENT_BIN` to another binary. Suites that commit through the pre-commit hook (eg `pre_commit_hook.bats`) also need an `intent` on PATH: the hook refuses a commit in an Intent project when it cannot run `intent`.

## Prerequisites

Install Bats:

```bash
# macOS with Homebrew
brew install bats-core

# Or from source
git clone https://github.com/bats-core/bats-core.git
cd bats-core
./install.sh /usr/local
```

## Running Tests

### Run all tests:

```bash
./tests/run_tests.sh
```

### Run specific test file:

```bash
./tests/run_tests.sh tests/unit/daemon_commands.bats
```

### Run all unit tests:

```bash
./tests/run_tests.sh tests/unit/
```

## Writing Tests

### Basic Test Structure

```bash
#!/usr/bin/env bats

load "../lib/test_helper.bash"

@test "description of what you're testing" {
  # Setup
  project_dir=$(create_test_project "Test Project")
  cd "$project_dir"

  # Run command
  run run_intent <command> <args>

  # Assert results
  assert_success  # or assert_failure
  assert_output_contains "expected text"
}
```

### Available Helper Functions

- `create_test_project "name" [dir]` - Creates a test Intent project whose `intent_version` is `INTENT_FIXTURE_VERSION`, or `VERSION` when that is unset
- `run_intent <args>` - Runs `$INTENT_BIN` with the arguments
- `setup_fake_home` / `teardown_fake_home` - Points `HOME` at a per-test sandbox under `TEST_TEMP_DIR`, and restores it
- `write_exempt_acceptance "st_dir"` - Writes an `acceptance.md` the close gate treats as exempt
- `assert_success` - Asserts command succeeded (exit 0)
- `assert_failure` - Asserts command failed (exit non-zero)
- `assert_output "text"` - Checks output matches exactly
- `assert_output_contains "text"` - Checks if output contains text
- `refute_output_contains "text"` - Checks output does not contain text
- `assert_file_exists "path"` - Checks if file exists
- `assert_file_not_exists "path"` - Checks a file does not exist
- `assert_directory_exists "path"` - Checks if directory exists
- `assert_file_contains "file" "text"` - Checks if file contains text

## Test Categories

### Unit Tests

Each file under `unit/` names its subject in its header comment; read that rather than a list here, which goes stale the first time a file is added or removed.

## Debugging Tests

To see more output when debugging:

```bash
# Print each `run`'s output
bats --verbose-run tests/unit/pre_commit_hook.bats

# Trace test commands as they execute
bats -x tests/unit/pre_commit_hook.bats

# Run with TAP output
bats -t tests/unit/pre_commit_hook.bats
```

## CI/CD

Tests run automatically via GitHub Actions on:

- Every push to `main`
- Every pull request targeting `main`
- Both Ubuntu and macOS environments

A push or pull request whose changed paths are all under `intent/whiteboard/**` does not trigger the suite. Each leg builds the release binaries before running `tests/run_tests.sh`. See `.github/workflows/tests.yml` for the CI configuration.
