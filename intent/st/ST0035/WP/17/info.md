---
verblock: "27 Apr 2026:v0.3: matts - scope-as-built; release engineering moved out (post-WP-19); 14-row scope; status flipped by intent wp done"
wp_id: WP-17
title: "Verification sweep and dogfood journal"
scope: Small
status: Done
---

# WP-17: Verification sweep and dogfood journal

> **Coordination note (ST0036/WP-09)**: this verification sweep covers both ST0035 (LLM canon) and ST0036 (directory relocation). The per-project checklist is 12 points (10 ST0035 + 2 ST0036; defined in WP-15). The feedback-report and dogfood-journal capture both concerns. See `intent/st/ST0036/impl.md` for bundled-release rationale.

> **Scope as built (2026-04-27)**: the 2026-04-24 spec assumed WP-17 closes ST0035 in the same WP, bundling release engineering (CHANGELOG finalisation, `git tag v2.10.0`, GitHub release, `intent st done ST0035`) into the deliverables. As built, ST0035 has 19 WPs (WP-19 added 2026-04-27 for per-language canon); release engineering moves out of WP-17 into post-WP-19 work. WP-17 is now scoped to: (1) verification matrix across 14 in-scope projects (Intent + 8 canary + 5 user-manual), (2) dogfood journal capturing rollout surprises and lessons, (3) decision on the user-manual `intent upgrade` cleanup gotcha. See `feedback-report.md` and `dogfood-journal.md`.

## Objective

After the fleet rollout (WP-15 + WP-16), sweep all 14 in-scope projects (Intent self + 8 canary + 5 user-manual) for compliance with the v2.10.0 canon. Produce a feedback-report.md (verification matrix) and a dogfood-journal.md capturing surprises, rough edges, lessons, and the decision on whether `intent upgrade` should warn / auto-stage leftover `.intent/` directories. Release engineering (CHANGELOG finalisation, tag, GitHub release, ST-close) deferred to post-WP-19.

## Context

This is the final WP. All canon artefacts exist (WP02–WP11), Intent self-dogfooded (WP14), canary clean (WP15), fleet rolled (WP16). WP17 is verification + documentation + release.

The dogfood journal is not optional — it's the load-bearing QA signal. Every roughness surfaced here informs the next ST.

## Deliverables

1. **Fleet-wide verification report** at `intent/st/ST0035/WP/17/feedback-report.md`:
   - 14 rows (Intent self + 8 canary + 5 user-manual).
   - 12-point verification checklist per project (10 ST0035 + 2 ST0036; defined in WP-15).
   - Pass / fail per check; references to the per-project canary report (where applicable) and to the WP-16 fleet-summary.
   - Any outstanding issues with tickets filed.
2. **Dogfood journal** at `intent/st/ST0035/WP/17/dogfood-journal.md`:
   - Chronological entries as each WP was executed (synthesised from session-end restart files, the WP-15 canary aggregate, the WP-16 fleet summary, and commit log).
   - What broke (with specifics). What was rough (friction points, unclear docs). What surprised.
   - Lessons for future STs (e.g., "WP closure pattern: tidy spec to as-built + write summary + `wp done`" — already our pattern, but reinforce).
   - Suggestions for v2.10.x / v2.11.0 follow-ups.
3. **Decision: should `intent upgrade` auto-handle leftover `.intent/` directories?** WP-15/WP-16 surfaced the gotcha three times (Multiplyer, MeetZaya, Courses/Agentic Coding): user-manual `intent upgrade` runs created the new `intent/.config/` layout but left the pre-existing tracked `.intent/config.json` in the working tree. Decision recorded in `dogfood-journal.md`; if "yes", a follow-up ticket is filed for v2.10.x.

**Out of scope for WP-17 (moved to post-WP-19 release engineering)**: CHANGELOG v2.10.0 finalisation, `git tag v2.10.0`, push to `upstream`, GitHub release, `intent st done ST0035`, `intent/wip.md` + `intent/restart.md` post-release update, MEMORY.md Active Work refresh. These all belong post-WP-19 (per-language canon), which closes ST0035.

