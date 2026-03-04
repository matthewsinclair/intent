---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-06
title: "Automated Enforcement (D5a, D5b)"
scope: Large
status: Not Started
---

# WP-06: Automated Enforcement (D5a, D5b)

## Objective

Implement automated rule enforcement via custom Credo checks (D5a) and `intent audit quick` (D5b).

## Deliverables

### D5a: Custom Credo Checks

| Check                   | Rule | What it catches                                 | Auto-fixable |
| ----------------------- | ---- | ----------------------------------------------- | ------------ |
| `BooleanOperators`      | R8   | `&&`/`\|\|` where `and`/`or` is correct         | Yes          |
| `MissingImplAnnotation` | R11  | Behaviour callbacks without `@impl true`        | Yes          |
| `DebugArtifacts`        | R15  | `IO.inspect`, `IO.puts`, `dbg()` in lib/        | Yes          |
| `MapGetOnStruct`        | R7   | `Map.get(struct, :field)` instead of dot access | No           |
| `ThickCoordinator`      | R2   | Controllers/LiveViews exceeding line threshold  | No           |
| `HighlanderSuspect`     | R6   | Same function name in multiple modules          | No           |

Ship as template files copied into project's `lib/mix/checks/` by ST0000 bootstrap.
Location: `lib/templates/credo_checks/elixir/`

### D5b: `intent audit quick`

```bash
intent audit quick              # Run all automated checks
intent audit quick --rule R8    # Specific rule
intent audit quick --fix        # Auto-fix R8, R11, R15
intent audit quick --json       # JSON output
```

For Elixir: detect `mix.exs`, run `mix credo` with custom checks, format report.
Initial version: Elixir only.

### Report Format

```
Intent Audit Quick -- <project_name>
Date: YYYY-MM-DD

R8  BooleanOperators:      3 violations (auto-fixable)
R11 MissingImplAnnotation: 7 violations (auto-fixable)
R15 DebugArtifacts:        0 violations
R7  MapGetOnStruct:        2 violations
R2  ThickCoordinator:      1 violation
R6  HighlanderSuspect:     4 suspects (manual review)

Total: 17 violations (10 auto-fixable)
```

## Acceptance Criteria

- [ ] All 6 Credo checks implemented and functional
- [ ] `intent audit quick` produces formatted report
- [ ] `--fix` auto-fixes R8, R11, R15
- [ ] `--rule` filters to specific rule
- [ ] Zero false positives on clean ST0000-bootstrapped project

## Dependencies

- Depends on: WP-03 (rules must be defined)
- Independent of: WP-04, WP-05
