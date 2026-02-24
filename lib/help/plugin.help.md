@short: Discover plugins and their commands

# intent plugin

Discover Intent plugins and their commands.

## Synopsis

```
intent plugin [command]
```

## Description

Intent uses a plugin architecture to provide extensible functionality. Each plugin lives in `intent/plugins/<name>/` and provides one or more commands. The `intent plugin` command lets you discover what plugins are available and what commands they provide.

## Commands

### list

List all plugins and their commands. This is the default when no subcommand is given.

```
intent plugin
intent plugin list
```

### show

Show detailed information for a specific plugin.

```
intent plugin show <name>
intent plugin show claude
intent plugin show agents
```

### help

Display usage information.

```
intent plugin help
```

## Examples

```bash
# List all plugins
intent plugin

# Show details for the claude plugin
intent plugin show claude

# Show details for the agents plugin
intent plugin show agents
```

## See Also

- `intent help claude` - Claude Code integration
- `intent help agents` - AGENTS.md management
- `intent help` - General help
