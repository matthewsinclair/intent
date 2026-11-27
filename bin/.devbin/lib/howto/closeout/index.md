---
summary: Close out a project after a stretch of work, so done work is closed, unfinished work is parked, and the docs say only what is true.
keywords: [closeout, close-out, wrap-up, fold, prune, park, lean, shutdown]
---

# How to close out a project

Use this topic when the user asks you to close out a project, to draw a line under a stretch of work, or to put done work to bed. A close-out leaves the project with:

- each done item closed;
- each unfinished part parked as future work that has not started;
- docs that say what the code does, and nothing more;
- status files that hold only current work and work to do;
- each change committed and pushed.

A close-out is not a release. Make no tag and no release unless the user asks for one.

## How to read this topic

Each numbered step applies to every project. A block with a label applies only where the label is true:

- **With Intent:** the project tracks its work with Intent (threads, work packages and issues).
- **In a team:** more than one agent works on the project, each in its own seat, with a mediator between the team and the user.
- **With devbin:** the project builds and tests through `bin/devbin`.

Skip a block whose label is not true for the project. In a project with no tracker, an "item" is an entry in the list that the project keeps, eg a TODO file.

## Before you start

1. Keep the user's words that started the close-out. Quote them in your report.
2. Read the project's own rules, eg `AGENTS.md`, `CLAUDE.md` and `usage-rules.md`. They win over this topic.
3. Read the clock with `date -u` before you write a timestamp.

**In a team:**

- Take your brief from the mediator. Each seat closes out its own remit and writes only its own board.
- Agree which docs are each seat's, by file or by line range, and an order for commits to a shared file. A commit by path takes the whole file.

## 1. Stop new work

1. Start no new item.
2. Stop the work that started after the close-out began, and park it in step 4.
3. Ask for no test run except the ones that this topic names.

## 2. Find all open and owed work

1. List each open item.
2. Read the closed items too. Look for parts that they still owe, eg the words "owed", "deferred", "later", "follow-up" and "stays open".
3. Check each hold and TODO against the world, not only against the list. Parked work can live only as a TODO line.
4. Helpers can read in parallel. Give each helper its own set of files and one shared rules file.

**With Intent:**

- Run `intent outstanding` and `intent issues list`. `intent outstanding` shows only open issues and the threads and work packages in WIP.
- Run `intent st list --status all` to find a thread in triage or on hold. The token for a thread that has not started is `tbc`.
- `intent ac descope --to` records only a pointer. Make sure that the target has the criterion.

**In a team:**

- Read each board's holds and TODO lines. An old hand-off may never have reached its seat.

## 3. Sync the docs to the code

1. Read each hand-written doc against the code as built, eg the README, the docs directory, the architecture and usage files, and the help pages.
2. Fix each line that the code does not bear out. Read the code that the line names before you fix it. Add no new doc.
3. A doc that is a contract between projects changes only when its parties agree. Park its gaps in step 4, and do not edit it.
4. The sync finds code defects. Change no code. Park each defect in step 4.
5. Base each cut on a full search. A truncated search result can hide a reader of the text.
6. Run the project's formatter and its text checks on the changed docs.

**In a team:**

- Give each doc one owner. The mediator takes the docs that have no owner, eg `usage-rules.md` and the restart files.
- Make the edits in a separate worktree or clone.

## 4. Close, prune and park

1. Done: close it.
2. Partly done: prune it to its done part, and close it. Move each unfinished part into future work that has not started. Name the new home in the close reason.
3. No work done: move it into future work that has not started.
4. An item stays on hold only when the user put the hold on it. Keep the condition that releases it.
5. Put the parked work into a few groups, each with one theme, eg "the next release", "code defects that the doc sync found" and "parked until the user schedules it".
6. A branch whose work is all on main is spent: `git cherry main <branch>` shows only "-" lines. Ask the user before you delete a branch.

**With Intent:**

