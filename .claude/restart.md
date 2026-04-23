# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** The `Session finish: v2.9.0 wrap-up + fleet rollout complete` commit (or whatever followed) should be the most recent. If `git status` shows uncommitted work, investigate.
3. **Read `intent/restart.md`** for the post-release state summary.
4. **Open queue is empty** — pick from Parked (ST0010, ST0015) or initiate a new ST when work surfaces.

## State

**Intent v2.9.0 released 2026-04-23 — fleet rollout complete.** ST0034 closed (all 12 WPs done). Release commit `d1b0fe1`; tag `v2.9.0` on both remotes; GitHub release published. **13/13 active projects upgraded to 2.9.0**: canary (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex), batch 2 (Laksa, MeetZaya, MicroGPTEx, Molt, Molt-matts), batch 3 (Multiplyer, Prolix, Utilz, Courses/Agentic Coding). Conflab + Lamplight + A3/\* skipped per direction. Zero rollbacks. CI workflow retry-fixed in same session.

## Next up

Open queue is empty. Pick from Parked (ST0010, ST0015) or initiate a new ST when work surfaces.

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
