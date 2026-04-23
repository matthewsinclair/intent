---
verblock: "23 Apr 2026:v0.37: matts - ST0034 WP09 done; WP10 next"
intent_version: 2.8.2
---

# Work In Progress

## Current State

Intent v2.8.2 (VERSION unchanged; release bump is WP11). **ST0034 active — 10/12 WPs done.** WP09 (Migration and upgrade chain) closed: `migrate_v2_8_2_to_v2_9_0` + `needs_v2_9_0_upgrade` + `generate_ext_readme` shipped in `bin/intent_helpers`; `bin/intent_upgrade` chain wired across every prior version; ext_migration.bats with 28 tests. Canary dry-run gated on user, deferred until pre-WP11. WP10 (Documentation) is next.

## Recent

- **2026-04-23**: WP09 (migration chain) closed. v2.8.2 → v2.9.0 migration step authored; chain extended in `bin/intent_upgrade` (gate check + 16 chain-tails + new `"2.8.2"` case); 28 BATS tests in `ext_migration.bats`; full suite 696 ok.
- **2026-04-23**: WP07 (critic subagent family) closed in a single cohesive commit (`398de76`). Four critics + 16-row verification matrix green; critic-shell retrofitted; in-review stage-2 dispatcher; `.intent_critic.yml` schema + sample; intent/docs/critics.md; three new BATS suites (critic_dispatch, critic_report_format, critic_config).
- **2026-04-22**: WP12 (shell rule pack + `critic-shell` subagent) + WP06 (Rust/Swift/Lua rule packs) + WP08 (worker-bee extracted to `lib/templates/ext-seeds/` with `git mv` so history is preserved) + `/in-session` bootstrap skill shipped. Commits `44e05d1`, `c17d03b`, `65f3cea`, `d2edb59`.
- **2026-04-15**: v2.8.2 released (tagged twice; second commit `84a3a5f` slipstreamed a fix to `bin/intent_upgrade`). ST0033 (cwd-resilient dispatch) + ST0032 (Credo checks) + upgrade-chain gap fix (`migrate_v2_6_0_to_v2_8_0`). Fleet of 16 projects brought to 2.8.2.
- **2026-04-09**: v2.8.1 released -- ST0031 (TCA suite hardening).

## Active Steel Threads

- **ST0034 (Agentic Software Engineering Suite)** — v2.9.0 target. 12 WPs. 10 done: WP01 schema, WP02 ext system, WP03 skill/subagent rationalisation (elixir subagent deleted), WP04 agnostic pack, WP05 Elixir pack, WP06 Rust/Swift/Lua packs, WP07 critic family, WP08 worker-bee extraction, WP09 migration chain, WP12 shell pack + critic-shell. Remaining: **WP10 documentation**, WP11 release + fleet upgrade. Critical path: WP10 → WP11. See `intent/st/ST0034/impl.md` for the full progress tracker.

## Next Up

- **WP10 (Medium)**: documentation pass. Authoritative coverage for the new surfaces: `intent/docs/extensions.md` (writing extensions, `~/.intent/ext/` layout, manifest schema), `intent/docs/rules.md` (rule library structure, agnostic→language concretisation, authoring a rule), updates to `intent/docs/critics.md` from WP07 (cross-link Diogenes/Socrates handoffs and add the registration-freeze operational note surfaced in WP07 follow-ups), Migration Notes for v2.8.2 → v2.9.0, refresh `intent/llm/MODULES.md` + `intent/llm/DECISION_TREE.md` for the new surfaces, BATS `docs_completeness.bats` (already a placeholder row in MODULES.md). Worker-bee README in `lib/templates/ext-seeds/worker-bee/` cross-checked.

## Deferred observations

- **TCA skills retrospection**: once the `critic-<lang>` family lands in WP07, the `in-tca-*` suite is largely subsumed (TCA becomes "run the critics and synthesize findings"). Tomorrow's problem — either rewrite against the critic contract or retire. Noted so it does not get lost.
- **WP12 dogfood journal Entries 1-3**: deferred post-release. `critic-shell` is Claude-side, invoked via `Task()` from a session, not via `bash`; its first real invocation belongs to a future session. Journal Entry 0 documents the authorship-pass substitute.
- **Blog draft `docs/blog-drafts/shell-critic-inception.md`**: publication gated on real dogfood Entries 1-3.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
