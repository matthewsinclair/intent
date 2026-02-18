# Intent Test Suite

## Overview

This directory contains the test suite for Intent v2.4.0. The tests are written using [Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System).

## Directory Structure

```
tests/
├── unit/                          # Unit tests for individual commands
│   ├── agent_commands.bats        # AGENTS.md management tests
│   ├── basic.bats                 # Basic infrastructure tests
│   ├── bl_commands.bats           # Backlog wrapper command tests
│   ├── bootstrap.bats             # Bootstrap command tests
│   ├── config.bats                # Configuration and PROJECT_ROOT tests
│   ├── fileindex_commands.bats    # Fileindex command tests
│   ├── global_commands.bats       # Tests for global commands
│   ├── help_commands.bats         # Help system tests
│   ├── init_commands.bats         # Init command tests
│   ├── migration.bats             # Migration and backup tests
│   ├── project_commands.bats      # Tests for project-specific commands
│   ├── skills_commands.bats       # Skills management command tests
│   ├── st_commands.bats           # Steel thread command tests
│   ├── task_commands.bats         # Task management command tests
│   └── treeindex_commands.bats    # Treeindex command tests
├── integration/                   # Integration tests
│   └── end_to_end.bats            # Full workflow tests
├── fixtures/                      # Test fixtures (sample files, etc.)
├── lib/                           # Test libraries
│   └── test_helper.bash           # Common test functions
├── run_tests.sh                   # Main test runner
└── README.md                      # This file
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

- **agent_commands.bats** - Tests for AGENTS.md management (`intent agents init/generate/sync/validate`)
- **basic.bats** - Tests for basic infrastructure and environment setup
- **bl_commands.bats** - Tests for the Backlog wrapper (`intent bl`)
- **bootstrap.bats** - Tests for the bootstrap command
- **config.bats** - Tests configuration loading and PROJECT_ROOT detection
- **fileindex_commands.bats** - Tests for the fileindex command (file tracking and checkbox states)
- **global_commands.bats** - Tests commands that work without a project (help, doctor, info, etc.)
- **help_commands.bats** - Tests for the help system
- **init_commands.bats** - Tests for the init command
- **migration.bats** - Tests backup creation and version migration
- **project_commands.bats** - Tests commands that require a project context
- **st_commands.bats** - Tests for steel thread management commands
- **task_commands.bats** - Tests for task management commands
- **skills_commands.bats** - Tests for skills management (`intent claude skills install/list/sync/uninstall/show`)
- **treeindex_commands.bats** - Tests for the treeindex command (directory summaries)

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

Tests run automatically via GitHub Actions on:

- Every push to `main`
- Every pull request targeting `main`
- Both Ubuntu and macOS environments

See `.github/workflows/tests.yml` for the CI configuration.
