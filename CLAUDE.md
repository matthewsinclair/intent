# Intent

This project uses Intent v2.10.0. The primary config file for AI coding agents is `AGENTS.md` at the project root -- read that first. `CLAUDE.md` is a Claude Code-specific overlay that adds directives beyond the tool-agnostic contract.

## Required on every session

Run `/in-session` immediately after session start and after every `/compact` or context reset. It auto-detects the project language and loads the right skills (`/in-essentials`, `/in-standards`, plus language-specific). Rationale: `intent/docs/working-with-llms.md#skills-and-in-session-auto-load`.

## Persistent memory

Claude Code persists cross-session memories at `~/.claude/projects/<project-dir>/memory/`. Notes about user preferences, design decisions not derivable from code, and project context live there. See Claude Code's memory docs for management.

## Session hooks

`.claude/settings.json` wires Claude Code lifecycle hooks: `SessionStart` (inject project context + `/in-session` reminder), `UserPromptSubmit` (strict gate -- block first prompt until `/in-session` runs), `Stop` (remind `/in-finish` at wrap-up). Hook scripts live under `.claude/scripts/`. Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture`.

## File map

- `AGENTS.md` -- primary tool-agnostic contract. Read first.
- `usage-rules.md` -- terse DO / NEVER rules (Elixir convention; honoured by `mix usage_rules.sync`).
- `intent/docs/working-with-llms.md` -- canon narrative on how AGENTS.md + CLAUDE.md + usage-rules.md + hooks + critics + skills compose.
- `intent/llm/MODULES.md` -- Highlander registry; check before creating new modules.
- `intent/llm/DECISION_TREE.md` -- code-placement flow chart.
- `intent/` -- steel threads (`st/`), project docs (`docs/`), work tracking (`wip.md`, `restart.md`).
- `.intent/` -- configuration and metadata.

## Rules of the road

Four cross-language principles govern all Intent projects:

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure-Functional-Idiomatic-Coordination; pattern match, pipe, tag, compose.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

Full rule files live at `intent/plugins/claude/rules/agnostic/`. The terse DO / NEVER contract for this project lives in `usage-rules.md`. Language-specific concretisations at `intent/plugins/claude/rules/<lang>/`.

## Critic dispatch

Per-language rule enforcement via thin subagents that read the rule library at invocation:

```
Task(subagent_type="critic-<lang>", prompt="review <paths>")
Task(subagent_type="critic-<lang>", prompt="test-check <paths>")
```

`/in-review` auto-detects language and dispatches. Headless runner `bin/intent_critic` powers the pre-commit gate. Contract: `intent/docs/critics.md`.

## Project-specific

<!-- user:start -->
<!-- Author: matts, created 2026-04-25. Intent dogfoods its own canon -- this CLAUDE.md is the reference example of the WP09 overlay template applied to a real project. Preserved across regeneration. -->

### Intent dev rules (extend the four agnostic rules above)

1. **Highlander Rule** -- check `intent/llm/MODULES.md` before creating any new module, helper, or template.
2. **Thin scripts** (concretises `IN-AG-THIN-COORD-001`) -- business logic lives in dedicated modules under `bin/` or `intent/plugins/`, never inline in command dispatch or heredocs.
3. **No silent failures** (concretises `IN-AG-NO-SILENT-001`) -- every error path uses `error()` from `bin/intent_helpers`.
4. **Check before you create** -- before adding a new script or function, check `intent/llm/MODULES.md`.
5. **Register before you code** -- when you must create a new module, add the row to MODULES.md FIRST, then create the file.
6. **Single template source** -- all generated content comes from `lib/templates/` via `sed` substitution. No inline heredocs duplicating template content.

### Intent-specific files

- `intent/wip.md` -- current work in progress (read on session start).
- `intent/restart.md` -- session restart context (post-compact resume).
- `bin/` -- Intent CLI source.
- `lib/templates/` -- generated-content source of truth.
- `intent/plugins/` -- plugin canon (`claude/`, `agents/`).

### Internal authoring docs

The canon Critic dispatch section above already points at `intent/docs/critics.md`. Two more authoring guides live alongside:

- `intent/docs/rules.md` -- rule-library authoring guide (schema, Detection heuristics, attribution).
- `intent/docs/writing-extensions.md` -- user-extension authoring guide (subagents, skills, rule packs at `~/.intent/ext/`).

### Commit conventions

- DO NOT ADD CLAUDE TO GIT COMMITS. EVER. No `Co-Authored-By`, no Claude signatures, no AI attribution.
- T-shirt sizing only (XS / S / M / L / XL / XXL); never clock-time estimates.
- NEVER manually wrap lines in markdown files.

### Migration history

Intent originated as STP, migrated to Intent v2.0.0 on 2025-07-16, then through v2.1.0 -> v2.2.0 -> v2.3.0 -> v2.8.x -> v2.9.0 -> v2.10.0 (current). v2.10.0 ships the canonical LLM config (this overlay pattern, three-file canon AGENTS.md / CLAUDE.md / usage-rules.md, session hooks, pre-commit critic gate) and relocates `.intent/` to `intent/.config/`. See `CHANGELOG.md` for per-version detail.

### Author

matts (hello@matthewsinclair.com)

<!-- user:end -->

---

_Generated from `lib/templates/llm/_CLAUDE.md` on 2026-04-25 for Intent v2.10.0._
