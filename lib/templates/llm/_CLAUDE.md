# [[PROJECT_NAME]]

This project uses Intent v[[INTENT_VERSION]]. The primary config file for AI coding agents is `AGENTS.md` at the project root -- read that first. `CLAUDE.md` is a Claude Code-specific overlay that adds directives beyond the tool-agnostic contract.

## Required on every session

Run `/in-session` immediately after session start and after every `/compact` or context reset. It reads the project's declared languages and loads the right skills (`/in-essentials`, `/in-standards`, plus language-specific). Rationale: `intent/docs/working-with-llms.md#skills-and-in-session-auto-load` at the Intent install.

## Persistent memory

Claude Code persists cross-session memories at `~/.claude/projects/<project-dir>/memory/`. Notes about user preferences, design decisions not derivable from code, and project context live there. See Claude Code's memory docs for management.

## Session hooks

`.claude/settings.json` wires Claude Code lifecycle hooks: `SessionStart` (inject project context + `/in-session` reminder), `UserPromptSubmit` (strict gate -- block first prompt until `/in-session` runs), `Stop` (remind `/in-finish` at wrap-up). Hook scripts live under `.claude/scripts/`. Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture` at the Intent install.

## File map

- `AGENTS.md` -- primary tool-agnostic contract. Read first.
- `usage-rules.md` -- terse DO / NEVER rules (Elixir convention; honoured by `mix usage_rules.sync`).
- `intent/llm/MODULES.md` -- Highlander registry; check before creating new modules.
- `intent/llm/DECISION_TREE.md` -- code-placement flow chart.
- `intent/` -- steel threads (`st/`), project docs (`docs/`), work tracking (`wip.md`, `restart.md`).
- `intent/.config/` -- configuration and metadata.

Canon narrative on how AGENTS.md + CLAUDE.md + usage-rules.md + hooks + critics + skills compose: `intent/docs/working-with-llms.md` at the Intent install.

## Rules of the road

Four cross-language principles govern all Intent projects:

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure-Functional-Idiomatic-Coordination; pattern match, pipe, tag, compose.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

Rule files are served by the installed Intent tool, not vendored into this project -- read them with `intent claude rules show <id>` (`intent claude rules list` to enumerate, `--lang <lang>` to filter). The terse DO / NEVER contract for this project lives in `usage-rules.md`.

## Critic dispatch

Per-language rule enforcement via thin subagents that read the rule library at invocation:

```
Task(subagent_type="critic-<lang>", prompt="review <paths>")
Task(subagent_type="critic-<lang>", prompt="test-check <paths>")
```

`/in-review` reads the declared languages and dispatches. The installed Intent tool's headless runner (`intent critic <lang>`) powers the pre-commit gate. Contract: `intent/docs/critics.md` at the Intent install.

## Project-specific

<!-- user:start -->
<!-- Author: [[AUTHOR]], created [[DATE]]. Add project-specific Claude directives below this line. Preserved across regeneration. -->

<!-- user:end -->

---

_Generated from `lib/templates/llm/_CLAUDE.md` on [[DATE]] for Intent v[[INTENT_VERSION]]._
