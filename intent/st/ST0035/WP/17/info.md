---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-17
title: "Verification sweep and dogfood journal"
scope: Small
status: Not Started
---

# WP-17: Verification sweep and dogfood journal

## Objective

After the fleet rollout (WP16), sweep all 17 projects (Intent self + 16 downstream) for compliance with the canon. Produce a ST0035 feedback report and a dogfood journal capturing surprises, rough edges, and lessons. Close the ST, mark CHANGELOG v2.9.1 final, push upstream.

## Context

This is the final WP. All canon artefacts exist (WP02–WP11), Intent self-dogfooded (WP14), canary clean (WP15), fleet rolled (WP16). WP17 is verification + documentation + release.

The dogfood journal is not optional — it's the load-bearing QA signal. Every roughness surfaced here informs the next ST.

## Deliverables

1. **Fleet-wide verification report** at `intent/st/ST0035/WP/17/feedback-report.md`:
   - 17 rows (one per project + Intent).
   - 10-point verification checklist per project (same as WP15 / WP16).
   - Pass / fail per check.
   - Any outstanding issues with tickets filed.
2. **Dogfood journal** at `intent/st/ST0035/WP/17/dogfood-journal.md`:
   - Chronological entries as each WP was executed.
   - What broke (with specifics). What was rough (friction points, unclear docs). What surprised.
   - Lessons for future STs (e.g., "always dogfood the canon on Intent before canary" — already our pattern, but reinforce).
   - Suggestions for v2.9.2 / v2.10.0 follow-ups.
3. **CHANGELOG v2.9.1 final**: assemble the full v2.9.1 entry from all WP contributions (Added, Changed, Removed sections).
4. **Tag and push**:
   - `git tag -f v2.9.1 HEAD` in Intent.
   - Push to `local` (Dropbox) and `upstream` (GitHub).
   - GitHub release: `gh release edit v2.9.1 --notes-file CHANGELOG-v2.9.1.md` or similar.
5. **Close ST0035**:
   - `intent st done ST0035` (marks Completed status, moves to COMPLETED dir).
   - Update `intent/wip.md` to reflect no active ST.
   - Update `intent/restart.md` with post-release context.
6. **Memory update**: auto-memory entries updated to reflect v2.9.1 release (user's `MEMORY.md` Active Work section).

## Approach

1. Walk through each of 17 projects (Intent + 16 downstream) and run the 10-point verification, capturing results.
2. Assemble the feedback report (table form).
3. Review WP-by-WP commit logs + per-WP reports (from WP14/WP15/WP16) to assemble dogfood journal entries.
4. Draft CHANGELOG v2.9.1:
   - **Added**: `bin/intent_critic`; `.claude/settings.json` hooks canon; `.git/hooks/pre-commit` critic gate; `.intent_critic.yml` config; `intent/docs/working-with-llms.md`; `intent/docs/pre-commit-hook.md`; `lib/templates/.claude/`; `lib/templates/hooks/`.
   - **Changed**: `intent agents sync` now writes to root `AGENTS.md`; root `CLAUDE.md` template is now Claude-specific overlay; root `usage-rules.md` refreshed to v2.9.0+ surface.
   - **Removed**: `intent/llm/AGENTS.md` (moved to root); `lib/templates/llm/_llm_preamble.md` (deprecated); ST0010 (cancelled); ST0015 (cancelled).
5. Tag and push (confirm with user before force-pushing tag).
6. Mark ST0035 complete.
7. Commit the final set in Intent: reports + journal + CHANGELOG finalisation.

## Acceptance Criteria

- [ ] Feedback report exists with 17 rows, 10 checks each.
- [ ] Dogfood journal has ≥ one entry per WP (chronological).
- [ ] CHANGELOG v2.9.1 has Added / Changed / Removed sections.
- [ ] Zero outstanding critical issues (tickets filed for anything non-critical).
- [ ] `git tag v2.9.1` present in Intent.
- [ ] Tag pushed to `local` and `upstream`.
- [ ] GitHub release for v2.9.1 visible.
- [ ] `intent st done ST0035` executed; ST0035 now in COMPLETED.
- [ ] `intent/wip.md` updated; `intent/restart.md` updated.
- [ ] User's `MEMORY.md` Active Work section updated to reflect v2.9.1 shipped.
- [ ] All previous WP01–WP16 and WP18 exit checklists verified complete.
- [ ] Commit messages follow Intent conventions, no Claude attribution.

### Tests to add

None.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP16 (fleet applied), WP18 (user-doc review closed).
- **Blocks**: None — WP17 closes ST0035.

## Implementation Notes

- **Feedback report as a matrix**: 17 × 10 = 170 cells. Most will be green. Format as a compact markdown table; abbreviate check names in headers.
- **Dogfood journal voice**: first-person or passive voice, whichever feels natural. Focus on _surprises_, not confirmations. "The XY generator needed an extra path-escape step" matters; "canon applied successfully" is not journal-worthy.
- **CHANGELOG format**: follow Intent's existing CHANGELOG.md style. Check previous v2.9.0 entry as template.
- **Tag push confirmation**: per user's past convention (from memory), tag workflow is `git tag -f vX.Y.Z HEAD` then force-push to both remotes. Confirm with user before force-pushing if any ambiguity.
- **Release notes**: assemble CHANGELOG v2.9.1 section as the release notes; `gh release edit` or `gh release create` per user's usual flow.
- **MEMORY.md update**: remove ST0034-era "Active Work" entries; add ST0035 completion note.

## Risks and Edge Cases

- **Risk**: Verification reveals a project in inconsistent state after WP16. **Mitigation**: file issue; determine if it blocks ST0035 closure or is a follow-up.
- **Risk**: Tag push fails due to remote config drift. **Mitigation**: verify remotes configured correctly before pushing (`git remote -v`); confirm with user.
- **Risk**: GitHub release workflow requires an interactive step. **Mitigation**: `gh` CLI covers all steps; pre-authenticate if needed.
- **Edge**: Some projects may have uncommitted work by the time WP17 runs (user touched them). **Mitigation**: verification reports the state; doesn't force-apply. Note in feedback report.

## Verification Steps

See Acceptance Criteria. WP17 is itself the verification WP.

## Size and Estimate

- **Size**: S (Small). 1–2 sessions.
- Session 1: Sweep 17 projects + feedback report + dogfood journal.
- Session 2: CHANGELOG + tag + release + ST close.

## Exit Checklist

- [ ] Feedback report complete.
- [ ] Dogfood journal complete.
- [ ] CHANGELOG v2.9.1 finalised.
- [ ] Tag pushed to both remotes.
- [ ] GitHub release published.
- [ ] ST0035 in COMPLETED.
- [ ] wip.md + restart.md updated.
- [ ] User's MEMORY.md Active Work updated.
- [ ] Committed.
