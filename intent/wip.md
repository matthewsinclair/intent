---
verblock: "27 Apr 2026:v0.64: matts - v2.10.0 shipped; ST0035 + ST0036 in COMPLETED; no active ST"
intent_version: 2.10.0
---

# Work In Progress

## Current State

**Intent v2.10.0 shipped 2026-04-27.** Tag pushed to `local` + `upstream`; GitHub release live at https://github.com/matthewsinclair/intent/releases/tag/v2.10.0. ST0035 (Canonical LLM Config + Fleet Rollout) 19 of 19 Done. ST0036 (Directory relocation `.intent/` -> `intent/.config/`) 9 of 9 Done. Both in `intent/st/COMPLETED/`. No active steel thread.

Tests **810/810 green**. `intent doctor` clean. CI run for the release-prep commit (`cf37292`) was in progress at publish time; check `gh run list` if that needs follow-up.

## ST0035 final shape (Completed)

| Status | WP  | Title                                                                         | Size |
| ------ | --- | ----------------------------------------------------------------------------- | ---- |
| Done   | 01  | Self-upgrade to v2.10.0 + cancel ST0010 / ST0015                              | XS   |
| Done   | 02  | Refresh root `usage-rules.md`                                                 | S    |
| Done   | 03  | Write `intent/docs/working-with-llms.md`                                      | M    |
| Done   | 04  | `.claude/settings.json` template (SessionStart + UserPromptSubmit + Stop)     | M    |
| Done   | 05  | `bin/intent_critic` headless runner                                           | L    |
| Done   | 06  | `.git/hooks/pre-commit` template                                              | S    |
| Done   | 07  | `.intent_critic.yml` default template                                         | XS   |
| Done   | 08  | Root `AGENTS.md` generator rewrite                                            | M    |
| Done   | 09  | Root `CLAUDE.md` overlay template                                             | S    |
| Done   | 10  | Delete deprecated artefacts                                                   | XS   |
| Done   | 11  | Extend `intent claude upgrade` to apply canon artefacts                       | M    |
| Done   | 12  | Socrates/Diogenes FAQ cross-refs                                              | XS   |
| Done   | 13  | Update Intent's own CLAUDE.md                                                 | S    |
| Done   | 14  | Self-apply canon to Intent (dogfood; verification sweep post-WP08)            | S    |
| Done   | 15  | Canary rollout across in-scope fleet projects (11 of 11 pass)                 | M    |
| Done   | 16  | Fleet rollout (Intent ecosystem; 8 absorbed into WP-15, 5 user-manual, OOS 1) | L    |
| Done   | 17  | Verification sweep + dogfood journal (12-point per project)                   | S    |
| Done   | 18  | Review and update (or retire) `intent/usr/*.md`                               | M    |
| Done   | 19  | Per-language canon: `intent lang init` + `intent init --lang`                 | M    |

## Recent

- **2026-04-27 (this session, post-second-compact)**: WP-18 closed (`329e9f3` -- retire intent/usr/\*.md, three files canon-stale, replaced README + blog + migration cross-refs). WP-17 closed (`92e1ab7` -- spec tidy + 14-row verification matrix + dogfood journal; release engineering deferred from WP-17 to post-WP-19). WP-19 closed (`6c1f41e` -- new `intent lang` command + `intent init --lang` flag + per-language stub templates for rust/swift/lua/shell + intent_init lays down agnostic \_default RULES.md/ARCHITECTURE.md). ST0035 marked done via `intent st done ST0035`; moved to `intent/st/COMPLETED/`. Tests 791 -> 810 (+19 new scenarios across two BATS files).

- **2026-04-27 (this session, first-compact half)**: WP-15 + WP-16 closed (canary aggregate, fleet summary, three .intent/ cleanup commits). See `intent/st/COMPLETED/ST0035/WP/15/canary-summary.md` and `WP/16/fleet-summary.md`.

- **2026-04-27 (earlier)**: see `intent/restart.md` for the WP-08 to WP-14 arc.

## Next Up

No active steel thread. Pick the next ST off the v2.10.x follow-up backlog (below) or start fresh exploratory work for v2.11.

## Open follow-ups (post v2.10.0)

- **`intent doctor` warning for leftover `.intent/` post-migration**. Decision in WP-17 dogfood journal: warn at doctor; do NOT auto-stage. Filed as v2.10.x follow-up.
- **`intent claude upgrade --dry-run` UX polish**: reword "expected during dry-run" cases.
- **CLAUDE.md content drift**: per-project refresh decision (deferred; per-project judgement call).
- **Blog draft**: `docs/blog/_drafts/####-shell-critic-inception.md` -- Laksa is the first dogfood datapoint.
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

## Parked

_(None.)_
