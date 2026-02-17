# Tasks - ST0020: Modernizing Intent's Elixir Support for Agentic Coding

## Phase 0: Documentation

- [x] Create ST0020 info.md with objective, context, scope
- [x] Create ST0020 design.md with architecture decisions
- [x] Create ST0020 tasks.md (this file)
- [ ] Create WP/01 through WP/10 info.md files
- [ ] Commit Phase 0 documentation

## Phase 1: Foundation (WP-01 + WP-08)

- [ ] WP-01: Audit current 23 rules, identify overlaps
- [ ] WP-01: Distill to ~12 non-overlapping rules in 5 categories
- [ ] WP-01: Rewrite agent.md Core Elixir Programming Rules section
- [ ] WP-01: Update style.md pointers if needed
- [ ] WP-08: Create project-structure.md (~150-250 lines)

## Phase 2: Skills (WP-02 + WP-03 + WP-04)

- [ ] WP-02: Create skills directory structure
- [ ] WP-02: Write elixir-essentials/SKILL.md (8 rules + examples)
- [ ] WP-03: Write ash-ecto.md reference doc
- [ ] WP-03: Write ash-ecto-essentials/SKILL.md (7 rules + examples)
- [ ] WP-04: Write liveview.md reference doc
- [ ] WP-04: Write phoenix-liveview/SKILL.md (7 rules + examples)

## Phase 3: Testing (WP-05)

- [ ] WP-05: Write testing.md reference doc (DataCase, ConnCase, LiveView, Mox, Ash)

## Phase 4: Infrastructure (WP-06 + WP-07)

- [ ] WP-06: Factor shared install/sync logic from intent_claude_subagents
- [ ] WP-06: Create intent_claude_skills CLI command
- [ ] WP-06: Implement list/install/sync/uninstall/show subcommands
- [ ] WP-06: Create installed-skills.json manifest handling
- [ ] WP-06: Register skills command in intent CLI
- [ ] WP-06: Write BATS tests for skills commands
- [ ] WP-07: Create elixir AGENTS.md template
- [ ] WP-07: Create elixir RULES.md template
- [ ] WP-07: Create elixir ARCHITECTURE.md template
- [ ] WP-07: Update intent_agents to support --template elixir
- [ ] WP-07: Write BATS tests for template functionality

## Phase 5: Upgrade + Docs (WP-09 + WP-10)

- [ ] WP-09: Create upgrade diagnostic functionality
- [ ] WP-09: Create upgrade plan generation
- [ ] WP-09: Create upgrade execution with confirmations
- [ ] WP-09: Test on Intent project
- [ ] WP-10: Write usage-rules.md (Intent's own, <500 lines)
- [ ] WP-10: Cover all commands with examples

## Cleanup

- [ ] Regenerate intent/llm/AGENTS.md (currently stale v2.2.1)
- [ ] Delete intent/llm/llm_preamble.md (deprecated)
- [ ] Run full test suite — all tests must pass
- [ ] Update CHANGELOG.md

## Dependencies

```
WP-01 ──┬── WP-02 ──── WP-06 ──┬── WP-09
         │                       ├── WP-07
         ├── WP-03 ──── WP-05   └── WP-10
         ├── WP-04 ──┘
         └── (WP-08 runs in parallel, no deps)
```
