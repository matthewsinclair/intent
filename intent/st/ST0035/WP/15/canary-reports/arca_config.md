---
verblock: "27 Apr 2026:v0.1: matts - arca_config canary report"
project: Arca/arca_config
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: ca85f26
---

# arca_config canary report

## Outcome

**Pass.** Sixth canary of the v2.10.0 canon. Clean apply once stale state was cleaned (config.json + two pre-existing untracked stubs).

## Pre-flight

- **Working-tree state**: `M .intent/config.json` (stale manual bump) + untracked `AGENTS.md` and `intent/llm/AGENTS.md` (probably partial canon attempt from a prior session). Reset config.json to HEAD; deleted the untracked stubs so the canon could write canonical versions.
- **HEAD before**: `9345b42`. Doctor clean. Remotes: `local`, `upstream`. Pushed to `local` only.

## Apply summary

`intent upgrade` ran the chain `2.9.0 -> 2.10.0`. Standard canon plus PLANT MODULES.md, DECISION_TREE.md, RULES.md, ARCHITECTURE.md. Foreign pre-commit chained via marker block. Legacy `intent/llm/llm_preamble.md` removed.

## 12-point verification

All 12 points pass.

## Project-specific notes

- **Untracked `AGENTS.md` and `intent/llm/AGENTS.md` at session start** -- stub files from a prior partial canon attempt. Deleted before applying so the canon could write canonical versions; not lost (no actual content was customised).
- **AGENTS.md was a symlink** (legacy v2.4.0-era pattern); now a real file at root.
- **CLAUDE.md left alone** (user-authored).

## Decision

**Proceed.** Next: arca_notionex.
