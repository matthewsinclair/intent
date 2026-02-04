---
verblock: "03 Feb 2026:v0.1: matts - Initial version"
wp: "03"
title: "Integration and registration"
status: Not Started
---
# WP03: Integration and Registration

## Objective

Wire the treeindex subagent into the Intent plugin system, update project documentation, and verify everything works end-to-end.

## Scope

### 1. Register in global-agents.json

Add entry to `intent/plugins/claude/subagents/.manifest/global-agents.json`:

```json
{
  "name": "treeindex",
  "version": "1.0.0",
  "description": "Generates and maintains .treeindex directory summaries for fast codebase navigation",
  "path": "intent/plugins/claude/subagents/treeindex",
  "checksum": ""
}
```

### 2. Update CLAUDE.md -- Agent Listing

Add treeindex to the "Available Agents" section:

```markdown
5. **treeindex** - Directory summary generator
   - Generates and maintains .treeindex files
   - Fast codebase navigation and orientation
   - Bottom-up recursive directory summarization
```

Add "When to Use" guidance:

```markdown
**Use the treeindex agent for:**
- Generating .treeindex summaries for directories
- Checking staleness of existing .treeindex files
- Understanding the .treeindex format
```

### 3. Add .treeindex Convention to CLAUDE.md

Add new section:

```markdown
## .treeindex Files

This project uses `.treeindex` files to summarize directory contents.

- Before reading files in an unfamiliar directory, check for `.treeindex` first
- Use the summary to decide which files to read -- do not read every file
- If the fingerprint (line 1) looks stale, still use it as a starting point
- Do not modify `.treeindex` files directly; use the treeindex subagent to regenerate
```

### 4. Update intent help

Ensure `intent help` includes treeindex in its command listing.

### 5. Generate initial .treeindex for Intent itself

Run `intent treeindex intent/plugins/claude/subagents/ --depth 1` to create the first `.treeindex` files as a validation test.

## Files to Modify

| File                                                           | Action                                                          |
|----------------------------------------------------------------|-----------------------------------------------------------------|
| `intent/plugins/claude/subagents/.manifest/global-agents.json` | Add treeindex entry                                             |
| `CLAUDE.md`                                                    | Add agent listing + `.treeindex` convention                     |
| `bin/intent_help`                                              | Verify treeindex appears (may auto-detect from GLOBAL_COMMANDS) |

## Dependencies

- WP01 and WP02 must be complete

## Acceptance Criteria

- [ ] `intent claude subagents list` shows treeindex as available
- [ ] `intent claude subagents install treeindex` installs successfully
- [ ] `intent claude subagents show treeindex` displays correct metadata
- [ ] CLAUDE.md documents the treeindex agent and `.treeindex` convention
- [ ] `intent help` includes treeindex in command listing
- [ ] `tests/run_tests.sh` passes (no regressions)
- [ ] At least one `.treeindex` file generated for the Intent project itself as validation
