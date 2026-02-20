# Implementation - ST0020: Modernizing Intent's Elixir Support for Agentic Coding

## Implementation Notes

### Phase 0: Documentation

- ST0020 steel thread populated with full plan
- 10 work package info.md files created

### Phase 1: Foundation (WP-01 + WP-08)

- Refactored agent.md from 23 rules to 12 non-overlapping rules in 5 categories
- Minor alignment updates to style.md
- Created project-structure.md (~220 lines)

### Phase 2: Skills (WP-02 + WP-03 + WP-04)

- Created skills directory structure under `intent/plugins/claude/skills/`
- elixir-essentials: 8 rules with before/after examples
- ash-ecto-essentials: 7 rules + ash-ecto.md reference doc (~300 lines)
- phoenix-liveview: 7 rules + liveview.md reference doc (~280 lines)

### Phase 3: Testing (WP-05)

- Created testing.md reference doc covering DataCase, ConnCase, LiveView, Mox, Ash

### Phase 4: Infrastructure (WP-06 + WP-07)

- Created intent_claude_skills CLI with list/install/sync/uninstall/show subcommands
- SHA256 manifest tracking at ~/.intent/skills/installed-skills.json
- 37 BATS tests for skills commands
- Created elixir templates: AGENTS.md, RULES.md, ARCHITECTURE.md
- Updated intent_agents to support --template elixir

### Phase 5: Upgrade + Docs (WP-09 + WP-10)

- Created intent_claude_upgrade with diagnose/plan/execute phases
- Dry-run and --apply modes
- Written usage-rules.md (~310 lines) covering all Intent commands

### Cleanup

- Regenerated intent/llm/AGENTS.md (was stale v2.2.1)
- Deleted intent/llm/llm_preamble.md and intent/llm/usage-rules.md (deprecated)
- All 292 tests passing at cleanup

### Phase 6: Roll-out

- Upgraded 8 target projects: Intent, Prolix, Laksa-web, Lamplight, Anvil, MeetZaya, Multiplyer, Utilz
- Wrote upgrade guide (docs/upgrade-guide-2.4.0.md)
- Merged upgrade branches to main

### Post-Release (v2.4.0)

- Namespaced all skills with `intent-` prefix (intent-essentials, intent-elixir-essentials, etc.)
- Added YAML frontmatter to skills for Claude Code discovery
- Updated list display padding widths to 30 for namespaced names
- Added `-v`/`--verbose` flag to both `skills list` and `subagents list`
- Test count grew to 302 across 15 files

### Phase 7: Testing Quality (WP-11)

- Created Diogenes subagent (Socratic dialog test architect with Aristotle + Diogenes personas)
- Two modes: specify (5-phase dialog producing *.spec.md files) and validate (gap analysis)
- Created intent-elixir-testing skill (8 mandatory test quality rules)
- Added diogenes to global-agents.json manifest
- 19 BATS tests for both artifacts
- Updated skills_commands.bats manifest test to use relative counts (not brittle absolute counts)

## File Inventory

### Modified Files

| File                                              | WP    | Change                  |
| ------------------------------------------------- | ----- | ----------------------- |
| `intent/plugins/claude/subagents/elixir/agent.md` | WP-01 | Refactor 23 rules to 12 |
| `intent/plugins/claude/subagents/elixir/style.md` | WP-01 | Minor alignment updates |

### New Files

| File                                                                    | WP    | Purpose                  |
| ----------------------------------------------------------------------- | ----- | ------------------------ |
| `intent/plugins/claude/skills/intent-elixir-essentials/SKILL.md`        | WP-02 | Core Elixir skill        |
| `intent/plugins/claude/skills/intent-ash-ecto-essentials/SKILL.md`      | WP-03 | Ash/Ecto skill           |
| `intent/plugins/claude/skills/intent-phoenix-liveview/SKILL.md`         | WP-04 | LiveView skill           |
| `intent/plugins/claude/subagents/elixir/ash-ecto.md`                    | WP-03 | Ash/Ecto reference       |
| `intent/plugins/claude/subagents/elixir/liveview.md`                    | WP-04 | LiveView reference       |
| `intent/plugins/claude/subagents/elixir/testing.md`                     | WP-05 | Testing reference        |
| `intent/plugins/claude/subagents/elixir/project-structure.md`           | WP-08 | Project layout reference |
| `intent/plugins/claude/bin/intent_claude_skills`                        | WP-06 | Skill lifecycle CLI      |
| `intent/plugins/claude/bin/intent_claude_upgrade`                       | WP-09 | Project upgrade CLI      |
| `intent/plugins/agents/templates/elixir/AGENTS.md`                      | WP-07 | Elixir AGENTS template   |
| `intent/plugins/agents/templates/elixir/RULES.md`                       | WP-07 | Elixir RULES template    |
| `intent/plugins/agents/templates/elixir/ARCHITECTURE.md`                | WP-07 | Elixir ARCH template     |
| `usage-rules.md`                                                        | WP-10 | Intent's own usage-rules |
| `intent/plugins/claude/skills/intent-essentials/SKILL.md`               | post  | Universal essentials     |
| `intent/plugins/claude/subagents/diogenes/agent.md`                     | WP-11 | Test architect subagent  |
| `intent/plugins/claude/skills/intent-elixir-testing/SKILL.md`           | WP-11 | Testing quality skill    |
| `tests/unit/test_diogenes.bats`                                         | WP-11 | BATS tests for WP-11     |

### Deleted Files

| File                            | Reason     |
| ------------------------------- | ---------- |
| `intent/llm/llm_preamble.md`   | Deprecated |
| `intent/llm/usage-rules.md`    | Deprecated |

### Regenerated Files

| File                   | Reason                   |
| ---------------------- | ------------------------ |
| `intent/llm/AGENTS.md` | Was stale at v2.2.1     |

## Challenges & Solutions

| Challenge                                     | Solution                                                      |
| --------------------------------------------- | ------------------------------------------------------------- |
| Em dashes in skill files caused truncation     | Replaced all em dashes with double hyphens in SKILL.md files  |
| List padding broke with namespaced skill names | Switched to char-count based padding (`${#var}`)              |
| Terminal width detection unreliable            | Cascading fallback: $COLUMNS, stty, tput, hardcoded 100       |
| Skills not auto-discovered by Claude Code      | Added YAML frontmatter with name/description to SKILL.md      |
