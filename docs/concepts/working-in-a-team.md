# Working in a team

**Each person has their own store and the repository carries the canon.** Everything on this page follows from that: what you commit, what a reviewer reads in a pull request, what happens to your store when you pull, and why two people creating records at the same time no longer collide.

The page walks one project from its first push to GitHub through the pull requests, merges and conflicts a team meets, in the order it meets them. Every command below was driven on the build this page describes: a bare repository stood in for the GitHub repository, Alice and Bob each had a clone of it, a third clone stood in for GitHub's merge buttons, and everything ran under an isolated `HOME`, with intentd running only in the section that says so. The team's story ran in the order this page shows it. The section on ordinals ran in a second project, made with Intent 3.4.x and upgraded as the upgrade note says, and the section on a committed store in a third. The ids are the ones that drive minted. Where a command prints Intent's version or a build's commit, this page shows `...`.

## The rules, in one list

- Commit the canon, the event files and each realised thread's `info.md` and its packages' `info.md` together, and never the store. The two generated lists, `intent/st/steel_threads.md` and `intent/todo.md`, and each thread's `acceptance.md`, are each clone's own render and are not committed.
- Let the git hooks run `intent sync --apply` after a pull, a checkout or a rebase. After a `git reset`, run it yourself before your next write.
- Read an id off the `created:` line that made it. Intent takes it from the act that created the record, so two people creating threads, packages or issues at the same time never get the same one.
- An ordinal both branches made before the project's upgrade still collides: `intent st remint` or `intent issues remint` moves yours before the merge, and `intent sync --apply --yes` in the middle of the merge does the same.
- Resolve a conflict in a canon JSON file by hand, `git add` it, then run `intent sync --apply --yes`. Never merge a generated view by hand.
- Write a thread's objective and context with `intent set`, not by editing `info.md`.
- Upgrade Intent together, and run `intent doctor` on the merge result in CI as a required check.
- A pull takes a teammate's whiteboard into your store through the same hook; if your own board moved too, the hook names it and `intent sync --to-store` is the step.

## What travels and what does not

