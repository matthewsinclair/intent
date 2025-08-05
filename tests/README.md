# Intent Test Suite

## Overview

This directory contains the test suite for Intent v2.2.0. The tests are written using [Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System).

## Directory Structure

```
tests/
├── unit/              # Unit tests for individual commands
│   ├── basic.bats     # Basic infrastructure tests
│   ├── config.bats    # Configuration and PROJECT_ROOT tests
│   ├── global_commands.bats  # Tests for global commands
│   ├── migration.bats # Migration and backup tests
│   └── project_commands.bats # Tests for project-specific commands
├── integration/       # Integration tests
│   └── end_to_end.bats # Full workflow tests
├── fixtures/          # Test fixtures (sample files, etc.)
├── lib/               # Test libraries
│   └── test_helper.bash # Common test functions
├── run_tests.sh       # Main test runner
└── README.md          # This file
```

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
./tests/run_tests.sh tests/unit/global_commands.bats
```

### Run all unit tests:
```bash
./tests/run_tests.sh tests/unit/
```

### Run all integration tests:
```bash
./tests/run_tests.sh tests/integration/
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

Unit tests focus on individual commands and features:

- **global_commands.bats** - Tests commands that work without a project (help, doctor, info, etc.)
- **project_commands.bats** - Tests commands that require a project context (st, bl, task, etc.)
- **config.bats** - Tests configuration loading and PROJECT_ROOT detection
- **migration.bats** - Tests backup creation and version migration

### Integration Tests

Integration tests verify complete workflows:

- **end_to_end.bats** - Tests full user workflows like creating a project and managing steel threads

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

Tests should be run on:
- Every push to main
- Every pull request
- Multiple OS versions (macOS, Linux)

See `.github/workflows/test.yml` for CI configuration (TODO).