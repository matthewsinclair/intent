---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-01
title: Distill Core Elixir Rules
scope: Small
status: Not Started
---

# WP-01: Distill Core Elixir Rules

## Objective

Deduplicate the 23 core Elixir programming rules in `agent.md` down to ~12 non-overlapping rules organized into clear categories.

## Deliverables

- Refactored `intent/plugins/claude/subagents/elixir/agent.md` with ~12 distilled rules
- Rules organized into categories: Data Access, Control Flow, Composition, Error Handling, Code Hygiene
- Minor alignment updates to `style.md` if needed

## Current State

Rules 1-2, 7, 10-11 all cover pipe/composition. Rules 3, 9, 13 all cover pattern matching. Rules 14-15 both cover avoiding if/else. This dilution weakens the signal.

## Target State

~12 rules, each covering a distinct concern, with no overlap. Each rule should be actionable and memorable. The distilled rules become the basis for the `elixir-essentials` skill (WP-02) and RULES.md templates (WP-07).

## Acceptance Criteria

- [ ] No two rules cover the same concept
- [ ] Each rule is a single clear imperative statement
- [ ] Rules organized into 5 categories
- [ ] Existing NEVER DO list preserved unchanged
- [ ] Framework-specific patterns section unchanged
- [ ] Code review workflow section unchanged
- [ ] Antipattern/style guide references unchanged

## Dependencies

- None (foundation for all other WPs)
