# Intent

This project is built with Intent -- run `intent --version` for the version it is running. The primary config file for AI coding agents is `AGENTS.md` at the project root -- read that first. `CLAUDE.md` is a Claude Code-specific overlay that adds directives beyond the tool-agnostic contract.

## Required on every session

Run `/in-session` immediately after session start and after every `/compact` or context reset. It reads the project's declared languages and loads the right skills (`/in-essentials`, `/in-standards`, plus language-specific). Rationale: `intent/docs/working-with-llms.md#skills-and-in-session-auto-load` at the Intent install.

## Persistent memory

Claude Code persists cross-session memories at `~/.claude/projects/<project-dir>/memory/`. Notes about user preferences, design decisions not derivable from code, and project context live there. See Claude Code's memory docs for management.

## Session hooks

`.claude/settings.json` is written by `intent claude upgrade --apply` -- **not** by `intent upgrade`, and not by default. **There is currently no flag to decline it**: v2's `intent claude upgrade --skip-settings` was not carried into v3, whose flags on that verb are `--apply` and `--force`. When the file is installed it wires Claude Code lifecycle hooks: `SessionStart` (inject project context + `/in-session` reminder), `UserPromptSubmit` (strict gate -- block first prompt until `/in-session` runs), `Stop` (remind `/in-finish` at wrap-up). Each dispatches through `intent claude hook <name>`; the hook BODIES are served from the Intent install (`$INTENT_HOME/lib/templates/.claude/scripts/`), not from this project, so a hook fix reaches every project on the next `intent upgrade` without touching `.git/hooks/`. Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture` at the Intent install.

## File map

- `AGENTS.md` -- primary tool-agnostic contract. Read first.
- `usage-rules.md` -- terse DO / NEVER rules (Elixir convention; honoured by `mix usage_rules.sync`).
- `intent/llm/MODULES.md` -- OPTIONAL Highlander registry. `intent init` does not create one; a project that wants it creates the file and keeps it. Where it exists, search it with `intent modules find <name>` rather than reading it -- a mature registry is too large to read.
- `intent/llm/DECISION_TREE.md` -- code-placement flow chart.
- `intent/` -- steel threads (`st/`), project docs (`docs/`), work tracking (`wip.md`, `restart.md`).
- `intent/.config/` -- configuration and metadata.

Canon narrative on how AGENTS.md + CLAUDE.md + usage-rules.md + hooks + critics + skills compose: `intent/docs/working-with-llms.md` at the Intent install.

## Rules of the road

The four cross-language principles -- Highlander, PFIC, Thin Coordinator, No Silent Errors -- are stated in `AGENTS.md` under "Rules of the Road", with their rule IDs. **They are not restated here**: AGENTS.md is the tool-agnostic contract and this file is the Claude-specific overlay, so a second copy would be a Highlander violation in the document that defines the rule.

Read any of them with `intent claude rules show <id>` (`intent claude rules list` to enumerate, `--lang <lang>` to filter).

## Critic dispatch

Per-language rule enforcement via thin subagents that read the rule library at invocation:

```
Task(subagent_type="critic-<lang>", prompt="review <paths>")
Task(subagent_type="critic-<lang>", prompt="test-check <paths>")
```

`/in-review` reads the declared languages and dispatches. The installed Intent tool's headless runner (`intent critic <lang>`) powers the pre-commit gate. Contract: `intent/docs/critics.md` at the Intent install.

## Project-specific

<!-- user:start -->
<!-- Author: matts, created 2026-04-25. Intent dogfoods its own canon -- this CLAUDE.md is the reference example of the WP09 overlay template applied to a real project. Preserved across regeneration. -->

### Intent dev rules (extend the four agnostic rules above)

1. **Highlander Rule** -- search `intent/llm/MODULES.md` before creating any new module, helper, or template: `grep -n '<name>' intent/llm/MODULES.md`. **Search it, do not read it** -- the registry is ~354KB across ~367 rows, so "read it first" is not an instruction anyone can follow. (`intent modules find` is the intended verb: **drive it, and fall back to the grep above if it does not answer.** Which spelling works is a property of the tool version you are standing on, and pinning that answer here would make this a second home for a fact the tool already reports about itself.)
2. **Thin scripts** (concretises `IN-AG-THIN-COORD-001`) -- business logic lives in dedicated modules under `bin/` or `intent/plugins/`, never inline in command dispatch or heredocs.
3. **No silent failures** (concretises `IN-AG-NO-SILENT-001`) -- every error path uses `error()` from `bin/intent_helpers`.
4. **Check before you create** -- before adding a new script or function, search the registry as in rule 1.
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

Intent originated as STP, migrated to Intent v2.0.0 on 2025-07-16, then through v2.1.0 -> v2.2.0 -> v2.3.0 -> v2.8.x -> v2.9.0 -> v2.10.0 -> v2.11.0 (current). v2.10.0 ships the canonical LLM config (this overlay pattern, three-file canon AGENTS.md / CLAUDE.md / usage-rules.md, session hooks, pre-commit critic gate) and relocates `.intent/` to `intent/.config/`. v2.11.0 (ST0037) replaces filesystem-marker language detection with an explicit `languages` config field. See `CHANGELOG.md` for per-version detail.

### Author

matts (hello@matthewsinclair.com)

<!-- user:end -->

---

_Generated by Intent v3.0.0 from `lib/templates/llm/_CLAUDE.md`._
