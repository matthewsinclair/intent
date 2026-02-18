---
verblock: "17 Feb 2026:v2.4.0: Matthew Sinclair - Updated for Intent v2.4.0"
intent_version: 2.4.0
---

# Reference Guide

This reference guide provides comprehensive information about the Intent system (v2.4.0). Unlike the task-oriented User Guide, this reference guide serves as a complete reference for all aspects of the system.

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

#### `intent upgrade`

Upgrades a project from an older version to Intent v2.4.0.

**Usage:**

```bash
intent upgrade [--backup-dir <dir>] [--no-backup]
```

**Options:**

- `--backup-dir <dir>`: Custom backup directory (default: .intent-backup-TIMESTAMP)
- `--no-backup`: Skip backup creation (not recommended)

**Example:**

```bash
intent upgrade
```

**Output:**

- Creates timestamped backup of existing structure
- Migrates directory structure (legacy paths → intent/\*)
- Converts YAML configuration to JSON
- Updates all file references and frontmatter
- Converts single steel thread files to directory structure
- Reports upgrade status for each step

Example output:

```
Starting upgrade to Intent v2.1.0...

Detected version: 1.2.1
Creating backup at .intent-backup-20250717-123456...
Backup completed.

Migrating directory structure...
  legacy/prj → intent
  legacy/eng → intent/eng
  legacy/usr → intent/usr
  legacy/llm → intent/llm

Converting configuration...
  legacy config (YAML) → .intent/config.json (JSON)

Migrating steel threads...
  Converting ST0001.md → ST0001/info.md
  Converting ST0002.md → ST0002/info.md

Updating file references...
  Updated 15 files with Intent naming

Upgrade complete! Intent v2.1.0 is ready.
Run 'intent doctor' to verify configuration.
```

#### `intent bootstrap`

Bootstraps Intent environment and dependencies.

**Usage:**

```bash
intent bootstrap [--check]
```

**Options:**

- `--check`: Only check requirements without installing

**Example:**

```bash
intent bootstrap
```

**Output:**

- Checks system requirements
- Installs missing dependencies
- Configures shell environment
- Validates Intent installation

#### `intent doctor`

Checks and diagnoses Intent configuration and environment.

**Usage:**

```bash
intent doctor
```

**Example:**

```bash
intent doctor
```

**Output:**

- Validates Intent installation
- Checks project configuration
- Verifies directory structure
- Reports any issues found
- Suggests fixes for common problems

#### `intent init`

Initializes a new Intent project.

**Usage:**

```bash
intent init <project_name> [directory]
```

**Parameters:**

- `project_name`: Name of the project (required)
- `directory`: Target directory (optional, defaults to current directory)

**Example:**

```bash
intent init "My Project" ./my-project
```

**Output:**

- Creates Intent directory structure
- Creates `.intent/config.json` with project configuration
- Initializes `intent/` directories (st/, eng/, usr/, llm/)
- Creates `CLAUDE.md` with project instructions
- Creates `intent/wip.md` for work tracking

#### `intent st`

Manages steel threads.

**Usage:**

```bash
intent st <command> [options] [arguments]
```

**Subcommands:**

`intent st new`

Creates a new steel thread.

**Usage:**

```bash
intent st new <title>
```

**Parameters:**

- `title`: Title of the steel thread (required)

**Example:**

```bash
intent st new "Implement User Authentication"
```

**Output:**

- Creates directory `intent/st/ST####/`
- Creates `info.md` with metadata and template
- Auto-increments thread ID
- Reports: "Created new steel thread: ST####"

`intent st done`

Marks a steel thread as complete.

**Usage:**

```bash
intent st done <id>
```

**Parameters:**

- `id`: ID of the steel thread (required)

**Example:**

```bash
intent st done ST0001
```

`intent st list`

Lists all steel threads.

**Usage:**

```bash
intent st list [--status <status>] [--width <columns>]
```

**Options:**

- `--status`: Filter by status ("Not Started", "In Progress", "Completed")
- `--width`: Set the output table width in columns (optional, defaults to terminal width)

**Example:**

```bash
intent st list --status "In Progress" --width 100
```

**Output:**

```
ID     | Title                                | Status      | Created    | Completed
-------|--------------------------------------|-------------|------------|-----------
ST0003 | Implement Feature X                  | In Progress | 2025-03-08 |
ST0002 | Design Database Schema               | In Progress | 2025-03-07 |
ST0001 | Project Setup                        | Completed   | 2025-03-05 | 2025-03-06
```

