---
verblock: "26 Apr 2026:v0.3: matts - Done"
wp_id: WP-05
title: "BATS fixtures and create_test_project flip"
scope: Medium
status: Done
---

# WP-05: BATS fixtures and `create_test_project` flip

## Objective

Update BATS test fixtures, helpers, and assertions so the suite exercises `intent/.config/` as the per-project metadata root. Add new test scenarios for the WP01 migration function (happy path, idempotence, recovery, symlink/conflict refusal). Result: full BATS suite green on the v2.10.0 layout.

## Context

Recon found 153 `.intent/` references across the test tree, with the heaviest concentration in:

- `tests/unit/ext_migration.bats` -- 42 hits.
- `tests/unit/init_commands.bats` -- 21 hits.
- `tests/unit/pre_commit_hook.bats` -- 14 hits.
- `tests/unit/agent_commands.bats` -- 13 hits.
- `tests/unit/skills_commands.bats` -- 10 hits.
- `tests/unit/learn_commands.bats` -- 9 hits.
- `tests/unit/intent_critic.bats` -- 8 hits.
- `tests/unit/test_diogenes.bats` -- 6 hits.
- `tests/unit/st_zero_commands.bats` -- 6 hits.
- `tests/unit/subdir_invocation.bats` -- 4 hits.

Plus `tests/lib/test_helper.bash::create_test_project` (lines 42-65) is the central fixture that creates `.intent/config.json` in test temp dirs. Updating that helper is the single biggest lever -- every test that calls it inherits the flip automatically.

After the helper flips: every test that asserts on `.intent/...` paths fails. Per-test fix-ups are mechanical but tedious. New BATS for WP01 add the migration scenarios.

## Deliverables

