---
verblock: "09 Jul 2025:v0.3: Matthew Sinclair - Updated llm command with --symlink option"
stp_version: 1.2.0
---
# User Guide

This user guide provides task-oriented instructions for using the Steel Thread Process (STP) system. It explains how to accomplish common tasks and provides workflow guidance.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Working with Steel Threads](#working-with-steel-threads)
5. [Working with Backlog](#working-with-backlog)
6. [Documentation Management](#documentation-management)
7. [LLM Collaboration](#llm-collaboration)
8. [Troubleshooting](#troubleshooting)

## Introduction

Steel Thread Process (STP) is a system designed to create a structured workflow and documentation process for developers working collaboratively with Large Language Models (LLMs). STP provides templates, scripts, and process guidelines to enhance productivity while ensuring high-quality documentation as a byproduct of the development process.

### Purpose

STP helps developers:

- Organize and track development work
- Create and maintain project documentation
- Collaborate effectively with LLMs
- Preserve context across development sessions

### Core Concepts

- **Steel Thread**: A self-contained unit of work focusing on a specific piece of functionality
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
   # Clone the STP repository
   git clone https://github.com/username/stp.git ~/stp

   # Add STP bin directory to PATH
   echo 'export STP_HOME=~/stp' >> ~/.bashrc
   echo 'export PATH=$PATH:$STP_HOME/bin' >> ~/.bashrc

   # Reload shell configuration
   source ~/.bashrc
   ```

2. **Project-Specific Installation**:

   ```bash
   # From your project directory
   git clone https://github.com/username/stp.git .stp

   # Create a local alias for the project
   alias stp='./.stp/bin/stp'
   ```

## Getting Started

### Initializing a Project

To set up STP in a new or existing project:

```bash
# Navigate to project directory
cd my-project

# Initialize STP with default directories (eng, llm, prj, usr)
stp init "Project Name"

# Or specify which directories to include
stp init --dirs "eng,llm,prj,usr" "Project Name"

# Or include all directories (including bin, _templ, tests) 
stp init --all "Project Name"

# Initialize Backlog for task management
stp bl init
```

This creates the STP directory structure with template documents and sets up Backlog for task management.

### Directory Structure

After initialization with the default directories, you'll have this structure:

```
my-project/
├── stp/                    # Project documentation
│   ├── prj/                # Project documentation
│   │   ├── st/             # Steel threads
│   │   └── wip.md          # Work in progress
│   ├── eng/                # Engineering docs
│   │   └── tpd/            # Technical Product Design
│   ├── usr/                # User documentation
│   └── llm/                # LLM-specific content
└── backlog/                # Backlog.md task management
    ├── tasks/              # Active tasks
    ├── drafts/             # Draft tasks
    └── config.yml          # Backlog configuration
```

If you use the `--all` option or include specific directories with `--dirs`, additional directories may be included:

```
my-project/
└── stp/
    ├── bin/                # STP scripts (only with --all or --dirs "bin")
    ├── _templ/             # Templates (only with --all or --dirs "_templ")
    └── tests/              # Tests (only with --all or --dirs "tests")
```

Note: Even when not copying bin files to the new project, STP commands will still work because they execute from the centrally installed location.

## Working with Steel Threads

### Creating a Steel Thread

To create a new steel thread:

```bash
stp st new "Implement Feature X"
```

This creates a new steel thread document (e.g., `stp/prj/st/ST0001.md`) and adds it to the index.

### Viewing Steel Threads

To list all steel threads:

```bash
# Basic list of all steel threads
stp st list

# Filter by status
stp st list --status "In Progress"

# Adjust table width (useful for wide terminals)
stp st list --width 120
```

To view a specific steel thread:

```bash
stp st show ST0001
```

To edit a steel thread in your default editor:

```bash
stp st edit ST0001
```

### Synchronizing Steel Threads

To update the steel threads index file with information from individual ST files:

```bash
# Preview changes without writing to file
stp st sync

# Write changes to steel_threads.md
stp st sync --write

# Adjust output width
stp st sync --write --width 120
```

### Completing a Steel Thread

When all tasks in a steel thread are done:

```bash
stp st done ST0001
```

This updates the status and completion date.

## Working with Backlog

STP integrates with Backlog.md for fine-grained task management. The `stp bl` wrapper provides a streamlined interface that avoids common issues like git fetch errors.

### Initializing Backlog

To set up Backlog in your project:

```bash
# Initialize Backlog with STP-friendly settings
stp bl init
```

This configures Backlog for local use, disabling remote operations that can cause errors.

### Creating Tasks

Tasks are linked to steel threads for traceability:

```bash
# Create a task linked to a steel thread
stp bl create ST0001 "Implement user authentication"

# Or use the task command
stp task create ST0001 "Add password validation"
```

### Listing Tasks

View all tasks or filter by steel thread:

```bash
# List all tasks (without git errors)
stp bl list

# List tasks for a specific steel thread
stp task list ST0001

# View tasks in Kanban board
stp bl board
```

### Managing Task Status

Update task status as work progresses:

```bash
# Edit a task
stp bl task edit task-5 --status "In Progress"

# Mark a task as done
stp bl task edit task-5 --status Done
```

### Synchronizing Status

Keep steel thread status in sync with task completion:

```bash
# View status summary
stp status show ST0001

# Sync steel thread status based on tasks
stp status sync ST0001

# Generate status report for all active threads
stp status report
```

### Migrating Existing Tasks

If you have embedded tasks in steel threads, migrate them to Backlog:

```bash
# Migrate tasks from a specific steel thread
stp migrate ST0001

# Preview migration without making changes
stp migrate --dry-run ST0001

# Migrate all active steel threads
stp migrate --all-active
```

### Best Practices

1. **Use the wrapper**: Always use `stp bl` instead of `backlog` directly to avoid git errors
2. **Task naming**: Tasks are automatically named with the pattern "ST#### - Description"
3. **Regular syncing**: Run `stp status sync` to keep steel thread status current
4. **Task granularity**: Create tasks that can be completed in 1-2 days

## Documentation Management

STP provides a structured approach to managing project documentation:

### Updating Technical Product Design

The technical product design document is the central reference for the project:

```bash
# Open the TPD document
stp tpd
```

When making significant changes to the project, update the TPD to keep it in sync with the implementation.

### Working with User Documentation

User documentation is maintained in the `stp/usr/` directory:

- `user_guide.md`: Task-oriented instructions for users
- `reference_guide.md`: Comprehensive reference information
- `deployment_guide.md`: Installation and deployment guidance

Update these documents as features are added or changed.

## LLM Collaboration

STP is designed for effective collaboration with Large Language Models like Claude:

### Using the LLM Preamble

The LLM preamble file contains context that should be shared with LLMs at the beginning of each session:

```bash
# View the LLM preamble
cat stp/llm/llm_preamble.md
```

Include this preamble when starting new sessions with an LLM to provide essential context.

### Understanding STP Usage Patterns

STP provides usage rules documentation specifically designed for LLMs:

```bash
# Display usage patterns and workflows for LLMs
stp llm usage_rules

# Create symlink for Elixir projects (or other tools expecting usage-rules.md)
stp llm usage_rules --symlink

# Save to a file for reference
stp llm usage_rules > usage-rules.md
```

This document helps LLMs understand:
- How to use STP commands effectively
- Common workflows and best practices
- Steel thread management patterns
- Task integration with Backlog.md

### Contextualizing Work with Steel Threads

When working with an LLM on a specific steel thread:

```bash
# Share the steel thread document with the LLM
stp st show ST0001 | [send to LLM]
```

This provides the LLM with task-specific context for more effective collaboration.

## Testing

STP includes a comprehensive test suite to verify functionality:

### Running Tests

To run the test suite:

```bash
# Run all tests
cd stp/tests/
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

## Upgrading STP

When new versions of STP are released, you may need to upgrade your existing STP projects to ensure compatibility with the latest features.

### Running the Upgrade Command

To upgrade all STP files in your project to the latest format:

```bash
stp upgrade
```

This command:
- Updates metadata in all STP files
- Adds or updates YAML frontmatter
- Ensures files follow the current format standards
- Adds section markers for automatic sync

### Forcing Upgrades

For major version differences, the upgrade command will warn you before proceeding. To force the upgrade:

```bash
stp upgrade --force
```

### After Upgrading

After upgrading, it's a good practice to:

1. Review updated files to ensure everything looks correct
2. Run a sync to update the steel threads index:
   ```bash
   stp st sync --write
   ```
3. Commit the changes if you're using version control

## Troubleshooting

### Common Issues

#### STP Commands Not Found

If STP commands are not found:

```bash
# Check STP_HOME environment variable
echo $STP_HOME

# Ensure STP bin directory is in PATH
echo $PATH | grep stp

# Fix PATH if needed
export PATH=$PATH:$STP_HOME/bin
```

#### Permission Issues

If you encounter permission errors:

```bash
# Make scripts executable
chmod +x $STP_HOME/bin/*
```

#### Template Generation Errors

If template generation fails, check file permissions and ensure template files exist in the `_templ` directory.

#### Backlog Git Fetch Errors

If you see git fetch errors when using Backlog:

```bash
# Use the STP wrapper instead
stp bl list  # Instead of: backlog task list

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
