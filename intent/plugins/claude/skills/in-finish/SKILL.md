---
description: "Session finish: update ST docs, wip.md, restart.md, commit cleanly"
---

# Session Finish

End-of-session wrap-up. Ensure all state is captured so the next session can pick up cleanly.

## Procedure

### 1. Update steel thread docs

For each ST/WP worked on this session:

- Update `tasks.md` with completed and remaining tasks
- Update `design.md` with as-built status (if design changed)
- Update `impl.md` with implementation notes (if applicable)
- Move completed tasks from `tasks.md` to `done.md` if that file exists

### 2. Update work-in-progress

Update `intent/wip.md` with:

- What was accomplished this session
- Current state of in-progress work
- What's next

### 3. Update restart context

Update `intent/restart.md` with:

- Key context the next session needs
- Any decisions made or deferred
- Pointers to relevant files and STs

Rewrite `.claude/restart.md` with:

- WIP/TODO focus for Claude Code startup
- Concise pointers to current work

### 4. File quality checks

- No non-printing characters in any files (proper emojis and ASCII only)
- All markdown tables are column-aligned
- No Claude signature in commit messages

### 5. ONLY update .md doc files

Do NOT write new code during session finish. This step is documentation only. Commit the documentation updates.
