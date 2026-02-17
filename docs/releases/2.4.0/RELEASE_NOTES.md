# Intent v2.4.0 Release Notes

**Release Date**: 17 February 2026

## Overview

Intent v2.4.0 introduces a skills system for always-on code enforcement, an upgrade command for migrating existing projects, and comprehensive Elixir/Ash/Phoenix reference documentation. This is the largest feature release since v2.0.0.

## What's New

### Skills System

Skills are always-on Claude Code enforcement rules that shape code as it is generated. Unlike subagents (which review after the fact), skills are proactive -- loaded into every session automatically.

**New command**: `intent claude skills`

```bash
intent claude skills list              # Show available and installed skills
intent claude skills install <name>    # Install a skill to .claude/skills/
intent claude skills sync              # Update installed skills
intent claude skills uninstall <name>  # Remove a skill
intent claude skills show <name>       # Display skill content
```

**Four skills ship with v2.4.0** (one universal + three Elixir):

| Skill                  | Rules | Focus                                        |
|------------------------|:-----:|----------------------------------------------|
| `intent-essentials`    |   7   | CLI usage, treeindex, steel thread conventions |
| `elixir-essentials`    |   8   | Pattern matching, tagged tuples, pipes, naming |
| `ash-ecto-essentials`  |   7   | Code interfaces, migrations, actor placement  |
| `phoenix-liveview`     |   7   | Two-phase mount, streams, components          |

Skills install to `.claude/skills/<name>/SKILL.md` in the target project and use SHA256 checksum manifests for sync tracking.

### Upgrade Command

New `intent claude upgrade` command diagnoses and upgrades LLM guidance files in existing projects.

```bash
intent claude upgrade                     # Dry-run: diagnose and plan
intent claude upgrade --apply             # Apply changes
intent claude upgrade --project-dir DIR   # Target external project
```

The upgrade command:
- Scans for existing files (AGENTS.md, RULES.md, ARCHITECTURE.md)
- Detects deprecated files (AGENTS-phx.md, llm_preamble.md)
- Checks subagent and skill installation status
- Generates a project-specific upgrade plan
- Applies changes with `--apply`

### Elixir Reference Documentation

Four new reference documents added to the Elixir subagent:

| Document               | Lines | Coverage                                        |
|------------------------|:-----:|------------------------------------------------|
| `ash-ecto.md`          |  ~300 | Ash code interfaces, migrations, query patterns |
| `liveview.md`          |  ~280 | Two-phase rendering, streams, uploads, forms    |
| `testing.md`           |  ~350 | DataCase, ConnCase, LiveView, Mox, Ash testing  |
| `project-structure.md` |  ~220 | Standard Phoenix/Ash project layout             |

### Elixir Project Templates

`intent agents init --template elixir` now creates three files:

- **AGENTS.md** -- Project overview with Elixir-specific commands and setup
- **RULES.md** -- 9 core rules + framework rules + NEVER DO list
- **ARCHITECTURE.md** -- Domain map and directory structure skeleton

### Three-File LLM Guidance System

Intent projects now use a rationalized three-file system:

| File              | Purpose                          | Management        |
|-------------------|----------------------------------|-------------------|
| `AGENTS.md`       | Factual project overview         | Auto-generated    |
| `RULES.md`        | Mandatory coding rules           | Human-curated     |
| `ARCHITECTURE.md` | System structure and decisions   | Human-curated     |

### Intent Usage Rules

New `usage-rules.md` in the project root -- a comprehensive, LLM-optimized reference for how to use Intent itself (~310 lines). Follows the pattern established by `deps/ash/usage-rules.md` in Ash projects.

### Rule Refactoring

The Elixir subagent's core rules were refactored from 23 overlapping rules to 12 non-overlapping rules organized into 5 categories: Data Access, Control Flow, Composition, Error Handling, and Code Hygiene.

## Testing

- 37 new BATS tests for skills commands (`tests/unit/skills_commands.bats`)
- Full test suite: **302 tests** across **15 test files**
- All tests passing on macOS and Linux

## Upgrade Guide

See `docs/releases/2.4.0/upgrade-guide-2.4.0.md` for step-by-step instructions on upgrading existing projects.

### Quick Upgrade

```bash
# From your project directory:
intent claude upgrade --apply

# Or target a specific project:
intent claude upgrade --apply --project-dir /path/to/project
```

### What Gets Upgraded

- Stale AGENTS.md files are regenerated
- Missing RULES.md and ARCHITECTURE.md are created from templates
- Deprecated files (AGENTS-phx.md, llm_preamble.md, old usage-rules.md) are flagged for removal
- Subagents and skills are installed/updated

## Projects Upgraded

The v2.4.0 upgrade was tested and applied to 8 projects:

| Project    | Type     | Actions                                          |
|------------|----------|--------------------------------------------------|
| Intent     | Bash     | Deprecated files removed, AGENTS.md regenerated  |
| Prolix     | Elixir   | RULES.md + ARCHITECTURE.md created, AGENTS-phx.md merged |
| Laksa-web  | Elixir   | Phoenix rules merged into RULES.md, deprecated files removed |
| Lamplight  | Elixir   | AGENTS.md regenerated (already had RULES.md + ARCHITECTURE.md) |
| Anvil      | Elixir   | RULES.md + ARCHITECTURE.md from template         |
| MeetZaya   | Elixir   | RULES.md + ARCHITECTURE.md created, AGENTS-phx.md merged |
| Multiplyer | Elixir   | RULES.md + ARCHITECTURE.md from template         |
| Utilz      | Bash     | AGENTS.md regenerated only                       |

## Breaking Changes

None. All new functionality is additive.

## Migration

No migration required for the core Intent CLI. The `intent claude upgrade` command handles LLM guidance file migration automatically.
