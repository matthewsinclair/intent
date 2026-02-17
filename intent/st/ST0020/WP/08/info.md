---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-08
title: Project Structure Reference Doc
scope: Small
status: Not Started
---

# WP-08: Project Structure Reference Doc

## Objective

Create a reference doc covering the standard Phoenix/Ash project layout for subagent consultation during code reviews.

## Deliverables

- `intent/plugins/claude/subagents/elixir/project-structure.md` (~150-250 lines)

## Coverage

- Standard Phoenix project directory layout
- Ash domain/resource organization patterns
- Web layer structure (controllers, LiveViews, components)
- Test directory mirroring conventions
- Configuration file locations
- Asset pipeline structure
- Where new code goes (decision tree)

## Acceptance Criteria

- [ ] Covers standard Phoenix/Ash project layout
- [ ] 150-250 lines
- [ ] Provides clear guidance on where new code should be placed
- [ ] Covers both traditional Phoenix contexts and Ash domain patterns

## Dependencies

- None (can run in parallel with WP-01)