`intent st show`

Displays the contents of a steel thread.

**Usage:**

```bash
intent st show <id> [file]
```

**Parameters:**

- `id`: ID of the steel thread (required)
- `file`: Specific file to show (optional: info, design, impl, tasks, results)

**Example:**

```bash
intent st show ST0001
intent st show ST0001 design
```

`intent st edit`

Opens a steel thread file for editing.

**Usage:**

```bash
intent st edit <id> [file]
```

**Parameters:**

- `id`: ID of the steel thread (required)
- `file`: Specific file to edit (optional: info, design, impl, tasks, results)

**Example:**

```bash
intent st edit ST0001
intent st edit ST0001 tasks
```

#### `intent st repair`

Repairs malformed steel thread metadata.

**Usage:**

```bash
intent st repair [id] [--write]
```

**Purpose:**

Fixes common metadata issues in steel threads that may occur after migrations or manual edits:

- Repairs malformed YAML frontmatter (eg escaped newlines)
- Updates legacy field names (stp_version → intent_version)
- Reconciles conflicting status values between frontmatter and body
- Validates and fixes date formats
- Adds missing required fields with sensible defaults

**Parameters:**

- `id`: ID of specific steel thread to repair (optional)
- `--write`: Apply repairs (without this flag, performs dry-run)

**Options:**

- Without `--write`: Shows what would be changed (dry-run mode)
- With `--write`: Actually performs the repairs and organizes files

**Examples:**

```bash
# Dry-run repair on all steel threads
intent st repair

# Actually repair all steel threads
intent st repair --write

# Dry-run repair on specific steel thread
intent st repair ST0001

# Actually repair specific steel thread
intent st repair ST0001 --write
```

**Expected Output (dry-run):**

```
Processing: ST0001
  - Found malformed frontmatter
    Would fix malformed frontmatter
  - Found legacy stp_version field
    Would update to intent_version
  - Found conflicting status:
    Frontmatter: Not Started
    Body: Completed
    Would update frontmatter status to: Completed

Dry run complete. Use --write to apply changes.
```

**Expected Output (with --write):**

```
Processing: ST0001
  - Found malformed frontmatter
    Fixed malformed frontmatter
  - Found legacy stp_version field
    Updated to intent_version
  - Found conflicting status:
    Frontmatter: Not Started
    Body: Completed
    Updated frontmatter status to: Completed

Repairs complete.

Running organize to ensure correct file locations...
Moved ST0001 to intent/st/COMPLETED
Updated steel threads index.
```

#### `intent help`

Displays help information.

**Usage:**

```bash
intent help [command]
```

**Parameters:**

- `command`: Command to get help for (optional)

**Example:**

```bash
intent help st
intent help task
```

#### `intent llm`

Commands for LLM integration and assistance.

**Usage:**

```bash
intent llm <subcommand> [options]
```

**Purpose:**

Provides utilities for working with Large Language Models (LLMs) in the context of Intent. Helps LLMs understand how to use Intent effectively and facilitates better collaboration between developers and AI assistants.

**Subcommands:**

`intent llm usage_rules`

Displays the complete Intent usage patterns and workflows documentation.

**Usage:**

```bash
intent llm usage_rules
```

**Example:**

```bash
# Display usage rules
intent llm usage_rules

# Create symlink in current directory
intent llm usage_rules --symlink

# Create symlink in specific directory
intent llm usage_rules --symlink ~/my-project

# Pipe to less for easier reading
intent llm usage_rules | less

# Save to a file
intent llm usage_rules > intent-usage-rules.md
```

**Options:**

- `--symlink [dir]`: Create a symlink to usage-rules.md in current or specified directory

**Notes:**

- The usage rules document is located at `intent/llm/usage-rules.md`
- It follows the pattern established by the Elixir Hex package 'usage_rules'
- The --symlink option creates a symlink named 'usage-rules.md' for integration with other tools

#### `intent treeindex`

Generates LLM-optimized directory summaries in a shadow directory.

**Usage:**

```bash
intent treeindex [OPTIONS] DIR
```

**Parameters:**

- `DIR`: Directory to index (required)

**Options:**

