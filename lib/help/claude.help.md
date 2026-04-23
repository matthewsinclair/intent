@short: Claude Code integration (subagents, skills, upgrade, prime)

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

Manage Claude Code subagents -- specialized AI assistants with domain-specific expertise. The canon family includes `intent`, `socrates`, `diogenes`, and the `critic-<lang>` rule-library critics (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`). Extension-supplied subagents (e.g. `worker-bee` post-v2.9.0) are listed transparently alongside canon entries.

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

### rules

Manage Intent's rule library -- the single source of truth that Critic subagents enforce against code and tests.

```
intent claude rules list [--lang <lang>] [--severity <level>]
intent claude rules show <id>
intent claude rules validate [<id>|<path>]
intent claude rules index
```

See `intent help rules` for full usage and `intent/docs/rules.md` for the authoring guide.

### upgrade

Diagnose and upgrade project LLM guidance files.

```
intent claude upgrade
intent claude upgrade --apply
intent claude upgrade --project-dir <dir>
```

Dry-run by default. Use `--apply` to execute the upgrade plan.

### prime

Pre-load Claude Code's persistent project memory with operational knowledge, rules, modules, and decision trees.

```
intent claude prime
intent claude prime --refresh
intent claude prime --dry-run
intent claude prime --from <project>
```

Without `--refresh`, prompts before overwriting existing MEMORY.md. Output stays under 200 lines (Claude Code's truncation limit).

## Examples

```bash
# List available subagents
intent claude subagents list

# Install all subagents
intent claude subagents install --all

# List available skills
intent claude skills list

# Install a specific skill
intent claude skills install in-essentials

# Check for LLM guidance upgrades
intent claude upgrade

# Apply upgrades
intent claude upgrade --apply

# Preview memory injection
intent claude prime --dry-run

# Write project memory
intent claude prime --refresh
```

## See Also

- `intent help agents` - AGENTS.md management
- `intent help plugin` - Discover all plugins
- `intent help rules` - Rule library CLI
- `intent help ext` - User extensions
- `intent/docs/critics.md` - Critic subagent contract
- `intent/docs/writing-extensions.md` - Extension authoring
- `intent help` - General help