1. **`tests/lib/test_helper.bash::create_test_project`** updated:
   - `mkdir -p "$dir/intent/.config"` (was `mkdir -p "$dir/.intent"`).
   - `cat > "$dir/intent/.config/config.json"` (was `> "$dir/.intent/config.json"`).
   - Stamp `intent_version: "2.10.0"` in the new fixture (was `"2.0.0"` -- bump while we're touching it).
   - Existing standard-directory creation (`intent/st/{COMPLETED,NOT-STARTED,CANCELLED}` etc.) unchanged.
2. **Per-file BATS assertion updates**: every `assert_file_exists "...../.intent/..."`, every direct `cat "$dir/.intent/..."`, every `[ -f .intent/... ]` flips to `intent/.config/...`. Files affected (per recon): the 10 unit BATS listed above plus any test that uses `create_test_project` and then directly probes `.intent/`.
3. **New `tests/unit/migrate_v2_9_0_to_v2_10_0.bats`** with 6 scenarios:
   - **Clean migration**: pre-state v2.9.0 (`.intent/config.json` v2.9.0); run function; post-state v2.10.0 (`intent/.config/config.json` v2.10.0; `.intent/` absent; no sentinel).
   - **Idempotent re-run**: post-clean-migration state; run function again; assert no filesystem mutation (tree-snapshot diff via `find ... | xargs shasum`).
   - **Sentinel recovery**: pre-state has `intent/.config/.migration-in-progress`; run function; assert non-zero exit, no mutation, diagnostic mentions `migration-v2.10.0.md`.
   - **Symlink refusal**: pre-state has `.intent` as symlink to elsewhere; run function; assert non-zero exit, symlink intact, target unchanged.
   - **Conflict refusal**: pre-state has both `.intent/` and `intent/.config/`; run function; assert non-zero exit, both trees intact.
   - **Cross-filesystem fallback**: skipped on macOS with explanatory comment; runs on Linux if `tmpfs` mount available (CI).
4. **Update `tests/unit/ext_migration.bats`** -- 42 hits in this file alone. The bulk of WP05 effort. Each fixture-or-assertion needs `.intent/` -> `intent/.config/`.
5. **Update other affected BATS files** per the recon list.
6. **`intent_doctor` sentinel-detection BATS scenario** in `tests/unit/intent_doctor.bats` (or a new file): create a fixture with the sentinel; run doctor; assert it surfaces the diagnostic.
7. **Confirm `tests/run_tests.sh`** green at end.

## Approach

1. Update `create_test_project` first; commit nothing yet.
2. Run `tests/run_tests.sh` -- expect many failures.
3. Walk failing tests in order; for each, update the assertion to the new path.
4. Add the new BATS file `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` with all 6 scenarios.
5. Add the doctor sentinel scenario.
6. Re-run; iterate until green.
7. Commit as a single coherent change (or sub-commit per affected test file if the diff is large).

## Acceptance Criteria

- [ ] `create_test_project` emits `intent/.config/config.json`.
- [ ] Every BATS green (`tests/run_tests.sh` exit 0).
- [ ] New `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` exists; covers all 6 scenarios; all green.
- [ ] Doctor sentinel scenario exists; passes.
- [ ] No `.intent/` literals remaining in `tests/` except: legitimate references to `~/.intent/` (extension root) or `.intent_critic.yml` (file at root).
- [ ] No false negatives: spot-check 3-4 critical tests by deliberately breaking the migration; confirm they fail loudly, not silently.

### Tests to add

- `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` (6 scenarios).
- Doctor sentinel scenario (one new `@test` block).

### Tests to update

- All 10 BATS files in the recon list, plus any others uncovered by the run.

## Dependencies

- **Blocks**: WP08 (self-apply requires green BATS suite).
- **Blocked by**: WP03 (literal sweep establishes the categorisation rules used here too).

## Implementation Notes

- `create_test_project` is the single highest-leverage fix. Updating it covers every test that uses it; the manual per-test work is for tests with hand-rolled fixtures.
- **Tree-snapshot idempotence pattern** (proven in ST0035/WP-11 Session 3 BATS): `before=$(cd "$dir" && find . -type f \! -newer ... | xargs shasum | sort | shasum); ... ; after=$(...); [ "$before" = "$after" ]`. Same approach works here for the WP01 idempotence scenario.
- For the cross-filesystem scenario: use `[ "$(uname)" = "Linux" ] && [ -w /dev/shm ] && skip_if_no_tmpfs` or similar guard; on macOS, skip with a one-line `skip "cross-FS test requires Linux tmpfs"`.
- The symlink refusal scenario: `ln -s /tmp/elsewhere "$dir/.intent"`; run; assert symlink target unchanged.
- The migration function takes `$1` as project root; tests invoke directly via `source bin/intent_helpers; migrate_v2_9_0_to_v2_10_0 "$test_dir"` -- no need to spawn `intent upgrade` for the unit-level scenarios.
- For the integration-level assertion (running `intent upgrade` end-to-end), one or two scenarios in `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` should exercise the full dispatcher. Mark them clearly.
- Per the no-vanity-metrics rule, do NOT report test counts in commit messages or wrap docs.

## Risks and Edge Cases

- **Hidden assertions** on file-system layout: a test that does `cd .intent/...` not via `assert_file_exists` will fail at runtime, not in the grep sweep. Mitigation: full BATS run after each batch of edits.
- **Test ordering**: BATS isolates each test, but `setup_file` / `teardown_file` patterns in some files may share state. Confirm none of the 10 affected files share fragile state.
- **Symlink-test cleanup**: ensure `teardown` removes the symlink, otherwise subsequent tests inherit the cruft.
- **Cross-filesystem test in CI**: if CI does not have `tmpfs`, the test skips silently -- ensure the skip message is informative.
- **Migration scenarios race with the canon-apply call**: WP01's function calls `intent claude upgrade --apply` after the relocation. In test harnesses, this may not be desired (slow; downloads canon files). Mitigation: a test-mode flag, or use `INTENT_SKIP_CANON_APPLY=1` environment guard. Decide in WP05; document.

## Verification Steps

1. `tests/run_tests.sh` -- exit 0.
2. Spot-grep: `grep -rn '\.intent/' tests/ | grep -v '~/.intent/' | grep -v '.intent_critic'` -- 0 hits or only documented keeps.
3. New `migrate_v2_9_0_to_v2_10_0.bats` exists; 6 scenarios; all green.
4. Doctor sentinel test exists and green.

## Size and Estimate

- **Size**: M (Medium). Likely 1-2 sessions; large file count + new BATS file. The mechanical per-test flips dominate.

## Exit Checklist

- [ ] `create_test_project` flipped.
- [ ] All affected BATS updated.
- [ ] New migrate BATS file with 6 scenarios.
- [ ] Doctor sentinel BATS scenario.
- [ ] Full suite green.
- [ ] Committed: `test: ST0036/WP-05 BATS fixtures + migration test scenarios for intent/.config`.
