@short: Run automated code quality checks on Elixir projects

# intent audit

Run automated code quality checks on Elixir projects using custom Credo checks.

## Synopsis

```
intent audit <command> [options]
```

## Description

The audit command provides automated enforcement of Intent coding rules via custom Credo checks. On first run, check templates are automatically copied into the target project's `credo_checks/` directory.

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

### health

Lightweight periodic health check for end-of-day use.

```
intent audit health
intent audit health --report
intent audit health --diff
```

Options:

- `--report` -- Save markdown report to `intent/audit/YYYYMMDD-health.md`
- `--diff` -- Only check files changed since last health check

Checks performed:

1. Modules not registered in MODULES.md
2. Thick coordinators (controllers/LiveViews over 100 lines)
3. Highlander suspects (duplicate function names across modules)
4. Credo checks (if credo is available)

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
| D11  | Dependency graph   | Design      | No       |

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

# Health check
intent audit health

# Health check with report
intent audit health --report

# Health check on changed files only
intent audit health --diff
```

## See Also

- `intent help learn` -- Capture project learnings
- `intent help` -- General help
- `intent claude upgrade` -- Upgrade project LLM guidance
