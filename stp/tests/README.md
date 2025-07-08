# STP Test Suite

This directory contains automated tests for the Steel Thread Process (STP) utilities.

## Overview

The STP test suite uses [Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System) to test the functionality of STP scripts and commands.

## Directory Structure

- `/bootstrap`: Tests for the bootstrap script
- `/init`: Tests for the initialization command
- `/st`: Tests for steel thread management
- `/help`: Tests for the help command
- `/main`: Tests for the main stp script
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

The test suite includes a helper script that makes it easy to run tests:

```bash
# Navigate to the tests directory
cd /path/to/STP/stp/tests/

# Run the test script (runs all tests)
./run_tests.sh

# Run only specific tests (e.g., bootstrap tests)
./run_tests.sh bootstrap

# Run a specific test file
./run_tests.sh bootstrap/bootstrap_test.bats
```

Alternatively, you can run the tests directly with Bats:

```bash
# Run all tests from the project root
bats stp/tests/**/*.bats

# Run only bootstrap tests
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

## Interactive Script Testing

For testing scripts that require user input (like confirming operations):

1. Use the `expect` utility to automate interactive testing:

```bash
#!/usr/bin/expect -f
set timeout 5
spawn ./command_to_test arg1 arg2
expect "Prompt message"
send "y\r"
expect eof
```

2. Create the expect script in your test's setup function and call it from your test case.

## Tips for Reliable Testing

1. **Special Characters**: When testing for strings with special characters like asterisks (`*`), 
   use the `-F` flag with grep:
   
   ```bash
   run grep -F "**bold text**" file.md
   ```

2. **Exclude Library Tests**: The test runner excludes tests in the `/lib/` directory to avoid 
   running tests from the Bats libraries themselves.

3. **`.gitignore`**: The test directory includes a `.gitignore` file to exclude the Bats library
   directories and temporary files from source control.