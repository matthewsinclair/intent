---
verblock: "20 Feb 2026:v2.4.0: Matthew Sinclair - Updated for Intent v2.4.0"
intent_version: 2.4.0
---

# 1. Introduction

[index](./technical_product_design.md)

## 1.1 Purpose

Intent (formerly the Steel Thread Process or STP) is a system designed to create a structured workflow and documentation process for developers working collaboratively with Large Language Models (LLMs) such as Claude Code. Intent provides templates, scripts, and process guidelines to enhance productivity while ensuring high-quality documentation as a byproduct of the development process.

### 1.1.1 The Rebrand to Intent

In July 2025, STP was rebranded to "Intent" to better reflect the system's core purpose: capturing and preserving the intention behind software development decisions. While the methodology remains the "Steel Thread Process," the tool itself is now Intent.

## 1.2 Scope

Intent v2.4.0 encompasses:

- A flattened directory structure under `intent/` for organizing documentation
- JSON-based configuration system (.intent/config.json)
- Plugin architecture for extensible command system
- Shell scripts for managing Intent workflows
- The Steel Thread Process methodology for incremental development
- Claude Code skills for proactive code shaping during generation
- Claude Code subagents for on-demand deep review
- Treeindex system for pre-computed directory summaries
- AGENTS.md management for project LLM guidance
- Diagnostic and setup tools (doctor, bootstrap)

Intent is designed to be lightweight, adaptable, and to work alongside existing development workflows without requiring significant changes to development practices.

## 1.3 Definitions

| Term           | Definition                                                                                       |
| -------------- | ------------------------------------------------------------------------------------------------ |
| Intent         | The tool and framework for intention-aware development (v2.4.0)                                  |
| Steel Thread   | A self-contained unit of work that represents a logical piece of functionality to be implemented |
| Work Package   | A subdivision of a steel thread for large-scope work                                             |
| LLM            | Large Language Model, an AI system capable of understanding and generating text                  |
| Context Window | The amount of text an LLM can process in a single interaction                                    |
| Skill          | An always-on Claude Code artifact that shapes code generation in real-time                       |
| Subagent       | An on-demand Claude Code agent for bounded review/analysis tasks                                 |
| Treeindex      | Pre-computed directory summaries for fast codebase navigation                                    |
| Plugin         | An extensible command module under intent/plugins/                                               |
| Bootstrap      | Initial global setup process for Intent installation                                             |
| Doctor         | Diagnostic tool for identifying and fixing Intent configuration issues                           |

## 1.4 System Overview

Intent v2.4.0 operates as a meta-layer on top of existing development processes. It provides structure for:

1. **Documentation Management**: Flattened structure under `intent/` for all documentation
2. **LLM Collaboration**: Skills for proactive code shaping, subagents for deep review
3. **Incremental Development**: The Steel Thread methodology for breaking work into manageable units
4. **Project Tracking**: Work-in-progress tracking and project history
5. **Plugin Architecture**: Extensible command system under `intent/plugins/`
6. **Codebase Navigation**: Treeindex for pre-computed directory summaries
7. **Configuration Management**: JSON-based configuration with hierarchy support
8. **Project Guidance**: AGENTS.md generation and management for LLM context
9. **Self-Hosting**: Intent is developed using Intent itself

The system remains intentionally simple, using markdown files and shell scripts to maximize portability and minimize dependencies.

## 1.5 References

- Modern LLM-assisted development practices
- Documentation-as-code methodologies
- Incremental development processes
- Intent Blog Series (docs/blog/)
- Migration from STP to Intent v2.1.0

## 1.6 Version History

- **v0.0.0 - v1.2.1**: Original Steel Thread Process (STP) development
- **v2.0.0 (July 2025)**: Complete rebrand to Intent with:
  - Flattened directory structure
  - JSON configuration system
  - New bootstrap, doctor, and upgrade commands
  - Enhanced Backlog.md integration [Removed in v2.5.0]
  - Self-hosting capability
- **v2.1.0 (July 2025)**: Enhanced agent system with:
  - Agent initialization command
  - Improved upgrade from v2.0.0
  - Better agent manifest management
- **v2.2.0 (August 2025)**: Plugin architecture:
  - Commands moved to `intent/plugins/`
  - Extensible command routing
- **v2.3.0 (August 2025)**: AGENTS.md and subagent rename:
  - `intent agents` now manages AGENTS.md (init, generate, sync, validate, template)
  - Claude subagents moved to `intent claude subagents`
  - Added socrates and worker-bee subagents
- **v2.4.0 (February 2026)**: Skills, treeindex, and Claude upgrade:
  - `intent claude skills` for skill lifecycle management
  - `intent claude upgrade` for project LLM guidance upgrades
  - `intent treeindex` / `intent fileindex` for codebase navigation
  - 4 built-in skills: intent-essentials, intent-elixir-essentials, intent-ash-ecto-essentials, intent-phoenix-liveview
  - 302 tests across 15 test files
