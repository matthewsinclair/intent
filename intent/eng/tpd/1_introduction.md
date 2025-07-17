---
verblock: "17 Jul 2025:v2.0.0: Matthew Sinclair - Updated for Intent v2.0.0 (As-Built)"
intent_version: 2.0.0
---
# 1. Introduction

[index](<./technical_product_design.md>)

## 1.1 Purpose

Intent (formerly the Steel Thread Process or STP) is a system designed to create a structured workflow and documentation process for developers working collaboratively with Large Language Models (LLMs) such as Claude Code. Intent provides templates, scripts, and process guidelines to enhance productivity while ensuring high-quality documentation as a byproduct of the development process.

### 1.1.1 The Rebrand to Intent

In July 2025, STP was rebranded to "Intent" to better reflect the system's core purpose: capturing and preserving the intention behind software development decisions. While the methodology remains the "Steel Thread Process," the tool itself is now Intent.

## 1.2 Scope

Intent v2.0.0 encompasses:

- A flattened directory structure under `intent/` for organizing documentation
- JSON-based configuration system (.intent/config.json)
- Shell scripts for managing Intent workflows (intent_* commands)
- The Steel Thread Process methodology for incremental development
- Enhanced integration patterns for working with LLMs
- Advanced Backlog.md integration with status filtering
- Automated steel thread status synchronization
- Migration tools for upgrading from any STP version
- Diagnostic and setup tools (doctor, bootstrap)

Intent is designed to be lightweight, adaptable, and to work alongside existing development workflows without requiring significant changes to development practices.

## 1.3 Definitions

| Term           | Definition                                                                                       |
|----------------|--------------------------------------------------------------------------------------------------|
| Intent         | The tool and framework for intention-aware development (v2.0.0)                                  |
| Steel Thread   | A self-contained unit of work that represents a logical piece of functionality to be implemented |
| LLM            | Large Language Model, an AI system capable of understanding and generating text                  |
| Context Window | The amount of text an LLM can process in a single interaction                                    |
| Backlog        | Task management system integrated with Intent for tracking fine-grained work items               |
| Task           | Individual unit of work linked to a steel thread, tracked in Backlog                             |
| Bootstrap      | Initial global setup process for Intent installation                                             |
| Doctor         | Diagnostic tool for identifying and fixing Intent configuration issues                           |

## 1.4 System Overview

Intent v2.0.0 operates as a meta-layer on top of existing development processes. It provides structure for:

1. **Documentation Management**: Flattened structure under `intent/` for all documentation
2. **LLM Collaboration**: Enhanced guidelines and tools for effective AI assistance
3. **Incremental Development**: The Steel Thread methodology for breaking work into manageable units
4. **Project Tracking**: Work-in-progress tracking and project history
5. **Task Management**: Advanced Backlog.md integration with configurable status filtering
6. **Status Synchronization**: Automatic steel thread status updates based on task completion
7. **Configuration Management**: JSON-based configuration with hierarchy support
8. **Migration Support**: Tools to upgrade from any previous STP version
9. **Self-Hosting**: Intent is developed using Intent itself

The system remains intentionally simple, using markdown files and shell scripts to maximize portability and minimize dependencies.

## 1.5 References

- Modern LLM-assisted development practices
- Documentation-as-code methodologies
- Incremental development processes
- Intent Blog Series (docs/blog/)
- Migration from STP to Intent v2.0.0

## 1.6 Version History

- **v0.0.0 - v1.2.1**: Original Steel Thread Process (STP) development
- **v2.0.0 (July 2025)**: Complete rebrand to Intent with:
  - Flattened directory structure
  - JSON configuration system
  - New bootstrap, doctor, and upgrade commands
  - Enhanced Backlog.md integration
  - Self-hosting capability
