---
description: "Coding standards: agnostic principles (Highlander, PFIC, Thin Coordinator, No Silent Errors) + project rules"
---

# Coding Standards

Load coding discipline into context. Invoke at the start of coding or after any context reset.

## Procedure

### 1. Re-read project rules

- `CLAUDE.md` (project rules) -- always present.

The other two are both ABSENT unless somebody chose them, and their presence is therefore a decision. Until 2026-09-11 they had opposite dispositions -- `init` wrote the decision tree into every project whether it fitted or not -- so an older project may still carry one nobody chose.

- `intent/llm/MODULES.md` (module registry) -- **NOT written by `init`**, deliberately (no template for it ships, so `init` has nothing to write; ruled 2026-08-24: a hand-maintained index of a tree the store already indexes is the thing Highlander forbids). **Its absence is the normal case and is not a gap to fix.** To check for prior art, ask the index first: `intent search --kind def <name>` answers whether a thing with that name already exists anywhere in the tree, and the answer carries the index's own freshness. When it says the index is not complete for the paths that matter, fall back to grep. Where the project keeps a registry, `intent modules find <name>` searches that as well. Where a project keeps one anyway, **search the registry, never read it** -- a mature registry runs to hundreds of kilobytes, so "check it first" is not an instruction anyone can follow by reading.
- `intent/llm/DECISION_TREE.md` (code placement) -- **NOT written by `init` since 2026-09-11**, and no template for it ships. **Where a project has one, it was chosen for that project and describes that project's own stack**, so read it as that project's guidance. A project initialised before the change may still carry the Elixir/Phoenix copy `init` used to hand to every project whatever its language; outside an Elixir/Phoenix project, treat that unchosen copy as noise rather than as guidance.

### 2. Load the agnostic rule pack

These are the cross-language principles. Every language pack (Elixir, Rust, Swift, Lua, Shell) concretises them. Read each rule on demand when the situation matches; the full text is served by the installed Intent tool via `intent claude rules show <id>` (`intent claude rules list --lang agnostic` to enumerate -- the pack carries more than these principles).

| Rule ID                | Slug               | What it enforces                                                                 |
| ---------------------- | ------------------ | -------------------------------------------------------------------------------- |
| `IN-AG-HIGHLANDER-001` | `highlander`       | There can be only one. No divergent copies of the same concern.                  |
| `IN-AG-PFIC-001`       | `pfic`             | Pure Function, Impure Coordination -- `intent claude rules show IN-AG-PFIC-001`. |
| `IN-AG-THIN-COORD-001` | `thin-coordinator` | Coordinators parse -> call -> render. Business logic lives elsewhere.            |
| `IN-AG-NO-SILENT-001`  | `no-silent-errors` | Every failure is surfaced. Rescue-and-swallow is forbidden.                      |

Each rule has `concretised_by:` language-specific rules. For Elixir: `IN-EX-CODE-006` concretises Highlander, `IN-EX-CODE-004` concretises PFIC (with-railway), `IN-EX-PHX-001` / `IN-EX-LV-003` concretise Thin Coordinator, `IN-EX-CODE-005` concretises No Silent Errors.

### 3. Load relevant framework Usage Rules

- `deps/ash/usage-rules.md`
- `deps/ash_postgres/usage-rules.md`
- `deps/phoenix_live_view/usage-rules.md`
- Any other `deps/*/usage-rules.md` or `deps/*/AGENTS.md` relevant to the task
- Topical sub-rules: the `deps/*/usage-rules/*.md` folders that v1.x deps ship alongside the single file (eg `deps/usage_rules/usage-rules/{elixir,otp}.md`). Read these too, not just the top-level `usage-rules.md`.

### 4. Load the language skill when coding begins

The agnostic rules are universal; the language-specific application lives in the language skill:

- Elixir: `/in-elixir-essentials`, `/in-elixir-testing`, `/in-ash-ecto-essentials`, `/in-phoenix-liveview`
- Rust / Swift / Lua / Shell: no essentials skill, and that is not an omission -- `intent claude rules list --lang <lang>`, `intent claude rules show <id>`, applied by `critic-<lang>`.
- Prose: `/in-author-essentials`, `/in-content-essentials`.

### 5. Formatting standards

- All markdown tables must be column-aligned.
- No non-printing characters (proper ASCII, emojis only if explicitly requested).
- No em dashes in skill files (multi-byte truncation bug in list display).
- 2-space indentation in all code, in all languages, in Intent.

## Red Flags

| Rationalisation                                 | Reality                                                                       |
| ----------------------------------------------- | ----------------------------------------------------------------------------- |
| "This helper is only used once, it's OK."       | Search for an existing owner first; someone else may already have it.         |
| "The coordinator needs this logic inline."      | See IN-AG-THIN-COORD-001. If it's not parse/call/render, extract it.          |
| "This domain function can just call the API."   | See IN-AG-PFIC-001 (`intent claude rules show IN-AG-PFIC-001`).               |
| "Rescuing and returning :ok is easier."         | See IN-AG-NO-SILENT-001. Easier now; invisible in prod.                       |
| "Two copies are fine; they're almost the same." | See IN-AG-HIGHLANDER-001. "Almost the same" becomes "drift" within a quarter. |
