---
verblock: "27 Apr 2026:v0.1: matts - Lamplight canary report"
project: Lamplight
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: 1b0b3bbe
---

# Lamplight canary report

## Outcome

**Pass.** Eleventh (and final in-scope) canary of the v2.10.0 canon. Lamplight had a partially-applied v2.10.0 canon migration sitting uncommitted in the working tree from an earlier session, plus the same gaps as Conflab (chain block missing, session-context.sh diverged, `.gitignore` not normalised). This session completed the migration in a single canon commit, isolated from active Elixir engine/ingestor WIP.

## Pre-flight

- **Working-tree state**: partial canon migration uncommitted (D `.intent/config.json`, T `AGENTS.md`, D `intent/llm/AGENTS.md`, untracked `.claude/scripts/`, `.claude/settings.json`, `.intent_critic.yml`, `intent/.config/`, `usage-rules.md`); plus M `CLAUDE.md` (canon refresh from earlier session); plus active WIP in `apps/lamplight/lib/lamplight/{content/fidelity,explorer,ingestor}` and `apps/lamplight/test/lamplight/{content/fidelity,runengine}`. All WIP preserved untouched.
- **HEAD before**: `e54f7cea`. Doctor clean. Remotes: `local`, `upstream`. Pushed to `local` only.

## Apply summary

`intent claude upgrade --apply` ran 4 actions, same shape as Conflab:

1. **INSTALL_HOOK_SCRIPT:session-context.sh** -- refreshed to current canon source.
2. **CHAIN_PRE_COMMIT** -- re-installed `pre-commit.intent` and inserted the marker chain block into `pre-commit`. Idempotent.
3. **REFRESH_CLAUDE_MD** -- date stamp refreshed (user section preserved).
4. **NORMALIZE_GITIGNORE** -- added `.claude/settings.local.json` + `/AGENTS.md.bak` entries.

Plus the previously-uncommitted partial migration files were staged together with the new canon deltas to land everything as a single coherent canon commit.

## 12-point verification

All 12 points pass.

## Project-specific notes

- **Legacy `intent/.config/version` file dropped** (was `2.0.0`; superseded by `intent_version` in `config.json`). Mirrors what other canaries did in their pre-2.10.0 cleanup; brings Lamplight in line.
- **`.claude/settings.local.json`** had unrelated tool-allowlist edits (per-developer); explicitly not staged. Will be gitignored on re-clone post NORMALIZE_GITIGNORE.
- **Active Elixir WIP** (`counter.ex`, `media_browser.ex`, `validate_references.ex`, plus 2 test files) intentionally not in this commit.
- **Pre-commit chain was the load-bearing fix**: same as Conflab -- `pre-commit.intent` was installed but never invoked.

## Decision

**Proceed.** Eleventh canary clean. **All 11 in-scope canary projects done.** Pplr remains out of scope (does not need intent). Suggest running `intent wp done ST0035/15` when convenient and proceeding to WP-16/17/18.
