---
description: "Ash Framework database-access rules: domain code interfaces, actor on query/changeset"
---

# Ash/Ecto Essentials

Load the Intent Ash rule pack into context. All database access in Ash-framework projects goes through Ash — never raw Ecto in application code. Authoritative upstream reference: `deps/ash/usage-rules.md`.

## Procedure

### 1. Load the Ash rules

Read each `RULE.md` on demand. The full text lives in `intent/plugins/claude/rules/elixir/ash/<slug>/RULE.md`.

| Rule ID         | Slug                   | What it enforces                                                        |
| --------------- | ---------------------- | ----------------------------------------------------------------------- |
| `IN-EX-ASH-001` | `code-interfaces-only` | All web/worker code hits the domain via a code interface, not `Ash.*`.  |
| `IN-EX-ASH-002` | `actor-on-query`       | Actor goes on `for_read/3` or `for_create/3`, not on the terminal call. |

### 2. Additional operational conventions

Not yet first-class rules — treat as mandatory Ash discipline:

- **Migrations via `mix ash.codegen`.** Never write Ecto migrations by hand for Ash resources. `mix ash.codegen <name>` reads resource definitions and produces correct migrations; `mix ash.migrate` applies them.
- **Code-interface options over manual pipelines.** Prefer `MyApp.Content.list_posts!(query: [filter: [...], sort: [...], limit: 20], actor: user)` over hand-rolled `Ash.Query.filter |> Ash.Query.sort |> Ash.read!` pipelines in web modules.
- **Custom change/validation modules, not anonymous functions.** `change MyApp.Content.Changes.SlugifyTitle` instead of `change fn changeset, _ -> ... end`. Modules are testable and reusable.
- **Atomic changes preferred.** Implement `atomic/3` whenever the change can be expressed as a database-level update. Use `require_atomic? false` only when external I/O (API call, computation on external data) genuinely forces non-atomic behaviour.
- **`require Ash.Query` at module level.** `Ash.Query.filter/2` is a macro. The `require` belongs at the top of the module alongside other requires, never inside a function body.

### 3. Check Ash Usage Rules

When the task deepens, read `deps/ash/usage-rules.md` for the upstream authoritative contract. Intent's rules are a subset — framework Usage Rules are the ground truth for anything this skill doesn't cover.

## Red Flags

| Rationalisation                                          | Reality                                                                         |
| -------------------------------------------------------- | ------------------------------------------------------------------------------- |
| "I'll just `Ash.get!` here; it's one line."              | See IN-EX-ASH-001. The one-line bypass is how policies die quietly.             |
| "Actor on `Ash.read!` is fine; same effect."             | See IN-EX-ASH-002. Not the same — calculations and policies see `nil`.          |
| "`mix ecto.gen.migration` is faster than `ash.codegen`." | It is also wrong. Ash needs to read your resource DSL to produce the migration. |