- Use the CLI for each change, eg `intent wp done`, `intent st done` and `intent issues close --reason`. Never run `intent fc`.
- Future work is a thread or a work package that has not started. Close a moved issue with a reason that names its new home.
- `intent st done` is legal only from WIP. Start a thread in triage before you close it.
- A work package in a thread that is older than acceptance criteria can need a criterion before `intent wp done`.
- The views of closed records leave the tree only by `intent organize --apply`. If your permission check refuses it, the user runs it.

## 5. Lean out the docs

1. A status file holds only current work and work to do, one short line each, and never done work.
2. Done work lives in git, in the closed items and in the release notes. Do not write it again in a doc.
3. Remove a doc that only repeats another doc, or that describes nothing in the repository.
4. Cut stale and repeated text from the other docs. Keep what a reader needs to use or change the code.

**With Intent:**

- The status files are wip.md, the restart file and each verblock. A verblock is one short line.
- Remove stale seed files that an Intent upgrade wrote. Lean `usage-rules.md` and do not remove it, because `intent claude upgrade` writes it again when it is absent.

**In a team:**

- On each board, archive each executed decision and each done item. Keep the live rules, DOING, TODO, the holds with their conditions, and the watch-outs that still apply. Clear the messages that you handled.
- Leave each `.history/` directory as it is. It is append-only, and the pre-commit gate refuses a removal in it.

## 6. Prove the changed docs

1. A doc that a test reads needs that test's run. Find the tests that name the doc's path. Add by hand the tests that read it another way, eg through a help page, a `--help` or a version string in a README.
2. Run the project's tree scans too. A doc edit can make a scan fail, eg when a shipped page names a repository path or an issue number.
3. Run each test file on its own, or check the count of tests that ran. A runner that takes one path can run only the first of several and still show green.
4. Count a skipped test as not run, never as a pass.

**With devbin:**

- Run each test through `bin/devbin`, never through a language's own test tool.

**In a team:**

- Each seat names the tests that read its files. The mediator merges the lists into one run, and plans its weight from the tests, not from the files.
- Release no seat until this run is green. A hard lean can cut words that a test needs.
- Where the host has a slot controller, book each run with `intent estate slots ask` first.

## 7. Commit and push

1. Commit each change by path through the project's gate, eg `git commit --only <paths>`. Never use `--no-verify`.
2. `git commit --only` refuses an untracked path, so `git add` a new file first.
3. Commit the final status files before the push, so that the push carries them.
4. Push only on the user's word or a standing approval, fast-forward only, to the project's own remotes. Never push to a remote that belongs to someone else. The push rule can differ by project, so read each project's own.
5. If your permission check refuses a push, give the user the exact push line to type. Never ask another session to push for you.
6. Read the CI result after the push.

**With Intent:**

- Commit the files that the verbs wrote, eg the views that a thread verb renders.

**With devbin:**

- If a devbin update is due in the project, let it land first, so that one push carries both.

**In a team:**

- Commit when no other seat is committing. A long gate can lose a race for HEAD, and seats that commit together meet git's index lock. Try the commit again.
- Commit the board files that `intent wb release` writes.

## 8. Report

1. Send one CLOSED-OUT report to the user. Give: what closed; what moved, with the id of each new home; the docs that changed; the commit ids; what is local only; and anything that is still open.
2. Add the steps that you took, any step that this topic missed, and anything that went wrong.

**In a team:**

- The report goes to the mediator, and the mediator reports to the user.
- Each seat that shuts down commits its last board lean, runs `intent wb release --node <seat>`, and reports that it is ready to shut down. The user closes its window.
- A seat with no session cannot fold or release. Name its stale board in the report.
- A seat that shuts down can owe a message to a session that is not running. The mediator keeps it.
- When you write as the user's node, pass `--session none`, so that your session is not recorded on the user's node.

## What only the user can do

- A push without a standing approval.
- A security setting, eg a package manager's trust entry. Your permission check refuses it, so the user types it.
- A delete outside the project's tools.
- A decision whose premise proves false. Put it back to the user.
- The user's own uncommitted files, also in another project's tree: ask whether to commit them or drop them.
