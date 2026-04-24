# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Expected recent commits: `b6fc2fe` (WP-18 add + WP-03 tasks flips) · `b148ac0` (WP-03 Done) · `983ffdb` (WP-03 content) · `362e444` (wip after WP-02) · `357e0c4` (WP-02 Done) · `4e75ebd` (WP-02 content). If `git status` shows uncommitted work beyond `.claude/settings.local.json` (user-local, ignore), investigate — the only expected pending change post-compact is WP04's `intent wp start` status flip in `intent/st/ST0035/WP/04/info.md`, which may still be uncommitted depending on whether this doc update landed in the same commit.
3. **Read `intent/restart.md` + `intent/wip.md`** for state summary + WP queue.
4. **Read `intent/st/ST0035/WP/04/info.md`** — the current active WP. Continue from its Deliverables list.
5. If time permits before the active WP, **read `intent/st/ST0035/info.md` + `design.md`** for canon decisions refresher.

## State (2026-04-24, end of WP-03 + WP-18 added; WP-04 WIP scaffold-only)

**ST0035 active, WIP.** Canon docs are in place: root `usage-rules.md` refreshed, downstream template shipped, `intent/docs/working-with-llms.md` authored (459 lines, D1–D10 as H2, ASCII arch diagram, JSON hook example, critic cadence, Socrates/Diogenes FAQ with git hashes `7f4529e` + `37a0ed0`, seven troubleshooting gotchas). README.md gained "For LLM Collaboration" section.

- 3 of 18 WPs Done (WP01–WP03). ST0035 has 18 WPs total: 17 original + WP18 added 2026-04-24 for `intent/usr/*.md` review.
- WP04 WIP but scaffold-only — no deliverables written. Resume is drafting the `.claude/settings.json` template + three helper scripts + `/in-session` cooperating step.
- WP12 (XS — Socrates/Diogenes agent.md cross-refs) and WP05 (L — `bin/intent_critic`) remain parallelisable.
- 5 of 5 open decisions resolved.
- ST0010 and ST0015 cancelled → `intent/st/CANCELLED/`.
- `.intent/config.json`: `intent_version: 2.9.1`.

## WP-04 resume target (in detail)

Full spec: `intent/st/ST0035/WP/04/info.md`. Summary of remaining work:

1. `lib/templates/.claude/settings.json` — three default hook stanzas (SessionStart / strict UserPromptSubmit / Stop; **no PostToolUse** in default).
2. Three helper scripts at `lib/templates/.claude/scripts/`:
   - `session-context.sh` — SessionStart context injector, < 200ms, project/git/ST/WIP info, also writes session_id to `/tmp/intent-claude-session-current-id`.
   - `require-in-session.sh` — UserPromptSubmit strict gate using sentinel at `/tmp/intent/in-session-${session_id}.sentinel`. Exit 2 + stderr if absent.
   - `post-tool-advisory.sh` — PostToolUse advisory, opt-in, non-blocking.
3. `/in-session` SKILL.md cooperating step — write the sentinel after loading skills so the strict gate releases.
4. MODULES.md registration for `lib/templates/.claude/` and all three scripts.
5. BATS test for `session-context.sh` across three scenarios (git+wip / git only / no git).

Placeholder convention: `[[INTENT_HOME]]` for absolute paths in `settings.json`. WP11 (installer) substitutes at install time.

## Next up after WP-04

1. **WP12** (XS) — Socrates/Diogenes FAQ cross-refs in `critic-*`/`socrates`/`diogenes` agent.md files pointing at the FAQ in `intent/docs/working-with-llms.md`.
2. **WP05** (L) — `bin/intent_critic` headless runner. Biggest engineering WP in the ST. Can run in parallel with WP04/12.
3. Continue through WP17 per dep graph in `intent/st/ST0035/tasks.md`.
4. **WP18** (M, user-doc review) runs in parallel with WP15/WP16; must land before WP17.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (octal gotcha: use `ST0035` or `35`, not `0035`).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next, with a hard review gate after Phase 0 (already passed for ST0035).

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` — blog draft path moved in `aa9e0dc`; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.
