---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-02
title: "Workflow Skills"
scope: Medium
status: Done
---

# WP-02: Workflow Skills

## Objective

Convert the set of manually copy-pasted Claude Code prompt strings into proper `/in-*` skills. These encode workflow discipline that currently lives only in the user's head or an Emacs buffer.

## Deliverables

### 5 New Skills

#### /in-start -- Session Start

Invoked at the beginning of a new Claude Code session.

**Procedure:**

1. Read `.claude/restart.md` and `intent/restart.md`
2. Read `intent/wip.md`
3. Review open STs (`intent st list`)
4. Review not-started STs
5. Provide comprehensive overview: where we are, what's next, plan forward
6. DO NOT WRITE ANY CODE -- read-only orientation, then wait for instructions

#### /in-plan -- Planning Kickoff

Invoked when starting a new piece of work.

**Procedure:**

1. Show detailed workplan for what's next
2. Invoke all relevant `/in-*` coding skills during planning
3. Enforce: Highlander Rule, Thin Coordinator Rule, PFIC Rule
4. Rules apply equally to Elixir, Rust, Swift, and Lua (where it makes sense)
5. ALWAYS document steel threads and work packages BEFORE coding -- no exceptions
6. Wait for user review before proceeding

#### /in-standards -- Coding Standards

Invoked at the start of coding or after context reset.

**Procedure:**

1. Re-read `intent/llm/RULES.md`
2. Re-read relevant usage rules (`deps/{lib}/usage-rules.md` for Ash, AshAi, etc.)
3. Enforce the Highlander Rule -- no duplicated code paths, ever
4. Enforce Thin Coordinators -- CLI, controllers, LiveViews are simple coordinators
5. Enforce PFIC -- Pure-Functional Idiomatic Code
6. Ensure all markdown tables are column-aligned

#### /in-next -- Next Step

Invoked mid-session to pick the next unit of work.

**Procedure:**

1. Review current steel thread/WP state
2. Identify the smallest, simplest, self-contained, coherent piece of work
3. Describe it in detail
4. Wait for instructions (do not start coding)

#### /in-finish -- Session Finish

Invoked at end of session.

**Procedure:**

1. Update all steel thread and work package docs with current state
2. Update `design.md` and `impl.md` with as-built status
3. Move completed tasks from `tasks.md` to `done.md`
4. Update `intent/wip.md` with current state and what's next
5. Update `intent/restart.md` with relevant context
6. Rewrite `.claude/restart.md` with WIP/TODO focus
7. No non-printing characters in any files (proper emojis and ASCII only)
8. No Claude signature in commit messages
9. ONLY update .md doc files -- do NOT write new code

## Design Notes

- These are **procedural** skills (step-by-step workflows), unlike the existing **enforcement** skills (always-on rules). SKILL.md format supports both.
- `/in-start` and `/in-finish` are bookends that frame every session
- `/in-plan` and `/in-next` structure work within a session
- `/in-standards` loads coding discipline into context

## Acceptance Criteria

- [ ] All 5 new skills created under `intent/plugins/claude/skills/in-*/`
- [ ] Each has SKILL.md with proper frontmatter
- [ ] `intent claude skills list` shows all 5 new skills
- [ ] `intent claude skills install in-start` (etc.) works
- [ ] Skills are functional when invoked via `/in-start` (etc.) in Claude Code

## Dependencies

- Blocked by: WP-01 (naming convention must be established first)
