---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# User Guide

This user guide provides task-oriented instructions for using the Steel Thread Project (STP) system. It explains how to accomplish common tasks and provides workflow guidance.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Working with Steel Threads](#working-with-steel-threads)
5. [Documentation Management](#documentation-management)
6. [LLM Collaboration](#llm-collaboration)
7. [Troubleshooting](#troubleshooting)

## Introduction

Steel Thread Project (STP) is a system designed to create a structured workflow and documentation process for developers working collaboratively with Large Language Models (LLMs). STP provides templates, scripts, and process guidelines to enhance productivity while ensuring high-quality documentation as a byproduct of the development process.

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
```

This creates the STP directory structure with template documents.

### Directory Structure

After initialization with the default directories, you'll have this structure:

```
my-project/
└── stp/                    # Project documentation
    ├── prj/                # Project documentation
    │   ├── st/             # Steel threads
    │   ├── wip.md          # Work in progress
    │   └── journal.md      # Project journal
    ├── eng/                # Engineering docs
    │   └── tpd/            # Technical Product Design
    ├── usr/                # User documentation
    └── llm/                # LLM-specific content
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
stp st list
```

To view a specific steel thread:

```bash
stp st show ST0001
```

### Completing a Steel Thread

When all tasks in a steel thread are done:

```bash
stp st done ST0001
```

This updates the status and completion date.

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
