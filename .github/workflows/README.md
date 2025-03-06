# GitHub Actions Workflows

## STP Tests Workflow

The `tests.yml` workflow automates testing for the STP project across different operating systems.

### Workflow Configuration

- **Trigger**: Runs on pushes and pull requests to the `main` branch
- **Test Environments**: 
  - Ubuntu Linux (Latest)
  - macOS (Latest)

### Workflow Steps

For each environment, the workflow:

1. **Checks out code**: Fetches the full repository history
2. **Sets up test environment**: Runs the setup script to install dependencies
3. **Installs Bats**: Installs the Bats testing framework 
   - Ubuntu: Uses APT package manager
   - macOS: Uses Homebrew
4. **Runs tests**: Executes the full test suite

### Modifying the Workflow

To modify the workflow:

1. Edit `.github/workflows/tests.yml`
2. Test changes locally if possible
3. Commit and push to see results in GitHub Actions

### Local Testing

You can simulate this workflow locally by running:

```bash
cd stp/tests
./setup_test_env.sh
./run_tests.sh
```