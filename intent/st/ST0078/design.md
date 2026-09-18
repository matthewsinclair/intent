# ST0078 — Design: Intent on a multi-person project with a git workflow

**Status: PROPOSAL for hv's review, 2026-09-18. Nothing here is built, and nothing is ruled until hv rules it.** Written by vc under the close-out pen after driving the two-clone case and reading the code paths it touches. Each claim says whether it was measured or read.

## The question

How does Intent work when more than one human uses it through a shared git repository, with branches, commits and pull requests? The store is a local SQLite database on each machine. What happens to it when a PR changes the model?

## What holds today

The model is per-artefact canon in git and a per-machine store rebuilt from it. `intent/.canon/st/<ID>.json` and `intent/.canon/issues/<NNNN>.json` are committed, one file per thread and per issue; `intent/.cache/intent.db` is gitignored and is truth on the machine that holds it (D01 as reversed, D34: the extract is the interchange). A fresh clone's store is cold and is rebuilt from canon through the ingest gate on first use. The single-file alternative was rejected when the layout was chosen, as a merge-conflict generator (ST0056 `realisation.md`, option B), and a binary database in git was rejected because it cannot clone, merge or revert (ST0056 `design.md`, alternatives).

**Measured on 2026-09-18** with the 3.0.3 pair, a fresh `intent init` estate, a bare origin, two clones, an isolated HOME and NO daemon (`scratchpad/st0078-drive.log`):

1. A fresh clone works: the store rebuilds from canon on the first verb. Intent's own CI does this on every PR.
2. Thread and issue ids are minted highest-plus-one over the LOCAL canon (`facade.rs` `next_thread_id`, `next_issue_number`). Two clones minted the same `ST0001` and the same issue `0001`. Git refused each merge with an add/add conflict on the canon file and on the generated views beside it. Loud, never silent. No verb repairs it.
3. Two people writing different fields of one thread conflict on adjacent JSON lines (`objective` and `context`). Loud; resolved by hand-editing the JSON and `intent sync --to-store <ID>`, which `docs/concepts/the-store.md` already documents for a hand edit.
4. **After a pull the store is stale and stays stale.** After a fast-forward pull bringing a new thread, `intent st show ST0002` answered `no steel thread ST0002 in this project` at rc=1, and `st show ST0001` answered with the pre-pull title. `intent doctor` printed 0 findings both times: the store-stale check fired, and it lives in the hidden advisory tier that only `--verbose` prints. Its remedy says to run a bare `intent sync`; `the-store.md` says a bare `intent sync` "is almost never what you meant". `intent sync --to-store` repaired it, previewing the pulled thread as an overwrite.
5. Issue numbers collide the same way as thread ids, with `intent/issues/<NNNN>.md` conflicting beside the canon.

**Read, not driven:**

