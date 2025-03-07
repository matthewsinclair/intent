---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.0.0
---
# LLM Preamble

This document provides essential context for LLMs working with the Steel Thread Project (STP) codebase.

## Project Overview

STP is a system designed to create a structured workflow and documentation process for developers collaborating with Large Language Models like you. It provides:

1. A standardized directory structure for project documentation
2. Shell scripts for managing project workflows
3. A methodology centered around "steel threads" - self-contained units of work
4. Markdown templates for documentation
5. Testing frameworks for ensuring reliability

The system is intentionally lightweight, using only shell scripts and markdown files to maximize portability and minimize dependencies. It integrates with existing development workflows and helps preserve context across development sessions with LLMs.

## Navigation Guide

When working with this repository, you should focus on these key documents in this specific order:

1. **START HERE**: `stp/eng/tpd/technical_product_design.md` - Contains comprehensive information about the project vision, architecture and current state. Pay special attention to the "Preamble to Claude" section.

2. **NEXT**: `stp/prj/st/steel_threads.md` - Provides a complete index of all steel threads with their status. Review this to understand what work has been completed and what remains.

3. **THEN**: `stp/prj/wip.md` - Details the current work in progress and priorities. This is your guide to what should be worked on now.

4. **FINALLY**: `stp/prj/journal.md` - Records the historical narrative of work completed. Reference this for context on previous development.

## Key System Components

The STP system consists of these main components:

1. **Core Script Framework**: Shell scripts in `stp/bin/` that manage steel threads and documentation workflow
   - `stp` - Main entry point script
   - `stp_init` - Initializes a new STP project
   - `stp_st` - Manages steel threads (new, list, show, done)
   - `stp_help` - Provides help information

2. **Documentation Structure**: Organized markdown files in specific directories
   - `stp/prj/` - Project management documents
   - `stp/eng/` - Engineering documentation
   - `stp/usr/` - User documentation
   - `stp/llm/` - LLM-specific content (like this preamble)

3. **Test Suite**: BATS-based tests in `stp/tests/` that verify functionality
   - Tests for core scripts, initialization, and steel thread management

## Current Status

The STP system has completed 11 steel threads so far, implementing all core functionality:

- Directory structure ✓
- Core script framework ✓
- Template system ✓
- Steel thread commands ✓
- Initialization ✓
- Help system ✓
- User documentation ✓
- LLM integration ✓
- Process refinement ✓
- Test suite implementation ✓

Future work (potential new steel threads) may include:

- Anthropic MCP integration
- CI/CD integration for automated testing
- Configuration commands for customizing STP behavior
- Enhanced version control integration

## Development Guidelines

1. **Code Style**:
   - Use 2-space indentation in any programming language
   - Follow language-specific conventions as noted in CLAUDE.md
   - Maintain POSIX compatibility for scripts to ensure cross-platform support

2. **Documentation**:
   - Keep markdown documents consistently formatted
   - Update documentation as part of any implementation work
   - Follow the verblock pattern for versioning (`verblock: "DD MMM YYYY:vX.Y: Author - Note"`)

3. **Steel Thread Process**:
   - Work is organized into steel threads (ST####)
   - Steel threads have states: Not Started, In Progress, Completed, On Hold, Cancelled
   - Each steel thread has its own markdown document in `stp/prj/st/`

## How to Help

When assisting with this project, you should:

1. First, understand the current context by reviewing the documents in the order specified
2. Focus on the work in progress as defined in `stp/prj/wip.md`
3. Maintain consistency with existing patterns and documentation standards
4. Update documentation alongside code changes
5. When suggesting improvements, reference relevant architectural patterns
6. Summarize completed work in the journal document

Most tasks will involve implementing new functionality, enhancing existing features, or improving documentation within the steel thread framework. If needed, use the shell scripts to create or update steel threads.
