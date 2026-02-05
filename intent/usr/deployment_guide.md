---
verblock: "05 Feb 2026:v0.3: Matthew Sinclair - Updated to Intent v2.3.4"
intent_version: 2.3.4
---
# Deployment Guide

This deployment guide provides instructions for deploying the Intent v2.3.4 system in various environments. It covers installation, configuration, and integration with other tools and workflows.

## Table of Contents

1. [Installation](#installation)
2. [Configuration](#configuration)
3. [Integration](#integration)
4. [Maintenance](#maintenance)
5. [Upgrading](#upgrading)
6. [Troubleshooting](#troubleshooting)
7. [Plugin and Subagent Deployment](#plugin-and-subagent-deployment)
8. [Treeindex Integration](#treeindex-integration)

## Installation

### System Requirements

- POSIX-compatible shell environment (bash, zsh)
- Git (optional, for version control)
- Text editor with markdown support
- Backlog.md (for task management integration)

### Installation Methods

#### Global Installation

Install Intent globally to make it available for all projects:

```bash
# Clone the Intent repository
git clone https://github.com/matthewsinclair/intent.git ~/intent

# Add Intent bin directory to PATH in shell profile
echo 'export INTENT_HOME=~/intent' >> ~/.bashrc
echo 'export PATH=$PATH:$INTENT_HOME/bin' >> ~/.bashrc

# Reload shell configuration
source ~/.bashrc
```

#### Project-Specific Installation

Install Intent within a specific project:

```bash
# From your project directory
git clone https://github.com/matthewsinclair/intent.git .intent

# Create a local alias for the project
alias intent='./.intent/bin/intent'
```

#### Installation Verification

Verify the installation:

```bash
intent help
```

This should display the help information for Intent commands.

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
# Initialize Backlog with Intent-friendly settings
intent bl init
```

## Configuration

### Environment Variables

Configure Intent behavior using these environment variables:

| Variable      | Purpose                        | Default                           |
|---------------|--------------------------------|-----------------------------------|
| INTENT_HOME   | Location of Intent installation| Path to cloned repository         |
| INTENT_PROJECT| Current project name           | Determined from initialization    |
| INTENT_AUTHOR | Default author name            | Determined from git configuration |
| INTENT_EDITOR | Preferred text editor          | Determined from system defaults   |

Example configuration in `.bashrc` or `.zshrc`:

```bash
export INTENT_HOME=~/intent
export INTENT_AUTHOR="Jane Doe"
export INTENT_EDITOR="vim"
```

### Project Configuration

Create a project-specific configuration using `.intent/config.json`:

```json
{
  "project_name": "Project Name",
  "author": "Default Author",
  "st_prefix": "ST"
}
```

## Integration

### Version Control Integration

Intent works seamlessly with git and other version control systems:

#### Recommended .gitignore

```
# Intent temporary files
.intent-tmp/

# Intent configuration (contains local paths)
.intent/config.json

# Backlog configuration
backlog/config.yml
backlog/.git/
```

#### Commit Practices

- Commit steel thread documents along with code changes
- Use steel thread IDs in commit messages for traceability

#### Branch Strategy

- Create feature branches based on steel threads
- Name branches using steel thread IDs (eg `feature/ST0001`)

### CI/CD Integration

To integrate Intent with CI/CD pipelines:

1. Include the Intent test suite in your CI pipeline:

```yaml
# Example GitHub Actions workflow
name: Intent Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Set up test environment
        run: ./tests/setup_test_env.sh
      - name: Run tests
        run: ./tests/run_tests.sh
```

2. Configure notifications for test failures
3. Add documentation generation steps if needed

### IDE Integration

#### VS Code Integration

1. Install the "Bash Debug" extension for debugging Intent scripts
2. Configure `.vscode/tasks.json` for common Intent tasks:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run Intent Tests",
      "type": "shell",
      "command": "${workspaceFolder}/tests/run_tests.sh",
      "group": {
        "kind": "test",
        "isDefault": true
      }
    }
  ]
}
```

#### JetBrains IDE Integration

1. Configure run configurations for Intent commands
2. Set up file watchers for markdown linting
3. Add shell script run configurations for tests

### LLM Platform Integration

#### Claude Code Integration

To integrate Intent with Claude Code:

1. Share the `intent/llm/llm_preamble.md` at the beginning of each session
2. Keep relevant steel thread documents in the context window
3. Use structured templates for consistent information sharing

Example Claude Code command:

```bash
claude code --context intent/llm/llm_preamble.md --context intent/st/ST0001/info.md
```

#### Other LLM Integration

For other LLM platforms:

1. Create platform-specific scripts to extract and format Intent context
2. Maintain a consistent formatting pattern when sharing information
3. Consider implementing automatic context extraction helpers

## Maintenance

### Regular Maintenance Tasks

- Update Intent installation periodically
- Review and clean up completed steel threads
- Archive older project documents
- Sync steel thread status with Backlog tasks
- Archive completed tasks in Backlog
- Run `intent doctor` to check configuration health

### Backup Practices

- Include Intent documents in regular backups
- Ensure documentation is committed to version control
- Back up Backlog task data (backlog/tasks/, backlog/archive/)
- Export task data periodically:

  ```bash
  # Export all tasks to JSON
  backlog task list --export > backlog-export-$(date +%Y%m%d).json
  ```

## Upgrading

### Upgrading Intent Installation

To upgrade a global Intent installation:

```bash
cd $INTENT_HOME
git pull
```

To upgrade a project-specific installation:

```bash
cd my-project/.intent
git pull
```

### Migrating Between Versions

When upgrading Intent with Backlog integration:

1. **Backup existing data**:

   ```bash
   # Backup steel threads
   cp -r intent/st intent/st.backup
   
   # Backup Backlog data
   cp -r backlog backlog.backup
   ```

2. **Run upgrade command**:

   ```bash
   intent upgrade
   ```

3. **Migrate embedded tasks** (if upgrading from pre-Backlog version):

   ```bash
   # Migrate all active steel threads
   intent migrate --all-active
   ```

4. **Verify integration**:

   ```bash
   # Check task status
   intent status report
   
   # Verify tasks in Backlog
   intent bl list
   ```

## Test Suite Deployment

The Intent test suite uses Bats (Bash Automated Testing System) and requires proper setup:

### Test Dependencies

The test suite requires the following dependencies:

- Bats: Core testing framework
- bats-support: Support library for better test output
- bats-assert: Assertion library for test validation
- bats-file: File-related assertions

### Setting Up the Test Environment

Run the setup script to install all dependencies:

```bash
cd tests/
./setup_test_env.sh
```

This script will:

1. Check for existing Bats installation
2. Install Bats if needed
3. Install required Bats libraries
4. Configure the test environment

### Test Suite Configuration

The test suite can be configured through environment variables:

| Variable         | Purpose                            | Default                      |
|------------------|------------------------------------|------------------------------|
| BATS_LIB_PATH    | Location of Bats libraries         | tests/lib                    |
| INTENT_TEST_TEMP | Temporary directory for test files | /tmp/intent-test-XXXXXX      |
| INTENT_BIN_PATH  | Path to Intent executables         | Determined from current path |

### Running Tests in Different Environments

```bash
# Set custom paths for testing
export INTENT_BIN_PATH=/custom/path/to/intent/bin
export BATS_LIB_PATH=/custom/path/to/bats/libs

# Run tests with custom configuration
./tests/run_tests.sh
```

## Troubleshooting

### Common Issues

#### Backlog Git Fetch Errors

If you encounter git fetch errors with Backlog:

```bash
# Use the Intent wrapper instead of direct backlog commands
intent bl list  # Instead of: backlog task list

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
cd tests/
./setup_test_env.sh
```

#### Test Failures

For test failures:

1. Check the test output for specific errors
2. Verify the Intent installation is correct
3. Ensure all paths are correctly configured
4. Check for permission issues on script files

#### Permission Errors

If you encounter permission errors:

```bash
# Make scripts executable
chmod +x bin/*
chmod +x tests/*.sh
```

#### Task Synchronization Issues

If tasks aren't syncing properly with steel threads:

```bash
# Check task naming convention (should be "ST#### - Description")
intent bl list | grep "ST[0-9]"

# Manually sync a specific steel thread
intent status sync ST0001

# Force sync all active threads
for st in $(intent st list --status "In Progress" | awk '{print $1}' | grep "^ST"); do
  intent status sync "$st"
done
```

### Diagnostic Tools

Intent provides several diagnostic tools:

- `intent help`: Verify command availability
- `intent doctor`: Check configuration and environment health
- `run_tests.sh`: Run tests to verify functionality
- Test failure output: Contains detailed error information

To debug test failures, examine the test output and check the corresponding script functionality.

### Getting Help

If you encounter issues:

1. Check the troubleshooting section in this guide
2. Review the test output for specific errors
3. Consult the Intent documentation
4. Submit issues to the Intent project repository
5. Refer to the Bats documentation for test-specific problems

## Plugin and Subagent Deployment

### Plugin Architecture

Intent v2.3.0 introduced a plugin architecture. Plugins live in `intent/plugins/` and extend Intent's functionality:

```
intent/plugins/
├── agents/              # AGENTS.md management plugin
│   ├── bin/
│   │   └── intent_agents
│   └── templates/
│       └── default.md
└── claude/              # Claude Code integration plugin
    ├── bin/
    │   └── intent_claude_subagents
    └── subagents/
        ├── intent/
        ├── elixir/
        ├── socrates/
        └── worker-bee/
```

### Deploying AGENTS.md

AGENTS.md provides universal AI agent instructions that work across platforms:

```bash
# Initialize AGENTS.md in a project
intent agents init

# Validate the AGENTS.md structure
intent agents validate

# Keep AGENTS.md in sync with project changes
intent agents sync
```

The `init` command creates `intent/llm/AGENTS.md` and a symlink at the project root.

### Deploying Claude Subagents

Claude subagents install to `~/.claude/agents/` for global availability:

```bash
# Initialize subagent configuration
intent claude subagents init

# Install subagents
intent claude subagents install --all

# Verify installation
intent claude subagents status
```

For team deployments, each developer runs `intent claude subagents install` after cloning the project.

### Subagent Maintenance

```bash
# Check for modifications or version drift
intent claude subagents status --verbose

# Sync with latest versions from the project
intent claude subagents sync

# Force reinstall if needed
intent claude subagents uninstall --all
intent claude subagents install --all
```

## Treeindex Integration

### Setting Up Treeindex

Treeindex generates LLM-oriented directory summaries. It requires Claude CLI for AI-powered summarization.

```bash
# Generate summaries for key directories
intent treeindex lib
intent treeindex src

# Set custom depth
intent treeindex --depth 3 lib
```

### CI/CD Integration for Treeindex

Use `--check` mode to verify summaries are up to date in CI:

```bash
# Check if summaries are stale (non-zero exit = stale)
intent treeindex --check lib
```

### Treeindex Maintenance

```bash
# Remove orphaned shadow entries
intent treeindex --prune lib

# Force regeneration
intent treeindex --force lib

# Preview without writing
intent treeindex --dry-run lib
```

### Configuring Treeindex Exclusions

Edit `intent/.treeindex/.treeindexignore` to exclude files or directories from indexing. The format follows `.gitignore` conventions.