| Flag            | Description                                           |
| --------------- | ----------------------------------------------------- |
| `-d, --depth N` | Depth to traverse (default: 2)                        |
| `--check`       | Check staleness only, do not generate                 |
| `--prune`       | Remove orphaned .treeindex files (source dir removed) |
| `--force`       | Regenerate ignoring fingerprints                      |
| `--model MODEL` | Claude model to use (default: haiku)                  |
| `--dry-run`     | Show what would be generated without doing it         |
| `-h, --help`    | Show help                                             |

**Example:**

```bash
# Generate summaries for lib/ directory
intent treeindex lib

# Deep scan with forced regeneration
intent treeindex --depth 4 --force src

# CI staleness check
intent treeindex --check lib
```

**Output:**

- Creates/updates `intent/.treeindex/<dir>/.treeindex` summary files
- Auto-creates `.treeindexignore` and `README.md` in `.treeindex/` on first run
- Uses fingerprint-based staleness detection (filenames + sizes)

#### `intent fileindex`

Creates and manages file indexes with checkbox states for tracking progress.

**Usage:**

```bash
intent fileindex [OPTIONS] [STARTDIR] [FILESPEC]
```

**Options:**

| Flag                 | Description                                |
| -------------------- | ------------------------------------------ |
| `-r`                 | Recurse through subdirectories             |
| `-v`                 | Verbose mode (show processing and summary) |
| `-f, --file FILE`    | Output to file instead of stdout           |
| `-i, --index FILE`   | Use index file to maintain checked states  |
| `-X, --toggle FILE`  | Toggle checked state of FILE in index      |
| `-C, --check FILE`   | Set FILE to checked [x] state in index     |
| `-U, --uncheck FILE` | Set FILE to unchecked [ ] state in index   |
| `--index-dir DIR`    | Default directory for index files          |
| `--intent-dir`       | Specify Intent project directory           |
| `--no-intent`        | Disable Intent integration                 |
| `-h`                 | Show help                                  |

**Example:**

```bash
# Generate a file index for the current directory
intent fileindex

# Recursive index of Elixir files
intent fileindex -r lib "*.{ex,exs}"

# Mark a file as checked in an index
intent fileindex -C lib/my_module.ex -i review.md

# Toggle a file's checked state
intent fileindex -X lib/my_module.ex -i review.md
```

**Defaults:**

- In an Intent project: `STARTDIR=lib/`, `FILESPEC=*.{ex,exs}`, `INDEX_DIR=.intent/indexes/`
- Standalone: `STARTDIR=.`, `FILESPEC=*.{ex,exs}`, `INDEX_DIR=.`

#### `intent agents`

Manages AGENTS.md, a universal format for AI agent instructions.

**Usage:**

```bash
intent agents <command> [options]
```

**Subcommands:**

| Command    | Description                                      |
| ---------- | ------------------------------------------------ |
| `init`     | Initialize AGENTS.md for the project             |
| `generate` | Generate/regenerate AGENTS.md from project state |
| `sync`     | Update AGENTS.md with latest project state       |
| `validate` | Validate AGENTS.md against specification         |
| `template` | Manage AGENTS.md templates                       |

**Example:**

```bash
# Initialize AGENTS.md
intent agents init

# Regenerate from current project state
intent agents generate

# Validate structure
intent agents validate

# List available templates
intent agents template list
```

**Output:**

- Creates `intent/llm/AGENTS.md` with project-specific instructions
- Creates a symlink at the project root: `AGENTS.md` -> `intent/llm/AGENTS.md`

#### `intent claude subagents`

Manages Claude Code subagents for Intent projects.

**Usage:**

```bash
intent claude subagents <command> [options]
```

**Subcommands:**

| Command     | Description                                   |
| ----------- | --------------------------------------------- |
| `init`      | Initialize subagent configuration             |
| `list`      | List available and installed subagents        |
| `install`   | Install subagent(s) to Claude configuration   |
| `sync`      | Sync installed subagents with latest versions |
| `uninstall` | Remove Intent-managed subagents               |
| `show`      | Display detailed subagent information         |
| `status`    | Check subagent health and integrity           |

**Options:**

| Flag            | Description                  |
| --------------- | ---------------------------- |
| `--project, -p` | Project-level initialization |
| `--force, -f`   | Force operation              |
| `--all`         | Apply to all subagents       |
| `--verbose, -v` | Verbose output for status    |

