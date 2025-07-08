---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.0.0
---
# 1. Introduction

[index](<./technical_product_design.md>)

## 1.1 Purpose

The Steel Thread Process (STP) is a system designed to create a structured workflow and documentation process for developers working collaboratively with Large Language Models (LLMs) such as Claude Code. STP provides templates, scripts, and process guidelines to enhance productivity while ensuring high-quality documentation as a byproduct of the development process.

## 1.2 Scope

STP encompasses:

- A directory structure for organizing project documentation
- Template documents for technical design, user guides, and development tracking
- Shell scripts for managing the STP workflow
- A process methodology centered around "steel threads" of work
- Integration patterns for working with LLMs
- Integration with Backlog.md for fine-grained task management
- Commands for synchronizing steel thread status with task completion

STP is designed to be lightweight, adaptable, and to work alongside existing development workflows without requiring significant changes to development practices.

## 1.3 Definitions

| Term           | Definition                                                                                       |
|----------------|--------------------------------------------------------------------------------------------------|
| Steel Thread   | A self-contained unit of work that represents a logical piece of functionality to be implemented |
| LLM            | Large Language Model, an AI system capable of understanding and generating text                  |
| Context Window | The amount of text an LLM can process in a single interaction                                    |
| Canned Prompt  | A pre-defined, reusable instruction template for an LLM                                          |
| Backlog        | Task management system integrated with STP for tracking fine-grained work items                  |
| Task           | Individual unit of work linked to a steel thread, tracked in Backlog                             |

## 1.4 System Overview

STP operates as a meta-layer on top of existing development processes. It provides structure for:

1. **Documentation Management**: Templates and organization for technical, user, and process documentation
2. **LLM Collaboration**: Guidelines and tools for effective LLM assistance in development
3. **Incremental Development**: A methodology for breaking work into manageable "steel threads"
4. **Project Tracking**: Mechanisms for tracking work-in-progress and project history
5. **Task Management**: Integration with Backlog.md for fine-grained task tracking linked to steel threads
6. **Status Synchronization**: Automatic status updates based on task completion metrics

The system is intentionally simple, using markdown files and shell scripts to maximize portability and minimize dependencies.

## 1.5 References

- Modern LLM-assisted development practices
- Documentation-as-code methodologies
- Incremental development processes
