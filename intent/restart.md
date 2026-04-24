# Claude Code Session Restart — narrative state

## Current state (2026-04-24, end of session — 11 of 18 WPs Done + ST0036 opened)

**Intent v2.10.0 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) active, WIP. ST0036 (Directory relocation) opened as Phase 0 stub, ships bundled.**

### Version retarget (mid-session)

- Retargeted v2.9.1 → v2.10.0 to bundle ST0036 (`.intent/` → `intent/.config/`) as a single breaking release.
- `VERSION`, `.intent/config.json`, `bin/intent_helpers` (`migrate_v2_9_0_to_v2_10_0` + `needs_v2_10_0_upgrade`), `bin/intent_upgrade` chain, CHANGELOG, AGENTS.md all updated.
- Retarget commit: `b760b39`.

### ST0035 shape

- **Done (11)**: WP01, WP02, WP03, WP04, WP05, WP06, WP07, WP08, WP09, WP10, WP12.
- **Not Started (7)**: WP11, WP13, WP14, WP15, WP16, WP17, WP18.
- **WIP (0)** — session ended cleanly.

Critical path remaining: `WP11 → WP14 → WP15 → WP16 → WP17`. WP13 (S) can run in parallel before WP14. WP17 has a second gating input (WP18 — user-doc audit).

### ST0036 shape (new)

Phase 0 stub only at `intent/st/NOT-STARTED/ST0036/`:

- `info.md` — objective, why bundled with ST0035, scope, success criteria, Phase 0 gate.
- `design.md` — provisional canon decisions D1–D5 (new path `intent/.config/`, atomic fail-forward migration, whole-tree preservation, shared rollout with ST0035, CHANGELOG + migration guide), risk register, open questions.
- `tasks.md` — 9 provisional WPs with T-shirt sizing (WP01 migration function, WP02 path probes, WP03 literal sweep, WP04 templates, WP05 BATS, WP06 gitignore, WP07 migration guide, WP08 Intent self-apply, WP09 merge with ST0035 fleet rollout).

Forensic `WP/NN/info.md` elaboration deferred until ST0036 is actively picked up (projected after ST0035/WP13 lands). Phase 0 review gate before any ST0036 WP01 start.

### Progress since last restart-note (post-compact after WP07)

This session shipped three WPs + two structural changes:

- **WP08 (M) Done**: `intent agents init/sync/validate` flip to root `AGENTS.md` (real file, not symlink). Generator enriched with 16 canon sections; dynamic skills/subagents rendering from `.claude/`; symlink-migration helper; `validate` inverted; `templates/default.md` deleted. Source-tree bug fixed (generator was reading `intent/plugins/claude/subagents/` not `.claude/agents/`). 12 new BATS. Commits: `546dc3d` content · `61fad69` Done.
- **WP09 (S) Done**: `lib/templates/llm/_CLAUDE.md` full rewrite as 58-line Claude overlay pointing at root AGENTS.md as primary. Reference block, `/in-session` directive, memory dir, session hooks, file map, rules-of-road via agnostic rule IDs (no duplication), critic dispatch, user-preservation markers. 12 new BATS. Commits: `d3c147d` content · `09cad07` Done.
- **Retarget v2.9.1 → v2.10.0** + **ST0036 Phase 0 stub**. Commits: `b760b39` retarget · `f4c68b9` ST0036 stub.
- **WP10 (XS) Done**: Deleted `intent/llm/AGENTS.md` + `lib/templates/llm/_llm_preamble.md`. Flipped residual code paths that still wrote the old path (intent_init, \_generate_basic_agents_md, intent_doctor, intent_claude_upgrade, docs_completeness BATS). Full suite 762/762 green. Commits: `1ae5f61` content · `2e99857` Done.

Test suite: **762/762 green** across all affected areas.

### Lessons worth keeping

