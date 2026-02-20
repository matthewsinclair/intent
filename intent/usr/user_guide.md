---
verblock: "17 Feb 2026:v2.4.0: Matthew Sinclair - Updated to Intent v2.4.0"
intent_version: 2.4.0
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
8. [Treeindex](#treeindex)
9. [AGENTS.md](#agentsmd)
10. [Claude Subagent Management](#claude-subagent-management)
11. [Claude Skills Management](#claude-skills-management)
12. [LLM Guidance Upgrade](#llm-guidance-upgrade)
13. [Testing](#testing)
14. [Troubleshooting](#troubleshooting)

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

This creates a new steel thread directory (eg `intent/st/NOT-STARTED/ST0001/`) with an `info.md` file containing metadata and an auto-generated slug.

To create and immediately start a steel thread (skips NOT-STARTED, goes straight to WIP):

```bash
intent st new -s "Quick Fix"
```

Titles can contain special characters like `/`, `&`, and `\` safely.

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

Backlog can use zero-padded task IDs (eg task-001 instead of task-1) for better sorting. To retroactively update existing tasks:

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

### Three-File LLM Guidance System

Intent v2.4.0 uses a rationalized three-file system for LLM guidance:

| File              | Purpose                        | Management     |
| ----------------- | ------------------------------ | -------------- |
| `AGENTS.md`       | Factual project overview       | Auto-generated |
| `RULES.md`        | Mandatory coding rules         | Human-curated  |
| `ARCHITECTURE.md` | System structure and decisions | Human-curated  |

These files live in `intent/llm/`. AGENTS.md is managed by `intent agents sync`. RULES.md and ARCHITECTURE.md are human-curated and never overwritten by Intent commands.

### Usage Rules

Intent ships with `usage-rules.md` in the project root -- an LLM-optimized reference for how to use Intent itself. This file follows the pattern established by `deps/ash/usage-rules.md` in Ash projects.

### Contextualizing Work with Steel Threads

When working with an LLM on a specific steel thread:

```bash
# Share the steel thread document with the LLM
intent st show ST0001 | [send to LLM]
```

This provides the LLM with task-specific context for more effective collaboration.

## Treeindex

Intent v2.3.4 includes the `treeindex` command for generating LLM-optimized directory summaries. These summaries let Claude (or any LLM) quickly orient itself in a codebase without reading every file.

### What is Treeindex?

Treeindex creates a shadow directory at `intent/.treeindex/` that mirrors your project structure. Each directory gets a `.treeindex` file containing a concise summary of the directory's contents, purpose, and key files. These summaries are generated using Claude AI.

### Generating Directory Summaries

```bash
# Generate summaries for a directory tree
intent treeindex lib

# Control traversal depth (default: 2)
intent treeindex --depth 3 src

# Preview what would be generated without writing
intent treeindex --dry-run lib

# Force regeneration regardless of staleness
intent treeindex --force lib
```

### Checking and Maintaining Summaries

```bash
# Check which summaries are stale (useful in CI)
intent treeindex --check lib

# Remove orphaned shadow entries (source directory was deleted)
intent treeindex --prune lib
```

### How It Works

- **Shadow directory**: Summaries are stored in `intent/.treeindex/`, keeping your source tree clean
- **Fingerprint-based staleness**: Uses filenames and sizes (not timestamps) for git-clone-stable detection
- **Bottom-up indexing**: Processes leaf directories first, building context upward
- **Auto-created files**: `.treeindexignore` and `README.md` are created in `.treeindex/` on first run

### CLAUDE.md Convention

Add this to your project's `CLAUDE.md` to help Claude use treeindex summaries:

> Before exploring an unfamiliar directory, check `intent/.treeindex/<dir>/.treeindex` for an existing summary.

## AGENTS.md

Intent v2.3.0 introduced AGENTS.md, a universal format for AI agent instructions that works across AI platforms (Claude, GPT, Gemini, etc.).

### What is AGENTS.md?

AGENTS.md is a standardized markdown file that captures project-specific instructions for AI assistants. Unlike Claude-specific subagents, AGENTS.md works with any AI tool that reads project documentation.

### Setting Up AGENTS.md

```bash
# Initialize AGENTS.md for your project
intent agents init

# Generate/regenerate AGENTS.md from project state
intent agents generate

# Update AGENTS.md with latest project state
intent agents sync

# Validate AGENTS.md against specification
intent agents validate
```

The `intent agents init` command creates `intent/llm/AGENTS.md` and a symlink at the project root for discoverability.

### Managing Templates

```bash
# List available AGENTS.md templates
intent agents template list

# Show a specific template
intent agents template show default
```

## Claude Subagent Management

Intent v2.3.0 introduced a plugin architecture and renamed the old `intent agents` commands to `intent claude subagents`. Claude subagents are specialized AI assistants with focused knowledge that integrate directly with Claude Code.

### Available Subagents

| Subagent   | Description                                                  |
| ---------- | ------------------------------------------------------------ |
| intent     | Intent methodology, steel threads, and project structure     |
| elixir     | Elixir code doctor with Usage Rules and Ash/Phoenix patterns |
| socrates   | CTO Review Mode via Socratic dialog                          |
| worker-bee | Worker-Bee Driven Design specialist for Elixir applications  |
| diogenes   | Elixir Test Architect via Socratic dialog                    |

### Setting Up Subagents

#### Initializing Configuration

Before installing subagents, initialize the configuration:

```bash
# Initialize global subagent configuration
intent claude subagents init

# Initialize project-specific configuration
intent claude subagents init --project
```

#### Installing Subagents

```bash
# List available subagents
intent claude subagents list

# Install a specific subagent
intent claude subagents install intent

# Install all available subagents
intent claude subagents install --all
```

#### Verifying Installation

```bash
# Check subagent health and integrity
intent claude subagents status

# Show detailed subagent information
intent claude subagents show intent
```

### Managing Subagents

#### Keeping Subagents Updated

```bash
# Sync installed subagents with latest versions
intent claude subagents sync

# Check for modifications
intent claude subagents status
```

#### Removing Subagents

```bash
# Remove a specific subagent
intent claude subagents uninstall intent

# Remove all Intent-managed subagents
intent claude subagents uninstall --all
```

### Using Subagents with Claude

Once installed, subagents automatically provide Claude with domain-specific expertise:

- **intent**: Complete knowledge of Intent commands, steel thread methodology, and project structure
- **elixir**: Elixir coding patterns, antipattern detection, style guidance, and Ash/Phoenix expertise
- **socrates**: Facilitates Socratic dialog for technical decision-making and architecture review
- **worker-bee**: Worker-Bee Driven Design (WDD) validation, scaffolding, and compliance checking

### Subagent vs AGENTS.md

| Feature      | Claude Subagents                   | AGENTS.md                    |
| ------------ | ---------------------------------- | ---------------------------- |
| Platform     | Claude Code only                   | Any AI assistant             |
| Location     | `intent/plugins/claude/subagents/` | `intent/llm/AGENTS.md`       |
| Command      | `intent claude subagents <cmd>`    | `intent agents <cmd>`        |
| Scope        | Deep, domain-specific expertise    | General project instructions |
| Installation | Installed to `~/.claude/agents/`   | Symlinked to project root    |

### Troubleshooting Subagents

#### Subagent Not Found

```bash
# Check if Claude Code is installed
which claude

# Verify Claude agents directory exists
ls ~/.claude/agents/
```

#### Subagent Out of Sync

```bash
# Check for local modifications
intent claude subagents status

# Sync with latest versions (overwrites local changes)
intent claude subagents sync
```

#### Reinstalling Subagents

```bash
# Remove and reinstall
intent claude subagents uninstall intent
intent claude subagents install intent
```

## Claude Skills Management

Intent v2.4.0 introduces skills -- always-on Claude Code enforcement rules that shape code as it is generated. Unlike subagents (which review after the fact), skills are proactive.

### Available Skills

| Skill                        | Rules | Focus                                          |
| ---------------------------- | :---: | ---------------------------------------------- |
| `intent-essentials`          |   7   | CLI usage, treeindex, steel thread conventions |
| `intent-elixir-essentials`   |   8   | Pattern matching, tagged tuples, pipes, naming |
| `intent-ash-ecto-essentials` |   7   | Code interfaces, migrations, actor placement   |
| `intent-phoenix-liveview`    |   7   | Two-phase mount, streams, components           |
| `intent-elixir-testing`      |   8   | Strong assertions, no control flow, spec-driven|

### Installing Skills

```bash
# List available and installed skills
intent claude skills list

# Install a specific skill
intent claude skills install intent-elixir-essentials

# Install all available skills
intent claude skills install --all
```

### Managing Skills

```bash
# Check for updates
intent claude skills sync

# View skill content
intent claude skills show intent-elixir-essentials

# Remove a skill
intent claude skills uninstall intent-elixir-essentials
```

Skills install to `.claude/skills/<name>/SKILL.md` in the target project and are loaded into every Claude Code session automatically.

## LLM Guidance Upgrade

Intent v2.4.0 includes an upgrade command for migrating existing projects to the new LLM guidance structure.

### Running the Upgrade

```bash
# Dry-run: diagnose and show upgrade plan
intent claude upgrade

# Apply the upgrade
intent claude upgrade --apply

# Target a different project
intent claude upgrade --apply --project-dir /path/to/project
```

### What Gets Upgraded

- Stale AGENTS.md files are regenerated
- Missing RULES.md and ARCHITECTURE.md are created from templates (Elixir projects)
- Deprecated files (AGENTS-phx.md, llm_preamble.md) are flagged for removal
- Subagents and skills are installed/updated

### Elixir Project Templates

For Elixir projects, initialize the three-file system from templates:

```bash
intent agents init --template elixir
```

This creates AGENTS.md, RULES.md, and ARCHITECTURE.md with Elixir-specific defaults.

## Testing

Intent includes a comprehensive test suite (302 tests across 15 files) to verify functionality:

### Running Tests

To run the test suite:

```bash
# Run all tests
./tests/run_tests.sh

# Run a specific test file
./tests/run_tests.sh tests/unit/treeindex_commands.bats

# Run all unit tests
./tests/run_tests.sh tests/unit/
```

### Test Structure

Tests are organized in `tests/unit/` with 15 test files covering all commands:

| Test File                 | Tests                                                 |
| ------------------------- | ----------------------------------------------------- |
| `agent_commands.bats`     | AGENTS.md management (init, generate, sync, validate) |
| `basic.bats`              | Basic infrastructure and environment                  |
| `bl_commands.bats`        | Backlog wrapper commands                              |
| `bootstrap.bats`          | Bootstrap command                                     |
| `config.bats`             | Configuration and PROJECT_ROOT detection              |
| `fileindex_commands.bats` | Fileindex (file tracking and checkbox states)         |
| `global_commands.bats`    | Global commands (help, doctor, info, etc.)            |
| `help_commands.bats`      | Help system                                           |
| `init_commands.bats`      | Init command                                          |
| `migration.bats`          | Migration and backup                                  |
| `project_commands.bats`   | Project-specific commands                             |
| `st_commands.bats`        | Steel thread management                               |
| `skills_commands.bats`    | Skills management (install, sync, uninstall, show)    |
| `task_commands.bats`      | Task management                                       |
| `treeindex_commands.bats` | Treeindex (directory summaries)                       |

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
