---
verblock: "05 Mar 2026:v2.6.0: Matthew Sinclair - Updated to Intent v2.6.0"
intent_version: 2.6.0
---

# User Guide

This user guide provides task-oriented instructions for using the Intent system. It explains how to accomplish common tasks and provides workflow guidance.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Working with Steel Threads](#working-with-steel-threads)
5. [Documentation Management](#documentation-management)
6. [LLM Collaboration](#llm-collaboration)
7. [Treeindex](#treeindex)
8. [AGENTS.md](#agentsmd)
9. [Claude Subagent Management](#claude-subagent-management)
10. [Claude Skills Management](#claude-skills-management)
11. [LLM Guidance Upgrade](#llm-guidance-upgrade)
12. [Code Quality](#code-quality)
13. [Module Guardrails](#module-guardrails)
14. [ST Zero Retrofit](#st-zero-retrofit)
15. [Plugin Discovery](#plugin-discovery)
16. [Testing](#testing)
17. [Troubleshooting](#troubleshooting)

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

```

This creates the Intent directory structure with template documents.

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
└── intent/.config/         # Configuration
    └── config.json         # Intent configuration
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

### Working with Work Packages

Work packages (WPs) break a steel thread into smaller units of work. Each WP lives in a numbered subdirectory under `STXXXX/WP/NN/`.

#### Creating Work Packages

```bash
# Create a work package for ST0005
intent wp new ST0005 "Implement core logic"

# Create another (auto-assigns next number)
intent wp new ST0005 "Write tests"

# Use bare number shorthand
intent wp new 5 "Update docs"
```

Titles can contain special characters like `/`, `&`, and `\` safely. The WP info.md is generated from the template at `lib/templates/prj/st/WP/info.md`.

#### Managing Work Package Status

```bash
# Start working on a WP
intent wp start ST0005/01

# Mark as done
intent wp done ST0005/01
```

When the last WP is marked done, you'll see a hint to complete the steel thread.

#### Viewing Work Packages

```bash
# List all WPs for a steel thread
intent wp list ST0005

# Show a specific WP's info
intent wp show ST0005/01
```

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

| Skill                    | Rules | Focus                                           |
| ------------------------ | :---: | ----------------------------------------------- |
| `in-essentials`          |   7   | CLI usage, treeindex, steel thread conventions  |
| `in-elixir-essentials`   |   8   | Pattern matching, tagged tuples, pipes, naming  |
| `in-ash-ecto-essentials` |   7   | Code interfaces, migrations, actor placement    |
| `in-phoenix-liveview`    |   7   | Two-phase mount, streams, components            |
| `in-elixir-testing`      |   8   | Strong assertions, no control flow, spec-driven |
| `in-autopsy`             |  --   | Session forensics, memory meta-learning         |
| `in-start`               |  --   | Session start: read restart files, orientation  |
| `in-plan`                |  --   | Planning kickoff: show workplan, invoke skills  |
| `in-standards`           |  --   | Coding standards: re-read rules, enforce checks |
| `in-next`                |  --   | Next step: review state, identify work unit     |
| `in-finish`              |  --   | Session finish: update docs, commit cleanly     |

### Installing Skills

```bash
# List available and installed skills
intent claude skills list

# Install a specific skill
intent claude skills install in-elixir-essentials

# Install all available skills
intent claude skills install --all
```

### Managing Skills

```bash
# Check for updates
intent claude skills sync

# View skill content
intent claude skills show in-elixir-essentials

# Remove a skill
intent claude skills uninstall in-elixir-essentials
```

Skills install to `.claude/skills/<name>/` in the user's home directory. The entire skill directory is copied, including any supporting scripts. Skills are loaded into every Claude Code session automatically.

> **Note:** The `in-autopsy` skill requires Elixir to be installed. Run `intent doctor` to check.

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

## Code Quality

Intent v2.6.0 includes automated code quality enforcement for Elixir projects.

### Quick Audit

Run targeted code quality checks using custom Credo rules:

```bash
# Run all custom checks
intent audit quick

# Run a specific rule
intent audit quick --rule R2

# Get machine-readable output
intent audit quick --json

# Auto-fix issues where possible
intent audit quick --fix
```

Available rules:

| Rule | Check                    | Description                               |
| ---- | ------------------------ | ----------------------------------------- |
| R2   | Thick coordinator        | Controllers/LiveViews with business logic |
| R6   | Highlander suspect       | Potential code duplication                |
| R7   | Map.get on struct        | Unsafe struct field access                |
| R11  | Missing @impl annotation | Callback without @impl true               |
| R15  | Debug artifacts          | IO.inspect, dbg() left in code            |
| R16  | Bracket access on struct | Bracket access on struct variable         |

### Health Check

Run a comprehensive project health assessment:

```bash
# Run health check
intent audit health

# Save report to intent/audit/
intent audit health --report

# Check only files changed since last check
intent audit health --diff
```

Health checks: MODULES.md coverage, thick coordinators, Highlander suspects, Credo status. Umbrella-aware -- scans `apps/*/lib/` in umbrella projects.

### Capturing Learnings

Record project learnings for future sessions:

```bash
# Record a footgun (default category)
intent learn "Never use Map.get on structs"

# Record what worked
intent learn --category worked "Ash code interfaces simplify everything"

# Record what failed
intent learn --category failed "Tried raw Ecto, broke the dependency graph"

# List all learnings
intent learn --list
```

Learnings are stored in `intent/.config/learnings.md` and automatically included by `intent claude prime`.

## Module Guardrails

Intent v2.6.0 provides module registry guardrails via `intent modules`.

### Checking the Registry

Compare MODULES.md against actual project files:

```bash
# Check for unregistered files and stale entries
intent modules check

# Interactively register missing files
intent modules check --register
```

Output uses `+` for unregistered files and `-` for stale entries. Exit code 0 means clean, 1 means issues found.

### Searching the Registry

Find the canonical module for a concern:

```bash
intent modules find "auth"
intent modules find "helpers"
```

### Dependency Graph

For Elixir umbrella apps, declare allowed dependencies in `intent/llm/DEPENDENCY_GRAPH.md`:

```markdown
| App  | May depend on | Must NOT depend on |
| ---- | ------------- | ------------------ |
| core | (nothing)     | web, cli, workers  |
| web  | core          | cli, workers       |
```

Enforce with `intent audit quick --rule D11`. A template is available at `lib/templates/llm/_DEPENDENCY_GRAPH.md`.

### Claude Code Hook

An advisory hook template is available at `lib/templates/hooks/module_check_hook.json`. Copy it into your `.claude/settings.json` to get warnings when writing to unregistered module paths.

## ST Zero Retrofit

Intent v2.6.0 includes a retrofit command for installing ST0000 deliverables into existing (brownfield) projects.

### For Existing Projects

```bash
# Audit what's present/missing/partial
intent st zero install --audit-only

# Preview what would be installed
intent st zero install --dry-run

# Install missing deliverables
intent st zero install

# Install a specific deliverable only
intent st zero install --deliverable D3
```

The command checks 9 ST0000 deliverables (D2-D11): CLAUDE.md, MODULES.md, ARCHETYPES.md, Credo check templates, DECISION_TREE.md, MEMORY.md, module check hook, learnings.md, and DEPENDENCY_GRAPH.md.

Elixir-specific deliverables (ARCHETYPES.md, Credo checks, DEPENDENCY_GRAPH.md) are only installed when `mix.exs` is present. CLAUDE.md is never overwritten -- only a diff of missing sections is shown.

For umbrella projects, module auto-discovery scans `apps/*/lib/` and generates per-app sections in MODULES.md.

### For New Projects

```bash
# Initialize project with all ST0000 deliverables
intent init "My Project" --with-st0000
```

This runs the standard `intent init` followed by `intent st zero install`, giving new projects the full code quality enforcement stack from day one.

## Plugin Discovery

Intent v2.6.0 includes a plugin discovery system:

```bash
# List all plugins and their commands
intent plugin

# Show details for a specific plugin
intent plugin show claude
```

Plugins are defined by `plugin.json` files in `intent/plugins/*/`.

## Testing

Intent includes a comprehensive test suite to verify functionality:

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

Tests are organized in `tests/unit/` with 22 test files covering all commands:

| Test File                 | Tests                                                 |
| ------------------------- | ----------------------------------------------------- |
| `agent_commands.bats`     | AGENTS.md management (init, generate, sync, validate) |
| `audit_commands.bats`     | Audit command (quick, health)                         |
| `basic.bats`              | Basic infrastructure and environment                  |
| `bootstrap.bats`          | Bootstrap command                                     |
| `config.bats`             | Configuration and PROJECT_ROOT detection              |
| `fileindex_commands.bats` | Fileindex (file tracking and checkbox states)         |
| `global_commands.bats`    | Global commands (help, doctor, info, etc.)            |
| `help_commands.bats`      | Help system                                           |
| `init_commands.bats`      | Init command (including --with-st0000)                |
| `learn_commands.bats`     | Learn command (footgun, worked, failed)               |
| `migration.bats`          | Migration and backup                                  |
| `modules_commands.bats`   | Modules command (check, find)                         |
| `plugin_commands.bats`    | Plugin discovery (list, show)                         |
| `project_commands.bats`   | Project-specific commands                             |
| `st_commands.bats`        | Steel thread management                               |
| `st_zero_commands.bats`   | ST Zero retrofit (install, audit, gap analysis)       |
| `skills_commands.bats`    | Skills management (install, sync, uninstall, show)    |
| `subagent_commands.bats`  | Subagent management                                   |
| `test_autopsy.bats`       | Autopsy skill and full directory install              |
| `test_diogenes.bats`      | Diogenes subagent and testing skill                   |
| `treeindex_commands.bats` | Treeindex (directory summaries)                       |
| `wp_commands.bats`        | Work package management                               |

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