**Available Subagents:**

| Name         | Description                                                  |
| ------------ | ------------------------------------------------------------ |
| `intent`     | Intent methodology, steel threads, and project structure     |
| `elixir`     | Elixir code doctor with antipatterns, style, and Ash/Phoenix |
| `socrates`   | CTO Review Mode via Socratic dialog                          |
| `worker-bee` | Worker-Bee Driven Design specialist                          |

**Example:**

```bash
# Initialize subagent configuration
intent claude subagents init

# Install the Intent subagent
intent claude subagents install intent

# Install all subagents
intent claude subagents install --all

# Check subagent health
intent claude subagents status --verbose
```

**Plugin Location:** Subagent definitions live in `intent/plugins/claude/subagents/`.

#### `intent claude skills`

Manages Claude Code skills for Intent projects. Skills are always-on enforcement rules loaded into every session.

**Usage:**

```bash
intent claude skills <command> [options]
```

**Subcommands:**

| Command     | Description                                |
| ----------- | ------------------------------------------ |
| `list`      | List available and installed skills        |
| `install`   | Install skill(s) to `.claude/skills/`      |
| `sync`      | Sync installed skills with latest versions |
| `uninstall` | Remove Intent-managed skills               |
| `show`      | Display skill content and status           |

**Options:**

| Flag          | Description         |
| ------------- | ------------------- |
| `--force, -f` | Force operation     |
| `--all`       | Apply to all skills |

**Available Skills:**

| Name                  | Rules | Description                                    |
| --------------------- | :---: | ---------------------------------------------- |
| `intent-essentials`   |   7   | Intent workflow rules (CLI usage, conventions) |
| `elixir-essentials`   |   8   | Core Elixir rules (pattern matching, pipes)    |
| `ash-ecto-essentials` |   7   | Ash/Ecto rules (code interfaces, migrations)   |
| `phoenix-liveview`    |   7   | LiveView rules (streams, two-phase mount)      |

**Example:**

```bash
# List skills
intent claude skills list

# Install all skills
intent claude skills install --all

# Check for updates
intent claude skills sync

# View skill content
intent claude skills show elixir-essentials
```

**Plugin Location:** Skill definitions live in `intent/plugins/claude/skills/`.
**Install Location:** Skills install to `.claude/skills/<name>/SKILL.md` in the target project.

#### `intent claude upgrade`

Diagnoses and upgrades LLM guidance files in existing Intent projects.

**Usage:**

```bash
intent claude upgrade [options]
```

**Options:**

| Flag                | Description                        |
| ------------------- | ---------------------------------- |
| `--apply`           | Apply changes (default is dry-run) |
| `--project-dir DIR` | Target external project directory  |

**Example:**

```bash
# Dry-run: diagnose and show plan
intent claude upgrade

# Apply upgrade to current project
intent claude upgrade --apply

# Apply upgrade to external project
intent claude upgrade --apply --project-dir /path/to/project
```

**What it does:**

1. Diagnoses current LLM guidance files (AGENTS.md, RULES.md, ARCHITECTURE.md)
2. Detects deprecated files (AGENTS-phx.md, llm_preamble.md, usage-rules.md)
3. Checks subagent and skill installation status
4. Generates a project-specific upgrade plan
5. With `--apply`, executes the plan

#### `intent bl` / `intent backlog`

Intent wrapper for Backlog.md task management.

**Usage:**

```bash
intent bl <command> [options] [arguments]
intent backlog <command> [options] [arguments]
```

**Purpose:**

Provides a streamlined interface to Backlog.md that avoids common issues like git fetch errors and provides shortcuts for Intent workflows. Respects the `backlog_list_status` configuration setting.

**Subcommands:**

`intent bl init`

Initializes Backlog with Intent-friendly settings.

**Usage:**

```bash
intent bl init
```

**Effect:**

- Creates backlog directory structure
- Disables remote operations to prevent git errors
- Sets default status to "To Do"

`intent bl create`

Creates a task linked to a steel thread.

**Usage:**

```bash
intent bl create <ST####> <title>
```

**Parameters:**

- `ST####`: Steel thread ID (required)
- `title`: Task description (required)

**Example:**

```bash
intent bl create ST0014 "Add validation logic"
```

`intent bl list`

Lists all tasks without git fetch errors.

