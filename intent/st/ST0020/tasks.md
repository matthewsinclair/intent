# Tasks - ST0020: Modernizing Intent's Elixir Support for Agentic Coding

## Phase 0: Documentation

- [x] Create ST0020 info.md with objective, context, scope
- [x] Create ST0020 design.md with architecture decisions
- [x] Create ST0020 tasks.md (this file)
- [x] Create WP/01 through WP/10 info.md files
- [x] Commit Phase 0 documentation

## Phase 1: Foundation (WP-01 + WP-08)

- [x] WP-01: Audit current 23 rules, identify overlaps
- [x] WP-01: Distill to ~12 non-overlapping rules in 5 categories
- [x] WP-01: Rewrite agent.md Core Elixir Programming Rules section
- [x] WP-01: Update style.md pointers if needed
- [x] WP-08: Create project-structure.md (~220 lines)

## Phase 2: Skills (WP-02 + WP-03 + WP-04)

- [x] WP-02: Create skills directory structure
- [x] WP-02: Write elixir-essentials/SKILL.md (8 rules + examples)
- [x] WP-03: Write ash-ecto.md reference doc (~300 lines)
- [x] WP-03: Write ash-ecto-essentials/SKILL.md (7 rules + examples)
- [x] WP-04: Write liveview.md reference doc (~280 lines)
- [x] WP-04: Write phoenix-liveview/SKILL.md (7 rules + examples)

## Phase 3: Testing (WP-05)

- [x] WP-05: Write testing.md reference doc (DataCase, ConnCase, LiveView, Mox, Ash)

## Phase 4: Infrastructure (WP-06 + WP-07)

- [x] WP-06: Create intent_claude_skills CLI command
- [x] WP-06: Implement list/install/sync/uninstall/show subcommands
- [x] WP-06: Create installed-skills.json manifest handling
- [x] WP-06: Register skills command in bin/intent CLI
- [x] WP-06: Write BATS tests for skills commands (37 tests)
- [x] WP-07: Create elixir AGENTS.md template
- [x] WP-07: Create elixir RULES.md template
- [x] WP-07: Create elixir ARCHITECTURE.md template
- [x] WP-07: Update intent_agents to support --template elixir

## Phase 5: Upgrade + Docs (WP-09 + WP-10)

- [x] WP-09: Create intent_claude_upgrade with dry-run/apply modes
- [x] WP-09: Diagnostic phase (files, subagents, skills)
- [x] WP-09: Plan generation with action list
- [x] WP-09: Execute phase with --apply
- [x] WP-09: Register upgrade command in bin/intent CLI
- [x] WP-09: Test dry-run on Intent project
- [x] WP-10: Write usage-rules.md (Intent's own, ~310 lines)
- [x] WP-10: Cover all commands with examples

## Cleanup

- [x] Regenerate intent/llm/AGENTS.md (was stale v2.2.1)
- [x] Delete intent/llm/llm_preamble.md (deprecated)
- [x] Delete intent/llm/usage-rules.md (deprecated, replaced by skills)
- [x] Run full test suite — all 292 tests pass across 15 files
- [x] Update CHANGELOG.md

## Phase 6: Roll-out

- [x] Run upgrade on Intent project (cleaned deprecated files, updated subagent, installed skills)
- [x] Run upgrade on Prolix (new RULES.md + ARCHITECTURE.md, merged AGENTS-phx.md)
- [x] Run upgrade on Laksa-web (new AGENTS.md + ARCHITECTURE.md, merged AGENTS-phx.md, deleted llm_preamble.md)
- [x] Run upgrade on Lamplight (regenerated AGENTS.md, already had RULES.md + ARCHITECTURE.md)
- [x] Run upgrade on Anvil (new RULES.md + ARCHITECTURE.md from template, deleted llm_preamble.md)
- [x] Run upgrade on MeetZaya (new RULES.md + ARCHITECTURE.md, merged AGENTS-phx.md, deleted llm_preamble.md)
- [x] Run upgrade on Multiplyer (new RULES.md + ARCHITECTURE.md from template, deleted llm_preamble.md)
- [x] Run upgrade on Utilz (regenerated AGENTS.md only, non-Elixir project)
- [x] Write upgrade guide (docs/upgrade-guide-2.4.0.md)
- [x] Merge upgrade branches to main

## Phase 7: Testing Quality (WP-11)

- [x] WP-11: Finalize WP-11 design docs (info.md, design.md, tasks.md)
- [x] WP-11: Create Diogenes subagent (agent.md with Aristotle + Diogenes personas)
- [x] WP-11: Create intent-elixir-testing skill (8 rules with BAD/GOOD examples)
- [x] WP-11: Add diogenes to global-agents.json manifest
- [x] WP-11: Write BATS tests (19 tests for subagent + skill)
- [x] WP-11: Fix skills_commands.bats manifest test (use relative counts)
- [x] WP-11: Update all project documentation
- [ ] WP-11: Manual test on a real Elixir project

## Dependencies

```
WP-01 ──┬── WP-02 ──── WP-06 ──┬── WP-09
         │                       ├── WP-07
         ├── WP-03 ──── WP-05   └── WP-10
         ├── WP-04 ──┘
         ├── (WP-08 runs in parallel, no deps)
         └── WP-05 + WP-06 ──── WP-11
```
