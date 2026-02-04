# Tasks - ST0019: Treeindex

## Work Packages

- [ ] **WP01**: `bin/intent_treeindex` CLI command
- [ ] **WP02**: Treeindex Claude Code subagent
- [ ] **WP03**: Integration and registration

## Task Breakdown

### WP01: CLI Command

- [ ] Create `bin/intent_treeindex` with argument parsing and help
- [ ] Implement fingerprint computation (`treeindex_fingerprint`)
- [ ] Implement staleness checking (`treeindex_is_stale`)
- [ ] Implement directory walking (bottom-up, depth-limited)
- [ ] Implement file gathering with ignore lists
- [ ] Implement Claude invocation (`claude -p` with format prompt)
- [ ] Implement `.treeindex` file writing with header
- [ ] Implement `--check` mode (staleness report only)
- [ ] Implement `--dry-run` mode
- [ ] Implement progress reporting
- [ ] Add `treeindex` to GLOBAL_COMMANDS in `bin/intent`
- [ ] Manual testing on `intent/plugins/claude/subagents/`

### WP02: Subagent

- [ ] Create `intent/plugins/claude/subagents/treeindex/metadata.json`
- [ ] Create `intent/plugins/claude/subagents/treeindex/agent.md`
- [ ] Verify format spec matches CLI output exactly

### WP03: Integration

- [ ] Register in `global-agents.json`
- [ ] Update CLAUDE.md with agent listing and `.treeindex` convention
- [ ] Verify `intent claude subagents list/install/show` work
- [ ] Run `tests/run_tests.sh` -- no regressions
- [ ] Generate initial `.treeindex` for Intent project as validation

## Dependencies

WP01 -> WP02 -> WP03 (sequential)
