---
verblock: "20 Aug 2025:v0.1: matts - Initial version"
intent_version: 2.2.0
status: WIP
created: 20250820
completed: 
---
# ST0018: Upgrade Intent to support AGENTS.md

## Objective

Upgrade Intent to support the AGENTS.md specification - a standardized format for providing instructions to AI coding agents. This involves restructuring the existing agent system into a plugin architecture and adding AGENTS.md generation and management capabilities.

## Context

AGENTS.md is an emerging standard for providing instructions to AI coding agents (see https://agents.md/). Intent currently has a Claude Code subagent system but lacks support for the universal AGENTS.md standard. 

This upgrade involves:
1. Restructuring existing "intent agents" commands to "intent claude subagents"
2. Creating a plugin architecture under intent/plugins/
3. Implementing AGENTS.md generation and management as a plugin
4. Maintaining backward compatibility through migration tools

## Related Steel Threads

- ST0017: Add an Intent sub-agent for Claude Code
- ST0016: Rename STP CLI to INTENT (v2.0.0)

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.