**Usage:**

```bash
intent bl list
```

**Note:** Automatically adds `--plain` flag to prevent git operations.

`intent bl board`

Displays tasks in Kanban board view.

**Usage:**

```bash
intent bl board
```

`intent bl task`

Manages individual tasks.

**Usage:**

```bash
intent bl task <subcommand> [options]
```

**Example:**

```bash
intent bl task edit task-5 --status Done
```

#### `intent task`

Manages Backlog tasks linked to steel threads.

**Usage:**

```bash
intent task <command> [options] [arguments]
```

**Subcommands:**

`intent task create`

Creates a new task linked to a steel thread.

**Usage:**

```bash
intent task create <ST####> <title>
```

**Parameters:**

- `ST####`: Steel thread ID (required)
- `title`: Task description (required)

**Example:**

```bash
intent task create ST0014 "Implement error handling"
```

`intent task list`

Lists all tasks for a specific steel thread.

**Usage:**

```bash
intent task list <ST####>
```

**Parameters:**

- `ST####`: Steel thread ID (required)

**Example:**

```bash
intent task list ST0014
```

**Output:**

```
Tasks for ST0014:
================
task-1       [done]          ST0014 - Create directory structure
task-2       [todo]          ST0014 - Add unit tests
```

`intent task sync`

Synchronizes task status with steel thread.

**Usage:**

```bash
intent task sync <ST####>
```

**Parameters:**

- `ST####`: Steel thread ID (required)

#### `intent status`

Synchronizes steel thread status based on Backlog task completion.

**Usage:**

```bash
intent status <command> [options] [arguments]
```

**Subcommands:**

`intent status show`

Displays status of steel thread and its tasks.

**Usage:**

```bash
intent status show <ST####>
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

`intent status sync`

Updates steel thread status based on task completion.

**Usage:**

```bash
intent status sync <ST####> [--dry-run]
```

**Parameters:**

- `ST####`: Steel thread ID (required)

**Options:**

- `--dry-run`: Preview changes without updating

`intent status report`

Generates status report for all active threads.

**Usage:**

```bash
intent status report
```

#### `intent migrate`

Migrates embedded tasks from steel threads to Backlog.

**Usage:**

```bash
intent migrate [options] <ST####>
```

**Parameters:**

- `ST####`: Steel thread ID to migrate

**Options:**

- `--dry-run`: Preview migration without creating tasks
- `--all-active`: Migrate all active steel threads

**Example:**

```bash
# Migrate a single steel thread
intent migrate ST0014

# Preview migration
intent migrate --dry-run ST0014

# Migrate all active threads
intent migrate --all-active
```

**Effect:**

- Extracts checkbox tasks from steel thread documents
- Creates corresponding Backlog tasks
- Updates steel thread to reference Backlog
- Preserves task completion status

### Additional Commands

#### Test Suite Commands

The Intent test suite provides commands for verifying system functionality:

```bash
# Run all tests
./tests/run_tests.sh

# Run a specific test file
./tests/run_tests.sh tests/unit/treeindex_commands.bats

# Run all unit tests
./tests/run_tests.sh tests/unit/

# Set up test environment
./tests/setup_test_env.sh
```

The test environment setup script installs necessary dependencies, including:

- Bats (Bash Automated Testing System)
- bats-support
- bats-assert
- bats-file

## Document Templates

### Steel Thread Document Format

Steel thread documents (located in `intent/st/ST####/`) use a standardized format with two ways to store metadata:

#### Intent Versioning

Each Intent file includes version information to track compatibility:

```yaml
---
intent_version: 2.4.0
---
```

The version follows semantic versioning (MAJOR.MINOR.PATCH) where:

- MAJOR: Incompatible changes that require manual migration
- MINOR: New features in a backward-compatible manner
- PATCH: Backward-compatible bug fixes

When running `intent upgrade`, the system checks this version to determine what upgrades are needed.

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

The steel_threads.md document uses HTML comment markers to identify sections that can be automatically updated by the `intent st sync` command:

```markdown
<!-- BEGIN: STEEL_THREAD_INDEX -->

(content here will be replaced by sync command)

<!-- END: STEEL_THREAD_INDEX -->
```

These markers should not be removed from the document, as they enable automatic updates while preserving manually edited content outside the marked sections.

### Project Templates

#### Work in Progress (WIP) Template

