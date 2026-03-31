---
date: 20260331
session: 001
slug: add-in-handoff-skill
steel_threads: [ST0029]
---

# Handoff: Add /in-handoff skill and fix skills list display

## Summary

Added the `/in-handoff` skill to Intent (ST0029) and fixed the `intent claude skills list` display to be terminal-width-aware with dynamic name column sizing.

## What Was Accomplished

- Created `intent/plugins/claude/skills/in-handoff/SKILL.md` -- 5-step procedural guide for generating session handoff documents
- Created `intent/plugins/claude/skills/in-handoff/scripts/handoff-prep.sh` -- bash 3.x helper that determines filename (YYYYMMDD-NNN-slug format), scans for sequence numbers, and gathers git context
- Handoff docs are stored in `intent/.handoff/` as permanent session records
- Registered skill in `intent/llm/MODULES.md`
- Updated `intent/st/ST0029/info.md` with objective and deliverables; removed unused scaffolding files (design.md, impl.md, tasks.md)
- Fixed `get_terminal_width()` in `bin/intent_helpers` -- removed `[ -t 1 ]` guard that blocked `stty` fallback, allowing `tput cols` to be reached
- Fixed `intent claude skills list` display:
  - Name column auto-sizes to widest skill name (was hardcoded `%-30s`)
  - Status tags (`[INSTALLED]`) right-aligned to terminal edge
  - Removed "Available Skills:" header noise
  - Changed format from `"  name    - desc"` to `"name: desc"` for compactness

## Key Decisions

- Handoff docs go to `intent/.handoff/` (Intent artifact, not project docs)
- Separate from `/in-finish`: handoff docs are permanent archival records; restart/wip files are ephemeral and get overwritten each session
- Helper script outputs structured key=value pairs (not JSON) for bash 3.x compatibility
- Skill does not auto-commit -- user commits when ready
- No version bump -- re-tag v2.8.0 at HEAD

## Important Context

- 19 skills total after this addition (was 18)
- The `get_terminal_width()` fix affects all commands that use terminal width detection (skills list, subagents list, st list, etc.)
- The skills list format change (colon separator, no indent, no header) is a visual breaking change if anyone is parsing the output

## Files Changed

- `intent/plugins/claude/skills/in-handoff/SKILL.md` (new)
- `intent/plugins/claude/skills/in-handoff/scripts/handoff-prep.sh` (new)
- `intent/llm/MODULES.md` (added Skills: Handoff section)
- `intent/st/ST0029/info.md` (filled in objective/deliverables)
- `bin/intent_helpers` (get_terminal_width fix)
- `intent/plugins/claude/bin/intent_claude_skills` (dynamic name column, format change)

## Next Steps

- Commit all changes
- Re-tag v2.8.0 at HEAD, push to both remotes
- Mark ST0029 as done
- Consider peer language skills (in-rust-essentials, in-swift-essentials) as future work
