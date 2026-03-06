---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-04
title: "Memory Injection (D8)"
scope: Medium
status: Done
---

# WP-04: Memory Injection (D8)

## Objective

Implement `intent claude prime` -- the command that pre-loads Claude Code's persistent project memory with operational knowledge, project rules, canonical modules, and footguns. This solves session amnesia.

## Command Interface

```bash
intent claude prime                    # Prime current project
intent claude prime --refresh          # Full overwrite of existing MEMORY.md
intent claude prime --dry-run          # Show what would be written
intent claude prime --from <project>   # Import learnings from another project
```

## Implementation

### New Script

`intent/plugins/claude/bin/intent_claude_prime` following established command pattern.

**Algorithm:**

1. Source helpers, load config, require project root
2. Compute memory path: `~/.claude/projects/<HASH>/memory/MEMORY.md`
   - Path: absolute project path, `/` replaced with `-`, prepended with `-`
3. Read source files (all optional, skip gracefully if missing):
   - `intent/llm/RULES.md`
   - `intent/llm/MODULES.md`
   - `intent/llm/DECISION_TREE.md`
   - `intent/llm/ARCHETYPES.md`
   - `.intent/learnings.md`
4. Read bundled operational knowledge: `lib/templates/prime/operational-knowledge.md`
5. Synthesize into structured MEMORY.md (must stay under 200 lines)
6. Write to memory path (merge or overwrite per flags)

### Memory File Output Structure

```markdown
# <Project Name> Project Memory

## Operational Knowledge

[Intent commands, skill invocation, session workflow]

## Project Rules

[Key rules condensed from RULES.md]

## Canonical Modules

[Module ownership from MODULES.md]

## Decision Tree

[Quick reference from DECISION_TREE.md]

## Known Footguns

[From .intent/learnings.md]

## Session Checklist

Re-read on every context reset:

- intent/llm/RULES.md
- intent/llm/MODULES.md
- CLAUDE.md
```

### Bundled Operational Knowledge

New file: `lib/templates/prime/operational-knowledge.md`

Contents:

- Intent commands (st, wp, agents, claude, treeindex, plugin)
- Skill invocation patterns (/in-\* skills)
- Session workflow conventions
- Common mistakes and fixes
- Convention reminders (no Claude in commits, markdown table alignment, etc.)

### Plugin Registration

Add to `intent/plugins/claude/plugin.json`:

```json
{
  "syntax": "intent claude prime [--refresh] [--dry-run] [--from <project>]",
  "description": "Pre-load Claude Code project memory with rules, modules, and operational knowledge"
}
```

### Help

Add `prime` section to `lib/help/claude.help.md`.

## Design Decisions

- **200-line limit**: MEMORY.md is auto-loaded into every Claude Code conversation. Lines after 200 are truncated. Content must be condensed, not verbatim.
- **Default merge**: Without `--refresh`, prime merges new/updated sections into existing MEMORY.md, preserving user-added sections.
- **Bundled knowledge**: Operational knowledge ships with Intent, not per-project. Same for all projects.
- **Path computation**: Must match Claude Code's internal algorithm exactly.

## Acceptance Criteria

- [ ] `intent claude prime` writes well-structured MEMORY.md
- [ ] `intent claude prime --refresh` overwrites completely
- [ ] `intent claude prime --dry-run` shows output without writing
- [ ] Memory file stays under 200 lines
- [ ] Missing source files handled gracefully
- [ ] New Claude Code session has rules pre-loaded

## Dependencies

- Depends on: WP-03 (templates must exist to read)
- Blocks: WP-10 (integrator runs prime as final step)
