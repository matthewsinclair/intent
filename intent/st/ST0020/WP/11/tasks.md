# Tasks - WP-11: Diogenes Test Architect Subagent + intent-elixir-testing Skill

## Phase 1: Design

- [x] Update WP-11 info.md with finalized scope and acceptance criteria
- [x] Create WP-11 design.md with architecture, personas, modes, and template
- [x] Create WP-11 tasks.md (this file)

## Phase 2: Subagent

- [x] Create `intent/plugins/claude/subagents/diogenes/agent.md`
- [x] Two personas: Aristotle (Empiricist) + Diogenes (Skeptic)
- [x] Specify mode with 5-phase dialog
- [x] Validate mode with gap analysis
- [x] Test spec template
- [x] Example dialog excerpts

## Phase 3: Skill

- [x] Create `intent/plugins/claude/skills/intent-elixir-testing/SKILL.md`
- [x] 8 rules with BAD/GOOD Elixir examples
- [x] YAML frontmatter for Claude Code discovery
- [x] No em dashes (multi-byte truncation bug)

## Phase 4: Tests

- [x] Create `tests/unit/test_diogenes.bats`
- [x] Test subagent install/list/sync
- [x] Test skill install/list/sync
- [x] All tests pass

## Phase 5: Documentation

- [x] Update CHANGELOG.md
- [x] Update README.md (subagent + skill entries)
- [x] Update CLAUDE.md (Available Agents)
- [x] Update usage-rules.md (subagents + skills lists)
- [x] Update intent/llm/AGENTS.md (subagents list)
- [x] Update intent/usr/user_guide.md (subagents + skills tables)
- [x] Update intent/usr/reference_guide.md (subagents + skills tables)
- [x] Update intent/usr/deployment_guide.md (directory tree + skills table)
- [x] Update tests/README.md (new test file)
- [x] Update TPD files (technical_product_design.md, 3_architecture.md, 8_appendices.md)
- [x] Update ST0020 impl.md and tasks.md

## Phase 6: Verification

- [ ] `intent claude subagents list` shows diogenes
- [ ] `intent claude skills list` shows intent-elixir-testing
- [ ] `intent claude subagents install diogenes` works
- [ ] `intent claude skills install intent-elixir-testing` works
- [ ] `tests/run_tests.sh` passes all tests
- [ ] Manual test on a real Elixir project
