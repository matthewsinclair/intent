---
verblock: "03 Feb 2026:v0.1: matts - Initial version"
wp: "02"
title: "Treeindex Claude Code subagent"
status: Not Started
---
# WP02: Treeindex Claude Code Subagent

## Objective

Create a Claude Code subagent that knows the `.treeindex` format and can generate/maintain `.treeindex` files when invoked within a Claude session via the Task tool.

## Scope

### Subagent Files

```
intent/plugins/claude/subagents/treeindex/
  agent.md        -- system prompt (~150-200 lines)
  metadata.json   -- agent metadata
```

### agent.md Sections

1. **Identity and role** -- treeindex generator and maintainer
2. **Format specification** -- exact `.treeindex` format (header, H1, summary, Dirs, Files)
3. **Fingerprint functions** -- bash functions for computing and checking staleness
4. **Generation algorithm** -- bottom-up, skip up-to-date, progress reporting
5. **File reading strategy** -- what to read per file type:
   - Elixir: moduledoc, public functions, struct defs, use/import (not function bodies unless <50 lines)
   - Config files: read full (typically small)
   - Markdown: first 10 lines
   - Never: binaries, lock files, generated files, `.beam`, `_build`, `deps`
6. **Summary writing guidelines** -- purpose not mechanics, one-liners for files, importance ordering
7. **Interaction patterns** -- generate, check, regenerate, force
8. **Ignore lists** -- directories and files to skip

### agent.md Frontmatter

```yaml
---
name: treeindex
description: Generates and maintains .treeindex directory summaries for fast codebase navigation
tools: Bash, Read, Write, Glob, Grep
---
```

### metadata.json

```json
{
  "name": "treeindex",
  "version": "1.0.0",
  "description": "Generates and maintains .treeindex directory summaries for fast codebase navigation",
  "author": "Intent Contributors",
  "tools": ["Bash", "Read", "Write", "Glob", "Grep"],
  "tags": ["treeindex", "navigation", "documentation", "directory-summary", "codebase-orientation"]
}
```

## Files to Create

| File                                                      | Action |
|-----------------------------------------------------------|--------|
| `intent/plugins/claude/subagents/treeindex/agent.md`      | Create |
| `intent/plugins/claude/subagents/treeindex/metadata.json` | Create |

## Dependencies

- WP01 should be complete first (so the subagent prompt can reference the CLI command)
- Understanding of existing subagent patterns (elixir, intent, socrates)

## Acceptance Criteria

- [ ] `agent.md` follows existing frontmatter conventions (name, description, tools)
- [ ] `metadata.json` follows existing schema (name, version, description, author, tools, tags)
- [ ] Format specification in agent.md matches the CLI command's output exactly
- [ ] Fingerprint functions in agent.md match the CLI command's implementation
- [ ] Subagent can generate valid `.treeindex` files when invoked via Task tool
- [ ] No Lamplight-specific or project-specific references in the prompt
