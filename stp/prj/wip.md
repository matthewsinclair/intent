---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# Work In Progress

## Current Focus

**003-DONE: Build a test suite**

Created ST0011 for implementing a comprehensive test suite that will verify bootstrap, init, and other core STP functions work as expected across different environments.

**002-DONE: Consider potential use of Anthropic MCP?**

Created ST0010 to explore the potential use of Anthropic's Machine Control Protocol (MCP) for allowing STP scripts to interact with LLMs in a more controlled manner. This is low priority work that can be addressed later.

**001-DONE: Updating directory references from "doc" to "stp"**

Completed:

1. Updated all shell scripts (bootstrap, stp, stp_help, stp_init, stp_st) to reflect new directory structure
2. Updated all documentation files with correct directory references
3. Confirmed all relative and absolute paths now use "stp" instead of "doc"

## Active Steel Threads

| ID                    | Title                  | Status      | Created    | Completed  |
|-----------------------|------------------------|-------------|------------|------------|
| [ST0011](./st/ST0011.md) | Test Suite Implementation | Not Started | 2025-06-03 |            |
| [ST0010](./st/ST0010.md) | Anthropic MCP Integration | Not Started | 2025-06-03 |            |

## Upcoming Work

| ID                    | Title                  | Status      | Created    | Completed  |
|-----------------------|------------------------|-------------|------------|------------|
| None                  |                        |             |            |            |

## Notes

- The directory name for engineering documentation has been changed from "des" to "eng" to better reflect its content
- The Technical Product Design (TPD) document has been split into separate files for easier management
- All templates use markdown format for maximum portability
- Scripts follow POSIX compatibility for cross-platform support
- The system is designed to be lightweight with minimal dependencies

## Context for LLM

This document captures the current state of development on the Steel Thread Project. When beginning work with an LLM assistant, start by sharing this document to provide context about what's currently being worked on.

### Next Steps

1. Begin implementation of ST0011 (Test Suite Implementation)
2. Research and document MCP capabilities for ST0010
3. Update the technical product design with any new insights
4. Create additional help documentation as needed

### Questions to Address

- Should we add a configuration command to customize STP behavior?
- How can we best handle version control integration?
- What additional canned prompts would be most useful for common tasks?
