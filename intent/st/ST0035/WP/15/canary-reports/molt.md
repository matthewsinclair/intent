---
verblock: "27 Apr 2026:v0.1: matts - Molt canary report"
project: Molt
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: 7abd972
---

# Molt canary report

## Outcome

**Pass.** Third canary of the v2.10.0 canon. Clean apply -- no flybys, no canon-installer surprises. Pre-commit chain action ran via the foreign-hook path (CHAIN_PRE_COMMIT) since Molt had a pre-existing non-Intent pre-commit hook. All 12 verification points pass.

## Pre-flight

- **Working-tree state at session start**: clean.
- **HEAD before**: `08c43ab Intent upgrade` (one commit ahead of `local/main`; push picked up that + the canary commit).
- **Doctor (pre)**: clean (Intent CLI v2.10.0 against Molt v2.9.0).
- **Branch**: `main`. Remotes: `local` (Dropbox), `upstream` (GitHub). Pushed to `local` only.

## Dry-run summary

`intent claude upgrade` reported standard v2.10.0 plan: REGENERATE AGENTS.md, install `.claude/{settings.json,scripts/*}`, install `pre-commit.intent` + chain via marker block (foreign hook present), install `.intent_critic.yml`, install root `usage-rules.md`, install `.treeindex/.treeindexignore`, delete legacy `intent/llm/AGENTS.md`. No `LEGACY` (single-file) detected. CLAUDE.md left alone (user-authored).

## Apply summary

`intent upgrade` ran the full chain:

- **Phase 1**: `.intent/` -> `intent/.config/` (config.json + learnings.md migrated).
- **Phase 2**: stamp re-target `2.9.0 -> 2.10.0`.
- **Phase 3**: 10 actions executed cleanly. Foreign pre-commit hook chained via marker block; canon body installed at `pre-commit.intent`.

Backup written to `.backup/backup-20260427-163957`. Final stamp confirmed `2.10.0`.

## 12-point verification

| #   | Check                                                                  | Result |
| --- | ---------------------------------------------------------------------- | ------ |
| 1   | `intent_version` at `intent/.config/config.json` -> `2.10.0`           | ok     |
| 2   | `AGENTS.md` is a real file at root (no longer a symlink)               | ok     |
| 3   | `intent/llm/AGENTS.md` absent                                          | ok     |
| 4   | `usage-rules.md` present at root                                       | ok     |
| 5   | `.claude/settings.json` carries 3 hooks (SessionStart/Stop/UserPrompt) | ok     |
| 6   | `.git/hooks/pre-commit` and `pre-commit.intent` both executable        | ok     |
| 7   | `.intent_critic.yml` present                                           | ok     |
| 8   | `.claude/scripts/session-context.sh` executable                        | ok     |
| 9   | `intent-chain-block:start` marker present in `pre-commit`              | ok     |
| 10  | `bin/intent_critic --help` reachable                                   | ok     |
| 11  | `intent/.config/` present                                              | ok     |
| 12  | `.intent/` absent                                                      | ok     |

## Project-specific notes

- **AGENTS.md was a symlink before this canary** (legacy v2.4.0-era pattern). Git logged the change as `T` (type change) before the rename was finalised; the new AGENTS.md is a real file at root.
- **`.intent/learnings.md` migrated to `intent/.config/learnings.md`** alongside `config.json`. Git correctly detected both as renames.
- **CLAUDE.md left alone** (user-authored; no Intent footer marker). Refresh deferred per WP-15 protocol.

## Issues encountered

None. Clean canary.

## Decision

**Proceed.** Third canary clean. Next: Utilz.
