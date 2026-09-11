# AGENTS.md

This is the primary tool-agnostic config file for AI coding agents working on this project. Agentic CLIs that follow the AGENTS.md convention (Codex, Cursor, Copilot, Aider, Continue, Cline, Gemini CLI) read it as the canonical project contract. Claude Code does not: it reads `CLAUDE.md`, which points here. Spec: https://agents.md.

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
- Markdown: no manual line wrapping; verblock frontmatter on persistent docs.
- See `usage-rules.md` for the terse "DO / NEVER" contract.
- See `intent/docs/working-with-llms.md` in the Intent source repository (https://github.com/matthewsinclair/intent) for the canon tech note on the LLM-facing layout and how hooks + critics + skills compose.

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

## Installed Skills and Subagents

Read live rather than reproduced here, because a snapshot of what is installed goes stale the moment anything is installed and nothing regenerates this file to notice:

```bash
intent claude skills list
intent claude subagents list
```

## Critic Family

Per-language rule enforcement via thin subagents that read the rule library at invocation time: `critic-<lang>` for code, `critic-prose` for prose. `intent claude subagents list` names them. Invoke:

```
Task(subagent_type="critic-<lang>", prompt="review <targets>")
```

The installed Intent tool's headless runner (`intent critic <lang>`: rules armed with a greppable proxy or a named tool such as `shellcheck`; no LLM required) powers the pre-commit gate. Contract: `intent/docs/critics.md` in the Intent source repository (https://github.com/matthewsinclair/intent). Exit codes: `0` clean, `1` findings, `2` the runner could not answer (the gate fails open), `3` an armed rule's tool is absent on this machine (the gate blocks).

## Rules of the Road

Cross-language principles govern all Intent projects. Every language pack concretises them; the critics enforce them.

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure Function, Impure Coordination. Read it with `intent claude rules show IN-AG-PFIC-001`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

Read any of them with `intent claude rules show <id>`. The terse DO / NEVER contract for this project lives in `usage-rules.md`.

## Rule Library

The coding-rule library is served by the installed Intent tool, not vendored into this project. Access rules through the CLI below -- each rule carries YAML frontmatter, a Detection heuristic, and bad/good examples. Skills cite rule IDs; critics enforce them.

```bash
intent claude rules list        # enumerate
intent claude rules show <id>   # inspect
intent claude rules validate    # schema check
```

## Session Hooks

`.claude/settings.json` wires Claude Code lifecycle hooks: **SessionStart** (inject context + `/in-session` reminder), **UserPromptSubmit** (strict gate -- block first prompt until `/in-session` runs), **Stop** (remind `/in-finish` at wrap-up). Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Socrates vs Diogenes FAQ

Distinct subagents for distinct concerns:

- **Socrates** -- CTO Review Mode. Architectural and strategic technical decision-making via Socratic dialog.
- **Diogenes** -- Elixir Test Architect. Test-specification generation and gap analysis.

They have never been the same agent. FAQ + forensic detail: `intent/docs/working-with-llms.md#socrates-vs-diogenes-faq` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Testing Instructions

1. Run the test suite before making changes -- establish a baseline.
2. Make changes.
3. Run the suite again; confirm no regressions.
4. Add new tests for new functionality.
5. Before commit, the pre-commit critic gate checks staged files for `CRITICAL` + `WARNING` findings. See `intent/docs/pre-commit-hook.md` in the Intent source repository (https://github.com/matthewsinclair/intent).

## Commit Guidelines

- Descriptive commit messages; reference steel thread or work package IDs where applicable.
- No AI attribution in commit messages (no `Co-Authored-By`, no tool signatures).
- Small, coherent commits preferred over large batches.

## Security Considerations

- Never commit secrets or credentials.
- Review external inputs for injection-class bugs.
- Follow the project's security policy if one exists.

## Additional Resources

In the Intent source repository (https://github.com/matthewsinclair/intent), not this project:

- `intent/docs/working-with-llms.md` -- canon tech note (narrative).
- `intent/docs/critics.md` -- critic contract.
- `intent/docs/rules.md` -- rule library authoring guide.
- `intent/docs/writing-extensions.md` -- extension author guide.
- `intent/docs/pre-commit-hook.md` -- pre-commit critic gate install + configure.

In this project:

- `intent/llm/MODULES.md` -- OPTIONAL module registry (Highlander enforcement). `intent init` does not create one; where it exists, search it with `intent modules find <name>`.
- `CLAUDE.md` -- Claude-specific overlay.
- `usage-rules.md` -- DO / NEVER contract for this project.

---

_Generated by Intent v3.0.1 from `lib/templates/llm/_AGENTS.md`._
