# Claude Code Session Restart — narrative state

## Current state (2026-04-24, end of session — 8 of 18 WPs Done)

**Intent v2.9.1 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) active, WIP.**

### ST0035 shape

- **Done (8)**: WP01, WP02, WP03, WP04, WP05, WP06, WP07, WP12.
- **Not Started (10)**: WP08, WP09, WP10, WP11, WP13, WP14, WP15, WP16, WP17, WP18.
- **WIP (0)** — session ended cleanly; nothing scaffold-only.

Critical path remaining: `WP08 → WP09 → WP11 → WP14 → WP15 → WP16 → WP17`. WP17 has a second gating input (WP18 — user-doc audit).

### Progress since last restart-note (`/compact` checkpoint after WP03)

This session shipped five WPs:

- **WP04 (M) Done**: `lib/templates/.claude/settings.json` + three helper scripts (`session-context.sh`, `require-in-session.sh`, `post-tool-advisory.sh`) + `/in-session` SKILL.md cooperating sentinel step + BATS. Commits `e36b6f1` content · `8c4df6f` Done.
- **WP12 (XS) Done**: Socrates/Diogenes FAQ cross-refs in both `socrates/agent.md` and `diogenes/agent.md` pointing at `intent/docs/working-with-llms.md#socrates-vs-diogenes-faq`. Commits `c01d9fe` content · `cfdcb51` Done.
- **WP05 (L) Done**: `bin/intent_critic` headless runner + `intent/plugins/claude/lib/rules_lib.sh` (extracted from `intent_claude_rules` for Highlander) + `intent/plugins/claude/lib/critic_runner.sh` + `tests/unit/intent_critic.bats`. `intent/docs/critics.md` gained a "Headless runner" section. Commits `c47fbfc` content · `cab9e06` Done.
- **WP07 (XS) Done**: `lib/templates/_intent_critic.yml` default template + `post_tool_use_advisory` row added to critics.md schema table. **Also fixed a critical WP-05 bug** caught during schema review: `critic_rule_disabled()` was returning "disabled" for every rule because awk's natural exit-0 collided with match-exit-0; now uses exit code 10 as a distinct sentinel. Renamed field from nonstandard `disabled_rules:` to canonical `disabled:`. 2 regression BATS added. Commits `0b0d72d` content · `21ed9c4` Done.
- **WP06 (S) Done**: `lib/templates/hooks/pre-commit.sh` scans staged files per detected language (`mix.exs`/`Cargo.toml`/`Package.swift`/`.luarc.json`; shell always included), runs `intent critic <lang> --staged --severity-min <sev>`, blocks on findings. Fail-open on missing `git`/`intent`/`.intent/config.json`. `intent/docs/pre-commit-hook.md` covers install/configure/opt-out/CI/troubleshooting. 9 BATS scenarios (scratch-repo end-to-end). Commits `c994579` content · `aa9b7ca` Done.

Test suite total across affected areas: ~75 BATS tests green.

### Lessons learned worth keeping

- **awk exit-code trap**: `awk ... && return 0` is wrong when awk uses `exit 0` as "match" — natural completion also exits 0. Use a distinct sentinel (exit 10) for "matched" and check it in the caller. See `critic_rule_disabled()` in `critic_runner.sh`.
- **Field name canon**: `.intent_critic.yml` uses `disabled:` (not `disabled_rules:`). Canon is `intent/docs/critics.md` schema table + `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`.
- **Slash-command pass-through in UserPromptSubmit gate**: without pass-through, the strict gate would block `/in-session` itself — chicken-and-egg. `require-in-session.sh` checks the prompt text from stdin JSON; any `/*` prompt passes.
- **`intent critic` exit contract**: 0 clean / 1 findings / 2 error. The pre-commit hook interprets 2 as "fail-open" (don't block commit on broken tooling).

## Resume target (WP08 — Root AGENTS.md generator rewrite)

WP08 spec in `intent/st/ST0035/WP/08/info.md`. Summary (read the info.md for the forensic detail):

Today: `intent agents sync` writes `intent/llm/AGENTS.md` and the root `AGENTS.md` is a symlink to it.

Canon per D3: root `AGENTS.md` becomes a real file (the primary LLM-facing doc). `intent/llm/AGENTS.md` retires (WP10 deletes it).

Scope:

1. Change the generator (`intent/plugins/agents/bin/intent_agents`) output path from `intent/llm/AGENTS.md` to root `AGENTS.md` (real file, not a symlink).
2. Enrich contents per canon: project overview, build/test commands, coding conventions summary, steel-thread process, installed skills/subagents, rule library pointer, critic invocation, Socrates/Diogenes FAQ paragraph.
3. Idempotency: regenerating the same content produces a byte-identical file (existing `agents_sync_idempotent` test must still pass).
4. MODULES.md update.
5. BATS test for the new output location + enriched contents.

Downstream consequences to watch:

- WP09 needs this WP's new root-level shape for the Claude-specific overlay.
- WP10 (trivial — delete `intent/llm/AGENTS.md` + `lib/templates/llm/_llm_preamble.md`) unblocks once WP08 lands.
- WP11 (`intent claude upgrade --apply`) depends on WP08's generator.

## Rollout universe (17 projects, unchanged)

- 15 downstream Intent projects: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab, Courses/Agentic Coding, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz.
- Intent (self) — dogfooded in WP14.
- Pplr — non-Intent today; `intent init` first, then canon apply.

Excluded: Sites (inside Laksa), llm-tropes (content-only), A3/\* (content-only).

Canary order (WP15): Conflab → Lamplight → Laksa. Fleet (WP16) highest-delta first (Multiplyer, Arca trio), ends with Pplr.

## Resolved decisions (all 5, unchanged)

1. Version: 2.9.1.
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

- `aa9b7ca` — mark ST0035/WP-06 Done.
- `c994579` — WP-06: pre-commit critic gate template.
- `21ed9c4` — mark ST0035/WP-07 Done.
- `0b0d72d` — WP-07: `.intent_critic.yml` template + WP-05 bug fix.
- `cab9e06` — mark ST0035/WP-05 Done.
- `c47fbfc` — WP-05: `bin/intent_critic` headless runner.
- `cfdcb51` — mark ST0035/WP-12 Done.
- `c01d9fe` — WP-12: Socrates/Diogenes FAQ cross-refs.
- `8c4df6f` — mark ST0035/WP-04 Done.
- `e36b6f1` — WP-04: `.claude/settings.json` template + hook scripts.
- `4e6076c` — (prev session wrap) WP-04 WIP scaffold-only.
- `b6fc2fe` — add ST0035/WP-18 + WP-03 close.
- `b148ac0` — mark ST0035/WP-03 Done.
- `983ffdb` — WP-03: working-with-llms.md canon tech note.
- `567d5d1` — WP-01: bump to v2.9.1 + cancel ST0010/ST0015.
