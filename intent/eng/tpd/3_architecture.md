---
verblock: "20 Feb 2026:v2.4.0: Matthew Sinclair - Updated for Intent v2.4.0"
intent_version: 2.4.0
---

# 3. Architecture

[index](./technical_product_design.md)

## 3.1 System Architecture Overview

The Steel Thread Process (STP) follows a modular architecture with three primary components:

1. **Documentation Structure**: A standardized directory layout and document templates
2. **Command-line Interface**: Shell scripts for managing STP workflows
3. **Process Guidelines**: Documentation of workflow patterns and best practices

This architecture is designed to be lightweight, portable, and to integrate with existing development environments without significant friction.

```
┌─────────────────────────────────────────────────────────────┐
│                      STP System                             │
│                                                             │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐│
│  │ Documentation │    │  Command-line │    │    Process    ││
│  │   Structure   │◄───┤   Interface   │────►   Guidelines  ││
│  └───────────────┘    └───────────────┘    └───────────────┘│
│         ▲                     ▲                   ▲         │
└─────────┼─────────────────────┼───────────────────┼─────────┘
          │                     │                   │
┌─────────┼─────────────────────┼───────────────────┼─────────┐
│         │                     │                   │         │
│  ┌──────▼──────┐      ┌───────▼─────┐     ┌───────▼─────┐   │
│  │  Project    │      │    Shell    │     │     LLM     │   │
│  │ Repository  │      │ Environment │     │ Interaction │   │
│  └─────────────┘      └─────────────┘     └─────────────┘   │
│                                                             │
│                  Development Environment                    │
└─────────────────────────────────────────────────────────────┘
```

## 3.2 Component Architecture

### 3.2.1 Documentation Structure

The Documentation Structure consists of a standardized directory layout and markdown templates. Key features include:

- **Directory Organization**: Clear separation of project, technical, user, and LLM-specific documentation
- **Templated Documents**: Standardized starting points for all document types
- **Cross-Referencing**: Internal links to maintain relationships between documents
- **Progressive Documentation**: Documents that evolve alongside the code

```
stp/
├── _templ/             # Templates
├── bin/                # STP scripts
├── prj/                # Project documentation
│   ├── st/             # Steel threads
│   └── wip.md          # Work in progress
├── eng/                # Engineering docs
│   └── tpd/            # Technical Product Design
├── usr/                # User documentation
└── llm/                # LLM-specific content
```

### 3.2.2 Command-line Interface

The Command-line Interface provides shell-based tools for managing STP workflows. Key features include:

- **Unified Command**: Single `stp` entry point with subcommands
- **Modular Implementation**: Each subcommand implemented as a separate script
- **Contextual Help**: Built-in documentation for commands
- **Environment Configuration**: Settings for STP behavior

```
┌─────────────────┐
│  stp (main)     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐    ┌─────────────────┐
│  Command        │───►│  Command-       │
│  Dispatcher     │    │  specific       │
└────────┬────────┘    │  implementation │
         │             └─────────────────┘
         ▼
┌─────────────────┐
│  Help System    │
└─────────────────┘
```

### 3.2.3 Process Guidelines

The Process Guidelines define how Intent is used in practice. Key features include:

- **Steel Thread Methodology**: Process for incremental development
- **LLM Collaboration**: Enhanced patterns for AI assistance
- **Documentation Lifecycle**: How documents evolve through project stages
- **Integration Points**: How Intent integrates with other practices
- **Self-Hosting**: Intent is developed using Intent itself

## 3.3 Data Architecture [AS-BUILT]

### 3.3.1 Configuration System

Intent v2.0.0 uses a hierarchical JSON configuration system:

```
Configuration Hierarchy (highest to lowest priority):
1. Environment Variables (INTENT_*, AUTHOR, EDITOR)
2. Local Project Config (.intent/config.json)
3. Global User Config (~/.config/intent/config.json)
4. Built-in Defaults

Example .intent/config.json:
{
  "version": "2.0.0",
  "project_name": "MyProject",
  "author": "username",
  "created": "2025-07-17",
  "st_prefix": "ST",
  "backlog_dir": "backlog",          // [Removed in v2.5.0]
  "intent_dir": "intent",
  "backlog_list_status": "todo"      // [Removed in v2.5.0]
}
```

