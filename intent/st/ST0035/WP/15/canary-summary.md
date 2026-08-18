---
verblock: "27 Apr 2026:v0.1: matts - WP-15 canary aggregate summary"
wp: ST0035/WP-15
status: complete
date: 2026-04-27
in_scope_count: 11
passed: 11
failed: 0
deferred: 0
out_of_scope: 1
---

# WP-15 canary summary

## Outcome

**11 of 11 in-scope canaries pass.** Zero rollbacks. Zero blocked. Pplr remains out of scope (does not need intent). Decision: **proceed to WP-16 (fleet rollout)**.

## In-scope projects

| #   | Project       | Date       | Commit (project repo) | Pre-flight notes                                               | Outcome |
| --- | ------------- | ---------- | --------------------- | -------------------------------------------------------------- | ------- |
| 01  | Laksa         | 2026-04-27 | a729ec64              | 2.8.2 to 2.10.0 chain in one pass                              | pass    |
| 02  | Anvil         | 2026-04-27 | 39c63bd               | First LEGACY single-file pre-commit migration; lazy_html flyby | pass    |
| 03  | Molt          | 2026-04-27 | 7abd972               | Stale 2.9.1 stamp reset; foreign pre-commit chained            | pass    |
| 04  | Utilz         | 2026-04-27 | ed31017               | 2.8.2 to 2.10.0 chain                                          | pass    |
| 05  | arca_cli      | 2026-04-27 | 2e7c14f               | Stale 2.9.1 stamp reset                                        | pass    |
| 06  | arca_config   | 2026-04-27 | ca85f26               | Stale 2.9.1 stamp reset                                        | pass    |
| 07  | arca_notionex | 2026-04-27 | 9de67e9               | Was gitignoring entire .claude/                                | pass    |
| 08  | Prolix        | 2026-04-27 | 4508e94               | Stale 2.9.1 stamp reset                                        | pass    |
| 09  | MicroGPTEx    | 2026-04-27 | b375d1f               | Was gitignoring entire .claude/                                | pass    |
| 10  | Conflab       | 2026-04-27 | 973dae62              | Predated chain-block + NORMALIZE_GITIGNORE; isolated from WIP  | pass    |
| 11  | Lamplight     | 2026-04-27 | 1b0b3bbe              | Predated chain-block + NORMALIZE_GITIGNORE; isolated from WIP  | pass    |

Per-project verification reports: `intent/st/ST0035/WP/15/canary-reports/<project>.md`.

## Out of scope

| Project | Reason                                                          |
| ------- | --------------------------------------------------------------- |
| Pplr    | Does not need intent (out of scope per user; no migration path) |

## Scope expansion (as built)

The original WP-15 plan named three canaries (Conflab, Lamplight, Laksa). Mid-execution the scope expanded to 11 projects because:

1. **Confidence accumulated quickly** -- after Laksa + Anvil passed clean, batching the remaining smaller projects (Molt, Utilz, arca\_\*, Prolix, MicroGPTEx) was lower friction than deferring them to WP-16.
2. **Conflab + Lamplight were busy** -- both had substantial active WIP throughout the canary window. Deferred until last; isolated commits applied only to canon files (precise `git add`).
3. **Canon installer matured mid-rollout** -- new actions kept landing as canaries surfaced rough edges; running them through canary mode (one-at-a-time with verification) was cheap and caught regressions before WP-16.

## Canon-installer refinements driven by canary findings

Three new actions baked back into Intent during WP-15 execution:

| Action                         | Trigger / discovery                                                                | Intent commit |
| ------------------------------ | ---------------------------------------------------------------------------------- | ------------- |
| `MIGRATE_LEGACY_PRE_COMMIT`    | Anvil canary: canon body sat at `pre-commit` with no `pre-commit.intent` file      | d5b9203       |
| `CHAIN_PRE_COMMIT` auto-insert | WP14 self-dogfood: marker chain block needed manual paste; auto-insert via markers | 9315bb6       |
| `NORMALIZE_GITIGNORE`          | Three canaries (Utilz, arca_notionex, MicroGPTEx) gitignored entire `.claude/`     | ac4fd9f       |

Each action came with new BATS scenarios; test count grew 785 to 791 across the canary window. All in-scope canaries re-verified `OK` on `.gitignore` after NORMALIZE_GITIGNORE landed.

## Verification matrix (12 points)

All 11 in-scope projects pass all 12 points (10 ST0035 + 2 ST0036). Highlights:

- **Pre-commit chain block** is load-bearing: prior to the chain block, `pre-commit.intent` existed but `pre-commit` never invoked it -- the critic gate was dormant. Canaries 10 + 11 (Conflab + Lamplight) needed this fix despite being already at v2.10.0 layout.
- **`session-context.sh`** drifted across older v2.10.0 installs; refreshed via `INSTALL_HOOK_SCRIPT:session-context.sh` action.
- **CLAUDE.md user section** preserved between markers across all canaries; only date stamps refreshed.

## Issues discovered + filed for follow-up

None left unresolved. All canon-installer issues found during canary were fixed in Intent before WP-15 closed (see commits in table above).

Out-of-scope observations (deferred to other WPs):

- **CLAUDE.md content drift**: pre-existing user CLAUDE.md (from STP-era) is preserved by canon (correct behaviour). A separate session can refresh by `intent claude upgrade --force` or hand-editing against `lib/templates/llm/_CLAUDE.md`. Tracked for WP-17 dogfood journal.
- **Three projects had broad `.claude/` ignore** (Utilz, arca_notionex, MicroGPTEx) -- fixed and now uniform fleet-wide. Canon-installer enforces uniformity via NORMALIZE_GITIGNORE.
- **Foreign pre-commit hooks predominant** in the bash/CLI projects (arca\_\*, Utilz, MicroGPTEx, Prolix). All chained via marker block; no project lost its existing pre-commit logic.

## Decision

**Proceed to WP-16 (fleet rollout).** Canon recipe is mature; canon-installer handles the four common pre-flight patterns (fresh install, stale stamp reset, LEGACY single-file pre-commit, gitignore normalisation) automatically. WP-16 should be straightforward bulk application across any remaining fleet projects not covered here.
