# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Most recent commit should be `983ccbf` (stp/ prompt fix) or whatever followed. If `git status` shows uncommitted work, investigate.
3. **Read `intent/restart.md`** for the post-release state summary.
4. **Run `critic-shell` on Intent's own bash code** — see Next-up below.

## State

**Intent v2.9.0 released 2026-04-23 — fleet rollout complete.** ST0034 closed (all 12 WPs done). Release commit `d1b0fe1`; tag `v2.9.0` on both remotes; GitHub release published. **13/13 active projects upgraded to 2.9.0**: canary (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex), batch 2 (Laksa, MeetZaya, MicroGPTEx, Molt, Molt-matts), batch 3 (Multiplyer, Prolix, Utilz, Courses/Agentic Coding). Conflab + Lamplight + A3/\* skipped per direction. Zero rollbacks. CI workflow retry-fixed in same session.

## Next up

1. **Run `critic-shell` on Intent's bash codebase** — first real-world dogfood of the new critic against Intent's own code. Satisfies WP12 dogfood journal Entry 1 simultaneously.
   - **Targets** (in scope): `bin/intent`, `bin/intent_main`, `bin/intent_helpers`, `bin/intent_st`, `bin/intent_wp`, `bin/intent_upgrade`, `bin/intent_init`, `bin/intent_doctor`, `bin/intent_bootstrap`, `bin/intent_config`, `bin/intent_help`, `bin/intent_ext`, `bin/intent_treeindex`, `bin/intent_fileindex`, `bin/intent_audit`, `bin/intent_learn`, `bin/intent_modules`, `bin/intent_organize`, `bin/intent_organise`, `bin/intent_minimal`, `bin/intent_info`, `bin/intent_plugin`, `intent/plugins/agents/bin/intent_agents`, `intent/plugins/claude/bin/intent_claude_*`, `tests/run_tests.sh`.
   - **Targets** (out of scope, this pass): `tests/setup_*.sh`, `tests/*.bats` themselves (test files are bats, not strict bash; critic-shell rules target production bash).
   - **Dispatch**: `Task(subagent_type="critic-shell", prompt="review <space-separated paths>")`. Consume the severity-grouped report (CRITICAL / WARNING / RECOMMENDATION / STYLE).
   - **Triage**: classify findings into P0/P1/P2a/P2b/P3 tiers per `/in-tca-synthesize` mapping. Decide what to fix now vs queue as a follow-up ST. Genuine false positives → consider lifting into `.intent_critic.yml` `disabled:` (project-wide carve-out).
   - **Output**: at minimum, append a Dogfood Journal Entry 1 to `intent/st/COMPLETED/ST0034/WP/12/dogfood-journal.md` with: rule findings counts, top P0/P1 hits, decisions taken, FP rate, time-to-first-finding.

2. Open queue otherwise empty. Pick from Parked (ST0010, ST0015) or initiate a new ST when work surfaces.

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
