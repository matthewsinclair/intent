# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Check for any uncommitted WP07 hand-off.** If `git status` shows the WP07 scaffolding still un-staged, the Phase 8 commit didn't land — see "Uncommitted state (only if commit pending)" below. If the tree is clean, WP07 is fully closed; resume on WP09.
3. **Read `intent/restart.md`** for the post-WP07 state summary and the WP07 follow-ups list.

## State

WP07 is **Done**. ST0034 is now 9/12. critic-{elixir,rust,swift,lua} authored, verified against the 16-row Phase 7 matrix, integrated with `in-review` stage-2, documented in `intent/docs/critics.md`. critic-shell retrofitted to the same family report format. Three new BATS suites green; full suite passes; rules validator clean.

## Uncommitted state (only if commit pending)

If `git status` is non-empty and matches the list below, the WP07 docs/tracker updates have happened but the commit has not been made yet (user paused for review before commit). The commit message template lives in the approved plan at `/Users/matts/.claude/plans/ultrathink-on-please-ingest-inherited-hippo.md` §Phase 8.

```
M  .claude/restart.md
M  .claude/settings.local.json
M  intent/llm/MODULES.md
M  intent/plugins/claude/skills/in-review/SKILL.md
M  intent/plugins/claude/skills/in-session/SKILL.md
M  intent/plugins/claude/subagents/.manifest/global-agents.json
M  intent/plugins/claude/subagents/critic-shell/agent.md
M  intent/restart.md
M  intent/st/ST0034/WP/07/info.md
M  intent/st/ST0034/impl.md
M  intent/wip.md
?? intent/docs/critics.md
?? intent/plugins/claude/rules/_schema/sample-intent-critic.yml
?? intent/plugins/claude/subagents/critic-elixir/
?? intent/plugins/claude/subagents/critic-lua/
?? intent/plugins/claude/subagents/critic-rust/
?? intent/plugins/claude/subagents/critic-swift/
?? tests/fixtures/critics/
?? tests/unit/critic_config.bats
?? tests/unit/critic_dispatch.bats
?? tests/unit/critic_report_format.bats
```

Pre-commit gate: `./tests/run_tests.sh` exits 0; `intent claude rules validate` exits 0. Stage by name (no `-A`), single cohesive commit, no Claude attribution.

## Next up after WP07

- **WP09 (Medium)**: migration chain. `bin/intent_upgrade` migration step `migrate_v2_8_x_to_v2_9_0`: prune `~/.claude/agents/elixir.md` and `~/.claude/agents/worker-bee.md`, rebuild critic-{shell,elixir,rust,swift,lua}, install `~/.intent/ext/` scaffold, install worker-bee from `lib/templates/ext-seeds/worker-bee/` if user opts in, sanity-check `.intent_critic.yml` placement. BATS coverage for the migration step.

## WP07 follow-ups (small, do not block WP09)

- Align fixture-context handling for the test-spec (Diogenes) handoff across all four critic agent.md files (`would-miss` test fixtures: elixir emits the recommendation; rust/swift/lua suppress it).
- `critic-rust` flagged a STYLE-tier IN-RS-CODE-005 (lifetime-elision-first) on `clean.rs`. Decide: tighten the rule's "When This Does Not Apply" carve-out for teaching fixtures, or simplify the fixture.

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

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`
