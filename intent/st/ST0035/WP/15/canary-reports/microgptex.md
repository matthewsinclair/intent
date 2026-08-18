---
verblock: "27 Apr 2026:v0.1: matts - MicroGPTEx canary report"
project: MicroGPTEx
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: b375d1f
---

# MicroGPTEx canary report

## Outcome

**Pass.** Ninth canary of the v2.10.0 canon. Clean apply once stale `.intent/config.json` was reset. `.claude/` gitignored in MicroGPTEx by existing convention.

## Pre-flight

- **Working-tree state**: `M .intent/config.json` (stale manual bump). Reset to HEAD.
- **HEAD before**: `89f56b2`. Doctor clean. Remotes: `local`, `upstream`. Pushed to `local` only.

## Apply summary

`intent upgrade` ran the chain `2.9.0 -> 2.10.0`. Standard canon plus PLANT MODULES.md and DECISION_TREE.md. Foreign pre-commit chained via marker block.

## 12-point verification

All 12 points pass.

## Project-specific notes

- **`.claude/` gitignored** by existing project convention. Canon session hooks installed on disk; not tracked. Same trade-off as Utilz / arca_notionex.
- **AGENTS.md was a symlink** (legacy v2.4.0-era pattern); now a real file at root.
- **CLAUDE.md left alone** (user-authored).

## Decision

**Proceed.** All in-scope canaries except Conflab + Lamplight (deferred -- both busy) now done. 9 of 11 in-scope; 2 deferred. Pplr out of scope.
