---
verblock: "17 Jul 2025:v2.0.0: Matthew Sinclair - Updated for Intent v2.0.0 (As-Built)"
intent_version: 2.0.0
---

# 4. Detailed Design [AS-BUILT]

[index](./technical_product_design.md)

## 4.1 Directory Structure [AS-BUILT]

Intent v2.0.0 uses a flattened directory structure that simplifies navigation:

```
<project_root>/
├── .intent/                    # Intent configuration
│   └── config.json            # JSON configuration file
├── intent/                     # Main Intent directory (flattened)
│   ├── st/                    # Steel threads
│   │   ├── ST0001/            # Steel thread directory
│   │   │   ├── info.md       # Main information (required)
│   │   │   ├── design.md     # Design decisions (optional)
│   │   │   ├── impl.md       # Implementation (optional)
│   │   │   └── tasks.md      # Task tracking (optional)
│   │   ├── ST0002/            # Another steel thread
│   │   │   └── info.md       # Minimum required file
│   │   └── ...
│   ├── docs/                  # Documentation
│   │   ├── blog/              # Blog posts
│   │   │   ├── 0000-motivation-for-intent.md
│   │   │   ├── 0001-introduction-to-intent.md
│   │   │   └── ...
│   │   ├── eng/               # Engineering docs
│   │   │   └── tpd/           # Technical Product Design
│   │   └── usr/               # User documentation
│   ├── llm/                   # LLM-specific content
│   └── wip.md                 # Work in progress
├── backlog/                    # Backlog.md integration
│   ├── Backlog.md             # Main backlog file
│   ├── tasks/                 # Task files
│   └── ...
├── bin/                        # Intent scripts
│   ├── intent                 # Main command
│   ├── intent_st              # Steel thread commands
│   ├── intent_bl              # Backlog wrapper
│   ├── intent_task            # Task management
│   ├── intent_status          # Status sync
│   ├── intent_init            # Project initialization
│   ├── intent_bootstrap       # Global setup
│   ├── intent_doctor          # Diagnostics
│   ├── intent_upgrade         # Migration tool
│   ├── intent_config          # Configuration library
│   ├── intent_helpers         # Helper functions
│   └── stp                    # Backward compatibility symlink
└── CLAUDE.md                   # Project guidelines for LLMs

[Legacy STP structure removed - see v1.2.1 documentation]
```

## 4.2 Document Templates

### 4.2.1 Configuration System [AS-BUILT]

Intent v2.0.0 uses JSON configuration with hierarchy support:

#### Configuration File Format

```json
{
  "version": "2.0.0",
  "project_name": "MyProject",
  "author": "username",
  "created": "2025-07-17",
  "st_prefix": "ST",
  "backlog_dir": "backlog",
  "intent_dir": "intent",
  "backlog_list_status": "todo"
}
```

#### Configuration Fields

| Field               | Description         | Default    | Added  |
| ------------------- | ------------------- | ---------- | ------ |
| version             | Intent version      | 2.0.0      | v2.0.0 |
| project_name        | Project identifier  | (required) | v2.0.0 |
| author              | Default author      | $USER      | v0.0.0 |
| created             | Creation date       | (auto)     | v2.0.0 |
| st_prefix           | Steel thread prefix | ST         | v1.0.0 |
| backlog_dir         | Backlog directory   | backlog    | v1.2.0 |
| intent_dir          | Intent directory    | intent     | v2.0.0 |
| backlog_list_status | Default list filter | (none)     | v2.0.0 |

#### Configuration Hierarchy

```
1. Environment Variables (highest priority)
   - INTENT_* variables
   - AUTHOR, EDITOR

2. Local Project Config
   - .intent/config.json

3. Global User Config
   - ~/.config/intent/config.json

4. Built-in Defaults (lowest priority)
   - Hardcoded in intent_config
```

### 4.2.2 Document Metadata

Intent documents use YAML frontmatter for metadata:

```yaml
---
verblock: "DD MMM YYYY:v0.1: Author Name - Initial version"
intent_version: 2.0.0
status: Not Started|In Progress|Completed|On Hold|Cancelled
created: YYYYMMDD
completed: YYYYMMDD
---
```

**Metadata Fields:**

- `verblock`: Version tracking with date, version, author, description
- `intent_version`: Intent version for compatibility
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

| ID     | Title          | Status    | Created    | Completed  |
| ------ | -------------- | --------- | ---------- | ---------- |
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

| ID                  | Title   | Status   | Created  | Completed |
| ------------------- | ------- | -------- | -------- | --------- |
| [ST0002](./ST0002/) | [Title] | [Status] | YYYYMMDD | YYYYMMDD  |
| [ST0001](./ST0001/) | [Title] | [Status] | YYYYMMDD | YYYYMMDD  |

