@AGENTS.md

# Intent

This project is built with Intent -- run `intent --version` for the version it is running. The primary config file for AI coding agents is `AGENTS.md`, beside this file, and the `@AGENTS.md` line at the top of this file loads it. `CLAUDE.md` is a Claude Code-specific overlay that adds directives beyond the tool-agnostic contract.

## Required on every session

Run `/in-session` immediately after session start and after every `/compact` or context reset. It reads the project's declared languages and loads the right skills (`/in-essentials` plus language-specific). Rationale: `intent/docs/working-with-llms.md#skills-and-in-session-auto-load` in Intent's repository.

## Finding code

To find where a symbol is defined or where a name is used, ask Intent's index before grep: load `mcp__intent__intent_search` through ToolSearch, then call it with `kind` set to `def` and the name as `query`, or with `context` set to the name. Fall back to grep only when its answer says the index is not complete for the paths involved. The index knows a file's language only by its extension, so a script without one has no symbols whatever it is written in, and it names symbols only in the languages the project declares and this build can parse (`intent index status` shows each declared language's grammar); a def, outline or context answer that could reach such files says it is not complete and names the language under `unindexed`.

## Persistent memory

Claude Code persists cross-session memories under its config dir, at `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/projects/<project-dir>/memory/`. Notes about user preferences, design decisions not derivable from code, and project context live there. See Claude Code's memory docs for management.

## Session hooks

`intent claude upgrade --apply` writes `.claude/settings.json`, which wires the `SessionStart`, `UserPromptSubmit` and `Stop` hooks through `intent claude hook <name>`; `intent claude upgrade --help` says how to decline it or keep your own. Architecture: `intent/docs/working-with-llms.md#session-hook-architecture` in Intent's repository.

## Rules of the road

The cross-language principles, Highlander, PFIC, Thin Coordinator and No Silent Errors, are indexed under "Rules of the Road" in `AGENTS.md`, which the `@AGENTS.md` line at the top of this file loads into every Claude Code session. Read any of them with `intent claude rules show <id>` (`intent claude rules list` to enumerate, `--lang <lang>` to filter).

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

`AGENTS.md`'s Critic Family section names the critics; their contract is `intent/docs/critics.md`. More authoring guides live alongside:

- `intent/docs/rules.md` -- rule-library authoring guide (schema, Detection heuristics, attribution).
- `intent/docs/writing-extensions.md` -- user-extension authoring guide (subagents, skills, rule packs at `~/.local/share/intent/ext/`). Extensions are declared and not built in this release: `intent ext` refuses and nothing reads `~/.local/share/intent/ext/`.
- `intent/docs/releasing.md` -- the cut sequence: the steps a release runs that no script runs for it, the ordering the tag depends on, and the two revision inputs the reference generators take.

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

_Generated by Intent v3 from `lib/templates/llm/_CLAUDE.md`._
