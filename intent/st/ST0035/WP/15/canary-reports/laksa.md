---
verblock: "27 Apr 2026:v0.1: matts - Laksa canary report"
project: Laksa
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: a729ec64
---

# Laksa canary report

## Outcome

**Pass.** First canary application of the v2.10.0 canon. Single-pass migration via `intent upgrade` chained 2.8.2 -> 2.9.0 -> 2.10.0 cleanly. All 12 verification points pass. Committed (`a729ec64`); pushed to `local` remote.

## Pre-flight

- **Pre-flight cleanup**: working tree had a stale manual bump of `.intent/config.json` from `intent_version: 2.8.2 -> 2.9.0` (unstaged). Reset to HEAD so the migration could write the canonical `2.10.0` value end-to-end. No data loss; the migration writes 2.10.0 over 2.8.2 directly via the chained migration.
- **HEAD before**: `e5ef078a ST0069: as-built docs after WP-01 ship`. One commit ahead of `local/main` (push picked up both that commit and our canary commit).
- **Doctor (pre)**: clean (Intent CLI v2.10.0 against Laksa v2.8.2; all checks ok).
- **Branch**: `main`. Remotes: `local` (Dropbox), `upstream` (GitHub). Per WP-15 protocol, pushed to `local` only; `upstream` deferred.

## Dry-run summary

`intent upgrade --dry-run` does not exist (the WP-15 spec was speculative; the migrator is `intent upgrade` invoking `migrate_v2_9_0_to_v2_10_0` which is itself non-destructive on stamp-current projects). Used `intent claude upgrade` (canon-installer dry-run) instead, which surfaced the full canon-apply plan. See `/tmp/laksa-canon-dry.txt` for full output.

Key actions enqueued:

```
1. REGENERATE AGENTS.md (currently unknown, target 2.10.0)
2-5. Install .claude/{settings.json, scripts/session-context.sh, require-in-session.sh, post-tool-advisory.sh}
6. Install .git/hooks/pre-commit.intent and chain it from existing pre-commit (idempotent block)
7. Install .intent_critic.yml
8. Install root usage-rules.md
9. Install intent/.treeindex/.treeindexignore
10. Delete legacy intent/llm/AGENTS.md
```

Manual review flagged: `! No intent/.config/config.json found -- cannot update version`. This was expected -- the canon-installer's dry-run ran on the pre-relocation layout. After `intent upgrade` performs the relocation, the canonical config is in place.

Manual review also flagged: `! CLAUDE.md at root is user-authored (no Intent footer marker); not refreshed.` Correct behavior -- the canon respects authorship. Refresh deferred (would require `--force` or manual reconciliation against `lib/templates/llm/_CLAUDE.md`).

## Apply summary

`intent upgrade` ran the full chain:

- **Phase 1 (relocate)**: `.intent/` -> `intent/.config/` via the migration function. Git correctly detected the rename (3 files: config.json, last-health-check, learnings.md).
- **Phase 2 (stamp re-target)**: `intent_version: 2.8.2 -> 2.10.0`.
- **Phase 3 (canon-apply)**: 11 actions executed cleanly (regen AGENTS.md, install 4 .claude/ artefacts, install pre-commit.intent + auto-insert chain block, install .intent_critic.yml, install root usage-rules.md, install .treeindexignore, delete legacy intent/llm/AGENTS.md).

Backup written to `.backup/backup-20260427-130317`. Final stamp confirmed `2.10.0`. See `/tmp/laksa-upgrade.txt` for full output.

## 12-point verification

| #   | Check                                                                        | Result |
| --- | ---------------------------------------------------------------------------- | ------ |
| 1   | `jq -r .intent_version intent/.config/config.json` -> `2.10.0`               | ok     |
| 2   | `[ -f AGENTS.md ] && [ ! -L AGENTS.md ]`                                     | ok     |
| 3   | `[ ! -e intent/llm/AGENTS.md ]`                                              | ok     |
| 4   | `[ -f usage-rules.md ]`                                                      | ok     |
| 5   | `jq -r '.hooks \| keys[]' .claude/settings.json` returns 3 hooks             | ok     |
| 6   | `[ -x .git/hooks/pre-commit ]` and `[ -x .git/hooks/pre-commit.intent ]`     | ok     |
| 7   | `[ -f .intent_critic.yml ]`                                                  | ok     |
| 8   | `[ -x .claude/scripts/session-context.sh ]`                                  | ok     |
| 9   | `grep -qF 'intent-chain-block:start' .git/hooks/pre-commit`                  | ok     |
| 10  | `bin/intent_critic --help` reachable; commit pre-commit hook fired the chain | ok     |
| 11  | `[ -d intent/.config ]`                                                      | ok     |
| 12  | `[ ! -e .intent ]`                                                           | ok     |

Hook 10 was confirmed live: `git commit` invoked the prettier formatter, then the chain block called `pre-commit.intent` -> `intent_critic`, which ran and returned 0 (no findings at severity >= warning). Commit landed cleanly.

## Project-specific notes

- **Sites subdir** (per WP-15 step 12): does not exist on disk at canary time. Check N/A. WP-15 spec was apparently based on stale info.
- **`.claude/skills/`** (Conflab-specific check, but worth noting): does not exist in Laksa either. No skills to preserve.
- **CLAUDE.md content drift**: existing CLAUDE.md is the v2.8.0 STP-era text ("This is an Intent v2.8.0 project (formerly STP)..."). Canon respects authorship and did not modify it. A separate session can refresh by running `intent claude upgrade --force` or by hand-editing against `lib/templates/llm/_CLAUDE.md`.
- **`.gitignore`**: added `/AGENTS.md.bak` (transient regen safety net) to align with Intent's housekeeping.

## Issues encountered

None blocking. Three observations, all benign:

1. **Stale manual version bump** (pre-flight): cleared by reset; no data loss.
2. **WP-15 spec drift**: `intent upgrade --dry-run` doesn't exist; Sites subdir doesn't exist either. Both worth fixing in WP-15 info.md before next canary, but neither blocks Laksa.
3. **CLAUDE.md drift**: pre-existing user content references v2.8.0; preserved by design. Refresh is a separate user choice, not a canon-apply concern.

## Decision

**Proceed.** First canary clean. Next canary: choose between Conflab or Lamplight (both currently busy per user). User asked to "do other projects one at a time" before returning to Conflab/Lamplight. Recommend picking the next available project (Pplr, Molt, Utilz, Arca, Prolix, MicroGPTEx, Sites) per WP-16 fleet list, treating each as a mini-canary until enough confidence accumulates to switch to batch mode for the rest.