<!-- END: STEEL_THREAD_INDEX -->
```

**Steel Thread Directory Structure (v1.2.1+):**

Stepping with STP v1.2.1, steel threads are organized as directories containing multiple files:

```
ST####/
├── info.md      # Main information file (required)
├── design.md    # Design decisions and approach
├── impl.md      # Implementation details
└── tasks.md     # Task tracking
```

**info.md Template (Main Information File):**

```markdown
---
verblock: "DD MMM YYYY:v0.1: Author Name - Initial version"
stp_version: 2.0.0
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
```

**design.md Template:**

```markdown
# Design - ST####: [Title]

## Approach

[Planned approach for implementing this steel thread]

## Key Design Decisions

[Document important design choices and rationale]

## Architecture

[Architectural diagrams or descriptions if applicable]
```

**impl.md Template:**

```markdown
# Implementation - ST####: [Title]

## Implementation Notes

[Technical details about the implementation]

## Code Changes

[Summary of code changes made]

## Challenges

[Any implementation challenges encountered]
```

**tasks.md Template:**

```markdown
# Tasks - ST####: [Title]

Tasks are tracked in Backlog. View with: `stp task list ST####`

## Task Summary

[High-level summary of tasks if needed]
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

## 4.3 Command-line Interface [AS-BUILT]

### 4.3.1 Command Structure

Intent v2.0.0 uses a unified command structure:

```
intent <command> [options] [arguments]
```

#### Primary Commands

| Command   | Description                    | Added  |
| --------- | ------------------------------ | ------ |
| init      | Initialize Intent in a project | v0.0.0 |
| st        | Manage steel threads           | v0.0.0 |
| bl        | Enhanced Backlog.md wrapper    | v1.2.0 |
| task      | Manage tasks linked to threads | v1.2.0 |
| status    | Synchronize thread/task status | v1.2.0 |
| bootstrap | Global Intent setup            | v2.0.0 |
| doctor    | Diagnose configuration issues  | v2.0.0 |
| upgrade   | Migrate from any STP version   | v2.0.0 |
| help      | Display comprehensive help     | v0.0.0 |

#### Steel Thread Subcommands

```
intent st new [title]         # Create new steel thread
intent st list [--status X]   # List threads with filtering
intent st show ST####         # Display thread contents
intent st edit ST#### [file]  # Edit thread files
```

#### Backlog Subcommands

```
intent bl create [options]    # Create task (git-safe)
intent bl list [--all]        # List tasks (respects config)
intent bl done <task-id>      # Mark task complete
```

#### New v2.0.0 Features

- **backlog_list_status**: Configurable default task filtering
- **--all flag**: Override status filtering
- **doctor --fix**: Automatic issue resolution
- **bootstrap**: One-time global setup

### 4.3.2 Command Implementation [AS-BUILT]

Intent v2.0.0 uses modular shell scripts:

#### Script Architecture

```
bin/
├── intent              # Main dispatcher
├── intent_<command>    # Command implementations
├── intent_config       # Configuration loader (JSON)
├── intent_helpers      # Shared utility functions
└── stp                 # Backward compatibility symlink
```

#### Key Implementation Files

| Script           | Purpose           | Key Features                    |
| ---------------- | ----------------- | ------------------------------- |
| intent           | Main entry point  | Command dispatch, version check |
| intent_config    | Config management | JSON parsing, hierarchy support |
| intent_helpers   | Utilities         | Version detection, migration    |
| intent_st        | Steel threads     | Create, list, show, edit        |
| intent_bl        | Backlog wrapper   | Git-safe operations, filtering  |
| intent_task      | Task management   | Thread linking, status tracking |
| intent_status    | Status sync       | Task completion analysis        |
| intent_bootstrap | Global setup      | First-time configuration        |
| intent_doctor    | Diagnostics       | Issue detection and fixes       |
| intent_upgrade   | Migration         | Any version to v2.0.0           |

### 4.3.3 Help System [AS-BUILT]

Intent v2.0.0 provides comprehensive help:

```
intent help              # General help
intent help <command>    # Command-specific help
intent <command> -h      # Quick help
```

#### Help Implementation

- Embedded help in each command script
- Consistent format across all commands
- Examples included for common usage
- Version information displayed

### 4.3.4 Configuration Loading [AS-BUILT]

The configuration system loads settings in order:

```bash
# From intent_config
load_intent_config() {
  # 1. Set defaults
  INTENT_DIR="intent"
  BACKLOG_DIR="backlog"

  # 2. Find project root
  PROJECT_ROOT=$(find_project_root)

  # 3. Load global config
  parse_json "~/.config/intent/config.json"

  # 4. Load local config (overrides global)
  parse_json ".intent/config.json"

  # 5. Apply environment variables (highest priority)
  [ -n "$INTENT_DIR" ] && INTENT_DIR="$INTENT_DIR"
}
```

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

