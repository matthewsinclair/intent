---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-16
title: "Fleet rollout to remaining 13 projects"
scope: Large
status: Not Started
---

# WP-16: Fleet rollout to remaining 13 projects

## Objective

Apply ST0035 canon to the remaining 13 Intent-using projects after a clean canary (WP15): 12 existing Intent projects + Pplr (via `intent init` to bring it into the fleet). Zero rollbacks target. Per-project report for each.

## Context

After canary (WP15) passes clean, the rollout is mechanical: `intent upgrade && intent claude upgrade --apply` per project, commit, push, verify. WP16 executes the sweep and catches any project-specific quirks.

Projects in scope (13):

1. Anvil — Elixir, no DECISION_TREE — planted by upgrade.
2. Arca/arca_cli — Elixir, AGENTS.md is symlink — replaced with real file.
3. Arca/arca_config — Elixir, AGENTS.md is symlink — replaced.
4. Arca/arca_notionex — Elixir, AGENTS.md is symlink — replaced.
5. Courses/Agentic Coding — bash.
6. MeetZaya — Elixir, no DECISION_TREE — planted.
7. MicroGPTEx — Elixir, no DECISION_TREE — planted.
8. Molt — docs.
9. Molt-matts — bash, AGENTS.md is symlink — replaced.
10. Multiplyer — Elixir, **missing AGENTS.md entirely** — created.
11. Prolix — Elixir, no DECISION_TREE — planted.
12. Utilz — docs.
13. **Pplr** — non-Intent today; `intent init` first, then canon apply.

Explicitly excluded (per ST0035 scope): Sites (handled inside Laksa as subdir), llm-tropes (content-only), A3/\* (content-only).

## Deliverables

1. **Canon applied to all 13 projects**, one commit each in each project's repo. Pushed to `local` remote per convention.
2. **Per-project reports** at `intent/st/ST0035/WP/16/fleet-reports/<project>.md`. Same format as WP15 canary reports.
3. **Aggregate fleet summary** at `intent/st/ST0035/WP/16/fleet-summary.md` listing 10-point verification outcomes for all 13.
4. **Pplr-specific handling**: `intent init` bootstrap documented; user's pre-existing `usage-rules.md` preserved / merged.
5. **Arca/\* symlink replacement note**: document the symlink-to-realfile migration in each Arca project's report — it's a distinct case that needs visible confirmation.
6. **Multiplyer AGENTS.md creation**: confirmed via dry-run and apply.
7. **Rollback playbook**: if any project fails, document the exact `git reset` invocation to restore pre-apply state.

## Approach

### Per-project procedure (same as WP15 canary):

1. `cd <project>` and pull latest from `local` remote.
2. Clean working tree confirmation.
3. Language-specific concerns (e.g., Elixir projects may want a pre-apply `mix deps.get`).
4. `intent upgrade --dry-run` (stamp bump preview).
5. `intent claude upgrade --dry-run` (canon preview).
6. Apply both.
7. 10-point verification.
8. Commit.
9. Push to `local`.
10. Write report.

### Pplr handling (special case):

1. Pplr is bash, non-Intent today. Has CLAUDE.md and usage-rules.md but no `.intent/`.
2. `cd ~/Devel/prj/Pplr && intent init` to scaffold `.intent/config.json` + `intent/` directory structure.
3. After init, `intent upgrade` is not needed (init starts at current version). Then `intent claude upgrade --apply` for canon.
4. Verify pre-existing `usage-rules.md` is preserved (merged if user markers exist; otherwise backed up as `.pre-intent.bak`).
5. Verify pre-existing `CLAUDE.md` is preserved (same mechanism).

### Order of operations:

Execute in this order to catch related issues early:

1. Multiplyer (missing AGENTS.md — highest-delta project).
2. Arca trio (symlink replacements — three similar projects; do back-to-back).
3. Anvil, MeetZaya, MicroGPTEx, Prolix (missing DECISION_TREE — four similar plants).
4. Molt, Molt-matts, Utilz (docs/bash — simpler deltas).
5. Courses/Agentic Coding (bash, course content).
6. Pplr last (non-Intent bootstrap).

