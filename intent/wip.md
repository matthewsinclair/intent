---
verblock: "23 Apr 2026:v0.35: matts - ST0034 WP07 next"
intent_version: 2.8.2
---

# Work In Progress

## Current State

Intent v2.8.2 (VERSION unchanged; release bump is WP11). **ST0034 active — 8/12 WPs done.** WP07 (Critic subagent family) is the next WP and the main join point for the rest of the ST. 48 rules validator-clean. 633/633 BATS green. Tree clean.

## Recent

- **2026-04-22**: WP12 (shell rule pack + `critic-shell` subagent) + WP06 (Rust/Swift/Lua rule packs) + WP08 (worker-bee extracted to `lib/templates/ext-seeds/` with `git mv` so history is preserved) + `/in-session` bootstrap skill shipped. Commits `44e05d1`, `c17d03b`, `65f3cea`, `d2edb59`.
- **2026-04-15**: v2.8.2 released (tagged twice; second commit `84a3a5f` slipstreamed a fix to `bin/intent_upgrade`). ST0033 (cwd-resilient dispatch) + ST0032 (Credo checks) + upgrade-chain gap fix (`migrate_v2_6_0_to_v2_8_0`). Fleet of 16 projects brought to 2.8.2.
- **2026-04-09**: v2.8.1 released -- ST0031 (TCA suite hardening).
- **2026-04-06**: Agentic Coding Course migrated to `../Courses/Agentic Coding/`.

## Active Steel Threads

- **ST0034 (Agentic Software Engineering Suite)** — v2.9.0 target. 12 WPs. 8 done: WP01 schema, WP02 ext system, WP03 skill/subagent rationalisation (elixir subagent deleted), WP04 agnostic pack, WP05 Elixir pack, WP06 Rust/Swift/Lua packs, WP08 worker-bee extraction, WP12 shell pack + critic-shell. Remaining: **WP07 critic family (elixir/rust/swift/lua)**, WP09 migration chain, WP10 docs, WP11 release + fleet upgrade. Critical path: WP07 → WP09 → WP10 → WP11. See `intent/st/ST0034/impl.md` for the full progress tracker.

## Next Up

- **WP07 (Large)**: 4 critic subagents (`critic-elixir/rust/swift/lua`), cloned from the `critic-shell` template. Each is a thin orchestrator that reads agnostic + its own language pack, applies Detection heuristics, emits severity-grouped reports. Includes `in-review` stage-2 dispatcher update, `.intent_critic.yml` config plumbing, `intent/docs/critics.md` expansion from WP01 draft, and 16 fixture directories (4 langs × 2 modes × 2 catch/miss). Estimated 3-5 focused hours — author `critic-elixir` first, then clone with language substitutions.

## Deferred observations

- **TCA skills retrospection**: once the `critic-<lang>` family lands in WP07, the `in-tca-*` suite is largely subsumed (TCA becomes "run the critics and synthesize findings"). Tomorrow's problem — either rewrite against the critic contract or retire. Noted so it does not get lost.
- **WP12 dogfood journal Entries 1-3**: deferred post-release. `critic-shell` is Claude-side, invoked via `Task()` from a session, not via `bash`; its first real invocation belongs to a future session. Journal Entry 0 documents the authorship-pass substitute.
- **Blog draft `docs/blog-drafts/shell-critic-inception.md`**: publication gated on real dogfood Entries 1-3.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
