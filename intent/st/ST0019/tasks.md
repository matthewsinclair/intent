# Tasks - ST0019: Treeindex

## Work Packages

- [x] **WP01**: `bin/intent_treeindex` CLI command -- COMPLETE (2026-02-04)
- [ ] **WP02**: Treeindex Claude Code subagent
- [ ] **WP03**: Integration and registration

## Task Breakdown

### WP01: CLI Command -- COMPLETE

- [x] Create `bin/intent_treeindex` with argument parsing and help
- [x] Implement fingerprint computation (`treeindex_fingerprint`)
- [x] Implement staleness checking (`treeindex_is_stale`)
- [x] Implement directory walking (bottom-up, depth-limited)
- [x] Implement file gathering with ignore lists
- [x] Implement Claude invocation (`claude -p` with format prompt)
- [x] Implement `.treeindex` file writing with header (centralized shadow directory)
- [x] Implement `--check` mode (staleness report only)
- [x] Implement `--dry-run` mode
- [x] Implement progress reporting
- [x] Implement `.treeindexignore` (auto-created, gitignore-style exclusions)
- [x] Create `tests/unit/treeindex_commands.bats` (38 tests)
- [x] Run full test suite -- 250/250 passing
- [x] Manual testing on `intent/`, `bin/`, `lib/` directories

#### WP01 As-Built Notes

- **Not a global command**: requires Intent project context (uses `intent/.treeindex/` shadow dir)
- **DIR is mandatory** (not optional): shows usage if omitted, rejects project root
- **Default depth 2** (not 1): covers dir + children + grandchildren
- **Default model haiku** (not sonnet): Claude Haiku 4.5, cost-effective for summarization
- **Budget $0.50/dir** (not $0.02): original cap was too low, caused false failures
- **612 lines** (not estimated 250-350): bash 3.2 compat and shadow path logic added bulk
- **38 bats tests**: covering help, shadow paths, fingerprints, staleness, check/dry-run, depth, ignore patterns, generation with mock Claude, argument validation

### WP02: Subagent

- [ ] Create `intent/plugins/claude/subagents/treeindex/metadata.json`
- [ ] Create `intent/plugins/claude/subagents/treeindex/agent.md`
- [ ] Verify format spec matches CLI output exactly

### WP03: Integration

- [ ] Register in `global-agents.json`
- [ ] Update CLAUDE.md with agent listing and `.treeindex` convention
- [ ] Verify `intent claude subagents list/install/show` work
- [ ] Run `tests/run_tests.sh` -- no regressions
- [x] Generate initial `.treeindex` for Intent project as validation

## Dependencies

WP01 -> WP02 -> WP03 (sequential)