Location: `intent/wip.md`

Purpose: Tracks current development focus and active steel threads.

Structure:

- Current Focus
- Active Steel Threads
- Upcoming Work
- Notes

#### Steel Thread Templates

Location: `intent/st/`

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

Engineering templates are located in `intent/_templ/eng/`:

- `tpd/`: Technical Product Design templates
  - `_technical_product_design.md`: Main TPD template
  - `_1_introduction.md` through `_8_appendices.md`: Section templates

These templates provide structured formats for capturing technical design decisions and architectural information.

### User Documentation Templates

User documentation templates are located in `intent/_templ/usr/`:

- `_user_guide.md`: Template for task-oriented user instructions
- `_reference_guide.md`: Template for comprehensive reference information
- `_deployment_guide.md`: Template for installation and deployment guidance

### LLM Templates

LLM-specific templates are located in `intent/_templ/llm/`:

- `_llm_preamble.md`: Template for creating context preambles for LLM sessions

## Directory Structure

```
Intent/
├── intent/                # Main Intent directory
│   ├── .treeindex/        # Shadow directory for LLM-oriented summaries
│   │   ├── .treeindexignore  # Files/dirs to exclude from indexing
│   │   └── README.md      # Treeindex orientation for LLMs
│   ├── _templ/            # Templates directory
│   ├── st/                # Steel threads
│   │   ├── ST####/        # Individual steel thread directories
│   │   │   ├── info.md          # Steel thread metadata
│   │   │   ├── design.md        # Design documentation
│   │   │   ├── impl.md          # Implementation notes
│   │   │   └── tasks.md         # Task tracking
│   │   ├── COMPLETED/           # Completed steel threads
│   │   ├── NOT-STARTED/         # Not started steel threads
│   │   └── CANCELLED/           # Cancelled steel threads
│   ├── wip.md             # Work in progress
│   ├── eng/               # Engineering docs
│   │   └── tpd/           # Technical Product Design
│   ├── usr/               # User documentation
│   ├── llm/               # LLM-specific content
│   │   ├── AGENTS.md      # Universal AI agent instructions (auto-generated)
│   │   ├── RULES.md       # Coding rules and conventions (human-curated)
│   │   └── ARCHITECTURE.md # System architecture (human-curated)
│   └── plugins/           # Plugin architecture
│       ├── agents/        # AGENTS.md plugin
│       │   ├── bin/       # Plugin scripts
│       │   └── templates/ # AGENTS.md templates (default, elixir)
│       └── claude/        # Claude Code integration
│           ├── bin/       # Plugin scripts
│           ├── skills/    # Skill definitions
│           │   ├── intent-essentials/
│           │   ├── elixir-essentials/
│           │   ├── ash-ecto-essentials/
│           │   └── phoenix-liveview/
│           └── subagents/ # Subagent definitions
│               ├── intent/
│               ├── elixir/
│               ├── socrates/
│               └── worker-bee/
├── bin/                   # Intent scripts (executable)
├── tests/                 # Test suite
│   ├── unit/              # Unit tests (15 .bats files)
│   ├── integration/       # Integration tests
│   ├── lib/               # Test helper libraries
│   ├── fixtures/          # Test fixtures
│   └── run_tests.sh       # Test runner script
├── .intent/               # Intent configuration
│   └── config.json        # Project configuration
└── backlog/               # Backlog.md task management
    ├── tasks/             # Active tasks
    ├── drafts/            # Draft tasks
    ├── archive/           # Archived tasks
    └── config.yml         # Backlog configuration
```

## Configuration Options

### Environment Variables

| Variable       | Purpose                         | Default                           |
| -------------- | ------------------------------- | --------------------------------- |
| INTENT_HOME    | Location of Intent installation | Path to cloned repository         |
| INTENT_PROJECT | Current project name            | Determined from initialization    |
| INTENT_AUTHOR  | Default author name             | Determined from git configuration |
| INTENT_EDITOR  | Preferred text editor           | Determined from system defaults   |

### Project Configuration

Location: `.intent/config.json`

Format: JSON configuration file

Example:

