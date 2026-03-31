---
verblock: "31 Mar 2026:v0.2: matts - As-built, completed"
intent_version: 2.8.0
status: Completed
slug: add-in-handoff-skill
created: 20260331
completed: 20260331
---

# ST0029: Add /in-handoff skill

## Objective

Add a `/in-handoff` skill that generates a permanent handoff document summarizing session work for future agents/sessions.

## As-Built

- `intent/plugins/claude/skills/in-handoff/SKILL.md` -- 5-step procedural guide
- `intent/plugins/claude/skills/in-handoff/scripts/handoff-prep.sh` -- bash 3.x helper (date, sequence, slug, git context)
- Handoff docs stored at `intent/.handoff/YYYYMMDD-NNN-<slug>.md`
- Registered in `intent/llm/MODULES.md` under Skills: Handoff
- Also fixed: `intent claude skills list` display (dynamic name column, terminal-width-aware, compact format)
- Also fixed: `get_terminal_width()` in `bin/intent_helpers` (removed `[ -t 1 ]` guard)

## Related Steel Threads

- ST0026 (skills system, in-\* prefix convention)
