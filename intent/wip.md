---
verblock: "24 Apr 2026:v0.42: matts - ST0035 Phase 0 elaborated, awaiting review"
intent_version: 2.9.0
---

# Work In Progress

## Current State

**ST0035 (Canonical LLM Config + Fleet Rollout) opened 2026-04-24.** Phase 0 complete — full scope elaboration committed: `info.md`, `design.md` (10 canon decisions D1–D10 + risk register), `tasks.md` (master checklist), and 17 × `WP/NN/info.md` files with forensic detail (objective, deliverables, approach, acceptance criteria, dependencies, risks, verification, sizing). No production code touched. **Awaiting user review of Phase 0 before WP01 begins.**

Five open decisions carried from planning — user resolution required before WP01:

1. Version bump: 2.9.1 (default) vs 2.10.0.
2. Hook enforcement strictness: soft reminder (default) vs hard gate via UserPromptSubmit.
3. Pre-commit critic threshold: block on CRITICAL + WARNING (default) vs CRITICAL only.
4. PostToolUse advisory critic: off by default (default) vs on.
5. (Resolved during Phase 0) Cancelled ST location: `intent/st/CANCELLED/` convention confirmed.

## Recent

- **2026-04-24**: **ST0035 opened; Phase 0 complete.** Scope: canonical LLM-config surface (AGENTS.md / CLAUDE.md / usage-rules.md / intent/llm/ / .claude/settings.json hooks / pre-commit critic gate) refreshed to state-of-the-art (AGENTS.md community spec, Linux Foundation Agentic AI Foundation governance, Elixir `usage-rules.md` convention preserved), ship as v2.9.1, roll to 17 projects (16 Intent + Pplr). 17 WPs sized XS–L, total ST size XL. Phase 0 committed; implementation gated on user review.
- **2026-04-23 to 2026-04-24**: `critic-shell` dogfood on Intent's own bash codebase. Findings classified P0/P1/P2. Fix commits: `a9ee349` (P0/P1), `0de89cd` (P2 sweep); prompt-fix `60dfcd6`. WP12 dogfood journal Entry 1 complete.
- **2026-04-23**: **v2.9.0 released + fleet rollout complete.** ST0034 closed. Release commit `d1b0fe1`; tag `v2.9.0` on `local` + `upstream`; GitHub release published. Canary (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex) + batch 2 (Laksa, MeetZaya, MicroGPTEx, Molt, Molt-matts) + batch 3 (Multiplyer, Prolix, Utilz, Courses/Agentic Coding) — 13/13 active projects upgraded clean. `intent st done` empty-`completed:` field bug fixed in the release commit. CI workflow retry-fixed (`237f5ce`) after transient GitHub HTTP 500 broke the macOS bats-library clone. False-positive stp/ removal prompt fixed in `983ccbf` (now gated on actual `stp/` directory presence).
- **2026-04-23**: WP10 (Documentation) closed. New canonical docs (rules.md, expanded writing-extensions.md, updated critics.md), all reference files updated, CHANGELOG + release-notes drafts. Mid-WP scope expansion absorbed the TCA suite refactor (in-tca-init/audit/synthesize/remediate/finish drop ad-hoc R-numbering and dispatch critic-<lang>) and 1195-line total-codebase-audit.md update for v2.9.0. `tests/unit/docs_completeness.bats` ships. Task #26 (generator fixes for `intent agents sync`) closed in `f2beaed`; follow-on cleanup commit removed dead `bl)` dispatch case from `bin/intent_main` and swept TPD `intent bl` residue from v2.5.0's Backlog.md removal.
- **2026-04-23**: WP09 (migration chain) closed. v2.8.2 → v2.9.0 migration step authored; chain extended in `bin/intent_upgrade` (gate check + 16 chain-tails + new `"2.8.2"` case); 28 BATS tests in `ext_migration.bats`; full suite 696 ok.
- **2026-04-23**: WP07 (critic subagent family) closed in a single cohesive commit (`398de76`). Four critics + 16-row verification matrix green; critic-shell retrofitted; in-review stage-2 dispatcher; `.intent_critic.yml` schema + sample; intent/docs/critics.md; three new BATS suites (critic_dispatch, critic_report_format, critic_config).
- **2026-04-22**: WP12 (shell rule pack + `critic-shell` subagent) + WP06 (Rust/Swift/Lua rule packs) + WP08 (worker-bee extracted to `lib/templates/ext-seeds/` with `git mv` so history is preserved) + `/in-session` bootstrap skill shipped. Commits `44e05d1`, `c17d03b`, `65f3cea`, `d2edb59`.

## Active Steel Threads

- **ST0035**: Canonical LLM Config + Fleet Rollout (WIP). Phase 0 docs elaborated; implementation gated on user review. `intent/st/ST0035/` — see `info.md`, `design.md`, `tasks.md`, `WP/01/` through `WP/17/`. Plan source: `/Users/matts/.claude/plans/ultrathink-on-please-ingest-elegant-sundae.md`.

## Next Up

1. **Review ST0035 Phase 0** — user review of `info.md` + `design.md` + 17 × WP/NN/info.md. Resolve 5 open decisions. Approve implementation start.
2. **WP01 begins after Phase 0 approval**: self-upgrade Intent to v2.9.1 + cancel ST0010/ST0015 via existing `Cancelled` status with deprecation annotations.

## Deferred observations

- **`critic-shell` dogfood blog post**: path updated to `docs/blog/_drafts/####-shell-critic-inception.md` (2026-04-24). Working title "Using Intent's critic-shell on Intent: Inception Edition". Publication gated on real dogfood runs (Entries 1-3, now underway post-v2.9.0).
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across four critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

- ST0010 (Anthropic MCP Integration, v2.0.0-era) — cancelled via ST0035 WP01 with deprecation annotation.
- ST0015 (Enhanced Steel Thread Templates, v2.0.0-era) — cancelled via ST0035 WP01 with deprecation annotation.