6. With `intentd` watching the project, a pull is ingested: the watcher debounces the checkout burst and runs the non-destructive ingest, which takes the disk wherever its bytes differ from what the store recorded for that file (`intentd/src/watch.rs`, `ingest.rs` `disk_takes`, issue 0216). The daemon watches registered projects only and the CLI is in-process by default, so a collaborator who never registered the project gets case 4.
7. No post-merge, post-checkout or post-rewrite hook exists in the templates, the docs or the code.
8. The event log has one home, the store (D53, hv 2026-08-20): the disk form was deleted and untracked, `intent export` produces it on demand, and history does not travel between clones. Every event's `principal` reads `local` today, so even a travelling event would not say who.
9. Generated views are committed (`intent/st/steel_threads.md`, `intent/todo.md`, each realised thread's views), so two PRs that both touch threads conflict in them, and no document says the resolution is to take either side and regenerate.
10. `intent/.canon/project.json` carries one field, `todo_watermark`; two branches that both flush conflict on it, and the semantic merge is the later value.
11. The strict canon reader refuses a schema string it does not know, so a team must run compatible Intent versions; `config.json` records `intent_version` and nothing checks it across a team.

## Assessment

Not trivial. The single-machine model holds. The two-clone model has three gaps that are design rather than documentation: nothing reconciles the store after a pull unless a daemon happens to be watching, and the only detector is hidden (4, 6, 7); ids are minted with no coordination and nothing repairs a collision (2, 5); history does not travel (8). Items 3, 9, 10 and 11 are documentation.

## Proposal

### P1 — The event log travels (reverses D53)

Every event is written as its own committed file under `intent/.canon/events/<YYYY>/<MM>/<DD>/<ulid>.json`, in the same write set as the canon and views of the mutation that produced it, so an act and its record land together or not at all. The id is already a ULID (`event.rs` `next_id`), so a file is unique and time-sortable by name; the dated directories bound directory size, and a day directory is a natural unit to read or prune. One file per event is merge-free by construction: no two writers ever touch one file.

Ingest is additive: a committed event file whose id the store does not hold is inserted; one it holds is skipped; a file never changes; nothing is deleted from `event_log` because its file is absent, since a local act that was never committed is still a fact. `intent export` keeps producing the single-file form on demand.

**The boundary that keeps D01 intact:** canon stays the STATE extract and events travel as the ACT record. Nothing rebuilds state by replaying events, and doctor never reconciles the two, so there is no second truth. Doctor checks that each event file parses, that its id matches its name, and that the store holds every committed event, reporting the last as store-stale.

`principal` must become the author for this to mean anything: the project's `author` from `config.json` or git's `user.name` and `user.email`, with `local` kept only where neither exists. `intent upgrade` removes the `intent/events.jsonl` ignore rule (`.gitignore:149`) and adds nothing, because `intent/.canon/` is already tracked and guarded (`canon-ignore-guard.sh`). Size M.

### P2 — A renumber verb

`intent st renumber <old> <new>` and `intent issues renumber <old> <new>`, refusing when `<new>` exists. Each rewrites the canon file and its name, the realised directory and views, the `.intentfiles` rows, every `related` reference in other threads, whiteboard claims, and the attachment paths the store records; emits its own event; and prints the prose references it found with `intent search` and did NOT rewrite, because prose is authored. The collision itself stays git's to catch, which the drive showed it does, loudly. Size S to M.

Rejected: changing the minting scheme. `ST####` is the identity every reference and every reader uses, and a scheme change is a project-wide migration to avoid a conflict git already refuses.

### P3 — The store after a pull

Three parts, smallest first.

- **Store-stale moves out of the hidden tier** into shown-not-counted, the tier `report.unattached` uses: the exit code is untouched and the line prints on a default run. It was hidden because it fires during a peer's canon write on a shared tree (issue 0313); it keeps saying so, and one advisory line is the right cost for never again answering `no steel thread` after a pull. XS.
- **A CLI door for the non-destructive ingest.** The daemon's pass is `Load::Ingest`, reachable today only through the daemon; the CLI's `--to-store` is `Load::Restore`, the declared destructive direction. `intent sync --ingest` (name open) runs the daemon's rule from the command line: take the disk only where it says something the store did not write, under the hold-unless-moved lock issue 0441 built, so it is safe beside a running daemon and beside a peer's write. S.
- **Git hooks that run it.** `post-merge`, `post-checkout` and `post-rewrite`, wired by `intent claude upgrade --apply` the way the pre-commit gate is (a region-edited chain in `.git/hooks`, per clone, which is why every fresh clone runs `claude upgrade --apply` once already). The hook runs `intent sync --ingest`, prints one line when it took anything, and always exits 0: a hook must never fail a checkout. Where a daemon is also watching, both passes run the same engine under the same lock and the second finds nothing to take. S.

`the-store.md` changes its sentence: after a pull, `sync --ingest` (which the hook runs for you); `--to-store` is the restore and is still almost never what you meant.

### P4 — Documentation: working in a team

One page, `docs/concepts/working-in-a-team.md`: what travels and what does not; what a reviewer reads in a PR (the canon extract) and what to review in it; the pull and the store (P3); id collisions and the repair (P2); JSON conflicts and `sync --to-store <ID>`; generated-view conflicts and regeneration (`intent st sync --write`, `intent todo update`, `intent organize --apply`); the watermark; version compatibility; the `.gitignore` lines a project needs; and a CI job that builds `intent` and runs `intent doctor` on the merge result, because a merge made on the forge is judged by nobody's commit gate. Size S, written from driven commands rather than composed.

## Sequencing

P3 first: it closes the measured failure and is the smallest. Then P2, then P1 with the most design in it, and P4 alongside each. P1 and P3 are project-wide (an ignore rule and hook wiring reach every estate through `intent upgrade` and `claude upgrade --apply`), so the fleet trawl waits for them and runs once, on the pair that carries them. Recommended: all of ST0078 in 3.1.0, the trawl after the pair is rebuilt, the cut after the trawl. The cost is that the cut waits for an M to L build; the standing ruling is that completeness beats schedule.

## What does not change

D01 as reversed: the store is truth on its machine. D34: the extract is the interchange. Per-artefact canon. `ST####` and `NNNN` as identities. The daemon stays optional. Concurrency on ONE tree (five nodes, one store) is already handled by the store's locks and the daemon, and is not this thread's subject.

## Open questions for hv

1. Reverse D53, and on the boundary written in P1 (state in canon, acts in events, no replay, no reconciliation)?
2. Sharding by date, as proposed, or by id prefix? Date is what a reader and a prune want; a hash spreads more evenly and means nothing to a human.
3. The hooks are wired by `claude upgrade --apply`, the door the pre-commit gate already uses, rather than a new verb. Keep one door?
4. The flag name for the non-destructive ingest.
5. Does ST0078 go into 3.1.0, with the trawl and the cut waiting for it?

## Acceptance sketch

To be minted with `intent ac new` after ratification, one row per user-facing behaviour: a pull with no daemon is reflected by the next verb; a stale store is reported on a default `doctor` run; two clones that mint one id are repaired by one verb; an act on one clone is readable as an event on another after a pull, with its author; and every command on the team page has been driven.
