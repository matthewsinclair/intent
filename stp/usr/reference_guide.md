---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# Reference Guide

# Reference Guide

This reference guide provides comprehensive information about the Steel Thread Project (STP) system. Unlike the task-oriented User Guide, this reference guide serves as a complete reference for all aspects of the system.

## Table of Contents

1. [Command Reference](#command-reference)
2. [Document Templates](#document-templates)
3. [Directory Structure](#directory-structure)
4. [Configuration Options](#configuration-options)
5. [Best Practices](#best-practices)
6. [Concepts and Terminology](#concepts-and-terminology)

## Command Reference

### Core Commands

#### `stp init`

Initializes a new STP project.

**Usage:**

```bash
stp init <project_name> [directory]
```

**Parameters:**

- `project_name`: Name of the project (required)
- `directory`: Target directory (optional, defaults to current directory)

**Example:**

```bash
stp init "My Project" ./my-project
```

**Output:**

- Creates STP directory structure
- Initializes template documents
- Creates initial configuration

#### `stp st`

Manages steel threads.

**Usage:**

```bash
stp st <command> [options] [arguments]
```

**Subcommands:**

`stp st new`

Creates a new steel thread.

**Usage:**

```bash
stp st new <title>
```

**Parameters:**

- `title`: Title of the steel thread (required)

**Example:**

```bash
stp st new "Implement User Authentication"
```

`stp st done`

Marks a steel thread as complete.

**Usage:**

```bash
stp st done <id>
```

**Parameters:**

- `id`: ID of the steel thread (required)

**Example:**

```bash
stp st done ST0001
```

`stp st list`

Lists all steel threads.

**Usage:**

```bash
stp st list [--status <status>]
```

**Options:**

- `--status`: Filter by status (optional)

**Example:**

```bash
stp st list --status "In Progress"
```

`stp st show`

Shows details of a specific steel thread.

**Usage:**

```bash
stp st show <id>
```

**Parameters:**

- `id`: ID of the steel thread (required)

**Example:**

```bash
stp st show ST0001
```

#### `stp help`

Displays help information.

**Usage:**

```bash
stp help [command]
```

**Parameters:**

- `command`: Command to get help for (optional)

**Example:**

```bash
stp help st
```

### Additional Commands

#### Test Suite Commands

The STP test suite provides commands for verifying system functionality:

```bash
# Run all tests
cd stp/tests/
./run_tests.sh

# Run specific test suite
./run_tests.sh bootstrap
./run_tests.sh init
./run_tests.sh st
./run_tests.sh help
./run_tests.sh main

# Set up test environment
./setup_test_env.sh
```

The test environment setup script installs necessary dependencies, including:
- Bats (Bash Automated Testing System)
- bats-support
- bats-assert
- bats-file

## Document Templates

### Project Templates

#### Work in Progress (WIP) Template

Location: `stp/prj/wip.md`

Purpose: Tracks current development focus and active steel threads.

Structure:

- Current Focus
- Active Steel Threads
- Upcoming Work
- Notes

#### Journal Template

Location: `stp/prj/journal.md`

Purpose: Maintains a chronological record of project activities.

Structure:

- Date entries
- Activity descriptions
- Decisions
- Challenges and resolutions

#### Steel Thread Templates

Location: `stp/prj/st/`

Purpose: Defines and tracks individual units of work.

Structure:

- Metadata (ID, status, dates)
- Objective
- Context
- Approach
- Tasks
- Implementation notes
- Results

### Engineering Templates

Engineering templates are located in `stp/_templ/eng/`:

- `tpd/`: Technical Product Design templates
  - `_technical_product_design.md`: Main TPD template
  - `_1_introduction.md` through `_8_appendices.md`: Section templates

These templates provide structured formats for capturing technical design decisions and architectural information.

### User Documentation Templates

User documentation templates are located in `stp/_templ/usr/`:

- `_user_guide.md`: Template for task-oriented user instructions
- `_reference_guide.md`: Template for comprehensive reference information
- `_deployment_guide.md`: Template for installation and deployment guidance

### LLM Templates

LLM-specific templates are located in `stp/_templ/llm/`:

- `_llm_preamble.md`: Template for creating context preambles for LLM sessions

## Directory Structure

```
STP/
├── stp/                # Main STP directory
│   ├── _templ/         # Templates directory
│   ├── prj/            # Project documentation
│   │   ├── st/         # Steel threads
│   │   ├── wip.md      # Work in progress
│   │   └── journal.md  # Project journal
│   ├── eng/            # Engineering docs
│   │   └── tpd/        # Technical Product Design
│   ├── usr/            # User documentation
│   ├── llm/            # LLM-specific content
│   └── tests/          # Test suite
│       ├── bootstrap/  # Bootstrap tests
│       ├── init/       # Init command tests
│       ├── st/         # Steel thread command tests
│       ├── help/       # Help system tests
│       ├── main/       # Main script tests
│       ├── lib/        # Test helper libraries
│       ├── fixtures/   # Test fixtures
│       └── run_tests.sh # Test runner script
├── bin/                # STP scripts (executable)
```

## Configuration Options

### Environment Variables

| Variable    | Purpose                      | Default                           |
|-------------|------------------------------|-----------------------------------|
| STP_HOME    | Location of STP installation | Path to cloned repository         |
| STP_PROJECT | Current project name         | Determined from initialization    |
| STP_AUTHOR  | Default author name          | Determined from git configuration |
| STP_EDITOR  | Preferred text editor        | Determined from system defaults   |

### Project Configuration

Location: `.stp-config`

Format: INI-style configuration file

Example:

```ini
# STP Project Configuration
PROJECT_NAME="Project Name"
AUTHOR="Default Author"
ST_PREFIX="ST"
```

## Best Practices

### Steel Thread Management

- Keep steel threads focused on discrete pieces of functionality
- Aim for steel threads that can be completed in hours, not days
- Create clear objectives for each steel thread
- Update documentation as work progresses
- Link related steel threads for context

### Documentation Practices

- Use consistent formatting across documents
- Keep the WIP document updated with current focus
- Document decisions and their rationale in the journal
- Use clear, descriptive titles for steel threads
- Maintain cross-references between related documents

### LLM Collaboration

- Share relevant context at the beginning of each session
- Use steel thread documents to maintain context across sessions
- Create canned prompts for common tasks
- Have the LLM update documentation as work progresses
- Provide clear instructions for specific tasks

## Concepts and Terminology

| Term | Definition |
|------|------------|
| Steel Thread | A self-contained unit of work representing a logical piece of functionality |
| LLM | Large Language Model, an AI system capable of understanding and generating text |
| Context Window | The amount of text an LLM can process in a single interaction |
| Canned Prompt | A pre-defined, reusable instruction template for an LLM |
| WIP | Work in Progress, a document tracking current development focus |

