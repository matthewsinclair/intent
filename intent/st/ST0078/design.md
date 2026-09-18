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

## Worked examples

**Every command below was RUN, in this order, on 2026-09-18** with the pair at `47e269483` (3.0.3 plus the SQLite bump): two clones, Alice and Bob, against a bare origin, no daemon, an isolated HOME. The script and its log are banked at `refs/bank/vc/st0078/drive.sh` and `refs/bank/vc/st0078/drive.log`; the output quoted here is abbreviated from that log. Commands marked PROPOSED do not exist yet and are what P1 to P3 would add.

### E1 — Starting a shared project, and joining one

Alice, once for the project:

```
$ intent init Team
created: Team at .../alice
$ intent claude upgrade --apply --skip-settings     # wires the commit gate into THIS clone's .git/hooks
$ intent bootstrap                                   # once per MACHINE: records the Intent install the gate execs
$ intent sync --to-disk                              # needed on 3.0.3 only: init wrote neither aggregate view (0448, fixed 2026-09-18 at 21d1a6652)
$ git add -A && git commit -m "intent init" && git push -u origin main
```

`intent init` writes the three ignore lines a project needs (`intent/.cache/`, `intent/events.jsonl`, `intent/.backup/`), so nothing per-machine can reach the repository; `docs/concepts/the-store.md` still says it does not, and P4 corrects that. On 3.0.3, without the `sync --to-disk`, the first commit was refused by the gate the previous line installed, because doctor counted the two absent aggregate views as skew; 0448 was filed from this drive and is fixed at 21d1a6652, so a project `init` creates now commits clean without it.

Bob, once per clone, then once per machine:

```
$ git clone <origin> bob && cd bob
$ intent claude upgrade --apply --skip-settings     # .git/hooks is never cloned, so every clone wires its own gate
$ intent bootstrap
$ intent st list --status all                        # the cold store loads from canon on the first verb
```

### E2 — A thread on a branch, reviewed as a PR

```
$ git switch -c alice/onboarding
$ intent st new "Onboarding guide"                   # created: ST0001
$ intent st start ST0001
$ intent wp new ST0001 "Write the guide"             # created: ST0001/01
$ git status --short
 M intent/.intentfiles
 M intent/st/steel_threads.md
 M intent/todo.md
?? intent/.canon/st/
?? intent/st/ST0001/
$ git add -A && git commit -m "ST0001: onboarding guide, WP-01" && git push -u origin alice/onboarding
```

What the reviewer sees, and what to review: the canon file is the change, the rest is rendered from it.

```
$ git diff --stat main...alice/onboarding
 intent/.canon/st/ST0001.json   | 23 ++++++   <- the model: review this
 intent/.intentfiles            |  1 +        <- ST0001 is realised on disk
 intent/st/ST0001/WP/01/info.md | 20 ++++++   <- generated views
 intent/st/ST0001/acceptance.md | 20 ++++++
 intent/st/ST0001/info.md       | 31 ++++++
 intent/st/steel_threads.md     |  5 ++--
 intent/todo.md                 |  3 ++-
```

The commit gate ran on Alice's machine and judged Alice's tree. The merge the forge makes is judged by nobody; P4's CI job (`intent doctor` on the merge result) is the answer.

### E3 — Pulling a merged PR

The first pull into a store that holds nothing is fine, because a store with nothing in it is treated as cold and loads from canon:

```
$ git pull
$ intent st list --status all
ST0001      | Onboarding guide              | WIP
```

Every pull after that leaves the store where it was. Alice's next thread, ST0002, is merged; Bob pulls it:

```
$ git pull
$ intent st list --status all
ST0001      | Onboarding guide              | WIP           <- ST0002 is in the tree and not in the answer
$ intent st show ST0002
error: no steel thread ST0002 in this project
  remedy: run `intent st list` to see the threads this project has
$ intent doctor
doctor: 0 finding(s) across 2 thread(s) ... -- 2 advisory(ies), not counted
$ intent doctor --verbose | grep -A1 store-stale
advisory: store-stale -- 1 finding, not counted in the verdict
  intent/.cache/intent.db -- the runtime store does not match a rebuild from committed canon -- commands are answering from the store ...
```

What clears it today, and it is the whole-store restore:

