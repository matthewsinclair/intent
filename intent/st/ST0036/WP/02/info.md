---
verblock: "26 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-02
title: "Path probes for intent/.config root"
scope: Small
status: Not Started
---

# WP-02: Path probes for `intent/.config/` root

## Objective

Update Intent's central path-probing functions so every CLI invocation finds project metadata at `intent/.config/` instead of `.intent/`. Specifically: `find_project_root` and `load_intent_config` in `bin/intent_config`, plus any direct probes in other `bin/*` scripts that bypass the central function (Highlander cleanup target).

## Context

`bin/intent_config` houses the canonical path-probe surface:

- `find_project_root` (`bin/intent_config:40-67`) walks up the directory tree from `$(pwd)` looking for `.intent/config.json` (line 45). This is the v2.9.0 marker.
- `load_intent_config` (`bin/intent_config:70-116`) reads `$PROJECT_ROOT/.intent/config.json` (line 92).
- Both export `PROJECT_ROOT` for downstream commands.

Recon revealed that several other `bin/*` scripts probe `.intent/config.json` directly (Highlander leak):

- `bin/intent_st:72`, `:89` -- direct file checks.
- `bin/intent_st_zero:224, :229, :591, :594, :603, :694, :695` -- multiple direct probes and creation.
- `bin/intent_learn:13, :38` -- learnings file lookup.
- `bin/intent_init:83, :95, :100` -- creates `.intent/` directly.
- `bin/intent_upgrade:38, :365, :391` -- references in upgrade flow.
- `bin/intent_treeindex:440` -- a treeindex-related lookup.

WP02 flips every direct probe and every direct creation site. The Highlander posture: ideally everything funnels through `find_project_root` rather than re-probing. Where reasonable, this WP refactors duplicate probes to call the central function. Where not (e.g., `intent_init` creates the dir; can't probe what doesn't exist yet), the literal flips in place.

`bin/intent_doctor` deserves special attention: WP02 adds a "interrupted migration" check for the sentinel `intent/.config/.migration-in-progress` (the WP01 sentinel) and surfaces it with a remediation hint.

## Deliverables

1. **`bin/intent_config::find_project_root`** flipped: `[ -f "$current_dir/intent/.config/config.json" ]` is the v2.10.0 marker. Per design D2 (fail-forward), no v2.9.0-fallback probe -- a v2.9.0 project will fail to be found, prompting the user to run `intent upgrade`.
2. **`bin/intent_config::load_intent_config`** reads `$PROJECT_ROOT/intent/.config/config.json`.
3. **`bin/intent_helpers::require_project_root`** if it does any direct probe (line 116; verify), flipped.
4. **`bin/intent_init`** lines 83 (existence check), 95 (`mkdir -p .intent`), 100 (config write), 232 (`.intent/cache/` reference): all flip to `intent/.config/...`.
5. **`bin/intent_st`** lines 72, 89: flip the direct probes (or refactor to call `find_project_root`).
6. **`bin/intent_st_zero`** lines 224, 229, 591, 594, 603, 694, 695: flip and refactor where central function is callable.
7. **`bin/intent_learn`** lines 13, 38: flip the `LEARNINGS_FILE` path constant and the storage echo.
8. **`bin/intent_upgrade`** lines 38, 365, 391: flip references in the upgrade flow itself (NB: this is the user-facing upgrade dispatcher, not the migration functions; the dispatcher that selects + runs `migrate_v2_*`).
9. **`bin/intent_treeindex`** line 440: flip.
10. **`bin/intent_doctor`** new check: probe for `intent/.config/.migration-in-progress` sentinel; if present, report `interrupted migration: see intent/docs/migration-v2.10.0.md#recovery`. Slot into the existing `checking:` series so it lives alongside `intent_home`, `local config`, etc.
11. **Highlander refactor pass**: any script that probed `.intent/` directly and could call `find_project_root` instead -- refactor.

## Approach

1. Re-confirm the recon list with `grep -nE '\.intent/config' bin/` -- catch anything new.
2. Flip `bin/intent_config` first (the canonical surface). All other scripts depend on this.
3. Flip every other script in dependency order (intent_helpers -> intent_init -> intent_st -> ... ).
4. For each script, decide: literal flip vs refactor to call `find_project_root`. Default: refactor where the probe is purely for "am I in a project?"; flip where the probe is for a specific file path.
5. Add the `intent_doctor` sentinel check.
6. Run `intent doctor` on Intent itself (still on v2.10.0 / .intent layout pre-WP08); confirm it surfaces nothing unexpected.
7. Manual verification: build a scratch project with `intent/.config/` directly, source `bin/intent_config`, call `find_project_root` from inside -- confirm it walks up correctly.

