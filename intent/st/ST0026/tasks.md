# Tasks - ST0026: Steel Thread Zero

Phase 1 (WP-01 through WP-05 + WP-11) complete. See `done.md` for completed tasks.

## Phase 2: Remaining Work Packages

## WP-06: Automated Enforcement (D5a, D5b) -- Done

- [x] Create `lib/templates/credo_checks/elixir/boolean_operators.ex`
- [x] Create `lib/templates/credo_checks/elixir/missing_impl_annotation.ex`
- [x] Create `lib/templates/credo_checks/elixir/debug_artifacts.ex`
- [x] Create `lib/templates/credo_checks/elixir/map_get_on_struct.ex`
- [x] Create `lib/templates/credo_checks/elixir/thick_coordinator.ex`
- [x] Create `lib/templates/credo_checks/elixir/highlander_suspect.ex`
- [x] Create `bin/intent_audit` command script
- [x] Implement `intent audit quick` subcommand
- [x] Implement `--rule` filter
- [x] Implement `--fix` flag
- [x] Implement `--json` output mode
- [x] Create `lib/help/audit.help.md`
- [x] Add v2.5.0->v2.6.0 upgrade path
- [x] Wire into CLI dispatch, help, plugin.json
- [x] 17 BATS tests (382 total passing)

## WP-07: Health Check & Learnings (D7, D10) -- Done

- [x] Implement `intent audit health` subcommand in `bin/intent_audit`
- [x] Implement `--report` flag (save markdown to `intent/audit/YYYYMMDD-health.md`)
- [x] Implement `--diff` flag (git-based changed files only)
- [x] Implement timestamp tracking in `.intent/last-health-check`
- [x] Create `bin/intent_learn` command script
- [x] Implement `intent learn "description"` (append to .intent/learnings.md)
- [x] Implement `--category` flag (footgun/worked/failed)
- [x] Implement `--list` flag
- [x] Learnings integration with `intent claude prime` (already wired -- reads `.intent/learnings.md`)
- [x] 25 BATS tests (407 total passing)

## WP-08: Guardrails (D9, D11) -- Done

- [x] Create `bin/intent_modules` command script (~230 lines)
- [x] Implement `intent modules check` (compare MODULES.md vs filesystem)
- [x] Implement `intent modules check --register` (interactive registration)
- [x] Implement `intent modules find` (search registry)
- [x] Create Claude Code hook template (`lib/templates/hooks/module_check_hook.json`)
- [x] Create `lib/templates/llm/_DEPENDENCY_GRAPH.md` template
- [x] Create `lib/templates/credo_checks/elixir/dependency_graph.ex` (D11 Credo check)
- [x] Integrate D11 with `intent audit quick --rule D11`
- [x] Create `lib/help/modules.help.md`
- [x] Wire into CLI dispatch, help, plugin.json
- [x] 19 BATS tests in `tests/unit/modules_commands.bats`
- [x] Rationalize CLI output across all commands (Rust-style conventions)
- [x] Add `--help`/`-h` flag support to `intent st`
- [x] Register missing bin scripts + remove stale entries in MODULES.md
- [x] 427 total tests passing across 21 BATS test files

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

## Documentation (post-Phase 2)

- [ ] Comprehensive README update
- [ ] CHANGELOG update for v2.6.0
- [ ] New blog post for agent/claude work (not shoe-horned into existing posts)
- [ ] Update existing blog posts where needed

## Dependencies

```
WP-06 -> WP-07
WP-03 -> WP-08
WP-06 -> WP-08
WP-03 + WP-04 + WP-06 -> WP-09 -> WP-10
```
