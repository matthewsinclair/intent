---
verblock: "23 Apr 2026:v0.38: matts - ST0034 WP10 done; WP11 next (release)"
intent_version: 2.8.2
---

# Work In Progress

## Current State

Intent v2.8.2 (VERSION unchanged; release bump is WP11). **ST0034 active — 11/12 WPs done.** WP10 (Documentation) closed in this session. Three new docs landed (`intent/docs/rules.md`, expanded `intent/docs/writing-extensions.md`, updated `intent/docs/critics.md`); CLAUDE.md / MODULES.md / DECISION_TREE.md / creating-custom-agents.md / `lib/help/*.help.md` updated for v2.9.0; AGENTS.md regenerated via `intent agents sync` (with a noted generator-deficiency follow-up); CHANGELOG + release-notes drafts staged for WP11. Mid-WP scope expansion: full TCA suite refactor (5 skills) for rule library + 1195-line `intent/docs/total-codebase-audit.md` updated + `rules:` frontmatter added to in-elixir-essentials/in-elixir-testing. WP11 (release + fleet upgrade) is the only remaining WP.

## Recent

- **2026-04-23**: WP10 (Documentation) closed. New canonical docs (rules.md, expanded writing-extensions.md, updated critics.md), all reference files updated, CHANGELOG + release-notes drafts. Mid-WP scope expansion absorbed the TCA suite refactor (in-tca-init/audit/synthesize/remediate/finish drop ad-hoc R-numbering and dispatch critic-<lang>) and 1195-line total-codebase-audit.md update for v2.9.0. `tests/unit/docs_completeness.bats` ships (11 tests). 707/707 BATS green; 48/48 rules validate ok.
- **2026-04-23**: WP09 (migration chain) closed. v2.8.2 → v2.9.0 migration step authored; chain extended in `bin/intent_upgrade` (gate check + 16 chain-tails + new `"2.8.2"` case); 28 BATS tests in `ext_migration.bats`; full suite 696 ok.
- **2026-04-23**: WP07 (critic subagent family) closed in a single cohesive commit (`398de76`). Four critics + 16-row verification matrix green; critic-shell retrofitted; in-review stage-2 dispatcher; `.intent_critic.yml` schema + sample; intent/docs/critics.md; three new BATS suites (critic_dispatch, critic_report_format, critic_config).
- **2026-04-22**: WP12 (shell rule pack + `critic-shell` subagent) + WP06 (Rust/Swift/Lua rule packs) + WP08 (worker-bee extracted to `lib/templates/ext-seeds/` with `git mv` so history is preserved) + `/in-session` bootstrap skill shipped. Commits `44e05d1`, `c17d03b`, `65f3cea`, `d2edb59`.
- **2026-04-15**: v2.8.2 released (tagged twice; second commit `84a3a5f` slipstreamed a fix to `bin/intent_upgrade`). ST0033 (cwd-resilient dispatch) + ST0032 (Credo checks) + upgrade-chain gap fix (`migrate_v2_6_0_to_v2_8_0`). Fleet of 16 projects brought to 2.8.2.
- **2026-04-09**: v2.8.1 released -- ST0031 (TCA suite hardening).

## Active Steel Threads

- **ST0034 (Agentic Software Engineering Suite)** — v2.9.0 target. 12 WPs. 11 done: WP01 schema, WP02 ext system, WP03 skill/subagent rationalisation (elixir subagent deleted), WP04 agnostic pack, WP05 Elixir pack, WP06 Rust/Swift/Lua packs, WP07 critic family, WP08 worker-bee extraction, WP09 migration chain, WP10 documentation (this session), WP12 shell pack + critic-shell. Remaining: **WP11 release + fleet upgrade**. See `intent/st/ST0034/impl.md` for the full progress tracker.

## Next Up

- **WP11 (Medium)**: release + fleet upgrade. Bump `VERSION` to `2.9.0`; tag `v2.9.0` and force-push to both remotes; publish GitHub release using the v2.9.0 release-notes draft (`docs/releases/2.9.0/RELEASE_NOTES.md`) and the CHANGELOG v2.9.0 entry. Bump worker-bee seed `intent_compat.min` in lockstep with VERSION. Run WP09 canary dry-run against fleet projects (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab) before tagging. Roll upgrade across the 16-project fleet. See `intent/st/ST0034/WP/11/info.md` for the tagged plan.

## Deferred observations

- **TCA skills retrospection: closed in WP10.** Full TCA suite (5 skills + total-codebase-audit.md) refactored against the critic contract — drops ad-hoc R-numbering, dispatches `critic-<lang>`, consumes the stable severity-grouped report.
- **WP12 dogfood journal Entries 1-3**: deferred post-release. `critic-shell` is Claude-side, invoked via `Task()` from a session, not via `bash`; its first real invocation belongs to a future session. Journal Entry 0 documents the authorship-pass substitute.
- **Blog draft `docs/blog-drafts/shell-critic-inception.md`**: publication gated on real dogfood Entries 1-3.
- **`intent agents sync` generator deficiencies**: surfaced during WP10. Generator drops `intent wp` commands (replaced with stale `intent bl`), can't detect Bats test framework, renders empty descriptions for some subagents (e.g. diogenes). Not WP10 scope — needs a dedicated ST or WP. Workaround until then: hand-edits to AGENTS.md will be wiped by the next sync.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
