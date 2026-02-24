# GitHub Actions Workflows

This directory contains automated workflows that run on GitHub to ensure code quality and test coverage for the Intent project.

## Workflows

### 1. Intent Tests (`tests.yml`)

**Triggers**: On push to `main` branch and on all pull requests

**What it does**:
- Runs all unit tests on Ubuntu and macOS
- Performs ShellCheck analysis on all scripts
- Provides a summary of test results

**Jobs**:
- `test-linux` (Test on Ubuntu): Runs full test suite on Ubuntu
- `test-macos` (Test on macOS): Runs full test suite on macOS
- `shellcheck` (Shell Script Analysis): Static analysis of shell scripts (non-blocking)
- `test-summary` (Test Summary): Aggregates results from all test jobs

**Key Features**:
- Tests on both Ubuntu and macOS to ensure cross-platform compatibility
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
[![Intent Tests](https://github.com/matthewsinclair/intent/actions/workflows/tests.yml/badge.svg)](https://github.com/matthewsinclair/intent/actions/workflows/tests.yml)
```

## Local Testing

Before pushing, you can run tests locally:

```bash
# Run all tests
cd tests
./run_tests.sh

# Run specific test file
bats tests/unit/global_commands.bats

# Run integration tests
bats tests/integration/end_to_end.bats

# Run ShellCheck locally
shellcheck bin/intent*
```

## Workflow Maintenance

### Dependencies
- **GitHub Actions**: Uses `actions/checkout@v4`
- **Bats**: Installed via system package manager (Homebrew on macOS, from source on Ubuntu)
- **Bats Libraries**: Cloned from GitHub if not present (bats-support, bats-assert, bats-file)

### Test Environments
- **Ubuntu**: Latest version with apt package manager
- **macOS**: Latest version with Homebrew
- Both environments test the full suite to ensure cross-platform compatibility

### Best Practices
- ShellCheck runs are non-blocking to allow gradual improvements
- Tests continue running even if some fail (using `|| true`)
- Integration tests validate end-to-end functionality
- All scripts are made executable before running

## Adding New Tests

When adding new features:

1. **Add unit tests** in `tests/unit/` as a `.bats` file
2. **Update integration tests** if the feature affects end-to-end workflows
3. **Run tests locally** before pushing
4. **Reference the steel thread** in your PR description (eg "Implements ST0042")
5. **Update documentation** if adding new commands or changing behavior

## Troubleshooting

### Common Issues

1. **Tests pass locally but fail in CI**
   - Check for environment-specific paths
   - Ensure all dependencies are properly installed in the workflow
   - Verify file permissions are set correctly

2. **Integration tests fail**
   - Check for missing dependencies
   - Verify file permissions are set correctly

3. **ShellCheck warnings**
   - These are non-blocking but should be addressed
   - Run `shellcheck` locally to see specific issues

### Debugging Workflows

- Check the Actions tab in GitHub for detailed logs
- Each step shows its output when expanded
- Failed steps are highlighted in red
- Use `echo` statements for debugging in workflows
