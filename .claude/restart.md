# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Check for any uncommitted WP10 hand-off.** If `git status` is non-empty and matches the list below, the WP10 work is staged but the commit has not been made yet (user paused for review before commit). If the tree is clean, WP10 is fully closed; resume on WP11.
3. **Read `intent/restart.md`** for the post-WP10 state summary.

## State

WP10 is **Done** (status flipped via `intent wp done ST0034/10`). ST0034 is now 11/12. Three new docs (rules.md / writing-extensions.md expanded / critics.md updated); CLAUDE.md / MODULES.md / DECISION_TREE.md / creating-custom-agents.md / lib/help/\* updated; AGENTS.md regenerated; CHANGELOG + release-notes drafts; tests/unit/docs_completeness.bats (11 tests); +TCA suite full refactor for rule library; +intent/docs/total-codebase-audit.md updated for v2.9.0; +rules: frontmatter on Elixir skills. Full BATS suite 707/707 ok. `intent claude rules validate` 48/48 ok.

## Uncommitted state (only if commit pending)

If `git status` shows uncommitted work, the WP10 commit is the next action. The expected file list:

```
M  CHANGELOG.md
M  CLAUDE.md
M  intent/docs/creating-custom-agents.md
M  intent/docs/critics.md
M  intent/docs/total-codebase-audit.md
M  intent/docs/writing-extensions.md
M  intent/llm/AGENTS.md
M  intent/llm/DECISION_TREE.md
M  intent/plugins/claude/skills/in-elixir-essentials/SKILL.md
M  intent/plugins/claude/skills/in-elixir-testing/SKILL.md
M  intent/plugins/claude/skills/in-tca-audit/SKILL.md
M  intent/plugins/claude/skills/in-tca-finish/SKILL.md
M  intent/plugins/claude/skills/in-tca-init/SKILL.md
M  intent/plugins/claude/skills/in-tca-remediate/SKILL.md
M  intent/plugins/claude/skills/in-tca-synthesize/SKILL.md
M  intent/restart.md
M  intent/st/ST0034/WP/10/info.md
M  intent/st/ST0034/impl.md
M  intent/wip.md
M  lib/help/claude.help.md
M  lib/help/ext.help.md
M  lib/help/rules.help.md
M  .claude/restart.md
?? intent/docs/rules.md
?? docs/releases/2.9.0/
?? tests/unit/docs_completeness.bats
```

(Plus the still-uncommitted WP07 inheritance items if they weren't in `b79e1a2`: the four critic-\* subagent dirs, critic fixtures, .intent_critic.yml schema sample, manifest changes — verify against `git status`.)

Pre-commit gate: `./tests/run_tests.sh` exits 0; `intent claude rules validate` exits 0; `intent doctor` clean. Stage by name (no `-A`), single cohesive commit, no Claude attribution.

Suggested commit message:

```
WP10: documentation pass + TCA suite refactor for rule library

- New canonical docs: intent/docs/rules.md (rule library guide);
  intent/docs/writing-extensions.md expanded with worker-bee worked
  example; intent/docs/critics.md gains registration-freeze note.
- CLAUDE.md, MODULES.md, DECISION_TREE.md, creating-custom-agents.md
  updated for v2.9.0 architecture (drops elixir subagent, adds
  critic-* family, worker-bee relocation, Migration Notes).
- DECISION_TREE.md adds three new branches: rule placement, skill
  placement, rule-vs-skill-vs-subagent.
- lib/help/{ext,rules,claude}.help.md updated.
- AGENTS.md regenerated via intent agents sync (mid-WP follow-up
  filed for generator deficiencies surfaced by the diff).
- TCA suite refactored end-to-end for the rule library:
  in-tca-init selects rule packs by ecosystem; in-tca-audit
  dispatches critic-<lang> per WP; in-tca-synthesize consumes the
  stable critic schema; in-tca-remediate and in-tca-finish cite
  IN-* IDs throughout.
- intent/docs/total-codebase-audit.md (1195 lines) updated for
  v2.9.0; pre-v2.9.0 lessons-learned appendices preserved with
  historical-context notes.
- in-elixir-essentials and in-elixir-testing declare machine-readable
  rules: frontmatter listing the IN-EX-CODE-* and IN-EX-TEST-* IDs.
- tests/unit/docs_completeness.bats: 11 new tests covering doc
  presence, cross-reference resolution, no-dead-refs to deleted
  elixir/canon worker-bee, and intent agents sync idempotency.
- CHANGELOG.md v2.9.0 entry and docs/releases/2.9.0/RELEASE_NOTES.md
  drafted (final commit in WP11). No vanity metrics.
- ST0034/WP10 status: Done; 11/12 WPs complete.

Pre-commit gate: full BATS 707/707 ok; intent claude rules validate
48/48 ok; intent doctor clean.

(C) hello@matthewsinclair.com
```

## Next up after WP10

- **WP11 (Medium)**: release + fleet upgrade.
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
