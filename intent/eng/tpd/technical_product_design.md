---
verblock: "20 Feb 2026:v2.4.0: Matthew Sinclair - Update for Intent v2.4.0 as-built"
intent_version: 2.4.0
---

# Intent Technical Product Design v2.4.0 (As-Built)

## Preamble to Claude

This document is a Technical Product Design (TPD) for the Intent system (formerly known as STP - Steel Thread Process). When processing this document, please understand:

1. This is the AS-BUILT documentation for Intent v2.4.0, reflecting the actual implementation
2. Intent underwent a complete rebrand from STP to Intent in July 2025
3. The system is designed to facilitate collaboration between developers and LLMs
4. This document contains:
   - Actual v2.4.0 architecture and implementation
   - Plugin architecture for extensible command system
   - Skills and subagent lifecycle management
   - Treeindex and fileindex for codebase navigation
   - Complete command reference for all intent commands

5. The code is developed through "steel threads" which are incremental implementation stages
6. Steel threads are organized as directories under intent/st/ containing:
   - info.md: Main information and metadata (required)
   - design.md: Design decisions and approach (optional)
   - impl.md: Implementation details (optional)
   - tasks.md: Task tracking (optional)
   - WP/: Work packages for large steel threads (optional)
7. The system consists of shell scripts and markdown templates
8. Configuration uses JSON format (.intent/config.json) instead of YAML
9. Intent is self-hosting - this project is built using Intent v2.4.0
10. Intent v2.4.0 includes Claude Code skills and subagent integration for enhanced AI collaboration
11. Key commands include:

- `intent st list`: List all steel threads with status filtering
- `intent st new`: Create a new steel thread
- `intent st show`: Display steel thread contents
- `intent st edit`: Edit steel thread files
- `intent agents`: Manage AGENTS.md for projects
- `intent claude subagents`: Manage Claude Code subagents (install, sync, uninstall)
- `intent claude skills`: Manage Claude Code skills (install, sync, uninstall)
- `intent claude upgrade`: Diagnose and upgrade project LLM guidance files
- `intent treeindex`: Generate directory summaries for codebase navigation
- `intent fileindex`: Generate file-level summaries
- `intent init`: Initialize a new Intent project
- `intent bootstrap`: Global Intent setup and configuration
- `intent doctor`: Diagnose and fix configuration issues
- `intent help`: Unified help system for all commands

# Intent v2.4.0 Technical Product Design

This document serves as the central index for the Technical Product Design (TPD) of Intent v2.4.0. The TPD has been updated to reflect the as-built state including the plugin architecture, skills system, treeindex, and Claude Code integration. Sections marked with "[AS-BUILT]" indicate deviations from the original design.

## Table of Contents

1. [Introduction](./1_introduction.md)
2. [Requirements](./2_requirements.md)
3. [Architecture](./3_architecture.md)
4. [Detailed Design](./4_detailed_design.md)
5. [Implementation Strategy](./5_implementation_strategy.md)
6. [Deployment and Operations](./6_deployment_and_operations.md)
7. [Technical Challenges and Mitigations](./7_technical_challenges_and_mitigations.md)
8. [Appendices](./8_appendices.md)

## Plugin Architecture (v2.4.0) [AS-BUILT]

Intent v2.4.0 uses a plugin architecture for extensible command management:

### Plugin Structure

```
intent/plugins/
├── agents/                          # AGENTS.md management
│   ├── bin/intent_agents            # agents command
│   └── templates/elixir/            # Elixir project templates
│       ├── AGENTS.md
│       ├── RULES.md
│       └── ARCHITECTURE.md
└── claude/                          # Claude Code integration
    ├── bin/
    │   ├── intent_claude_subagents  # Subagent lifecycle
    │   ├── intent_claude_skills     # Skill lifecycle
    │   └── intent_claude_upgrade    # Project upgrade
    ├── subagents/                   # Subagent definitions
    │   ├── intent/
    │   ├── elixir/
    │   ├── socrates/
    │   └── worker-bee/
    └── skills/                      # Skill definitions
        ├── intent-essentials/
        ├── intent-elixir-essentials/
        ├── intent-ash-ecto-essentials/
        └── intent-phoenix-liveview/
```

