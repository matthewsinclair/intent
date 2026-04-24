---
verblock: "24 Apr 2026:v0.50: matts - WP04, WP05, WP06, WP07, WP12 Done (8 of 18)"
intent_version: 2.9.1
---

# Work In Progress

## Current State

**ST0035 (Canonical LLM Config + Fleet Rollout) active. WP01–WP07 and WP12 Done (8 of 18).** Intent stamped at `2.9.1`. Canon docs + hooks + headless critic + pre-commit gate + config template all shipped. **WP06 shipped**: `lib/templates/hooks/pre-commit.sh` — scans staged files per detected language (`mix.exs`/`Cargo.toml`/etc.; shell always included), runs `intent critic <lang> --staged --severity-min <sev>`, blocks on findings. Fail-open on missing `git`/`intent`/`.intent/config.json`. Reads `severity_min` from `.intent_critic.yml`. `intent/docs/pre-commit-hook.md` covers install/configure/opt-out/CI/troubleshooting. 9 new BATS tests green (scratch-repo end-to-end). **WP07 shipped**: `lib/templates/_intent_critic.yml` with canonical defaults (`severity_min: warning`, `disabled: []`, `post_tool_use_advisory: false`). `critics.md` schema table extended. **Bonus**: caught + fixed a critical WP-05 bug — `critic_rule_disabled()` returning 0 for every rule (awk natural-exit-0 colliding with match-exit-0); now uses exit code 10 to distinguish, field name aligned with canon `disabled:`. 2 new regression BATS tests. **WP18 still open** (intent/usr/\*.md audit). **Next up**: WP08 (M — root AGENTS.md generator rewrite; needs WP03 ✓) + WP09 (S — root CLAUDE.md template; needs WP08).

## ST0035 progress

| Status      | WP  | Title                                                                                 | Size |
| ----------- | --- | ------------------------------------------------------------------------------------- | ---- |
| Done        | 01  | Self-upgrade to v2.9.1 + cancel ST0010 / ST0015                                       | XS   |
| Done        | 02  | Refresh root `usage-rules.md`                                                         | S    |
| Done        | 03  | Write `intent/docs/working-with-llms.md`                                              | M    |
| Done        | 04  | `.claude/settings.json` template (SessionStart + UserPromptSubmit strict gate + Stop) | M    |
| Done        | 05  | `bin/intent_critic` headless runner                                                   | L    |
| Done        | 06  | `.git/hooks/pre-commit` template                                                      | S    |
| Done        | 07  | `.intent_critic.yml` default template                                                 | XS   |
| Not Started | 08  | Root `AGENTS.md` generator rewrite                                                    | M    |
| Not Started | 09  | Root `CLAUDE.md` overlay template                                                     | S    |
| Not Started | 10  | Delete deprecated artefacts                                                           | XS   |
| Not Started | 11  | Extend `intent claude upgrade`                                                        | M    |
| Done        | 12  | Socrates/Diogenes FAQ cross-refs                                                      | XS   |
| Not Started | 13  | Update Intent's own CLAUDE.md                                                         | S    |
| Not Started | 14  | Self-apply canon to Intent (dogfood)                                                  | S    |
| Not Started | 15  | Canary rollout (Conflab, Lamplight, Laksa)                                            | M    |
| Not Started | 16  | Fleet rollout (12 Intent + Pplr)                                                      | L    |
| Not Started | 17  | Verification sweep + dogfood journal                                                  | S    |
| Not Started | 18  | Review and update (or retire) `intent/usr/*.md`                                       | M    |

## Resolved decisions

1. **Version**: 2.9.1.
2. **Hook enforcement**: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. User will reassess intrusiveness post-rollout.
3. **Pre-commit critic threshold**: CRITICAL + WARNING blocks (`--warnings-are-errors` posture).
4. **PostToolUse advisory critic**: off by default (too noisy + too costly in tokens). Helper script ships; opt-in via `.intent_critic.yml post_tool_use_advisory: true` + user adds the stanza to `.claude/settings.local.json`.
5. **Cancelled STs** go to `intent/st/CANCELLED/`; deprecation annotations inline in the cancelled ST's `info.md`.

## Recent

