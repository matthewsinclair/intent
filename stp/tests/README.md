# STP Test Suite

This directory contains automated tests for the Steel Thread Project (STP) utilities.

## Overview

The STP test suite uses [Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System) to test the functionality of STP scripts and commands.

## Directory Structure

- `/bootstrap`: Tests for the bootstrap script
- `/init`: Tests for the initialization command
- `/st`: Tests for steel thread management
- `/fixtures`: Test fixtures and mock environments
- `/lib`: Test helpers and utility functions

## Setup

### Automated Setup

The easiest way to set up the test environment is to use the provided setup script:

```bash
./setup_test_env.sh
```

This will install Bats and the required libraries. On macOS with Homebrew, it will use `brew install bats-core` for convenience.

### Manual Setup

1. Install Bats:

   **Using Homebrew (macOS):**
   ```bash
   brew install bats-core
   ```

   **From Source:**
   ```bash
   git clone https://github.com/bats-core/bats-core.git
   cd bats-core
   ./install.sh /usr/local
   ```

2. Install Bats libraries:
   ```bash
   mkdir -p stp/tests/lib
   git clone https://github.com/bats-core/bats-support.git stp/tests/lib/bats-support
   git clone https://github.com/bats-core/bats-assert.git stp/tests/lib/bats-assert
   git clone https://github.com/bats-core/bats-file.git stp/tests/lib/bats-file
   ```

## Running Tests

Run all tests:
```bash
bats stp/tests/**/*.bats
```

Run a specific test suite:
```bash
bats stp/tests/bootstrap/*.bats
```

## Writing Tests

Each test file should follow this pattern:

```bash
#!/usr/bin/env bats

load '../lib/test_helper'

setup() {
  # Set up test environment
}

teardown() {
  # Clean up after test
}

@test "Test description" {
  # Test code
  run some_command
  assert_success
  assert_output "Expected output"
}
```

## Test Fixtures

Test fixtures are stored in the `fixtures` directory and provide known states
and environments for tests to run against.

## Test Helper Functions

Common test helper functions are defined in the `lib/test_helper.bash` file.