- **Bundled semver bumps are cheap before release tag.** ST0035 was mid-flight at v2.9.1; retargeting to v2.10.0 to bundle ST0036 cost ~5 files' worth of string replacement (VERSION, config.json, helpers, upgrade chain, CHANGELOG). Zero rollout cost since no tag existed. Rule: check for uncommitted version bumps before committing to a breaking release strategy.
- **Deprecation sweeps leave ghost code paths.** Deleting a file doesn't delete its readers/writers. WP10's "two rm commands" ballooned into 8 file updates because `intent_init`, `intent_helpers::_generate_basic_agents_md`, `intent_doctor`, `intent_claude_upgrade`, and `docs_completeness.bats` all still pointed at `intent/llm/AGENTS.md`. Always grep for the deleted path and scope the WP accordingly.
- **Test suite hides stale-file false positives.** `docs_completeness.bats::agents_sync_idempotent` was passing post-WP08 because both runs copied the same stale intent/llm/AGENTS.md file (not the newly-written root one). 762 tests green can still hide a silent test. Periodic audit: do my tests actually exercise the code path they claim to?
- **"AGENTS.md is a symlink" migration works by deletion then write.** `_replace_symlink_if_present` in `intent_agents` handles the legacy-layout case (root AGENTS.md → symlink to intent/llm/AGENTS.md). Idempotent, safe to call on any project state.

## Resume target (WP11 — Extend `intent claude upgrade --apply`)

WP11 spec in `intent/st/ST0035/WP/11/info.md`. Summary:

`intent claude upgrade --apply` today (post-WP10) regenerates root `AGENTS.md` and creates intent/llm/{RULES,ARCHITECTURE}.md if absent. WP11 extends the `--apply` path to also install (idempotently):

1. `.claude/settings.json` from `lib/templates/.claude/settings.json` + three helper scripts (`session-context.sh`, `require-in-session.sh`, `post-tool-advisory.sh`) into `.claude/scripts/`.
2. `.git/hooks/pre-commit` from `lib/templates/hooks/pre-commit.sh` (chmod +x).
3. `.intent_critic.yml` from `lib/templates/_intent_critic.yml` (only if absent — user may have customised).
4. Root `CLAUDE.md` from `lib/templates/llm/_CLAUDE.md` (only if absent, or if marker block says Intent-generated and user hasn't edited outside `<!-- user:start --> / <!-- user:end -->` markers).

Idempotency: running `--apply` twice must produce byte-identical output.

Downstream: WP14 (Intent self-dogfood) runs `intent claude upgrade --apply` on Intent itself as the first target. WP15 (canary) + WP16 (fleet) apply it across the rollout universe.

## Rollout universe (17 projects, unchanged)

- 15 downstream Intent projects: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab, Courses/Agentic Coding, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz.
- Intent (self) — dogfooded in WP14.
- Pplr — non-Intent today; `intent init` first, then canon apply.

Excluded: Sites (inside Laksa), llm-tropes (content-only), A3/\* (content-only).

Canary order (WP15): Conflab → Lamplight → Laksa. Fleet (WP16) highest-delta first (Multiplyer, Arca trio), ends with Pplr.

## Resolved decisions (all 5, retargeted #1)

1. Version: **2.10.0** (retargeted from 2.9.1 mid-ST to bundle ST0036).
2. Hook enforcement: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. Reassess post-rollout.
3. Pre-commit critic threshold: CRITICAL + WARNING blocks (default; tunable per-project via `.intent_critic.yml severity_min`).
4. PostToolUse advisory critic: off by default. Opt-in via `.intent_critic.yml post_tool_use_advisory: true` + manual stanza add to `.claude/settings.local.json`.
5. Cancelled STs go to `intent/st/CANCELLED/`; deprecation annotation inline.

## Session conventions (unchanged)

- T-shirt sizing only.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (gotcha: `ST0035` or `35`, not `0035` — leading zero is parsed as octal).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- Fail-forward: no backwards-compat shims.
- Document first, code next, hard review gate.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.

## Recent commits (chronological)

- `2e99857` — mark ST0035/WP-10 Done.
- `1ae5f61` — WP-10: delete deprecated artefacts + flip residual code paths.
- `f4c68b9` — ST0036 Phase 0 stub.
- `b760b39` — retarget ST0035 v2.9.1 → v2.10.0.
- `09cad07` — mark ST0035/WP-09 Done.
- `d3c147d` — WP-09: rewrite \_CLAUDE.md as Claude overlay.
- `61fad69` — mark ST0035/WP-08 Done.
- `546dc3d` — WP-08: root AGENTS.md generator rewrite.
- `199c605` — session wrap for previous compact (8 of 18).
