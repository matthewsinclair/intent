# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Expected recent commits: `1472cca` (WP-01 marked Done) · `567d5d1` (WP01 content) · `b265987` (ST0035 decisions resolved) · `055a7e4` (ST0035 Phase 0). If `git status` shows uncommitted work beyond `.claude/settings.local.json` (user-local, ignore), investigate.
3. **Read `intent/restart.md` + `intent/wip.md`** for state summary + WP queue.
4. **Read `intent/st/ST0035/info.md` + `design.md`** for scope + canon decisions.
5. **Read the active WP's `info.md`** — currently WP02 (`intent/st/ST0035/WP/02/info.md`).

## State (2026-04-24, end of WP01)

**ST0035 (Canonical LLM Config + Fleet Rollout) active, WIP.** Intent stamped at v2.9.1 via WP01; canon artefacts not yet installed (that's WP02–WP11). Phase 0 docs + decisions resolved + WP01 all committed.

- 1 of 17 WPs done (WP01).
- 5 of 5 open decisions resolved (see `intent/st/ST0035/info.md` Open Decisions section).
- ST0010 and ST0015 cancelled → `intent/st/CANCELLED/` with deprecation annotations.
- `.intent/config.json`: `intent_version: 2.9.1`.
- Blog draft path: `docs/blog/_drafts/####-shell-critic-inception.md` (updated from `docs/blog-drafts/`).

## Next up

1. **WP02** — Refresh root `usage-rules.md` to v2.9.0+ surface (/in-\* skills, critic family, extensions, hooks overview) + author `lib/templates/llm/_usage-rules.md` template. Size S.
2. **WP03** — Author `intent/docs/working-with-llms.md` canon tech note. Size M. Depends on WP02.
3. **WP05** — `bin/intent_critic` headless runner can start in parallel with WP02/03. Size L (biggest engineering WP in the ST).
4. Continue through WP17 per the dependency graph in `intent/st/ST0035/tasks.md`.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (watch for octal gotcha: use `ST0035` or `35`, not `0035`).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next, with a hard review gate after Phase 0 (already passed for ST0035).

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` — blog draft path move completed in `aa9e0dc`; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.
