---
verblock: "31 Mar 2026:v0.23: matts - Add /in-handoff skill + skills list display fixes"
intent_version: 2.8.0
---

# Work In Progress

## Current State

v2.8.0 with /in-handoff skill and skills list display improvements. 19 skills, 5 subagents.

## This Session

- Created `/in-handoff` skill (ST0029) -- permanent session handoff documents at `intent/.handoff/`
- Helper script `handoff-prep.sh` automates date/sequence/slug/git-context gathering
- Fixed `intent claude skills list` display:
  - Dynamic name column (auto-sizes to widest skill)
  - Terminal-width-aware right-aligned status tags
  - Compact format: `name: description` (no indent, no header)
- Fixed `get_terminal_width()` -- removed `[ -t 1 ]` guard blocking fallback chain

## Active Steel Threads

None.

## TODO

- Consider peer language skills (in-rust-essentials, in-swift-essentials)
- Run detrope on other Intent docs (intent/docs/\*.md)

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`

## Key References

- Handoff skill: `intent/plugins/claude/skills/in-handoff/`
- Handoff docs: `intent/.handoff/`
- Test suite: `tests/run_tests.sh` (22 .bats files, 462 tests)
