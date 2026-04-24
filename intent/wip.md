---
verblock: "24 Apr 2026:v0.44: matts - ST0035 WP02 complete; WP03 next"
intent_version: 2.9.1
---

# Work In Progress

## Current State

**ST0035 (Canonical LLM Config + Fleet Rollout) active. WP01 + WP02 Done.** Intent stamped at `2.9.1`. Root `usage-rules.md` refreshed to v2.9.0+ surface (/in-\* skill family, critic-\* subagents, rule library, extensions, session hooks, pre-commit). Downstream template shipped at `lib/templates/llm/_usage-rules.md`. **WP03 (write `intent/docs/working-with-llms.md` canon tech note) is next.**

## ST0035 progress

| Status      | WP  | Title                                                                                 | Size |
| ----------- | --- | ------------------------------------------------------------------------------------- | ---- |
| Done        | 01  | Self-upgrade to v2.9.1 + cancel ST0010 / ST0015                                       | XS   |
| Done        | 02  | Refresh root `usage-rules.md`                                                         | S    |
| Not Started | 03  | Write `intent/docs/working-with-llms.md`                                              | M    |
| Not Started | 04  | `.claude/settings.json` template (SessionStart + UserPromptSubmit strict gate + Stop) | M    |
| Not Started | 05  | `bin/intent_critic` headless runner                                                   | L    |
| Not Started | 06  | `.git/hooks/pre-commit` template                                                      | S    |
| Not Started | 07  | `.intent_critic.yml` default template                                                 | XS   |
| Not Started | 08  | Root `AGENTS.md` generator rewrite                                                    | M    |
| Not Started | 09  | Root `CLAUDE.md` overlay template                                                     | S    |
| Not Started | 10  | Delete deprecated artefacts                                                           | XS   |
| Not Started | 11  | Extend `intent claude upgrade`                                                        | M    |
| Not Started | 12  | Socrates/Diogenes FAQ cross-refs                                                      | XS   |
| Not Started | 13  | Update Intent's own CLAUDE.md                                                         | S    |
| Not Started | 14  | Self-apply canon to Intent (dogfood)                                                  | S    |
| Not Started | 15  | Canary rollout (Conflab, Lamplight, Laksa)                                            | M    |
| Not Started | 16  | Fleet rollout (12 Intent + Pplr)                                                      | L    |
| Not Started | 17  | Verification sweep + dogfood journal                                                  | S    |

## Resolved decisions

1. **Version**: 2.9.1.
2. **Hook enforcement**: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. User will reassess intrusiveness post-rollout.
3. **Pre-commit critic threshold**: CRITICAL + WARNING blocks (`--warnings-are-errors` posture).
4. **PostToolUse advisory critic**: off by default (too noisy + too costly in tokens). Helper script ships; opt-in via `.intent_critic.yml post_tool_use_advisory: true` + user adds the stanza to `.claude/settings.local.json`.
5. **Cancelled STs** go to `intent/st/CANCELLED/`; deprecation annotations inline in the cancelled ST's `info.md`.

## Recent

- **2026-04-24**: WP02 complete. Root `usage-rules.md` refreshed to v2.9.0+ surface; `lib/templates/llm/_usage-rules.md` template added with `[[PROJECT_NAME]] / [[INTENT_VERSION]] / [[LANG]]` placeholders; MODULES.md registers the template. Commits: `4e75ebd` content · `357e0c4` Done.
- **2026-04-24**: ST0035 opened and populated. Phase 0 forensic elaboration across `info.md`, `design.md` (10 canon decisions D1–D10 + risk register), `tasks.md`, 17 × `WP/NN/info.md`. Decisions resolved 2026-04-24. WP01 complete: VERSION → 2.9.1, `.intent/config.json` stamped, migration chain extended, ST0010 + ST0015 cancelled to `intent/st/CANCELLED/`. Commits: `055a7e4` Phase 0 · `b265987` decisions resolved · `567d5d1` WP01 · `1472cca` WP01 marked Done.
- **2026-04-23 to 2026-04-24**: `critic-shell` dogfood on Intent's own bash. WP12 dogfood journal Entry 1 complete. Commits: `a9ee349` P0/P1 · `0de89cd` P2 sweep · `60dfcd6` prompt-fix.
- **2026-04-23**: v2.9.0 released + fleet rollout complete. ST0034 closed. Tag `v2.9.0` on both remotes; GitHub release published.

## Next Up

1. **WP03**: author `intent/docs/working-with-llms.md` (canon tech note — narrative guide explaining the three-file canon, how skills / subagents / rules / critics / hooks / extensions fit together, Socrates vs Diogenes FAQ). Size M.
2. **WP05** can start in parallel with WP03 — `bin/intent_critic` runner has no doc dependencies. Size L (biggest engineering WP in the ST).

See `intent/st/ST0035/tasks.md` for full dependency graph.

## Deferred observations

- **Blog draft path**: `docs/blog/_drafts/####-shell-critic-inception.md` (updated from `docs/blog-drafts/` on 2026-04-24). Publication gated on real dogfood runs.
- **WP07 follow-ups (from ST0034)**: align Diogenes fixture-context handling across the critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.

## Parked

_(None — ST0010 and ST0015 cancelled in WP01.)_
