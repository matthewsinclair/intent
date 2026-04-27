---
verblock: "26 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-01
title: "Migration function: atomic .intent to intent/.config relocation"
scope: Medium
status: Done
---

# WP-01: Migration function -- atomic `.intent/` to `intent/.config/` relocation

## Objective

Implement the directory-relocation logic inside `migrate_v2_9_0_to_v2_10_0` (`bin/intent_helpers:1098-1137`). After WP01, calling the function on a v2.9.0 project performs an atomic move of `.intent/` to `intent/.config/`, stamps the new location with `intent_version: 2.10.0`, and is fully idempotent on re-invocation.

## Context

The migration function is already scaffolded with version-stamp + canon-apply logic. The relocation is the missing core. Per `design.md` D2 (atomic; fail-forward; no backwards-compat) and D3 (whole-tree preservation; do not understand contents), the move is a single `mv` with a sentinel for crash recovery and an `EXDEV` fallback for cross-filesystem cases.

Current state of the function:

- Line 1098: function signature `migrate_v2_9_0_to_v2_10_0()`.
- Lines 1103-1105: TODO comment reserving space for the relocation block (added during ST0035 retarget).
- Lines 1107-1120: existing version-stamp logic, **still pointing at `.intent/config.json`** -- needs to flip to `intent/.config/config.json` after WP01 lands.
- Lines 1122-1134: canon-apply via `intent claude upgrade --apply` -- runs after stamp, expects `intent/.config/` to exist.

WP01 inserts the relocation block immediately after the function header (before the stamp), then re-targets the stamp logic to the new path. Ordering: relocation -> stamp -> canon-apply.

## Deliverables

1. **Relocation block** inserted in `migrate_v2_9_0_to_v2_10_0` between line 1101 (entry log) and line 1107 (current stamp). Block performs:
   1. **Recovery probe**: if `intent/.config/.migration-in-progress` sentinel exists from a prior interrupted run, error out with a diagnostic pointing at `intent/docs/migration-v2.10.0.md#recovery` (created in WP07).
   2. **Idempotence shortcut**: if `intent/.config/` exists and `.intent/` does not, skip the move and proceed to stamp.
   3. **Symlink refusal**: if `.intent` is a symlink (`[ -L "$project_root/.intent" ]`), error and require manual intervention -- semver-breaking moves on user-symlinked layouts are unsafe.
   4. **Conflict refusal**: if both `.intent/` and `intent/.config/` exist (partial migration retry of unknown provenance), error and direct user to recovery doc.
   5. **Parent-create**: `mkdir -p "$project_root/intent"` (handles fresh-from-v2.0.0 projects where `intent/` exists already; harmless otherwise).
   6. **Sentinel write**: `touch "$project_root/.intent/.migration-in-progress"` before the move so a crash mid-`mv` is detectable.
   7. **Atomic move**: `mv "$project_root/.intent" "$project_root/intent/.config"`. On `EXDEV` (exit 18 on BSD; "Invalid cross-device link" on GNU), fall back to `cp -a` + checksum verification + `rm -rf`.
   8. **Sentinel removal**: `rm "$project_root/intent/.config/.migration-in-progress"` after the move succeeds.
2. **Stamp re-target**: lines 1107-1120 flip from `$project_root/.intent/config.json` to `$project_root/intent/.config/config.json` (both the idempotence read and the temp-file write).
3. **Helper extraction** (Highlander): if the relocation block exceeds ~30 LOC, extract to a named helper `intent_relocate_dotintent` in `bin/intent_helpers` and invoke from the migration function. Single source of truth in case future migrations need the same primitive.
4. **Function-header comment**: update the existing block-comment (lines 1082-1097) to reference the now-implemented relocation; remove the "added by ST0036/WP01" placeholder.

## Approach

1. Read the current scaffold (`bin/intent_helpers:1082-1137`) and the v2.8.2 -> v2.9.0 migration (`bin/intent_helpers:1023-1080`) for pattern reference.
2. Draft the relocation block as a self-contained sub-function first; verify by hand-tracing each branch on paper.
3. Insert into `migrate_v2_9_0_to_v2_10_0` between entry log and stamp.
4. Re-target stamp paths.
5. Extract to helper if over the LOC threshold.
6. Manual verification: scratch project at v2.9.0, run upgrade, inspect.
7. BATS coverage lands in WP05 (`tests/unit/migrate_v2_9_0_to_v2_10_0.bats`); listed there, not here.

## Acceptance Criteria

- [ ] On a fresh v2.9.0 project (`.intent/config.json` present, `intent/.config/` absent), calling the function produces: `intent/.config/config.json` with `intent_version: 2.10.0`, `.intent/` removed, no sentinel left behind.
- [ ] Idempotence: a second call on the same project (now at v2.10.0) returns 0 with no filesystem mutation. Verified by `find $project_root -newer ...` checks in BATS.
- [ ] Recovery: if sentinel `intent/.config/.migration-in-progress` exists at function entry, the function errors (exit non-zero) with a diagnostic pointing at the migration guide. No partial work performed.
- [ ] Symlink refusal: if `.intent` is a symlink, the function errors and does not unlink, copy, or rename anything.
- [ ] Conflict refusal: if both `.intent/` and `intent/.config/` exist, the function errors and does not touch either.
- [ ] Cross-filesystem: on `EXDEV` (exit 18), the fallback copies tree, verifies checksum, then removes source. Permissions and timestamps preserved.
- [ ] Stamp lives at the new path post-migration; reading the old path returns nothing.
- [ ] Helper extracted if relocation block exceeds ~30 LOC; otherwise inlined in the migration function.
- [ ] Function-header comment is current (no stale TODO).

### Tests to add

(Owned by WP05 -- listed here for visibility, not implemented in WP01.)

