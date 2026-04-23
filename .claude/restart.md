# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Task #26 follow-on commit should be the most recent. If `git status` shows uncommitted work, investigate before proceeding — there should be none.
3. **Read `intent/restart.md`** for the post-Task #26 state summary.
4. **Resume on WP11** (release + fleet upgrade). See "Next up" below.

## State

WP10 is **Done** (`6bb9d0d`). Task #26 is **Done** (`f2beaed` + follow-on cleanup) — `intent agents sync` generator now emits current `intent wp` commands, detects nested Bats layouts (`bats -r tests/`), and falls back to `agent.md` frontmatter for subagent descriptions. Dead `bl)` dispatch case removed from `bin/intent_main`; TPD `intent bl` residue from v2.5.0 swept. ST0034 is now 11/12 — only WP11 (release + fleet upgrade) remains. Full BATS suite 707/707 ok. `intent claude rules validate` 48/48 ok.

## Next up

1. **WP11 (Medium)**: release + fleet upgrade.
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

## Deferred

- WP12 dogfood journal Entries 1-3: post-release.
- `docs/blog-drafts/shell-critic-inception.md`: publication gated on real dogfood runs.
- WP07 follow-ups: align Diogenes fixture-context handling across the four critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`
