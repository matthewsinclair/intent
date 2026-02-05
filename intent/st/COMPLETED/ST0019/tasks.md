# Tasks - ST0019: Treeindex

## Work Packages

- [x] **WP01**: `bin/intent_treeindex` CLI command -- COMPLETE (2026-02-04)
- [~] **WP02**: Treeindex Claude Code subagent -- SKIPPED (folded into WP03; CLI + CLAUDE.md convention sufficient)
- [x] **WP03**: Integration -- COMPLETE (2026-02-04)

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
- [x] Create `tests/unit/treeindex_commands.bats` (53 tests)
- [x] Run full test suite -- 265/265 passing
- [x] Manual testing on `intent/`, `bin/`, `lib/` directories

#### WP01 As-Built Notes

- **Not a global command**: requires Intent project context (uses `intent/.treeindex/` shadow dir)
- **DIR is mandatory** (not optional): shows usage if omitted, rejects project root
- **Default depth 2** (not 1): covers dir + children + grandchildren
- **Default model haiku** (not sonnet): Claude Haiku 4.5, cost-effective for summarization
- **Budget $0.50/dir** (not $0.02): original cap was too low, caused false failures
- **612 lines** (not estimated 250-350): bash 3.2 compat and shadow path logic added bulk
- **38 bats tests**: covering help, shadow paths, fingerprints, staleness, check/dry-run, depth, ignore patterns, generation with mock Claude, argument validation

### WP02: Subagent -- SKIPPED

Skipped: The CLI command (`intent treeindex <dir>`) and CLAUDE.md convention (check `.treeindex` before exploring) are sufficient. A dedicated subagent adds complexity without meaningful benefit -- Claude can run the CLI command directly when needed.

### WP03: Integration -- COMPLETE

- [x] Update CLAUDE.md with `intent treeindex` command and `.treeindex` convention
- [x] Add Treeindex section to CLAUDE.md explaining usage and location
- [x] Update version reference to v2.3.4
- [x] Update CHANGELOG.md and RELEASE_NOTES with CLAUDE.md convention
- [x] Run full test suite -- 265/265 passing, no regressions
- [x] Run `intent doctor` -- 0 errors, 0 warnings
- [x] Generate initial `.treeindex` for Intent project as validation
- [x] Force-push v2.3.4 tag to both remotes (local + upstream)

#### WP03 As-Built Notes

- **No subagent registration**: WP02 was skipped, so no `global-agents.json`, `metadata.json`, or `agent.md` changes needed
- **No new subagent tests**: No `intent claude subagents list/install/show` testing since no subagent was created
- **Scope reduced**: WP03 became purely CLAUDE.md updates, release docs, and verification
- **Tag moved**: v2.3.4 tag was moved from the WP01-only commit to the WP03-inclusive commit

## Dependencies

WP01 -> WP03 (WP02 skipped)