- `tests/unit/migrate_v2_9_0_to_v2_10_0.bats`:
  - clean migration (happy path).
  - idempotent re-run.
  - sentinel-detected recovery error.
  - symlink refusal.
  - conflict refusal.
  - cross-filesystem fallback (skipped on macOS with comment; runs in CI on Linux if a tmpfs is available).

### Tests to update

- None directly; WP05 owns BATS-fixture flips for the rest of the suite.

## Dependencies

- **Blocks**: WP02 (path probes need to know the new layout); WP07 (migration guide describes this function's behaviour and recovery path); WP08 (Intent self-apply runs this function on Intent's own `.intent/`).
- **Blocked by**: none. WP01 is the leaf; everything else builds on it.

## Implementation Notes

- The existing `intent claude upgrade --apply` invocation at lines 1126-1133 must come **after** relocation. The canon installer reads `intent/.config/config.json`; moving that point before relocation breaks it.
- Use `command -v intent` guard already present (line 1126); unchanged.
- Sentinel name from design D2: literal `.migration-in-progress`. Lives inside `intent/.config/` (not `.intent/`) so detection survives a successful move.
  - Subtle: write the sentinel into `.intent/.migration-in-progress` **before** the move; after `mv`, it is now at `intent/.config/.migration-in-progress`; remove from there.
- Recovery doc path (created in WP07): `intent/docs/migration-v2.10.0.md#recovery-from-interrupted-migration`. Hard-code the anchor in the diagnostic; WP07 owns the doc anchor.
- The function gets `project_root` as `$1` (existing contract). Use `$project_root/.intent` and `$project_root/intent/.config` for all paths. No `cd`.
- BSD `mv` (macOS) and GNU `mv` (Linux) handle `EXDEV` differently:
  - BSD: returns 1; stderr contains `"cross-device link"`.
  - GNU: handles EXDEV transparently via internal copy+rm; no fallback needed.
  - Detect by capturing stderr and matching, or unconditionally try `cp -a + rm` after a failed `mv`.
- For the fallback `cp -a`: `tar`-pipe (`tar c -C "$src" . | tar x -C "$dst"`) preserves permissions, ownership, xattrs more reliably than `cp -a` across BSD/GNU. Either works for Intent's data (config + cache + backup); pick one and document.
- Checksum verification after fallback: `find ... -type f | xargs shasum` on src and dst before delete. Mismatch = abort, leave both trees + sentinel for manual recovery.
- The stamp re-target should preserve the existing `del(.version)` jq filter -- v2.0.0 -- v2.1.0 introduced a duplicate `version` field that's been pruned in every migration since.

## Risks and Edge Cases

- **`intent/` exists already with content** (the project already uses Intent, just at v2.9.0). Mitigation: `mkdir -p` is harmless; `mv .intent intent/.config` lands as a sibling of `intent/st/`, etc.
- **`.intent/` is a symlink**: Refuse and error. User decision -- they probably symlinked for a reason (separate volume, network share). Document recovery in WP07.
- **`.intent/` contains files with the `.migration-in-progress` name** (vanishingly unlikely but possible from a prior aborted user experiment). Mitigation: sentinel collision is detectable; refuse with diagnostic.
- **Permissions issue** (read-only filesystem at `$project_root`). Mitigation: probe writability before sentinel write; bail with clear diagnostic.
- **Disk-full during fallback `cp -a`**: detected via non-zero exit; sentinel left in place; source preserved; user re-runs after freeing space.
- **Cross-filesystem race** (rare): user moves their Intent project across volumes mid-migration. Mitigation: the fallback `cp -a` reads source path resolved at function entry; cross-volume midflight is undefined behaviour even in non-Intent contexts.
- **Concurrent migration runs**: two `intent upgrade` processes racing. Mitigation: sentinel-as-lock; second process sees sentinel and bails.

## Verification Steps

1. Read `bin/intent_helpers:1082-1137` post-edit; confirm relocation block ordering: entry log -> recovery probe -> idempotence shortcut -> symlink/conflict refusal -> parent-create -> sentinel write -> mv (with EXDEV fallback) -> sentinel remove -> stamp -> canon apply.
2. Hand-trace each branch on paper:
   - Happy path: `.intent/` exists, `intent/.config/` does not, no sentinel.
   - Re-run: `intent/.config/` exists, `.intent/` does not.
   - Recovery: sentinel exists.
   - Conflict: both exist.
   - Symlink: `.intent` is a symlink.
3. Manual scratch test: build a v2.9.0 project (`mkdir scratch && cd scratch && intent init scratch && jq '.intent_version = "2.9.0"' .intent/config.json | sponge .intent/config.json`), source `bin/intent_helpers`, call `migrate_v2_9_0_to_v2_10_0 "$(pwd)"`. Verify post-state.
4. Repeat manual test with sentinel pre-staged: `mkdir -p .intent && touch .intent/.migration-in-progress`. Confirm error.
5. WP05 BATS scenarios cover the same matrix automatically.

## Size and Estimate

- **Size**: M (Medium). 1-2 sessions including helper extraction and manual verification. BATS land in WP05.

## Exit Checklist

- [ ] Relocation block in `migrate_v2_9_0_to_v2_10_0` (or extracted to `intent_relocate_dotintent` if > 30 LOC).
- [ ] Recovery / idempotence / symlink / conflict / parent-create / sentinel / atomic mv / EXDEV fallback / sentinel-remove sequence in place.
- [ ] Stamp logic re-targeted to `intent/.config/config.json`.
- [ ] Function-header comment current (TODO removed).
- [ ] Manual scratch verification on all five branches.
- [ ] BATS coverage staged for WP05.
- [ ] Committed: `feat: ST0036/WP-01 migration function -- atomic .intent to intent/.config`.
