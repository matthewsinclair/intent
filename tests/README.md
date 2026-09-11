# Intent Test Suite

## Overview

This directory contains the test suite for Intent. The tests are written using [Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System).

## Directory Structure

```
tests/
├── unit/                          # One .bats file per subject
├── conformance/                   # BASELINE.md: the recorded v2-conformance baseline
├── fixtures/                      # Test fixtures (sample files, etc.)
├── lib/                           # Test libraries
│   └── test_helper.bash           # Common test functions
├── run_tests.sh                   # Main test runner
└── README.md                      # This file
```

The suite drives the v3 binary at `native/rust/target/release/intent`, so build it first (`cargo build --release` under `native/rust`), or set `INTENT_BIN` to another binary. The v2 shell implementation and the bats files that tested its commands were removed at the 3.0.1 cut.

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

- `create_test_project "name"` - Creates a test Intent project
- `run_intent <args>` - Runs the intent command
- `assert_success` - Asserts command succeeded (exit 0)
- `assert_failure` - Asserts command failed (exit non-zero)
- `assert_output_contains "text"` - Checks if output contains text
- `assert_file_exists "path"` - Checks if file exists
- `assert_directory_exists "path"` - Checks if directory exists
- `assert_file_contains "file" "text"` - Checks if file contains text

## Test Categories

### Unit Tests

Each file under `unit/` names its subject in its header comment; read that rather than a list here, which goes stale the first time a file is added or removed.

## Key Test Scenarios

1. **Global vs Project Commands**
   - Global commands work anywhere
   - Project commands show helpful error outside projects

2. **Configuration**
   - PROJECT_ROOT detected from subdirectories
   - Config files loaded correctly
   - Legacy projects detected

3. **Error Handling**
   - No silent failures
   - Clear error messages
   - Helpful suggestions

4. **Migration**
   - Backup directories use `.backup_*` prefix
   - Version fields use `intent_version`
   - Legacy projects can be upgraded

## Debugging Tests

To see more output when debugging:

```bash
# Run with verbose output
bats -v tests/unit/config.bats

# Run with tap output
bats -t tests/unit/config.bats
```

## CI/CD

Tests run automatically via GitHub Actions on:

- Every push to `main`
- Every pull request targeting `main`
- Both Ubuntu and macOS environments

See `.github/workflows/tests.yml` for the CI configuration.
