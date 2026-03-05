@short: Module registry guardrails and enforcement

# intent modules

Module registry guardrails and enforcement for the Highlander Rule.

## Synopsis

```
intent modules <command> [options]
```

## Description

The modules command compares the project's `intent/llm/MODULES.md` registry against the actual filesystem to detect unregistered files and stale entries. This enforces the Highlander Rule: every module must be registered before creation.

## Commands

### check

Compare MODULES.md registry against the filesystem.

```
intent modules check
intent modules check --register
```

Options:

- `--register` -- Show guidance for registering unregistered modules

Exit codes:

- `0` -- Registry is clean (no issues)
- `1` -- Issues found (unregistered or stale entries)

### find

Search MODULES.md for a term.

```
intent modules find <term>
```

### help

Display usage information.

```
intent modules help
```

## Examples

```bash
# Check registry vs filesystem
intent modules check

# Check and get registration guidance
intent modules check --register

# Search for a module
intent modules find helpers

# Search for templates
intent modules find template
```

## Hook Template

A Claude Code advisory write hook template is available at:
`lib/templates/hooks/module_check_hook.json`

Copy the hook definition into your project's `.claude/settings.json` to get warnings when creating files not registered in MODULES.md.

## See Also

- `intent help audit` -- Automated code quality checks
- `intent help learn` -- Capture project learnings
- `intent help` -- General help