## Acceptance Criteria

- [ ] `find_project_root` recognises a v2.10.0 project (presence of `intent/.config/config.json`).
- [ ] `load_intent_config` reads from `intent/.config/config.json`.
- [ ] `intent_doctor` reports interrupted migration when the sentinel is present.
- [ ] `grep -rn '\.intent/config' bin/` returns 0 hits.
- [ ] `grep -rn 'intent/\.config/config' bin/` shows the flipped surface.
- [ ] `grep -rn '\.intent/' bin/` returns only intentional matches: `~/.intent/ext/` references, `.intent_critic.yml` references, prose in help text describing pre-v2.10.0 state.
- [ ] `intent doctor`, `intent st list`, `intent wp list <ID>`, `intent agents sync`, `intent claude upgrade` all functional on a v2.10.0-shaped scratch project.
- [ ] Highlander pass: any `bin/*` script that previously did a direct probe is either refactored to call `find_project_root` or has a documented reason for not doing so.

### Tests to add

(Owned by WP05.) Doctor sentinel-detection BATS scenario.

### Tests to update

(Owned by WP05.) Every BATS that creates fake `.intent/` setups; flips at the helper level (`create_test_project`).

## Dependencies

- **Blocks**: WP03 (literal sweep for non-`bin/` paths -- once `bin/` is canonical, sweeping the rest follows the same pattern).
- **Blocked by**: WP01 (function defines the new layout).

## Implementation Notes

- Per design D2 (fail-forward), no "support both paths" period. A v2.9.0 project fails the new probe and the user runs `intent upgrade` -- which calls `migrate_v2_9_0_to_v2_10_0` -- which performs the relocation per WP01. Closed loop.
- `intent_doctor` sentinel check should slot into the existing diagnostic series so the user sees it alongside other checks. Suggested label: `interrupted migration` (lowercase, matches existing style at `bin/intent_doctor:155` etc.).
- For scripts like `intent_st_zero` with multiple probes scattered through the file, prefer the refactor-to-central-function approach -- otherwise WP02 leaves the Highlander state slightly worse than it found it.
- The `intent_init` flip is structural: line 95's `mkdir -p .intent` becomes `mkdir -p intent/.config` (and ensures `intent/` parent exists, similar to WP01's parent-create).

## Risks and Edge Cases

- **Hidden direct probes**: scripts that string-build `.intent/...` from a constant. Mitigation: thorough grep + read each `bin/*` end-to-end during the WP.
- **Tests that hard-code `.intent/`**: every BATS that creates a fixture. Mitigation: WP05 owns this; WP02 just guarantees the bin/-side path probes work on `intent/.config/`.
- **External caller**: any third-party script calling Intent's CLI relies on the side-effect of `find_project_root` printing the project root, not on the file location. Should not break.
- **Refactoring scope creep**: the Highlander refactor pass is tempting to expand. Defer non-essential cleanups to a follow-up.
- **`intent_doctor` adds a new check** -- ensure the diagnostic counter (`bin/intent_doctor:112`) is incremented appropriately so summary line is accurate.

## Verification Steps

1. `grep -rn '\.intent/config' bin/` -- 0 hits.
2. `grep -rn 'intent/\.config/' bin/` -- shows the flipped surface; spot-check 5 hits for context correctness.
3. Manual scratch: `mkdir -p /tmp/v210-scratch/intent/.config && echo '{"intent_version":"2.10.0","project_name":"scratch"}' > /tmp/v210-scratch/intent/.config/config.json`. From `/tmp/v210-scratch/some/subdir/`, run `intent doctor`. Should report clean.
4. Manual scratch with sentinel: `touch /tmp/v210-scratch/intent/.config/.migration-in-progress`. Run `intent doctor`. Should report interrupted migration.
5. `intent st list`, `intent wp list ST0036`, `intent agents sync` -- all clean against the scratch.
6. WP05 BATS run -- expect green.

## Size and Estimate

- **Size**: S (Small). One session.

## Exit Checklist

- [ ] All `bin/*` direct probes flipped.
- [ ] Doctor sentinel check added with correct counter handling.
- [ ] Highlander refactor pass complete (or explicit "leave alone" notes).
- [ ] Manual scratch verification passes.
- [ ] No regressions on test suite (deferred to WP05 BATS).
- [ ] Committed: `refactor: ST0036/WP-02 path probes for intent/.config root`.
