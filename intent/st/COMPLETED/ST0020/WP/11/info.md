---
verblock: "20 Feb 2026:v0.2: matts - Finalized design: Diogenes subagent + intent-elixir-testing skill"
wp_id: WP-11
title: "Diogenes Test Architect Subagent + intent-elixir-testing Skill"
scope: Medium
status: Done
---

# WP-11: Diogenes Test Architect Subagent + intent-elixir-testing Skill

## Objective

Create a two-part testing quality system for Elixir projects:

1. **Diogenes subagent** -- a Socratic dialog agent that extracts module intent and produces formal test specifications, then validates tests against those specifications
2. **intent-elixir-testing skill** -- 8 always-on rules that enforce test quality when Claude writes ExUnit tests

The subagent addresses the root cause of bad AI-generated tests: Claude writes "shape tests" that check data structure rather than behavior, uses control flow in test bodies that hides failures, and produces fake implementations that pass by design.

## Deliverables

- `intent/plugins/claude/subagents/diogenes/agent.md` -- Diogenes subagent
- `intent/plugins/claude/skills/intent-elixir-testing/SKILL.md` -- Testing skill (8 rules)
- `tests/unit/test_diogenes.bats` -- BATS tests for both artifacts
- Updated documentation across all project docs

## Design

Two-persona Socratic dialog (Aristotle + Diogenes) with two modes:

- **specify**: 5-phase dialog to analyze a module and produce a `*.spec.md` file
- **validate**: gap analysis comparing a spec against its test file

See `design.md` for full architecture.

## Acceptance Criteria

- [ ] Diogenes subagent installable via `intent claude subagents install diogenes`
- [ ] intent-elixir-testing skill installable via `intent claude skills install intent-elixir-testing`
- [ ] Both appear in `list` commands
- [ ] Both sync correctly via `sync` commands
- [ ] BATS tests pass
- [ ] Tested against at least one real Elixir project

## Dependencies

- WP-05 (testing.md reference doc -- existing foundation)
- WP-06 (skills infrastructure -- already built)
