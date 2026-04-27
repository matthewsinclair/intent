---
verblock: "27 Apr 2026:v0.55: matts - ST0036 closed (9 of 9 Done); ST0035 13 of 19 (WP19 added)"
intent_version: 2.10.0
---

# Work In Progress

## Current State

**ST0036 closed -- 9 of 9 WPs Done; moved to `intent/st/COMPLETED/ST0036/`. ST0035 active -- 13 of 19 Done (WP14-WP19 remaining; WP19 added this session as a Phase 0 spec).** Intent's repo is now at `intent/.config/` layout (post-WP08 relocation), stamped 2.10.0, with v2.10.0 canon installed (`.claude/settings.json` + 3 hook scripts, `.intent_critic.yml`, root CLAUDE.md refreshed, intent/llm/RULES.md + ARCHITECTURE.md from `_default` templates). Tests: **781/781 green**. Doctor: clean. Backup tag `wp08-pre-relocate` at `69069eca` (discardable).

## ST0035 progress

| Status      | WP  | Title                                                                                 | Size |
| ----------- | --- | ------------------------------------------------------------------------------------- | ---- |
| Done        | 01  | Self-upgrade to v2.10.0 + cancel ST0010 / ST0015                                      | XS   |
| Done        | 02  | Refresh root `usage-rules.md`                                                         | S    |
| Done        | 03  | Write `intent/docs/working-with-llms.md`                                              | M    |
| Done        | 04  | `.claude/settings.json` template (SessionStart + UserPromptSubmit strict gate + Stop) | M    |
| Done        | 05  | `bin/intent_critic` headless runner                                                   | L    |
| Done        | 06  | `.git/hooks/pre-commit` template                                                      | S    |
| Done        | 07  | `.intent_critic.yml` default template                                                 | XS   |
| Done        | 08  | Root `AGENTS.md` generator rewrite                                                    | M    |
| Done        | 09  | Root `CLAUDE.md` overlay template                                                     | S    |
| Done        | 10  | Delete deprecated artefacts                                                           | XS   |
| Done        | 11  | Extend `intent claude upgrade` to apply canon artefacts                               | M    |
| Done        | 12  | Socrates/Diogenes FAQ cross-refs                                                      | XS   |
| Done        | 13  | Update Intent's own CLAUDE.md                                                         | S    |
| Not Started | 14  | Self-apply canon to Intent (dogfood; verification sweep post-WP08)                    | S    |
| Not Started | 15  | Canary rollout (Conflab, Lamplight, Laksa) -- 12-point checklist per WP-09            | M    |
| Not Started | 16  | Fleet rollout (12 Intent + Pplr) -- 12-point checklist per WP-09                      | L    |
| Not Started | 17  | Verification sweep + dogfood journal (12-point per project)                           | S    |
| Not Started | 18  | Review and update (or retire) `intent/usr/*.md`                                       | M    |
| Not Started | 19  | Per-language canon: `intent lang init` + `intent init --lang` (added 2026-04-27)      | M    |

## ST0036 closed (2026-04-27)

| WP   | Commit                   | Notes                                                                                                                       |
| ---- | ------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| WP01 | `4dcccce`                | `migrate_v2_9_0_to_v2_10_0` + `intent_relocate_dotintent`. Half-fix surfaced by WP08; dispatcher fix landed at `01159ff`.   |
| WP02 | `5369afd` then `33a99d0` | Path probes flipped; `detect_project_version` narrow exception for legacy layouts.                                          |
| WP03 | `777c5b0`                | Literal sweep across `bin/`, `intent/plugins/`, `intent/docs/`, `intent/usr/`.                                              |
| WP04 | `5f8b61e` (+ `f04db11`)  | Templates + generators audit. Single material flip in `lib/templates/hooks/pre-commit.sh`.                                  |
| WP05 | `b62ea58`                | 11 BATS files flipped; new `tests/unit/migrate_v2_9_0_to_v2_10_0.bats`.                                                     |
| WP06 | `32df058`                | New `lib/templates/_treeindexignore` template; canon installer ships it.                                                    |
| WP07 | `1debc03`                | `intent/docs/migration-v2.10.0.md` migration guide.                                                                         |
| WP08 | `5c782b3`                | Intent self-apply. `intent upgrade` ran all three phases. Git rename detected. Pre-flight: `01159ff`, `ebd6620`, `a7c27c3`. |
| WP09 | `1497885`                | Cross-thread coordination: WP15-17 12-point checklist + version flips + impl.md finalisation.                               |

