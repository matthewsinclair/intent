---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-05
title: "Highlander consolidation pass"
scope: M
status: Done
---

# WP-05: Highlander consolidation pass

## Objective

Eliminate the divergent-copy violations of theme T5. The status-normalisation split is a live behaviour bug and goes first; the rest is consolidation to single sources.

## Evidence

- LIVE BUG (HIGH): `normalise_status` maps `wip` -> `WIP`, but inline copies in `repair` and `organize` map `wip` -> `In Progress` (`intent_st:107-140` vs `:1410-1426` vs `:1588-1604`).
- `get_intent_version` fallback literal repeated ~14x with drift (`2.2.1`/`2.6.0`/`2.3.x`/...) -- a broken install reports a different stale version depending on which script is asked (`bin/intent:27`, `intent_config:17`, `intent_helpers:35,607,...`, `intent_upgrade:14`).
- Config parsing implemented three divergent ways (`intent_config:23-37` eval+jq -- dies in WP-01; `intent_helpers:67-80` grep/cut; `intent_claude_upgrade:576` grep|sed) (F-ARCH-2).
- `find_project_root` reimplemented three times (`intent_config:40-67`, `critic_runner.sh:51-61`, `pre-commit.sh:58-65`) (F-ARCH-6).
- ST-dir resolver in three places (`intent_wp:62`, `intent_st:160-165`, `:1500`); ext-dir walk in five (F-PLG-6).
- Fake-HOME test isolation copy-pasted in six test files, absent in the seventh (F-TEST-9) -- handled in WP-09 part A, not here.

## Scope exclusions

- `update_config_version` inlined in ~12 migrate functions: upgrade-only code, ST0043 deletes most of those functions -- excluded here (gate decision 2026-06-11).

## Deliverables

- Part A (first commit): single `normalise_status`, inline copies deleted, regression test proving `repair`/`organize` now agree with the canonical mapping.
- One version-fallback constant (single definition, sourced everywhere).
- One config field-read helper (post-WP-01 `jq -r` form) used by all three former parsers where they read the same concern.
- One `find_project_root`, one ST-dir resolver, one ext-dir walk -- each in the shared helper layer, registered in MODULES.md.

## Acceptance Criteria

- [ ] `intent st repair` / `organize` normalise `wip` identically to `normalise_status`.
- [ ] Grep for the old fallback literals finds one definition site.
- [ ] MODULES.md rows updated for any relocated helper (register before code).
- [ ] Full bats suite green; critic-shell clean on touched scripts.

## Dependencies

- WP-01 (config eval) first -- the consolidation target for config parsing is the post-eval field-wise reader.
