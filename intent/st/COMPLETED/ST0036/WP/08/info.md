---
verblock: "26 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-08
title: "Intent self-apply directory relocation"
scope: Small
status: Done
---

# WP-08: Intent self-apply -- directory relocation

## Objective

Run the v2.10.0 migration on Intent itself: `.intent/` -> `intent/.config/`. After WP08, Intent's own working tree reflects the new layout; the full BATS suite is green; `intent doctor` is clean; treeindex is regenerated. Lands **before** ST0035/WP14 (Intent self-dogfood for the canon LLM config) so that ST0035/WP14 runs on a relocated repo.

## Context

WP01-07 land the implementation. WP08 is the moment of truth: the first real-world execution of `migrate_v2_9_0_to_v2_10_0` against a non-trivial project (Intent itself). If anything was missed in WP01-07 -- a hard-coded `.intent/` literal, a BATS fixture not flipped, a generator emitting stale paths -- WP08 will surface it.

Intent's pre-WP08 state:

- `.intent/config.json` exists with `intent_version: 2.10.0` (already stamped during ST0035 retarget; WP01's idempotence check should detect this if the relocation has already moved the file -- but currently, the stamp is at the OLD path).
- Full BATS suite passing (post-WP05).
- `intent doctor` clean.
- `intent/.treeindex/` populated.

After WP08:

- `intent/.config/config.json` with `intent_version: 2.10.0`. `.intent/` absent. No sentinel.
- BATS suite still passing.
- `intent doctor` clean.
- `intent/.treeindex/` regenerated to reflect the new layout.

## Deliverables

1. **Pre-flight verification**:
   - `tests/run_tests.sh` -- green.
   - `intent doctor` -- clean.
   - `intent/.treeindex/` -- valid (no stale entries).
2. **Backup checkpoint**: `git tag wp08-pre-relocate` before running upgrade. Discardable; provides instant rollback if WP08 hits an unexpected snag.
3. **Run `intent upgrade`** on Intent's working tree. This invokes `migrate_v2_9_0_to_v2_10_0` (WP01) -- which performs the relocation, stamps, and runs canon-apply.
4. **Post-flight verification**:
   - `ls -la` shows `intent/.config/`, no `.intent/`.
   - `cat intent/.config/config.json | jq .intent_version` returns `"2.10.0"`.
   - `tests/run_tests.sh` -- green.
   - `intent doctor` -- clean.
   - `intent treeindex intent` regenerated; new layout visible at `intent/.config/config.json` as a leaf; cache + backup excluded.
5. **Commit**: the WP08 commit captures Intent's working-tree state post-relocation. Diff is small in code (just the moved tree) but visible in `git status` (the rename of `.intent/...` to `intent/.config/...`).

## Approach

1. Pre-flight: confirm green BATS + clean doctor + valid treeindex.
2. `git tag wp08-pre-relocate`.
3. `intent upgrade` -- watch output. Confirms the WP01 sequence: recovery probe (none), idempotence check (none), parent create, sentinel write, mv, sentinel remove, stamp re-target, canon-apply.
4. Inspect `git status` -- expect a large rename diff: every file under `.intent/` showing as moved.
5. Post-flight verification (full BATS + doctor + treeindex).
6. If green: commit. If red: investigate, fix forward (in WP01-07 territory), re-tag, re-run.
7. Remove the backup tag after a session passes without rollback need.

## Acceptance Criteria

- [ ] `intent/.config/` present in repo root; `.intent/` absent.
- [ ] `intent/.config/config.json` stamped `2.10.0`.
- [ ] No sentinel `intent/.config/.migration-in-progress` left behind.
- [ ] `intent doctor` clean.
- [ ] `tests/run_tests.sh` green.
- [ ] Treeindex reflects new layout (`intent/.config/config.json` as a leaf; cache + backup excluded).
- [ ] Git history shows the rename as a coherent diff (one or a few commits, not scattered).
- [ ] Backup tag created and discardable.

### Tests to add / update

- None new -- WP05 covered the suite. WP08 just confirms green.

## Dependencies

- **Blocks**: WP09 (rollout coordination -- ST0035/WP15+WP16 carry both concerns; can't proceed until Intent self-applies cleanly). Also implicitly blocks ST0035/WP14 (Intent self-dogfood for ST0035 canon, which assumes the relocated layout).
- **Blocked by**: WP01-07 (every preparatory WP).

## Implementation Notes

- The git diff for a directory rename is large but mechanical. `git mv` is what Intent's BATS would use in tests, but WP01 uses plain `mv`; git detects the rename heuristically. Confirm by `git status -M` or `git log --follow intent/.config/config.json` post-commit.
- The `intent claude upgrade --apply` invocation inside `migrate_v2_9_0_to_v2_10_0` (line 1126-1133) will run on Intent. It performs the canon-apply. Most actions should be no-ops because Intent already has the canon installed (post-ST0035/WP01-12). Some may register as REFRESH due to the ST0036 file-map flip in templates (WP04). That's expected and desired.
- Pre-existing AGENTS.md cosmetic drift (date stamp) per session-3 conventions: handle by `git checkout -- AGENTS.md` or include the regen in the WP08 commit. Lean to including; it's the right state post-relocation.
- The `intent_version` is already `2.10.0` per ST0035 retarget. `migrate_v2_9_0_to_v2_10_0` first stamp-block check is `if [ "$current" = "2.10.0" ]` -- which would early-return and skip the relocation entirely. **This is a bug** for WP08's purpose: the function's idempotence check on the stamp is true, but the relocation was never performed. Fix in WP01: idempotence check should be on **layout state** (`intent/.config/` exists ?), not on stamp value. Cross-reference: WP01 acceptance criteria #2 already says "idempotence: a second call on a v2.10.0 project (already at intent/.config/) is a no-op". The implementation must check layout, not stamp.
- Treeindex regen may take a minute or two on Intent's full tree -- not a blocker, just a heads-up.

## Risks and Edge Cases

- **WP01 surfaces a missed literal**: in any of WP02-07. Symptom: a BATS fails post-upgrade, or `intent doctor` reports unexpected state. Mitigation: revert via `git reset --hard wp08-pre-relocate`, fix the missed literal in the appropriate WP, re-run.
- **Pre-existing local edits**: the user-local `.claude/settings.local.json` file should not be touched by the migration (it's in `.claude/`, not `.intent/`). Confirm.
- **Treeindex stale entries**: `intent treeindex intent --prune` may be needed to remove orphaned summaries from the old layout. Run pre-commit if so.
- **Git-detected rename heuristic**: `git` should detect the rename automatically (similarity ~100%); if it shows as "delete + add" instead of "rename", the diff is uglier but functionally equivalent. Use `git log --follow` to verify history is preserved.
- **Subagent sessions in flight during the move**: Intent uses `Task()` to invoke subagents; if any are mid-call, the subagent's project-root resolution mid-call may break. Mitigation: WP08 is a one-time, intentional operation -- coordinate (don't run during active subagent calls).
- **Editor open files**: VS Code / Vim with `.intent/config.json` open will show stale data after the move. User-side concern; document.

## Verification Steps

1. Pre-flight green (BATS + doctor + treeindex valid).
2. `git tag wp08-pre-relocate`.
3. `intent upgrade` -- read output for any errors.
4. `ls intent/.config/` -- expect config.json, cache/, backup/, anything else from `.intent/`.
5. `! ls .intent` -- expect "No such file or directory".
6. `cat intent/.config/config.json | jq .intent_version` -- `"2.10.0"`.
7. `tests/run_tests.sh` -- green.
8. `intent doctor` -- clean.
9. `intent treeindex intent` -- runs cleanly; sane output.
10. Commit; verify `git log --follow intent/.config/config.json` shows the rename history.
11. After session passes: `git tag -d wp08-pre-relocate`.

## Size and Estimate

- **Size**: S (Small). One session if WP01-07 are clean. Up to 2 if a missed literal needs fix-and-rerun.

## Exit Checklist

- [ ] Pre-flight green.
- [ ] Backup tag created.
- [ ] Migration ran successfully.
- [ ] Post-flight verification all-green.
- [ ] Treeindex regenerated.
- [ ] Committed: `chore: ST0036/WP-08 Intent self-apply v2.10.0 directory move`.
- [ ] Backup tag deleted (after a stable session).
