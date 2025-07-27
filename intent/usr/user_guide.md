---
verblock: "27 Jul 2025:v2.1.0: Matthew Sinclair - Updated to Intent v2.1.0"
intent_version: 2.1.0
---
# User Guide

This user guide provides task-oriented instructions for using the Intent system. It explains how to accomplish common tasks and provides workflow guidance.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Working with Steel Threads](#working-with-steel-threads)
5. [Working with Backlog](#working-with-backlog)
6. [Documentation Management](#documentation-management)
7. [LLM Collaboration](#llm-collaboration)
8. [Agent Management](#agent-management)
9. [Testing](#testing)
10. [Troubleshooting](#troubleshooting)

## Introduction

Intent is a system designed to create a structured workflow and documentation process for developers working collaboratively with Large Language Models (LLMs). Intent provides templates, scripts, and process guidelines to enhance productivity while ensuring high-quality documentation as a byproduct of the development process.

### Purpose

Intent helps developers:

- Organize and track development work
- Create and maintain project documentation
- Collaborate effectively with LLMs
- Preserve context across development sessions

### Core Concepts

- **Steel Thread**: A self-contained unit of work focusing on a specific piece of functionality, organized as a directory with structured documentation files
- **Documentation Structure**: Organized markdown files capturing project information
- **LLM Collaboration**: Patterns for effective work with language models

## Installation

### Prerequisites

- POSIX-compatible shell (bash, zsh)
- Git (optional, for version control)
- Text editor with markdown support
- Backlog.md (for task management integration)

### Installation Steps

1. **Global Installation**:

   ```bash
   # Clone the Intent repository
   git clone https://github.com/matthewsinclair/intent.git ~/intent

   # Add Intent bin directory to PATH
   echo 'export INTENT_HOME=~/intent' >> ~/.bashrc
   echo 'export PATH=$PATH:$INTENT_HOME/bin' >> ~/.bashrc

   # Reload shell configuration
   source ~/.bashrc
   ```

2. **Project-Specific Installation**:

   ```bash
   # From your project directory
   git clone https://github.com/matthewsinclair/intent.git .intent

   # Create a local alias for the project
   alias intent='./.intent/bin/intent'
   ```

## Getting Started

### Initializing a Project

To set up Intent in a new or existing project:

```bash
# Navigate to project directory
cd my-project

# Initialize Intent with default directories (eng, llm, st, usr)
intent init "Project Name"

# Or specify which directories to include
intent init --dirs "eng,llm,st,usr" "Project Name"

# Or include all directories (including bin, _templ, tests) 
intent init --all "Project Name"

# Initialize Backlog for task management
intent bl init
```

This creates the Intent directory structure with template documents and sets up Backlog for task management.

### Directory Structure

After initialization with the default directories, you'll have this structure:

```
my-project/
├── intent/                 # Project documentation
│   ├── st/                 # Steel threads (organized as directories)
│   │   └── ST0001/         # Example steel thread directory
│   │       ├── info.md     # Steel thread metadata
│   │       ├── design.md   # Design documentation
│   │       ├── impl.md     # Implementation details
│   │       └── tasks.md    # Task breakdown
│   ├── wip.md              # Work in progress
│   ├── eng/                # Engineering docs
│   │   └── tpd/            # Technical Product Design
│   ├── usr/                # User documentation
│   └── llm/                # LLM-specific content
├── .intent/                # Configuration
│   └── config.json         # Intent configuration
└── backlog/                # Backlog.md task management
    ├── tasks/              # Active tasks
    ├── drafts/             # Draft tasks
    └── config.yml          # Backlog configuration
```

If you use the `--all` option or include specific directories with `--dirs`, additional directories may be included:

```
my-project/
└── intent/
    ├── bin/                # Intent scripts (only with --all or --dirs "bin")
    ├── _templ/             # Templates (only with --all or --dirs "_templ")
    └── tests/              # Tests (only with --all or --dirs "tests")
```

Note: Even when not copying bin files to the new project, Intent commands will still work because they execute from the centrally installed location.

## Working with Steel Threads

### Creating a Steel Thread

To create a new steel thread:

```bash
intent st new "Implement Feature X"
```

This creates a new steel thread directory (e.g., `intent/st/ST0001/`) with an `info.md` file containing metadata.

### Viewing Steel Threads

To list all steel threads:

```bash
# Basic list of all steel threads
intent st list

# Filter by status
intent st list --status "In Progress"

# Adjust table width (useful for wide terminals)
intent st list --width 120
```

To view a specific steel thread:

```bash
intent st show ST0001
```

To edit a steel thread in your default editor:

```bash
intent st edit ST0001
```

### Synchronizing Steel Threads

To update the steel threads index file with information from individual ST directories:

```bash
# Preview changes without writing to file
intent st sync

# Write changes to steel_threads.md
intent st sync --write

# Adjust output width
intent st sync --write --width 120
```

### Completing a Steel Thread

When all tasks in a steel thread are done:

```bash
intent st done ST0001
```

This updates the status and completion date.

## Working with Backlog

Intent integrates with Backlog.md for fine-grained task management. The `intent bl` wrapper provides a streamlined interface that avoids common issues like git fetch errors.

### Initializing Backlog

To set up Backlog in your project:

```bash
# Initialize Backlog with Intent-friendly settings
intent bl init
```

This configures Backlog for local use, disabling remote operations that can cause errors.

### Creating Tasks

Tasks are linked to steel threads for traceability:

```bash
# Create a task linked to a steel thread
intent bl create ST0001 "Implement user authentication"

# Or use the task command
intent task create ST0001 "Add password validation"
```

### Listing Tasks

View all tasks or filter by steel thread:

```bash
# List all tasks (without git errors)
intent bl list

# List tasks for a specific steel thread
intent task list ST0001

# View tasks in Kanban board
intent bl board
```

### Managing Task Status

Update task status as work progresses:

```bash
# Edit a task
intent bl task edit task-5 --status "In Progress"

# Mark a task as done
intent bl task edit task-5 --status Done
```

### Synchronizing Status

Keep steel thread status in sync with task completion:

```bash
# View status summary
intent status show ST0001

# Sync steel thread status based on tasks
intent status sync ST0001

# Generate status report for all active threads
intent status report
```

### Migrating Existing Tasks

If you have embedded tasks in steel threads, migrate them to Backlog:

```bash
# Migrate tasks from a specific steel thread
intent migrate ST0001

# Preview migration without making changes
intent migrate --dry-run ST0001

# Migrate all active steel threads
intent migrate --all-active
```

### Managing Task ID Format

Backlog can use zero-padded task IDs (e.g., task-001 instead of task-1) for better sorting. To retroactively update existing tasks:

```bash
# Pad all tasks to 3 digits
intent bl task pad --all --size 3

# Or pad a specific task
intent bl task pad task-9 --size 3

# Use the configured padding size
intent bl task pad --all
```

After padding tasks, ensure new tasks use the same format:

```bash
intent bl config set zeroPaddedIds 3
```

### Best Practices

1. **Use the wrapper**: Always use `intent bl` instead of `backlog` directly to avoid git errors
2. **Task naming**: Tasks are automatically named with the pattern "ST#### - Description"
3. **Regular syncing**: Run `intent status sync` to keep steel thread status current
4. **Consistent IDs**: Use zero-padded task IDs for better sorting and organization
5. **Task granularity**: Create tasks that can be completed in 1-2 days

## Documentation Management

Intent provides a structured approach to managing project documentation:

### Updating Technical Product Design

The technical product design document is the central reference for the project:

```bash
# Open the TPD document
intent tpd
```

When making significant changes to the project, update the TPD to keep it in sync with the implementation.

### Working with User Documentation

User documentation is maintained in the `intent/usr/` directory:

- `user_guide.md`: Task-oriented instructions for users
- `reference_guide.md`: Comprehensive reference information
- `deployment_guide.md`: Installation and deployment guidance

Update these documents as features are added or changed.

## LLM Collaboration

Intent is designed for effective collaboration with Large Language Models like Claude:

### Using the LLM Preamble

The LLM preamble file contains context that should be shared with LLMs at the beginning of each session:

```bash
# View the LLM preamble
cat intent/llm/llm_preamble.md
```

Include this preamble when starting new sessions with an LLM to provide essential context.

### Understanding Intent Usage Patterns

Intent provides usage rules documentation specifically designed for LLMs:

```bash
# Display usage patterns and workflows for LLMs
intent llm usage_rules

# Create symlink for Elixir projects (or other tools expecting usage-rules.md)
intent llm usage_rules --symlink

# Save to a file for reference
intent llm usage_rules > usage-rules.md
```

This document helps LLMs understand:
- How to use Intent commands effectively
- Common workflows and best practices
- Steel thread management patterns
- Task integration with Backlog.md

### Contextualizing Work with Steel Threads

When working with an LLM on a specific steel thread:

```bash
# Share the steel thread document with the LLM
intent st show ST0001 | [send to LLM]
```

This provides the LLM with task-specific context for more effective collaboration.

## Agent Management

Intent v2.1.0 integrates with Claude Code sub-agents to provide specialized AI assistance that understands Intent methodology and your project conventions.

### What are Intent Agents?

Intent agents are Claude Code sub-agents - specialized AI assistants with focused knowledge:

- **Intent Agent**: Understands steel threads, Intent commands, and project structure
- **Elixir Agent**: Elixir code doctor with Usage Rules and Ash/Phoenix patterns
- **Custom Agents**: Project-specific agents you can create

### Setting Up Agents

#### Initializing Agent Configuration

Before installing agents, you need to initialize the agent configuration:

```bash
# Initialize global agent configuration
intent agents init

# Initialize project-specific agent configuration
intent agents init --project
```

This creates the necessary directories and manifest files for agent management.

#### Installing the Intent Agent

```bash
# Check available agents
intent agents list

# Install the Intent agent (recommended for all projects)
intent agents install intent

# Install all available agents
intent agents install --all
```

#### Verifying Installation

```bash
# Check agent status
intent agents status

# Show agent details
intent agents show intent
```

### Managing Agents

#### Keeping Agents Updated

```bash
# Update agents with latest versions
intent agents sync

# Check for modifications
intent agents status
```

#### Removing Agents

```bash
# Remove specific agent
intent agents uninstall intent

# Remove all Intent-managed agents
intent agents uninstall --all
```

### Using Agents with Claude

Once installed, the Intent agent automatically provides Claude with:

- Complete knowledge of Intent commands and methodology
- Understanding of steel thread structure and workflows
- Best practices for Intent project management
- Backlog.md integration patterns

**Example: Claude with Intent Agent**

```
# Without Intent agent:
You: "Create a new feature for authentication"
Claude: "I'll help create authentication. What's your project structure?"
[You explain Intent, steel threads, etc.]

# With Intent agent:
You: "Create a new feature for authentication"  
Claude: "I'll create a steel thread for authentication:
         
         intent st new 'User Authentication System'
         
         This creates ST0042. Let me help document the intent
         and break it into backlog tasks using Intent methodology..."
```

### Creating Custom Agents

For project-specific conventions, create custom agents:

```bash
# Create project agent directory
mkdir -p intent/agents/myproject

# Create agent definition
cat > intent/agents/myproject/agent.md << 'EOF'
---
name: myproject
description: Project-specific conventions and patterns
tools: Bash, Read, Write, Edit
---

You understand our specific project conventions:

## Architecture
- API endpoints: /api/v2/{resource}
- Authentication: JWT Bearer tokens
- Database: PostgreSQL with migrations

## Code Standards
- Test coverage: minimum 80%
- Documentation: JSDoc for all public APIs
- Git: conventional commits format
EOF

# Install the custom agent
intent agents install myproject
```

### Agent Integration with Intent Commands

Agents are automatically integrated with Intent's core workflow:

- **intent init**: Detects Claude Code and offers agent installation
- **intent doctor**: Includes agent health checks
- **intent upgrade**: Preserves agent directories during migrations

### Troubleshooting Agents

#### Agent Not Found

```bash
# Check if Claude Code is installed
which claude

# Verify Claude agents directory exists
ls ~/.claude/agents/
```

#### Agent Out of Sync

```bash
# Check for local modifications
intent agents status

# Sync with latest versions (overwrites local changes)
intent agents sync
```

#### Reinstalling Agents

```bash
# Remove and reinstall
intent agents uninstall intent
intent agents install intent
```

## Testing

Intent includes a comprehensive test suite to verify functionality:

### Running Tests

To run the test suite:

```bash
# Run all tests
cd intent/tests/
./run_tests.sh

# Run specific test suite
./run_tests.sh bootstrap
```

### Test Structure

Tests are organized by component:
- `bootstrap_test.bats`: Tests for bootstrap script
- `init_test.bats`: Tests for init command
- `st_test.bats`: Tests for steel thread commands
- `help_test.bats`: Tests for help system
- `main_test.bats`: Tests for main script

## Upgrading Intent

When new versions of Intent are released, you may need to upgrade your existing Intent projects to ensure compatibility with the latest features.

### Running the Upgrade Command

To upgrade all Intent files in your project to the latest format:

```bash
intent upgrade
```

This command:
- Updates metadata in all Intent files
- Adds or updates JSON configuration
- Ensures files follow the current format standards
- Adds section markers for automatic sync

### Forcing Upgrades

For major version differences, the upgrade command will warn you before proceeding. To force the upgrade:

```bash
intent upgrade --force
```

### After Upgrading

After upgrading, it's a good practice to:

1. Review updated files to ensure everything looks correct
2. Run a sync to update the steel threads index:
   ```bash
   intent st sync --write
   ```
3. Commit the changes if you're using version control

## Troubleshooting

### Common Issues

#### Intent Commands Not Found

If Intent commands are not found:

```bash
# Check INTENT_HOME environment variable
echo $INTENT_HOME

# Ensure Intent bin directory is in PATH
echo $PATH | grep intent

# Fix PATH if needed
export PATH=$PATH:$INTENT_HOME/bin
```

#### Permission Issues

If you encounter permission errors:

```bash
# Make scripts executable
chmod +x $INTENT_HOME/bin/*
```

#### Template Generation Errors

If template generation fails, check file permissions and ensure template files exist in the `_templ` directory.

#### Backlog Git Fetch Errors

If you see git fetch errors when using Backlog:

```bash
# Use the Intent wrapper instead
intent bl list  # Instead of: backlog task list

# Ensure remote operations are disabled
backlog config get remoteOperations
# Should return: false
```

#### Task Not Found

If tasks aren't showing up:

```bash
# Check task files exist
ls backlog/tasks/

# Use --plain flag if needed
backlog task list --plain
```
