# [[PROJECT_NAME]]

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

Four cross-language principles govern all Intent projects. Every language pack concretises them; the critics enforce them.

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure Function, Impure Coordination. Read it with `intent claude rules show IN-AG-PFIC-001`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

**THIS INDEX IS ALSO IN `AGENTS.md`, DELIBERATELY, AND A DRIFT TEST HOLDS THE TWO BYTE-IDENTICAL.** This section used to say the four were stated in `AGENTS.md` and **not restated here**, on the grounds that a second copy would be a Highlander violation in the document that defines the rule. That reasoning was sound and its outcome was wrong: **`AGENTS.md` is the one file the Claude Code agent never receives**, and it does not exist at all in a fresh project until `intent agents sync` runs -- so the pointer could not land for this file's own primary reader.

**HIGHLANDER GOVERNS IMPLEMENTATIONS, NOT INDEXES, AND THE TEST IS WHAT MAKES THE COPY LEGITIMATE.** The rule BODIES live in the rule library and are served by `intent claude rules show <id>`; there is exactly one of each and that is untouched. What is duplicated here is a table of contents pointing at them. **A copy that cannot silently diverge is not the failure mode Highlander names** -- drift is -- so the duplication is held by a test rather than by discipline.

**AND THE HONEST LIMIT, MEASURED RATHER THAN ASSUMED: THE THIRD HOME IS `in-standards/SKILL.md`, NOT `usage-rules.md`.** Driven 2026-09-04: `usage-rules.md` names the four principles in passing -- a skill description, a pointer to the rules directory, a rule-id format example -- and carries no index of them; the `_usage-rules.md` template carries nothing at all. **`intent/plugins/claude/skills/in-standards/SKILL.md` does carry a real index**, all four ids with slugs, as a TABLE. It cannot join a byte-identity test because it is a different RENDERING by design, not a copy of these bytes, and it reaches installed projects through `intent claude skills sync` rather than through `claude upgrade --apply`. **So this arrangement takes two of three homes into a tested pair and leaves one declared exception whose divergence is intended. It does not reach zero and does not claim to.**

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
<!-- Author: [[AUTHOR]]. Add project-specific Claude directives below this line. Preserved across regeneration. -->

<!-- user:end -->

---

_Generated by Intent v[[INTENT_VERSION]] from `lib/templates/llm/_CLAUDE.md`._
