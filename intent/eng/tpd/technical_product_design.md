---
verblock: "27 Jul 2025:v2.1.0: Matthew Sinclair - Update for Intent v2.1.0 with agent init"
intent_version: 2.1.0
---

# Intent Technical Product Design v2.1.0 (As-Built)

## Preamble to Claude

This document is a Technical Product Design (TPD) for the Intent system (formerly known as STP - Steel Thread Process). When processing this document, please understand:

1. This is the AS-BUILT documentation for Intent v2.1.0, reflecting the actual implementation
2. Intent underwent a complete rebrand from STP to Intent in July 2025
3. The system is designed to facilitate collaboration between developers and LLMs
4. This document contains:
   - Actual v2.1.0 architecture and implementation
   - JSON-based configuration system
   - Complete command reference for intent\_\* commands
   - Migration tools and processes
   - Lessons learned from development

5. The code is developed through "steel threads" which are incremental implementation stages
6. Steel threads are organized as directories under intent/st/ containing:
   - info.md: Main information and metadata (required)
   - design.md: Design decisions and approach (optional)
   - impl.md: Implementation details (optional)
   - tasks.md: Task tracking or Backlog.md integration (optional)
7. The system consists of shell scripts and markdown templates
8. Configuration uses JSON format (.intent/config.json) instead of YAML
9. The system integrates with Backlog.md for task management with enhanced filtering
10. Intent is self-hosting - this project is built using Intent v2.1.0
11. Intent v2.1.0 includes Claude Code sub-agent integration with proper initialization for enhanced AI collaboration
12. Key commands include:

- `intent st list`: List all steel threads with status filtering
- `intent st new`: Create a new steel thread
- `intent st show`: Display steel thread contents
- `intent st edit`: Edit steel thread files
- `intent bl`: Enhanced Backlog.md wrapper with status filtering
- `intent task`: Manage Backlog tasks linked to steel threads
- `intent status`: Synchronize steel thread status with task completion
- `intent agents`: Manage Claude Code sub-agents for Intent projects
- `intent init`: Initialize a new Intent project
- `intent bootstrap`: Global Intent setup and configuration
- `intent doctor`: Diagnose and fix configuration issues
- `intent upgrade`: Migrate any STP/Intent version to Intent v2.1.0
- `intent help`: Unified help system for all commands

# Intent v2.1.0 Technical Product Design

This document serves as the central index for the Technical Product Design (TPD) of Intent v2.1.0. The TPD has been forensically updated to reflect the actual as-built state of the system after the migration from STP to Intent and enhancement with agent initialization. Sections marked with "[AS-BUILT]" indicate deviations from the original design.

## Table of Contents

1. [Introduction](./1_introduction.md)
2. [Requirements](./2_requirements.md)
3. [Architecture](./3_architecture.md)
4. [Detailed Design](./4_detailed_design.md)
5. [Implementation Strategy](./5_implementation_strategy.md)
6. [Deployment and Operations](./6_deployment_and_operations.md)
7. [Technical Challenges and Mitigations](./7_technical_challenges_and_mitigations.md)
8. [Appendices](./8_appendices.md)

## Agent System (v2.1.0) [AS-BUILT]

Intent v2.1.0 includes Claude Code sub-agent integration with proper initialization, enhancing AI collaboration:

### Architecture

- **Agent Storage**: `$INTENT_HOME/agents/` (global), `./intent/agents/` (project)
- **Installation Target**: `~/.claude/agents/`
- **Manifest Tracking**: JSON manifests track installations and checksums
- **Sync Mechanism**: File-based sync with modification detection

### Available Agents

1. **Intent Agent**: Understands steel threads, Intent commands, and project structure
2. **Elixir Agent**: Elixir code doctor with Usage Rules and Ash/Phoenix patterns
3. **Socrates Agent**: CTO Review Mode for technical decision-making via Socratic dialog

### Agent Commands

- `intent agents init`: Initialize agent configuration
- `intent agents list`: Show available and installed agents
- `intent agents install`: Install agents to Claude configuration
- `intent agents sync`: Update agents while preserving modifications
- `intent agents uninstall`: Remove Intent-managed agents
- `intent agents show`: Display agent details and metadata
- `intent agents status`: Check agent health and integrity

### Integration Points

- **intent init**: Detects Claude and offers agent installation
- **intent doctor**: Includes agent health checks
- **intent upgrade**: Preserves agent directories during migration

## Migration Notes

Intent v2.1.0 represents a complete rebrand and restructuring from the Steel Thread Process (STP) to Intent:

- Directory structure flattened: `stp/prj/st/` → `intent/st/`
- Commands renamed: `stp_*` → `intent_*`
- Configuration migrated: YAML → JSON
- New features: bootstrap, doctor, upgrade commands
- Enhanced Backlog.md integration with status filtering
- Self-hosting success: Intent is built using Intent

## Current Status

- **Version**: 2.1.0 (Enhanced Agent System with init command - July 2025)
- **Tests**: 165/165 passing (includes 50 new agent tests)
- **Commands**: 13 primary commands including new `intent agents`
- **Agents**: 2 built-in agents (Intent, Elixir)
- **Projects Using Intent**: Intent itself (self-hosting)

## Links

- [Current Steel Threads](../../st/)
- [Intent Blog Series](../../../docs/blog/)
- [Migration Guide](./6_deployment_and_operations.md#migration)
