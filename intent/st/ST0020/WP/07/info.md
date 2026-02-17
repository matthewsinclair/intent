---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-07
title: RULES.md / ARCHITECTURE.md Templates and Tooling
scope: Medium
status: Not Started
---

# WP-07: RULES.md / ARCHITECTURE.md Templates and Tooling

## Objective

Create Elixir-specific templates for the three-file LLM guidance system and update `intent agents init` to support `--template elixir`.

## Deliverables

- `intent/plugins/agents/templates/elixir/AGENTS.md` — Elixir project AGENTS.md template
- `intent/plugins/agents/templates/elixir/RULES.md` — Pre-populated with distilled Elixir rules
- `intent/plugins/agents/templates/elixir/ARCHITECTURE.md` — Project structure skeleton
- Updated `intent agents init` to accept `--template elixir`
- BATS tests for template functionality

## Template Content

### AGENTS.md Template

- Project overview placeholders
- Elixir/Erlang version prereqs
- Mix commands (deps.get, test, format, credo)
- Installed skills and subagents listing

### RULES.md Template

- Core Elixir rules (from WP-01 distillation)
- Ash/Phoenix framework rules
- NEVER DO list
- Testing conventions
- Placeholder for project-specific conventions

### ARCHITECTURE.md Template

- System overview placeholder
- Elixir context/domain map structure
- Directory structure mapping
- Integration points placeholder
- Decision log placeholder

## Acceptance Criteria

- [ ] All three templates exist in correct directory
- [ ] `intent agents init --template elixir` creates all three files
- [ ] `intent agents sync` updates AGENTS.md without touching RULES.md or ARCHITECTURE.md
- [ ] RULES.md includes the shared 8 rules from real project analysis
- [ ] BATS tests for template init functionality

## Dependencies

- WP-01 (distilled rules for RULES.md template)
- WP-06 (skill infrastructure referenced in AGENTS.md template)
