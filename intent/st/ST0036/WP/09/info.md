---
verblock: "26 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-09
title: "Merge with ST0035 fleet rollout (coord)"
scope: Extra Small
status: Not Started
---

# WP-09: Merge with ST0035 fleet rollout (coordination bucket)

## Objective

Coordinate ST0036's directory move into ST0035's existing canary + fleet rollout WPs (`ST0035/WP15` Conflab/Lamplight/Laksa canary, `ST0035/WP16` 12 Intent + Pplr fleet, `ST0035/WP17` verification sweep). No standalone ST0036 rollout WP; both concerns ride together via a single `intent upgrade` per fleet project. WP09 is purely documentation: capture the coordination contract in the affected WP info.md files and in `ST0036/impl.md`.

## Context

Per `design.md` D4: shared rollout with ST0035. The fleet runs `intent upgrade` once per project; the `migrate_v2_9_0_to_v2_10_0` function (WP01) performs the directory move, then invokes `intent claude upgrade --apply` for the ST0035 canon. Both concerns land in the same upgrade pass.

ST0035's existing rollout WPs need to be aware:

- `ST0035/WP15` (canary): three projects in sequence (Conflab -> Lamplight -> Laksa). Each project's verification list now includes "no `.intent/` remaining; `intent/.config/` present".
- `ST0035/WP16` (fleet): 12 Intent projects + Pplr. Same verification.
- `ST0035/WP17` (verification sweep): the per-project 10-point checklist gains a tick for the directory state.

WP09 updates these info.md files. No code lands; no tests. Pure coordination.

## Deliverables

1. **`ST0035/WP/15/info.md`** updated:
   - Verification steps add: "(g) `ls intent/.config/` shows config.json + cache/ + backup/; (h) `! [ -d .intent ]` (no leftover legacy directory)".
   - Acceptance criteria adds the corresponding tick.
2. **`ST0035/WP/16/info.md`** updated similarly.
3. **`ST0035/WP/17/info.md`** updated:
   - The 10-point per-project checklist gains a row: "directory state: `intent/.config/` present, `.intent/` absent".
   - The fleet-wide feedback report (`feedback-report.md` deliverable) explicitly aggregates the directory-state tick.
4. **`intent/st/ST0036/impl.md`** updated:
   - Coordination note: "rollout shares ST0035/WP15-WP17; no standalone WP. Per design D4."
5. **`CHANGELOG.md`** v2.10.0 entry: confirm it captures both ST0035 (LLM canon) and ST0036 (directory move) under one release. WP03 likely landed this; WP09 verifies.

## Approach

1. After WP08 lands, before ST0035/WP15 begins: read `ST0035/WP/15/info.md`, `WP/16/info.md`, `WP/17/info.md` end-to-end.
2. For each, add the directory-state verification step in the appropriate section.
3. Update `ST0036/impl.md` with the coordination note.
4. Verify `CHANGELOG.md` covers both concerns.
5. Commit.

## Acceptance Criteria

- [ ] `ST0035/WP/15/info.md` mentions ST0036 directory verification.
- [ ] `ST0035/WP/16/info.md` mentions ST0036 directory verification.
- [ ] `ST0035/WP/17/info.md` checklist includes the directory-state tick.
- [ ] `ST0036/impl.md` documents the bundling.
- [ ] `CHANGELOG.md` entry captures both.
- [ ] No new ST0036 rollout WP exists (this is purely coord).

### Tests to add / update

- None. Pure documentation.

## Dependencies

- **Blocks**: ST0035/WP15 + WP16 + WP17 (which carry both ST0035 + ST0036 concerns post-WP09 doc updates). Also implicitly: ST0035/WP14 (Intent self-dogfood for canon, which assumes WP08 self-relocation has happened first).
- **Blocked by**: WP08 (self-apply done; rollout machinery proven on Intent).

## Implementation Notes

- WP09 is a coordination bucket -- existence justifies itself only if the cross-thread changes need to be tracked. Could in principle fold into WP08's commit, but separating gives a clean WP-level audit trail.
- The "directory state" check in WP17 should be machine-checkable: `[ ! -d .intent ] && [ -d intent/.config ]` per project. Adds to the per-project verification script if WP17 has one.
- ST0035/WP14 (Intent self-dogfood for canon) is **implicitly** dependent on WP08 (Intent self-relocation). When WP14 runs `intent claude upgrade --apply` on Intent, the ST0036 layout must be in place. Documenting this dependency in WP14's info.md is part of WP09's scope.
- Per `design.md` D4: "ST0036's implementation WPs (WP01 through the pre-rollout Intent self-apply) must land before ST0035/WP14." WP09 surfaces this in ST0035/WP14's info.md if it isn't already noted.

## Risks and Edge Cases

- **Cross-thread drift**: if ST0035 WP15-17 evolve independently after WP09 lands, the coordination notes may go stale. Mitigation: Phase 0 review -- the user reads this WP and confirms the cross-thread story is consistent.
- **Bundled commit messaging in fleet rollout**: when canary/fleet projects commit their `intent upgrade` results, the commit message should mention BOTH ST0035 + ST0036 concerns. Suggest the commit-message pattern in WP15/WP16 info.md. (Actual fleet commits happen during WP15/WP16 execution; WP09 just documents the convention.)

## Verification Steps

1. Read updated `ST0035/WP/15/info.md`, `WP/16/info.md`, `WP/17/info.md` -- coordination notes present.
2. Read `ST0036/impl.md` -- bundling note present.
3. `CHANGELOG.md` v2.10.0 entry -- single entry covers both ST0035 + ST0036.

## Size and Estimate

- **Size**: XS (Extra Small). 30-60 minutes -- doc edits across 3-4 files.

## Exit Checklist

- [ ] ST0035/WP15-17 info.md updated.
- [ ] ST0036/impl.md coordination note.
- [ ] CHANGELOG verified.
- [ ] Committed: `docs: ST0036/WP-09 coordinate directory move into ST0035 fleet rollout`.
