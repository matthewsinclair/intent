---
verblock: "27 Apr 2026:v0.1: matts - Utilz canary report"
project: Utilz
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: ed31017
---

# Utilz canary report

## Outcome

**Pass.** Fourth canary of the v2.10.0 canon. Clean apply once stale `.intent/config.json` was reset. One project-specific quirk: Utilz gitignores `.claude/` by convention, so the canon session hooks are present on disk but not tracked.

## Pre-flight

- **Working-tree state at session start**: `M .intent/config.json` (stale manual bump). Reset to HEAD before applying.
- **HEAD before**: `b2bce1f Intent ug` (one commit ahead of `local/main`).
- **Doctor (pre)**: clean.
- **Branch**: `main`. Remotes: `local`, `upstream`. Pushed to `local` only.

## Apply summary

`intent upgrade` ran the chain `2.8.2 -> 2.10.0`. 12 actions queued (one extra vs Molt: `PLANT MODULES.md` + `PLANT DECISION_TREE.md` since Utilz didn't have these seeds yet). Foreign pre-commit chained via marker block.

Backup at `.backup/backup-20260427-164209`. Final stamp `2.10.0`.

## 12-point verification

| #   | Check                                                | Result |
| --- | ---------------------------------------------------- | ------ |
| 1   | `intent_version` -> `2.10.0`                         | ok     |
| 2   | `AGENTS.md` real file at root                        | ok     |
| 3   | `intent/llm/AGENTS.md` absent                        | ok     |
| 4   | `usage-rules.md` present                             | ok     |
| 5   | `.claude/settings.json` carries 3 hooks              | ok     |
| 6   | `pre-commit` and `pre-commit.intent` both executable | ok     |
| 7   | `.intent_critic.yml` present                         | ok     |
| 8   | `session-context.sh` executable                      | ok     |
| 9   | `intent-chain-block:start` marker in `pre-commit`    | ok     |
| 10  | `bin/intent_critic --help` reachable                 | ok     |
| 11  | `intent/.config/` present                            | ok     |
| 12  | `.intent/` absent                                    | ok     |

## Project-specific notes

- **`.claude/` is gitignored in Utilz** (line 1 of `.gitignore`). The canon installer wrote `.claude/settings.json` and `.claude/scripts/*` (verified on disk; check 5 + check 8 pass), but they will not be committed. This is a deliberate user convention -- the canon respects it. Trade-off: a fresh clone of Utilz will not get the session hooks, so `/in-session` would not be auto-required there. Worth noting for the dogfood journal in WP-17.
- **Stamp chain `2.8.2 -> 2.10.0`** despite the working-tree showing `2.9.0` pre-reset; the migration reads HEAD's stamped version (`2.8.2`), not the staged-but-uncommitted bump.
- **CLAUDE.md left alone** (user-authored).
- **AGENTS.md was a symlink** (legacy v2.4.0-era pattern); now a real file at root.

## Issues encountered

None blocking. Two benign observations (stale config bump pre-reset; `.claude/` gitignore policy preserved).

## Decision

**Proceed.** Fourth canary clean. Next: Arca/arca_cli.
