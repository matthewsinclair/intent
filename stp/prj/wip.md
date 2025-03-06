---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# Work In Progress

## Current Focus

**003-TODO: Build a test suite**

We need to build a test suite that can be run in the STP project to test that bootstrap and init work as expected. Please create a new ST for this work.

**002-TODO: Consider potential use of Anthropic MCP? - New, to be done**

Given that we have the potential for some of the STP scripts to need to invoke an LLM is there an option here to create an MCP implementation (or proxy or whatever) so that we can more robustly have the LLM control itself in a controlled manner? For example, in the case of the "$ stp st done STID" command, is that something that we could parameterise (via the steel thread id) and then have the LLM call itself using an MCP interaction? Would that work? Hmm. 🤔 Please create a new ST for this work, but this is low priority for now and can be done later.

**001-DONE: Updating directory references from "doc" to "stp" - Done**

Completed:

1. Updated all shell scripts (bootstrap, stp, stp_help, stp_init, stp_st) to reflect new directory structure
2. Updated all documentation files with correct directory references
3. Confirmed all relative and absolute paths now use "stp" instead of "doc"

## Active Steel Threads

| ID                    | Title                  | Status      | Created    | Completed  |
|-----------------------|------------------------|-------------|------------|------------|
| None                  |                        |             |            |            |

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

1. Complete the remaining tasks for ST0001 and ST0002
2. Begin work on ST0003 (Template System)
3. Update the technical product design with any new insights
4. Create additional help documentation as needed

### Questions to Address

- Should we add a configuration command to customize STP behavior?
- How can we best handle version control integration?
- What additional canned prompts would be most useful for common tasks?