### 3.3.2 Data Types

Intent manages several types of data:

1. **Configuration Data**: JSON-based project and global settings
2. **Steel Thread Data**: Markdown files in intent/st/ directories
3. **Project Metadata**: Steel thread status, creation dates
4. **Work History**: Journal entries and completed threads
5. **Task Data**: Backlog.md integration with status tracking — **[Removed in v2.5.0]**

All data uses plain text formats (JSON and markdown) for maximum portability.

## 3.4 Interface Architecture

### 3.4.1 User Interfaces

Intent provides multiple user interfaces:

1. **Command-line Interface**: `intent` command with subcommands
2. **Document Structure**: Markdown for human and LLM consumption
3. **Configuration Interface**: JSON files for settings
4. **Diagnostic Interface**: `intent doctor` for troubleshooting
5. **Migration Interface**: `intent upgrade` for version transitions

### 3.4.2 External System Interfaces

STP is designed to interface with:

1. **Version Control Systems**: Through normal file operations
2. **LLM Systems**: Through document content and canned prompts
3. **Development Environments**: Through standard shell integration
4. **Task Management Systems**: Through Backlog.md integration for fine-grained task tracking — **[Removed in v2.5.0]**

## 3.5 Architectural Decisions

| Decision                     | Rationale                                                                  |
| ---------------------------- | -------------------------------------------------------------------------- |
| Use of Markdown              | Maximizes portability and readability for both humans and LLMs             |
| Shell Scripts Only           | Ensures compatibility across development environments without dependencies |
| Directory-Based Organization | Creates clear structure while maintaining simplicity                       |
| Template-Driven Approach     | Reduces friction in creating consistent documentation                      |
| Steel Thread Methodology     | Breaks work into manageable units suitable for LLM collaboration           |

## 3.6 Integration Architecture

STP is designed as an extensible system that can integrate with complementary tools while maintaining its core philosophy of simplicity and portability.

### 3.6.1 Backlog.md Integration

> **[Removed in v2.5.0]** Backlog.md integration was removed in Intent v2.5.0.

The integration with Backlog.md extends STP's capabilities with fine-grained task management while preserving the separation of concerns:

#### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      STP System                             │
│                                                             │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐│
│  │ Steel Threads │    │  STP Commands │    │   Templates   ││
│  │  (Intent)     │◄───┤  (Workflow)   │────►  (Structure)  ││
│  └───────────────┘    └───────────────┘    └───────────────┘│
│         ▲                     │                             │
│         │              ┌──────▼──────┐                      │
│         └──────────────┤ Integration │                      │
│                        │   Layer     │                      │
│                        └──────┬──────┘                      │
└───────────────────────────────┼─────────────────────────────┘
                                │
