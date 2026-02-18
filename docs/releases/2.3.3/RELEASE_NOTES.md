# Intent v2.3.3 Release Notes

**Release Date:** October 2, 2025

## Overview

Intent v2.3.3 adds a comprehensive Elixir style guide to the Elixir Claude subagent, complementing the antipattern detection introduced in v2.3.2.

## New Features

### Elixir Style Guide

The Elixir subagent now includes comprehensive style guidelines covering:

- **Module Organization**: Import/alias ordering, whitespace conventions, avoiding multi-alias declarations
- **Function Definitions**: Multiline do/end preferences
- **Testing Style**: DSL layout patterns, fixture design, tiny_maps usage
- **Code Composition**: Clean line breaks, pipeline usage with utility functions
- **Naming and Organization**: File/module consistency, expressiveness, ubiquitous language
- **Documentation**: Professional documentation standards, decision records, FIXME conventions
- **Type Specifications**: Domain-specific typespecs, persisted vs non-persisted types, pattern matching with guards
- **Dependency Management**: Documented dependencies, validated function options
- **Database Design**: Precision in schema definitions
- **Version Control**: PR title conventions

Full documentation available at: `intent/plugins/claude/subagents/elixir/style.md`

## Enhancements

- Updated `agent.md` to reference the new style guide alongside antipatterns
- Elixir subagent now provides both antipattern detection (v2.3.2) and style guidance (v2.3.3)

## Upgrade Notes

### From v2.3.2

To upgrade from v2.3.2 to v2.3.3:

```bash
intent upgrade
```

If you have the Elixir subagent installed, update it:

```bash
intent claude subagents sync
```

### Migration Details

The v2.3.2 → v2.3.3 migration:

1. Updates project version in `.intent/config.json` and `VERSION`
2. No structural changes required
3. Elixir subagent enhancements are available via `intent claude subagents sync`

## Compatibility

- Fully compatible with Intent v2.3.0, v2.3.1, and v2.3.2
- No breaking changes
- Existing projects can upgrade seamlessly

## Files Changed

- `intent/plugins/claude/subagents/elixir/style.md` (new)
- `intent/plugins/claude/subagents/elixir/agent.md` (updated)
- `VERSION` (2.3.2 → 2.3.3)
- `.intent/config.json` (version fields updated)
- `bin/intent_helpers` (added migration functions)
- `bin/intent_upgrade` (added v2.3.3 upgrade paths)

## For More Information

- **Full Changelog**: See CHANGELOG.md
- **Elixir Style Guide**: `intent/plugins/claude/subagents/elixir/style.md`
- **Elixir Antipatterns**: `intent/plugins/claude/subagents/elixir/antipatterns.md`
- **Elixir Agent Config**: `intent/plugins/claude/subagents/elixir/agent.md`
