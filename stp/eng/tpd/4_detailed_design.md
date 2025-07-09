---
verblock: "08 Jul 2025:v0.2: Matthew Sinclair - Added Backlog.md integration implementation details"
stp_version: 1.2.0
---
# 4. Detailed Design

[index](<./technical_product_design.md>)

## 4.1 Directory Structure

The STP system organizes documentation into a structured directory hierarchy:

```
stp/
├── _templ/             # Templates directory
│   ├── prj/            # Project document templates
│   │   ├── _wip.md
│   │   └── st/
│   │       ├── _steel_threads.md
│   │       └── _ST####.md
│   ├── eng/            # Engineering document templates
│   │   └── tpd/
│   │       ├── _technical_product_design.md
│   │       ├── _1_introduction.md
│   │       ├── ...
│   ├── usr/            # User document templates
│   │   ├── _user_guide.md
│   │   ├── _reference_guide.md
│   │   └── _deployment_guide.md
│   └── llm/            # LLM document templates
│       └── _llm_preamble.md
├── bin/                # STP scripts
│   ├── stp             # Main STP command
│   ├── stp_init        # Init command implementation
│   ├── stp_st          # Steel thread command implementation
│   ├── stp_help        # Help command implementation
│   ├── stp_backlog     # Backlog wrapper implementation
│   ├── stp_task        # Task management implementation
│   ├── stp_status      # Status sync implementation
│   ├── stp_migrate     # Task migration implementation
│   └── ...             # Other command implementations
├── prj/                # Project documentation
│   ├── st/             # Steel threads
│   │   ├── steel_threads.md   # Steel thread index
│   │   ├── ST0001.md          # Individual steel thread
│   │   └── ...
│   └── wip.md          # Work in progress
├── eng/                # Engineering docs
│   └── tpd/            # Technical Product Design
│       ├── technical_product_design.md   # Main TPD document
│       ├── 1_introduction.md            # TPD sections
│       └── ...
├── usr/                # User documentation
│   ├── user_guide.md
│   ├── reference_guide.md
│   └── deployment_guide.md
├── llm/                # LLM-specific content
│   ├── llm_preamble.md
│   └── *.prompt.md     # Canned prompts
└── backlog/            # Backlog.md task management
    ├── tasks/          # Active tasks
    ├── drafts/         # Draft tasks
    ├── archive/        # Archived tasks
    └── config.yml      # Backlog configuration
```

## 4.2 Document Templates

### 4.2.1 Document Metadata

All STP documents use YAML frontmatter to store structured metadata at the beginning of the file:

```yaml
---
verblock: "DD MMM YYYY:v0.1: Author Name - Initial version"
stp_version: 1.2.0
status: Not Started|In Progress|Completed|On Hold|Cancelled
created: YYYYMMDD
completed: YYYYMMDD
---
```

**Metadata Fields:**
- `verblock`: Tracks version information with date, version number, author, and description
- `stp_version`: Indicates the STP version used, for compatibility and upgrade purposes
- `status`: Current state of the document or steel thread
- `created`: Creation date in YYYYMMDD format
- `completed`: Completion date in YYYYMMDD format (when applicable)

### 4.2.2 Section Markers

STP uses HTML comment markers to identify sections in documents that can be automatically updated:

```markdown
<!-- BEGIN: SECTION_NAME -->
(Content here will be automatically managed by STP commands)
<!-- END: SECTION_NAME -->
```

In particular, the steel_threads.md index file uses these markers to allow the `stp st sync` command to update the index while preserving manually added content outside the marked sections:

```markdown
<!-- BEGIN: STEEL_THREAD_INDEX -->
| ID | Title | Status | Created | Completed |
|----|-------|--------|---------|-----------|
| ST0001 | Example Thread | Completed | 2025-03-01 | 2025-03-05 |
<!-- END: STEEL_THREAD_INDEX -->
```

### 4.2.3 Project Templates

#### Work In Progress (WIP) Template

The WIP document captures the current state of development and active tasks.

**Structure:**

```markdown
# Work In Progress

## Current Focus
[Brief description of the current development focus]

## Active Steel Threads
- ST####: [Brief description]
- ...

## Upcoming Work
- [Item 1]
- ...

## Notes
[Any additional notes about the current work]
```

#### Journal Template

The Journal document maintains a chronological record of project activities.

**Structure:**

```markdown
# Project Journal

## YYYY-MM-DD
### [Activity Title]
[Description of activity, decisions made, challenges encountered, etc.]

## YYYY-MM-DD
...
```

#### Steel Thread Templates

**Steel Threads Index Template:**

