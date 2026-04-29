---
verblock: "24 Apr 2026:v0.2: matts - Cancelled (superseded by v2.9.x skills/subagents/extensions)"
intent_version: 2.9.1
status: Cancelled
created: 20250603
completed: 20260424
---

> **Deprecated 2026-04-24.** Superseded by Intent v2.9.0 — the skill / subagent / extension system (`~/.intent/ext/`, `intent ext`, critic-_ family, `/in-_` skills) provides the parameterised LLM-interaction surface this ST was scoping. Cancelled under ST0035 (canonical LLM config). The original objective and context below are preserved for historical reference only.

# ST0010: Anthropic MCP Integration

- **Status**: Cancelled
- **Created**: 2025-06-03
- **Completed**: 2026-04-24
- **Author**: Matthew Sinclair

## Objective

Explore and implement the potential use of Anthropic's Machine Control Protocol (MCP) to enable STP scripts to interact with LLMs in a more robust and controlled manner.

## Context

Several STP commands could benefit from LLM integration, such as "stp st done STID" and other operations that might require AI assistance. Currently, these interactions are not standardized. Using MCP could provide a more structured and reliable way for STP scripts to leverage LLM capabilities programmatically.

This steel thread explores whether an MCP implementation (or proxy) could allow STP scripts to more effectively control LLM interactions, potentially enabling the LLM to call itself in a parameterized way from within the STP framework.

## Related Steel Threads

- ST0002: Core Script Framework
- ST0004: Steel Thread Commands

## Context for LLM

This steel thread explores the integration of Anthropic's Machine Control Protocol (MCP) into the STP system to enhance LLM interactions within STP scripts. This is a low-priority exploration for now that can be implemented later after more critical components are in place.

### How to update this document

1. Update the status as work progresses
2. Check off tasks as they are completed
3. Add implementation notes as decisions are made or challenges encountered
4. Add results when the steel thread is completed
5. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
