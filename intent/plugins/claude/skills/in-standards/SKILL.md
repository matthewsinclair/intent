---
description: "Coding standards: agnostic principles (Highlander, PFIC, Thin Coordinator, No Silent Errors) + project rules"
---

# Coding Standards

Load coding discipline into context. Invoke at the start of coding or after any context reset.

## Procedure

### 1. Re-read project rules

- `CLAUDE.md` (project rules)
- `intent/llm/MODULES.md` (module registry)
- `intent/llm/DECISION_TREE.md` (code placement)

### 2. Load the agnostic rule pack

These are the cross-language principles. Every language pack (Elixir, Rust, Swift, Lua, Shell) concretises them. Read each `RULE.md` on demand when the situation matches; the full text lives in `intent/plugins/claude/rules/agnostic/<slug>/RULE.md`.

| Rule ID                | Slug               | What it enforces                                                           |
| ---------------------- | ------------------ | -------------------------------------------------------------------------- |
| `IN-AG-HIGHLANDER-001` | `highlander`       | There can be only one. No divergent copies of the same concern.            |
| `IN-AG-PFIC-001`       | `pfic`             | Pure-Functional-Idiomatic-Coordination: pattern match, pipe, tag, compose. |
| `IN-AG-THIN-COORD-001` | `thin-coordinator` | Coordinators parse → call → render. Business logic lives elsewhere.        |
| `IN-AG-NO-SILENT-001`  | `no-silent-errors` | Every failure is surfaced. Rescue-and-swallow is forbidden.                |

Each rule has `concretised_by:` language-specific rules. For Elixir: `IN-EX-CODE-006` concretises Highlander, `IN-EX-CODE-004` concretises PFIC (with-railway), `IN-EX-PHX-001` / `IN-EX-LV-003` concretise Thin Coordinator, `IN-EX-CODE-005` concretises No Silent Errors.

### 3. Load relevant framework Usage Rules

- `deps/ash/usage-rules.md`
- `deps/ash_postgres/usage-rules.md`
- `deps/phoenix_live_view/usage-rules.md`
- Any other `deps/*/usage-rules.md` or `deps/*/AGENTS.md` relevant to the task

### 4. Load the language skill when coding begins

The agnostic rules are universal; the language-specific application lives in the language skill:

- Elixir: `/in-elixir-essentials`, `/in-elixir-testing`, `/in-ash-ecto-essentials`, `/in-phoenix-liveview`
- (Rust/Swift/Lua/Shell language skills land in later WPs.)

### 5. Formatting standards

- All markdown tables must be column-aligned.
- No non-printing characters (proper ASCII, emojis only if explicitly requested).
- No em dashes in skill files (multi-byte truncation bug in list display).
- 2-space indentation in all code, in all languages, in Intent.

## Red Flags

| Rationalisation                                 | Reality                                                                       |
| ----------------------------------------------- | ----------------------------------------------------------------------------- |
| "This helper is only used once, it's OK."       | Check MODULES.md. Someone else may already own it.                            |
| "The coordinator needs this logic inline."      | See IN-AG-THIN-COORD-001. If it's not parse/call/render, extract it.          |
| "Pattern matching is overkill here."            | See IN-AG-PFIC-001. Pattern matching is the default tool.                     |
| "Rescuing and returning :ok is easier."         | See IN-AG-NO-SILENT-001. Easier now; invisible in prod.                       |
| "Two copies are fine; they're almost the same." | See IN-AG-HIGHLANDER-001. "Almost the same" becomes "drift" within a quarter. |
