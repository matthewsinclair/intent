# Intent Usage Rules

Intent is a CLI tool for managing steel threads, project artifacts, and LLM guidance for software projects.

## Project Structure

An Intent project has this layout:

```
my-project/
├── intent/
│   ├── st/                    # Steel threads (each a directory)
│   │   ├── ST0001/
│   │   │   ├── info.md        # Objective, context, status (required)
│   │   │   ├── design.md      # Architecture decisions
│   │   │   ├── impl.md        # Implementation notes
│   │   │   ├── tasks.md       # Task checklist
│   │   │   └── WP/            # Work packages (optional subdirs)
│   │   └── steel_threads.md   # Index of all steel threads
│   ├── llm/                   # LLM guidance files
│   │   ├── AGENTS.md          # Auto-generated (intent agents sync)
│   │   ├── RULES.md           # Human-curated coding rules
│   │   └── ARCHITECTURE.md    # Human-curated system architecture
│   ├── docs/                  # Technical documentation
│   ├── .treeindex/            # Shadow directory for treeindex summaries
│   ├── wip.md                 # Current work in progress
│   └── restart.md             # Session restart context
├── .intent/
│   └── config.json            # Project configuration
├── .claude/
│   ├── agents/                # Installed Claude subagents
│   └── skills/                # Installed Claude skills
├── CLAUDE.md                  # Project instructions for Claude
├── AGENTS.md                  # Symlink to intent/llm/AGENTS.md
└── CHANGELOG.md               # Version history
```

## Core Commands

### Steel Threads

```bash
# Create a new steel thread
intent st new "Title of the work"

# List all steel threads
intent st list

# Show steel thread details
intent st show ST0001

# Edit a steel thread
intent st edit ST0001
```

Steel threads are self-contained units of work. Each gets a directory under `intent/st/` with at minimum an `info.md` file containing the objective, status, and context.

### info.md Frontmatter

```yaml
---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
intent_version: 2.3.4
status: WIP
created: 20260217
completed:
---
```

Status values: `WIP`, `Completed`, `Not Started`, `On Hold`, `Cancelled`.

### Treeindex

Generate pre-computed directory summaries for fast LLM navigation:

```bash
# Generate summaries for a directory tree
intent treeindex bin
intent treeindex lib
intent treeindex docs

# Run on all top-level dirs in parallel
intent treeindex bin & intent treeindex docs & wait

# Check for stale summaries (CI-friendly)
intent treeindex bin --check

# Remove orphaned summaries
intent treeindex bin --prune

# Force regeneration even if up to date
intent treeindex bin --force

# Control depth (default: 2)
intent treeindex lib --depth 3

# Dry run (show what would be generated)
intent treeindex lib --dry-run
```

Treeindex files live at `intent/.treeindex/<dir>/.treeindex`. Always check `.treeindex` before exploring an unfamiliar directory. Never run on the project root -- target subdirectories.

### AGENTS.md Management

```bash
# Create AGENTS.md for the project
intent agents init

# Create with Elixir template (AGENTS.md + RULES.md + ARCHITECTURE.md)
intent agents init --template elixir

# Update AGENTS.md with current project state
intent agents sync

# Validate AGENTS.md compliance
intent agents validate

# Force overwrite existing
intent agents init --force
```

`intent agents sync` updates AGENTS.md only. It never modifies RULES.md or ARCHITECTURE.md -- those are human-curated.

### Claude Subagents

```bash
# List available and installed subagents
intent claude subagents list

# Install a subagent
intent claude subagents install elixir
intent claude subagents install --all

# Update installed subagents to latest versions
intent claude subagents sync

# Remove a subagent
intent claude subagents uninstall elixir

# Show subagent details
intent claude subagents show elixir

# Check subagent health
intent claude subagents status
```

Subagents install to `~/.claude/agents/<name>.md`. They run in separate context windows when invoked via the Task tool.

### Claude Skills

