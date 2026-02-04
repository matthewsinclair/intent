---
verblock: "04 Feb 2026:v0.4: matts - WP03 complete (WP02 skipped), CLAUDE.md integration"
intent_version: 2.3.4
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

1. **`intent treeindex <dir>`** -- A bash CLI command that walks directories bottom-up, uses `claude -p` (headless mode) to summarize source files, and writes `.treeindex` files with fingerprint-based staleness detection. Treeindex files are stored in a centralized shadow directory at `intent/.treeindex/`.

2. **Treeindex subagent** -- A Claude Code subagent (`intent/plugins/claude/subagents/treeindex/`) that knows the `.treeindex` format and can generate/maintain them when asked within a Claude session.

## Key Design Decisions

- **Centralized shadow directory**: `.treeindex` files live at `intent/.treeindex/<rel_path>/.treeindex`, not inline in each directory. Keeps source tree clean, single `.gitignore` entry
- **Bottom-up generation**: Leaf directories are summarized first; parent summaries reference child `.treeindex` files
- **Fingerprint-based staleness**: 8-char SHA256 hash of filenames + file sizes (no mtime dependency, works across git clones)
- **`.treeindexignore` file**: Gitignore-style config at `intent/.treeindex/.treeindexignore` for excluding directories and files from indexing. Auto-created with sensible defaults on first run
- **Haiku model**: Uses Claude Haiku 4.5 by default -- fast and cost-effective for structured summarization
- **Bash 3.2 compatible**: macOS ships bash 3.2.57; script avoids `mapfile`, `[[ ]]` for path checks, heredocs inside `$()`, and `+=` string concatenation
- **CLAUDE.md convention for consumption**: Claude is instructed to check `.treeindex` before reading files in unfamiliar directories
- **Committed to git**: `.treeindex` files are project documentation, not ephemeral cache

## Work Packages

- **WP01**: `bin/intent_treeindex` CLI command -- **COMPLETE**
- **WP02**: Treeindex Claude Code subagent -- **SKIPPED** (folded into WP03; CLI command + CLAUDE.md convention sufficient)
- **WP03**: Integration (CLAUDE.md, tests) -- **COMPLETE**

## Related Steel Threads

- None (new capability)
