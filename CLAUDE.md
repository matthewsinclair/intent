# Intent

This project is built with Intent -- run `intent --version` for the version it is running. The primary config file for AI coding agents is `AGENTS.md` at the project root -- read that first. `CLAUDE.md` is a Claude Code-specific overlay that adds directives beyond the tool-agnostic contract.

## Required on every session

Run `/in-session` immediately after session start and after every `/compact` or context reset. It reads the project's declared languages and loads the right skills (`/in-essentials`, `/in-standards`, plus language-specific). Rationale: `intent/docs/working-with-llms.md#skills-and-in-session-auto-load` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Persistent memory

Claude Code persists cross-session memories at `~/.claude/projects/<project-dir>/memory/`. Notes about user preferences, design decisions not derivable from code, and project context live there. See Claude Code's memory docs for management.

## Session hooks

`.claude/settings.json` is written by `intent claude upgrade --apply` -- **not** by `intent upgrade`, and not by default. **Decline it with `intent claude upgrade --apply --skip-settings`**: the run leaves `.claude/settings.json` as it found it, absent or yours, reports it as `skipped`, and applies the rest of canon. When the file is installed it wires Claude Code lifecycle hooks: `SessionStart` (inject project context + `/in-session` reminder), `UserPromptSubmit` (strict gate -- block first prompt until `/in-session` runs), `Stop` (remind `/in-finish` at wrap-up). Each dispatches through `intent claude hook <name>`; the hook BODIES are served from the running Intent install's own `lib/templates/.claude/scripts/`, found from the `intent` executable's location (`$INTENT_HOME` is not read), not from this project, so a hook fix reaches every project as soon as the installed Intent carries it, without touching `.git/hooks/`. Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture` in the Intent source repository (https://github.com/matthewsinclair/intent).

## File map

- `AGENTS.md` -- primary tool-agnostic contract. Read first.
- `usage-rules.md` -- terse DO / NEVER rules (Elixir convention; honoured by `mix usage_rules.sync`). Seeded by `intent claude upgrade --apply` when absent and never overwritten after that; `intent init` does not create it.
- `intent/llm/MODULES.md` -- OPTIONAL Highlander registry. `intent init` does not create one; a project that wants it creates the file and keeps it. Where it exists, search it with `intent modules find <name>` rather than reading it -- a mature registry is too large to read.
- `intent/llm/DECISION_TREE.md` -- OPTIONAL code-placement flow chart, Elixir/Phoenix-specific. `intent init` does not create one; where a project has one, it was chosen for that project.
- `intent/llm/RULES.md`, `intent/llm/ARCHITECTURE.md` -- this project's own rules and architecture, seeded empty by `intent init` for the project to author.
- `intent/` -- steel threads (`st/`), project docs (`docs/`), work tracking (`wip.md`, `restart.md`).
- `intent/.config/` -- configuration and metadata.

Canon narrative on how AGENTS.md + CLAUDE.md + usage-rules.md + hooks + critics + skills compose: `intent/docs/working-with-llms.md` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Rules of the road

Cross-language principles govern all Intent projects. Every language pack concretises them; the critics enforce them.

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure Function, Impure Coordination. Read it with `intent claude rules show IN-AG-PFIC-001`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

**THIS INDEX IS ALSO IN `AGENTS.md`, DELIBERATELY, AND A DRIFT TEST HOLDS THE TWO BYTE-IDENTICAL.** This section used to say the principles were stated in `AGENTS.md` and **not restated here**, on the grounds that a second copy would be a Highlander violation in the document that defines the rule. That reasoning was sound and its outcome was wrong: **`AGENTS.md` is the one file the Claude Code agent never receives**, so the pointer could not land for this file's own primary reader.

**HIGHLANDER GOVERNS IMPLEMENTATIONS, NOT INDEXES, AND THE TEST IS WHAT MAKES THE COPY LEGITIMATE.** The rule BODIES live in the rule library and are served by `intent claude rules show <id>`; there is exactly one of each and that is untouched. What is duplicated here is a table of contents pointing at them. **A copy that cannot silently diverge is not the failure mode Highlander names** -- drift is -- so the duplication is held by a test rather than by discipline.

**AND THE HONEST LIMIT, MEASURED RATHER THAN ASSUMED: THE OTHER HOME IS `in-standards/SKILL.md`, NOT `usage-rules.md`.** Driven 2026-09-04: `usage-rules.md` names the principles in passing -- a skill description, a pointer to the rules directory, a rule-id format example -- and carries no index of them; the `_usage-rules.md` template carries nothing at all. **`intent/plugins/claude/skills/in-standards/SKILL.md` does carry a real index**, every id with its slug, as a TABLE. It cannot join a byte-identity test because it is a different RENDERING by design, not a copy of these bytes, and it reaches installed projects through `intent claude skills sync` rather than through `claude upgrade --apply`. **So this arrangement takes the two root-file templates into a tested pair and leaves `in-standards/SKILL.md` as a declared exception whose divergence is intended. It does not reach zero and does not claim to.**

Read any of them with `intent claude rules show <id>` (`intent claude rules list` to enumerate, `--lang <lang>` to filter).

## Critic dispatch

Per-language rule enforcement via thin subagents that read the rule library at invocation:

```
Task(subagent_type="critic-<lang>", prompt="review <paths>")
Task(subagent_type="critic-<lang>", prompt="test-check <paths>")
```

`/in-review` reads the declared languages and dispatches. The installed Intent tool's headless runner (`intent critic <lang>`) powers the pre-commit gate. Contract: `intent/docs/critics.md` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Project-specific

<!-- user:start -->
<!-- Author: matts, created 2026-04-25. Intent dogfoods its own canon -- this CLAUDE.md is the reference example of the WP09 overlay template applied to a real project. Preserved across regeneration. -->

### Intent dev rules (extend the agnostic rules above)

1. **Highlander Rule** -- search `intent/llm/MODULES.md` before creating any new module, helper, or template: `intent modules find <name>`, falling back to `grep -n '<name>' intent/llm/MODULES.md` if it does not answer. **Search it, do not read it** -- a mature registry is too large to read; `wc -c intent/llm/MODULES.md` reports its size.
2. **Thin coordinators** (concretises `IN-AG-THIN-COORD-001`) -- business logic lives in the library crate (`native/rust/crates/intentsvcs/`), never in the CLI's dispatch or render code (`native/rust/crates/intent-cli/src/`).
3. **No silent failures** (concretises `IN-AG-NO-SILENT-001`) -- every error path returns a typed error carrying a remedy (`intentsvcs::remedy::Remedy`); nothing is rescued and swallowed.
4. **Check before you create** -- before adding a new script or function, search the registry as in rule 1.
5. **Register before you code** -- when you must create a new module, add the row to MODULES.md FIRST, then create the file.
6. **Single template source** -- root files and `init`'s starter content render from `lib/templates/` through the Rust token expander (`intentsvcs::rootfiles::substitute`); never restate a template's content inline. Generated views (`info.md`, `acceptance.md`, `steel_threads.md`, `todo.md`) are rendered by `intentsvcs::views`.

### Intent-specific files

- `intent/wip.md` -- current work in progress (read on session start).
- `intent/restart.md` -- session restart context (post-compact resume).
- `native/rust/crates/` -- Intent CLI source (the v3 Rust workspace); `bin/` holds dev tooling (`devbin`, `int`).
- `lib/templates/` -- generated-content source of truth.
- `intent/plugins/` -- plugin canon (`claude/`, `agents/`).

### Internal authoring docs

The canon Critic dispatch section above already points at `intent/docs/critics.md`. More authoring guides live alongside:

- `intent/docs/rules.md` -- rule-library authoring guide (schema, Detection heuristics, attribution).
- `intent/docs/writing-extensions.md` -- user-extension authoring guide (subagents, skills, rule packs at `~/.intent/ext/`). Extensions are declared and not built in this release: `intent ext` refuses and nothing reads `~/.intent/ext/`.

### Commit conventions

- DO NOT ADD CLAUDE TO GIT COMMITS. EVER. No `Co-Authored-By`, no Claude signatures, no AI attribution.
- T-shirt sizing only (XS / S / M / L / XL / XXL); never clock-time estimates.
- NEVER manually wrap lines in markdown files.

### Migration history

Intent originated as STP, migrated to Intent v2.0.0 on 2025-07-16, then through v2.1.0 -> v2.2.0 -> v2.3.0 -> v2.8.x -> v2.9.0 -> v2.10.0 -> v2.11.0. v2.10.0 ships the canonical LLM config (this overlay pattern, the root-file canon AGENTS.md / CLAUDE.md / usage-rules.md, session hooks, pre-commit critic gate) and relocates `.intent/` to `intent/.config/`. v2.11.0 (ST0037) replaces filesystem-marker language detection with an explicit `languages` config field. Later versions, including the v3 Rust rewrite, are in `CHANGELOG.md`; `intent --version` reports the running one.

### Author

matts (hello@matthewsinclair.com)

<!-- user:end -->

---

_Generated by Intent v3.0.1 from `lib/templates/llm/_CLAUDE.md`._
