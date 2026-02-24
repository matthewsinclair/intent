@short: Manage AGENTS.md for Intent projects

# intent agents

Manage AGENTS.md files for Intent projects.

## Synopsis

```
intent agents <command> [options]
```

## Description

The `intent agents` command manages AGENTS.md files -- universal AI agent instructions that work across Claude Code, Cursor, Windsurf, and other AI-powered editors. AGENTS.md provides a standardized way to give AI tools project-specific context and rules.

## Commands

### init

Initialize AGENTS.md for the current project.

```
intent agents init
intent agents init --template elixir
```

Options:

- `--template <name>` - Use a project template (copies AGENTS.md, RULES.md, ARCHITECTURE.md)
- `--force`, `-f` - Overwrite existing files

### generate

Generate AGENTS.md from project structure and configuration.

```
intent agents generate
```

Analyzes the project and produces an AGENTS.md reflecting current state.

### sync

Update AGENTS.md with latest project state.

```
intent agents sync
```

Re-syncs the AGENTS.md content based on current project configuration.

### validate

Check AGENTS.md for compliance and completeness.

```
intent agents validate
```

Reports missing sections, stale references, and structural issues.

### template

List or show available project templates.

```
intent agents template
intent agents template list
intent agents template show <name>
```

## Examples

```bash
# Initialize AGENTS.md for a new project
intent agents init

# Initialize with Elixir template
intent agents init --template elixir

# Validate AGENTS.md
intent agents validate

# Re-sync after project changes
intent agents sync
```

## See Also

- `intent help claude` - Claude Code integration
- `intent help plugin` - Discover all plugins
- `intent help` - General help
