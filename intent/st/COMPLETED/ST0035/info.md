---
verblock: "24 Apr 2026:v0.2: matts - Populated after Phase 0 planning"
intent_version: 2.9.0
status: Completed
slug: canonical-llm-config-fleet-rollout
created: 20260424
completed: 20260427
---

# ST0035: Canonical LLM Config + Fleet Rollout

## Objective

Define Intent's canonical, opinionated configuration surface for LLM–codebase interaction — a single agreed answer to AGENTS.md, CLAUDE.md, usage-rules.md, `intent/llm/`, `.claude/settings.json` hooks, and pre-commit critic scheduling. Refresh Intent's own artefacts to that canon. Extend `intent claude upgrade` to ship the canon into any project. Roll out to all 17 active Intent-using projects (16 under Intent governance + Pplr via `intent init`). Ship as Intent v2.10.0.

## Context

Intent v2.9.0 shipped on 2026-04-23 and reached 13 fleet projects, but its LLM-facing configuration surface has drifted:

- Four-plus overlapping files (root CLAUDE.md, root AGENTS.md, root usage-rules.md, intent/llm/\*) with no single canon saying who owns what.
- 9 of the 15 upgraded projects are missing `intent/llm/DECISION_TREE.md`; Multiplyer is missing `AGENTS.md` entirely.
- Fleet-wide, `.claude/` is empty. Every project has the _capability_ for session hooks, auto-loaded skills, and scheduled critic runs — but none of the _enforcement_.
- Intent's own `.intent/config.json` still reads `2.8.2` despite v2.9.0 being tagged and released. (The generated AGENTS.md footer correctly reads v2.9.0 — it's a stamp lag, not a content lag.)
- Root `usage-rules.md` is hand-authored and mostly current but pre-dates `/in-*` skills, extensions at `~/.intent/ext/`, and the critic family.
- Two NOT-STARTED steel threads (ST0010 MCP exploration, ST0015 enhanced ST templates) are v2.0.0-era — overtaken by v2.9.0 work. They should be cancelled with a deprecation annotation.
- User confusion reported on **Socrates vs Diogenes** agent naming. Git log confirms they were never the same agent: Socrates (2025-08-05) is CTO Review; Diogenes (2026-02-20) is test-spec dialog. Needs a clarifying FAQ.

Meanwhile, external conventions have converged. **AGENTS.md is the de facto standard** (60k+ GitHub repos, Linux Foundation Agentic AI Foundation governance, adopted by Anthropic / OpenAI / Google / Cursor / Copilot / Codex / Aider / Continue / Cline / Gemini CLI). **CLAUDE.md is complementary** — Anthropic positions it as a Claude-specific overlay on top of AGENTS.md, not a competing file. The Elixir `usage-rules.md` convention is orthogonal: per-package LLM guidance shipped with dependencies, discoverable via `mix usage_rules.sync`. Intent's Elixir fleet already benefits from this pattern, and Intent's own root `usage-rules.md` is discoverable the same way — it should **stay and be refreshed**, not be deleted.

ST0035 resolves the drift with a single release. Fail-forward, no backwards-compat shims.

## Scope

### In scope (v2.10.0)

