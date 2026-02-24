@short: Claude Code integration (subagents, skills, upgrade)

# intent claude

Claude Code integration for Intent projects.

## Synopsis

```
intent claude <subcommand> [options]
```

## Description

The `intent claude` namespace provides commands for managing Claude Code subagents, skills, and project upgrades. These commands integrate Intent's methodology with Claude Code's extensibility features.

## Subcommands

### subagents

Manage Claude Code subagents -- specialized AI assistants with domain-specific expertise.

```
intent claude subagents init
intent claude subagents list [-v]
intent claude subagents install <name> [--all] [--force]
intent claude subagents sync [--force]
intent claude subagents uninstall <name> [--all]
intent claude subagents show <name>
intent claude subagents status [--verbose]
```

### skills

Manage Claude Code skills -- always-on enforcement rules for code quality.

```
intent claude skills list [-v]
intent claude skills install <name> [--all] [--force]
intent claude skills sync [--force]
intent claude skills uninstall <name> [--all]
intent claude skills show <name>
```

### upgrade

Diagnose and upgrade project LLM guidance files.

```
intent claude upgrade
intent claude upgrade --apply
intent claude upgrade --project-dir <dir>
```

Dry-run by default. Use `--apply` to execute the upgrade plan.

## Examples

```bash
# List available subagents
intent claude subagents list

# Install all subagents
intent claude subagents install --all

# List available skills
intent claude skills list

# Install a specific skill
intent claude skills install intent-essentials

# Check for LLM guidance upgrades
intent claude upgrade

# Apply upgrades
intent claude upgrade --apply
```

## See Also

- `intent help agents` - AGENTS.md management
- `intent help plugin` - Discover all plugins
- `intent help` - General help
