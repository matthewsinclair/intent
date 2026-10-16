@AGENTS.md

# [[PROJECT_NAME]]

This project is built with Intent -- run `intent --version` for the version it is running. The primary config file for AI coding agents is `AGENTS.md`, beside this file, and the `@AGENTS.md` line at the top of this file loads it. `CLAUDE.md` is a Claude Code-specific overlay that adds directives beyond the tool-agnostic contract.

## Required on every session

Run `/in-session` immediately after session start and after every `/compact` or context reset. It reads the project's declared languages and loads the right skills (`/in-essentials`, `/in-standards`, plus language-specific). Rationale: `intent/docs/working-with-llms.md#skills-and-in-session-auto-load` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Finding code

To find where a symbol is defined or where a name is used, ask Intent's index before grep: load `mcp__intent__intent_search` through ToolSearch, then call it with `kind` set to `def` and the name as `query`, or with `context` set to the name. Fall back to grep only when its answer says the index is not complete for the paths involved. The index knows a file's language only by its extension, so a script without one has no symbols whatever it is written in, and it names symbols only in the languages the project declares and this build can parse (`intent index status` shows each declared language's grammar); a def, outline or context answer that could reach such files says it is not complete and names the language under `unindexed`.

## Persistent memory

Claude Code persists cross-session memories at `~/.claude/projects/<project-dir>/memory/`. Notes about user preferences, design decisions not derivable from code, and project context live there. See Claude Code's memory docs for management.

## Session hooks

`.claude/settings.json` is written by `intent claude upgrade --apply` -- **not** by `intent upgrade`, and not by default. An existing `.claude/settings.json` that does not name `intent claude hook` is held rather than overwritten unless `--force` is given. **Decline it with `intent claude upgrade --apply --skip-settings`**: the run leaves `.claude/settings.json` and `.mcp.json` as it found them, absent or yours, reports each as `skipped`, and applies the rest of canon. When the file is installed it wires Claude Code lifecycle hooks: `SessionStart` (inject project context + `/in-session` reminder), `UserPromptSubmit` (strict gate -- block first prompt until `/in-session` runs), `Stop` (remind `/in-finish` at wrap-up). Each dispatches through `intent claude hook <name>`; the hook BODIES are served from the running Intent install's own `lib/templates/.claude/scripts/`, found from the `intent` executable's location (`$INTENT_HOME` is not read), not from this project, so a hook fix reaches every project as soon as the installed Intent carries it, without touching `.git/hooks/`. Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture` in the Intent source repository (https://github.com/matthewsinclair/intent).

## File map

- `AGENTS.md` -- primary tool-agnostic contract. Read first.
- `usage-rules.md` -- terse DO / NEVER rules (Elixir convention; honoured by `mix usage_rules.sync`). Seeded by `intent claude upgrade --apply` in the root layout when absent, not seeded in the `.claude/` layout, and never overwritten; `intent init` does not create it.
- `intent/llm/MODULES.md` -- OPTIONAL Highlander registry. To check for prior art, ask the index first: `intent search --kind def <name>` answers whether a thing with that name already exists anywhere in the tree, and the answer carries the index's own freshness. When it says the index is not complete for the paths that matter, fall back to grep. Where the project keeps a registry, `intent modules find <name>` searches that as well. `intent init` does not create one; a project that wants it creates the file and keeps it, and searches it rather than reading it -- a mature registry is too large to read.
- `intent/llm/DECISION_TREE.md` -- OPTIONAL code-placement flow chart. `intent init` does not create one; where a project has one, it was chosen for that project and describes that project's own stack.
- `intent/llm/RULES.md`, `intent/llm/ARCHITECTURE.md` -- this project's own rules and architecture, seeded empty by `intent init` for the project to author.
- `intent/` -- steel threads (`st/`), project docs (`docs/`), work tracking (`wip.md`, `restart.md`).
- `intent/.config/` -- configuration and metadata.

Canon narrative on how AGENTS.md + CLAUDE.md + usage-rules.md (where the project keeps one) + hooks + critics + skills compose: `intent/docs/working-with-llms.md` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Rules of the road

The cross-language principles, Highlander, PFIC, Thin Coordinator and No Silent Errors, are indexed under "Rules of the Road" in `AGENTS.md`, which the `@AGENTS.md` line at the top of this file loads into every Claude Code session. Read any of them with `intent claude rules show <id>` (`intent claude rules list` to enumerate, `--lang <lang>` to filter).

## Critic dispatch

Per-language rule enforcement via thin subagents that read the rule library at invocation:

```
Task(subagent_type="critic-<lang>", prompt="review <paths>")
Task(subagent_type="critic-<lang>", prompt="test-check <paths>")
```

`/in-review` reads the declared languages and dispatches. The installed Intent tool's headless runner (`intent critic <lang>`) powers the pre-commit gate. Contract: `intent/docs/critics.md` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Project-specific

<!-- user:start -->
<!-- Author: [[AUTHOR]]. Add project-specific Claude directives below this line. Preserved across regeneration. -->

<!-- user:end -->

---

_Generated by Intent v[[INTENT_VERSION]] from `lib/templates/llm/_CLAUDE.md`._
