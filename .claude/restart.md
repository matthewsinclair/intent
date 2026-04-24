# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Expected recent commits (top to bottom): `aa9b7ca` · `c994579` · `21ed9c4` · `0b0d72d` · `cab9e06` · `c47fbfc` · `cfdcb51` · `c01d9fe` · `8c4df6f` · `e36b6f1`. If `git status` shows uncommitted work beyond `.claude/settings.local.json` (user-local, ignore), investigate.
3. **Read `intent/restart.md` + `intent/wip.md`** for state summary + WP queue.
4. **Read `intent/st/ST0035/WP/08/info.md`** — the next active WP. Continue from its Deliverables list.
5. If time permits before the active WP, **read `intent/st/ST0035/info.md` + `design.md`** for canon decisions refresher.

## State (2026-04-24, end of session — 8 of 18 Done, no WIP)

**ST0035 active.** Canon docs (usage-rules, working-with-llms) + hooks template (.claude/settings.json + 3 scripts + /in-session cooperating sentinel) + headless critic runner (bin/intent_critic + rules_lib.sh + critic_runner.sh) + pre-commit gate (.git/hooks/pre-commit) + critic config template (.intent_critic.yml) + Socrates/Diogenes cross-refs all shipped.

- 8 of 18 WPs Done: **WP01–WP07 + WP12**. No WIP — clean handoff.
- Decisions all resolved (1–5).
- `.intent/config.json`: `intent_version: 2.10.0` (retargeted from 2.9.1 to bundle ST0036 directory relocation).

## WP-08 resume target

Full spec: `intent/st/ST0035/WP/08/info.md`. Summary of remaining work:

1. Change `intent agents sync` output path from `intent/llm/AGENTS.md` to root `AGENTS.md` (as a real file, NOT a symlink).
2. Enrich AGENTS.md contents per canon D3: project overview, build/test commands, coding conventions summary, steel-thread process, installed skills/subagents, rule library pointer, critic invocation, Socrates/Diogenes FAQ paragraph.
3. Idempotency preserved: existing `agents_sync_idempotent` BATS test must still pass (two sync runs produce byte-identical output).
4. MODULES.md registration updated.
5. BATS tests for new root-level output + enriched contents.

Downstream: WP09 (Claude overlay) and WP10 (delete deprecated artefacts at `intent/llm/AGENTS.md` + `_llm_preamble.md`) gate on WP08. WP11 (installer) depends on WP08 for the generator shape.

## Next up after WP-08

1. **WP09** (S) — Rewrite root `CLAUDE.md` template as a Claude-specific overlay that imports AGENTS.md.
2. **WP10** (XS) — Delete `intent/llm/AGENTS.md` (retired) and `lib/templates/llm/_llm_preamble.md` (legacy).
3. **WP11** (M) — Extend `intent claude upgrade --apply` to install the full canon (settings.json + hooks + critic config + AGENTS.md + CLAUDE.md).
4. **WP13** (S) — Update Intent's own CLAUDE.md to reference the canon (needs WP09).
5. **WP14** (S) — Self-apply canon to Intent repo (dogfood).
6. **WP15/WP16/WP17** — Canary + fleet rollout + verification sweep.
7. **WP18** (M) — `intent/usr/*.md` audit (can run in parallel with WP15/16; must land before WP17).

See `intent/st/ST0035/tasks.md` for the full dependency graph.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (octal gotcha: use `ST0035` or `35`, not `0035`).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next, with a hard review gate after Phase 0 (passed for ST0035 long since).

## Lessons worth keeping from this session

- **awk exit-code trap**: `awk ... && return 0` is wrong when the awk script uses `exit 0` for "matched" — awk's natural completion also exits 0, so the caller can't distinguish. Fix: use a distinct exit sentinel (exit 10) for "matched". See `critic_rule_disabled()` in `intent/plugins/claude/lib/critic_runner.sh` for the canonical pattern.
- **`.intent_critic.yml` canonical field is `disabled:`** (not `disabled_rules:`). Schema lives in `intent/docs/critics.md` and the sample at `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`.
- **UserPromptSubmit strict gate must pass-through slash commands**: otherwise `/in-session` itself is blocked — chicken-and-egg. `require-in-session.sh` reads the prompt from stdin JSON and bails out cleanly on `/*`.
- **`intent critic` exit contract**: 0 clean / 1 findings / 2 error. Pre-commit hook treats 2 as "fail-open" (don't block a commit on broken tooling).

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` — blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.
