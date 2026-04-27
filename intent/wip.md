---
verblock: "27 Apr 2026:v0.56: matts - WP-14 done; canon-installer fixes; Laksa canary 1 of 16"
intent_version: 2.10.0
---

# Work In Progress

## Current State

**ST0035 14 of 19 Done. WP-14 (Intent self-dogfood verification) closed; canon-installer rough edges surfaced by WP-14 fixed (auto-insert chain block; markered idempotence; REVIEW warnings only on verbatim \_default; AGENTS.md footer asymmetry; rule-count rendering glitch). Intent's RULES.md + ARCHITECTURE.md populated (no longer \_default stubs). WP-15 canary in progress -- Laksa first canary done (1 of 16 in-scope projects); Conflab + Lamplight deferred (busy); Pplr out of scope (doesn't need intent).** Tests: **785/785 green** (was 781; +4 new chain-block + REVIEW scenarios). Doctor: clean. Pre-commit chain block now wired in Intent itself (auto-inserted, not manual paste). Backup tag `wp08-pre-relocate` deleted; `.claude/settings.local.json` untracked.

## ST0035 progress

| Status      | WP  | Title                                                                                 | Size |
| ----------- | --- | ------------------------------------------------------------------------------------- | ---- |
| Done        | 01  | Self-upgrade to v2.10.0 + cancel ST0010 / ST0015                                      | XS   |
| Done        | 02  | Refresh root `usage-rules.md`                                                         | S    |
| Done        | 03  | Write `intent/docs/working-with-llms.md`                                              | M    |
| Done        | 04  | `.claude/settings.json` template (SessionStart + UserPromptSubmit strict gate + Stop) | M    |
| Done        | 05  | `bin/intent_critic` headless runner                                                   | L    |
| Done        | 06  | `.git/hooks/pre-commit` template                                                      | S    |
| Done        | 07  | `.intent_critic.yml` default template                                                 | XS   |
| Done        | 08  | Root `AGENTS.md` generator rewrite                                                    | M    |
| Done        | 09  | Root `CLAUDE.md` overlay template                                                     | S    |
| Done        | 10  | Delete deprecated artefacts                                                           | XS   |
| Done        | 11  | Extend `intent claude upgrade` to apply canon artefacts                               | M    |
| Done        | 12  | Socrates/Diogenes FAQ cross-refs                                                      | XS   |
| Done        | 13  | Update Intent's own CLAUDE.md                                                         | S    |
| Done        | 14  | Self-apply canon to Intent (dogfood; verification sweep post-WP08)                    | S    |
| WIP         | 15  | Canary rollout (Laksa done; Conflab + Lamplight deferred -- busy)                     | M    |
| Not Started | 16  | Fleet rollout (Intent ecosystem; Pplr out of scope) -- 12-point checklist per WP-09   | L    |
| Not Started | 17  | Verification sweep + dogfood journal (12-point per project)                           | S    |
| Not Started | 18  | Review and update (or retire) `intent/usr/*.md`                                       | M    |
| Not Started | 19  | Per-language canon: `intent lang init` + `intent init --lang` (added 2026-04-27)      | M    |

## Recent

- **2026-04-27 (this session)**: WP-14 closed; canon-installer hardened; Intent's `intent/llm/RULES.md` + `ARCHITECTURE.md` populated; Laksa canary done. Six commits in order:
  - `9a6387b` -- WP-14 Intent self-dogfood verification (dry-run + apply + 12-point + reports under `intent/st/ST0035/WP/14/`).
  - `9315bb6` -- canon-installer rough edges surfaced by WP-14: AGENTS.md generator footer (no more linter oscillation), RULES.md count rendering, REVIEW warnings only on verbatim \_default, real chain-block detection (markers + helper), auto-insert chain block in Phase 3 (idempotent). +4 new BATS scenarios (785/785).
  - `d0d0dc6` -- populate Intent's RULES.md + ARCHITECTURE.md (no longer verbatim \_default; canon-installer's REVIEW warning is now silent for Intent).
  - `f5d9df9` -- housekeeping: untrack `.claude/settings.local.json` (per-developer noise); gitignore `/AGENTS.md.bak`.
  - `a729ec64` (in Laksa) -- `chore: apply ST0035 + ST0036 canon (v2.10.0 rollout canary)`. 2.8.2 -> 2.10.0 chain in one pass; pushed to `local`.
  - `2e90556` -- Laksa canary report at `intent/st/ST0035/WP/15/canary-reports/laksa.md`.

  Plus: backup tag `wp08-pre-relocate` deleted; `.git/hooks/pre-commit` chain block auto-inserted in Intent itself (live test confirmed during commit cycle).

- **Previous session (2026-04-27 morning)**: see `intent/restart.md` -- ST0036 closed; 9 of 9 Done.
- **2026-04-25**: see `intent/st/ST0035/` for the ST0035 history through WP-13.

## Resolved decisions (this session)

1. **Auto-insert beats manual paste**: canon-installer's `CHAIN_PRE_COMMIT` now auto-inserts the markered chain block rather than printing a snippet for the user to paste. Marker pair (`intent-chain-block:start/end`) makes re-application a guaranteed no-op. Insert location: after shebang + leading `set -*` lines, before the body.
2. **REVIEW warnings only on verbatim \_default**: per the canon-installer, RULES.md / ARCHITECTURE.md "REVIEW" reminders now fire only when the project file is byte-identical to the `_default` template. Customised content suppresses the warning (so legitimate downstream projects aren't nagged).
3. **Per-developer settings.local.json should not be tracked**: paths and tool allowlists vary by user. Gitignored + removed from index. `/AGENTS.md.bak` similarly gitignored (regen safety net).
4. **Laksa is "canary 1 of 16", not strictly "canary 1 of 3"**. Per user, Conflab and Lamplight are busy and Pplr is out of scope. Strategy: do other fleet projects one at a time; come back to Conflab/Lamplight when free; switch to batch mode once enough confidence accumulates.

## Next Up

1. **ST0035/WP-15 (next canary)** -- per user direction, "do other projects one at a time before Conflab/Lamplight". Candidates: **Molt**, **Utilz**, **Arca**, **Prolix**, **MicroGPTEx**, **Sites** (Pplr explicitly out of scope; doesn't need intent). Pick the next one and apply the same recipe (`intent upgrade`, 12-point verify, commit + push to `local`, write `intent/st/ST0035/WP/15/canary-reports/<project>.md`).
2. **WP-15 spec tidy-up** (S) -- before more canaries: fix `intent upgrade --dry-run` reference (doesn't exist), drop the "Sites subdir" check (Laksa doesn't have one), drop Pplr (out of scope), and clarify that the canary is now 16 projects, not 3.
3. **ST0035/WP-16/WP-17/WP-18** -- queued per existing plan.
4. **ST0035/WP-19** -- Phase 0 elaborated; implementation independent; M (2-3 sessions).

## Deferred observations

- **CLAUDE.md content drift in canary projects**: pre-existing user CLAUDE.md (from STP-era) is preserved by the canon (correct behaviour). A separate session can refresh by `intent claude upgrade --force` or by hand-editing against `lib/templates/llm/_CLAUDE.md`. Track per-project for the dogfood journal in WP-17.
- **Blog draft path**: `docs/blog/_drafts/####-shell-critic-inception.md`. Publication gated on real dogfood runs (Laksa is the first datapoint).
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

_(None.)_
