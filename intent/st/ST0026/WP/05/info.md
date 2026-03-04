---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-05
title: "Archetype Templates (D4)"
scope: Medium
status: Not Started
---

# WP-05: Archetype Templates (D4)

## Objective

Create starter templates for common module types that are ALREADY correct. New modules start from a template that enforces thin coordinators, proper callback annotations, and correct architecture from line one.

## Deliverables

### Elixir Archetype Templates

| Archetype          | File                        | Key enforcement                                       |
| ------------------ | --------------------------- | ----------------------------------------------------- |
| Ash Domain         | `ash_domain.ex.eex`         | `resources` block, code interfaces, no business logic |
| Ash Resource       | `ash_resource.ex.eex`       | Actions, policies, attributes skeleton                |
| Phoenix Controller | `phoenix_controller.ex.eex` | Thin: parse params, call service, render              |
| LiveView           | `live_view.ex.eex`          | Two-phase mount, handle_event delegates to service    |
| Service Module     | `service.ex.eex`            | Pure business logic, tagged tuples, @spec             |
| CLI Command        | `cli_command.ex.eex`        | BaseCommand, config macro, delegate to service        |
| GenServer          | `genserver.ex.eex`          | @impl on all callbacks, init/handle\_\* skeleton      |
| Oban Worker        | `oban_worker.ex.eex`        | @impl perform/1, tagged tuple returns                 |
| Test Module        | `test.ex.eex`               | ExUnit skeleton with describe blocks, setup           |

Each template includes:

- Module skeleton with correct structure
- `@impl true` on all callbacks
- Comments: `# DO NOT put business logic here -- delegate to service modules`
- Comments: `# Check MODULES.md before creating new modules`
- Anti-pattern warnings as inline comments

### ARCHETYPES.md Reference

Create `lib/templates/llm/_ARCHETYPES.md`:

- Lists all available archetypes with when to use each
- Cross-references decision tree (D6)
- Key anti-patterns for each archetype

### Location

Templates at `lib/templates/archetypes/elixir/` (language-scoped for future Rust, Swift, Lua).

### Phase 1 vs Phase 2

- **Phase 1 (this WP)**: Templates exist as reference docs in ARCHETYPES.md. Claude uses them as patterns.
- **Phase 2 (future)**: `intent generate <archetype> <module_name>` command with variable substitution.

## Acceptance Criteria

- [ ] All 9 Elixir archetype templates created
- [ ] ARCHETYPES.md reference document created
- [ ] Templates are syntactically valid Elixir (with placeholders)
- [ ] Each template includes anti-pattern comments
- [ ] Templates follow existing code style conventions

## Dependencies

- None (standalone)
- Blocks: WP-04 (memory injection references ARCHETYPES.md)
