---
description: "Session start: read restart files, review STs, orientation overview before coding"
---

# Session Start

Read-only orientation at the beginning of a new Claude Code session. Understand where we are before doing anything.

## Procedure

### 1. Read restart context

Read these files (skip any that don't exist):

- `.claude/restart.md`
- `intent/restart.md`
- `intent/wip.md`

### 2. Read project rules

- `CLAUDE.md`
- `intent/llm/MODULES.md`
- `intent/llm/DECISION_TREE.md`

### 3. Review open steel threads

Run `intent st list` to see in-progress work.
Run `intent st list --status not-started` to see queued work.

### 4. Provide orientation overview

Summarize:

- Where we left off (from restart files)
- What's currently in progress
- What's queued and ready to start
- Suggested plan forward

### 5. Wait for instructions

DO NOT WRITE ANY CODE. This is a read-only orientation step. Wait for the user to direct what to work on.
