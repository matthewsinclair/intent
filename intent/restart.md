# Claude Code Session Restart — narrative state

## Current state (2026-04-24, end of WP01)

**Intent v2.9.1 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) active, WIP.** Intent's own version stamped at `2.9.1` via WP01. The canon artefacts (AGENTS.md-at-root, CLAUDE.md overlay, refreshed usage-rules.md, `.claude/settings.json` hooks, pre-commit critic gate, `bin/intent_critic` runner) are defined in Phase 0 docs and land across WP02–WP11 before self-apply (WP14), canary (WP15), fleet rollout (WP16), and verification (WP17).

Phase 0 was the "document first, code next" elaboration: `info.md`, `design.md` (10 canon decisions D1–D10 + risk register), `tasks.md` (critical path + dependency matrix), and all 17 × `WP/NN/info.md` files with forensic detail. Committed as `055a7e4` and reviewed. All 5 open decisions from planning are resolved (see `intent/st/ST0035/info.md` Open Decisions section).

## ST0035 context

Driven by fleet audit findings:

- The LLM-facing config surface had drifted — root `CLAUDE.md` / `AGENTS.md` / `usage-rules.md` / `intent/llm/*` overlapping with no single canon.
- Fleet-wide `.claude/` was universally empty: no session hooks, no auto-loaded skills, no critic scheduling. Capability existed; enforcement didn't.
- 9 of the 15 upgraded projects were missing `intent/llm/DECISION_TREE.md`. Multiplyer was missing root `AGENTS.md` entirely.
- Root `usage-rules.md` was current-ish but pre-dated the `/in-*` skill family, critic subagents, and extensions.
- External state of the art: AGENTS.md is the de facto standard (60k+ repos, Linux Foundation governance); CLAUDE.md is positioned as a Claude-specific overlay; Elixir's per-package `usage-rules.md` convention is strong and worth honouring.
- Two stale NOT-STARTED steel threads (ST0010 MCP exploration, ST0015 enhanced ST templates) were overtaken — cancelled in WP01.

## ST0035 progress

Done: WP01 (self-upgrade + cancel ST0010/ST0015).

Not Started (16): WP02 through WP17. Critical path is WP01 → WP02 → WP03 → WP08 → WP09 → WP11 → WP14 → WP15 → WP16 → WP17. WP05 (`bin/intent_critic`) can start in parallel with WP02/03.

## Resolved decisions (all 5)

1. **Version**: 2.9.1.
2. **Hook enforcement**: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. Reassess post-rollout.
3. **Pre-commit critic threshold**: CRITICAL + WARNING blocks.
4. **PostToolUse advisory critic**: off by default. Helper script ships; opt-in via `.intent_critic.yml post_tool_use_advisory: true`.
5. **Cancelled STs** go to `intent/st/CANCELLED/`; deprecation annotation inline.

## Rollout universe (17 projects)

- 15 downstream Intent projects: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab, Courses/Agentic Coding, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz.
- Intent (self) — dogfooded in WP14.
- Pplr — non-Intent today; `intent init` first, then canon apply.

Excluded: Sites (handled inside Laksa as a subdir), llm-tropes (content-only), A3/\* (content-only).

Canary order (WP15): Conflab → Lamplight → Laksa. Fleet sweep (WP16) starts with highest-delta projects (Multiplyer, Arca trio) and ends with Pplr (bootstrap + apply).

## Next up

**WP02 — Refresh root `usage-rules.md`.** Size S. Updates the hand-authored file to cover /in-\* skill family, critic-\* subagents, extension system, hooks overview. Also author `lib/templates/llm/_usage-rules.md` template for downstream rollout. Keep the DO / NEVER structure — rules only; narrative lives in `working-with-llms.md` (WP03).

## Session conventions (unchanged)

- T-shirt sizing only.
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (gotcha: `ST0035` or `35`, not `0035` — leading zero is parsed as octal).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- Fail-forward: no backwards-compat shims.
- Document first, code next, hard review gate.

## Recent commits

- `1472cca` — mark ST0035/WP-01 Done.
- `567d5d1` — WP01: bump to v2.9.1 + cancel ST0010/ST0015.
- `b265987` — resolve ST0035 open decisions.
- `aa9e0dc` — Moved (blog-drafts path change).
- `055a7e4` — ST0035 Phase 0 scope and work packages.
- `0de89cd` — critic-shell dogfood Entry 1 P2 sweep.