- **2026-04-24**: WP-06 complete. `lib/templates/hooks/pre-commit.sh` runs `intent critic <lang> --staged --severity-min <sev>` per detected language; blocks on findings, fails open on missing tooling. `intent/docs/pre-commit-hook.md` covers install/configure/opt-out/CI/troubleshooting (6 entries). 9 BATS scenarios green (scratch-repo end-to-end: bad blocks, good passes, --no-verify bypass, missing intent fail-open, non-Intent repo fail-open, severity_min honoured, disabled honoured, syntax valid, template executable). Commit `c994579`.
- **2026-04-24**: WP-07 complete. `lib/templates/_intent_critic.yml` with commented canonical defaults (severity_min: warning, disabled: [], post_tool_use_advisory: false). critics.md schema table extended. Caught + fixed WP-05 bug during schema review: `critic_rule_disabled` was matching every rule because awk's match-exit-0 collided with natural-exit-0; now uses exit code 10. Renamed field from nonstandard `disabled_rules:` to canonical `disabled:` (matches intent/docs/critics.md and sample-intent-critic.yml). 2 regression BATS tests added. Commit `0b0d72d`.
- **2026-04-24**: WP-05 complete. `bin/intent_critic` — headless runner for the mechanical subset of the rule library (Greppable-proxy rules). `rules_lib.sh` extracted from `intent_claude_rules` (Highlander refactor — both CLIs now source the same primitives). `critic_runner.sh` implements Detection extraction, rule application with disabled honouring, text + JSON report formatters. `intent/docs/critics.md` documents the headless-runner surface. 15 new BATS tests green; 62 tests total across 7 suites pass without regression. Self-critique of `bin/intent_critic` under shell rules: 0 critical findings. Commit `c47fbfc`.
- **2026-04-24**: WP-12 complete. Socrates/Diogenes FAQ cross-refs in `intent/plugins/claude/subagents/socrates/agent.md` and `diogenes/agent.md` pointing at `intent/docs/working-with-llms.md#socrates-vs-diogenes-faq`. Commit `c01d9fe`.
- **2026-04-24**: WP-04 complete. `lib/templates/.claude/settings.json` shipped with three default hook stanzas (SessionStart + strict UserPromptSubmit + Stop; no PostToolUse in default). Three helper scripts at `lib/templates/.claude/scripts/` (`session-context.sh`, `require-in-session.sh`, `post-tool-advisory.sh`) — all executable, all green. `/in-session` SKILL.md step 4 writes cooperating sentinel at `/tmp/intent/in-session-${session_id}.sentinel`; require-in-session.sh passes through slash commands so `/in-session` itself can run. MODULES.md registers template + 3 scripts + 1 BATS. New BATS test covers git+wip / git-only / no-git / session-id-capture — 5/5 green; existing `in_session_skill.bats` still 10/10. Commit `e36b6f1`.
- **2026-04-24**: WP-18 added to ST0035 (late addition per user request). Review `intent/usr/*.md` (user_guide 877L, reference_guide 1370L, deployment_guide 619L — all pre-v2.9.0) and apply keep / update / throw before v2.9.1 release. `WP/18/info.md` populated Phase 0-style; tasks.md + ST0035/info.md WP table updated; WP17 now gated on WP16 + WP18. Commit `b6fc2fe`.
- **2026-04-24**: WP03 complete. New `intent/docs/working-with-llms.md` (459 lines, D1–D10 as H2, ASCII arch diagram, hooks JSON, critic cadence, Socrates/Diogenes FAQ with commits `7f4529e` + `37a0ed0`, seven troubleshooting gotchas). Cross-refs in README.md ("For LLM Collaboration" section) and MODULES.md registration. Commits: `983ffdb` content · `b148ac0` Done.
- **2026-04-24**: WP02 complete. Root `usage-rules.md` refreshed to v2.9.0+ surface; `lib/templates/llm/_usage-rules.md` template added with `[[PROJECT_NAME]] / [[INTENT_VERSION]] / [[LANG]]` placeholders; MODULES.md registers the template. Commits: `4e75ebd` content · `357e0c4` Done.
- **2026-04-24**: ST0035 opened and populated. Phase 0 forensic elaboration across `info.md`, `design.md` (10 canon decisions D1–D10 + risk register), `tasks.md`, 17 × `WP/NN/info.md`. Decisions resolved 2026-04-24. WP01 complete: VERSION → 2.9.1, `.intent/config.json` stamped, migration chain extended, ST0010 + ST0015 cancelled to `intent/st/CANCELLED/`. Commits: `055a7e4` Phase 0 · `b265987` decisions resolved · `567d5d1` WP01 · `1472cca` WP01 marked Done.
- **2026-04-23 to 2026-04-24**: `critic-shell` dogfood on Intent's own bash. WP12 dogfood journal Entry 1 complete. Commits: `a9ee349` P0/P1 · `0de89cd` P2 sweep · `60dfcd6` prompt-fix.
- **2026-04-23**: v2.9.0 released + fleet rollout complete. ST0034 closed. Tag `v2.9.0` on both remotes; GitHub release published.

## Next Up

1. **Resume WP04** (currently WIP, no content). Draft `lib/templates/.claude/settings.json` with three default hook stanzas (SessionStart / strict UserPromptSubmit / Stop; PostToolUse opt-in only). Author three helper scripts (`session-context.sh`, `require-in-session.sh`, `post-tool-advisory.sh`). Add cooperating sentinel-write step to `/in-session` SKILL.md. Register in MODULES.md. Add BATS test for `session-context.sh`. Full spec in `intent/st/ST0035/WP/04/info.md`.
2. **WP12** (XS): Socrates/Diogenes FAQ cross-refs in `socrates/agent.md` and `diogenes/agent.md` pointing at `intent/docs/working-with-llms.md` FAQ section. Quick win.
3. **WP05** (L, parallelisable): `bin/intent_critic` headless runner.
4. **WP18**: audit can start any time post-WP03; applied updates soft-gated on WP14 dogfood.

See `intent/st/ST0035/tasks.md` for full dependency graph.

## Deferred observations

- **Blog draft path**: `docs/blog/_drafts/####-shell-critic-inception.md` (updated from `docs/blog-drafts/` on 2026-04-24). Publication gated on real dogfood runs.
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

_(None — ST0010 and ST0015 cancelled in WP01.)_
