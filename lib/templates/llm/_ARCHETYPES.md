# Archetype Templates

> When creating a new module, start from the appropriate archetype template.
> Each template enforces correct architecture from line one.
> Templates are in `lib/templates/archetypes/elixir/` in your Intent installation.

## Available Archetypes

| Archetype          | Template File               | When to Use                                  |
| ------------------ | --------------------------- | -------------------------------------------- |
| Ash Domain         | `ash_domain.ex.eex`         | New domain boundary for a group of resources |
| Ash Resource       | `ash_resource.ex.eex`       | New database-backed entity                   |
| Phoenix Controller | `phoenix_controller.ex.eex` | New HTTP endpoint (REST API or page)         |
| LiveView           | `live_view.ex.eex`          | New real-time interactive page               |
| Service Module     | `service.ex.eex`            | New business logic (the default choice)      |
| CLI Command        | `cli_command.ex.eex`        | New command-line interface command           |
| GenServer          | `genserver.ex.eex`          | New stateful process (caches, coordinators)  |
| Oban Worker        | `oban_worker.ex.eex`        | New background job                           |
| Test Module        | `test.ex.eex`               | New test file                                |

## Key Principles in Every Template

1. **THIN coordinators**: Controllers, LiveViews, CLI commands, and workers are thin. They parse input, call a service, and format output. No business logic.
2. **`@impl true`**: Every callback has `@impl true` annotation.
3. **Tagged tuples**: All public functions return `{:ok, result}` or `{:error, reason}`.
4. **Highlander comments**: Templates include "Check MODULES.md" reminders.
5. **Anti-pattern warnings**: Inline comments warn against common mistakes.

## Decision Guide

Not sure which archetype to use? See DECISION_TREE.md.

**The most common archetype is Service Module.** When in doubt, create a service module. It's easier to extract a service into a GenServer or worker later than to extract business logic out of a controller or LiveView.
