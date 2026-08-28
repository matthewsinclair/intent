---
description: "Session finish (fold): update ST docs, wip.md, restart.md, commit cleanly. localfold = per-workstream before a compact; globalfold = project-wide before EOD"
chains_to: ["in-whiteboard", "in-verify"]
---

# Session Finish

End-of-session wrap-up. Ensure all state is captured so the next session can pick up cleanly.

## Fold scopes: localfold and globalfold

"Fold" is shorthand for this wrap-up (compact, tidy, snapshot). Two scopes, coined in the Lamplight project:

- **localfold** -- per-workstream tidy before a **compact** or context reset. Fold only THIS workstream's own Intent docs and whiteboard node: its ST/WP docs, its in-progress state, and (in a whiteboard project) its own-node `archive` + `release`. Scope is just you; you do not touch project-wide docs or other nodes.
- **globalfold** -- project-wide tidy before **end of day**, when all workstreams close out. Fold the shared project tracking docs (`intent/wip.md`, `intent/restart.md`, `.claude/restart.md`, `done.md`) into a coherent snapshot. In a multi-node setup this is typically the coordinating / validation workstream's job, not every node's.

When the human says "localfold", run the wrap below scoped to your own workstream; when they say "globalfold", run the project-wide version across the shared docs. A solo project has one workstream, so local and global coincide.

## Procedure

### 1. Release the whiteboard

If `intent/whiteboard/` exists in the project root, invoke `/in-whiteboard release`. This sets your node's board `status: paused` and refreshes its heartbeat before any doc updates are committed. Before releasing, consider whether any entries in your node's `## Decisions` should be migrated into `wip.md` / `done.md` for permanent record -- the whiteboard is the live channel, `wip.md` / `done.md` are the snapshots. If the directory doesn't exist, skip silently.

### 2. Update steel thread docs

For each ST/WP worked on this session:

- Update `tasks.md` with completed and remaining tasks
- Update `design.md` with as-built status (if design changed)
- Update `impl.md` with implementation notes (if applicable)
- Move completed tasks from `tasks.md` to `done.md` if that file exists
- Closing a thread or WP? `intent st done` / `intent wp done` refuse while its `acceptance.md` contract is BLOCKED (the close-gate). Cover or satisfy the remaining ACs first; the ST-level sign-off AC is the verifier's. See `working-with-llms.md` (D11).
- **An AC you are not going to do has two honest exits, and neither is satisfying it.** If it moved to another thread, `intent ac descope <id> <AC> --to <ID>`; if it was dropped, `intent ac withdraw <id> <AC> --reason "..."`. Both are non-blocking and both stay on the record. Reach for them instead of the two alternatives that lose the truth: marking work done that was not, or deleting the line. `intent ac rescope` / `reinstate` undo them.
- **BLOCKED on an AT contract finding, not an unsatisfied AC?** The gate lints the AT rows too -- a row that fails the grammar, or a `green` AT citing a file that does not exist, is coverage that cannot be resolved. `intent at lint <id>` names each one; `--fix` migrates what is mechanical.
- `st done` / `wp done` also **warn** (never block) if `## Objective` is still the template placeholder. If it fires, the unit is closing without anyone having said what it was for -- write the sentence.

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

`.claude/restart.md` is the ENTRY POINT and holds NO STATE. It says where to start and points at the two files above; it does not carry WIP, TODO, focus, or a summary of current work. Leave it alone unless the entry procedure itself changed.

**This is a rule with a history.** These three files were three copies of one narrative, each opening with a banner declaring it superseded the others -- which is the tell that nobody was deleting, only prepending. They were folded back into one on 2026-08-24. **State written here is how that rebuilds**, because the same content then has three homes and three values, and nothing reports the divergence.

**If you find yourself writing a supersedes banner, delete what it supersedes instead.**

### 5. File quality checks

- No non-printing characters in any files (proper emojis and ASCII only)
- All markdown tables are column-aligned
- No Claude signature in commit messages

### 6. ONLY update .md doc files

Do NOT write new code during session finish. This step is documentation only. Commit the documentation updates.

## Skill Chain

Before finishing, consider:

- `/in-whiteboard release` -- pause this session's whiteboard node (fires automatically as step 1 if `intent/whiteboard/` exists)
- `/in-verify` -- verify any completion claims made this session

## Red Flags

| Rationalization                          | Reality                                                      |
| ---------------------------------------- | ------------------------------------------------------------ |
| "I'll update restart.md next session"    | Next session won't have this context. Write it now.          |
| "The code speaks for itself"             | Code changes without docs are invisible to the next session. |
| "Just one more quick fix before wrap-up" | Finish means finish. No new code.                            |
