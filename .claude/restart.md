# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** The `Release v2.9.0` commit should be the most recent. If `git status` shows uncommitted work, investigate before proceeding.
3. **Read `intent/restart.md`** for the post-release state summary.
4. **Resume on WP11 fleet rollout** (sessions 2-3). See "Next up" below.

## State

**Intent v2.9.0 released 2026-04-23.** ST0034 (Agentic Software Engineering Suite) complete; all 12 WPs done. `VERSION` 2.9.0; tag `v2.9.0` on both remotes; GitHub release published from `docs/releases/2.9.0/RELEASE_NOTES.md`; CHANGELOG dated; worker-bee seed `intent_compat.min` bumped to 2.9.0 in lockstep. Pre-release gate green: bats suite ok; `intent claude rules validate` 48/48 ok; `intent doctor` clean; ext_seed_validity 18/18 ok. Fleet rollout (16 projects) pending in WP11 sessions 2-3.

## Next up

1. **WP11 fleet rollout (sessions 2-3)**:
   - **Canary batch (serial, halt-on-error)**: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab.
   - **Batch 2 (parallel within batch)**: Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts.
   - **Batch 3 (parallel within batch)**: Multiplyer, Prolix, Utilz, Courses/Agentic Coding.
   - **A3/\* skipped** per existing direction.
   - **Per-project verification**: `cat .intent/config.json | jq .intent_version` → "2.9.0"; `intent doctor` clean; `ls ~/.claude/agents/elixir.md ~/.claude/agents/worker-bee.md` → both absent; `ls ~/.intent/ext/worker-bee/` → seeded.
   - **Halt discipline**: any canary failure → diagnose, fix in Intent repo (may require v2.9.1), re-run canary end-to-end before batches 2-3.
   - **Post-fleet**: update `intent/wip.md` and `.claude/restart.md` with rollout confirmation; commit `Session finish: v2.9.0 fleet rollout complete`; push.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.

## Deferred

- WP12 dogfood journal Entries 1-3: post-release.
- `docs/blog-drafts/shell-critic-inception.md`: publication gated on real dogfood runs.
- WP07 follow-ups: align Diogenes fixture-context handling across the four critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`
