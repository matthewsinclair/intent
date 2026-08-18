---
verblock: "27 Apr 2026:v0.1: matts - arca_cli canary report"
project: Arca/arca_cli
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: 2e7c14f
---

# arca_cli canary report

## Outcome

**Pass.** Fifth canary of the v2.10.0 canon. Clean apply once stale `.intent/config.json` was reset. Migration also dropped the legacy `intent/llm/llm_preamble.md` (an artefact from before the v2.4.0 three-file structure).

## Pre-flight

- **Working-tree state**: `M .intent/config.json` (stale manual bump). Reset to HEAD.
- **HEAD before**: `b516de4`. Doctor clean. Remotes: `local`, `upstream`. Pushed to `local` only.

## Apply summary

`intent upgrade` ran the chain `2.9.0 -> 2.10.0`. 13 actions: standard canon plus PLANT MODULES.md, DECISION_TREE.md, RULES.md, ARCHITECTURE.md (none existed). Foreign pre-commit chained via marker block.

## 12-point verification

All 12 points pass.

## Project-specific notes

- **Legacy `intent/llm/llm_preamble.md` removed** by the migration -- pre-v2.4.0 artefact.
- **AGENTS.md was a symlink** (legacy v2.4.0-era pattern); now a real file at root.
- **CLAUDE.md left alone** (user-authored).

## Decision

**Proceed.** Next: arca_config.