```
$ intent sync --to-store
ok: store rewritten from the canon extract; nothing the store already held was overwritten
$ intent st list --status all
ST0002      | Release checklist               | Triage
ST0001      | Onboarding guide                | WIP
```

PROPOSED (P3): nothing to type. The `post-merge` hook runs `intent sync --ingest`, which takes ST0002 from the disk because the store never wrote it, and a default `intent doctor` run shows store-stale wherever the hook did not run.

### E4 — Two people mint the same id

Both clones are at the same commit; Alice's `st new` and Bob's `st new` both mint ST0003. Alice pushes first. Bob:

```
$ git pull --no-rebase
CONFLICT (add/add): Merge conflict in intent/.canon/st/ST0003.json
CONFLICT (content): Merge conflict in intent/st/steel_threads.md
CONFLICT (content): Merge conflict in intent/todo.md
$ git merge --abort
```

The repair today, by hand, because no verb does it: rename the canon file, edit its `id`, restore the store from the renamed canon, regenerate the views, then merge.

```
$ git mv intent/.canon/st/ST0003.json intent/.canon/st/ST0004.json
$ (edit "id": "ST0003" -> "ST0004" in intent/.canon/st/ST0004.json)
$ intent sync --to-store
warning: replacing the store from the extract OVERWRITES:
  ST0003: absent from disk, would be DELETED
ok: store replaced from the canon extract, taking the 1 difference(s) listed above
$ intent sync --to-disk && intent todo update
$ git add -A && git commit -m "renumber my ST0003 to ST0004"
$ git pull --no-rebase
CONFLICT (content): Merge conflict in intent/st/steel_threads.md   <- generated views only
CONFLICT (content): Merge conflict in intent/todo.md
$ git checkout --theirs -- intent/st/steel_threads.md intent/todo.md   # either side; they are regenerated next
$ intent sync --to-store && intent sync --to-disk && intent todo update
$ git add -A && git commit -m "merge main: Alice's ST0003, mine is ST0004" && git push
$ intent st list --status all
ST0004      | Bob's next                      | Triage
ST0003      | Alice's next                    | Triage
ST0002      | Release checklist               | Triage
ST0001      | Onboarding guide                | WIP
$ intent doctor
doctor: 0 finding(s) across 4 thread(s)
```

A thread that had been started is realised under `intent/st/ST0003/` and declared in `.intentfiles`, and both need the same rename by hand; this one was in Triage, so neither existed. PROPOSED (P2): `intent st renumber ST0003 ST0004` does the rename, the id, the realised directory, the register row, the `related` references and the store in one move, and prints the prose references it found and did not rewrite.

### E5 — Two people edit the same thread

Alice writes the objective; Bob, without pulling, writes the context:

```
alice$ intent set ST0001 objective "Bring a new engineer to a first merged PR in a day"
alice$ git add -A && git commit -m "ST0001 objective" && git push
bob$   intent set ST0001 context "Requested by support after three onboarding escalations"
bob$   git add -A && git commit -m "ST0001 context"
bob$   git pull --no-rebase
CONFLICT (content): Merge conflict in intent/.canon/st/ST0001.json
Auto-merging intent/st/ST0001/info.md
$ grep -n -E '^(<<<<<<<|=======|>>>>>>>)|"objective"|"context"' intent/.canon/st/ST0001.json
8:<<<<<<< HEAD
9:  "objective": "",
10:  "context": "Requested by support after three onboarding escalations",
11:=======
12:  "objective": "Bring a new engineer to a first merged PR in a day",
13:  "context": "",
14:>>>>>>> 44320e0f
```

The two fields are adjacent lines, so git cannot merge them; a person keeps both and then loads the merged canon:

```
$ (edit intent/.canon/st/ST0001.json by hand: keep both fields, drop the markers)
$ intent sync --to-store ST0001
warning: replacing the store from the extract OVERWRITES:
  ST0001: differs on disk
ok: ST0001 replaced from the canon extract, taking the 1 difference(s) listed above
$ intent sync --to-disk && intent todo update
$ git add -A && git commit -m "merge: both ST0001 edits" && git push
$ intent st show ST0001
objective:
  Bring a new engineer to a first merged PR in a day
$ intent doctor
doctor: 0 finding(s) across 4 thread(s)
```

