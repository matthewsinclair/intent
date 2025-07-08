# GitHub Actions Workflows

This directory contains automated workflows that run on GitHub to ensure code quality and test coverage for the STP project.

## Workflows

### 1. STP Tests (`tests.yml`)

**Triggers**: On push to `main` branch and on all pull requests

**What it does**:
- Runs all unit tests on Ubuntu and macOS
- Runs integration tests with Backlog.md
- Performs ShellCheck analysis on all scripts
- Provides a summary of test results

**Jobs**:
- `test-linux`: Runs full test suite on Ubuntu with Node.js for Backlog.md
- `test-macos`: Runs full test suite on macOS with Node.js for Backlog.md
- `shellcheck`: Static analysis of shell scripts (non-blocking)
- `test-summary`: Aggregates results from all test jobs

**Key Features**:
- Tests on both Linux and macOS to ensure cross-platform compatibility
- Installs Backlog.md to test integration features
- Includes both unit tests and integration tests
- ShellCheck provides code quality feedback without blocking

### 2. PR Checks (`pr-checks.yml`)

**Triggers**: On all pull request events (opened, synchronized, reopened)

**What it does**:
- Validates steel thread references in PRs
- Checks for documentation updates when code changes
- Verifies test coverage for new code
- Validates commit message format
- Checks PR size and suggests splitting if too large

**Jobs**:
- `validate-steel-thread`: Looks for ST#### references in PR description
- `check-documentation`: Ensures docs are updated when scripts change
- `test-coverage`: Verifies tests are added/updated with code changes
- `commit-message-check`: Validates commit message length and format
- `pr-size-check`: Warns about large PRs (>1000 lines)

## Status Badge

The test status badge in the README shows the status of the latest test run:

```markdown
[![STP Tests](https://github.com/matthewsinclair/dev-stp/actions/workflows/tests.yml/badge.svg)](https://github.com/matthewsinclair/dev-stp/actions/workflows/tests.yml)
```

## Local Testing

Before pushing, you can run tests locally:

```bash
# Run all tests
cd stp/tests
./run_tests.sh

# Run specific test suite
bats task/task_test.bats

# Run integration tests
bats integration/stp_backlog_integration_test.bats

# Run ShellCheck locally
shellcheck stp/bin/stp*
```

## Workflow Maintenance

### Dependencies
- **GitHub Actions**: Uses `actions/checkout@v4` and `actions/setup-node@v4`
- **Node.js**: Version 20 for Backlog.md compatibility
- **Bats**: Installed via system package manager
- **Bats Libraries**: Cloned from GitHub if not present

### Test Environments
- **Ubuntu**: Latest version with apt package manager
- **macOS**: Latest version with Homebrew
- Both environments test the full suite to ensure compatibility

### Best Practices
- ShellCheck runs are non-blocking to allow gradual improvements
- Tests continue running even if some fail (using `|| true`)
- Integration tests check for Backlog.md availability
- All scripts are made executable before running

## Adding New Tests

When adding new features:

1. **Add unit tests** in the appropriate test directory
2. **Update integration tests** if the feature affects STP-Backlog integration
3. **Run tests locally** before pushing
4. **Reference the steel thread** in your PR description (e.g., "Implements ST0042")
5. **Update documentation** if adding new commands or changing behavior

## Troubleshooting

### Common Issues

1. **Tests pass locally but fail in CI**
   - Check for environment-specific paths
   - Ensure all dependencies are properly installed in the workflow
   - Verify file permissions are set correctly

2. **Integration tests fail**
   - Backlog.md might not be installed correctly
   - Check Node.js version compatibility

3. **ShellCheck warnings**
   - These are non-blocking but should be addressed
   - Run `shellcheck` locally to see specific issues

### Debugging Workflows

- Check the Actions tab in GitHub for detailed logs
- Each step shows its output when expanded
- Failed steps are highlighted in red
- Use `echo` statements for debugging in workflows