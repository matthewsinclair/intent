---
verblock: "27 Apr 2026:v0.1: matts - Conflab canary report"
project: Conflab
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: 973dae62
---

# Conflab canary report

## Outcome

**Pass.** Tenth canary of the v2.10.0 canon. Conflab had been migrated to v2.10.0 layout earlier in the rollout, but predates two canon-installer refinements (chain block auto-insert, NORMALIZE_GITIGNORE) and ships an older `session-context.sh`. This session completed the migration cleanly, isolated from active Rust/Swift WIP in the tree.

## Pre-flight

- **Working-tree state**: substantial unrelated work in progress (Rust daemon mutations, Swift macOS views, ~12 modified tracked files; untracked `intent/st/ST0099/` and `native/daemon/src/provider/probe.rs`). All preserved untouched.
- **HEAD before**: `a1188d85`. Doctor clean. Remotes: `local`, `upstream`. Pushed to `local` only.
- **Canon state pre-apply**: stamp at 2.10.0; layout at intent/.config/; `pre-commit.intent` installed; chain block missing from `pre-commit`; `session-context.sh` DIVERGED from current canon source.

## Apply summary

`intent claude upgrade --apply` ran 4 actions:

1. **INSTALL_HOOK_SCRIPT:session-context.sh** -- refreshed to current canon source.
2. **CHAIN_PRE_COMMIT** -- re-installed `pre-commit.intent` and inserted the marker chain block into `pre-commit`. Idempotent.
3. **REFRESH_CLAUDE_MD** -- date stamp refreshed (user section preserved between markers).
4. **NORMALIZE_GITIGNORE** -- added `.claude/settings.local.json` + `/AGENTS.md.bak` entries.

## 12-point verification

All 12 points pass.

## Project-specific notes

- **Active Rust/Swift WIP** kept out of this commit. Canon files staged precisely (`.claude/scripts/session-context.sh`, `.gitignore`, `CLAUDE.md`); the 12 modified WIP files and 2 untracked WIP entries are still in the working tree for the user to commit when ready.
- **Pre-commit chain was the load-bearing fix**: prior to this commit, `pre-commit.intent` existed but `pre-commit` never invoked it -- the critic gate was dormant.

## Decision

**Proceed.** Tenth canary clean. Next: Lamplight.
