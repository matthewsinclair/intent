---
verblock: "24 Apr 2026:v0.50: matts - WP04, WP05, WP06, WP07, WP12 Done (8 of 18)"
intent_version: 2.10.0
---

# Work In Progress

## Current State

**ST0035 (Canonical LLM Config + Fleet Rollout) active — retargeted to v2.10.0 bundling ST0036. WP01–WP10 and WP12 Done (11 of 18).** Intent stamped at `2.10.0`. Canon docs + hooks + headless critic + pre-commit gate + root AGENTS.md generator + root CLAUDE.md overlay template all shipped. Deprecated artefacts removed. **WP10 shipped**: deleted `intent/llm/AGENTS.md` + `lib/templates/llm/_llm_preamble.md`; flipped residual code paths (`intent_init`, `_generate_basic_agents_md`, `intent_doctor`, `intent_claude_upgrade`) to write root AGENTS.md; updated `docs_completeness.bats` idempotency test to point at root. **Retarget**: v2.9.1 → v2.10.0 to bundle ST0036 (directory relocation `.intent/` → `intent/.config/`) as a single breaking release. **ST0036 opened as Phase 0 stub** — implementation WPs interleave between ST0035/WP13 and WP14. **Next up**: WP11 (M — extend `intent claude upgrade --apply` to ship the full canon; needs WP04, WP06, WP07, WP08, WP09 — all ✓).

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
| Not Started | 11  | Extend `intent claude upgrade`                                                        | M    |
| Done        | 12  | Socrates/Diogenes FAQ cross-refs                                                      | XS   |
| Not Started | 13  | Update Intent's own CLAUDE.md                                                         | S    |
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

## Recent

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

1. **WP11** (M) — Extend `intent claude upgrade --apply` to ship the full canon in one shot: `.claude/settings.json` template + hooks scripts, pre-commit hook, `.intent_critic.yml`, root AGENTS.md regeneration, root CLAUDE.md overlay from refreshed template, plus the existing intent/llm/MODULES.md + DECISION_TREE.md. Dependencies (WP04, WP06, WP07, WP08, WP09) all ✓.
2. **WP13** (S) — Rewrite Intent's own `CLAUDE.md` to match the refreshed canon. Depends on WP09 (template) ✓.
3. **ST0036 Phase 0 elaboration** — after WP13 lands: populate all 9 `WP/NN/info.md` files for directory relocation. Phase 0 review gate before any ST0036 WP starts.
4. **WP14** (S) — Self-apply canon to Intent (dogfood). Carries both ST0035 canon AND ST0036 directory relocation post-Phase-0.
5. **WP18** — `intent/usr/*.md` audit can run in parallel with WP15/WP16; must land before WP17.

See `intent/st/ST0035/tasks.md` for ST0035 dependency graph; `intent/st/NOT-STARTED/ST0036/tasks.md` for ST0036.

## Deferred observations

- **Blog draft path**: `docs/blog/_drafts/####-shell-critic-inception.md` (updated from `docs/blog-drafts/` on 2026-04-24). Publication gated on real dogfood runs.
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

_(None — ST0010 and ST0015 cancelled in WP01.)_
