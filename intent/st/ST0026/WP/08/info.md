---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-08
title: "Guardrails (D9, D11)"
scope: Medium
status: Not Started
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

**Tier 2 -- Claude Code hook** (this WP):

- Fires on `Write` to `lib/**/*.ex`
- Checks if module path is registered in MODULES.md
- Advisory warning (does not block)

**Tier 3 -- `intent modules check` command**:

```bash
intent modules check              # Scan for unregistered modules
intent modules check --register   # Interactive registration
intent modules find "auth"        # Find canonical module for concern
```

### D11: Umbrella Dependency Graph Enforcement

**Context**: Lamplight `output.ex` imported from `llclient` app, violating the dependency graph. Compiled fine in dev but was an architectural boundary violation.

**Definition** in `intent/llm/DEPENDENCY_GRAPH.md`:

```markdown
| App  | May depend on | Must NOT depend on |
| ---- | ------------- | ------------------ |
| core | (nothing)     | web, cli, workers  |
| web  | core          | cli, workers       |
```

**Enforcement**: Check that scans `alias`/`import`/`use` statements and flags cross-app references violating the declared graph. Integrates with `intent audit quick`.

**Implementation options**:

- Custom Credo check: `DependencyGraphViolation`
- Standalone script
- Integrated into `intent audit quick --rule D11`

## Acceptance Criteria

- [ ] D9: CLAUDE.md instruction present
- [ ] D9: Hook functional (advisory warning on unregistered modules)
- [ ] D9: `intent modules check` works
- [ ] D11: Dependency graph definition format specified
- [ ] D11: Enforcement check catches known violations

## Dependencies

- D9 depends on: WP-03 (MODULES.md must exist)
- D11 depends on: WP-06 (integrates with audit quick)
