# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Expected top commits (newest first): `b2a6e5d` · `1db2b44` · `614980d` · `989451a` · `e999f82` · `052ba9d` · `2e99857` · `1ae5f61` · `f4c68b9` · `b760b39`. If `git status` shows uncommitted work beyond `.claude/settings.local.json` (user-local, ignore), investigate.
3. **Read `intent/restart.md` + `intent/wip.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/13/info.md`** — WP13 is the next active WP (smallest unit; updates Intent's own CLAUDE.md to reference the canon).
5. **Also read `intent/st/NOT-STARTED/ST0036/info.md` + `design.md` + `tasks.md`** — ST0036 Phase 0 elaboration is the parallel gating thread (populate 9 `WP/NN/info.md` files before any implementation WP).

## State (2026-04-25, end of session — 12 of 18 Done; WP-11 closed; ST0036 Phase 0 stub awaiting elaboration)

**Intent v2.10.0 in progress. ST0035 active; WP-11 closed. ST0036 sibling Phase 0 stub opened (ships bundled), awaiting forensic WP elaboration.**

- 12 of 18 WPs Done: **WP01–WP12**.
- 6 remain: **WP13, WP14, WP15, WP16, WP17, WP18**.
- `.intent/config.json`: `intent_version: 2.10.0`.
- `VERSION`: `2.10.0`.
- Full test suite: **767/767 green** (5 new BATS in `tests/unit/intent_claude_upgrade.bats`).
- `intent doctor`: clean.

## Resume target — WP13 + ST0036 Phase 0 (parallel)

These two threads are independent — pick either or both.

**WP13** (S — Update Intent's own `CLAUDE.md` to reference the canon):

- Spec: `intent/st/ST0035/WP/13/info.md`.
- Depends on WP09 ✓ (`lib/templates/llm/_CLAUDE.md`) and WP03 ✓ (`intent/docs/working-with-llms.md`).
- Smallest unit; ships first to unblock WP14 (self-dogfood).
- Note: Intent's own root `CLAUDE.md` is currently hand-authored (Intent's own developer guide). WP13 needs to either (a) refactor to use canon overlay format with user section, or (b) document the deviation. Read the WP13 info.md before committing to one approach.

**ST0036 Phase 0 elaboration**:

- Stub directory: `intent/st/NOT-STARTED/ST0036/`.
- Already populated: `info.md` (objective, scope, success criteria, Phase 0 gate), `design.md` (D1–D5 + risk register), `tasks.md` (9 provisional WPs with T-shirt sizing).
- Forensic `WP/NN/info.md` elaboration deferred — needs to land before any ST0036/WP01 code work.
- Phase 0 review gate: user reviews + approves all 9 WPs before any implementation lands.

After both: WP14 self-apply canon to Intent (carries both ST0035 canon AND ST0036 directory relocation in one pass). Then canary rollout (WP15: Conflab → Lamplight → Laksa), fleet rollout (WP16: 12 Intent + Pplr), verification sweep (WP17). WP18 (`intent/usr/*.md` audit) runs in parallel with WP15/WP16; must land before WP17.

## WP-11 closed -- summary of what shipped

`intent claude upgrade --apply` is the canon installer. From an Intent project:

- Phase 1 (DIAGNOSE): probes `.claude/settings.json` + 3 hook scripts, `.git/hooks/pre-commit` (worktree-aware via `git rev-parse --git-path hooks`), `.intent_critic.yml`, root `CLAUDE.md` (Intent-generated vs user-authored), root `usage-rules.md`, `intent/llm/MODULES.md` + `DECISION_TREE.md`, legacy `intent/llm/AGENTS.md`. Status column aligns at col 43 via `canon_print` helper.
- Phase 2 (PLAN): pretty-printed action queue. Diff-in-dry-run for `REFRESH_CLAUDE_MD` shows what the canon overlay adds vs the user's version.
- Phase 3 (EXECUTE): canon-install helpers do the work. Idempotent (placeholder-aware drift compare). Ready-to-paste multi-line snippet on `CHAIN_PRE_COMMIT`. Writability probe upfront so read-only FS bails with a clear diagnostic.
- Flags: `--apply` / `--dry-run` (default) / `--force` (overwrite user-edited canon — banner warning) / `--skip-settings` (escape hatch for deeply customised `.claude/settings.json`).
- BATS: 5 scenarios at `tests/unit/intent_claude_upgrade.bats` (fresh install, idempotence, user-section preservation, hook chain, dry-run no-op). Tests isolate `HOME` so installed subagents/skills don't bleed.
- `migrate_v2_9_0_to_v2_10_0` in `bin/intent_helpers` calls `intent claude upgrade --apply` after stamp bump (one-step canon apply for fleet upgrades).

Inline bug fixes folded in: `intent init /abs/path` no longer crashes sed (commit `614980d`); `canon_compute_refresh_preview` now stages preserved user content in a temp file (command substitution was stripping trailing blank lines).

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (octal gotcha: use `ST0035` or `35`, not `0035`).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next, with a hard review gate after Phase 0.

## Lessons worth keeping (cumulative across recent sessions)

- **Mid-ST version retargets are cheap before release tag.** v2.9.1 → v2.10.0 was ~5 files of string replacement when no tag existed. Check the "is it tagged?" question before committing to a bundling strategy.
- **Deprecation sweeps leave ghost readers.** Deleting `intent/llm/AGENTS.md` required updating 5 other code paths that still wrote to it. Always grep for the deleted path and scope the WP accordingly.
- **Idempotence requires placeholder-aware drift compare.** Any canon file whose install path runs sed substitution must use the same substitution before the drift probe (`canon_template_matches_installed` does this generically). Comparing raw template vs substituted install is a correctness bug, not cosmetic.
- **Command substitution silently strips trailing newlines.** Bash `$()` chops trailing `\n`, so a user section ending with a blank line round-trips one line shorter through a shell variable. For exact byte-for-byte preservation, stage in a temp file and read line-by-line. The bug only manifests as bogus DIVERGED reports — easy to miss without idempotence tests.
- **Scratch + BATS catch different bugs.** Scratch testing exposed both bugs in WP-11 Session 2 (intent init absolute-path crash + blank-line stripping); BATS would not have, because `make_scratch_project` would `cd` first by convention, hiding the path issue. Conversely, BATS catches behaviour drift that scratch testing only finds if you happen to look. Run both, scratch first to fix mechanics cheaply, BATS second to lock in the contract.
- **Don't paper over upstream bugs to keep the test green.** The reflex "work around by `cd` first" was wrong — the real fix was ~15 LOC away. Fix-first beats workaround-first when the cost is bounded; otherwise every future test author trips the same trap.
- **Helper extraction is cheaper than copy-paste even at N=2.** `canon_compute_refresh_preview` was inlined in two places; extracting fixed both at once when the blank-line bug surfaced AND made the new dry-run diff a one-liner. Highlander Rule applies inside files too.

## Open follow-ups (outside ST0035 + ST0036)

- `docs/blog/_drafts/####-shell-critic-inception.md` — blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 / ST0036 scope.