### AGENTS.md Commands

- `intent agents init`: Create AGENTS.md for the project
- `intent agents generate`: Generate AGENTS.md content
- `intent agents sync`: Update AGENTS.md with latest project state
- `intent agents validate`: Check AGENTS.md compliance
- `intent agents template`: List or apply templates (e.g., --template elixir)

### Claude Subagent Commands

- `intent claude subagents list`: Show available and installed subagents
- `intent claude subagents install <name>`: Install subagent to ~/.claude/agents/
- `intent claude subagents sync`: Update installed subagents (SHA256 checksums)
- `intent claude subagents uninstall <name>`: Remove subagent
- `intent claude subagents show <name>`: Display subagent details
- `intent claude subagents status`: Check subagent health

### Claude Skill Commands

- `intent claude skills list`: Show available and installed skills
- `intent claude skills install <name>`: Install skill to ~/.claude/skills/
- `intent claude skills sync`: Update installed skills (SHA256 checksums)
- `intent claude skills uninstall <name>`: Remove skill
- `intent claude skills show <name>`: Display skill content

### Claude Upgrade Command

- `intent claude upgrade`: Diagnose and upgrade project LLM guidance files
- Phases: diagnose, plan, execute (with --apply flag)

### Available Subagents

1. **intent**: Steel thread management, Intent commands, project structure
2. **elixir**: Elixir code doctor with Usage Rules, Ash/Phoenix patterns, testing reference
3. **socrates**: CTO Review Mode for technical decision-making via Socratic dialog
4. **worker-bee**: Worker-Bee Driven Design specialist
5. **diogenes**: Elixir Test Architect via Socratic dialog (specify + validate modes)

### Available Skills

1. **intent-essentials**: Core Intent workflow rules (universal)
2. **intent-elixir-essentials**: Elixir coding rules and patterns
3. **intent-elixir-testing**: Elixir test quality rules (strong assertions, spec-driven)
4. **intent-ash-ecto-essentials**: Ash Framework database access rules
5. **intent-phoenix-liveview**: Phoenix LiveView lifecycle rules

### Treeindex System

- `intent treeindex <dir>`: Generate .treeindex directory summaries
- Shadow directory at `intent/.treeindex/` mirrors project structure
- Flags: `--depth N`, `--check`, `--prune`, `--force`, `--model`, `--dry-run`

## Migration Notes

Intent has evolved through several major versions:

- **v2.0.0** (Jul 2025): Rebrand from STP, flattened structure, JSON config
- **v2.1.0** (Jul 2025): Agent init command, manifest management
- **v2.2.0** (Aug 2025): Plugin architecture
- **v2.3.0** (Aug 2025): AGENTS.md commands, Claude subagent rename
- **v2.4.0** (Feb 2026): Skills system, treeindex, Claude upgrade, skill namespacing

## Current Status

- **Version**: 2.4.0 (Skills + Treeindex + Claude Upgrade - February 2026)
- **Tests**: 302 passing across 15 test files (BATS framework)
- **Commands**: Core commands plus plugin commands (agents, claude subagents, claude skills, claude upgrade, treeindex, fileindex)
- **Subagents**: 5 built-in (intent, elixir, socrates, worker-bee, diogenes)
- **Skills**: 5 built-in (intent-essentials, intent-elixir-essentials, intent-elixir-testing, intent-ash-ecto-essentials, intent-phoenix-liveview)
- **Projects Using Intent**: Intent, Prolix, Laksa-web, Lamplight, Anvil, MeetZaya, Multiplyer, Utilz

## Links

- [Current Steel Threads](../../st/)
- [Intent Blog Series](../../../docs/blog/)
- [Migration Guide](./6_deployment_and_operations.md#migration)