- **Canon definition**: three root files with clear ownership — `AGENTS.md` (auto-generated, primary, tool-agnostic), `CLAUDE.md` (Claude-specific overlay, templated), `usage-rules.md` (hand-authored, refreshed, Elixir-convention-aligned). `intent/llm/` retains MODULES.md + DECISION_TREE.md only; `intent/llm/AGENTS.md` retires.
- **New narrative doc**: `intent/docs/working-with-llms.md` explaining the canon, the hook system, the critic cadence, and the Socrates/Diogenes FAQ.
- **Session hooks**: ship `.claude/settings.json` template that wires `SessionStart` → "Run /in-session" reminder and `Stop` → "Run /in-finish" reminder. `intent claude upgrade` installs per project.
- **Critic scheduling**: new `bin/intent_critic` headless runner (parses rules and applies Detection heuristics in bash — no LLM round-trip required for mechanical checks). Shipped `.git/hooks/pre-commit` template that blocks commits on CRITICAL/WARNING findings. Per-project `.intent_critic.yml` config for threshold overrides.
- **Generator extensions**: `intent claude upgrade --apply` applies all canon artefacts idempotently. `intent agents sync` writes `AGENTS.md` to root (not `intent/llm/`). `intent claude upgrade` prunes deprecated artefacts.
- **Self-dogfood**: apply canon to Intent itself first (bumping `.intent/config.json` to 2.10.0), before fleet rollout.
- **Fleet rollout**: 16 Intent projects + Pplr (via `intent init`) = 17 total. Canary in three (Conflab, Lamplight, Laksa) before sweep.
- **ST cancellations**: ST0010 and ST0015 cancelled via existing `Cancelled` status, moved to `intent/st/CANCELLED/`, annotated with one-liner explaining deprecation.
- **Socrates/Diogenes FAQ**: one-paragraph sidebar in `working-with-llms.md`, cross-reference in both `agent.md` files.

### Out of scope