Two things the drive showed that the team page must say. `sync --to-store` refuses while any generated view still carries a conflict marker (`residue: conflict-markers -- git conflict markers present; resolve the merge before Intent can read this file`), so the views are resolved, by taking either side, before the canon is loaded. And `sync --to-disk` is what regenerates `steel_threads.md` and the thread's views after a merge; `intent todo update` regenerates `todo.md`.

### E6 — History does not travel today

After every merge above, each clone holds only its own acts:

```
alice$ intent events --subject ST0001
2026-09-18T07:16:12.608Z  01M2SNYX60SPAR8G4BRYV2ZWJ3  st.new      ST0001
2026-09-18T07:16:12.622Z  01M2SNYX6D5F6ASM6FHX4SRA6E  st.start    ST0001
2026-09-18T07:16:16.519Z  01M2SNZ107HNA7RR8MYE6HCFHR  thread.set  ST0001
bob$   intent events --subject ST0001
2026-09-18T07:16:16.963Z  01M2SNZ1E2BSQRK8RBKE1CG3Z7  thread.set  ST0001
```

Bob's store does not know Alice created or started the thread, and neither store says who. PROPOSED (P1): after `git pull`, `intent events --subject ST0001` on either clone lists all four acts, each carrying its author.

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
- **A CLI door for the non-destructive ingest, as the first step of the plan P5 describes.** The daemon's pass is `Load::Ingest`, reachable today only through the daemon; the CLI's `--to-store` is `Load::Restore`, the declared destructive direction. Bare `intent sync` today refuses to guess a direction; under hv's ruling of 2026-09-18 it prints the plan for this clone instead, and `intent sync --apply` applies it. P3 builds that verb with one step in its plan, the ingest: the daemon's rule unchanged, the same engine and no second implementation, taking the disk only where it differs from what the store recorded writing, a recorded file the pull removed included, never deleting a row whose file was never written, under the hold-unless-moved lock issue 0441 built, so it is safe beside a running daemon and beside a peer's write. `--apply` combined with `--to-disk` or `--to-store` is refused. S.
- **Git hooks that run it.** `post-merge`, `post-checkout` and `post-rewrite`, wired by `intent claude upgrade --apply` the way the pre-commit gate is (a region-edited chain in `.git/hooks`, per clone, which is why every fresh clone runs `claude upgrade --apply` once already). The hook runs `intent sync --apply` with no terminal, which applies the quiet steps only, prints one line when it took anything or when something is left for a person, and always exits 0: a hook must never fail a checkout. `post-checkout` runs only on a branch checkout. Where a daemon is also watching, both passes run the same engine under the same lock and the second finds nothing to take. S.

`the-store.md` changes its sentence: after a pull, `intent sync --apply` (which the hook runs for you); `--to-store` is the restore and is still almost never what you meant.

### P4 — Documentation: working in a team

One page, `docs/concepts/working-in-a-team.md`: what travels and what does not; what a reviewer reads in a PR (the canon extract) and what to review in it; the pull and the store (P3); id collisions and the repair (P2); JSON conflicts and `sync --to-store <ID>`; generated-view conflicts and regeneration (`intent st sync --write`, `intent todo update`, `intent organize --apply`); the watermark; version compatibility; the `.gitignore` lines a project needs; and a CI job that builds `intent` and runs `intent doctor` on the merge result, because a merge made on the forge is judged by nobody's commit gate. Size S, written from driven commands rather than composed.

### P5 — One command after a pull

hv, 2026-09-18: nobody should type ten commands to bring a clone and its store back into step. The shape hv chose is `intent sync [--apply] [--to-disk|--to-store]`: bare `intent sync` prints the plan for this clone and writes nothing, `--apply` applies it, and the two explicit directions keep their meanings. The refusal bare `sync` gives today asked which direction you meant; the plan answers by reading the state rather than guessing, so no published meaning changes.

The plan is a pure function of the store, the tree and git's status; applying it is the impure half (PFIC). Its steps, in order, each a check with its repair:

