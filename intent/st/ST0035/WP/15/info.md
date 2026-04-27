---
verblock: "27 Apr 2026:v0.3: matts - scope expanded to 11 in-scope canaries; spec tidied to match as-built"
wp_id: WP-15
title: "Canary rollout across in-scope fleet projects"
scope: Medium
status: Done
---

# WP-15: Canary rollout across in-scope fleet projects

> **Coordination note (ST0036/WP-09)**: this canary carries both ST0035 (LLM canon) and ST0036 (directory relocation) concerns. `intent upgrade` invokes `migrate_v2_9_0_to_v2_10_0` which performs the relocation + canon-apply in a single pass. Verification has 12 points (10 ST0035 + 2 ST0036). See `intent/st/ST0036/impl.md` for the bundled-release rationale.

> **Scope as built (2026-04-27)**: the original three-project canary (Conflab, Lamplight, Laksa) expanded mid-execution as the canon-installer matured. Final in-scope set is 11 projects: Laksa, Anvil, Molt, Utilz, arca_cli, arca_config, arca_notionex, Prolix, MicroGPTEx, Conflab, Lamplight. Pplr remains out of scope (does not need intent). See `canary-summary.md` for the as-built decision and per-project outcomes.

## Objective

Apply ST0035 + ST0036 canon to the in-scope fleet projects in canary mode (one-at-a-time with verification) before the bulk fleet rollout (WP16). Mix of Elixir, Bash, polyglot, and small-tooling projects exercises every canon artefact; clean passes mean the fleet sweep (WP16) is low-risk.

## Context

