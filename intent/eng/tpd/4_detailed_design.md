---
verblock: "20 Feb 2026:v2.4.0: Matthew Sinclair - Updated for Intent v2.4.0"
intent_version: 2.4.0
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
├── bin/                        # Intent scripts
│   ├── intent                 # Main command
│   ├── intent_st              # Steel thread commands
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
  "intent_dir": "intent"
}
```

#### Configuration Fields

| Field        | Description         | Default    | Added  |
| ------------ | ------------------- | ---------- | ------ |
| version      | Intent version      | 2.0.0      | v2.0.0 |
| project_name | Project identifier  | (required) | v2.0.0 |
| author       | Default author      | $USER      | v0.0.0 |
| created      | Creation date       | (auto)     | v2.0.0 |
| st_prefix    | Steel thread prefix | ST         | v1.0.0 |
| intent_dir   | Intent directory    | intent     | v2.0.0 |

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

| Command          | Description                    | Added  |
| ---------------- | ------------------------------ | ------ |
| init             | Initialize Intent in a project | v0.0.0 |
| st               | Manage steel threads           | v0.0.0 |
| wp               | Manage work packages           | v2.6.0 |
| bootstrap        | Global Intent setup            | v2.0.0 |
| doctor           | Diagnose configuration issues  | v2.0.0 |
| agents           | Manage AGENTS.md               | v2.3.0 |
| claude subagents | Manage Claude Code subagents   | v2.3.0 |
| claude skills    | Manage Claude Code skills      | v2.4.0 |
| claude upgrade   | Upgrade project LLM guidance   | v2.4.0 |
| treeindex        | Generate directory summaries   | v2.4.0 |
| fileindex        | Generate file summaries        | v2.4.0 |
| help             | Display comprehensive help     | v0.0.0 |

#### Steel Thread Subcommands

```
intent st new [title]         # Create new steel thread
intent st list [--status X]   # List threads with filtering
intent st show ST####         # Display thread contents
intent st edit ST#### [file]  # Edit thread files
```

#### Work Package Subcommands

```
intent wp new <STID> "Title"  # Create new work package
intent wp list <STID>         # List work packages for a thread
intent wp show <STID/NN>      # Display work package contents
intent wp start <STID/NN>     # Mark work package as WIP
intent wp done <STID/NN>      # Mark work package as Done
```

#### New v2.0.0 Features

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

| Script                                     | Purpose              | Key Features                     |
| ------------------------------------------ | -------------------- | -------------------------------- |
| bin/intent                                 | Main entry point     | Command dispatch, plugin routing |
| bin/intent_config                          | Config management    | JSON parsing, hierarchy support  |
| bin/intent_helpers                         | Utilities            | Version detection, shared funcs  |
| bin/intent_st                              | Steel threads        | Create, list, show, edit         |
| bin/intent_treeindex                       | Directory summaries  | Shadow dir, LLM-generated        |
| bin/intent_fileindex                       | File summaries       | Single-file analysis             |
| plugins/agents/bin/intent_agents           | AGENTS.md management | Init, generate, sync, validate   |
| plugins/claude/bin/intent_claude_subagents | Subagent lifecycle   | Install, sync, uninstall, show   |
| plugins/claude/bin/intent_claude_skills    | Skill lifecycle      | Install, sync, uninstall, show   |
| plugins/claude/bin/intent_claude_upgrade   | Project upgrade      | Diagnose, plan, execute          |

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

### 4.8.1 Migration System [AS-BUILT v2.0.0]

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
     "intent_dir": "intent"
   }
   ```

5. **YAML Frontmatter Fix**
   - Handles files without frontmatter
   - Preserves existing content

### 4.8.2 Bootstrap System [AS-BUILT v2.0.0]

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

### 4.8.3 Diagnostic System [AS-BUILT v2.0.0]

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

4. **Dependencies**
   - jq availability
   - Command accessibility

#### Automatic Fixes (--fix)

- Creates missing directories
- Initializes config files
- Fixes JSON formatting
- Updates version fields

## 4.9 AS-BUILT Summary

Intent v2.4.0 represents a complete implementation with significant enhancements beyond the original design:

1. **Plugin Architecture**: Extensible command system under `intent/plugins/`
2. **Skills System**: 6 built-in skills for proactive code shaping and session analysis
3. **Subagent System**: 5 built-in subagents for on-demand review
4. **Treeindex**: Pre-computed directory summaries for codebase navigation
5. **AGENTS.md Management**: Project-level LLM guidance generation
6. **Claude Upgrade**: Automated project modernization tool
7. **302 Tests**: Comprehensive BATS test coverage across 15 files
8. **Self-Hosting**: Intent is developed using Intent itself, proven across 8 projects
