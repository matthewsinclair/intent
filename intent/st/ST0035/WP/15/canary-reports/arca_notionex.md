---
verblock: "27 Apr 2026:v0.1: matts - arca_notionex canary report"
project: Arca/arca_notionex
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: 9de67e9
---

# arca_notionex canary report

## Outcome

**Pass.** Seventh canary of the v2.10.0 canon. Clean apply once stale `.intent/config.json` was reset. `.claude/` gitignored in arca_notionex by existing convention -- session hooks present on disk but not tracked.

## Pre-flight

- **Working-tree state**: `M .intent/config.json` (stale manual bump) + untracked `.backup/`. Reset config.json; left .backup/ alone (gitignored after the canary commit).
- **HEAD before**: `393bdb6`. Doctor clean. Remotes: `local`, `upstream`. Pushed to `local` only.

## Apply summary

`intent upgrade` ran the chain `2.9.0 -> 2.10.0`. Standard canon plus PLANT MODULES.md, DECISION_TREE.md, RULES.md, ARCHITECTURE.md. Foreign pre-commit chained via marker block. Legacy `.intent/version` file dropped.

## 12-point verification

All 12 points pass.

## Project-specific notes

- **`.claude/` gitignored** by existing project convention. Canon session hooks installed on disk; not tracked. Same trade-off as Utilz -- a fresh clone won't get the session hooks. Track for the WP-17 dogfood journal.
- **`.backup/backup-*` ignore added** (was missing).
- **AGENTS.md was a symlink** (legacy v2.4.0-era pattern); now a real file at root.

## Decision

**Proceed.** Next: Prolix.
