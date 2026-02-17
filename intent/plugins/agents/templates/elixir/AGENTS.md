# AGENTS.md

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

### Code Style

- Follow Elixir conventions enforced by `mix format`
- See `RULES.md` for project-specific coding rules
- See `ARCHITECTURE.md` for system structure and domain boundaries
- Installed skills enforce rules automatically during code generation

### Commit Conventions

- Short, descriptive commit messages
- Prefix with type: `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`

## Installed Subagents

<!-- Updated by intent agents sync -->

- **elixir** — Elixir code doctor (code review, antipattern detection)

## Installed Skills

<!-- Updated by intent agents sync -->

- **elixir-essentials** — Core Elixir coding patterns
- **ash-ecto-essentials** — Ash/Ecto database access rules
- **phoenix-liveview** — LiveView lifecycle and rendering rules
