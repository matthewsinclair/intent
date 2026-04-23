# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** WP10 commit `6bb9d0d` should be the most recent commit. If `git status` shows uncommitted work, investigate before proceeding — there should be none.
3. **Read `intent/restart.md`** for the post-WP10 state summary.
4. **Resume on Task #26 (fix `intent agents sync` generator) before WP11.** See "Next up" below.

## State

WP10 is **Done** and committed (`6bb9d0d`). ST0034 is now 11/12 — only WP11 (release + fleet upgrade) remains, but **fix Task #26 first** (see "Next up" below). Three new docs (rules.md / writing-extensions.md expanded / critics.md updated); CLAUDE.md / MODULES.md / DECISION_TREE.md / creating-custom-agents.md / lib/help/\* updated; AGENTS.md regenerated (with the generator regression that Task #26 will fix); CHANGELOG + release-notes drafts; tests/unit/docs_completeness.bats (11 tests); TCA suite full refactor for rule library; intent/docs/total-codebase-audit.md updated for v2.9.0; rules: frontmatter on Elixir skills. Full BATS suite 707/707 ok. `intent claude rules validate` 48/48 ok.

## Next up after WP10 (in order)

1. **Fix Task #26: `intent agents sync` generator deficiencies** (small, no ST overhead — standalone fix).
   - Generator at `intent/plugins/agents/bin/intent_agents` drops `intent wp` commands (replaced with stale `intent bl` placeholder).
   - Generator can't detect Bats test framework (renders "No automated tests configured yet").
   - Generator renders empty descriptions for some subagents (e.g. `diogenes`).
   - Why now: WP11 will regenerate AGENTS.md across the 16-project fleet; fixing the generator first means every fleet project gets a clean AGENTS.md instead of the regression we noted in WP10.
   - Verification: `intent agents sync` on Intent itself produces a clean diff (no `intent bl` stub, real Bats command, no empty subagent descriptions); idempotent on second run; `tests/unit/docs_completeness.bats::agents_sync_idempotent` stays green.

2. **WP11 (Medium)**: release + fleet upgrade.
   - Bump `VERSION` to `2.9.0`; tag `v2.9.0` and force-push to `local` and `upstream`.
   - Publish GitHub release using `docs/releases/2.9.0/RELEASE_NOTES.md`; finalise the CHANGELOG `[2.9.0]` date.
   - Bump worker-bee seed `intent_compat.min` (`lib/templates/ext-seeds/worker-bee/extension.json`) from `2.8.2` to `2.9.0` in lockstep with VERSION.
   - Run the WP09 canary dry-run against fleet projects (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab) **before** tagging.
   - Roll the v2.9.0 upgrade across the 16-project fleet.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.

## Deferred (unchanged)

- `intent agents sync` generator deficiencies (filed mid-WP10): drops `intent wp` commands, can't detect Bats, empty descriptions for some subagents. Needs dedicated ST or WP.
- WP12 dogfood journal Entries 1-3: post-release.
- `docs/blog-drafts/shell-critic-inception.md`: publication gated on real dogfood runs.
- WP07 follow-ups: align Diogenes fixture-context handling across the four critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`
