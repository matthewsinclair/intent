---
description: "Session handoff: generate handoff doc for future sessions"
---

# Session Handoff

Generate a handoff document that captures what was accomplished, key decisions, and context for future agents or sessions. Handoff docs are permanent records stored in `intent/.handoff/`.

## Procedure

### 1. Run the prep script

Run the helper script to determine the filename and gather git context:

```bash
bash "$(find ~/.claude/skills/in-handoff -name handoff-prep.sh 2>/dev/null | head -1)" [optional-slug]
```

The script outputs key=value pairs and delimited blocks:

- `HANDOFF_FILE` -- proposed output path
- `TODAY` -- date in YYYYMMDD format
- `SEQ` -- zero-padded sequence number for today
- `SLUG` -- sanitized slug
- `BRANCH` -- current git branch
- `GIT_DIFF_STAT` block -- summary of files changed
- `GIT_LOG_RECENT` block -- recent commit messages

If no slug is provided, use the current session name or derive a short kebab-case topic slug from the main subject of this session (e.g., `eth-brownie-optimization`, `filter-sol-wsol-command`).

### 2. Confirm with user

Present the proposed filename and ask the user to confirm or correct:

- The slug (descriptive label for the filename)
- The scope (maybe only part of the session should be captured)

If the user provides a different slug, re-run the script with the corrected slug or adjust the path manually.

Wait for confirmation before proceeding.

### 3. Gather session context

Collect information for the handoff document:

- Read `intent/wip.md` for current work state (if it exists)
- Read `intent/restart.md` for session context (if it exists)
- Review the git log and diff stat from the prep script output
- Check `intent st list` for steel threads worked on this session
- Ask the user what key decisions were made and what context matters most

### 4. Write the handoff document

Create the file at the confirmed path. Ensure `intent/.handoff/` exists (the prep script creates it, but verify).

Use this template structure:

```markdown
---
date: YYYYMMDD
session: NNN
slug: <slug>
steel_threads: [STNNNN, ...]
---

# Handoff: <descriptive title>

## Summary

One to three sentences describing the overall session outcome.

## What Was Accomplished

- Concrete deliverables with file paths and ST/WP references
- Each item should be specific and verifiable

## Key Decisions

- Decision made and rationale
- Alternatives considered and rejected
- Decisions deferred and why

## Important Context

Context future sessions need that is not obvious from the code:

- Architectural constraints or trade-offs
- Known issues or technical debt introduced
- Dependencies on external systems or pending work

## Files Changed

[git diff --stat output or curated list of key files]

## Next Steps

- What the next session should pick up
- Blockers or prerequisites
- Suggested approach if non-obvious
```

### 5. Confirm with user

Show the user the completed handoff document path and a brief summary. Ask if any corrections are needed.

Do NOT commit the file automatically. The user will commit when ready.
