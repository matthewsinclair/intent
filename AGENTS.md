# AGENTS.md

This is the primary tool-agnostic config file for AI coding agents working on this project. Every major agentic CLI (Claude Code, Codex, Cursor, Copilot, Aider, Continue, Cline, Gemini CLI) reads AGENTS.md as the canonical project contract. Spec: https://agents.md.

## Project Overview

Intent -- an Intent project. See `CLAUDE.md` for the Claude-specific overlay.

## Development Environment

### Prerequisites


- Elixir / Erlang / OTP (see `mix.exs` for version)
- Rust toolchain (see `Cargo.toml` for edition)
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


## Build and Test Commands

### Testing


```bash
mix test
```


```bash
cargo test
```


```bash
bats tests/
```


### Building


```bash
mix compile
```


```bash
cargo build
```


### Validation

```bash
intent doctor       # check Intent configuration
intent st list      # list steel threads
```

## Code Style


- Follow Elixir conventions enforced by `mix format`.
- Follow Rust conventions enforced by `cargo fmt`.
- Shell scripts: 2-space indentation, POSIX-compliant where practical.
- Markdown: no manual line wrapping; verblock frontmatter on persistent docs.
- See `usage-rules.md` for the terse "DO / NEVER" contract.
- See `intent/docs/working-with-llms.md` at the Intent install for the canon tech note on the LLM-facing layout and how hooks + critics + skills compose.

## Steel Thread + Work Package Process

Work is organised into numbered **steel threads** (`ST####`), each containing **work packages** (`WP`). Every steel thread has its own directory under `intent/st/<ID>/` with `info.md` as the entry point.

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

Per-language rule enforcement via thin subagents that read the rule library at invocation time. Available: `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`. Invoke:

```
Task(subagent_type="critic-<lang>", prompt="review <targets>")
```

The installed Intent tool's headless runner (`intent critic <lang>`, Greppable-proxy rules only; no LLM required) powers the pre-commit gate. Contract: `intent/docs/critics.md` at the Intent install. Exit codes: `0` clean, `1` findings, `2` error.

## Rule Library

The coding-rule library is served by the installed Intent tool, not vendored into this project. Access rules through the CLI below -- each rule carries YAML frontmatter, a Detection heuristic, and bad/good examples. Skills cite rule IDs; critics enforce them.

```bash
intent claude rules list        # enumerate
intent claude rules show <id>   # inspect
intent claude rules validate    # schema check
```

## Extensions

User extensions live at `~/.intent/ext/<name>/` and contribute subagents, skills, or rule packs without modifying canon.

```bash
intent ext list
intent ext show <name>
intent ext new <name>
```

Authoring guide: `intent/docs/writing-extensions.md` at the Intent install.

## Session Hooks

`.claude/settings.json` wires three Claude Code lifecycle hooks: **SessionStart** (inject context + `/in-session` reminder), **UserPromptSubmit** (strict gate -- block first prompt until `/in-session` runs), **Stop** (remind `/in-finish` at wrap-up). Full architecture: `intent/docs/working-with-llms.md#session-hook-architecture` at the Intent install.

## Socrates vs Diogenes FAQ

Two distinct subagents for two distinct concerns:

- **Socrates** -- CTO Review Mode. Architectural and strategic technical decision-making via Socratic dialog.
- **Diogenes** -- Elixir Test Architect. Test-specification generation and gap analysis.

They have never been the same agent. FAQ + forensic detail: `intent/docs/working-with-llms.md#socrates-vs-diogenes-faq` at the Intent install.

## Testing Instructions

1. Run the test suite before making changes -- establish a baseline.
2. Make changes.
3. Run the suite again; confirm no regressions.
4. Add new tests for new functionality.
5. Before commit, the pre-commit critic gate checks staged files for `CRITICAL` + `WARNING` findings. See `intent/docs/pre-commit-hook.md` at the Intent install.

## Commit Guidelines

- Descriptive commit messages; reference steel thread or work package IDs where applicable.
- No AI attribution in commit messages (no `Co-Authored-By`, no tool signatures).
- Small, coherent commits preferred over large batches.

## Security Considerations

- Never commit secrets or credentials.
- Review external inputs for injection-class bugs.
- Follow the project's security policy if one exists.

## Additional Resources

At the Intent install (not this project):

- `intent/docs/working-with-llms.md` -- canon tech note (narrative).
- `intent/docs/critics.md` -- critic contract.
- `intent/docs/rules.md` -- rule library authoring guide.
- `intent/docs/writing-extensions.md` -- extension author guide.
- `intent/docs/pre-commit-hook.md` -- pre-commit critic gate install + configure.

In this project:

- `intent/llm/MODULES.md` -- module registry (Highlander enforcement).
- `CLAUDE.md` -- Claude-specific overlay.
- `usage-rules.md` -- DO / NEVER contract for this project.

---

_Generated by Intent v3.0.0-dev from `lib/templates/llm/_AGENTS.md`._