- Rewriting the rule library (48 rules remain canon; no authoring in this ST).
- Rewriting skill content (23 skills remain canon — only how they're auto-loaded changes).
- Changing the extension system at `~/.intent/ext/` (stays as v2.9.0 shipped).
- v2.10.0 feature work (MCP integration, remote critic infrastructure, new languages — each is its own ST).
- Replacing the critic subagents — `bin/intent_critic` is an _additional_ headless runner for pre-commit, not a subagent replacement.
- Sites (handled inside Laksa as a subdir), llm-tropes (content-only), A3/\* (content-only) — excluded from rollout.
- Auto-install of worker-bee extension (stays opt-in; users run `intent claude subagents install worker-bee` explicitly).
- Auto-fix in pre-commit critic (it blocks or passes; no rewrite). Fix mode deferred.
- Rule-pack versioning, central rule registry, cross-project rule sharing (future STs).

## Related Steel Threads

- **ST0034** (v2.9.0 Agentic Software Engineering Suite) — the immediate predecessor; ST0035 refines and ships the tooling ST0034 introduced.
- **ST0028** (TCA v3.0) — established the multi-language rule paradigm consumed by `bin/intent_critic`.
- **ST0030** (Superpowers cherry-picks) — source of `chains_to:` frontmatter and Red Flags table pattern; referenced by hook-injected reminders.
- **ST0033** (Cwd-resilient dispatch) — `INTENT_ORIG_CWD` pattern applicable to pre-commit hook invocation.
- **ST0010** (Anthropic MCP Integration, v2.0.0-era) — **cancelled in this ST** as overtaken by v2.9.0 work; annotated, not resurrected.
- **ST0015** (Enhanced Steel Thread Templates, v2.0.0-era) — **cancelled in this ST** as overtaken.

## Work Packages

| WP   | Title                                                                 | Deps                         | Size | Risk |
| ---- | --------------------------------------------------------------------- | ---------------------------- | ---- | ---- |
| WP01 | Self-upgrade Intent to v2.10.0 + cancel ST0010/ST0015                 | —                            | XS   | Low  |
| WP02 | Refresh root `usage-rules.md` to current-as-built state               | WP01                         | S    | Low  |
| WP03 | Write `intent/docs/working-with-llms.md` (canon tech note)            | WP02                         | M    | Low  |
| WP04 | `.claude/settings.json` template (hooks for /in-session, /in-finish)  | WP01                         | M    | Med  |
| WP05 | `bin/intent_critic` — headless critic runner in bash                  | —                            | L    | High |
| WP06 | `.git/hooks/pre-commit` template (critic gate)                        | WP05                         | S    | Med  |
| WP07 | `.intent_critic.yml` default template                                 | WP05                         | XS   | Low  |
| WP08 | Root `AGENTS.md` generator rewrite (move from intent/llm/ to root)    | WP03                         | M    | Med  |
| WP09 | Root `CLAUDE.md` template rewrite (Claude-specific overlay)           | WP08                         | S    | Low  |
| WP10 | Delete deprecated artefacts (intent/llm/AGENTS.md, \_llm_preamble.md) | WP08                         | XS   | Low  |
| WP11 | Extend `intent claude upgrade` to apply canon artefacts               | WP04, WP06, WP07, WP08, WP09 | M    | Med  |
| WP12 | Socrates/Diogenes FAQ + cross-refs in agent.md files                  | WP03                         | XS   | Low  |
| WP13 | Update Intent's own CLAUDE.md to reference canon                      | WP03, WP09                   | S    | Low  |
| WP14 | Self-apply canon to Intent repo (dogfood)                             | WP11, WP13                   | S    | Med  |
| WP15 | Canary rollout: Conflab, Lamplight, Laksa                             | WP14                         | M    | Med  |
| WP16 | Fleet rollout: remaining 13 projects (12 Intent + Pplr)               | WP15                         | L    | Med  |
| WP17 | Verification sweep + dogfood journal                                  | WP16, WP18                   | S    | Low  |
| WP18 | Review and update (or retire) `intent/usr/*.md`                       | WP03 (soft WP14)             | M    | Low  |

See `design.md` for the full canon decision rationale, per-WP acceptance-criteria philosophy, risk register, and fail-forward stance. Each `WP/NN/info.md` carries its own forensic detail: objective, deliverables, approach, acceptance criteria, risks, verification.

## Phase 0 posture

This steel thread was scaffolded under the "document first, code next" discipline: Phase 0 populates all ST and WP docs without changing any production file, then stops for user review. WP01 does not begin until Phase 0 is approved.

Plan source: `/Users/matts/.claude/plans/ultrathink-on-please-ingest-elegant-sundae.md` (approved 2026-04-24).

## Open Decisions — resolved 2026-04-24

All five open decisions from Phase 0 are now resolved. Resolutions baked into the relevant WP `info.md` files.

1. **Version bump**: **2.10.0** — initially scoped as refinement (2.9.1), retargeted mid-ST to bundle ST0036 (directory relocation, `.intent/` → `intent/.config/`) into a single breaking release. Semver-breaking directory move forces the minor bump within 2.x.
2. **Hook enforcement strictness**: **strict** — `UserPromptSubmit` hard gate that blocks the first prompt until `/in-session` has run in the conversation. User will reassess intrusiveness post-rollout; if it's too noisy, flip to soft-reminder via a template override. Affects WP04 acceptance criteria and the `.claude/settings.json` template.
3. **Pre-commit critic threshold**: **CRITICAL + WARNING** (`--warnings-are-errors` posture). Default already in `.intent_critic.yml` template (WP07).
4. **PostToolUse advisory critic**: **off by default** — would be too noisy (fires on every intermediate edit during multi-step work) and too expensive in tokens (every tool use injecting advisory findings into the context). Pre-commit gate catches everything at the canonical checkpoint. Helper script still ships so users can opt in via `.intent_critic.yml post_tool_use_advisory: true`, but the default `.claude/settings.json` stanza omits the PostToolUse hook entirely.
5. **Cancelled ST location**: `intent/st/CANCELLED/` — the existing graveyard. Headstones (deprecation annotations) go in the info.md of the corpse.

## Context for LLM

This document is the Objective and Context summary for steel thread ST0035. For architectural and canon decisions, see `design.md`. For per-WP forensic scope, see each `WP/NN/info.md`. When starting a new session on this ST, begin by reading this file + `design.md`, then check `intent/wip.md` for the active WP.

### How to update this document

1. Update the status as work progresses (Not Started → WIP at WP01 start → Completed when WP17 ships).
2. Update the WP table only if WPs are merged, split, or renumbered (rare — keep stable).
3. Update `design.md` for any canon change; update this file's Objective/Context only if the goal itself shifts.
4. Mark the completion date when the fleet sweep verifies clean.
