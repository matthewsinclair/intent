---
verblock: "05 Mar 2026:v0.2: matts - As-built"
wp_id: WP-08
title: "Guardrails (D9, D11)"
scope: Medium
status: Done
---

# WP-08: Guardrails (D9, D11)

## Objective

Proactive guardrails that prevent violations at the point of creation rather than catching them after the fact.

## Deliverables

### D9: New Module Checklist

Three-tier enforcement:

**Tier 1 -- CLAUDE.md instruction** (shipped with D2/WP-03):

- "Check MODULES.md before creating any new module"
- Steps: check registry, extend or delegate if exists, register FIRST if new

**Tier 2 -- Claude Code hook template** (this WP):

- Template at `lib/templates/hooks/module_check_hook.json`
- Advisory PostToolUse hook for Write|Edit operations
- Users install by copying into `.claude/settings.json`

**Tier 3 -- `intent modules check` command**:

```bash
intent modules check              # Scan for unregistered modules
intent modules check --register   # Interactive registration
intent modules find "auth"        # Find canonical module for concern
```

### D11: Umbrella Dependency Graph Enforcement

**Template** at `lib/templates/llm/_DEPENDENCY_GRAPH.md` with `[[PROJECT_NAME]]` placeholder.

**Credo check** at `lib/templates/credo_checks/elixir/dependency_graph.ex`:

- `Mix.Checks.DependencyGraph`, id `EX4007`, `run_on_all: true`
- Reads rules from `intent/llm/DEPENDENCY_GRAPH.md`
- Walks AST for alias/import/use, infers target app via `Macro.underscore`
- Integrated via `intent audit quick --rule D11`

## Acceptance Criteria

- [x] D9: CLAUDE.md instruction present (WP-03)
- [x] D9: Hook template created
- [x] D9: `intent modules check` works
- [x] D11: Dependency graph template created
- [x] D11: Credo check created and integrated with audit

## Also Delivered

- Rationalized CLI output across all 14+ commands to Rust-style conventions
- Added `--help`/`-h` flag support to `intent st`
- Registered 8 previously-missing bin scripts in MODULES.md
- Removed stale `lib/help/general.help.md` entry from MODULES.md

## Dependencies

- D9 depends on: WP-03 (MODULES.md must exist)
- D11 depends on: WP-06 (integrates with audit quick)
