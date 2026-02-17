---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-09
title: Elixir Upgrade Skill
scope: Medium-Large
status: Not Started
---

# WP-09: Elixir Upgrade Skill

## Objective

Create upgrade functionality that diagnoses existing Intent+Elixir projects and migrates them to the new three-file guidance system with skills.

## Deliverables

- Upgrade diagnostic, plan generation, and execution capability
- Tested on Intent, Prolix, Laksa, and Lamplight projects

## Capabilities

1. **Diagnose** — scan for existing files, report what exists vs missing
2. **Plan** — generate project-specific upgrade checklist
3. **Execute** — apply changes with user confirmation at each step

## Files Scanned

- `intent/llm/AGENTS.md` (version, staleness)
- `intent/llm/RULES.md` (present/absent, shared vs project-specific rules)
- `intent/llm/ARCHITECTURE.md` (present/absent)
- `intent/llm/AGENTS-phx.md` (deprecated — merge into RULES.md)
- `intent/llm/llm_preamble.md` (deprecated — remove)
- `intent/llm/usage-rules.md` (deprecated — remove)
- `.claude/skills/` (installed skills)
- `.claude/agents/` (installed subagents)

## Test Targets

| Project   | Current State                     | Upgrade Needed                           |
| --------- | --------------------------------- | ---------------------------------------- |
| Intent    | Stale AGENTS.md, deprecated files | Regenerate AGENTS.md, delete deprecated  |
| Prolix    | AGENTS.md + AGENTS-phx.md only    | Create RULES/ARCH, merge, install skills |
| Laksa     | +RULES.md (10 rules)              | Create ARCH, merge AGENTS-phx            |
| Lamplight | Most complete                     | Install skills, verify formats           |

## Acceptance Criteria

- [ ] Diagnose correctly identifies existing/missing files
- [ ] Plan generates accurate upgrade steps
- [ ] Execute applies changes with confirmation
- [ ] Tested on at least Intent project
- [ ] Handles deprecated file cleanup

## Dependencies

- WP-02, WP-03, WP-04 (skills to install)
- WP-06 (skill installation infrastructure)
- WP-07 (templates for generating missing files)
