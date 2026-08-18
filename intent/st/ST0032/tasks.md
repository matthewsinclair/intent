# Tasks - ST0032: Fix Intent's Elixir Credo Checks

## Work Packages

### WP-01: Template Cleanup

- [ ] Delete `boolean_operators.ex` from `lib/templates/credo_checks/elixir/`
- [ ] Delete `dependency_graph.ex` from `lib/templates/credo_checks/elixir/`
- [ ] Create `bracket_access_on_struct.ex` (EX4008, R16) with struct-variable tracking
- [ ] Fix `map_get_on_struct.ex`: struct-variable tracking, stop flagging all Map.get
- [ ] Fix `missing_impl_annotation.ex`: flat issues accumulator instead of tuple
- [ ] Fix `debug_artifacts.ex`: add `excluded_paths` param
- [ ] Fix `thick_coordinator.ex`: strip `quote do...end` blocks before coordinator detection

### WP-02: Create configure_credo.exs

- [ ] Create `lib/scripts/configure_credo.exs` with `.credo.exs` patching logic
- [ ] Register in `intent/llm/MODULES.md`
- [ ] Handle edge cases: missing `.credo.exs`, existing `requires:` list, stale entries
- [ ] Test idempotency (running twice produces same result)

### WP-03: Update Shell Scripts

- [ ] `bin/intent_st_zero`: replace D5a hint with configure script call
- [ ] `bin/intent_st_zero`: update `check_d5a_credo()` thresholds (7->6) and add .credo.exs check
- [ ] `bin/intent_audit`: update VALID_RULES (drop R8/D11, add R16)
- [ ] `bin/intent_audit`: update FIXABLE_RULES (drop R8)
- [ ] `bin/intent_audit`: update `get_rule_template()` and `is_valid_rule()`
- [ ] `bin/intent_audit`: replace hint in `ensure_checks_installed()` with configure script call
- [ ] `bin/intent_audit`: add deprecated template removal to `ensure_checks_installed()`
- [ ] `bin/intent_audit`: remove `--checks-dir` from `run_credo()` (line 165)
- [ ] `bin/intent_audit`: remove `--checks-dir` from health check (line 619)
- [ ] `bin/intent_audit`: update `usage()` rule documentation
- [ ] Update `lib/help/audit.help.md`
- [ ] Update `lib/help/stzero.help.md`
- [ ] Update `intent/llm/MODULES.md` template count

### WP-04: Verification

- [ ] Fresh install on a target Elixir project: `intent st zero apply D5a`
- [ ] Direct `mix credo --strict` picks up custom checks (no --checks-dir needed)
- [ ] `intent audit quick` works without --checks-dir
- [ ] Idempotency: re-run D5a produces "already configured"
- [ ] Upgrade: old install with boolean_operators.ex gets cleaned up
- [ ] Run Intent's own BATS test suite: `tests/run_tests.sh`

## Dependencies

```
WP-01 (templates) --> WP-02 (script, needs final check list)
                       --> WP-03 (shell scripts, needs script path)
                            --> WP-04 (verification, needs everything)
```
