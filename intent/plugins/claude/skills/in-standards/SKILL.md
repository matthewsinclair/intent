---
description: "Coding standards: re-read rules, usage rules, enforce Highlander and PFIC"
---

# Coding Standards

Load coding discipline into context. Invoke at the start of coding or after any context reset.

## Procedure

### 1. Re-read project rules

- `CLAUDE.md` (project rules)
- `intent/llm/MODULES.md` (module registry)
- `intent/llm/DECISION_TREE.md` (code placement)

### 2. Re-read relevant usage rules

Check for and read any usage rules files:

- `deps/ash/usage-rules.md`
- `deps/ash_postgres/usage-rules.md`
- `deps/phoenix_live_view/usage-rules.md`
- Any other `deps/*/usage-rules.md` or `deps/*/AGENTS.md` relevant to the current task

### 3. Enforce the Highlander Rule

- No duplicated code paths, ever
- Before creating a module, check MODULES.md
- Before adding a function, check if one already exists
- If tempted to copy-paste, refactor to share

### 4. Enforce Thin Coordinators

- CLI commands: parse args, call service, format output
- Controllers: parse params, call service, render
- LiveViews: handle events by delegating to services
- Oban workers: call service in perform/1
- GenServers: manage state, delegate logic to services

### 5. Enforce PFIC

Pure-Functional Idiomatic Code:

- Pattern matching over conditionals
- Tagged tuples (`{:ok, _}` / `{:error, _}`) for fallible operations
- Pipe chains for data transformations
- `@impl true` on all callbacks
- Descriptive function names (verbs for actions, nouns for getters)

### 6. Formatting standards

- All markdown tables must be column-aligned
- No non-printing characters (proper emojis and ASCII only)
- No em dashes in skill files
