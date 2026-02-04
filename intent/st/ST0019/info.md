---
verblock: "03 Feb 2026:v0.2: matts - Populated from design session"
intent_version: 2.2.0
status: WIP
created: 20260203
completed:
---
# ST0019: Treeindex -- Directory Summaries for Claude Navigation

## Objective

Build an `intent treeindex` CLI command and companion Claude Code subagent that generates `.treeindex` files -- concise directory summaries that let Claude quickly orient itself in a codebase without reading every file.

## Problem

Every Claude Code session spends significant context on codebase exploration. Claude reads files, uses Glob/Grep, and runs Explore agents -- all to answer the question "what's in this directory and which files matter?" Pre-computed directory summaries eliminate this repeated cost.

## Solution

Two deliverables:

1. **`intent treeindex <dir>`** -- A bash CLI command that walks directories bottom-up, uses `claude -p` (headless mode) to summarize source files, and writes `.treeindex` files with fingerprint-based staleness detection.

2. **Treeindex subagent** -- A Claude Code subagent (`intent/plugins/claude/subagents/treeindex/`) that knows the `.treeindex` format and can generate/maintain them when asked within a Claude session.

## Key Design Decisions

- **Bottom-up generation**: Leaf directories are summarized first; parent summaries reference child `.treeindex` files
- **Fingerprint-based staleness**: 8-char SHA256 hash of filenames + file sizes (no mtime dependency, works across git clones)
- **CLAUDE.md convention for consumption**: Claude is instructed to check `.treeindex` before reading files in unfamiliar directories
- **Committed to git**: `.treeindex` files are project documentation, not ephemeral cache

## Work Packages

- **WP01**: `bin/intent_treeindex` CLI command
- **WP02**: Treeindex Claude Code subagent
- **WP03**: Integration and registration (CLAUDE.md, global-agents.json, help, tests)

## Related Steel Threads

- None (new capability)

## Context for LLM

Plan file: `/Users/matts/.claude/plans/nested-tinkering-reddy.md`