| Travels in git                                                                                                                                                                       | Stays on your machine                                                                |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| `intent/.canon/st/`, `intent/.canon/records/` and `intent/.canon/issues/`: one file per record in an upgraded project, where 3.4.0's format keeps a thread's records in its one file | `intent/.cache/`: the store, `intent/.cache/intent.db`                               |
| `intent/.canon/events/YYYY/MM/DD/`: one JSON file per project act                                                                                                                    | Machine events, as [the store](the-store.md#what-lives-only-in-the-store) lists them |
| A realised thread's `info.md` and its packages' `info.md`; `intent/issues/`                                                                                                          | `intent/.backup/`: backups                                                           |
| `intent/.config/config.json`, `intent/.canon/project.json`                                                                                                                           | `intent/st/steel_threads.md`, `intent/todo.md`, each thread's `acceptance.md`        |
| `intent/whiteboard/<node>/`: each board's `board.json` and its views                                                                                                                 | The search index                                                                     |

**Your store is truth on your machine, and the canon is how it reaches anyone else's.** A clone's store starts empty and is built from the committed canon on the first verb, so a fresh clone needs no step of its own for the model.

## Put the project on GitHub

**The first person makes the project and pushes it.** `intent init` writes the ignore lines for what stays on your machine, so the first commit carries the canon and none of the store:

```
  $ intent init Team
  $ grep -n -E '^intent/' .gitignore
  3:intent/.cache/
  6:intent/.backup/
  9:intent/st/steel_threads.md
  12:intent/todo.md
  15:intent/st/*/acceptance.md
  $ intent bootstrap
  $ intent claude upgrade --apply --skip-settings
  $ git add -A && git commit -m "intent init"
  $ git push -u origin main
```

## Once per machine, once per clone

Everyone else clones the repository and runs two commands:

```
  $ intent bootstrap
  $ intent claude upgrade --apply --skip-settings
```

`bootstrap` sets up your machine, once per machine rather than per project. Run it first, because the git hooks the next command installs find Intent through what `bootstrap` records.

`claude upgrade --apply` wires those hooks into `.git/hooks`, and `.git/hooks` is not cloned, so every clone runs it once. On Bob's fresh clone it says:

```
  $ intent claude upgrade --apply --skip-settings
  unchanged: CLAUDE.md
  unchanged: AGENTS.md
  unchanged: usage-rules.md
  unchanged: .intent_critic.yml
  skipped: .claude/settings.json (--skip-settings)
  skipped: .mcp.json (--skip-settings)
  ok: 8 written, 4 already canonical, 0 preserved, 0 held, 2 skipped.
```

## What a reviewer reads in a pull request

**Review the canon, not the views.** A thread's change arrives as a diff of its canon files: in an upgraded project, its own file under `intent/.canon/st/` and one file per work package, criterion, test and attachment record under `intent/.canon/records/<thread>/`, each with a schema and diffing field by field, beside one event file per act. The views are generated from the same JSON and say the same thing at more length. Alice's branch, which creates a thread, starts it and adds a work package:

```
  $ intent st new "Onboarding guide"
  created: ST3060b2b2d824b45f
  $ intent st start ST3060b2b2d824b45f
  $ intent wp new ST3060b2b2d824b45f "Write the guide"
  created: ST3060b2b2d824b45f/8819193e
  $ git diff --stat main...alice/onboarding
   .../2026/11/20/01M7X9BTTMWWV0SFYZA1CZKDTT.json     | 14 ++++++++++
   .../2026/11/20/01M7X9BTVKB26T00ZXVPR1TJDN.json     | 16 +++++++++++
   .../2026/11/20/01M7X9BTWS1YASTM79Z048YHC6.json     | 15 +++++++++++
   intent/.canon/st/ST3060b2b2d824b45f.json           | 26 ++++++++++++++++++
   intent/st/ST3060b2b2d824b45f/WP/8819193e/info.md   | 20 ++++++++++++++
   intent/st/ST3060b2b2d824b45f/info.md               | 31 ++++++++++++++++++++++
   6 files changed, 122 insertions(+)
```

The canon file and the directory are named by the thread's whole id. Tables and messages print its short form, `ST3060b2b`, and any unique prefix of at least four hex, or five if they are all digits, names it in every command.

Things worth checking in the JSON: the status transition is the one the PR describes, a criterion's state moved because a test moved, and nothing changed that the PR does not mention.

Switching back to `main` leaves the tree clean, and the branch merges:

```
  $ git switch main
  intent (post-checkout): took 1 change(s) from the files into the store: ST3060b2b (removed)
  $ git merge --no-ff alice/onboarding
```

## Merging on GitHub: a merge commit, a squash or a rebase

**The canon is files, so every one of GitHub's merge buttons carries it.** The merge above made a merge commit. Alice's second pull request adds a thread in one commit and a work package in the next, and is squash-merged. A squash folds the commits and keeps every file they wrote, the event files included:

```
  $ git merge --squash origin/alice/checklist
  ...
   create mode 100644 intent/.canon/events/2026/11/20/01M7X9BWD0N0XZKAT2E673TMEY.json
   create mode 100644 intent/.canon/events/2026/11/20/01M7X9BWPXM0073JXAQQHS5G6S.json
   create mode 100644 intent/.canon/st/ST1e403319943ee5c7.json
```

On Alice's own clone, switching to `main` takes the thread out of her store, because `main` does not carry it yet, and the pull puts it back:

```
  $ git switch main
  intent (post-checkout): took 1 change(s) from the files into the store: ST1e40331 (removed)
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: ST1e40331
  $ git branch -D alice/checklist
```

A rebase merge replays the branch's commits onto `main` and reads the same way: Alice's third pull request, rebase-merged, came back through the same two hook lines for its thread, `STb580080`.

## After a pull

**`intent sync --apply` is the command after a pull, and the hooks run it for you.** It takes a pulled teammate's whiteboard as well, [below](#the-whiteboard-across-clones). Bob's first pull brings all three merges. It prints nothing, because his store is still empty and the next verb builds it from the canon the pull brought:

```
  $ git pull
  $ intent st show ST1e403319943ee5c7
  ST1e40331: Release checklist
  status: Triage
  created: 2026-11-20
  objective: _(not yet written)_
```

`intent st list --status all` on Bob's clone then lists all three threads.

From then on, the hook prints one line whenever it changed the store, here for the security review Alice added:

```
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: ST9d5e063; and 1 event file(s)
```

`intent sync` with no flag prints the plan for this clone and writes nothing. `--apply` applies it and runs `doctor` last, whose verdict is the exit code. Each step says what kind it is:

- **quiet** steps run without asking: taking the files into the store, taking event files, regenerating views and bringing the search index up to date.
- **reversible** steps, such as keeping both sides' work packages, moving an ordinal both sides made, or staging a regenerated view, are what `--yes` answers.
- a **non-reversible** step, taking one side of a conflicting canon file, is always left for a person, and no flag answers for it.

Without a terminal and without `--yes`, `--apply` runs the quiet steps and names what it left.

### Pulling with `--rebase`

A rebase checks out what you pulled and then replays your own commits over it, so two hooks run, each saying what it took. Here Bob had a local commit setting the onboarding guide's context, and Alice had pushed an issue:

```
  $ git pull --rebase
  intent (post-checkout): took 2 change(s) from the files into the store: ST3060b2b, issue I57cb7bd; and 1 event file(s)
  intent (post-rewrite): took 1 change(s) from the files into the store: ST3060b2b
```

The first line takes the pulled tree, where the guide has no context yet; the second takes Bob's replayed commit back. Nothing is left for him to do.

### With intentd running

A running intentd changes nothing about a pull. The hook takes the change as it would without one, and a `sync` afterwards has nothing left to do:

```
  $ intent daemon start
  ok: intentd is answering at ...
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: ST9d5e063; and 1 event file(s)
  $ intent sync
  plan: nothing to do -- this clone's store already holds what the files say (plan ...)
```

### The hooks do not see every way a tree moves

A `git reset --hard` moves the canon under a store that still holds the old model, and no hook runs. `intent doctor` says so on a default run. The line is shown and not counted, so it does not fail the gate:

```
  $ git reset --hard origin/main
  $ intent doctor
  advisory: store-stale -- 2 findings, not counted in the verdict
    remedy: run `intent sync --apply`: ...
```

**Run `intent sync --apply` before the first write, not after it.** A write to a stale store projects the old model over the new canon. Here Bob's write undid Alice's pulled retitle, and said so:

```
  $ intent set ST1e403319943ee5c7 context "Requested by support; retitled upstream"
  warning: overwrote bytes that were not the store's render -- an edit to a generated file is gone:
    intent/.canon/st/ST1e403319943ee5c7.json
    remedy: a generated view has one writer, so recover the text with `git diff` / `git checkout` if you need it, and make the change through the verb that owns the field
  $ git diff --stat
   intent/.canon/st/ST1e403319943ee5c7.json | 4 ++--
```

After `git checkout -- .` to throw that away, the same write after a sync touches only the field it names:

```
  $ intent sync --apply
  ok: took 1 change(s) from the files into the store: ST1e40331; and 1 event file(s)
  $ intent set ST1e403319943ee5c7 context "Requested by support; retitled upstream"
  $ git diff --stat
   intent/.canon/st/ST1e403319943ee5c7.json | 2 +-
```

## Five pull requests open at once

**Intent takes each new id from the act that created the record, so ids made on different branches never meet.** Alice and Bob opened five pull requests, merge requests on some forges, from one commit of `main`, each creating a thread, a work package and an issue:

```
  $ git switch -c mr-one 463eaca06d8ddd2f1f9e24602a8fdc3a9ad4e27a
  $ intent st new "Thread from mr-one"
  created: STa1db81b2549e0378
  $ intent wp new STa1db81b2549e0378 "Package from mr-one"
  created: STa1db81b2549e0378/2d979fb1
  $ intent issues add "Issue from mr-one"
  created: I3d9fbd87d844b6f6
  ...
  created: STad16411974dbfd49
  created: STa8a9047163f1b939
  created: ST77a728a44a1c0b63
  created: ST7831dca650a06511
```

The forge merged them one after another, and none of them conflicted:

```
  $ git merge --no-ff origin/mr-one
  Merge made by the 'ort' strategy.
  ...
  $ git merge --no-ff origin/mr-five
  Merge made by the 'ort' strategy.
```

Alice's pull takes all five threads, with their packages, and all five issues, and nothing waits for a person:

```
  $ git pull
  intent (post-merge): took 10 change(s) from the files into the store: ST77a728a, ST7831dca, STa1db81b, STa8a9047, STad16411, issue I010191d, issue I18c0e2d, issue I24d5bbf, issue I3d9fbd8, issue I816b7b2; and 6 event file(s)
  $ intent issues list
  ID            | Status     | Sev        | Created           | Title
  --------------|------------|------------|-------------------|---------------------------------------
  I57cb7bd      | OPEN       | medium     | 2026-11-20        | Login fails on Safari
  I3d9fbd8      | OPEN       | medium     | 2026-11-20        | Issue from mr-one
  I18c0e2d      | OPEN       | medium     | 2026-11-20        | Issue from mr-two
  I816b7b2      | OPEN       | medium     | 2026-11-20        | Issue from mr-three
  I24d5bbf      | OPEN       | medium     | 2026-11-20        | Issue from mr-four
  I010191d      | OPEN       | medium     | 2026-11-20        | Issue from mr-five
  $ intent doctor
  doctor: 0 finding(s) across 9 thread(s), 6 issue(s), 33 view(s), 62 file(s) -- ...
```

`intent issues list` and `intent wp list` show records in the order they were made, so the issue from the oldest request comes first, after any ordinals the project has from before it took these ids, in number order.

### A work package each, in one thread: both are kept

A thread's work packages live in its canon file, so two people who each add one to the same thread before pulling meet a conflict in that file, and in the thread's realised `info.md`. `intent sync` reads the merge's three versions of the canon. When adding packages is all either side did, it keeps every one:

```
  $ git pull --no-rebase
  CONFLICT (content): Merge conflict in intent/.canon/st/ST3060b2b2d824b45f.json
  CONFLICT (content): Merge conflict in intent/st/ST3060b2b2d824b45f/info.md
  $ intent sync --apply --yes
    1. keep packages (reversible): both sides added work packages to ST3060b2b2d824b45f: every one is kept, staged
  ...
  ok: the work packages both sides added to ST3060b2b2d824b45f: every one kept, staged
  ok: took 1 event file(s) from the files into the store
  ok: regenerated and staged 1 view(s): intent/st/ST3060b2b2d824b45f/info.md
  $ git commit --no-edit
  $ intent wp list ST3060b2b2d824b45f
  WP        | Title                                         | Scope       | Status
  ----------|-----------------------------------------------|-------------|---------------------------
  8819      | Write the guide                               | S           | Not Started
  6390      | Record a walkthrough                          | S           | Not Started
  3079      | Add a glossary                                | S           | Not Started
  6486      | Add screenshots                               | S           | Not Started
```

**When either side changed anything else on the thread as well**, such as its title, a criterion or an attachment, the plan offers only taking a side, as [two people editing one thread](#when-two-people-edit-one-thread) do. Taking a side drops the other person's work package. The route that loses nothing is to abandon the merge, drop your commit, take the pull and add yours again with `intent wp new`.

### When two records share a short form

A short form is a prefix of the whole id: seven hex for a thread or an issue, four for a work package within its thread. A new record never takes a short form that a record you can already see has, because Intent draws again when it would. Two branches can still each make one that shares a short form, and a package's four hex make that the likely case: it took the drive 108 packages on Bob's branch, against 300 on Alice's, before one matched.

```
  $ git merge --no-edit origin/alice/many
  CONFLICT (content): Merge conflict in intent/.canon/st/ST1ef5fac8e26c9455.json
  $ intent sync --apply --yes
  ok: the work packages both sides added to ST1ef5fac8e26c9455: every one kept, staged
  ...
  $ intent doctor --verbose
  ...
  advisory: short-form-shared -- 1 finding, not counted in the verdict
    remedy: nothing to do: each id is whole in canon, and a door handed this short form names both records and asks for more of the id
    intent/.canon/st/ST1ef5fac8e26c9455.json -- 2 work packages share the short form ff4d: ff4ddbac, ff4d93b5, in ST1ef5fac
```

**Nothing is lost and nothing needs moving.** Each record keeps its whole id. A command handed the shared short form names both and asks for more of the id, and one more hex is enough:

```
  $ intent wp show ST1ef5fac8e26c9455/ff4d
  error: `ST1ef5fac8e26c9455/ff4d` names 2 records in this project: ST1ef5fac/ff4d (wp, "Bob's package 107", created 2026-11-20); ST1ef5fac/ff4d (wp, "Alice's package 173", created 2026-11-20)
    remedy: name the one you meant by more of its id, or by all of it: `ST1ef5fac8e26c9455/ff4d93b5`, `ST1ef5fac8e26c9455/ff4ddbac`
  $ intent id ST1ef5fac8e26c9455/ff4d
  ambiguous token=ST1ef5fac8e26c9455/ff4d kind=wp,wp candidates=ST1ef5fac8e26c9455/ff4d93b5,ST1ef5fac8e26c9455/ff4ddbac
  $ intent wp show ST1ef5fac8e26c9455/ff4d9
  ST1ef5fac/WP-ff4d: Bob's package 107
  status: Not Started
  scope: S
  criteria: none scoped to ST1ef5fac/ff4d
```

`intent id` answers a script the same question without failing on it: `ambiguous` and both whole ids. A thread's or an issue's seven hex become likely to be shared only in a project of tens of thousands of records, and the same rules hold.

## Ordinals made before the upgrade

**A project made before Intent took git-style ids keeps its ordinals, and two branches cut before its upgrade can each hold the same one.** Here Alice and Bob each made `ST0002`, `ST0003` and issue `0001` with the older Intent, Alice on `main` and Bob on an open branch, and both upgraded as [the upgrade note](../install.md#upgrading) says. Git refuses the merge with an add/add conflict on each one's canon file:

```
  $ git merge --no-ff origin/main
  CONFLICT (add/add): Merge conflict in intent/.canon/issues/0001.json
  CONFLICT (add/add): Merge conflict in intent/.canon/st/ST0002.json
  CONFLICT (add/add): Merge conflict in intent/.canon/st/ST0003.json
  CONFLICT (add/add): Merge conflict in intent/issues/0001.md
```

### Remint before you merge

**The person on the losing side moves theirs to a git-style id and merges again:**

```
  $ git merge --abort
  $ intent st remint ST0002
  ok: ST0002 reminted to STf84ad01962bb14d4
  moved: intent/.canon/st/ST0002.json removed
  $ intent issues remint 0001
  ok: issue 0001 reminted to I3bd079d2ed4dbaf0
  moved: intent/.canon/issues/0001.json removed
  moved: intent/issues/0001.md removed
```

`remint` moves the record to the git-style id its own creation event makes, with everything that names it structurally. **It does not rewrite prose.** It lists each line of prose that names the old id and leaves it, because a sentence that names `ST0002` might mean either thread.

### Or repair it mid-merge

**If you merged first, `intent sync` sees the collision and plans the move.** `ST0003` was left for this:

```
  $ git merge --no-ff origin/main
  CONFLICT (add/add): Merge conflict in intent/.canon/st/ST0003.json
  $ intent sync
  plan: 5 step(s) for this clone (plan cdd41dd4...), and nothing has been written -- `intent sync --apply` applies them
    1. renumber (reversible): both sides minted steel thread ST0003: this clone's moves to STd0107cfa50e897e3 and the pulled one keeps ST0003, both staged
    2. ingest (quiet): take the merged canon into the store; what it takes can be read once the conflicts are resolved
    3. views (quiet): regenerate the views the merged canon changes; which ones can be read once the conflicts are resolved
    4. events (quiet): 4 event file(s) to take
    then: doctor (quiet): run doctor last; its verdict is the exit code
```

Without a terminal, `--apply` leaves the move and says how to run it:

```
  $ intent sync --apply
  ...
  left: 3 step(s): renumber (reversible), ingest (waits on the conflicts), views (waits on the conflicts) -- run `intent sync --apply` on a terminal, or with `--yes`
```

`--yes` answers it:

```
  $ intent sync --apply --yes
  ...
  ok: renumbered this clone's ST0003 to STd0107cfa50e897e3, kept the pulled ST0003, and staged 2 path(s)
  ok: took 4 change(s) from the files into the store: ST0002, ST0003, STfd23168, issue 0001
  $ git commit --no-edit
```

Your thread moves to the git-style id its creation event makes, the pulled thread keeps the ordinal, and both are staged. `STfd23168` is the thread Alice made after the upgrade, which the merge brought.

## When two people edit one thread

Two people who change different fields of one thread conflict on neighbouring lines of its canon JSON:

```
  $ git pull --no-rebase
  CONFLICT (content): Merge conflict in intent/.canon/st/ST1e403319943ee5c7.json
  $ intent sync
    1. take a side (non-reversible): both sides changed intent/.canon/st/ST1e403319943ee5c7.json: a person chooses ours or theirs, and it is staged -- or resolve it by hand and `git add` it
```

**Resolve the JSON by hand, keeping both edits, and stage it before you sync.** Until it is staged, git still lists it as unmerged and the plan keeps the take-a-side step:

```
  $ $EDITOR intent/.canon/st/ST1e403319943ee5c7.json
  $ git add intent/.canon/st/ST1e403319943ee5c7.json
  $ intent sync --apply --yes
  ok: took 1 change(s) from the files into the store: ST1e40331; and 1 event file(s)
  $ grep -n -E '"objective"|"context"' intent/.canon/st/ST1e403319943ee5c7.json
  9:  "objective": "Bring a new engineer to a first merged PR in a day",
  10:  "context": "Requested by support after three onboarding escalations",
```

`intent st show` prints only a thread's objective, so read the merged fields in the JSON.

**Never merge a generated view by hand.** The two lists and each thread's `acceptance.md` travel in no commit, so they never conflict. A realised thread's `info.md` and its packages' `info.md` are committed, and two branches that both change a thread conflict in them too: `intent sync --apply --yes` regenerates and stages each one from the merged canon, as the packages above show.

## Commit the views with what they were rendered from

**A view committed without the change it renders from fails the gate on every other clone.** The commit gate `claude upgrade --apply` installs runs `doctor`, and refuses a commit whose views disagree with the store. Here Bob edited a thread's Objective in `info.md` by hand and committed without a sync:

```
  intent/st/STb580080523fa4fa9/info.md -- generated view differs from the model (871 bytes on disk, 840 rendered, first difference at byte 128) in its Objective or Context, which a hand edit changes and the store carries back -- `intent sync --to-store` carries the edit into the store and keeps it; `intent sync --to-disk` would regenerate the cover from the store and discard it
  intent doctor: the estate disagrees with the store -- commit refused.
```

The Objective and the Context are the two sections of `info.md` the store takes back, so the finding names the verb that keeps the edit. Run it, and the commit passes:

```
  $ intent sync --to-store
  warning: replacing the store from the extract OVERWRITES:
    STb580080: intent/st/STb580080523fa4fa9/info.md carries an edit to its Objective or Context, which REPLACES the store's
  ok: store replaced from the canon extract, taking the 1 difference(s) listed above
  $ intent st show STb580080523fa4fa9
  STb580080: Triage rota
  status: WIP
  created: 2026-11-20
  objective:
    Edited by hand and not synced. _(not yet written)_
```

**Write a thread's objective and context with `intent set`, not by editing `info.md`.** The hooks' `intent sync --apply` after a pull, a checkout or a rebase takes a hand edit to either section into the store, but any other Intent write that renders the cover first writes the store's version over it, with a warning. An edit anywhere else in `info.md` is not carried at all.

The same holds for anything generated from one store state: commit it together. On the whiteboard, each verb prints what it wrote on its `wrote:` lines -- the node's own `board.json` and `wip.md`, the inbox views it wrote in its peers' directories, and its event -- and those files go in one commit. A commit that carries a node's board without the inbox views the same write rendered leaves views on `main` that no committed board renders, and a fresh clone's `intent doctor` counts them.

## History travels

Every project act is its own file under `intent/.canon/events/`, carrying who did it, so a teammate's acts are in your event log after a pull:

```
  $ intent events --subject ST1e403319943ee5c7
  2026-11-20T20:11:33.153Z  01M7X9BWD0N0XZKAT2E673TMEY  st.new  ST1e403319943ee5c7  by Alice <alice@example.com>
  2026-11-20T20:11:41.124Z  01M7X9C4638KWV6SWR288EJ3KR  thread.set  ST1e403319943ee5c7  by Alice <alice@example.com>
  2026-11-20T20:11:41.699Z  01M7X9C4R3DS6F8BH4X5YEDWGQ  thread.set  ST1e403319943ee5c7  by Bob <bob@example.com>
  2026-11-20T20:11:42.102Z  01M7X9C54PJR9C081P1ZEX733Y  thread.set  ST1e403319943ee5c7  by Bob <bob@example.com>
  2026-11-20T20:12:15.600Z  01M7X9D5VGN1H2VHSSPS5WYYNY  thread.set  ST1e403319943ee5c7  by Alice <alice@example.com>
  2026-11-20T20:12:16.004Z  01M7X9D684E47G65PZGXW53K08  thread.set  ST1e403319943ee5c7  by Bob <bob@example.com>
  events: showing 6 of 6 matched, 349 in this store.
```

Both clones print those same six lines; the store's total under them differs, because each store also holds its own machine events. The author is git's identity. Machine events, [listed on the store's page](the-store.md#what-lives-only-in-the-store), stay in the store, because they describe one machine and would be false on every other. The thread's id is made from the id of its `st.new` event, which is why two clones never mint the same one.

### A project that committed its store

**A project made with Intent 3.0 may have committed its store**, because Intent 3.0's `init` wrote no `.gitignore` and the first `git add -A` took `intent/.cache/intent.db` with everything else. `intent upgrade` writes the ignore lines, and backfills an event file for any history only the store held. An ignore line does not untrack a file git already tracks, so the upgrade names the command that does, and `intent doctor` reports a tracked store until it runs. Here the store was committed on purpose to show it:

```
  $ git ls-files intent/.cache
  intent/.cache/intent.db
  $ intent upgrade
  ...
  untrack: git tracks intent/.cache/intent.db, and the ignore rule this upgrade wrote does not untrack it -- run `git rm --cached intent/.cache/intent.db` and commit that with this upgrade, or every commit carries this machine's store
  ok: this project is now Intent ... -- commit the canon and the generated views
  $ intent doctor
  advisory: store-tracked -- 1 finding, not counted in the verdict
  ...
  $ git rm -q --cached intent/.cache/intent.db
  $ git status --short --untracked-files=all
  D  intent/.cache/intent.db
   M intent/.config/config.json
```

Commit all of it. Left tracked, the store rides in every teammate's next commit, and their pulls collide on it. `git ls-files intent/.cache` then lists nothing.

## Everyone runs a compatible Intent

`intent/.config/config.json` records the `intent_version` that wrote the project. Each canon file names its schema, and a binary refuses one it does not know. Here one file claims a newer schema than this Intent reads:

```
  $ grep -n '"schema"' intent/.canon/st/ST1e403319943ee5c7.json
  2:  "schema": "intent/thread@9.0",
  $ intent sync --apply
  error: could not read the committed canon
    caused by: residue: intent/.canon/st/ST1e403319943ee5c7.json -- schema-invalid -- schema is "intent/thread@9.0"; this binary reads "intent/thread@4.0" or "intent/thread@3.0"
    remedy: correct the field named above; `intent schema` prints the shape the file must match
  error: refused 1 finding(s) -- schema-invalid: 1
  $ intent doctor
  residue: schema-invalid -- 1 finding
    intent/.canon/st/ST1e403319943ee5c7.json -- schema is "intent/thread@9.0"; this binary reads "intent/thread@4.0" or "intent/thread@3.0"
  doctor: 1 finding(s) -- the estate could not be read, so no thread, issue, view or file was checked -- ...
```

A read verb such as `intent st show ST1e403319943ee5c7` still answers, from what the store held before: the store is truth on your machine, and it is a sync that reads the canon. Nothing more arrives until the clone runs an Intent that reads the newer schema. An Intent from before git-style ids refuses an upgraded project the same way, naming each canon file it cannot read. **Upgrade together.**

## The whiteboard across clones

A whiteboard's boards are committed files, and they travel like the rest of the canon. **Each `board.json` has one writer, its own node**: it carries the node's items, the messages it sent and its marks on the messages it has handled, so a message travels in its sender's commit and a clear in its recipient's. **The hook's `intent sync --apply` takes a pulled board into your store** when the file is the only side that moved: its bytes changed since your store last wrote or read it, and your store's own copy has not changed since. It names each board it takes on its `took` line.

A claim names a thread as its id, a work package as its address, or an issue as `ISSUE:` and its id, each copied whole from the `created:` line that made it:

```
  $ intent wb claim ST3060b2b2d824b45f --node al
  ok: al claims ST3060b2b2d824b45f
```

Bob's ask names what it wrote, and he commits exactly that:

```
  $ intent wb ask al "Can I take the release checklist?" --node bo
  ok: bo -> al
  wrote: intent/.canon/events/2026/11/20/01M7X9DBB1W3F82JS2SAZR86E0.json
  wrote: intent/whiteboard/al/inbox.bo.md
  wrote: intent/whiteboard/bo/board.json
```

Alice pulls it, and the hook's `took` line names the board it carried:

```
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: board bo; and 2 event file(s)
  $ intent wb pickup --node al
  ...
  messages (1)
    bo -> al Can I take the release checklist?
```

**When both sides moved, the board is left and named.** Because each board has one writer, that takes one node acting in two clones; a reply crossing a message moves two different files. A board your store changed and has not yet written to its file, meeting a pulled file, has two newer sides, and neither overwrites the other. The hook names it on its `left:` line, every `intent wb` verb refuses until it is resolved, and `intent sync --to-store` is the step: it replaces the store's board from the file and lists each difference before it takes it.

**A checkout of an older commit is not taken.** It moves a board file as a pull does, but backwards: your store's board is newer than the file. The hook leaves it and names it on its `left:` line, and every `intent wb` verb refuses and names both ways out: `intent sync --to-store` takes the older board over the store's, and deleting the file and running `intent sync --to-disk` keeps the store's. Checking the newer commit out again puts back the file your store matches.

## Check the merge in CI

The commit gate runs on the author's machine and judges the author's tree. **The merge the forge makes is a tree nobody's gate judged**, so run `intent doctor` on it in CI. On `pull_request`, `actions/checkout` checks out the merge result. Intent's own repository builds Intent for this job, because it is Intent; this is that job, from `.github/workflows/pr-checks.yml` without its comments:

```yaml
doctor-on-the-merge-result:
  name: Intent doctor on the merge result
  runs-on: ubuntu-latest
  env:
    XDG_CONFIG_HOME: ""
    XDG_DATA_HOME: ""
    XDG_STATE_HOME: ""
    XDG_RUNTIME_DIR: ""

  steps:
    - uses: actions/checkout@v7
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
      with:
        workspaces: native/rust
    - name: Build intent
      run: cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli
    - name: intent doctor
      run: native/rust/target/release/intent doctor
```

A project that uses Intent installs the release instead of building it. Intent is published through its Homebrew tap for macOS on Apple silicon and for [Linux](../install.md#linux), and the job below runs on an Apple silicon macOS runner. This job was not driven for this page, because it runs on GitHub, which the drive cannot reach:

```yaml
intent-doctor:
  name: Intent doctor on the merge result
  runs-on: macos-15
  steps:
    - uses: actions/checkout@v7
    - name: Install Intent
      run: brew install matthewsinclair/intent/intent
    - name: intent doctor
      run: intent doctor
```

A finding fails the job. The runner's store is cold and is built from the committed canon, the same way a fresh clone builds one. The two lists and each thread's `acceptance.md` are not in the clone, so doctor names them as views this clone has not rendered, and does not count them:

```
  $ git clone origin.git ci && cd ci
  $ intent doctor
  advisory: aggregate-view-stale -- 4 findings, not counted in the verdict
  ...
  doctor: 0 finding(s) across 10 thread(s), 6 issue(s), 42 view(s), 83 file(s) -- ...
```

### Make it a required check

A job that fails without blocking the merge protects nothing. In the repository's settings on GitHub, add a rule for the default branch that requires status checks to pass before merging, either a branch ruleset or a classic branch protection rule, and choose the check by its job name, **Intent doctor on the merge result**. GitHub offers a check by name once it has run in the repository, so open one pull request first. This step is in GitHub's settings rather than in a command, so the drive could not reach it either.

---

Back to [Concepts](index.md), or on to the [command reference](../reference/).
