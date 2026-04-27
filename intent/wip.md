---
verblock: "27 Apr 2026:v0.61: matts - WP-15 closed; 11 of 11 in-scope canaries pass; aggregate summary written"
intent_version: 2.10.0
---

# Work In Progress

## Current State

**ST0035 15 of 19 Done. WP-15 closed: all 11 in-scope canaries pass; aggregate summary at `intent/st/ST0035/WP/15/canary-summary.md`.** This session: Conflab + Lamplight canaries committed, WP-15 spec tidied to match as-built reality, fleet summary written, `intent wp done ST0035/15` clean. Canon recipe is mature; canon-installer handles all four common pre-flight patterns automatically (fresh install, stale stamp reset, LEGACY single-file pre-commit, gitignore normalisation). **Next: WP-16 reconciliation** -- 8 of WP-16's original 13 projects were absorbed into WP-15 canary; 4-5 (Multiplyer, MeetZaya, Molt-matts, Courses/Agentic Coding, A3/a3-content) were user-manually upgraded between sessions; Pplr now out of scope per user. WP-16 spec needs the same as-built tidy + fleet summary as WP-15 got. Tests: **791/791 green**. Doctor: clean.

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
| Done        | 15  | Canary rollout across in-scope fleet projects (11 of 11 pass)                         | M    |
| Not Started | 16  | Fleet rollout (Intent ecosystem; Pplr out of scope) -- 12-point checklist per WP-09   | L    |
| Not Started | 17  | Verification sweep + dogfood journal (12-point per project)                           | S    |
| Not Started | 18  | Review and update (or retire) `intent/usr/*.md`                                       | M    |
| Not Started | 19  | Per-language canon: `intent lang init` + `intent init --lang` (added 2026-04-27)      | M    |

## Recent

- **2026-04-27 (this session, post-compact)**: WP-15 closed. Conflab + Lamplight canary reports committed (`e5134ee`); WP-15 spec tidied to match as-built reality + canary aggregate summary written + `intent wp done ST0035/15` clean (`300334d`). Aggregate summary at `intent/st/ST0035/WP/15/canary-summary.md` documents 11/11 pass, scope-expansion narrative, three canon-installer refinements driven by canary findings, and the proceed-to-WP-16 decision.

  Plus housekeeping in the user's global `~/.claude` repo (5 commits): folded `/in-start` orientation into `/in-session` (so a single command does orient + skills + gate); expanded `.gitignore` to cover `projects/` (1.6GB of session transcripts), runtime caches, IDE state, backups; checked in months of curated agents (`critic-{elixir,lua,rust,shell,swift}`, `diogenes`) + the full `in-*` skill suite (22 skills) + `CLAUDE.md` / `settings.json` / `restart.md` / `plugins/blocklist.json` updates. Pushed to `matthewsinclair/cfg-claude` on GitHub.

