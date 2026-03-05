@short: Run automated code quality checks on Elixir projects

# intent audit

Run automated code quality checks on Elixir projects using custom Credo checks.

## Synopsis

```
intent audit <command> [options]
```

## Description

The audit command provides automated enforcement of Intent coding rules via custom Credo checks. On first run, check templates are automatically copied into the target project's `lib/mix/checks/` directory.

## Commands

### quick

Run Credo with Intent's custom check templates.

```
intent audit quick
intent audit quick --rule R8
intent audit quick --fix
intent audit quick --json
intent audit quick --checks-only
```

Options:

- `--rule RN` -- Run only the specified rule
- `--fix` -- Apply auto-fixes for fixable rules (R8, R11, R15)
- `--json` -- Output results as JSON
- `--checks-only` -- Only install check templates, don't run credo

### help

Display usage information.

```
intent audit help
```

## Rules

| Rule | Check              | Category    | Auto-fix |
| ---- | ------------------ | ----------- | -------- |
| R2   | Thick coordinator  | Design      | No       |
| R6   | Highlander suspect | Design      | No       |
| R7   | Map.get on struct  | Refactor    | No       |
| R8   | Boolean operators  | Readability | Yes      |
| R11  | Missing @impl      | Readability | Yes      |
| R15  | Debug artifacts    | Warning     | Yes      |

## Prerequisites

- An Elixir/Mix project (mix.exs must exist)
- Credo dependency in mix.exs: `{:credo, "~> 1.7", only: [:dev, :test], runtime: false}`

## Examples

```bash
# Run all checks
intent audit quick

# Run only boolean operator check
intent audit quick --rule R8

# Get JSON output
intent audit quick --json

# Just install check templates
intent audit quick --checks-only
```

## See Also

- `intent help` -- General help
- `intent claude upgrade` -- Upgrade project LLM guidance
