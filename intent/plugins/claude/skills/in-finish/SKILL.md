---
description: "Session finish: update ST docs, wip.md, restart.md, commit cleanly"
chains_to: ["in-whiteboard", "in-verify"]
---

# Session Finish

End-of-session wrap-up. Ensure all state is captured so the next session can pick up cleanly.

## Procedure

### 1. Release the whiteboard

If `intent/whiteboard/` exists in the project root, invoke `/in-whiteboard release`. This sets your stream's `status: paused` and refreshes its heartbeat before any doc updates are committed. Before releasing, consider whether any entries in your stream file's `## Recent decisions affecting other streams` should be migrated into `wip.md` / `done.md` for permanent record -- the whiteboard is the live channel, `wip.md` / `done.md` are the snapshots. If the directory doesn't exist, skip silently.

### 2. Update steel thread docs

For each ST/WP worked on this session:

- Update `tasks.md` with completed and remaining tasks
- Update `design.md` with as-built status (if design changed)
- Update `impl.md` with implementation notes (if applicable)
- Move completed tasks from `tasks.md` to `done.md` if that file exists
- Closing a thread or WP? `intent st done` / `intent wp done` refuse while its `acceptance.md` contract is BLOCKED (the close-gate). Cover or satisfy the remaining ACs first; the ST-level sign-off AC is the verifier's. See `working-with-llms.md` (D11).

### 3. Update work-in-progress

Update `intent/wip.md` with:

- What was accomplished this session
- Current state of in-progress work
- What's next

### 4. Update restart context

Update `intent/restart.md` with:

- Key context the next session needs
- Any decisions made or deferred
- Pointers to relevant files and STs

Rewrite `.claude/restart.md` with:

- WIP/TODO focus for Claude Code startup
- Concise pointers to current work

### 5. File quality checks

- No non-printing characters in any files (proper emojis and ASCII only)
- All markdown tables are column-aligned
- No Claude signature in commit messages

### 6. ONLY update .md doc files

Do NOT write new code during session finish. This step is documentation only. Commit the documentation updates.

## Skill Chain

Before finishing, consider:

- `/in-whiteboard release` -- pause this session's whiteboard stream (fires automatically as step 1 if `intent/whiteboard/` exists)
- `/in-verify` -- verify any completion claims made this session

## Red Flags

| Rationalization                          | Reality                                                      |
| ---------------------------------------- | ------------------------------------------------------------ |
| "I'll update restart.md next session"    | Next session won't have this context. Write it now.          |
| "The code speaks for itself"             | Code changes without docs are invisible to the next session. |
| "Just one more quick fix before wrap-up" | Finish means finish. No new code.                            |