```bash
# List available and installed skills
intent claude skills list

# Install a skill
intent claude skills install intent-essentials
intent claude skills install intent-elixir-essentials
intent claude skills install --all

# Update installed skills to latest versions
intent claude skills sync

# Remove a skill
intent claude skills uninstall intent-elixir-essentials

# Show skill details
intent claude skills show intent-elixir-essentials
```

Skills install to `~/.claude/skills/<name>/SKILL.md`. They load into every Claude Code session automatically, shaping code as it is generated.

### Fileindex

```bash
# Create a file index for a directory
intent fileindex lib/my_app '*.ex' -i review.index

# Include subdirectories
intent fileindex lib/my_app '*.ex' -r -i review.index

# Toggle a file as checked/unchecked
intent fileindex -i review.index -X lib/my_app/user.ex
```

### Other Commands

```bash
# Check project configuration
intent doctor

# Show Intent version and project info
intent version
intent info

# Initialize a new Intent project
intent init

# Upgrade project to latest format
intent upgrade
```

## LLM Guidance Files

The three-file system for target projects:

### AGENTS.md -- "How to work here"

Factual and stable. Generated by `intent agents sync`. Contains project overview, prerequisites, setup commands, installed skills and subagents. **Auto-generated -- do not edit manually.**

### RULES.md -- "How to write code"

Prescriptive and opinionated. Every statement is "must" or "never". Contains coding rules, framework conventions, NEVER DO list. **Human-curated -- never auto-modified by Intent.**

### ARCHITECTURE.md -- "How the system works"

Descriptive. Contains system overview, domain map, data flow, directory structure, decision log. **Human-curated -- never auto-modified by Intent.**

## Treeindex Conventions

- Treeindex files live at `intent/.treeindex/<relative_path>/.treeindex`
- Check `.treeindex` before exploring unfamiliar directories
- Run on subdirectories, not the project root
- Files are committed to git (they are documentation, not cache)
- Auto-created files on first run: `.treeindexignore`, `README.md`
- Uses fingerprint-based staleness detection (SHA256 of filenames + sizes)

## Steel Thread Methodology

### Creating a Steel Thread

1. Run `intent st new "Title"` to create the directory and info.md
2. Fill in objective, context, and related threads in info.md
3. Create work packages under `WP/01/`, `WP/02/`, etc. if needed
4. Add task checklist to tasks.md
5. Track design decisions in design.md

### Working a Steel Thread

1. Update status to `WIP` in info.md
2. Work through tasks, checking them off in tasks.md
3. Record implementation notes in impl.md
4. Record design decisions in design.md
5. When complete, update status and completion date

### Frontmatter Format

The `verblock` field tracks document history:

```
"DD Mon YYYY:vX.Y: author - Description of change"
```

## Common Workflows

### Start a new Elixir project with LLM guidance

```bash
intent init
intent agents init --template elixir
intent claude subagents install elixir
intent claude skills install --all
```

### Review and update directory summaries

```bash
intent treeindex bin
intent treeindex lib
intent treeindex docs
intent treeindex intent
```

### Check project health

```bash
intent doctor
intent claude subagents status
intent claude skills sync
```

### Session wrap-up

1. Update steel thread docs (tasks.md, impl.md) with as-built state
2. Update `intent/wip.md` with current state and what's next
3. Update `intent/restart.md` with session restart context
4. Commit and push

## NEVER DO

- Never edit AGENTS.md directly -- it is auto-generated by `intent agents sync`
- Never run `intent treeindex` on the project root -- target subdirectories
- Never create steel thread directories manually -- use `intent st new`
- Never modify `.intent/config.json` manually -- use Intent commands
- Never delete `.treeindex` files without running `--prune` first
- Never put RULES.md or ARCHITECTURE.md content in AGENTS.md -- keep them separate
- Never auto-modify RULES.md or ARCHITECTURE.md -- they are human-curated
