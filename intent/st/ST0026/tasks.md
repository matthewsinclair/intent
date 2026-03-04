# Tasks - ST0026: Steel Thread Zero

Phase 1 (WP-01 through WP-05 + WP-11) complete. See `done.md` for completed tasks.

## Phase 2: Remaining Work Packages

## WP-06: Automated Enforcement (D5a, D5b)

- [ ] Create `lib/templates/credo_checks/elixir/boolean_operators.ex`
- [ ] Create `lib/templates/credo_checks/elixir/missing_impl_annotation.ex`
- [ ] Create `lib/templates/credo_checks/elixir/debug_artifacts.ex`
- [ ] Create `lib/templates/credo_checks/elixir/map_get_on_struct.ex`
- [ ] Create `lib/templates/credo_checks/elixir/thick_coordinator.ex`
- [ ] Create `lib/templates/credo_checks/elixir/highlander_suspect.ex`
- [ ] Create `bin/intent_audit` command script
- [ ] Implement `intent audit quick` subcommand
- [ ] Implement `--rule` filter
- [ ] Implement `--fix` auto-fix for R8, R11, R15
- [ ] Implement `--json` output mode
- [ ] Create `lib/help/audit.help.md`
- [ ] Test against known-clean and known-dirty codebases

## WP-07: Health Check & Learnings (D7, D10)

- [ ] Implement `intent audit health` subcommand in `bin/intent_audit`
- [ ] Implement `--report` flag (save markdown)
- [ ] Implement `--diff` flag (git-based changed files only)
- [ ] Implement timestamp tracking in `.intent/last-health-check`
- [ ] Create `bin/intent_learn` command script
- [ ] Implement `intent learn "description"` (append to .intent/learnings.md)
- [ ] Implement `--category` flag (footgun/worked/failed)
- [ ] Implement `--list` flag
- [ ] Test learnings integration with `intent claude prime`

## WP-08: Guardrails (D9, D11)

- [ ] Create `bin/intent_modules` command script
- [ ] Implement `intent modules check`
- [ ] Implement `intent modules check --register`
- [ ] Implement `intent modules find`
- [ ] Create Claude Code hook for `Write` to `lib/**/*.ex`
- [ ] Create `lib/templates/llm/_DEPENDENCY_GRAPH.md` template
- [ ] Implement dependency graph check (scan alias/import/use vs declared deps)
- [ ] Integrate with `intent audit quick`

## WP-09: Retrofit Installation (D12)

- [ ] Create `bin/intent_st_zero` (or extend `bin/intent_st`)
- [ ] Implement `intent st zero install` with 4 phases
- [ ] Implement module auto-discovery algorithm
- [ ] Implement gap analysis report
- [ ] Implement proposal generation
- [ ] Implement interactive apply with confirmation
- [ ] Implement `--audit-only` flag
- [ ] Implement `--deliverable` filter
- [ ] Implement `--dry-run` flag
- [ ] Test on Intent project itself (eating our own dogfood)

## WP-10: Integrator Command (D1)

- [ ] Modify `bin/intent_init` to accept `--with-st0000` flag
- [ ] Implement `bootstrap_st0000()` function
- [ ] Create `bin/intent_st_zero` for `intent st zero` subcommand
- [ ] Implement `intent st zero --check` compliance verification
- [ ] Test greenfield: `intent init --with-st0000` creates fully equipped project
- [ ] Test existing project: `intent st zero` delegates to retrofit
- [ ] Run `intent audit quick` on bootstrapped project (expect zero violations)

## Dependencies

```
WP-06 -> WP-07
WP-03 -> WP-08
WP-06 -> WP-08
WP-03 + WP-04 + WP-06 -> WP-09 -> WP-10
```