## 4.8 Integration Implementations [AS-BUILT]

### 4.8.1 Enhanced Backlog.md Integration

Intent v2.0.0 provides enhanced Backlog.md integration with configurable filtering and improved git safety.

#### Command Implementations [AS-BUILT]

**1. Backlog Wrapper (`intent_bl`)**

The `intent bl` command provides enhanced functionality:

- Automatic `--plain` flag for git safety
- Configurable status filtering via `backlog_list_status`
- `--all` flag to override filtering
- Consistent error handling

```bash
# Key wrapper behaviors
intent bl list         → backlog task list --plain [filtered]
intent bl list --all   → backlog task list --plain
intent bl board        → backlog board --plain
intent bl create       → backlog task create [with prefix]
```

**Configuration:**

```json
{
  "backlog_list_status": "todo" // Default filter
}
```

**2. Task Management (`intent_task`)**

The `intent task` command links steel threads to Backlog tasks:

```bash
intent task create ST#### "title"  # Creates with ST prefix
intent task list ST####            # Lists thread's tasks
intent task count ST####           # Task completion stats
```

**3. Status Synchronisation (`intent_status`)**

The `intent status` command synchronizes thread and task states:

```bash
intent status show ST####   # Display status comparison
intent status sync ST####   # Update thread from tasks
intent status check         # Project-wide status report
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

#### File Structure Integration [AS-BUILT]

```
<project_root>/
├── .intent/
│   └── config.json              # Intent configuration
├── intent/
│   └── st/
│       ├── ST0001/              # Steel thread directory
│       │   ├── info.md
│       │   └── tasks.md        # Links to Backlog
│       └── ST0002/
└── backlog/
    ├── Backlog.md               # Main backlog file
    ├── tasks/                   # Task files
    │   ├── task-001.md         # ST0001 - Task title
    │   └── task-002.md
    └── ...
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

### 4.8.2 Migration System [AS-BUILT v2.0.0]

Intent v2.0.0 includes comprehensive migration support for upgrading from any STP version:

#### Migration Command

```bash
intent upgrade [--backup-dir DIR]
```

#### Migration Process

1. **Version Detection**
   - Checks for .stp-config (YAML) or .intent/config.json
   - Identifies version from 0.0.0 to 1.2.1

2. **Backup Creation**
   - Creates timestamped backup directory
   - Preserves entire project state

3. **Structure Migration**

   ```
   stp/prj/st/ST####.md → intent/st/ST####/info.md
   stp/eng/             → intent/docs/eng/
   stp/usr/             → intent/docs/usr/
   stp/llm/             → intent/llm/
   ```

4. **Configuration Conversion**

   ```yaml
   # Old YAML (.stp-config)
   author: username
   editor: vim
   ```

   ```json
   # New JSON (.intent/config.json)
   {
     "version": "2.0.0",
     "author": "username",
     "editor": "vim",
     "intent_dir": "intent",
     "backlog_dir": "backlog"
   }
   ```

5. **YAML Frontmatter Fix**
   - Handles files without frontmatter
   - Preserves existing content

### 4.8.3 Bootstrap System [AS-BUILT v2.0.0]

Global Intent setup for first-time users:

```bash
intent bootstrap [--force]
```

#### Bootstrap Features

1. **INTENT_HOME Detection**
   - Automatically finds installation
   - Validates directory structure

2. **Global Configuration**
   - Creates ~/.config/intent/config.json
   - Sets default values

3. **Shell Integration**
   - Provides PATH setup instructions
   - Detects shell type (bash/zsh/fish)

4. **Validation**
   - Checks all commands accessible
   - Verifies jq availability

### 4.8.4 Diagnostic System [AS-BUILT v2.0.0]

Configuration diagnostics and automatic fixes:

```bash
intent doctor [--fix] [--verbose]
```

#### Diagnostic Checks

1. **Project Detection**
   - Validates .intent/config.json
   - Checks directory structure

2. **Configuration Validation**
   - JSON syntax verification
   - Required field checks
   - Version compatibility

3. **Directory Structure**
   - Ensures intent/ exists
   - Validates backlog/ setup

4. **Dependencies**
   - jq availability
   - Command accessibility

#### Automatic Fixes (--fix)

- Creates missing directories
- Initializes config files
- Fixes JSON formatting
- Updates version fields

## 4.9 AS-BUILT Summary

Intent v2.0.0 represents a complete implementation of the design with significant enhancements:

1. **Unified Command Structure**: All commands follow intent\_\* pattern
2. **JSON Configuration**: Hierarchical config with validation
3. **Enhanced Integration**: Backlog.md with status filtering
4. **User Experience**: Bootstrap, doctor, and upgrade commands
5. **Self-Hosting**: Intent is developed using Intent itself
