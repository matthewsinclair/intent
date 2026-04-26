---
verblock: "26 Apr 2026:v0.54: matts - ST0036 7 of 9 Done (WP04-WP06 + new mig BATS); WP08 in working tree"
intent_version: 2.10.0
---

# Work In Progress

## Current State

**ST0035 active -- 13 of 18 Done (WP01-WP13). 5 remain: WP14, WP15, WP16, WP17, WP18.** **ST0036 active -- 7 of 9 Done (WP01-WP07).** WP08 (Intent self-apply) half-done in the working tree: a manual `mv .intent intent/.config` was performed during WP05 as a diagnostic (to validate that no test had hard-coded `.intent/` literals depending on Intent's old layout). The diagnostic surfaced 0 such bugs; only the per-test fixture/assertion flips remained, which WP05 covered. The rename is uncommitted; WP08 next session will revert it and re-do via `intent upgrade` so the canon-apply Phase 3 also lands. Tests: **774/774 green**. Doctor: clean (verified mid-WP05).

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
| Not Started | 14  | Self-apply canon to Intent (dogfood)                                                  | S    |
| Not Started | 15  | Canary rollout (Conflab, Lamplight, Laksa)                                            | M    |
| Not Started | 16  | Fleet rollout (12 Intent + Pplr)                                                      | L    |
| Not Started | 17  | Verification sweep + dogfood journal                                                  | S    |
| Not Started | 18  | Review and update (or retire) `intent/usr/*.md`                                       | M    |

## Resolved decisions

1. **Version**: 2.10.0 (retargeted from 2.9.1 mid-ST to bundle ST0036 directory relocation).
2. **Hook enforcement**: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. User will reassess intrusiveness post-rollout.
3. **Pre-commit critic threshold**: CRITICAL + WARNING blocks (`--warnings-are-errors` posture).
4. **PostToolUse advisory critic**: off by default (too noisy + too costly in tokens). Helper script ships; opt-in via `.intent_critic.yml post_tool_use_advisory: true` + user adds the stanza to `.claude/settings.local.json`.
5. **Cancelled STs** go to `intent/st/CANCELLED/`; deprecation annotations inline in the cancelled ST's `info.md`.

## ST0036 progress

| Status      | WP  | Title                                               | Size |
| ----------- | --- | --------------------------------------------------- | ---- |
| Done        | 01  | Migration function + atomic relocation              | M    |
| Done        | 02  | Path probes flip                                    | S    |
| Done        | 03  | Literal sweep `.intent/` -> `intent/.config/`       | M    |
| Done        | 04  | Template + generator updates                        | S    |
| Done        | 05  | BATS fixtures + new migration test scenarios        | M    |
| Done        | 06  | Ignore patterns + `_treeindexignore` template       | XS   |
| Done        | 07  | v2.10.0 migration guide                             | XS   |
| Not Started | 08  | Intent self-apply (canon-apply phase still pending) | S    |
| Not Started | 09  | Coordination notes into ST0035/WP15-17              | XS   |

## Recent

- **2026-04-26 (this session)**: **ST0036/WP-04, WP-05, WP-06 Done**, plus the WP05 diagnostic that doubled as a half-done WP08. Five session commits:
  - `32df058` -- WP-06 ignore patterns + new `lib/templates/_treeindexignore` template; canon installer ships it via new `INSTALL_TREEINDEXIGNORE` action; Highlander cleanup of the inline heredoc in `bin/intent_treeindex::ensure_treeindexignore`.
  - `5f8b61e` -- WP-04 templates + generators audit. Single material flip: `lib/templates/hooks/pre-commit.sh` (4 hits where the hook probed `.intent/config.json`). All other findings were `~/.intent/ext/` (user-level, KEEP).
  - `b62ea58` -- WP-05 BATS work. 11 BATS files flipped (assertions + fixtures + the test name in `fileindex_commands.bats`); new `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` (6 scenarios -- clean migration, idempotence, sentinel/symlink/conflict refusals, cross-FS placeholder); doctor sentinel scenario in `global_commands.bats`. Real bug fixed: macOS BSD `mktemp /tmp/foo-XXXXXX.md` creates the LITERAL file `foo-XXXXXX.md` (X's not substituted when followed by a suffix); `agents_sync_idempotent` was silently broken on macOS by this until the second run collided with "File exists". Fixed by dropping the `.md` suffix.
  - The user proposed and approved a diagnostic `mv .intent intent/.config` on Intent's own repo before doing the per-test BATS flips. Result was crisp: **+1 passing, 0 newly broken**. The +1 was test 374 (`intent critic dispatches to bin/intent_critic`) which had been failing because Intent's CLI rejected Intent's own repo as not-a-project. The 26 remaining failures were all per-test BATS fixture/assertion flips, none of which depended on Intent's own state. WP02 path probes proven correct end-to-end. The rename remains uncommitted in the working tree as a half-done WP08; tomorrow we revert + invoke `intent upgrade` to do it properly.
- **2026-04-26**: **ST0036 Phase 0 elaborated** (commit pending). All nine `WP/NN/info.md` files populated under `intent/st/ST0036/WP/01..09/`; total ~1072 lines of forensic detail. T-shirt sizing finalised: 3 × M (WP01 migration function, WP03 literal sweep, WP05 BATS), 3 × S (WP02 path probes, WP04 templates, WP08 self-apply), 3 × XS (WP06 ignore patterns, WP07 migration guide, WP09 fleet-rollout coord). Recurring concerns surfaced cross-WP and locked into specifications: WP01 idempotence on layout state (not stamp value, since Intent itself is already stamped 2.10.0 but not yet relocated); sentinel anchor coordination between WP01 diagnostic and WP07 migration-guide doc; `~/.intent/ext/` preservation discipline across all literal-flip WPs; AGENTS.md regen handling per session-3 conventions. ST0036 now status WIP via `intent st start 36`. **Phase 0 gate: user review before WP01 starts.**
- **2026-04-26**: **ST0035/WP-13 Done** (commit `66f6793`). Rewrote Intent's own root `CLAUDE.md` from 236-line hand-authored developer guide to 96-line Claude-specific overlay following the WP09 template (`lib/templates/llm/_CLAUDE.md`). Canon shell matches the template byte-for-byte (after `[[PROJECT_NAME]] / [[INTENT_VERSION]] / [[AUTHOR]] / [[DATE]]` substitution); all Intent-specific content lives between `<!-- user:start --> / <!-- user:end -->` markers (6 numbered dev rules with #1-3 marked as concretisations of the agnostic IN-AG-\* rules, Intent-specific files pointer, internal authoring docs pointer to satisfy `tests/unit/docs_completeness.bats`, commit conventions, compressed migration history, author block). Pulled out: project structure, ST methodology, command tables, rules-library/critic/extension narratives, treeindex section, agent narratives, v2.8.2 -> v2.9.0 migration block -- all routed through `AGENTS.md` / `usage-rules.md` / `working-with-llms.md`. The upgrade tool's dry-run on Intent now reports `CLAUDE.md (root): REFRESH (user section preserved)` with only the dynamic `[[DATE]]` placeholder differing -- exactly the canon round-trip behaviour. WP14 self-apply will refresh the date and preserve every line of the user section.
- **2026-04-25**: **WP-11 Done.** Sessions 2 + 3 shipped on top of Session 1's install machinery. Session 2 (`1db2b44`): `--force` flag (overwrite user-edited `.intent_critic.yml` / `usage-rules.md` / user-authored CLAUDE.md, with banner warning + per-probe OVERWRITE marker); `--skip-settings` flag (skip `.claude/settings.json` + 3 hook scripts); paste-ready multi-line `CHAIN_PRE_COMMIT` snippet using `git rev-parse --git-path hooks` so the chain works in worktrees and submodules; diff-in-dry-run for `REFRESH_CLAUDE_MD` (capped at 60 lines); worktree-aware hook resolution via new `canon_hooks_dir` helper; upfront writability probe on `--apply`. Inline bug fixes: `canon_compute_refresh_preview` now stages preserved user content in a temp file (command substitution was stripping trailing blank lines, causing freshly-installed CLAUDE.md to immediately report DIVERGED). Session 3 (`b2a6e5d`): BATS suite at `tests/unit/intent_claude_upgrade.bats` covering all 5 spec scenarios with HOME isolation; MODULES.md row updated for WP-11 scope; `canon_print` helper unifies diagnostic alignment (status column lands at col 43 for every artefact line, regardless of label length). Bonus fix `614980d`: `intent init /abs/path` now works (was crashing sed because the path leaked into PROJECT_NAME). Tests: 767/767 green. Commits: `b2a6e5d` Session 3 + Done · `1db2b44` Session 2 · `614980d` init fix.
- **2026-04-24**: **WP-11 Session 1 shipped.** 7 canon-install helpers in `intent_claude_upgrade` (`canon_install_file`, `canon_install_script`, `canon_delete_file`, `canon_refresh_with_user_section`, `canon_substitute_placeholders`, `canon_template_matches_installed`, plus supporting primitives). 11 new action codes wired through Phases 1/2/3 (INSTALL_SETTINGS, INSTALL_HOOK_SCRIPT, INSTALL_PRE_COMMIT, CHAIN_PRE_COMMIT, INSTALL_CRITIC_CONFIG, INSTALL_CLAUDE_MD, REFRESH_CLAUDE_MD, INSTALL_USAGE_RULES, PLANT_MODULES, PLANT_DECISION_TREE, DELETE_LEGACY_AGENTS). REGENERATE/CREATE AGENTS.md now calls `intent agents sync` (the WP-08 generator) instead of copying the Elixir template. `migrate_v2_9_0_to_v2_10_0` invokes `intent claude upgrade --apply` after stamp bump. Two idempotence bug fixes: AGENTS.md multi-digit-semver regex; placeholder-aware compare for hook scripts. Scratch-project test: fresh install all-green; second `--apply` zero actions. Full suite 762/762 green. Commit `e999f82`. Sessions 2+3 deferred (edge cases + BATS + MODULES.md + Done).
- **2026-04-24**: WP-10 complete. Deleted `intent/llm/AGENTS.md` + `lib/templates/llm/_llm_preamble.md`. Flipped residual code paths that still wrote to `intent/llm/AGENTS.md`: `bin/intent_init::_create_basic_agents_md`, `bin/intent_helpers::_generate_basic_agents_md` + `migrate_v2_2_to_v2_3_0`, `bin/intent_doctor` (AGENTS.md check simplified), `intent/plugins/claude/bin/intent_claude_upgrade` (diagnosis + CREATE + REGENERATE paths flipped), `tests/unit/docs_completeness.bats::agents_sync_idempotent` (path to root). `intent/docs/working-with-llms.md` troubleshooting bumped to v2.10.0. Full suite 762/762 green. Commits: `1ae5f61` content · `2e99857` Done.
- **2026-04-24**: **Retarget ST0035 v2.9.1 → v2.10.0** + **ST0036 opened (Phase 0 stub)**. Mid-ST decision to bundle directory relocation (`.intent/` → `intent/.config/`) into the same release. Semver-breaking directory move forces the minor bump within 2.x. ST0036 lands its implementation WPs between ST0035/WP13 and ST0035/WP14; canary + fleet rollout (WP15/WP16) carry both concerns in one pass. Commits: `b760b39` retarget.
- **2026-04-24**: WP-09 complete. `lib/templates/llm/_CLAUDE.md` rewritten as a short overlay (58 lines) pointing at root AGENTS.md as primary contract. Canon placeholders preserved. 12 new BATS scenarios covering length budget, landmarks, placeholder substitution via `intent init`. Commits: `d3c147d` content · `09cad07` Done.
- **2026-04-24**: WP-08 complete. `intent agents sync/init/validate` flip to root `AGENTS.md` (real file, not symlink). 16 canon sections; dynamic skills/subagents rendering from `.claude/`; symlink-migration helper. `templates/default.md` deleted (Highlander). Pre-existing source-tree bug fixed (renderer now reads per-project `.claude/` not Intent canon). 12 new BATS scenarios. Full suite 750/750 green. Commits: `546dc3d` content · `61fad69` Done.
- **2026-04-24**: WP-06 complete. `lib/templates/hooks/pre-commit.sh` runs `intent critic <lang> --staged --severity-min <sev>` per detected language; blocks on findings, fails open on missing tooling. `intent/docs/pre-commit-hook.md` covers install/configure/opt-out/CI/troubleshooting (6 entries). 9 BATS scenarios green (scratch-repo end-to-end: bad blocks, good passes, --no-verify bypass, missing intent fail-open, non-Intent repo fail-open, severity_min honoured, disabled honoured, syntax valid, template executable). Commit `c994579`.
- **2026-04-24**: WP-07 complete. `lib/templates/_intent_critic.yml` with commented canonical defaults (severity_min: warning, disabled: [], post_tool_use_advisory: false). critics.md schema table extended. Caught + fixed WP-05 bug during schema review: `critic_rule_disabled` was matching every rule because awk's match-exit-0 collided with natural-exit-0; now uses exit code 10. Renamed field from nonstandard `disabled_rules:` to canonical `disabled:` (matches intent/docs/critics.md and sample-intent-critic.yml). 2 regression BATS tests added. Commit `0b0d72d`.
- **2026-04-24**: WP-05 complete. `bin/intent_critic` — headless runner for the mechanical subset of the rule library (Greppable-proxy rules). `rules_lib.sh` extracted from `intent_claude_rules` (Highlander refactor — both CLIs now source the same primitives). `critic_runner.sh` implements Detection extraction, rule application with disabled honouring, text + JSON report formatters. `intent/docs/critics.md` documents the headless-runner surface. 15 new BATS tests green; 62 tests total across 7 suites pass without regression. Self-critique of `bin/intent_critic` under shell rules: 0 critical findings. Commit `c47fbfc`.
- **2026-04-24**: WP-12 complete. Socrates/Diogenes FAQ cross-refs in `intent/plugins/claude/subagents/socrates/agent.md` and `diogenes/agent.md` pointing at `intent/docs/working-with-llms.md#socrates-vs-diogenes-faq`. Commit `c01d9fe`.
- **2026-04-24**: WP-04 complete. `lib/templates/.claude/settings.json` shipped with three default hook stanzas (SessionStart + strict UserPromptSubmit + Stop; no PostToolUse in default). Three helper scripts at `lib/templates/.claude/scripts/` (`session-context.sh`, `require-in-session.sh`, `post-tool-advisory.sh`) — all executable, all green. `/in-session` SKILL.md step 4 writes cooperating sentinel at `/tmp/intent/in-session-${session_id}.sentinel`; require-in-session.sh passes through slash commands so `/in-session` itself can run. MODULES.md registers template + 3 scripts + 1 BATS. New BATS test covers git+wip / git-only / no-git / session-id-capture — 5/5 green; existing `in_session_skill.bats` still 10/10. Commit `e36b6f1`.
- **2026-04-24**: WP-18 added to ST0035 (late addition per user request). Review `intent/usr/*.md` (user_guide 877L, reference_guide 1370L, deployment_guide 619L — all pre-v2.9.0) and apply keep / update / throw before v2.10.0 release. `WP/18/info.md` populated Phase 0-style; tasks.md + ST0035/info.md WP table updated; WP17 now gated on WP16 + WP18. Commit `b6fc2fe`.
- **2026-04-24**: WP03 complete. New `intent/docs/working-with-llms.md` (459 lines, D1–D10 as H2, ASCII arch diagram, hooks JSON, critic cadence, Socrates/Diogenes FAQ with commits `7f4529e` + `37a0ed0`, seven troubleshooting gotchas). Cross-refs in README.md ("For LLM Collaboration" section) and MODULES.md registration. Commits: `983ffdb` content · `b148ac0` Done.
- **2026-04-24**: WP02 complete. Root `usage-rules.md` refreshed to v2.9.0+ surface; `lib/templates/llm/_usage-rules.md` template added with `[[PROJECT_NAME]] / [[INTENT_VERSION]] / [[LANG]]` placeholders; MODULES.md registers the template. Commits: `4e75ebd` content · `357e0c4` Done.
- **2026-04-24**: ST0035 opened and populated. Phase 0 forensic elaboration across `info.md`, `design.md` (10 canon decisions D1–D10 + risk register), `tasks.md`, 17 × `WP/NN/info.md`. Decisions resolved 2026-04-24. WP01 complete: VERSION → 2.9.1, `.intent/config.json` stamped, migration chain extended, ST0010 + ST0015 cancelled to `intent/st/CANCELLED/`. Commits: `055a7e4` Phase 0 · `b265987` decisions resolved · `567d5d1` WP01 · `1472cca` WP01 marked Done.
- **2026-04-23 to 2026-04-24**: `critic-shell` dogfood on Intent's own bash. WP12 dogfood journal Entry 1 complete. Commits: `a9ee349` P0/P1 · `0de89cd` P2 sweep · `60dfcd6` prompt-fix.
- **2026-04-23**: v2.9.0 released + fleet rollout complete. ST0034 closed. Tag `v2.9.0` on both remotes; GitHub release published.

## Next Up

1. **ST0036/WP08 finalisation (S)** -- the moment of truth. Working tree currently has the manual `mv` half. Steps:
   1. Revert the manual rename (`mv intent/.config .intent`) so Intent goes back to the v2.9.x layout in the working tree.
   2. Run `intent upgrade` to invoke `migrate_v2_9_0_to_v2_10_0` properly: relocate (Phase 1) + stamp re-target (Phase 2, no-op since already 2.10.0) + canon-apply (Phase 3, will REFRESH a few cosmetic-drift files).
   3. Verify post-state: `tests/run_tests.sh` (expect 774 green), `intent doctor` (expect clean), `git log --follow intent/.config/config.json` (expect rename-preserving history).
   4. Commit the rename + canon-apply diff together as the WP08 commit.
2. **ST0036/WP09 (XS)** -- coordination doc updates into `ST0035/WP15-17` info.md and ST0036/impl.md finalisation. Pure docs.
3. **ST0035/WP14 (S)** -- Self-apply canon to Intent (dogfood). Now runs on a relocated repo (post-WP08). Most of the canon-apply work will have already been done by WP08's Phase 3, so WP14 is a verification sweep + commit cleanup.
4. **ST0035/WP15-17** -- Canary (Conflab -> Lamplight -> Laksa) + fleet rollout (12 Intent + Pplr) + verification sweep.
5. **ST0035/WP18 (M)** -- `intent/usr/*.md` audit (parallel with WP15/WP16; must land before WP17).

See `intent/st/ST0035/tasks.md` for ST0035 dependency graph; `intent/st/NOT-STARTED/ST0036/tasks.md` for ST0036.

## Deferred observations

- **Blog draft path**: `docs/blog/_drafts/####-shell-critic-inception.md` (updated from `docs/blog-drafts/` on 2026-04-24). Publication gated on real dogfood runs.
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

_(None — ST0010 and ST0015 cancelled in WP01.)_
