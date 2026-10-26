---
description: "Session finish (fold): update ST docs, wip.md, restart.md, commit cleanly. localfold = per-workstream before a compact; globalfold = project-wide before EOD"
chains_to: ["in-whiteboard", "in-verify"]
---

# Session Finish

End-of-session wrap-up. Ensure all state is captured so the next session can pick up cleanly.

## Fold scopes: localfold and globalfold

"Fold" is shorthand for this wrap-up (compact, tidy, snapshot). Two scopes, coined in the Lamplight project:

- **localfold** -- per-workstream tidy before a **compact** or context reset. Fold only THIS workstream's own Intent docs and whiteboard node: its ST/WP docs, its in-progress state, and (in a whiteboard project) its own-node `archive` of items, and of the messages it sent that their recipients have handled (`intent wb archive message --handled`). In a whiteboard project it also removes the worktree of each of its units that has landed, after the checks in `/in-whiteboard`'s Folding section. Scope is just you; you do not touch project-wide docs or other nodes. **IT DOES NOT `release`.** A compact is not the end of a session, so the node's `status` stays `active` -- `/in-whiteboard` invariant 6 says so and its red-flag table names setting `paused` here as the error. Release belongs to a session that is actually ending.
- **globalfold** -- project-wide tidy before **end of day**, when all workstreams close out. Fold the shared project tracking docs the project keeps (`intent/wip.md`, and `intent/restart.md` and `done.md` where it keeps them) into a coherent snapshot. In a multi-node setup this is typically the coordinating / validation workstream's job, not every node's.

When the human says "localfold", run the wrap below scoped to your own workstream; when they say "globalfold", run the project-wide version across the shared docs. A solo project has one workstream, so local and global coincide. In a whiteboard project, `/in-whiteboard` adds what a fold may and may not archive on a board.

## Procedure

### 1. Release the whiteboard

If `intent/whiteboard/` exists in the project root **and this session is actually ending**, invoke `/in-whiteboard release`. This sets your node's board `status: paused` and refreshes its heartbeat before any doc updates are committed. **If you are folding before a `/compact` or a context reset, DO NOT release** -- run `/in-whiteboard touch` instead and leave `status: active`. The discriminator is whether the SESSION ends, never whether a fold happened. Before releasing, consider whether any entries in your node's `## Decisions` should be migrated into `wip.md` / `done.md` for permanent record -- the whiteboard is the live channel, `wip.md` / `done.md` are the snapshots. If the directory doesn't exist, skip silently.

### 2. Update steel thread docs

For each ST/WP worked on this session:

- Write the thread's objective and context through `intent set <ID> objective|context --from <file>`
- Update the attached companion documents the thread carries (`design.md`, `impl.md`, `tasks.md`) with as-built status; `intent st edit <ID> <name> --path` names each and refuses one the thread does not carry, and `intent st attach <ID> <name>.md --from <file>` adds one
- Closing a thread or WP? `intent st done` / `intent wp done` refuse while its `acceptance.md` contract is BLOCKED (the close-gate). Cover or satisfy the remaining ACs first; the ST-level sign-off AC is the verifier's. See `working-with-llms.md` (D11).
- **An AC you are not going to do has two honest exits, and neither is satisfying it.** If it moved to another thread, `intent ac descope <id> <AC> --to <ID>`; if it was dropped, `intent ac withdraw <id> <AC> --reason "..."`. Both are non-blocking and both stay on the record. Reach for them instead of the two alternatives that lose the truth: marking work done that was not, or deleting the line. `intent ac rescope` / `reinstate` undo them.
- **BLOCKED on an AT contract finding, not an unsatisfied AC?** The gate lints the AT rows too -- a row that fails the grammar, or a `green` AT citing a file that does not exist, is coverage that cannot be resolved. `intent at lint <id>` names each one; fix each row it names by hand with `intent at edit <id> <AT-id>`.
- `st done` / `wp done` close a unit whose `## Objective` is still `_(not yet written)_`, and warn when they do. A unit that closed with that warning closed without anyone having said what it was for -- write the sentence with `intent set <ID> objective --from <file>`, ideally before closing.

### 3. Update work-in-progress

Update `intent/wip.md` with:

- Current state of in-progress work
- What's next
- Its verblock, per `IN-PR-STYLE-005`

**`wip.md` IS DOING AND TODO ONLY. DONE WORK IS ILLEGAL IN IT.**

Finished work already has homes that outlive the session and cannot rot -- the commit, the closed issue, the satisfied AC, `done.md` where the project keeps one. Restating it in `wip.md` gives one fact a second home that nobody updates.

The test before you write a line here: **would this line still be true if the work were abandoned tomorrow?** DOING and TODO survive that question. _What was accomplished_ does not -- it is a claim about the past, and `wip.md` is not a log.

### 4. Update restart context, where the project keeps one

Where the project keeps an `intent/restart.md`, update it with:

- Key context the next session needs
- Any decisions made or deferred
- Pointers to relevant files and STs

A `.claude/restart.md`, where the project keeps one, is the ENTRY POINT and holds NO STATE: it says where to start and points at the files above. Leave it alone unless the entry procedure itself changed. One fact in two of these files gets two values, so if you find yourself writing a supersedes banner, delete what it supersedes instead.

### 5. File quality checks

- The formatting standards in `/in-essentials` hold in every file the wrap-up touches.
- No Claude signature in commit messages.

### 6. ONLY update .md doc files

Do NOT write new code during session finish. This step is documentation only. Commit the documentation updates, exactly the paths the verbs you ran printed, as `/in-essentials` says.

## Skill Chain

Before finishing, consider:

- `/in-whiteboard release` -- pause this session's whiteboard node (step 1 runs it when `intent/whiteboard/` exists AND the session is actually ending; a fold before `/compact` runs `touch` instead)
- `/in-verify` -- verify any completion claims made this session

## Red Flags

| Rationalization                          | Reality                                                      |
| ---------------------------------------- | ------------------------------------------------------------ |
| "I'll update restart.md next session"    | Next session won't have this context. Write it now.          |
| "The code speaks for itself"             | Code changes without docs are invisible to the next session. |
| "Just one more quick fix before wrap-up" | Finish means finish. No new code.                            |
