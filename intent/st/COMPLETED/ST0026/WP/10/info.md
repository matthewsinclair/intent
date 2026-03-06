---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-10
title: "Integrator Command (D1)"
scope: Medium
status: Done
---

# WP-10: Integrator Command (D1)

## Objective

Implement `intent init --with-st0000` and `intent st zero` as the single entry points that bootstrap a project with all ST0000 countermeasures. This is the LAST WP -- it wraps everything from WP-01 through WP-09.

## Command Interface

```bash
intent init --with-st0000              # New project with full ST0000
intent init MyProject --with-st0000    # Named new project
intent st zero                         # Run ST0000 on existing project
intent st zero --check                 # Verify ST0000 compliance
```

## What It Does

`intent init --with-st0000` runs standard `intent init` then:

1. Create CLAUDE.md from enhanced template (D2)
2. Create `intent/llm/RULES.md` from language template
3. Create `intent/llm/MODULES.md` from template (D3)
4. Create `intent/llm/DECISION_TREE.md` from template (D6)
5. Create `intent/llm/ARCHETYPES.md` from template (D4)
6. Create `.intent/learnings.md` starter (D10)
7. Scaffold Credo checks if Elixir project (D5a)
8. Run `intent claude prime` for memory injection (D8)
9. Install all `in-*` skills (WP-01/WP-02)

`intent st zero` delegates to retrofit (D12/WP-09) for existing projects.

`intent st zero --check` validates all ST0000 deliverables present and current.

## Implementation

- Modify `bin/intent_init` to accept `--with-st0000` flag
- Add `bootstrap_st0000()` function orchestrating steps 1-9
- Each step idempotent (safe to re-run)
- Create `bin/intent_st_zero` for the `st zero` subcommand

## Acceptance Criteria

- [ ] `intent init --with-st0000` creates fully equipped project
- [ ] `intent st zero` works on existing Intent projects
- [ ] `intent st zero --check` reports compliance status
- [ ] `intent audit quick` on result produces zero violations
- [ ] All steps idempotent

## Dependencies

- Depends on: ALL other WPs (this is the integrator)
- LAST WP to implement