- **2026-04-27 (this session, batch run)**: WP-15 canary batch -- 7 projects applied, committed, pushed to `local`, reports written. Canaries in order: Molt (`7abd972`), Utilz (`ed31017`), arca_cli (`2e7c14f`), arca_config (`ca85f26`), arca_notionex (`9de67e9`), Prolix (`4508e94`), MicroGPTEx (`b375d1f`). Each: pre-flight reset of stale `.intent/config.json` bump (where present), `intent upgrade` chain `2.9.0 -> 2.10.0` (or `2.8.2 -> 2.10.0` for Utilz), foreign pre-commit chained via marker block, `/AGENTS.md.bak` added to `.gitignore`. All 12-point verifications pass; no canon-installer surprises this batch (the LEGACY single-file path that the previous session added didn't fire on any of these -- they all had foreign pre-commit hooks). Reports at `intent/st/ST0035/WP/15/canary-reports/`.

  Follow-up (same session): `.claude/` gitignore normalisation -- Utilz (`65bbf3e`), arca_notionex (`13f6a71`), MicroGPTEx (`c8889f4`). Was ignoring the entire `.claude/` directory; switched to the Intent canonical pattern (track `.claude/{settings.json,scripts/*,restart.md}`, ignore only `.claude/settings.local.json`). Fresh clones now get the SessionStart / UserPromptSubmit / Stop hooks consistently across the fleet.

  Second follow-up: canon-installer `NORMALIZE_GITIGNORE` action added (`ac4fd9f` in Intent) so the fleet stays uniform automatically. Then applied across the 6 remaining canary projects: Anvil (`77ae65f`), Laksa (`239e9947`), Molt (`a7ccc29`), arca_cli (`86499fb`), arca_config (`f3b5053`), Prolix (`6b87963`) -- each got `.claude/settings.local.json` added (`/AGENTS.md.bak` was already present). All 9 in-scope canaries now show `.gitignore: OK (canon entries present and canonical)` on dry-run.

- **2026-04-27 (this session, post-compact)**: Anvil canary done; canon-installer LEGACY single-file pre-commit migration added; fresh-install path also now produces chained architecture from the start. Two commits in Intent + one in Anvil:
  - `d5b9203` -- canon-installer: new `MIGRATE_LEGACY_PRE_COMMIT` action (detect canon body at `pre-commit` with no `pre-commit.intent` -> mv canon body, write chain stub). `INSTALL_PRE_COMMIT` updated to install chained architecture from the start. +3 new BATS scenarios; fresh-install test asserts chained layout. 788/788 green (was 785).
  - `0724f88` -- Anvil canary report at `intent/st/ST0035/WP/15/canary-reports/anvil.md`.
  - `39c63bd` (in **Anvil**) -- `Intent upgrade to 2.10.0` (user-authored single commit covering canon application + flybys: lazy_html `:only` removal; `Anvil.Projects.create -> create_project` in 4 policy tests for Ash 3.24 compat). mix test 192/192. Pushed to `local`.

- **2026-04-27 (earlier session)**: WP-14 closed; canon-installer hardened; Intent's `intent/llm/RULES.md` + `ARCHITECTURE.md` populated; Laksa canary done. Six commits in order:
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
5. **Canon body lives at `pre-commit.intent`, not `pre-commit`**: the chained architecture is the canonical layout for the entire fleet. Fresh installs produce it directly; legacy single-file installs (canon body at `pre-commit`, no `pre-commit.intent`) are auto-migrated by `MIGRATE_LEGACY_PRE_COMMIT`. Watch for `LEGACY (single-file)` reports on the remaining canaries -- the installer now handles them automatically.

## Next Up

1. **WP-16 reconciliation** (S/M) -- WP-16 originally listed 13 projects; 8 absorbed into WP-15 canary, 4-5 (Multiplyer, MeetZaya, Molt-matts, Courses/Agentic Coding, A3/a3-content) user-manually upgraded between sessions, Pplr now out of scope. Apply the WP-15 closure pattern: tidy spec to match as-built reality, write `fleet-summary.md` documenting outcomes for the user-manual projects (CLI verification of `intent_version` + `intent/.config/` + chain block + .gitignore on each), `intent wp done ST0035/16`. Note: Multiplyer has 1 known test failure (File.ls!/1 in catalog/sources/filesystem.ex:111 against stale fixture); MeetZaya does not compile -- both flagged as user out-of-scope.
2. **ST0035/WP-17 (verification sweep + dogfood journal)** (S) -- per-project journal entries; 12-point matrix becomes 12 = 12 projects \* 12 checks. Capture `.claude/`-gitignored history (Utilz, arca_notionex, MicroGPTEx pre-NORMALIZE_GITIGNORE) + CLAUDE.md drift per project + decide whether to refresh user-section or leave.
3. **ST0035/WP-18 (audit `intent/usr/*.md`)** (M) -- review/update or retire. Parallel with WP-17; must land before WP-17 closes (per WP-17 spec).
4. **ST0035/WP-19 (per-language canon: `intent lang init` + `intent init --lang`)** (M) -- Phase 0 elaborated; implementation independent; ~2-3 sessions.

## Deferred observations

- **CLAUDE.md content drift in canary projects**: pre-existing user CLAUDE.md (from STP-era) is preserved by the canon (correct behaviour). A separate session can refresh by `intent claude upgrade --force` or by hand-editing against `lib/templates/llm/_CLAUDE.md`. Track per-project for the dogfood journal in WP-17.
- **Blog draft path**: `docs/blog/_drafts/####-shell-critic-inception.md`. Publication gated on real dogfood runs (Laksa is the first datapoint).
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

_(None.)_
