---
verblock: "23 Apr 2026:v0.36: matts - ST0034 WP07 done; WP09 next"
intent_version: 2.8.2
---

# Work In Progress

## Current State

Intent v2.8.2 (VERSION unchanged; release bump is WP11). **ST0034 active — 9/12 WPs done.** WP07 (Critic subagent family) closed: critic-{elixir,rust,swift,lua} authored and verified, critic-shell retrofitted to the family report format, in-review stage-2 dispatcher wired for all five critics, intent/docs/critics.md is the contract reference. WP09 (migration chain) is next.

## Recent

- **2026-04-23**: WP07 (critic subagent family) closed in a single cohesive commit. Four critics + 16-row verification matrix green; critic-shell retrofitted; in-review stage-2 dispatcher; `.intent_critic.yml` schema + sample; intent/docs/critics.md; three new BATS suites (critic_dispatch, critic_report_format, critic_config).
- **2026-04-22**: WP12 (shell rule pack + `critic-shell` subagent) + WP06 (Rust/Swift/Lua rule packs) + WP08 (worker-bee extracted to `lib/templates/ext-seeds/` with `git mv` so history is preserved) + `/in-session` bootstrap skill shipped. Commits `44e05d1`, `c17d03b`, `65f3cea`, `d2edb59`.
- **2026-04-15**: v2.8.2 released (tagged twice; second commit `84a3a5f` slipstreamed a fix to `bin/intent_upgrade`). ST0033 (cwd-resilient dispatch) + ST0032 (Credo checks) + upgrade-chain gap fix (`migrate_v2_6_0_to_v2_8_0`). Fleet of 16 projects brought to 2.8.2.
- **2026-04-09**: v2.8.1 released -- ST0031 (TCA suite hardening).
- **2026-04-06**: Agentic Coding Course migrated to `../Courses/Agentic Coding/`.

## Active Steel Threads

- **ST0034 (Agentic Software Engineering Suite)** — v2.9.0 target. 12 WPs. 9 done: WP01 schema, WP02 ext system, WP03 skill/subagent rationalisation (elixir subagent deleted), WP04 agnostic pack, WP05 Elixir pack, WP06 Rust/Swift/Lua packs, WP07 critic family, WP08 worker-bee extraction, WP12 shell pack + critic-shell. Remaining: **WP09 migration chain**, WP10 docs, WP11 release + fleet upgrade. Critical path: WP09 → WP10 → WP11. See `intent/st/ST0034/impl.md` for the full progress tracker.

## Next Up

- **WP09 (Medium)**: migration chain. Bridges from v2.8.x project layouts to v2.9.0 (extension system + critic family). `bin/intent_upgrade` migration step `migrate_v2_8_x_to_v2_9_0` covering: prune `~/.claude/agents/elixir.md` and `~/.claude/agents/worker-bee.md` (both already deleted from canon), rebuild critic-{shell,elixir,rust,swift,lua} from canon, install `~/.intent/ext/` scaffold, install worker-bee from `lib/templates/ext-seeds/worker-bee/` if user opts in, sanity-check `.intent_critic.yml` placement. BATS coverage for the migration step. No fleet-wide rollout yet (that's WP11).

## Deferred observations

- **TCA skills retrospection**: once the `critic-<lang>` family lands in WP07, the `in-tca-*` suite is largely subsumed (TCA becomes "run the critics and synthesize findings"). Tomorrow's problem — either rewrite against the critic contract or retire. Noted so it does not get lost.
- **WP12 dogfood journal Entries 1-3**: deferred post-release. `critic-shell` is Claude-side, invoked via `Task()` from a session, not via `bash`; its first real invocation belongs to a future session. Journal Entry 0 documents the authorship-pass substitute.
- **Blog draft `docs/blog-drafts/shell-critic-inception.md`**: publication gated on real dogfood Entries 1-3.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
