# Claude Code Session Restart — narrative state

## Current state (2026-04-24, end of WP03 + WP18 added; WP04 WIP scaffold-only)

**Intent v2.9.1 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) active, WIP.**

### Progress since last restart-note

- **WP02 Done**: root `usage-rules.md` refreshed; `lib/templates/llm/_usage-rules.md` template shipped with `[[PROJECT_NAME]] / [[INTENT_VERSION]] / [[LANG]]` placeholders; MODULES.md registers template. Commits `4e75ebd` content · `357e0c4` Done.
- **WP03 Done**: `intent/docs/working-with-llms.md` authored — 459 lines, D1–D10 as H2 sections, ASCII three-file architecture diagram, `.claude/settings.json` JSON snippet, critic cadence split, Socrates/Diogenes FAQ (with git hashes `7f4529e` + `37a0ed0`), seven troubleshooting gotchas. README.md gained a "For LLM Collaboration" section linking the new doc. MODULES.md registers it under Docs. Commits `983ffdb` content · `b148ac0` Done.
- **WP18 added (late)**: audit `intent/usr/user_guide.md` (877L), `reference_guide.md` (1370L), `deployment_guide.md` (619L) — all pre-v2.9.0 — against v2.9.1 canon and apply per-file keep / update / throw before release. Deps: WP03 (hard) + WP14 (soft). Blocks WP17. Phase 0 forensic detail populated in `WP/18/info.md`. Tasks.md + ST0035/info.md WP table + WP17 deps updated. Commit `b6fc2fe`.
- **WP04 WIP but scaffold-only**: `intent wp start ST0035/04` ran to flip status; no deliverables yet authored. Resume target is `WP/04/info.md` Deliverables list.

### ST0035 shape post-WP03

- Done: WP01, WP02, WP03.
- WIP (scaffold): WP04.
- Not Started (15): WP05, WP06, WP07, WP08, WP09, WP10, WP11, WP12, WP13, WP14, WP15, WP16, WP17, WP18.

Critical path unchanged: `WP01 → WP02 → WP03 → WP08 → WP09 → WP11 → WP14 → WP15 → WP16 → WP17`. WP17 now has a second gating input (WP18). Parallelisable: WP04 (WIP) and WP05 (not started) independent of the docs chain; WP12 (XS quick win, WP03-gated); WP18 audit can start any time, applied updates soft-gated on WP14.

## Resume target (WP04)

WP04 spec in `intent/st/ST0035/WP/04/info.md`. Summary of what's owed:

1. **`lib/templates/.claude/settings.json`** — JSON template with three default hook stanzas:
   - `SessionStart` (matchers: `startup|resume|clear|compact`) → runs `session-context.sh`.
   - `UserPromptSubmit` (strict gate, per resolved decision #2) → runs `require-in-session.sh`. Blocks first prompt until `/in-session` sentinel is written.
   - `Stop` → echoes `/in-finish` reminder.
   - **No `PostToolUse` in the default stanza** (resolved decision #4 — off by default; helper script ships for opt-in via `.intent_critic.yml post_tool_use_advisory: true`).
2. **Helper scripts at `lib/templates/.claude/scripts/`**:
   - `session-context.sh` (< 200ms) — emits project name, git branch, short SHA, active ST, WIP summary. Graceful degradation when git or `intent/wip.md` absent. Writes session_id to `/tmp/intent-claude-session-current-id`.
   - `require-in-session.sh` — reads session_id from stdin JSON; checks `/tmp/intent/in-session-${session_id}.sentinel`; exit 2 + stderr block if absent, exit 0 pass-through if present.
   - `post-tool-advisory.sh` (opt-in, non-blocking) — parses tool-use JSON from stdin; if Write|Edit|MultiEdit on a supported source language AND `.intent_critic.yml post_tool_use_advisory: true`, runs `intent critic <lang> --files <path>` and emits findings as system-reminder. Exit 0 always.
3. **`/in-session` SKILL.md cooperating step** — add a final "write sentinel" step (`mkdir -p /tmp/intent && touch /tmp/intent/in-session-$(cat /tmp/intent-claude-session-current-id 2>/dev/null || echo unknown).sentinel`) so the strict gate releases after `/in-session` runs.
4. **MODULES.md registration** — `lib/templates/.claude/` + all three helper scripts.
5. **BATS test** for `session-context.sh` — three scenarios: git+wip present, git only, no git. All green.
6. **Verification**: `jq . lib/templates/.claude/settings.json` exits 0; scripts executable; BATS suite still green.

Use `[[INTENT_HOME]]` placeholder for absolute paths in `settings.json`. WP11 (installer) substitutes at install time.

## Resolved decisions (all 5)

1. **Version**: 2.9.1.
2. **Hook enforcement**: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. Reassess post-rollout.
3. **Pre-commit critic threshold**: CRITICAL + WARNING blocks.
4. **PostToolUse advisory critic**: off by default. Helper script ships; opt-in via `.intent_critic.yml post_tool_use_advisory: true` + user adds PostToolUse stanza to `.claude/settings.local.json`.
5. **Cancelled STs** go to `intent/st/CANCELLED/`; deprecation annotation inline.

## Rollout universe (17 projects)

- 15 downstream Intent projects: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab, Courses/Agentic Coding, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz.
- Intent (self) — dogfooded in WP14.
- Pplr — non-Intent today; `intent init` first, then canon apply.

Excluded: Sites (handled inside Laksa as a subdir), llm-tropes (content-only), A3/\* (content-only).

Canary order (WP15): Conflab → Lamplight → Laksa. Fleet sweep (WP16) starts with highest-delta projects (Multiplyer, Arca trio) and ends with Pplr (bootstrap + apply).

## Session conventions (unchanged)

- T-shirt sizing only.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (gotcha: `ST0035` or `35`, not `0035` — leading zero is parsed as octal).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- Fail-forward: no backwards-compat shims.
- Document first, code next, hard review gate.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.

## Recent commits

- `b6fc2fe` — add WP-18 (review intent/usr/\*.md) + WP-03 close (tasks checkbox flips).
- `b148ac0` — mark ST0035/WP-03 Done.
- `983ffdb` — WP-03: working-with-llms.md canon tech note.
- `362e444` — wip.md update for WP-02 close.
- `357e0c4` — mark ST0035/WP-02 Done.
- `4e75ebd` — WP-02: refresh root usage-rules.md + add downstream template.
- `22ff450` — restart docs for WP-01 close.
- `1472cca` — mark ST0035/WP-01 Done.
- `567d5d1` — WP-01: bump to v2.9.1 + cancel ST0010/ST0015.
- `b265987` — resolve ST0035 open decisions.
- `055a7e4` — ST0035 Phase 0 scope and work packages.
