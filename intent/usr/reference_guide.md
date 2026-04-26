---
verblock: "05 Mar 2026:v2.6.0: Matthew Sinclair - Updated for Intent v2.6.0"
intent_version: 2.6.0
---

# Reference Guide

This reference guide provides comprehensive information about the Intent system (v2.6.0). Unlike the task-oriented User Guide, this reference guide serves as a complete reference for all aspects of the system.

## Table of Contents

1. [Command Reference](#command-reference)
2. [Document Templates](#document-templates)
3. [Directory Structure](#directory-structure)
4. [Configuration Options](#configuration-options)
5. [Best Practices](#best-practices)
6. [Concepts and Terminology](#concepts-and-terminology)

## Command Reference

### Core Commands

#### `intent upgrade`

Upgrades a project from an older version to Intent v2.6.0.

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
  legacy config (YAML) → intent/.config/config.json (JSON)

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
intent doctor [options]
```

**Flags:**

- `-f|--fix`: Attempt to fix issues automatically
- `-v|--verbose`: Show detailed information (core tools, permissions)
- `-q|--quiet`: Only show errors and warnings

**Checks performed:**

| Check            | Level    | Description                                          |
| ---------------- | -------- | ---------------------------------------------------- |
| INTENT_HOME      | Required | Environment variable set and directory exists        |
| Executable       | Required | `intent` found and executable                        |
| Global config    | Warning  | `~/.config/intent/config.json` exists and valid JSON |
| Local config     | Info     | Project `intent/.config/config.json` if in a project |
| AGENTS.md        | Warning  | Present for v2.3+ projects                           |
| PATH             | Warning  | `$INTENT_HOME/bin` in PATH                           |
| Required tools   | Required | bash, sed, grep, mkdir, jq                           |
| Optional tools   | Info     | bats, elixir, checksum (sha256sum/shasum)            |
| File permissions | Verbose  | All bin/ files executable                            |
| Agent system     | Info     | Subagent availability and installation status        |

**Example:**

```bash
intent doctor              # standard check
intent doctor -v           # verbose (includes optional tools detail)
intent doctor --fix        # attempt automatic fixes
```

#### `intent init`

Initializes a new Intent project.

**Usage:**

```bash
intent init [--with-st0000] <project_name> [directory]
```

**Parameters:**

- `project_name`: Name of the project (required)
- `directory`: Target directory (optional, defaults to current directory)

**Options:**

- `--with-st0000`: After init, run `intent st zero install` to bootstrap all ST0000 deliverables

**Example:**

```bash
intent init "My Project" ./my-project
intent init "My Project" --with-st0000    # Init + full ST0000 bootstrap
```

**Output:**

- Creates Intent directory structure
- Creates `intent/.config/config.json` with project configuration
- Initializes `intent/` directories (st/, eng/, usr/, llm/)
- Creates `CLAUDE.md` with project instructions
- Creates `intent/wip.md` for work tracking
- With `--with-st0000`: additionally installs all ST0000 deliverables (MODULES.md, DECISION_TREE.md, learnings.md, etc.)

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
intent st new [-s|--start] <title>
```

**Parameters:**

- `title`: Title of the steel thread (required)
- `-s|--start`: Immediately start the thread (creates in `intent/st/` with `status: WIP` instead of `NOT-STARTED/`)

**Examples:**

```bash
intent st new "Implement User Authentication"
intent st new -s "Quick Fix"          # create and start in one command
intent st new "My Feature" --start    # flag can come after title
```

**Output:**

- Creates directory `intent/st/NOT-STARTED/ST####/` (or `intent/st/ST####/` with `--start`)
- Creates `info.md` with metadata, template, and auto-generated slug
- Auto-increments thread ID
- Special characters in titles (`/`, `&`, `\`) are handled safely
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

#### `intent wp`

Manages work packages within steel threads.

**Usage:**

```bash
intent wp new <STID> "Title"     # Create a new work package
intent wp done <STID/NN>         # Mark WP as Done
intent wp start <STID/NN>        # Mark WP as WIP
intent wp list <STID>            # List WPs for a steel thread
intent wp show <STID/NN>         # Show WP info.md
intent wp help                   # Show help
```

**Specifier syntax:**

- STID: `ST0011` or `11` (bare number, zero-padded to ST####)
- STID/NN: `ST0011/01` or `11/1` (slash-separated, WP zero-padded to 2 digits)

**Examples:**

```bash
# Create work packages
intent wp new ST0005 "Implement core logic"
intent wp new 5 "Write tests"

# Manage status
intent wp start 5/01
intent wp done 5/01

# View work packages
intent wp list ST0005
intent wp show ST0005/01
```

**Notes:**

- WP numbers are auto-assigned sequentially (01-99)
- When the last WP is marked done, a hint to complete the ST is printed
- The `WP/` directory is created automatically on first `wp new`
- Titles can contain special characters (`/`, `&`, `\`) safely
- Template: `lib/templates/prj/st/WP/info.md`

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

- In an Intent project: `STARTDIR=lib/`, `FILESPEC=*.{ex,exs}`, `INDEX_DIR=intent/.config/indexes/`
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
| `diogenes`   | Elixir Test Architect via Socratic dialog                    |

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
| `install`   | Install skill(s) to `~/.claude/skills/`    |
| `sync`      | Sync installed skills with latest versions |
| `uninstall` | Remove Intent-managed skills               |
| `show`      | Display skill content and status           |

**Options:**

| Flag          | Description         |
| ------------- | ------------------- |
| `--force, -f` | Force operation     |
| `--all`       | Apply to all skills |

**Available Skills:**

| Name                     | Rules | Description                                     |
| ------------------------ | :---: | ----------------------------------------------- |
| `in-essentials`          |   7   | Intent workflow rules (CLI usage, conventions)  |
| `in-elixir-essentials`   |   8   | Core Elixir rules (pattern matching, pipes)     |
| `in-ash-ecto-essentials` |   7   | Ash/Ecto rules (code interfaces, migrations)    |
| `in-phoenix-liveview`    |   7   | LiveView rules (streams, two-phase mount)       |
| `in-elixir-testing`      |   8   | Test quality rules (strong assertions, specs)   |
| `in-autopsy`             |  --   | Session forensics and memory meta-learning      |
| `in-start`               |  --   | Session start: orientation and context loading  |
| `in-plan`                |  --   | Planning kickoff: workplan and skill invocation |
| `in-standards`           |  --   | Coding standards: rules enforcement             |
| `in-next`                |  --   | Next step: identify smallest coherent work unit |
| `in-finish`              |  --   | Session finish: update docs, commit cleanly     |

**Example:**

```bash
# List skills
intent claude skills list

# Install all skills
intent claude skills install --all

# Check for updates
intent claude skills sync

# View skill content
intent claude skills show in-elixir-essentials
```

**Plugin Location:** Skill definitions live in `intent/plugins/claude/skills/`.
**Install Location:** Skills install to `~/.claude/skills/<name>/` (entire directory, including scripts).

> **Note:** `in-autopsy` requires Elixir. Run `intent doctor` to check prerequisites.

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

#### `intent st zero`

Retrofit installation of ST0000 deliverables for existing (brownfield) projects.

**Usage:**

```bash
intent st zero install [options]
intent st zero help
```

**Subcommands:**

| Command   | Description                                   |
| --------- | --------------------------------------------- |
| `install` | Audit and install missing ST0000 deliverables |
| `help`    | Show usage                                    |

**Options for install:**

| Flag                 | Description                            |
| -------------------- | -------------------------------------- |
| `--audit-only`       | Gap analysis only (no changes applied) |
| `--dry-run`          | Show what would change without writing |
| `--deliverable <ID>` | Target a single deliverable (e.g. D3)  |

**Deliverables checked:**

| ID  | File/Target                         | Notes                        |
| --- | ----------------------------------- | ---------------------------- |
| D2  | `CLAUDE.md`                         | Never overwritten            |
| D3  | `intent/llm/MODULES.md`             | Auto-generated from codebase |
| D4  | `intent/llm/ARCHETYPES.md`          | Elixir only                  |
| D5a | `credo_checks/*.ex`                 | Elixir only (7 Credo checks) |
| D6  | `intent/llm/DECISION_TREE.md`       | From template                |
| D8  | MEMORY.md via `intent claude prime` | Delegates to prime           |
| D9  | Module check hook                   | From template                |
| D10 | `intent/.config/learnings.md`       | Empty structure              |
| D11 | `intent/llm/DEPENDENCY_GRAPH.md`    | Elixir only                  |

**Example:**

```bash
intent st zero install --audit-only   # See what's missing
intent st zero install --dry-run      # Preview changes
intent st zero install                # Install everything missing
intent st zero install --deliverable D3  # Just MODULES.md
```

**Notes:**

- Umbrella-aware: detects `apps/` and scans `apps/*/lib/` for module discovery
- CLAUDE.md (D2) is never overwritten -- only shows diff of missing sections
- Elixir-specific deliverables (D4, D5a, D11) require `mix.exs`

#### `intent audit`

Runs automated code quality checks for Elixir projects.

**Usage:**

```bash
intent audit <subcommand> [options]
```

**Subcommands:**

`intent audit quick`

Runs custom Credo checks against the project.

**Options:**

| Flag            | Description                         |
| --------------- | ----------------------------------- |
| `--rule RN`     | Run a specific rule only            |
| `--fix`         | Auto-fix issues where possible      |
| `--json`        | Machine-readable JSON output        |
| `--checks-only` | Install checks without running them |

**Available Rules:**

| Rule | Template                      | Description                       |
| ---- | ----------------------------- | --------------------------------- |
| R2   | `thick_coordinator.ex`        | Controllers with business logic   |
| R6   | `highlander_suspect.ex`       | Potential code duplication        |
| R7   | `map_get_on_struct.ex`        | Unsafe struct field access        |
| R11  | `missing_impl_annotation.ex`  | Callback without @impl true       |
| R15  | `debug_artifacts.ex`          | IO.inspect, dbg() in code         |
| R16  | `bracket_access_on_struct.ex` | Bracket access on struct variable |

`intent audit health`

Runs comprehensive project health assessment.

**Options:**

| Flag       | Description                             |
| ---------- | --------------------------------------- |
| `--report` | Save report to `intent/audit/`          |
| `--diff`   | Check only files changed since last run |

**Health checks:** MODULES.md coverage, thick coordinators, Highlander suspects, Credo status.

**Example:**

```bash
intent audit quick                # Run all custom checks
intent audit quick --rule R2      # Check for thick coordinators only
intent audit quick --checks-only  # Force-copy check templates
intent audit health --report      # Save health report
intent audit health --diff        # Check changed files only
```

**Notes:**

- Templates auto-copied to project's `credo_checks/` on first run
- `--checks-only` force-copies templates (ensures updates are applied on re-run)
- Umbrella-aware: health checks scan `apps/*/lib/` in umbrella projects
- Requires an Elixir project with Credo configured

#### `intent learn`

Captures project learnings for future sessions.

**Usage:**

```bash
intent learn "description" [--category <cat>]
intent learn --list
```

**Options:**

| Flag               | Description                                 |
| ------------------ | ------------------------------------------- |
| `--category <cat>` | Category: footgun (default), worked, failed |
| `--list`           | List all recorded learnings                 |

**Example:**

```bash
intent learn "Never use Map.get on structs"
intent learn --category worked "Ash code interfaces simplify everything"
intent learn --list
```

**Notes:**

- Stores in `intent/.config/learnings.md` with date-prefixed entries
- Automatically included by `intent claude prime`

#### `intent modules`

Module registry guardrails and enforcement.

**Usage:**

```bash
intent modules <subcommand> [options]
```

**Subcommands:**

| Command | Description                           |
| ------- | ------------------------------------- |
| `check` | Compare MODULES.md against filesystem |
| `find`  | Search MODULES.md for a term          |
| `help`  | Show usage                            |

**Options for check:**

| Flag         | Description                          |
| ------------ | ------------------------------------ |
| `--register` | Interactively register missing files |

**Example:**

```bash
intent modules check              # Report unregistered/stale entries
intent modules check --register   # Interactive registration
intent modules find "helpers"     # Find modules matching a term
```

**Output:**

- `+` prefix: unregistered file (exists but not in MODULES.md)
- `-` prefix: stale entry (in MODULES.md but file missing)
- Exit code 0: clean, exit code 1: issues found

#### `intent plugin`

Discovers installed plugins and their commands.

**Usage:**

```bash
intent plugin [list]              # List all plugins
intent plugin show <name>         # Show details for a plugin
intent plugin help                # Show usage
```

**Example:**

```bash
intent plugin                     # List all plugins
intent plugin show claude         # Show claude plugin details
```

**Notes:**

- Global command (works without project context)
- Reads `plugin.json` metadata from `intent/plugins/*/`

#### `intent claude prime`

Synthesizes operational knowledge into MEMORY.md for Claude Code sessions.

**Usage:**

```bash
intent claude prime [options]
```

**Options:**

| Flag               | Description                     |
| ------------------ | ------------------------------- |
| `--refresh`        | Overwrite existing MEMORY.md    |
| `--dry-run`        | Preview without writing         |
| `--from <project>` | Source from a different project |

**Example:**

```bash
intent claude prime               # Generate MEMORY.md
intent claude prime --refresh     # Overwrite existing
intent claude prime --dry-run     # Preview output
```

**Notes:**

- Reads MODULES.md, DECISION_TREE.md, ARCHETYPES.md, learnings
- Enforces 200-line limit with truncation warning
- Output goes to `.claude/projects/.../memory/MEMORY.md`

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
slug: implement-user-auth
created: 20250307
completed:
---
```

**Supported Metadata Fields:**

- `status`: Current state of the steel thread (Not Started, In Progress, Completed, On Hold, or Cancelled)
- `slug`: URL-safe identifier auto-generated from the title (max 50 chars, lowercase, hyphens for non-alphanumeric)
- `created`: Creation date in YYYYMMDD format
- `completed`: Completion date in YYYYMMDD format (omit or leave empty if not completed)
- `verblock`: Version tracking information

The `slug` field is generated automatically by `intent st new` and displayed in `intent st list` instead of the full title. For steel threads created before this feature, the list falls back to the title from the heading.

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
│   │   │   ├── tasks.md         # Task tracking
│   │   │   └── WP/              # Work packages (optional)
│   │   │       └── NN/          # WP directory (01-99)
│   │   │           └── info.md  # WP metadata and details
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
│           │   ├── in-essentials/
│           │   ├── in-elixir-essentials/
│           │   ├── in-ash-ecto-essentials/
│           │   ├── in-phoenix-liveview/
│           │   ├── in-elixir-testing/
│           │   └── in-autopsy/
│           │       ├── SKILL.md
│           │       └── scripts/
│           │           ├── autopsy.exs
│           │           └── banned-words.txt
│           └── subagents/ # Subagent definitions
│               ├── intent/
│               ├── elixir/
│               ├── socrates/
│               ├── worker-bee/
│               └── diogenes/
│   │   ├── MODULES.md      # Module registry (Highlander enforcer)
│   │   └── DECISION_TREE.md # Code placement guide
├── lib/                   # Templates and libraries
│   ├── templates/
│   │   ├── llm/           # LLM guidance templates
│   │   ├── archetypes/    # Code archetype templates
│   │   ├── credo_checks/  # Custom Credo check templates (7)
│   │   ├── hooks/         # Claude Code hook templates
│   │   └── prime/         # Operational knowledge for prime
│   └── help/              # Help files for commands
├── bin/                   # Intent scripts (executable)
├── tests/                 # Test suite
│   ├── unit/              # Unit tests (22 .bats files)
│   ├── integration/       # Integration tests
│   ├── lib/               # Test helper libraries
│   ├── fixtures/          # Test fixtures
│   └── run_tests.sh       # Test runner script
└── intent/.config/               # Intent configuration
    └── config.json        # Project configuration
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

Location: `intent/.config/config.json`

Format: JSON configuration file

Example:

```json
{
  "project_name": "Project Name",
  "author": "Default Author",
  "intent_version": "2.6.0",
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
- Document decisions and their rationale in steel threads
- Use clear, descriptive titles for steel threads
- Maintain cross-references between related documents

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
| Treeindex       | LLM-oriented directory summaries stored in a shadow directory                   |
| Fileindex       | File tracking tool with checkbox states for progress management                 |
| AGENTS.md       | Universal AI agent instructions file for any AI platform                        |
| Subagent        | A Claude Code sub-agent with domain-specific expertise                          |
| Plugin          | An extension module for Intent (eg agents, claude)                              |
| Skill           | An always-on Claude Code enforcement file installed to `.claude/skills/`        |
| RULES.md        | Human-curated coding rules and conventions file in `intent/llm/`                |
| ARCHITECTURE.md | Human-curated system architecture description in `intent/llm/`                  |
| MODULES.md      | Module registry for the Highlander Rule (one module per concern)                |
| Audit           | Automated code quality checks using custom Credo rules                          |
| Health check    | Comprehensive project quality assessment via `intent audit health`              |
| Learning        | Captured project insight stored in `intent/.config/learnings.md`                |
| Prime           | Memory injection via `intent claude prime` for session context                  |
| ST Zero         | Retrofit installation of ST0000 deliverables for brownfield projects            |
