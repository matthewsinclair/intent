---
verblock: "20 Feb 2026:v0.3: matts - Completed"
intent_version: 2.4.0
status: Completed
created: 20260220
completed: 20260220
---

# ST0021: Intent Autopsy -- Session Analysis & Memory Meta-Learning

## Objective

Build a memory-aware session analysis system that compares Claude Code session behavior against stated rules in MEMORY.md and CLAUDE.md. Identifies gaps, enforcement failures, undocumented conventions, and stale memory entries. Proposes concrete memory updates to close the loop.

## Scope

### In Scope

- Elixir script (`autopsy.exs`) for pre-processing JSONL session files
- Claude skill (`intent-autopsy`) for running analysis and producing reports
- Extension to `intent claude skills install` for full directory copy (scripts alongside SKILL.md)
- Default banned-words file with common AI-isms
- BATS tests for skill lifecycle and directory install

### Out of Scope

- Interactive dashboard or web UI
- Automatic memory updates (always proposes, never auto-applies)
- Cross-project analysis (one project at a time)

## Deliverables

1. `intent/plugins/claude/skills/intent-autopsy/SKILL.md` -- skill definition
2. `intent/plugins/claude/skills/intent-autopsy/scripts/autopsy.exs` -- Elixir preprocessor
3. `intent/plugins/claude/skills/intent-autopsy/scripts/banned-words.txt` -- default banned words
4. Modified `intent_claude_skills` -- full directory install support
5. `tests/unit/test_autopsy.bats` -- BATS test suite
6. Updated documentation (CHANGELOG, user guide, reference guide, deployment guide)

## Related Steel Threads

- ST0020: Skills system (foundation this builds on)

## Context for LLM

This steel thread added the sixth Intent skill. Unlike the other skills (which are always-on enforcement rules), `intent-autopsy` is an analysis/diagnostic skill invoked on-demand via `/intent-autopsy`. It requires an Elixir script installed alongside the SKILL.md, which motivated extending the skills install mechanism to copy full directories.
