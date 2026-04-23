---
description: "Elixir coding rules: pattern matching, tagged tuples, with-railways, @impl true, Highlander"
rules:
  - IN-EX-CODE-001
  - IN-EX-CODE-002
  - IN-EX-CODE-003
  - IN-EX-CODE-004
  - IN-EX-CODE-005
  - IN-EX-CODE-006
---

# Elixir Essentials

Load the Intent Elixir rule pack into context. Invoke at the start of any Elixir coding task or after a context reset.

## Procedure

### 1. Load the production-code rules

Read each `RULE.md` on demand when the situation matches. The full text of each rule lives in `intent/plugins/claude/rules/elixir/code/<slug>/RULE.md`.

| Rule ID          | Slug                              | What it enforces                                           |
| ---------------- | --------------------------------- | ---------------------------------------------------------- |
| `IN-EX-CODE-001` | `pattern-match-over-conditionals` | Multi-clause heads beat nested `if`/`case` on shape.       |
| `IN-EX-CODE-002` | `tagged-tuple-returns`            | `{:ok, v}` / `{:error, r}` instead of bare `nil`.          |
| `IN-EX-CODE-003` | `impl-true-on-callbacks`          | `@impl true` on every behaviour callback.                  |
| `IN-EX-CODE-004` | `with-for-railway`                | Chain fallible operations with `with`, not nested `case`.  |
| `IN-EX-CODE-005` | `no-silent-failures`              | Never `rescue _ -> :ok`; failures are handled or raised.   |
| `IN-EX-CODE-006` | `module-highlander`               | One canonical module per concern; no divergent duplicates. |

Each rule file follows the same shape: `## Problem`, `## Detection`, `## Bad`, `## Good`, `## When This Applies`, `## When This Does Not Apply`, `## Further Reading`. Read the specific rule when you hit the situation it covers.

### 2. Additional operational conventions

These are not yet first-class rules in the library; treat them as mandatory style.

- **Pipe operator for sequential transformations.** Two or more transformations use pipes, first argument is always the data being transformed.
- **Naming conventions.** `?` suffix for booleans (`active?/1`). `!` suffix for raising variants (`fetch!/1`). `_name` for unused args with meaning.
- **Assertive data access.** `struct.field` for required keys (fails fast). `map[:key]` only for genuinely-optional keys. Pattern-match to destructure.
- **No debug artifacts.** Never commit `IO.inspect/2`, `dbg()`, or `IO.puts` for debugging. Use `dbg()` during development; strip before commit.

### 3. Check Usage Rules for framework behaviour

If the task touches a framework, consult the framework's Usage Rules:

- `deps/ash/usage-rules.md`
- `deps/ash_postgres/usage-rules.md`
- `deps/phoenix_live_view/usage-rules.md`
- Any other `deps/*/usage-rules.md` or `deps/*/AGENTS.md` relevant to the task

## Red Flags

| Rationalisation                         | Reality                                                            |
| --------------------------------------- | ------------------------------------------------------------------ |
| "A single `if` is fine here."           | See IN-EX-CODE-001. If the branch is on shape, use multi-clause.   |
| "I'll return `nil` on not-found."       | See IN-EX-CODE-002. Tag failure; callers must see it.              |
| "@impl true is clutter."                | See IN-EX-CODE-003. One typo catches 20 minutes of debugging.      |
| "Nested `case` is clearer than `with`." | See IN-EX-CODE-004. Three-deep pyramid is the Elixir arrow-code.   |
| "I'll just `rescue _ -> :ok` for now."  | See IN-EX-CODE-005. "For now" becomes production's silent failure. |