Canary discipline (from ST0034's rollout pattern): apply to a representative subset, verify, then sweep. The 11 in-scope projects span:

- **Elixir full-stack**: Laksa, Anvil, Conflab, Lamplight (full `intent/llm/`, deps `usage-rules.md`, polyglot test suites).
- **Bash/CLI tooling**: Utilz, arca_cli, arca_config, arca_notionex (smaller, foreign pre-commit hooks predominant).
- **Mixed/Other**: Molt (Elixir), Prolix (Elixir), MicroGPTEx (Elixir).

If any canary reveals an issue, fix it in the canon (earlier WPs), re-apply to Intent (WP14), re-canary. No fleet sweep until canary passes clean across the in-scope set. Issues surfaced during canary fed back into the canon installer mid-rollout: `MIGRATE_LEGACY_PRE_COMMIT` (single-file pre-commit migration), `CHAIN_PRE_COMMIT` (auto-insert markered chain block), `NORMALIZE_GITIGNORE` (uniform `.claude/settings.local.json` + `/AGENTS.md.bak` entries). All baked back into Intent before later canaries ran.

## Deliverables

1. **Canon applied to all in-scope canary projects** with a coherent commit each:
   - `cd <project> && intent upgrade` (drives relocation + canon-apply in a single pass via `migrate_v2_9_0_to_v2_10_0`).
   - `intent claude upgrade --apply` only as needed for incremental refreshes (chain-block inserts, gitignore normalisation) on projects already at v2.10.0.
   - Push each commit to `local` (Dropbox) only -- not `upstream` -- per canary protocol.
2. **Per-project verification report** in `intent/st/ST0035/WP/15/canary-reports/<project>.md` with:
   - Dry-run output.
   - Apply output.
   - 12-point checklist results.
   - Any issues encountered.
   - Outcome: pass / fix-required / blocked.
3. **Aggregate canary report** at `intent/st/ST0035/WP/15/canary-summary.md` with findings across all in-scope projects.
4. **Issue tickets** filed for any bugs discovered in the canon (may not be any).
5. **Decision to proceed / halt** on fleet sweep (WP16) — clearly documented.

## Approach

### For each in-scope canary project:

1. Pull latest from remote (`git pull`).
2. Ensure clean working tree (or carefully isolate canon files from active WIP).
3. `intent doctor` -- baseline clean.
4. `intent claude upgrade --dry-run` -- view canon plan.
5. `intent upgrade` -- run the migration chain (relocation + stamp bump + canon-apply in a single pass via `migrate_v2_9_0_to_v2_10_0`). For projects already at v2.10.0 layout, run `intent claude upgrade --apply` to pick up incremental refinements (chain-block, gitignore normalisation).
6. Review `git diff` / `git status`.
7. Run 12-point verification:
   - config.json at 2.10.0 (in `intent/.config/config.json`, post-ST0036 location).
   - Root AGENTS.md real file, not symlink.
   - intent/llm/AGENTS.md absent.
   - Root usage-rules.md present.
   - .claude/settings.json hooks present.
   - .git/hooks/pre-commit executable + chain block markers present.
   - .intent_critic.yml present.
   - SessionStart reminder observed in a Claude Code session.
   - Pre-commit blocks on staged violation.
   - `intent critic <lang>` produces report.
   - **ST0036 (11)**: `[ -d intent/.config ]` -- new layout present.
   - **ST0036 (12)**: `[ ! -d .intent ]` -- legacy directory absent (no leftover).
8. Project-specific checks where relevant:
   - **Conflab**: pre-existing `.claude/skills/` installs survive the upgrade.
   - **Lamplight**: `mix usage_rules.sync` (if run) still produces correct output and does not conflict with Intent's refreshed root `usage-rules.md`.
9. Commit: `chore: apply ST0035 + ST0036 canon (v2.10.0 rollout canary)` (or similar).
10. Push to `local` remote (Dropbox) as per project convention.
11. Document in canary report at `intent/st/ST0035/WP/15/canary-reports/<project>.md`.
12. Return to Intent repo and update summary.

### Cross-project

- If issue found in any canary, pause. Investigate. Fix canon in Intent. Re-canary affected projects.
- If all in-scope canaries pass clean, proceed to WP16.

## Acceptance Criteria

- [ ] All in-scope projects have `intent_version: 2.10.0` in `intent/.config/config.json` (post-ST0036 location).
- [ ] All in-scope projects have no leftover `.intent/` directory after upgrade.
- [ ] All in-scope projects pass the 12-point verification checklist (10 from ST0035 + 2 from ST0036).
- [ ] All in-scope projects have their changes committed and pushed to `local`.
- [ ] Per-project reports written to `intent/st/ST0035/WP/15/canary-reports/<project>.md`.
- [ ] Aggregate summary at `intent/st/ST0035/WP/15/canary-summary.md`.
- [ ] Zero rollbacks during canary.
- [ ] Conflab's pre-existing `.claude/skills/` installs survive the upgrade (verified by listing before and after).
- [ ] Lamplight's `mix usage_rules.sync` (if run) produces a sane AGENTS.md with both Intent's and deps' rules visible.
- [ ] Any issues discovered are either (a) fixed in the canon before WP15 closes, or (b) filed as follow-up tickets with explicit approval to proceed.
- [ ] Commit messages follow Intent conventions, no Claude attribution.

### Tests to add

None.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP14 (Intent self-apply must be clean).
- **Blocks**: WP16 (fleet).

## Implementation Notes

- **Commit discipline per project**: each canary project gets its own commit in its own repo. Don't batch.
- **Push to `local` remote**: per user's convention (Dropbox remote is `local`). Don't push to `upstream` (GitHub) from the canary — that's deferred to the fleet sweep post-verification.
- **Observation over measurement**: some canary checks (SessionStart reminder injection) require human observation. Document as "observed" with a brief note.
- **Conflab skills preservation**: list `.claude/skills/` before, after. Diff. Should be identical set of installed skills.
- **Lamplight mix.exs interaction**: `mix usage_rules.sync` is an Elixir tool that reads `deps/*/usage-rules.md` and generates content. Intent's root `usage-rules.md` is hand-authored. They coexist — don't conflict. Verify by running both tools and inspecting output.

## Risks and Edge Cases

- **Risk**: Canon apply breaks one of the canary projects. **Mitigation**: git reset available; no pushes to upstream until verified.
- **Risk**: Conflab's installed skills are wiped by the upgrade. **Mitigation**: WP11 should preserve `.claude/skills/` contents; test explicitly.
- **Risk**: Lamplight's `mix usage_rules.sync` emits conflicting AGENTS.md. **Mitigation**: Intent's root AGENTS.md is written by `intent agents sync`; `mix usage_rules.sync` writes to a separate path by default. Confirm they don't collide.
- **Edge**: Canary project has uncommitted work. **Mitigation**: isolate canon files from WIP (precise `git add`); commit only the canon deltas.

## Verification Steps

See acceptance criteria. Each project produces its own report; the aggregate summary rolls them up.

## Size and Estimate

- **Size**: M (Medium). As built: 4 sessions across the 11 in-scope projects (canon-installer matured mid-rollout, surfacing the LEGACY single-file migration + chain-block + NORMALIZE_GITIGNORE refinements that fed back into the canon).

## Exit Checklist

- [ ] All in-scope canary reports complete.
- [ ] Aggregate summary documents outcomes.
- [ ] Zero unresolved issues (or filed as follow-up with explicit approval).
- [ ] Decision to proceed to WP16 explicitly noted.
- [ ] All in-scope project commits pushed to `local`.
