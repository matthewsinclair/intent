---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.0.0
---
# Deployment Guide

This deployment guide provides instructions for deploying the Steel Thread Project (STP) system in various environments. It covers installation, configuration, and integration with other tools and workflows.

## Table of Contents

1. [Installation](#installation)
2. [Configuration](#configuration)
3. [Integration](#integration)
4. [Maintenance](#maintenance)
5. [Upgrading](#upgrading)
6. [Troubleshooting](#troubleshooting)

## Installation

### System Requirements

- POSIX-compatible shell environment (bash, zsh)
- Git (optional, for version control)
- Text editor with markdown support
- Backlog.md (for task management integration)

### Installation Methods

#### Global Installation

Install STP globally to make it available for all projects:

```bash
# Clone the STP repository
git clone https://github.com/username/stp.git ~/stp

# Add STP bin directory to PATH in shell profile
echo 'export STP_HOME=~/stp' >> ~/.bashrc
echo 'export PATH=$PATH:$STP_HOME/bin' >> ~/.bashrc

# Reload shell configuration
source ~/.bashrc
```

#### Project-Specific Installation

Install STP within a specific project:

```bash
# From your project directory
git clone https://github.com/username/stp.git .stp

# Create a local alias for the project
alias stp='./.stp/bin/stp'
```

#### Installation Verification

Verify the installation:

```bash
stp help
```

This should display the help information for STP commands.

#### Installing Backlog.md

Install Backlog.md for task management:

```bash
# Install Backlog globally
npm install -g backlog.md

# Or install locally in your project
npm install backlog.md

# Verify installation
backlog --version
```

Initialize Backlog in your project:

```bash
# Initialize Backlog with STP-friendly settings
stp bl init
```

## Configuration

### Environment Variables

Configure STP behavior using these environment variables:

| Variable    | Purpose                      | Default                           |
|-------------|------------------------------|-----------------------------------|
| STP_HOME    | Location of STP installation | Path to cloned repository         |
| STP_PROJECT | Current project name         | Determined from initialization    |
| STP_AUTHOR  | Default author name          | Determined from git configuration |
| STP_EDITOR  | Preferred text editor        | Determined from system defaults   |

Example configuration in `.bashrc` or `.zshrc`:

```bash
export STP_HOME=~/stp
export STP_AUTHOR="Jane Doe"
export STP_EDITOR="vim"
```

### Project Configuration

Create a project-specific configuration using `.stp-config`:

```ini
# STP Project Configuration
PROJECT_NAME="Project Name"
AUTHOR="Default Author"
ST_PREFIX="ST"
```

## Integration

### Version Control Integration

STP works seamlessly with git and other version control systems:

#### Recommended .gitignore

```
# STP temporary files
.stp-tmp/

# Backlog configuration
backlog/config.yml
backlog/.git/
```

#### Commit Practices

- Commit steel thread documents along with code changes
- Use steel thread IDs in commit messages for traceability

#### Branch Strategy

- Create feature branches based on steel threads
- Name branches using steel thread IDs (e.g., `feature/ST0001`)

### CI/CD Integration

To integrate STP with CI/CD pipelines:

1. Include the STP test suite in your CI pipeline:

```yaml
# Example GitHub Actions workflow
name: STP Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Set up test environment
        run: ./stp/tests/setup_test_env.sh
      - name: Run tests
        run: cd stp/tests && ./run_tests.sh
```

2. Configure notifications for test failures
3. Add documentation generation steps if needed

### IDE Integration

#### VS Code Integration

1. Install the "Bash Debug" extension for debugging STP scripts
2. Configure `.vscode/tasks.json` for common STP tasks:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run STP Tests",
      "type": "shell",
      "command": "cd ${workspaceFolder}/stp/tests && ./run_tests.sh",
      "group": {
        "kind": "test",
        "isDefault": true
      }
    }
  ]
}
```

#### JetBrains IDE Integration

1. Configure run configurations for STP commands
2. Set up file watchers for markdown linting
3. Add shell script run configurations for tests

### LLM Platform Integration

#### Claude Code Integration

To integrate STP with Claude Code:

1. Share the `stp/llm/llm_preamble.md` at the beginning of each session
2. Keep relevant steel thread documents in the context window
3. Use structured templates for consistent information sharing

Example Claude Code command:

```bash
claude code --context stp/llm/llm_preamble.md --context stp/prj/st/ST0001.md
```

#### Other LLM Integration

For other LLM platforms:

1. Create platform-specific scripts to extract and format STP context
2. Maintain a consistent formatting pattern when sharing information
3. Consider implementing automatic context extraction helpers

## Maintenance

### Regular Maintenance Tasks

- Update STP installation periodically
- Review and clean up completed steel threads
- Archive older project documents
- Sync steel thread status with Backlog tasks
- Archive completed tasks in Backlog

### Backup Practices

- Include STP documents in regular backups
- Ensure documentation is committed to version control
- Back up Backlog task data (backlog/tasks/, backlog/archive/)
- Export task data periodically:
  ```bash
  # Export all tasks to JSON
  backlog task list --export > backlog-export-$(date +%Y%m%d).json
  ```

## Upgrading

### Upgrading STP Installation

To upgrade a global STP installation:

```bash
cd $STP_HOME
git pull
```

To upgrade a project-specific installation:

```bash
cd my-project/.stp
git pull
```

### Migrating Between Versions

When upgrading STP with Backlog integration:

1. **Backup existing data**:
   ```bash
   # Backup steel threads
   cp -r stp/prj/st stp/prj/st.backup
   
   # Backup Backlog data
   cp -r backlog backlog.backup
   ```

2. **Run upgrade command**:
   ```bash
   stp upgrade
   ```

3. **Migrate embedded tasks** (if upgrading from pre-Backlog version):
   ```bash
   # Migrate all active steel threads
   stp migrate --all-active
   ```

4. **Verify integration**:
   ```bash
   # Check task status
   stp status report
   
   # Verify tasks in Backlog
   stp bl list
   ```

## Test Suite Deployment

The STP test suite uses Bats (Bash Automated Testing System) and requires proper setup:

### Test Dependencies

The test suite requires the following dependencies:

- Bats: Core testing framework
- bats-support: Support library for better test output
- bats-assert: Assertion library for test validation
- bats-file: File-related assertions

### Setting Up the Test Environment

Run the setup script to install all dependencies:

```bash
cd stp/tests/
./setup_test_env.sh
```

This script will:

1. Check for existing Bats installation
2. Install Bats if needed
3. Install required Bats libraries
4. Configure the test environment

### Test Suite Configuration

The test suite can be configured through environment variables:

| Variable        | Purpose                             | Default                       |
|-----------------|-------------------------------------|-------------------------------|
| BATS_LIB_PATH   | Location of Bats libraries          | stp/tests/lib                 |
| STP_TEST_TEMP   | Temporary directory for test files  | /tmp/stp-test-XXXXXX          |
| STP_BIN_PATH    | Path to STP executables             | Determined from current path  |

### Running Tests in Different Environments

```bash
# Set custom paths for testing
export STP_BIN_PATH=/custom/path/to/stp/bin
export BATS_LIB_PATH=/custom/path/to/bats/libs

# Run tests with custom configuration
cd stp/tests/
./run_tests.sh
```

## Troubleshooting

### Common Issues

#### Backlog Git Fetch Errors

If you encounter git fetch errors with Backlog:

```bash
# Use the STP wrapper instead of direct backlog commands
stp bl list  # Instead of: backlog task list

# Verify remote operations are disabled
backlog config get remoteOperations
# Should return: false

# If not disabled, fix it:
backlog config set remoteOperations false
```

#### Missing Test Dependencies

If test dependencies are missing:

```bash
# Re-run the setup script
cd stp/tests/
./setup_test_env.sh
```

#### Test Failures

For test failures:

1. Check the test output for specific errors
2. Verify the STP installation is correct
3. Ensure all paths are correctly configured
4. Check for permission issues on script files

#### Permission Errors

If you encounter permission errors:

```bash
# Make scripts executable
chmod +x stp/bin/*
chmod +x stp/tests/*.sh
chmod +x stp/tests/lib/*/src/*.bash
```

#### Task Synchronization Issues

If tasks aren't syncing properly with steel threads:

```bash
# Check task naming convention (should be "ST#### - Description")
stp bl list | grep "ST[0-9]"

# Manually sync a specific steel thread
stp status sync ST0001

# Force sync all active threads
for st in $(stp st list --status "In Progress" | awk '{print $1}' | grep "^ST"); do
  stp status sync "$st"
done
```

### Diagnostic Tools

STP provides several diagnostic tools:

- `stp help`: Verify command availability
- `run_tests.sh`: Run tests to verify functionality
- Test failure output: Contains detailed error information

To debug test failures, examine the test output and check the corresponding script functionality.

### Getting Help

If you encounter issues:

1. Check the troubleshooting section in this guide
2. Review the test output for specific errors
3. Consult the STP documentation
4. Submit issues to the STP project repository
5. Refer to the Bats documentation for test-specific problems