┌───────────────────────────────┼─────────────────────────────┐
│                               ▼                             │
│                        ┌─────────────┐                      │
│                        │ Backlog.md  │                      │
│                        │   System    │                      │
│                        └─────────────┘                      │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐│
│  │ Task Tracking │    │    Kanban     │    │  Task Files  ││
│  │  (Execution)  │◄───┤    Board      │────►  (Storage)   ││
│  └───────────────┘    └───────────────┘    └───────────────┘│
│                                                             │
│                    External Task Management                 │
└─────────────────────────────────────────────────────────────┘
```

#### Component Responsibilities [AS-BUILT]

**Intent Core Components:**

- **Steel Threads**: Capture objectives, context, and design in intent/st/
- **Documentation**: Maintain narrative and specs in intent/docs/
- **Configuration**: JSON-based settings in .intent/config.json
- **Process Coordination**: Orchestrate development workflow

**Backlog.md Integration:**

- **Task Management**: Track implementation tasks with metadata
- **Status Filtering**: Configurable backlog_list_status for focused views
- **Visualisation**: Kanban board and browser interfaces

**Integration Layer:**

- **Command Wrappers**: `intent bl`, `intent task`, `intent status`
- **Status Synchronisation**: Task completion drives thread status
- **Naming Conventions**: ST#### prefix links tasks to threads
- **Git Safety**: Wrappers prevent git operation conflicts

#### Data Flow

1. **Steel Thread Creation** → Integration layer creates linked task structure
2. **Task Updates** → Status changes propagate to steel thread status
3. **Migration** → Embedded tasks convert to Backlog.md format
4. **Queries** → Unified view of steel thread and task information

#### Integration Points [AS-BUILT]

1. **File System**:
   - Intent: `/intent/st/` for steel threads (flattened)
   - Backlog: `/backlog/` for task management
   - Config: `/.intent/config.json` for settings
   - No overlap in storage locations

2. **Command Interface**:
   - Unified `intent` command with subcommands
   - All commands follow intent\_\* pattern
   - Wrapper commands prevent git conflicts
   - Help system integrated

3. **Status Model**:
   - Steel thread status derived from task completion
   - Automatic synchronisation via `intent status sync`
   - Manual override supported
   - Configurable list filtering

4. **Workflow Integration**:
   - Steel threads define "what" and "why" (intent)
   - Backlog tasks define "how" and "when" (execution)
   - Clear separation of concerns
   - Self-hosting proven

### 3.6.2 Migration Architecture [AS-BUILT]

Intent v2.0.0 includes comprehensive migration support:

```
Migration Flow:
1. Detect existing STP version
2. Create timestamped backup
3. Migrate directory structure (stp/* → intent/*)
4. Convert YAML configs to JSON
5. Update file formats and metadata
6. Create .intent/config.json
7. Update CLAUDE.md guidelines

Supported Versions:
- v0.0.0 → v2.0.0
- v1.2.0 → v2.0.0
- v1.2.1 → v2.0.0
```

## 3.7 Plugin Architecture [AS-BUILT v2.4.0]

Intent v2.2.0+ uses a plugin architecture for extensible commands:

### Plugin Structure

```
intent/plugins/
├── agents/                          # AGENTS.md management plugin
│   ├── bin/intent_agents            # Command implementation
│   └── templates/elixir/            # Project templates
└── claude/                          # Claude Code integration plugin
    ├── bin/
    │   ├── intent_claude_subagents  # Subagent lifecycle
    │   ├── intent_claude_skills     # Skill lifecycle
    │   └── intent_claude_upgrade    # Project upgrade
    ├── subagents/                   # Subagent definitions
    └── skills/                      # Skill definitions
```

### Plugin Command Routing

The main `bin/intent` dispatcher routes plugin commands:

- `intent agents *` routes to `intent/plugins/agents/bin/intent_agents`
- `intent claude subagents *` routes to `intent/plugins/claude/bin/intent_claude_subagents`
- `intent claude skills *` routes to `intent/plugins/claude/bin/intent_claude_skills`
- `intent claude upgrade` routes to `intent/plugins/claude/bin/intent_claude_upgrade`

### Skills vs Subagents Architecture

| Dimension      | Subagents                         | Skills                          |
| -------------- | --------------------------------- | ------------------------------- |
| Context window | Separate -- no main impact        | Consumes main context tokens    |
| Activation     | Explicit delegation via Task tool | Always-on or slash-command      |
| Enforcement    | Reviews after the fact            | Shapes generation in real-time  |
| Scope          | Bounded tasks (review module X)   | Ambient standards (always do Y) |

Skills and subagents are complementary: skills shape code as it is generated, subagents do deep review on demand.

### Treeindex Architecture

- Shadow directory at `intent/.treeindex/` mirrors project structure
- Each `.treeindex` file contains a concise summary of a directory's contents
- Generated via `intent treeindex <dir>` using LLM analysis
- Supports `--depth`, `--check`, `--prune`, `--force`, `--model`, `--dry-run`

### Manifest-Based Lifecycle

Both subagents and skills use SHA256 checksum manifests:

- Subagent manifest: `~/.intent/agents/installed-agents.json`
- Skill manifest: `~/.intent/skills/installed-skills.json`
- Sync detects modified, stale, and up-to-date artifacts

## 3.8 AS-BUILT Summary

Intent v2.4.0 represents a significant evolution:

- Plugin architecture for extensible commands
- Skills system for proactive code shaping
- Subagent system for on-demand deep review
- Treeindex for codebase navigation
- AGENTS.md management for project LLM guidance
- Claude upgrade command for project modernization
- 302 tests across 15 files, proven through self-hosting and 8 target projects
