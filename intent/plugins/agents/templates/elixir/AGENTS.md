# AGENTS.md

This is the primary tool-agnostic config file for AI coding agents working on this project. Spec: https://agents.md.

## Project Overview

<!-- Replace with your project description -->

This is an Elixir/Phoenix project using Ash Framework, managed with Intent.

## Development Environment

### Prerequisites

- Elixir 1.17+ / Erlang/OTP 27+
- PostgreSQL 16+
- Node.js 20+ (for assets)

### Setup

```bash
mix setup           # Install deps, create DB, run migrations, seed
mix phx.server      # Start Phoenix server
```

### Common Commands

```bash
# Development
mix deps.get              # Install dependencies
mix ash.codegen <name>    # Generate migration from resource changes
mix ash.migrate           # Run database migrations
mix phx.server            # Start dev server
iex -S mix phx.server     # Start with IEx shell

# Testing
mix test                  # Run all tests
mix test --only wip       # Run tests tagged @tag :wip
mix test path/to/test.exs # Run specific test file

# Code Quality
mix format                # Format code
mix credo                 # Static analysis
mix dialyzer              # Type checking (if configured)
```

## Code Style

- Follow Elixir conventions enforced by `mix format`.
- See `intent/llm/RULES.md` for project-specific coding rules (human-curated).
- See `intent/llm/ARCHITECTURE.md` for system structure and domain boundaries (human-curated).
- See `usage-rules.md` for the DO / NEVER contract honoured by `mix usage_rules.sync` and friends.
- See `intent/docs/working-with-llms.md` for the canon tech note on the LLM-facing layout.

### Commit Conventions

- Short, descriptive commit messages.
- Reference steel thread or work package IDs where applicable.
- No AI attribution in commit messages.

## Installed Skills

<!-- Updated by `intent agents sync` -- leave blank for sync to populate. -->

## Installed Subagents

<!-- Updated by `intent agents sync` -- leave blank for sync to populate. -->

## Critic Family

Per-language rule enforcement via thin subagents. For Elixir, invoke:

```
Task(subagent_type="critic-elixir", prompt="review <paths>")
Task(subagent_type="critic-elixir", prompt="test-check <paths>")
```

Headless runner `bin/intent_critic` powers the pre-commit gate. Contract: `intent/docs/critics.md`.

## Rule Library

All coding rules live in `intent/plugins/claude/rules/`. Elixir rules at `intent/plugins/claude/rules/elixir/`.

```bash
intent claude rules list     # enumerate
intent claude rules show <id>
```

## Extensions

User extensions at `~/.intent/ext/<name>/`:

```bash
intent ext list
```

Authoring guide: `intent/docs/writing-extensions.md`.

## Session Hooks

`.claude/settings.json` wires Claude Code lifecycle hooks (SessionStart, UserPromptSubmit, Stop). Architecture: `intent/docs/working-with-llms.md#session-hook-architecture`.

## Socrates vs Diogenes FAQ

Two distinct subagents:

- **Socrates** -- CTO Review Mode (architecture and strategy).
- **Diogenes** -- Elixir Test Architect (test specification and validation).

Forensic detail: `intent/docs/working-with-llms.md#socrates-vs-diogenes-faq`.

## Security Considerations

- Never commit secrets or credentials.
- Review external inputs for injection-class bugs.
- Follow the project's security policy if one exists.

## Additional Resources

- `intent/docs/working-with-llms.md` -- canon tech note.
- `intent/docs/critics.md` -- critic contract.
- `intent/docs/rules.md` -- rule library authoring guide.
- `intent/llm/MODULES.md` -- module registry (Highlander enforcement).
- `CLAUDE.md` -- Claude-specific overlay (if present).
- `usage-rules.md` -- DO / NEVER contract (if present).