### If any project fails:

1. `git reset --hard HEAD` in that project.
2. Investigate root cause.
3. Fix in canon (earlier WPs in Intent repo).
4. Self-apply to Intent (re-run WP14 quickly).
5. Re-canary (WP15) if the fix is substantial.
6. Resume fleet from the failed project.

## Acceptance Criteria

- [ ] All 13 projects have `intent_version: 2.9.1` in `.intent/config.json`.
- [ ] All 13 projects pass the 10-point verification checklist.
- [ ] All 13 projects' changes committed and pushed to `local`.
- [ ] Per-project reports written.
- [ ] Aggregate fleet summary committed in Intent's repo.
- [ ] Pplr is now an Intent project (.intent/config.json present) with canon applied.
- [ ] Arca/\* root `AGENTS.md` are real files (`! test -L AGENTS.md`); no symlinks.
- [ ] Multiplyer now has a root `AGENTS.md`.
- [ ] Zero rollbacks during the sweep.
- [ ] Sites (Laksa subdir), llm-tropes, A3/\* remain untouched — confirmed by `git status` in those paths.
- [ ] Commit messages follow Intent conventions, no Claude attribution.

### Tests to add

None.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP15 (canary clean).
- **Blocks**: WP17 (verification sweep).

## Implementation Notes

- **Batch ordering matters**: start with highest-delta projects (Multiplyer, Arca/\*) to surface issues early.
- **Commit per project, not per batch**: each project's repo gets one clean commit.
- **Push to `local` not `upstream`**: user convention — Dropbox is `local`. GitHub push can come later.
- **Pplr `intent init` quirks**: Pplr is non-Intent today. The init may prompt for defaults; have a script ready that passes expected answers, or run interactively and document.
- **Arca symlink removal**: `rm AGENTS.md` followed by `intent agents sync` is the clean path. The upgrade handles this but verify each Arca project individually.
- **Observing hooks**: SessionStart verification is manual per project. Document "observed on one representative project, expected to generalise" if user is comfortable. Otherwise, run in each — slow but thorough.
- **Idempotence per project**: re-run `--apply` after each commit; confirm zero diff.

## Risks and Edge Cases

- **Risk**: A project has custom tooling that breaks after canon apply. **Mitigation**: `--dry-run` first; abort if surprises; investigate.
- **Risk**: Pplr's pre-existing `usage-rules.md` conflicts with Intent's template. **Mitigation**: preservation markers; fail-safe to back up pre-existing content to `.pre-intent.bak`.
- **Risk**: `mix.exs` Elixir projects — `mix deps.get` needed pre-apply if deps are stale. **Mitigation**: run as a pre-step; not strictly required for canon apply but hygienic.
- **Risk**: Upstream remotes drift during the sweep (other commits land). **Mitigation**: pull latest before applying per project.
- **Edge**: Project has uncommitted work. **Mitigation**: refuse to apply; document; return after user resolves.
- **Edge**: Project has a detached HEAD or unusual branch state. **Mitigation**: normalize before applying; don't proceed on weird state.

## Verification Steps

Per project: 10-point checklist (as in WP15). Aggregate sweep.

## Size and Estimate

- **Size**: L (Large). 3–5 sessions.
- Session 1: Multiplyer + Arca trio.
- Session 2: Anvil + MeetZaya + MicroGPTEx + Prolix.
- Session 3: Molt + Molt-matts + Utilz.
- Session 4: Courses/Agentic Coding + Pplr (bootstrap + apply).
- Session 5: Aggregate summary + verify excluded projects untouched.

## Exit Checklist

- [ ] All 13 projects applied and pushed.
- [ ] Fleet summary committed.
- [ ] Zero rollbacks.
- [ ] Pplr is now Intent-managed.
- [ ] Excluded projects (Sites, llm-tropes, A3/\*) confirmed untouched.
- [ ] Committed.