```markdown
# Steel Threads

This document serves as an index of all steel threads in the project.

## Index

<!-- BEGIN: STEEL_THREAD_INDEX -->
| ID                    | Title   | Status   | Created  | Completed |
|-----------------------|---------|----------|----------|-----------|
| [ST0002](./ST0002.md) | [Title] | [Status] | YYYYMMDD | YYYYMMDD  |
| [ST0001](./ST0001.md) | [Title] | [Status] | YYYYMMDD | YYYYMMDD  |
<!-- END: STEEL_THREAD_INDEX -->
```

**Individual Steel Thread Template:**

```markdown
---
verblock: "DD MMM YYYY:v0.1: Author Name - Initial version"
stp_version: 1.2.0
status: Not Started
created: YYYYMMDD
completed: 
---
# ST####: [Title]

- **Status**: [Not Started|In Progress|Completed]
- **Created**: YYYY-MM-DD
- **Completed**: YYYY-MM-DD
- **Author**: Author Name

## Objective
[Clear statement of what this steel thread aims to accomplish]

## Context
[Background information and context for this steel thread]

## Approach
[Planned approach for implementing this steel thread]

## Tasks
Tasks are tracked in Backlog. View with: `stp task list ST####`

## Implementation Notes
[Notes on implementation details, decisions, challenges, etc.]

## Results
[Summary of results after completion]
```

### 4.2.2 Engineering Templates

Technical Product Design templates follow the structure outlined in previous sections.

### 4.2.3 User Documentation Templates

User Guide, Reference Guide, and Deployment Guide templates follow standard technical documentation formats.

### 4.2.4 LLM Templates

The LLM Preamble template provides context and instructions for the LLM:

```markdown
# LLM Preamble

## Project Context
[Brief description of the project]

## Collaboration Guidelines
[Guidelines for how the LLM should collaborate with developers]

## Code Style and Conventions
[Code style and conventions to follow]

## Document Structure
[Description of the document structure for context]

## Process Guidelines
[Guidelines for the steel thread process]
```

## 4.3 Command-line Interface

### 4.3.1 Command Structure

The STP command-line interface follows a subcommand pattern:

```
stp <command> [options] [arguments]
```

Main commands include:

- `init`: Initialize STP in a project
- `st`: Manage steel threads
- `help`: Display help information
- `upgrade`: Upgrade STP files to the latest format
- `bl` / `backlog`: Wrapper for Backlog.md commands
- `task`: Manage tasks linked to steel threads
- `status`: Synchronize steel thread status with tasks
- `migrate`: Migrate embedded tasks to Backlog

Subcommands include:

- `st new`: Create a new steel thread
- `st done`: Mark a steel thread as complete
- `st list`: List all steel threads with optional filtering by status
- `st sync`: Synchronize the steel_threads.md index file with individual ST files
- `st show`: Show details of a specific steel thread
- `st edit`: Open a steel thread in the default editor
- `bl create`: Create a task linked to a steel thread
- `bl list`: List all tasks without git errors
- `task list`: List tasks for a specific steel thread
- `status sync`: Update steel thread status based on tasks

### 4.3.2 Command Implementation

Each command is implemented as a separate shell script:

1. `stp`: Main dispatcher that validates input and calls appropriate subcommand
2. `stp_<command>`: Implements specific command functionality
3. `stp_help`: Displays help information from `.help` directory
4. `stp_st`: Manages steel thread operations (new, done, list, sync, show, edit)
5. `stp_upgrade`: Upgrades STP files to the latest format and standards
6. `stp_backlog`: Wrapper for Backlog.md to avoid git errors and provide shortcuts
7. `stp_task`: Manages tasks linked to steel threads (create, list, sync)
8. `stp_status`: Synchronizes steel thread status based on task completion
9. `stp_migrate`: Migrates embedded tasks from steel threads to Backlog

### 4.3.3 Help System

The help system uses markdown files in a `.help` directory:

```
.help/
├── init.help.md
├── st.help.md
└── ...
```

Each help file follows a standardized format with sections for short description, detailed description, and usage information.

## 4.4 Process Guidelines

### 4.4.1 Steel Thread Workflow

The steel thread workflow follows these steps:

1. **Creation**: Developer creates a new steel thread
2. **Planning**: Developer defines objective, context, and approach
3. **Implementation**: Developer implements tasks with LLM assistance
4. **Documentation**: LLM and developer document implementation details
5. **Completion**: Developer marks the steel thread as complete

### 4.4.2 LLM Collaboration Model

The LLM collaboration model defines how developers work with LLMs:

1. **Context Setting**: Share relevant project documents with the LLM
2. **Task Description**: Clearly describe the current task
3. **Interactive Development**: Iteratively work with the LLM to develop solutions
4. **Documentation**: Have the LLM update documentation as work progresses
5. **Context Preservation**: Capture key information for future sessions

## 4.5 Data Flow

### 4.5.1 Command Data Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  User Input │────►│ STP Command │────►│  Subcommand │
└─────────────┘     └─────────────┘     └──────┬──────┘
                                               │
                                               ▼
                                        ┌─────────────┐
                                        │   Project   │
                                        │  Documents  │
                                        └─────────────┘
```

