---
verblock: "27 Apr 2026:v0.3: matts - scope reconciled to as-built; 8 absorbed into WP-15, 5 user-manual, Pplr out of scope"
wp_id: WP-16
title: "Fleet rollout (Intent ecosystem)"
scope: Large
status: Done
---

# WP-16: Fleet rollout (Intent ecosystem)

> **Coordination note (ST0036/WP-09)**: this fleet rollout carries both ST0035 (LLM canon) and ST0036 (directory relocation) concerns in a single `intent upgrade` per project. Verification is the 12-point checklist defined in WP-15 (10 ST0035 + 2 ST0036). Per-project reports must capture both directory-state outcomes (`intent/.config/` present, `.intent/` absent).

> **Scope as built (2026-04-27)**: WP-16's original 13-project enumeration was overtaken by execution. **8 projects** (Anvil, arca_cli, arca_config, arca_notionex, MicroGPTEx, Molt, Prolix, Utilz) were absorbed into WP-15 canary as scope expanded mid-rollout; per-project reports live at `intent/st/ST0035/WP/15/canary-reports/`. **5 projects** (Multiplyer, MeetZaya, Molt-matts, Courses/Agentic Coding, A3/a3-content) were user-manually upgraded between sessions; verified + cleaned this WP. **1 project** (Pplr) declared out of scope per user (does not need intent). See `fleet-summary.md` for the as-built disposition + per-project verification.

## Objective

Apply ST0035 + ST0036 canon to all in-scope Intent ecosystem projects after a clean canary (WP-15). As built: most projects landed via WP-15 canary; this WP completes the rollout by verifying the user-manual upgrades and cleaning up post-migration drift (legacy `.intent/` directories tracked at HEAD).

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

Explicitly excluded: Sites (handled inside Laksa as subdir), llm-tropes (content-only). (A3/a3-content was originally listed as excluded but reclassified as in-scope by user-manual upgrade -- see fleet-summary.md.)

## Deliverables

1. **Canon applied to all in-scope projects**, one commit each in each project's repo. Pushed to `local` remote per convention.
2. **Per-project verification**: WP-15 canary reports for the 8 absorbed projects; aggregate `fleet-summary.md` for the 5 user-manual ones.
3. **Aggregate fleet summary** at `intent/st/ST0035/WP/16/fleet-summary.md` documenting per-project disposition + verification.
4. **Arca/\* symlink replacement note**: documented in WP-15 canary reports + verified post-upgrade (all three Arca projects show real files).
5. **Multiplyer AGENTS.md creation**: verified post-upgrade.
6. **Rollback playbook**: if any project fails, document the exact `git reset` invocation to restore pre-apply state.

## Approach

### Per-project procedure (same as WP15 canary):

1. `cd <project>` and pull latest from `local` remote.
2. Clean working tree confirmation.
3. Language-specific concerns (e.g., Elixir projects may want a pre-apply `mix deps.get`).
4. `intent claude upgrade --dry-run` (canon preview).
5. `intent upgrade` (drives relocation + stamp bump + canon-apply in a single pass).
6. 12-point verification.
7. Commit + push to `local`.
8. Write report (or document in aggregate `fleet-summary.md`).

### Verification recipe for user-manual upgrades

For projects upgraded by the user outside the formal canary process, the per-project verification is a single bash command (covers all 12 checkpoints in one shot):

```bash
v=$(jq -r '.intent_version' intent/.config/config.json)
[ "$v" = "2.10.0" ] && [ -d intent/.config ] && [ ! -d .intent ] && \
  grep -q 'intent-chain-block:start' .git/hooks/pre-commit && \
  grep -qE '^\.claude/settings\.local\.json' .gitignore && \
  echo "ok"
```

Cleanup if `.intent/` is still present (tracked at HEAD with stale config from pre-upgrade): `git rm -rf .intent/ && git commit && git push local main`.

### If any project fails:

1. `git reset --hard HEAD` in that project.
2. Investigate root cause.
3. Fix in canon (earlier WPs in Intent repo).
4. Self-apply to Intent (re-run WP14 quickly).
5. Re-canary (WP15) if the fix is substantial.
6. Resume fleet from the failed project.

## Acceptance Criteria

- [ ] All in-scope projects have `intent_version: 2.10.0` in `intent/.config/config.json` (post-ST0036 location).
- [ ] All in-scope projects have no leftover `.intent/` directory after upgrade.
- [ ] All in-scope projects pass the 12-point verification checklist (10 ST0035 + 2 ST0036; defined in WP-15).
- [ ] All in-scope projects' changes committed and pushed to `local`.
- [ ] Per-project verification documented (canary reports for the 8 absorbed projects; aggregate `fleet-summary.md` for the 5 user-manual ones).
- [ ] Aggregate fleet summary committed in Intent's repo.
- [ ] Arca/\* root `AGENTS.md` are real files (`! test -L AGENTS.md`); no symlinks.
- [ ] Multiplyer now has a root `AGENTS.md`.
- [ ] Zero rollbacks during the sweep.
- [ ] Sites (Laksa subdir), llm-tropes remain untouched -- confirmed by `git status` in those paths.
- [ ] Commit messages follow Intent conventions, no Claude attribution.

### Tests to add

None.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP15 (canary clean).
- **Blocks**: WP17 (verification sweep).

## Implementation Notes

- **Commit per project, not per batch**: each project's repo gets one clean commit.
- **Push to `local` not `upstream`**: user convention -- Dropbox is `local`. GitHub push can come later.
- **Arca symlink removal**: `rm AGENTS.md` followed by `intent agents sync` is the clean path. The upgrade handles this but verify each Arca project individually.
- **Observing hooks**: SessionStart verification is manual per project. Document "observed on one representative project, expected to generalise" if user is comfortable. Otherwise, run in each -- slow but thorough.
- **Idempotence per project**: re-run `--apply` after each commit; confirm zero diff.
- **User-manual upgrade gotcha**: when the user runs `intent upgrade` outside a formal canary, the pre-existing tracked `.intent/config.json` may be left in the working tree (still tracked at HEAD with stale `intent_version`) because the user's commit only stages new files. Verification step must check `[ ! -d .intent ]`; cleanup is `git rm -rf .intent/`.

## Risks and Edge Cases

- **Risk**: A project has custom tooling that breaks after canon apply. **Mitigation**: `--dry-run` first; abort if surprises; investigate.
- **Risk**: `mix.exs` Elixir projects -- `mix deps.get` needed pre-apply if deps are stale. **Mitigation**: run as a pre-step; not strictly required for canon apply but hygienic.
- **Risk**: Upstream remotes drift during the sweep (other commits land). **Mitigation**: pull latest before applying per project.
- **Edge**: Project has uncommitted work. **Mitigation**: refuse to apply; document; return after user resolves.
- **Edge**: Project has a detached HEAD or unusual branch state. **Mitigation**: normalize before applying; don't proceed on weird state.

## Verification Steps

Per project: 12-point checklist (as in WP15). Aggregate sweep.

## Size and Estimate

- **Size**: L (Large) as planned; **as built**: most work absorbed into WP-15 canary execution. WP-16 itself reduced to one S session of verification + cleanup of the 5 user-manual projects (legacy `.intent/` removal in 3 of them) + writing the aggregate `fleet-summary.md`.

## Exit Checklist

- [ ] All in-scope projects applied and pushed.
- [ ] Fleet summary committed.
- [ ] Zero rollbacks.
- [ ] Excluded projects (Sites, llm-tropes) confirmed untouched.
- [ ] Committed.