See `intent/st/COMPLETED/ST0036/impl.md` for closing notes.

## Resolved decisions (this session)

1. **Auto-detection rejected** -- single-language project detection was tried and pushed back ("multi-language is the rule"). Replaced with explicit `intent lang init <lang>` + `intent init --lang ...` per ST0035/WP-19 (Phase 0 spec only; implementation deferred).
2. **canon-installer always uses `_default` templates** for RULES/ARCHITECTURE/AGENTS regardless of project language. Per-language templates remain as opt-in starters for `intent init --template <lang>`.
3. **Layout-keyed idempotence** in `bin/intent_upgrade` + `needs_v2_10_0_upgrade` -- catches the "stamp at target, layout pre-relocation" state that any stamp-only check misses.

## Recent

- **2026-04-27 (this session)**: **ST0036 closed (9 of 9 Done)**, with three follow-on commits prompted by WP-08's surfacing pass. Five session commits in order:
  - `01159ff` -- WP-01 dispatcher fix (layout-aware early-exit + `needs_v2_10_0_upgrade` shortcut + new `2.10.0` case arm). Plus 4 BATS scenarios.
  - `ebd6620` -- WP-11 canon-installer fix (PROJECT_NAME from `intent/.config/config.json`; always-`_default` templates). Plus `_default/` template directory + 3 BATS scenarios + `.backup/` to `.gitignore`.
  - `a7c27c3` -- WP-19 Phase 0 spec (per-language canon command). Captured the explicit-language design that replaces the rejected auto-detection.
  - `5c782b3` -- WP-08 Intent self-apply. Reverted WP-05 manual-mv diagnostic; ran proper `intent upgrade`. Three phases of `migrate_v2_9_0_to_v2_10_0` executed; git rename detected cleanly; canon-apply installed everything missing; treeindex pruned of 7 orphans.
  - `1497885` -- WP-09 cross-thread coordination. ST0035 WP15/WP16/WP17 info.md gained 12-point checklist + version flips. ST0036/impl.md finalised.

  Plus ST0036 closed via `intent st done ST0036` (commit pending in this Phase C wrap).

- **2026-04-26**: see `intent/st/COMPLETED/ST0036/impl.md` for the WP-by-WP table.
- **2026-04-25**: see `intent/st/ST0035/` for the ST0035 history through WP-13.
- **2026-04-23**: v2.9.0 released + fleet rollout complete. ST0034 closed.

## Next Up

1. **ST0035/WP14 (S)** -- Intent self-dogfood verification. WP08 already executed Phase 3 (canon-apply); WP14 is a verification sweep (re-run `intent claude upgrade` no-apply; confirm all UP TO DATE / PRESENT; doctor; BATS; mark Done).
2. **ST0035/WP15-17** -- Canary (Conflab -> Lamplight -> Laksa) + fleet rollout (12 Intent + Pplr) + verification sweep. Each project carries both ST0035 + ST0036 concerns per WP-09 coordination (12-point checklist).
3. **ST0035/WP18 (M)** -- `intent/usr/*.md` audit (parallel with WP15/WP16; must land before WP17).
4. **ST0035/WP19 (M)** -- `intent lang ...` command + `intent init --lang` flag. Phase 0 elaborated; implementation in 2-3 sessions. Independent of WP14-WP18; can land anywhere.

## Deferred observations

- **Pre-commit chain not yet active**: canon-installer left `.git/hooks/pre-commit.intent` but did NOT modify the existing `pre-commit` to call it. Manual chain block paste required (snippet in canon-installer output). Activate when WP14 runs.
- **Backup tag cleanup**: `git tag -d wp08-pre-relocate` once next session confirms stability.
- **Blog draft path**: `docs/blog/_drafts/####-shell-critic-inception.md`. Publication gated on real dogfood runs.
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

_(None.)_
