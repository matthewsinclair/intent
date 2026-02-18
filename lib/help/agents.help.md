@short: Manage Claude Code sub-agents for Intent projects

# intent agents

Manage Claude Code sub-agents for Intent projects.

## Synopsis

```
intent agents <command> [options]
```

## Description

The Intent agent system integrates with Claude Code's sub-agent feature to provide specialized AI assistants that understand Intent's methodology and can help with specific development tasks.

## Commands

### list

List available and installed agents.

```
intent agents list
```

Shows all available agents (global and project-specific) with their installation status.

### install

Install agent(s) to Claude configuration.

```
intent agents install <agent-name> [agent-name...]
intent agents install --all
intent agents install intent --force
```

Options:

- `--all` - Install all available agents
- `--force`, `-f` - Skip confirmation prompts

### sync

Sync installed agents with latest versions.

```
intent agents sync [--force]
```

Updates installed agents while respecting local modifications. Use `--force` to overwrite local changes.

### uninstall

Remove Intent-managed agents.

```
intent agents uninstall <agent-name> [agent-name...]
intent agents uninstall --all
```

Options:

- `--all` - Uninstall all Intent-managed agents
- `--force`, `-f` - Skip confirmation prompts

### show

Display detailed agent information.

```
intent agents show <agent-name>
```

Shows metadata, installation status, and system prompt preview for a specific agent.

### status

Check agent health and integrity.

```
intent agents status [--verbose]
```

Verifies installed agents against manifests, checks for modifications, and reports any issues.

Options:

- `--verbose`, `-v` - Show detailed information for each agent

## Available Agents

### intent

The Intent-aware development assistant that understands:

- Steel thread methodology
- Intent commands and structure
- Backlog task management
- Project organization

### elixir

Elixir code doctor featuring:

- 19 Elixir best practices
- Usage Rules methodology
- Ash and Phoenix patterns
- Functional programming guidance

## Examples

```bash
# List all available agents
intent agents list

# Install the Intent agent
intent agents install intent

# Install all available agents
intent agents install --all

# Check agent status
intent agents status

# Update agents with available changes
intent agents sync

# Show details about an agent
intent agents show elixir

# Uninstall a specific agent
intent agents uninstall elixir
```

## Agent Locations

- **Global agents**: `$INTENT_HOME/agents/`
- **Project agents**: `./intent/agents/`
- **Installed to**: `~/.claude/agents/`
- **Manifests**: `.manifest/` subdirectories

## Creating Custom Agents

To create a custom agent for your project:

1. Create directory: `intent/agents/my-agent/`
2. Add `agent.md` with YAML frontmatter and system prompt
3. Add `metadata.json` with version and description
4. Install with: `intent agents install my-agent`

## Troubleshooting

- **"Claude Code not detected"**: Install Claude Code from https://claude.ai/download
- **"Agent file not found"**: Run `intent agents install` to restore missing agents
- **"Local changes detected"**: Your modifications are preserved; use `sync --force` to overwrite

## See Also

- `intent help` - General help
- `intent doctor` - Check system configuration
- Claude Code sub-agents documentation: https://docs.anthropic.com/en/docs/claude-code/sub-agents
