---
verblock: "09 Jul 2025:v0.4: Matthew Sinclair - Updated llm command with --symlink option"
stp_version: 1.2.0
---
# Reference Guide

# Reference Guide

This reference guide provides comprehensive information about the Steel Thread Process (STP) system. Unlike the task-oriented User Guide, this reference guide serves as a complete reference for all aspects of the system.

## Table of Contents

1. [Command Reference](#command-reference)
2. [Document Templates](#document-templates)
3. [Directory Structure](#directory-structure)
4. [Configuration Options](#configuration-options)
5. [Best Practices](#best-practices)
6. [Concepts and Terminology](#concepts-and-terminology)
7. [Backlog.md Integration](#backlogmd-integration)

## Command Reference

### Core Commands

#### `stp upgrade`

Upgrades STP files to the latest format.

**Usage:**

```bash
stp upgrade [--force]
```

**Options:**

- `--force`: Force upgrade even for major version differences

**Example:**

```bash
stp upgrade
```

**Output:**

- Updates all STP files with the latest format and metadata
- Adds or updates STP version information in YAML frontmatter
- Adds or updates missing metadata fields
- Adds section markers to steel_threads.md for sync
- Runs `stp st sync --write` to update steel thread index
- Reports upgrade status for each file processed

Example output:

```
Starting STP upgrade process...
Current STP version: 1.0.0

Scanning for STP files to upgrade...
Checking steel_threads.md...
Added section markers to stp/prj/st/steel_threads.md
Upgrading steel thread files...
Processing stp/prj/st/ST0001.md (current version: 0.0.0)
  Missing Status field in file
  Missing Created field in file
  Updating file to add missing metadata fields...
Updated: stp/prj/st/ST0001.md
Processing stp/prj/st/ST0002.md (current version: 1.0.0)
  Already at latest version, no update needed.

Running sync to update steel_threads.md...
Updated steel threads index file: stp/prj/st/steel_threads.md

STP upgrade complete.
```

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
stp st list [--status <status>] [--width <columns>]
```

**Options:**

- `--status`: Filter by status (optional)
- `--width`: Set the output table width in columns (optional, defaults to terminal width)

**Example:**

```bash
stp st list --status "In Progress" --width 100
```

**Output:**

```
ID     | Title                                | Status      | Created    | Completed  
-------|--------------------------------------|-------------|------------|-----------
ST0003 | Implement Feature X                  | In Progress | 2025-03-08 |            
ST0002 | Design Database Schema               | In Progress | 2025-03-07 |            
ST0001 | Project Setup                        | Completed   | 2025-03-05 | 2025-03-06 
```

`stp st sync`

Synchronizes the steel_threads.md document with individual steel thread files.

**Usage:**

```bash
stp st sync [--write] [--width <columns>]
```

**Options:**

- `--write`: Update the steel_threads.md file (optional, without this flag output is sent to stdout)
- `--width`: Set the output table width in columns (optional, defaults to terminal width)

**Example:**

```bash
stp st sync --write --width 100
```

**Output:**

Updates the steel_threads.md file with the current status of all steel thread files, preserving content outside the marked section:

```markdown
# Steel Threads

This document serves as an index of all steel threads in the project.

## Index

<!-- BEGIN: STEEL_THREAD_INDEX -->
| ID                      | Title                                | Status      | Created    | Completed   |
|-------------------------|--------------------------------------|-------------|------------|-------------|
| [ST0003](./ST0003.md)   | Implement Feature X                  | In Progress | 2025-03-08 |             |
| [ST0002](./ST0002.md)   | Design Database Schema               | In Progress | 2025-03-07 |             |
| [ST0001](./ST0001.md)   | Project Setup                        | Completed   | 2025-03-05 | 2025-03-06  |
<!-- END: STEEL_THREAD_INDEX -->

## Notes

Additional notes about steel threads can be added here.
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

#### `stp llm`

Commands for LLM integration and assistance.

**Usage:**

```bash
stp llm <subcommand> [options]
```

**Purpose:**

Provides utilities for working with Large Language Models (LLMs) in the context of STP. Helps LLMs understand how to use STP effectively and facilitates better collaboration between developers and AI assistants.

**Subcommands:**

`stp llm usage_rules`

Displays the complete STP usage patterns and workflows documentation.

**Usage:**

```bash
stp llm usage_rules
```

**Example:**

```bash
# Display usage rules
stp llm usage_rules

# Create symlink in current directory
stp llm usage_rules --symlink

# Create symlink in specific directory
stp llm usage_rules --symlink ~/my-project

# Pipe to less for easier reading
stp llm usage_rules | less

# Save to a file
stp llm usage_rules > stp-usage-rules.md
```

**Options:**

- `--symlink [dir]`: Create a symlink to usage-rules.md in current or specified directory

**Notes:**

- The usage rules document is located at `stp/eng/usage-rules.md`
- It follows the pattern established by the Elixir Hex package 'usage_rules'
- The document can be regenerated using the prompt at `stp/eng/prompts/regenerate_usage_rules.md`
- The --symlink option creates a symlink named 'usage-rules.md' for integration with other tools

#### `stp bl` / `stp backlog`

STP wrapper for Backlog.md task management.

**Usage:**

```bash
stp bl <command> [options] [arguments]
stp backlog <command> [options] [arguments]
```

**Purpose:**

Provides a streamlined interface to Backlog.md that avoids common issues like git fetch errors and provides shortcuts for STP workflows.

**Subcommands:**

`stp bl init`

Initializes Backlog with STP-friendly settings.

**Usage:**

```bash
stp bl init
```

**Effect:**

- Creates backlog directory structure
- Disables remote operations to prevent git errors
- Sets default status to "To Do"

`stp bl create`

Creates a task linked to a steel thread.

**Usage:**

```bash
stp bl create <ST####> <title>
```

**Parameters:**

- `ST####`: Steel thread ID (required)
- `title`: Task description (required)

**Example:**

```bash
stp bl create ST0014 "Add validation logic"
```

`stp bl list`

Lists all tasks without git fetch errors.

**Usage:**

```bash
stp bl list
```

**Note:** Automatically adds `--plain` flag to prevent git operations.

`stp bl board`

Displays tasks in Kanban board view.

**Usage:**

```bash
stp bl board
```

`stp bl task`

Manages individual tasks.

**Usage:**

```bash
stp bl task <subcommand> [options]
```

**Example:**

```bash
stp bl task edit task-5 --status Done
```

#### `stp task`

Manages Backlog tasks linked to steel threads.

**Usage:**

```bash
stp task <command> [options] [arguments]
```

**Subcommands:**

`stp task create`

Creates a new task linked to a steel thread.

**Usage:**

```bash
stp task create <ST####> <title>
```

**Parameters:**

- `ST####`: Steel thread ID (required)
- `title`: Task description (required)

**Example:**

```bash
stp task create ST0014 "Implement error handling"
```

`stp task list`

Lists all tasks for a specific steel thread.

**Usage:**

```bash
stp task list <ST####>
```

**Parameters:**

- `ST####`: Steel thread ID (required)

**Example:**

```bash
stp task list ST0014
```

**Output:**

```
Tasks for ST0014:
================
task-1       [done]          ST0014 - Create directory structure
task-2       [todo]          ST0014 - Add unit tests
```

`stp task sync`

Synchronizes task status with steel thread.

**Usage:**

```bash
stp task sync <ST####>
```

**Parameters:**

- `ST####`: Steel thread ID (required)

#### `stp status`

Synchronizes steel thread status based on Backlog task completion.

**Usage:**

```bash
stp status <command> [options] [arguments]
```

**Subcommands:**

`stp status show`

Displays status of steel thread and its tasks.

**Usage:**

```bash
stp status show <ST####>
```

**Parameters:**

- `ST####`: Steel thread ID (required)

**Output:**

```
Steel Thread: ST0014
Current Status: In Progress

Task Summary:
  Total Tasks: 5
  - Done: 4
  - In Progress: 0
  - Todo: 1

Recommended Status: In Progress
```

`stp status sync`

Updates steel thread status based on task completion.

**Usage:**

```bash
stp status sync <ST####> [--dry-run]
```

**Parameters:**

- `ST####`: Steel thread ID (required)

**Options:**

- `--dry-run`: Preview changes without updating

`stp status report`

Generates status report for all active threads.

**Usage:**

```bash
stp status report
```

#### `stp migrate`

Migrates embedded tasks from steel threads to Backlog.

**Usage:**

```bash
stp migrate [options] <ST####>
```

**Parameters:**

- `ST####`: Steel thread ID to migrate

**Options:**

- `--dry-run`: Preview migration without creating tasks
- `--all-active`: Migrate all active steel threads

**Example:**

```bash
# Migrate a single steel thread
stp migrate ST0014

# Preview migration
stp migrate --dry-run ST0014

# Migrate all active threads
stp migrate --all-active
```

**Effect:**

- Extracts checkbox tasks from steel thread documents
- Creates corresponding Backlog tasks
- Updates steel thread to reference Backlog
- Preserves task completion status

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
./run_tests.sh task
./run_tests.sh status
./run_tests.sh migrate
./run_tests.sh backlog
./run_tests.sh bl

# Set up test environment
./setup_test_env.sh
```

The test environment setup script installs necessary dependencies, including:

- Bats (Bash Automated Testing System)
- bats-support
- bats-assert
- bats-file

## Document Templates

### Steel Thread Document Format

Steel thread documents (located in `stp/prj/st/ST####.md`) use a standardized format with two ways to store metadata:

#### STP Versioning

Each STP file includes version information to track compatibility:

```yaml
---
stp_version: 1.2.0
---
```

The version follows semantic versioning (MAJOR.MINOR.PATCH) where:

- MAJOR: Incompatible changes that require manual migration
- MINOR: New features in a backward-compatible manner
- PATCH: Backward-compatible bug fixes

When running `stp upgrade`, the system checks this version to determine what upgrades are needed.

#### YAML Frontmatter

Steel thread files can use YAML frontmatter at the beginning of the file to store structured metadata:

```yaml
---
verblock: "06 Mar 2025:v0.1: Author Name - Initial version"
status: In Progress
created: 20250307
completed: 
---
```

**Supported Metadata Fields:**

- `status`: Current state of the steel thread (Not Started, In Progress, Completed, On Hold, or Cancelled)
- `created`: Creation date in YYYYMMDD format
- `completed`: Completion date in YYYYMMDD format (omit or leave empty if not completed)
- `verblock`: Version tracking information

#### Document Body Metadata

Steel thread documents also include metadata within the document body in a human-readable format:

```markdown
# ST0001: Steel Thread Title

- **Status**: In Progress
- **Created**: 2025-03-07
- **Completed**: 
- **Author**: Author Name
```

When using both formats, the document body metadata takes precedence over YAML frontmatter when displayed in the steel threads list.

#### Section Markers in steel_threads.md

The steel_threads.md document uses HTML comment markers to identify sections that can be automatically updated by the `stp st sync` command:

```markdown
<!-- BEGIN: STEEL_THREAD_INDEX -->
(content here will be replaced by sync command)
<!-- END: STEEL_THREAD_INDEX -->
```

These markers should not be removed from the document, as they enable automatic updates while preserving manually edited content outside the marked sections.

### Project Templates

#### Work in Progress (WIP) Template

Location: `stp/prj/wip.md`

Purpose: Tracks current development focus and active steel threads.

Structure:

- Current Focus
- Active Steel Threads
- Upcoming Work
- Notes

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
│   │   │   ├── COMPLETED/    # Completed steel threads
│   │   │   ├── NOT-STARTED/  # Not started steel threads
│   │   │   └── CANCELLED/    # Cancelled steel threads
│   │   └── wip.md      # Work in progress
│   ├── eng/            # Engineering docs
│   │   └── tpd/        # Technical Product Design
│   ├── usr/            # User documentation
│   ├── llm/            # LLM-specific content
│   └── tests/          # Test suite
│       ├── bootstrap/  # Bootstrap tests
│       ├── init/       # Init command tests
│       ├── st/         # Steel thread command tests
│       ├── task/       # Task management tests
│       ├── status/     # Status sync tests
│       ├── migrate/    # Migration tests
│       ├── backlog/    # Backlog wrapper tests
│       ├── bl/         # bl command tests
│       ├── help/       # Help system tests
│       ├── main/       # Main script tests
│       ├── lib/        # Test helper libraries
│       ├── fixtures/   # Test fixtures
│       └── run_tests.sh # Test runner script
├── bin/                # STP scripts (executable)
└── backlog/            # Backlog.md task management
    ├── tasks/          # Active tasks
    ├── drafts/         # Draft tasks
    ├── archive/        # Archived tasks
    └── config.yml      # Backlog configuration
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

Location: `stp/.config/config`

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
- Document decisions and their rationale in steel threads and Backlog tasks
- Use clear, descriptive titles for steel threads
- Maintain cross-references between related documents

### Task Management with Backlog

- Use `stp bl` wrapper instead of `backlog` directly to avoid git errors
- Create tasks linked to steel threads for traceability
- Keep tasks granular (1-2 days of work)
- Regularly sync steel thread status with task completion
- Use task status values: "To Do", "In Progress", "Done"
- Migrate existing embedded tasks using `stp migrate`

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
| Backlog | Task management system integrated with STP for fine-grained work tracking |
| Task | Individual unit of work linked to a steel thread, tracked in Backlog |
| Task Status | State of a task: "To Do", "In Progress", or "Done" |

## Backlog.md Integration

This section provides comprehensive documentation for the integration between STP (Steel Thread Process) and Backlog.md for enhanced task management. The integration maintains STP's strength in intent capture while leveraging Backlog.md's powerful task tracking capabilities.

### Overview

The integration between STP and Backlog.md provides:

- **Intent Capture**: Steel threads for high-level objectives and context
- **Task Management**: Backlog for granular task tracking with rich metadata
- **Status Synchronisation**: Automatic updates between systems
- **Workflow Integration**: Seamless development workflows

### Architecture

#### STP Responsibilities

- **Intent Capture**: High-level objectives and context in steel thread documents
- **Design Documentation**: Detailed design specifications (ST####_design.md)
- **Implementation Records**: As-built documentation (ST####_impl.md)
- **Process Coordination**: Overall workflow and steel thread lifecycle

#### Backlog.md Responsibilities

- **Task Management**: Individual task tracking with rich metadata
- **Status Tracking**: Granular task states (draft, todo, in-progress, done)
- **Task Organisation**: Labels, priorities, dependencies, and subtasks
- **Visualisation**: Kanban board and browser interface

### Using the STP Backlog Wrapper

STP provides a wrapper command `stp backlog` (or `stp bl` for short) that streamlines Backlog usage:

```bash
# Initialize backlog with STP-friendly settings
stp bl init

# List tasks without git fetch errors  
stp bl list

# Create tasks linked to steel threads
stp bl create ST0014 "Add validation"

# View Kanban board
stp bl board
```

The wrapper automatically:

- Adds `--plain` to list/board commands to prevent git errors
- Disables remote operations for local projects
- Provides shortcuts for common workflows

### Naming Conventions

#### Backlog Task Naming

Tasks linked to steel threads follow this pattern:

```
ST#### - <task description>
```

Example:

```
ST0014 - Create directory structure
ST0014 - Update ST commands for new paths
ST0014 - Add unit tests
```

#### File Organisation

- Steel thread documents remain in `/stp/prj/st/`
- Backlog tasks are stored in `/backlog/tasks/`
- Task files are named: `task-<id> - <title>.md`

### Workflow Integration

#### 1. Creating a New Steel Thread

```bash
# Create the steel thread
stp st new "My New Feature"
# Returns: Created ST0015

# Create associated tasks using the backlog wrapper
stp bl create ST0015 "Design API structure"
stp bl create ST0015 "Implement core logic"
stp bl create ST0015 "Create registration flow"
stp bl create ST0015 "Add session management"
stp bl create ST0015 "Write integration tests"
```

#### 2. Task Lifecycle

1. **Draft Phase**: Ideas and potential tasks

   ```bash
   backlog draft create "ST0015 - Consider caching strategy"
   ```

2. **Active Development**: Move to active tasks

   ```bash
   backlog draft promote <task-id>
   backlog task edit <task-id> --status in-progress
   ```

3. **Completion**: Mark tasks done

   ```bash
   backlog task edit <task-id> --status done
   ```

4. **Archival**: Archive completed tasks

   ```bash
   backlog task archive <task-id>
   ```

#### 3. Status Synchronisation

Steel thread status is determined by task states:

- **Not Started**: No tasks created or all tasks in draft
- **In Progress**: At least one task in todo/in-progress state
- **On Hold**: Manual designation when work is paused
- **Completed**: All tasks done or archived
- **Cancelled**: Manual designation with tasks archived

Use `stp status` to sync:

```bash
stp status sync ST0015
```

#### 4. Viewing Tasks

```bash
# View all tasks for a steel thread
stp task list ST0015

# View all tasks without git errors
stp bl list

# View in Kanban board
stp bl board

# View in browser
stp bl browser
```

### Steel Thread Document Structure

With Backlog integration, steel thread documents focus on intent and context:

```markdown
---
verblock: "08 Jul 2025:v0.1: Author Name - Initial version"
stp_version: 1.2.0
status: In Progress
created: 20250708
completed: 
---
# ST0015: Feature Title

## Objective
High-level goal and business value

## Context
Background information and rationale

## Approach
Strategic approach and key decisions

## Tasks
Tasks are tracked in Backlog. View with: `stp task list ST0015`

## Implementation Notes
Key technical decisions and learnings

## Results
Outcomes and metrics (completed threads)
```

### Migration from Embedded Tasks

For existing steel threads with embedded task lists:

```bash
# Migrate a specific steel thread
stp migrate ST0014

# Migrate all active threads
stp migrate --all-active
```

This will:

1. Extract checkbox tasks from the markdown
2. Create corresponding Backlog tasks
3. Update the steel thread document
4. Preserve task completion status

### Best Practices

#### Task Granularity

- Keep tasks focused and achievable in 1-2 days
- Use subtasks for complex items
- Create separate tasks for research vs implementation

#### Labeling Strategy

- Always include steel thread ID in task title
- Use additional labels for cross-cutting concerns:
  - `bug`, `feature`, `refactor`, `docs`
  - `blocked`, `waiting-review`
  - Team or component labels

#### Dependencies

- Use Backlog's dependency features for task ordering
- Document external dependencies in task notes
- Link related tasks across steel threads

#### Regular Maintenance

- Run `stp status sync` regularly
- Archive completed tasks weekly
- Review and promote drafts in planning sessions

### Workflow Examples

#### New Feature Development

```bash
# 1. Create steel thread for high-level planning
stp st new "Implement user authentication"
# Output: Created ST0015

# 2. Create implementation tasks
stp task create ST0015 "Design auth database schema"
stp task create ST0015 "Implement login endpoint"
stp task create ST0015 "Create registration flow"
stp task create ST0015 "Add session management"
stp task create ST0015 "Write integration tests"

# 3. Work through tasks
stp bl board                      # View Kanban board
backlog task edit <id> --status in-progress

# 4. Sync status back to steel thread
stp status sync ST0015
```

#### Research and Design

```bash
# 1. Create steel thread for research
stp st new "Research caching strategies"

# 2. Create investigation tasks
stp task create ST0016 "Review Redis capabilities"
stp task create ST0016 "Benchmark Memcached performance"
stp task create ST0016 "Evaluate in-memory options"
stp task create ST0016 "Document recommendations"

# 3. Track progress
stp task list ST0016
```

#### Bug Fix Workflow

```bash
# 1. Create steel thread for bug
stp st new "Fix authentication timeout issue"

# 2. Create diagnostic and fix tasks
stp task create ST0017 "Reproduce timeout issue"
stp task create ST0017 "Debug session handling"
stp task create ST0017 "Implement fix"
stp task create ST0017 "Add regression test"

# 3. Fast status check
stp status show ST0017
```

### Troubleshooting

#### Common Issues

1. **Task ID Conflicts**
   - Backlog assigns unique IDs automatically
   - Don't manually edit task IDs

2. **Status Mismatch**
   - Run `stp status sync` to update
   - Check for tasks in unexpected states

3. **Missing Tasks**
   - Check drafts folder
   - Verify task wasn't archived

4. **Git Fetch Errors**
   - Use `stp bl` wrapper instead of `backlog` directly
   - The wrapper adds `--plain` flag automatically

#### Getting Help

- Run `stp help` for STP commands
- Run `stp help backlog` for STP's Backlog wrapper
- Run `backlog help` for native Backlog commands

### Testing

The integration includes comprehensive test coverage:

```bash
# Run all integration tests
cd stp/tests
./run_tests.sh task
./run_tests.sh status  
./run_tests.sh migrate

# Or run specific test files
bats task/task_test.bats
bats status/status_test.bats
bats migrate/migrate_test.bats
```

Test files are located in:

- `stp/tests/task/task_test.bats` - Task command tests
- `stp/tests/status/status_test.bats` - Status command tests  
- `stp/tests/migrate/migrate_test.bats` - Migration command tests