### 4.5.2 Document Update Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Developer  │────►│     LLM     │────►│   Updated   │
│    Input    │     │             │     │  Documents  │
└─────────────┘     └─────────────┘     └─────────────┘
```

## 4.6 Error Handling

STP implements error handling at multiple levels:

1. **Command Validation**: Validate input parameters and provide clear error messages
2. **Execution Validation**: Check for required files and directories before operations
3. **Status Reporting**: Provide clear success/failure indicators for operations
4. **Recovery Guidance**: Suggest recovery steps when errors occur

## 4.7 Security Considerations

STP addresses security through:

1. **No External Dependencies**: Minimizing attack surface through self-contained implementation
2. **File Permission Management**: Ensuring appropriate permissions for created files
3. **Input Validation**: Sanitizing user input to prevent script injection
4. **No Sensitive Data**: Avoiding storage of credentials or sensitive information

## 4.8 Integration Implementations

### 4.8.1 Backlog.md Integration Details

The Backlog.md integration extends STP with task management capabilities through a set of wrapper commands and conventions.

#### Command Implementations

**1. Backlog Wrapper (`stp_backlog`)**

The `stp bl` command provides a wrapper around Backlog.md to:
- Add `--plain` flag automatically to prevent git fetch errors
- Provide shortcuts for common operations
- Maintain consistent error handling

```bash
# Key wrapper behaviors
stp bl list          → backlog task list --plain
stp bl board         → backlog board --plain  
stp bl create <args> → backlog task create <args>
```

**2. Task Management (`stp_task`)**

The `stp task` command manages the relationship between steel threads and Backlog tasks:

```bash
stp task create <ST####> <title>  # Creates task with ST prefix
stp task list <ST####>            # Lists all tasks for a thread
stp task sync <ST####>            # Updates thread status from tasks
```

**3. Status Synchronisation (`stp_status`)**

The `stp status` command provides bidirectional status updates:

```bash
stp status show <ST####>   # Shows thread and task status
stp status sync <ST####>   # Updates thread status from tasks
stp status report          # Overall project status
```

Status mapping rules:
- All tasks in draft/none → Steel thread: "Not Started"
- Any task in todo/in-progress → Steel thread: "In Progress"  
- All tasks done/archived → Steel thread: "Completed"
- Manual override for "On Hold" and "Cancelled"

**4. Migration Tool (`stp_migrate`)**

The `stp migrate` command converts embedded task lists to Backlog:

```bash
stp migrate <ST####>        # Migrate specific thread
stp migrate --all-active    # Migrate all active threads
stp migrate --dry-run       # Preview migration
```

Migration process:
1. Parse markdown checkboxes from steel thread
2. Create Backlog tasks with appropriate status
3. Update steel thread to reference Backlog
4. Preserve completion status

#### Naming Conventions

Tasks linked to steel threads follow strict naming:

```
ST#### - <task description>
```

Examples:
- `ST0014 - Create directory structure`
- `ST0014 - Update command implementations`
- `ST0014 - Add integration tests`

This convention enables:
- Automatic linking between systems
- Filtering and grouping operations
- Status synchronisation

#### File Structure Integration

```
project/
├── stp/
│   └── prj/
│       └── st/
│           ├── steel_threads.md    # Thread index
│           ├── ST0001.md          # Steel thread docs
│           └── ...
└── backlog/
    ├── config.yml                 # Backlog configuration
    ├── tasks/                     # Active tasks
    │   ├── task-001 - ST0014 - Create structure.md
    │   └── ...
    ├── drafts/                    # Draft tasks
    └── archive/                   # Completed tasks
```

Key principles:
- Complete separation of STP and Backlog directories
- No file conflicts or overlaps
- Each system maintains its own structure

#### Workflow Integration Patterns

**1. New Feature Development**

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

**2. Research and Design**

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

**3. Bug Fix Workflow**

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

#### Error Handling

The integration includes specific error handling:

1. **Missing Backlog Installation**: Clear message with installation instructions
2. **Git Fetch Errors**: Automatically prevented with `--plain` flag
3. **Invalid Steel Thread IDs**: Validation before task creation
4. **Status Conflicts**: Warning when manual status doesn't match tasks

#### Testing Infrastructure

Integration tests are provided in:
- `stp/tests/task/task_test.bats` - Task command tests
- `stp/tests/status/status_test.bats` - Status synchronisation tests
- `stp/tests/migrate/migrate_test.bats` - Migration tests
- `stp/tests/backlog/backlog_test.bats` - Wrapper command tests

Test coverage includes:
- Command functionality
- Error conditions
- Edge cases
- Integration workflows
