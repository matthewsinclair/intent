# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Check for any uncommitted WP09 hand-off.** If `git status` is non-empty and matches the list below, the WP09 docs/tracker updates have happened but the commit has not been made yet (user paused for review before commit). If the tree is clean, WP09 is fully closed; resume on WP10.
3. **Read `intent/restart.md`** for the post-WP09 state summary and the canary follow-up.

## State

WP09 is **Done** (status flipped via `intent wp done ST0034/09`). ST0034 is now 10/12. Migration step `migrate_v2_8_2_to_v2_9_0` + predicate + ext README emitter shipped in `bin/intent_helpers`; chain wired in `bin/intent_upgrade` (gate + 16 chain-tails + new `"2.8.2"` case); `tests/unit/ext_migration.bats` adds 28 tests. Full BATS suite 696 ok. `intent claude rules validate` 48/48 ok.

## Uncommitted state (only if commit pending)

If `git status` shows uncommitted work, the WP09 commit is the next action. The expected file list:

```
M  .claude/restart.md
M  bin/intent_helpers
M  bin/intent_upgrade
M  intent/llm/MODULES.md
M  intent/restart.md
M  intent/st/ST0034/WP/09/info.md
M  intent/st/ST0034/impl.md
M  intent/wip.md
?? tests/unit/ext_migration.bats
```

Pre-commit gate: `./tests/run_tests.sh` exits 0; `intent claude rules validate` exits 0; `bash -n bin/intent_helpers && bash -n bin/intent_upgrade` exits 0. Stage by name (no `-A`), single cohesive commit, no Claude attribution.

Suggested commit message:

```
WP09: v2.8.2 -> v2.9.0 migration step + chain wiring

- bin/intent_helpers: migrate_v2_8_2_to_v2_9_0 (stamp, ext bootstrap,
  worker-bee seed, prune installed elixir + worker-bee); idempotent
- bin/intent_helpers: needs_v2_9_0_upgrade predicate
- bin/intent_helpers: generate_ext_readme emitter for ~/.intent/ext/README.md
- bin/intent_upgrade: gate check extended; new "2.8.2" case; every
  prior chain-tail extended with migrate_v2_8_2_to_v2_9_0
- tests/unit/ext_migration.bats: 28 new tests covering predicate,
  README, version stamp, ext bootstrap, worker-bee seed, agent prune,
  manifest cleanup, idempotency, chain coverage, and static gates on
  bin/intent_upgrade wiring
- intent/llm/MODULES.md: register generate_ext_readme alongside the
  existing migration entries
- ST0034/WP09 status: Done; 10/12 WPs complete

Canary dry-run against fleet projects (Anvil, Arca/*, Conflab) is
deferred and gated on user; run before WP11 tags v2.9.0.

(C) hello@matthewsinclair.com
```

## Next up after WP09

- **WP10 (Medium)**: documentation pass for the v2.9.0 surfaces.
  - `intent/docs/extensions.md` — writing extensions, `~/.intent/ext/` layout, `extension.json` manifest schema, lifecycle (install/list/show/validate).
  - `intent/docs/rules.md` — rule library structure, agnostic→language concretisation, authoring a rule.
  - `intent/docs/critics.md` updates — cross-link Diogenes/Socrates handoffs and add the registration-freeze operational note (mid-session subagent installs aren't visible until next session).
  - Migration Notes block in CLAUDE.md or top-level docs covering the v2.8.2 → v2.9.0 jump and how to verify post-upgrade state.
  - `intent/llm/MODULES.md` + `intent/llm/DECISION_TREE.md` refreshed for the new surfaces.
  - `tests/unit/docs_completeness.bats` — already a placeholder row in MODULES.md; ship the real check.
  - Worker-bee README in `lib/templates/ext-seeds/worker-bee/` cross-checked against the live ext layout.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.

## Deferred (unchanged)

- TCA suite retrospection: `in-tca-*` suite likely subsumed by `critic-<lang>` — rewrite or retire before WP10/WP11.
- WP12 dogfood journal Entries 1-3: post-release.
- `docs/blog-drafts/shell-critic-inception.md`: publication gated on real dogfood runs.
- Worker-bee seed `intent_compat.min` bump: WP11 must bump 2.8.2 → 2.9.0 in lockstep with VERSION.
- WP07 follow-ups: align Diogenes fixture-context handling across the four critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`