## Approach

1. Walk through each of 17 projects (Intent + 16 downstream) and run the 12-point verification, capturing results.
2. Assemble the feedback report (table form).
3. Review WP-by-WP commit logs + per-WP reports (from WP14/WP15/WP16) to assemble dogfood journal entries.
4. Draft CHANGELOG v2.10.0:
   - **Added**: `bin/intent_critic`; `.claude/settings.json` hooks canon; `.git/hooks/pre-commit` critic gate; `.intent_critic.yml` config; `intent/docs/working-with-llms.md`; `intent/docs/pre-commit-hook.md`; `lib/templates/.claude/`; `lib/templates/hooks/`.
   - **Changed**: `intent agents sync` now writes to root `AGENTS.md`; root `CLAUDE.md` template is now Claude-specific overlay; root `usage-rules.md` refreshed to v2.9.0+ surface.
   - **Removed**: `intent/llm/AGENTS.md` (moved to root); `lib/templates/llm/_llm_preamble.md` (deprecated); ST0010 (cancelled); ST0015 (cancelled).
5. Tag and push (confirm with user before force-pushing tag).
6. Mark ST0035 complete.
7. Commit the final set in Intent: reports + journal + CHANGELOG finalisation.

## Acceptance Criteria

- [ ] Feedback report exists with 14 rows, 12 checks each.
- [ ] Dogfood journal has chronological entries spanning the WP-08 to WP-18 execution arc.
- [ ] Decision recorded on whether `intent upgrade` should warn / auto-stage leftover `.intent/`.
- [ ] Zero outstanding critical issues (tickets filed for anything non-critical).
- [ ] All previous WP01-WP16 and WP18 exit checklists verified complete.
- [ ] Commit messages follow Intent conventions, no Claude attribution.

### Post-WP-19 release engineering (separate, not WP-17 acceptance)

- [ ] CHANGELOG v2.10.0 finalised (flip from "in progress" to release date).
- [ ] `git tag v2.10.0` present in Intent.
- [ ] Tag pushed to `local` and `upstream`.
- [ ] GitHub release for v2.10.0 visible.
- [ ] `intent st done ST0035` executed; ST0035 now in COMPLETED.
- [ ] `intent/wip.md` + `intent/restart.md` updated post-release.
- [ ] User's `MEMORY.md` Active Work section updated.

### Tests to add

None.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP-16 (fleet applied), WP-18 (user-doc review closed).
- **Blocks**: nothing structural. WP-19 (per-language canon) is independent and runs after WP-17 closes. ST0035 closure happens post-WP-19 via release engineering.

## Implementation Notes

- **Feedback report as a matrix**: 17 × 12 = 204 cells. Most will be green. Format as a compact markdown table; abbreviate check names in headers.
- **Dogfood journal voice**: first-person or passive voice, whichever feels natural. Focus on _surprises_, not confirmations. "The XY generator needed an extra path-escape step" matters; "canon applied successfully" is not journal-worthy.
- **CHANGELOG format**: follow Intent's existing CHANGELOG.md style. Check previous v2.9.0 entry as template.
- **Tag push confirmation**: per user's past convention (from memory), tag workflow is `git tag -f vX.Y.Z HEAD` then force-push to both remotes. Confirm with user before force-pushing if any ambiguity.
- **Release notes**: assemble CHANGELOG v2.10.0 section as the release notes; `gh release edit` or `gh release create` per user's usual flow.
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
- [ ] CHANGELOG v2.10.0 finalised.
- [ ] Tag pushed to both remotes.
- [ ] GitHub release published.
- [ ] ST0035 in COMPLETED.
- [ ] wip.md + restart.md updated.
- [ ] User's MEMORY.md Active Work updated.
- [ ] Committed.