```json
{
  "project_name": "Project Name",
  "author": "Default Author",
  "intent_version": "2.4.0",
  "st_prefix": "ST"
}
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

- Use `intent bl` wrapper instead of `backlog` directly to avoid git errors
- Create tasks linked to steel threads for traceability
- Keep tasks granular (1-2 days of work)
- Regularly sync steel thread status with task completion
- Use task status values: "To Do", "In Progress", "Done"
- Migrate existing embedded tasks using `intent migrate`

### LLM Collaboration

- Share relevant context at the beginning of each session
- Use steel thread documents to maintain context across sessions
- Create canned prompts for common tasks
- Have the LLM update documentation as work progresses
- Provide clear instructions for specific tasks

## Concepts and Terminology

| Term            | Definition                                                                      |
| --------------- | ------------------------------------------------------------------------------- |
| Steel Thread    | A self-contained unit of work representing a logical piece of functionality     |
| LLM             | Large Language Model, an AI system capable of understanding and generating text |
| Context Window  | The amount of text an LLM can process in a single interaction                   |
| Canned Prompt   | A pre-defined, reusable instruction template for an LLM                         |
| WIP             | Work in Progress, a document tracking current development focus                 |
| Backlog         | Task management system integrated with Intent for fine-grained work tracking    |
| Task            | Individual unit of work linked to a steel thread, tracked in Backlog            |
| Task Status     | State of a task: "To Do", "In Progress", or "Done"                              |
| Treeindex       | LLM-oriented directory summaries stored in a shadow directory                   |
| Fileindex       | File tracking tool with checkbox states for progress management                 |
| AGENTS.md       | Universal AI agent instructions file for any AI platform                        |
| Subagent        | A Claude Code sub-agent with domain-specific expertise                          |
| Plugin          | An extension module for Intent (eg agents, claude)                              |
| Skill           | An always-on Claude Code enforcement file installed to `.claude/skills/`        |
| RULES.md        | Human-curated coding rules and conventions file in `intent/llm/`                |
| ARCHITECTURE.md | Human-curated system architecture description in `intent/llm/`                  |

## Backlog.md Integration

This section provides comprehensive documentation for the integration between Intent and Backlog.md for enhanced task management. The integration maintains Intent's strength in intent capture while leveraging Backlog.md's powerful task tracking capabilities.

### Overview

The integration between Intent and Backlog.md provides:

- **Intent Capture**: Steel threads for high-level objectives and context
- **Task Management**: Backlog for granular task tracking with rich metadata
- **Status Synchronisation**: Automatic updates between systems
- **Workflow Integration**: Seamless development workflows

### Architecture

#### Intent Responsibilities

- **Intent Capture**: High-level objectives and context in steel thread documents
- **Design Documentation**: Detailed design specifications (ST####/design.md)
- **Implementation Records**: As-built documentation (ST####/impl.md)
- **Process Coordination**: Overall workflow and steel thread lifecycle

#### Backlog.md Responsibilities

- **Task Management**: Individual task tracking with rich metadata
- **Status Tracking**: Granular task states (draft, todo, in-progress, done)
- **Task Organisation**: Labels, priorities, dependencies, and subtasks
- **Visualisation**: Kanban board and browser interface

### Using the Intent Backlog Wrapper

Intent provides a wrapper command `intent backlog` (or `intent bl` for short) that streamlines Backlog usage:

```bash
# Initialize backlog with Intent-friendly settings
intent bl init

# List tasks without git fetch errors
intent bl list

# Create tasks linked to steel threads
intent bl create ST0014 "Add validation"

# View Kanban board
intent bl board

# Zero-pad task IDs retroactively
intent bl task pad task-9 --size 3     # Pad single task to task-009
intent bl task pad --all --size 3      # Pad all tasks to 3 digits
intent bl task pad --all               # Pad using configured size
```

The wrapper automatically:

- Adds `--plain` to list/board commands to prevent git errors
- Disables remote operations for local projects
- Provides shortcuts for common workflows

#### Task ID Padding

The `intent bl task pad` command allows you to retroactively zero-pad task IDs for consistent sorting and display:

```bash
# Pad a specific task
intent bl task pad task-9 --size 3    # Changes task-9 to task-009

# Pad all tasks to 3 digits
intent bl task pad --all --size 3

