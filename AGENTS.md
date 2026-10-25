# AGENTS.md

This is the primary tool-agnostic config file for AI coding agents working on this project. Agentic CLIs that follow the AGENTS.md convention (Codex, Cursor, Copilot, Aider, Continue, Cline, Gemini CLI) read it as the canonical project contract. Claude Code reads `CLAUDE.md`, which imports this file with an `@AGENTS.md` line. Spec: https://agents.md.

## Project Overview

Intent -- an Intent project. See `CLAUDE.md` for the Claude-specific overlay.

**Run `intent llm guide` before you start.** It prints the agent guide for the exact build you are standing on: every command this build ships, what each one is for, whether it reads or mutates, and whether it can be undone. It is generated from the tool rather than maintained by hand, so it cannot describe a command that is not there or omit one that is -- which is the failure a written command reference has whenever the tool moves and the document does not.

## Development Environment

### Prerequisites

- Elixir / Erlang / OTP (see `mix.exs` for version)
- Rust toolchain (see `Cargo.toml` for edition)
- Swift toolchain (see `Package.swift` for the tools version)
- Bash or Zsh (see the project's own docs for the target version)
- Bats testing framework

### Setup

```bash
intent doctor     # verify Intent configuration
```

```bash
mix deps.get
mix compile
```

```bash
cargo build
```

```bash
swift build
```

## Build and Test Commands

### Test commands

```bash
mix test
```

```bash
cargo test
```

```bash
swift test
```

```bash
bats -r tests/
```

### Building

```bash
mix compile
```

```bash
cargo build
```

```bash
swift build
```

### Validation

```bash
intent doctor       # check Intent configuration
intent st list      # list steel threads
```

## Code Style

- Follow Elixir conventions enforced by `mix format`.
- Follow Rust conventions enforced by `cargo fmt`.
- Follow Swift conventions enforced by `swift format`.
- Shell scripts: 2-space indentation, POSIX-compliant where practical.
- Markdown: no manual line wrapping; verblock frontmatter on hand-authored persistent docs such as `intent/wip.md` (generated views such as a thread's `info.md` carry none).
- Where the project keeps one, `usage-rules.md` carries the terse "DO / NEVER" contract.

## Steel Thread + Work Package Process

Work is organised into numbered **steel threads** (`ST####`), each containing **work packages** (`WP`). Threads live in the project store. A thread's directory, `intent/st/<ID>/` with `info.md` as the entry point, is on disk while `intent/.intentfiles` declares it (`intent st hydrate` / `intent st dehydrate`).

```bash
intent st list               # list steel threads
intent st new "Title"        # create new
intent st show <id>          # inspect
intent wp list <STID>        # list work packages
intent wp new <STID> "..."   # create work package
intent wp start <STID/NN>    # mark WIP
intent wp done <STID/NN>     # mark done
```

Never create steel thread or work package directories by hand -- always use the CLI.

## Critic Family

Per-language rule enforcement via thin subagents that read the rule library at invocation time: `critic-<lang>` for code, `critic-prose` for prose. `intent claude subagents list` names them. Invoke:

```
Task(subagent_type="critic-<lang>", prompt="review <targets>")
Task(subagent_type="critic-<lang>", prompt="test-check <targets>")
```

The installed Intent tool's headless runner (`intent critic <lang>`: rules armed with a greppable proxy or a named tool such as `shellcheck`; no LLM required) powers the pre-commit gate. Its exit codes, and every other command's, are in `intent llm guide`.

## Rules of the Road

Cross-language principles govern all Intent projects. Every language pack concretises them; the critics enforce them.

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure Function, Impure Coordination. Read it with `intent claude rules show IN-AG-PFIC-001`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

Read any of them with `intent claude rules show <id>`. The terse DO / NEVER contract for this project lives in `usage-rules.md`, where the project keeps one.

## Rule Library

The coding-rule library is served by the installed Intent tool, not vendored into this project. Access rules through the CLI below -- each rule carries YAML frontmatter, a Detection heuristic, and bad/good examples. Skills cite rule IDs; critics enforce them.

```bash
intent claude rules list        # enumerate
intent claude rules show <id>   # inspect
intent claude rules validate    # schema check
```

What is installed is read live, never listed here: `intent claude skills list` and `intent claude subagents list`.

## Session Hooks

`.claude/settings.json`, once `intent claude upgrade --apply` has written it, wires Claude Code lifecycle hooks: **SessionStart** (inject context + `/in-session` reminder), **UserPromptSubmit** (strict gate -- block first prompt until `/in-session` runs), **Stop** (remind `/in-finish` at wrap-up).

## Testing Instructions

1. Run the test suite before making changes -- establish a baseline.
2. Make changes.
3. Run the suite again; confirm no regressions.
4. Add new tests for new functionality.
5. Before commit, the pre-commit gate (installed by `intent claude upgrade --apply`) runs the shipped guards, `intent critic <lang>` on the staged files and `intent doctor`, and refuses the commit on a finding, a refusing guard or a doctor exit 1.

## Commit Guidelines

- Descriptive commit messages; reference steel thread or work package IDs where applicable.
- No AI attribution in commit messages (no `Co-Authored-By`, no tool signatures).
- Small, coherent commits preferred over large batches.

## Additional Resources

In the Intent source repository (https://github.com/matthewsinclair/intent), not this project:

- `intent/docs/working-with-llms.md` -- canon tech note (narrative).
- `intent/docs/critics.md` -- critic contract.
- `intent/docs/rules.md` -- rule library authoring guide.
- `intent/docs/writing-extensions.md` -- extension author guide.
- `intent/docs/pre-commit-hook.md` -- pre-commit critic gate install + configure.

In this project:

- `intent/llm/MODULES.md` -- OPTIONAL module registry (Highlander enforcement). To check for prior art, ask the index first: `intent search --kind def <name>` answers whether a thing with that name already exists anywhere in the tree, and the answer carries the index's own freshness. When it says the index is not complete for the paths that matter, fall back to grep. Where the project keeps a registry, `intent modules find <name>` searches that as well. `intent init` does not create a registry, so for most projects the index is the whole answer.
- `intent/llm/DECISION_TREE.md` -- OPTIONAL code-placement flow chart. `intent init` does not create one; where a project has one, it was chosen for that project and describes that project's own stack.
- `intent/llm/RULES.md`, `intent/llm/ARCHITECTURE.md` -- this project's own rules and architecture, seeded empty by `intent init` for the project to author.
- `intent/` -- steel threads (`st/`), project docs (`docs/`), work tracking (`wip.md`, `restart.md`); `intent/.config/` -- configuration.
- `CLAUDE.md` -- Claude-specific overlay.
- `usage-rules.md` -- DO / NEVER contract for this project, where it keeps one (seeded by `intent claude upgrade --apply` in the root layout when absent, not seeded in the `.claude/` layout, and never overwritten; `intent init` does not create it).

---

_Generated by Intent v3 from `lib/templates/llm/_AGENTS.md`._