- The branch is behind its upstream: said, and nothing done. Intent never runs `git pull`, `git commit` or `git push`.
- Unmerged paths. A generated view is regenerated from the merged canon and staged. A canon add/add is an id collision and the LOCAL id is renumbered to the next free one with P2's engine. Two constraints ic measured while building P2: mid-merge the old id's paths hold the OTHER side's canon file and directory, so the plan cannot call the plain verb, which would move and delete them; it renumbers the local thread in the store, writes the new id's canon and directory from the store, restores the pulled side at the old id and stages both. And "next free" asks the store AND the tree, because the store's own `next_thread_id` can return an id the tree holds from a pull not yet loaded, which the renumber then refuses. A canon content conflict asks for a side.
- The store lags the committed canon: P3's ingest, event files under P1 included.
- Views stale against the store: regenerated, the way `organize --apply` does it.
- The search index stale: refreshed, the incremental reconcile every `intent search` already runs before answering, never the full `index rebuild`. Quiet, because it changes only the index and loses no work (dc's reading, 2026-09-18).
- `doctor` last, and its verdict is the exit code.

Every step declares its recoverability, the field the whiteboard verbs already carry. A quiet step never asks: the ingest, a regeneration, the index refresh. A reversible step asks `y/N`, and `--yes` answers those: the renumber, staging what was regenerated. Generated views are regenerated AFTER the ingest, because a view renders from the store and the store holds the merged canon only once the ingest has run. A non-reversible step always asks a person and no flag answers for it, the rule `organize --default --force` already enforces: taking a side in a canon conflict, or anything that overwrites local store state. Without a terminal and without `--yes`, `--apply` runs the quiet steps, skips every ask, and prints one line naming what is left; that is what the hooks run, so the hook and the person share one engine. `--plan <digest>` is reused from `organize`: an `--apply` after the tree moved is refused.

The git boundary: the verb reads `git status` and the unmerged index, stages only the files it regenerated to resolve a conflict it was asked to resolve, and never pulls, commits or pushes. Without the staging the person is back to one `git add` per file. Size M, built after P2 and P3 land because it composes their engines.

## Sequencing

P3 first: it closes the measured failure and is the smallest. Then P2, then P5 on both, then P1 with the most design in it, and P4 alongside each and closing last, because it documents P5 as the one command. P1 and P3 are project-wide (an ignore rule and hook wiring reach every estate through `intent upgrade` and `claude upgrade --apply`), so the fleet trawl waits for them and runs once, on the pair that carries them. Recommended: all of ST0078 in 3.1.0, the trawl after the pair is rebuilt, the cut after the trawl. The cost is that the cut waits for an M to L build; the standing ruling is that completeness beats schedule.

## What does not change

D01 as reversed: the store is truth on its machine. D34: the extract is the interchange. Per-artefact canon. `ST####` and `NNNN` as identities. The daemon stays optional. Concurrency on ONE tree (five nodes, one store) is already handled by the store's locks and the daemon, and is not this thread's subject.

## Open questions for hv

1. Reverse D53, and on the boundary written in P1 (state in canon, acts in events, no replay, no reconciliation)?
2. Sharding by date, as proposed, or by id prefix? Date is what a reader and a prune want; a hash spreads more evenly and means nothing to a human.
3. The hooks are wired by `claude upgrade --apply`, the door the pre-commit gate already uses, rather than a new verb. Keep one door?
4. The flag name for the non-destructive ingest.
5. Does ST0078 go into 3.1.0, with the trawl and the cut waiting for it?

## Rulings

hv, 2026-09-18: 1 yes; 2 by date, `YYYY/MM/DD`; 3 yes, one door; 4 bare `intent sync` is the plan and `--apply` applies it, which P5 records; 5 yes, all of it in 3.1.0. The author on an event is git's identity first, then the config author, then `local`, because `config.json` is committed and would name one author for every clone (vc, under the pen, after cc measured it). hv, later the same morning, on vc's recommendations: machine-scoped events (heartbeats, ingests, the destructive restore, index rebuilds) stay store-only because they describe one machine and are false on every other clone, and project acts travel; and `intent upgrade` backfills the project events a store already holds into files once, idempotently, as WP-01's separable last commit, so a project's history before this change travels too.

## Acceptance sketch

To be minted with `intent ac new` after ratification, one row per user-facing behaviour: a pull with no daemon is reflected by the next verb; a stale store is reported on a default `doctor` run; two clones that mint one id are repaired by one verb; an act on one clone is readable as an event on another after a pull, with its author; and every command on the team page has been driven.