# Use configured padding (reads from zeroPaddedIds config)
intent bl task pad --all
```

This command:

- Updates both the task filename and the `id:` field in the YAML frontmatter
- Processes tasks in both `backlog/tasks/` and `backlog/archive/tasks/`
- Only pads tasks that need it (skips already padded tasks)
- Is idempotent - running it multiple times is safe

After padding, ensure new tasks use the same padding by setting:

```bash
intent bl config set zeroPaddedIds 3
```

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

- Steel thread documents remain in `/intent/st/`
- Backlog tasks are stored in `/backlog/tasks/`
- Task files are named: `task-<id> - <title>.md`

### Workflow Integration

#### 1. Creating a New Steel Thread

```bash
# Create the steel thread
intent st new "My New Feature"
# Returns: Created ST0015

# Create associated tasks using the backlog wrapper
intent bl create ST0015 "Design API structure"
intent bl create ST0015 "Implement core logic"
intent bl create ST0015 "Create registration flow"
intent bl create ST0015 "Add session management"
intent bl create ST0015 "Write integration tests"
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

Use `intent status` to sync:

```bash
intent status sync ST0015
```

#### 4. Viewing Tasks

```bash
# View all tasks for a steel thread
intent task list ST0015

# View all tasks without git errors
intent bl list

# View in Kanban board
intent bl board

# View in browser
intent bl browser
```

### Steel Thread Document Structure

With Backlog integration, steel thread documents focus on intent and context:

```markdown
---
verblock: "08 Jul 2025:v0.1: Author Name - Initial version"
intent_version: 2.3.4
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

Tasks are tracked in Backlog. View with: `intent task list ST0015`

## Implementation Notes

Key technical decisions and learnings

## Results

Outcomes and metrics (completed threads)
```

### Migration from Embedded Tasks

For existing steel threads with embedded task lists:

```bash
# Migrate a specific steel thread
intent migrate ST0014

# Migrate all active threads
intent migrate --all-active
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

- Run `intent status sync` regularly
- Archive completed tasks weekly
- Review and promote drafts in planning sessions

### Workflow Examples

#### New Feature Development

```bash
# 1. Create steel thread for high-level planning
intent st new "Implement user authentication"
# Output: Created ST0015

# 2. Create implementation tasks
intent task create ST0015 "Design auth database schema"
intent task create ST0015 "Implement login endpoint"
intent task create ST0015 "Create registration flow"
intent task create ST0015 "Add session management"
intent task create ST0015 "Write integration tests"

# 3. Work through tasks
intent bl board                      # View Kanban board
backlog task edit <id> --status in-progress

# 4. Sync status back to steel thread
intent status sync ST0015
```

#### Research and Design

```bash
# 1. Create steel thread for research
intent st new "Research caching strategies"

# 2. Create investigation tasks
intent task create ST0016 "Review Redis capabilities"
intent task create ST0016 "Benchmark Memcached performance"
intent task create ST0016 "Evaluate in-memory options"
intent task create ST0016 "Document recommendations"

# 3. Track progress
intent task list ST0016
```

#### Bug Fix Workflow

```bash
# 1. Create steel thread for bug
intent st new "Fix authentication timeout issue"

# 2. Create diagnostic and fix tasks
intent task create ST0017 "Reproduce timeout issue"
intent task create ST0017 "Debug session handling"
intent task create ST0017 "Implement fix"
intent task create ST0017 "Add regression test"

# 3. Fast status check
intent status show ST0017
```

### Troubleshooting

#### Common Issues

1. **Task ID Conflicts**
   - Backlog assigns unique IDs automatically
   - Don't manually edit task IDs

2. **Status Mismatch**
   - Run `intent status sync` to update
   - Check for tasks in unexpected states

3. **Missing Tasks**
   - Check drafts folder
   - Verify task wasn't archived

4. **Git Fetch Errors**
   - Use `intent bl` wrapper instead of `backlog` directly
   - The wrapper adds `--plain` flag automatically

#### Getting Help

- Run `intent help` for Intent commands
- Run `intent help backlog` for Intent's Backlog wrapper
- Run `backlog help` for native Backlog commands

### Testing

The integration includes comprehensive test coverage:

```bash
# Run all tests
./tests/run_tests.sh

# Run specific test files
bats tests/unit/task_commands.bats
bats tests/unit/bl_commands.bats
bats tests/unit/project_commands.bats
```

Test files are located in `tests/unit/`:

- `task_commands.bats` - Task management command tests
- `bl_commands.bats` - Backlog wrapper command tests
- `project_commands.bats` - Project-specific command tests (including status and migration)
