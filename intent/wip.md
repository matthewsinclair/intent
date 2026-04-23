---
verblock: "23 Apr 2026:v0.41: matts - Intent v2.9.0 release + fleet rollout complete"
intent_version: 2.9.0
---

# Work In Progress

## Current State

**Intent v2.9.0 released 2026-04-23. Fleet rollout complete: 13/13 active projects on 2.9.0.** ST0034 (Agentic Software Engineering Suite) closed. Canary (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex), batch 2 (Laksa, MeetZaya, MicroGPTEx, Molt, Molt-matts), batch 3 (Multiplyer, Prolix, Utilz, Courses/Agentic Coding) — every project at `intent_version: 2.9.0`, `intent doctor` clean, zero rollbacks. Conflab + Lamplight + A3/\* skipped per direction. Tag `v2.9.0` on `local` + `upstream`; GitHub release at <https://github.com/matthewsinclair/intent/releases/tag/v2.9.0>. CI workflow retry-fixed in same session after a transient GitHub HTTP 500 broke the macOS bats-library clone on the release push. No active steel threads.

## Recent

- **2026-04-23**: **v2.9.0 released + fleet rollout complete.** ST0034 (Agentic Software Engineering Suite) closed. Release commit `d1b0fe1`; tag `v2.9.0` on `local` + `upstream`; GitHub release published. Canary (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex) + batch 2 (Laksa, MeetZaya, MicroGPTEx, Molt, Molt-matts) + batch 3 (Multiplyer, Prolix, Utilz, Courses/Agentic Coding) — 13/13 active projects upgraded clean. `intent st done` empty-`completed:` field bug fixed in the release commit. CI workflow retry-fixed (`237f5ce`) after transient GitHub HTTP 500 broke the macOS bats-library clone. False-positive stp/ removal prompt fixed in `983ccbf` (now gated on actual `stp/` directory presence).
- **2026-04-23**: WP10 (Documentation) closed. New canonical docs (rules.md, expanded writing-extensions.md, updated critics.md), all reference files updated, CHANGELOG + release-notes drafts. Mid-WP scope expansion absorbed the TCA suite refactor (in-tca-init/audit/synthesize/remediate/finish drop ad-hoc R-numbering and dispatch critic-<lang>) and 1195-line total-codebase-audit.md update for v2.9.0. `tests/unit/docs_completeness.bats` ships. Task #26 (generator fixes for `intent agents sync`) closed in `f2beaed`; follow-on cleanup commit removed dead `bl)` dispatch case from `bin/intent_main` and swept TPD `intent bl` residue from v2.5.0's Backlog.md removal.
- **2026-04-23**: WP09 (migration chain) closed. v2.8.2 → v2.9.0 migration step authored; chain extended in `bin/intent_upgrade` (gate check + 16 chain-tails + new `"2.8.2"` case); 28 BATS tests in `ext_migration.bats`; full suite 696 ok.
- **2026-04-23**: WP07 (critic subagent family) closed in a single cohesive commit (`398de76`). Four critics + 16-row verification matrix green; critic-shell retrofitted; in-review stage-2 dispatcher; `.intent_critic.yml` schema + sample; intent/docs/critics.md; three new BATS suites (critic_dispatch, critic_report_format, critic_config).
- **2026-04-22**: WP12 (shell rule pack + `critic-shell` subagent) + WP06 (Rust/Swift/Lua rule packs) + WP08 (worker-bee extracted to `lib/templates/ext-seeds/` with `git mv` so history is preserved) + `/in-session` bootstrap skill shipped. Commits `44e05d1`, `c17d03b`, `65f3cea`, `d2edb59`.
- **2026-04-15**: v2.8.2 released (tagged twice; second commit `84a3a5f` slipstreamed a fix to `bin/intent_upgrade`). ST0033 (cwd-resilient dispatch) + ST0032 (Credo checks) + upgrade-chain gap fix (`migrate_v2_6_0_to_v2_8_0`). Fleet of 16 projects brought to 2.8.2.
- **2026-04-09**: v2.8.1 released -- ST0031 (TCA suite hardening).

## Active Steel Threads

None. ST0034 (Agentic Software Engineering Suite) closed at v2.9.0 release.

## Next Up

1. **Run `critic-shell` on Intent's bash codebase.** Targets: `bin/intent*`, `bin/intent_helpers`, `bin/intent_main`, `bin/intent_st`, `bin/intent_upgrade`, `intent/plugins/agents/bin/intent_agents`, `intent/plugins/claude/bin/intent_claude_*`, `tests/run_tests.sh`. Dispatch via `Task(subagent_type="critic-shell", prompt="review <files>")`. Consume the severity-grouped report; classify findings into the standard P0/P1/P2a/P2b/P3 tiers; decide which to act on now vs queue as a follow-up ST. First real-world dogfood of `critic-shell` against Intent's own bash — also satisfies WP12 dogfood journal Entry 1.

Open queue otherwise empty. Pick from Parked or initiate a new ST when work surfaces.

## Deferred observations

- **TCA skills retrospection: closed in WP10.** Full TCA suite (5 skills + total-codebase-audit.md) refactored against the critic contract — drops ad-hoc R-numbering, dispatches `critic-<lang>`, consumes the stable severity-grouped report.
- **WP12 dogfood journal Entries 1-3**: deferred post-release. `critic-shell` is Claude-side, invoked via `Task()` from a session, not via `bash`; its first real invocation belongs to a future session. Journal Entry 0 documents the authorship-pass substitute.
- **Blog draft `docs/blog-drafts/shell-critic-inception.md`**: publication gated on real dogfood Entries 1-3.
- **`intent agents sync` generator deficiencies: fixed in `f2beaed` (Task #26).** Generator now emits current `intent wp` commands, detects nested Bats layouts via recursive find (`bats -r tests/`), and falls back to `agent.md` frontmatter when `metadata.json` is missing. Follow-on commit removed the dead `bl)` dispatch case from `bin/intent_main` and swept the TPD `intent bl` residue from v2.5.0's Backlog.md removal.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
