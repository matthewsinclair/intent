# Working in a team

**Each person has their own store and the repository carries the canon.** Everything on this page follows from that: what you commit, what a reviewer reads in a pull request, what happens to your store when you pull, and what to do when two people minted the same id.

The page walks one project from its first push to GitHub through the pull requests, merges and collisions a team meets, in the order it meets them. Every command below was driven in the order this page shows it: a bare repository stood in for the GitHub repository, Alice and Bob each had a clone of it, a third clone stood in for GitHub's merge buttons, and everything ran under an isolated `HOME`, with intentd running only in the section that says so. The script and its log are attached to the steel thread that first built this page, ST0078; the thread is closed, so `intent st hydrate ST0078` writes them to `intent/st/ST0078/working-in-a-team-drive.sh` and `intent/st/ST0078/working-in-a-team-drive.log`. Where a command prints Intent's version or a build's commit, this page shows `...`.

## The rules, in one list

- Commit the canon, the event files and the generated views together, and never the store.
- Let the git hooks run `intent sync --apply` after a pull, a checkout or a rebase. After a `git reset`, run it yourself before your next write.
- When two people mint the same thread or issue id, `intent sync --apply --yes` in the middle of the merge renumbers yours. A work package minted twice is redone after the pull.
- Resolve a conflict in a canon JSON file by hand, `git add` it, then run `intent sync --apply --yes`. Never merge a generated view by hand.
- Write a thread's objective and context with `intent set`, not by editing `info.md`.
- Upgrade Intent together, and run `intent doctor` on the merge result in CI as a required check.
- After pulling a teammate's whiteboard, run `intent sync --to-store` before any `intent wb` verb.

## What travels and what does not

| Travels in git                                                             | Stays on your machine                                                                |
| -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `intent/.canon/st/` and `intent/.canon/issues/` — one JSON file per record | `intent/.cache/` — the store, `intent/.cache/intent.db`                              |
| `intent/.canon/events/YYYY/MM/DD/` — one JSON file per project act         | Machine events, as [the store](the-store.md#what-lives-only-in-the-store) lists them |
| The generated views: `intent/st/`, `intent/issues/`, `intent/todo.md`      | `intent/.backup/` — backups                                                          |
| `intent/.config/config.json`, `intent/.intentfiles`                        | The search index                                                                     |
| `intent/whiteboard/<node>/` — each board's `board.json` and its views      |                                                                                      |

**Your store is truth on your machine, and the canon is how it reaches anyone else's.** A clone's store starts empty and is built from the committed canon on the first verb, so a fresh clone needs no step of its own for the model.

## Put the project on GitHub

**The first person makes the project and pushes it.** `intent init` writes the ignore lines for what stays on your machine, so the first commit carries the canon and none of the store:

```
  $ intent init Team
  $ grep -n -E '^intent/(\.cache|\.backup)' .gitignore
  3:intent/.cache/
  6:intent/.backup/
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

`claude upgrade --apply` wires those hooks into `.git/hooks`, and `.git/hooks` is not cloned, so every clone runs it once. On Bob's fresh clone it writes:

```
  written: .git/hooks/pre-commit.intent
  written: .git/hooks/pre-commit
  written: .git/hooks/post-merge.intent
  written: .git/hooks/post-merge
  written: .git/hooks/post-checkout.intent
  written: .git/hooks/post-checkout
  written: .git/hooks/post-rewrite.intent
  written: .git/hooks/post-rewrite
```

## What a reviewer reads in a pull request

**Review the canon, not the views.** A thread's change arrives as a diff of `intent/.canon/st/<ID>.json`, which has a schema and diffs field by field, beside one event file per act. The views are generated from the same JSON and say the same thing at more length. Alice's branch, which creates a thread, starts it and adds a work package:

```
  $ git diff --stat main...alice/onboarding
   .../2026/09/24/01M39EN21CA9ZZDH4KEJ91EK6A.json     | 14 ++++++++++
   .../2026/09/24/01M39EN22AK57D60EE08XRRGWA.json     | 16 +++++++++++
   .../2026/09/24/01M39EN239HQCGFMPRFXWFJQWW.json     | 15 +++++++++++
   intent/.canon/st/ST0001.json                       | 23 ++++++++++++++++
   intent/.intentfiles                                |  1 +
   intent/st/ST0001/WP/01/info.md                     | 20 ++++++++++++++
   intent/st/ST0001/acceptance.md                     | 20 ++++++++++++++
   intent/st/ST0001/info.md                           | 31 ++++++++++++++++++++++
   intent/st/steel_threads.md                         |  5 ++--
   intent/todo.md                                     |  3 ++-
   10 files changed, 145 insertions(+), 3 deletions(-)
```

Things worth checking in the JSON: the status transition is the one the PR describes, a criterion's state moved because a test moved, and nothing changed that the PR does not mention.

Switching back to `main` leaves the tree clean, and the branch merges:

```
  $ git switch main
  intent (post-checkout): took 1 change(s) from the files into the store: ST0001 (removed)
  $ git merge --no-ff alice/onboarding
```

## Merging on GitHub: a merge commit, a squash or a rebase

**The canon is files, so every one of GitHub's merge buttons carries it.** The merge above made a merge commit. Alice's second pull request adds a thread in one commit and a work package in the next, and is squash-merged. A squash folds the commits and keeps every file they wrote, the event files included:

```
  $ git merge --squash origin/alice/checklist
  ...
   create mode 100644 intent/.canon/events/2026/09/24/01M39EN3KE8XVCGWRX3R2ZVQ2G.json
   create mode 100644 intent/.canon/events/2026/09/24/01M39EN3YY0B60S7TFJ5VA02YW.json
   create mode 100644 intent/.canon/st/ST0002.json
```

On Alice's own clone, switching to `main` takes the thread out of her store, because `main` does not carry it yet, and the pull puts it back:

```
  $ git switch main
  intent (post-checkout): took 1 change(s) from the files into the store: ST0002 (removed)
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: ST0002
  $ git branch -D alice/checklist
```

A rebase merge replays the branch's commits onto `main` and reads the same way: Alice's third pull request, rebase-merged, came back through the same two hook lines for its thread, ST0003.

## After a pull

**`intent sync --apply` is the command after a pull, and the hooks run it for you.** A pull that brings a teammate's whiteboard needs one more step, [below](#the-whiteboard-across-clones). Bob's first pull brings all three merges. It prints nothing, because his store is still empty and the next verb builds it from the canon the pull brought:

```
  $ git pull
  $ intent st list --status all
  bob         | Title                           | Status      | Created            | Completed
  ------------|---------------------------------|-------------|--------------------|------------------
  ST0003      | Triage rota                     | Triage      | 2026-09-24         |
  ST0002      | Release checklist               | Triage      | 2026-09-24         |
  ST0001      | Onboarding guide                | WIP         | 2026-09-24         |
  $ intent st show ST0002
  ST0002: Release checklist
  status: Triage
  created: 2026-09-24
  objective: _(not yet written)_
```

From then on, the hook prints one line whenever it changed the store:

```
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: ST0002; and 2 event file(s)
```

`intent sync` with no flag prints the plan for this clone and writes nothing. `--apply` applies it and runs `doctor` last, whose verdict is the exit code. Each step says what kind it is:

- **quiet** steps run without asking: taking the files into the store, taking event files, regenerating views and bringing the search index up to date.
- **reversible** steps, a renumber or staging a regenerated view, are what `--yes` answers.
- a **non-reversible** step, taking one side of a conflicting canon file, is always left for a person, and no flag answers for it.

Without a terminal and without `--yes`, `--apply` runs the quiet steps and names what it left.

### Pulling with `--rebase`

A rebase checks out what you pulled and then replays your own commits over it, so two hooks run, each saying what it took. Here Bob had a local commit setting ST0001's context, and Alice had pushed an issue:

```
  $ git pull --rebase
  intent (post-checkout): took 2 change(s) from the files into the store: ST0001, issue 0001; and 1 event file(s)
  intent (post-rewrite): took 1 change(s) from the files into the store: ST0001
```

The first line takes the pulled tree, where ST0001 has no context yet; the second takes Bob's replayed commit back. Nothing is left for him to do.

### With intentd running

A running intentd changes nothing about a pull. The hook takes the change as it would without one, and a `sync` afterwards has nothing left to do:

```
  $ intent daemon start
  ok: intentd is answering at ...
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: ST0004; and 1 event file(s)
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
  $ intent set ST0002 context "Requested by support; retitled upstream"
  warning: overwrote bytes that were not the store's render -- an edit to a generated file is gone:
    intent/.canon/st/ST0002.json
    intent/st/steel_threads.md
    intent/todo.md
    remedy: a generated view has one writer, so recover the text with `git diff` / `git checkout` if you need it, and make the change through the verb that owns the field
  $ git diff --stat
   intent/.canon/st/ST0002.json |  4 ++--
   intent/st/steel_threads.md   | 12 ++++++------
   intent/todo.md               |  2 +-
```

After `git checkout -- .` to throw that away, the same write after a sync touches only the field it names:

```
  $ intent sync --apply
  ok: took 1 change(s) from the files into the store: ST0002; and 1 event file(s)
  $ intent set ST0002 context "Requested by support; retitled upstream"
  $ git diff --stat
   intent/.canon/st/ST0002.json | 2 +-
```

## When two people mint the same id

Ids are the highest number plus one over your own canon, so two people who each create a thread before pulling get the same `ST` number. Git refuses the merge with an add/add conflict on the canon file:

```
  $ git pull --no-rebase
  CONFLICT (add/add): Merge conflict in intent/.canon/st/ST0005.json
  CONFLICT (content): Merge conflict in intent/st/steel_threads.md
  CONFLICT (content): Merge conflict in intent/todo.md
```

### Renumber before you merge

**The person on the losing side moves theirs to a free id and merges again:**

```
  $ git merge --abort
  $ intent st renumber ST0005 ST0006
  ok: ST0005 renumbered to ST0006
  moved: intent/.canon/st/ST0005.json removed
```

`st renumber` moves the thread's canon and its realised directory, and rewrites what names the thread structurally: other threads' `related` lists, the `.intentfiles` row and whiteboard claims on its work packages. **It does not rewrite prose.** It lists each line of prose that names the old id and leaves it, because a sentence that names `ST0005` might mean either thread. The id it moves to must be free in your store and in your tree, and a taken id is refused with nothing moved.

The next pull conflicts only in generated views, and one command regenerates and stages them:

```
  $ git pull --no-rebase
  CONFLICT (content): Merge conflict in intent/st/steel_threads.md
  CONFLICT (content): Merge conflict in intent/todo.md
  $ intent sync --apply --yes
  ok: took 1 change(s) from the files into the store: ST0005; and 1 event file(s)
  ok: regenerated and staged 2 view(s): intent/st/steel_threads.md, intent/todo.md
  doctor: 0 finding(s) across 6 thread(s), ...
```

### Or repair it mid-merge

**If you pulled first, `intent sync` sees the collision and plans the renumber:**

```
  $ intent sync
  plan: 5 step(s) for this clone (plan 3372d2d2...), and nothing has been written -- `intent sync --apply` applies them
    1. renumber (reversible): both sides minted steel thread ST0007: this clone's moves to ST0008 and the pulled one keeps ST0007, both staged
    2. ingest (quiet): take the merged canon into the store; what it takes can be read once the conflicts are resolved
    3. resolve views (reversible): regenerate 2 unmerged view(s) from the merged canon and stage them: intent/st/steel_threads.md, intent/todo.md
    4. views (quiet): regenerate the views the merged canon changes; which ones can be read once the conflicts are resolved
    5. events (quiet): 1 event file(s) to take
    then: doctor (quiet): run doctor last; its verdict is the exit code
```

Without a terminal, `--apply` leaves the renumber and says how to run it:

```
  $ intent sync --apply
  ...
  left: 4 step(s): renumber (reversible), ingest (waits on the conflicts), resolve views (waits on the conflicts), views (waits on the conflicts) -- run `intent sync --apply` on a terminal, or with `--yes`
```

`--yes` answers it:

```
  $ intent sync --apply --yes
  ...
  ok: renumbered this clone's ST0007 to ST0008, kept the pulled ST0007, and staged 2 path(s)
  ok: took 1 change(s) from the files into the store: ST0007
  ok: regenerated and staged 2 view(s): intent/st/steel_threads.md, intent/todo.md
  doctor: 0 finding(s) across 8 thread(s), ...
```

The renumber moves your thread to the next id free in your store and your tree, keeps the pulled thread at the old id, and stages both. Commit the merge as usual.

### Issues collide the same way, and the same command repairs them

Issues are numbered the way threads are, so two people who each add one before pulling meet an add/add conflict on its canon file and its view, and `intent sync --apply --yes` renumbers yours:

```
  $ git pull --no-rebase
  CONFLICT (add/add): Merge conflict in intent/.canon/issues/0002.json
  CONFLICT (add/add): Merge conflict in intent/issues/0002.md
  $ intent sync --apply --yes
  ...
  ok: renumbered this clone's 0002 to 0003, kept the pulled 0002, and staged 5 path(s)
  ok: took 1 change(s) from the files into the store: issue 0002; and 1 event file(s)
  $ intent issues list
  ID      | Status      | Sev         | Title
  --------|-------------|-------------|---------------------------------------------------------------
  0001    | OPEN        | medium      | Login fails on Safari
  0002    | OPEN        | medium      | Search misses archived threads
  0003    | OPEN        | medium      | Install page has a typo
```

### A work package minted twice: redo yours after the pull

**There is no renumber for a work package**, so two people who each add one to the same thread before pulling are left with a choice between sides. The conflict lands in the thread's canon file and its views, and `intent sync` plans only the non-reversible step:

```
  $ git pull --no-rebase
  CONFLICT (content): Merge conflict in intent/.canon/st/ST0001.json
  CONFLICT (add/add): Merge conflict in intent/st/ST0001/WP/02/info.md
  CONFLICT (content): Merge conflict in intent/st/ST0001/info.md
  CONFLICT (content): Merge conflict in intent/todo.md
  $ intent sync
  ...
    1. take a side (non-reversible): both sides changed intent/.canon/st/ST0001.json: a person chooses ours or theirs, and it is staged -- or resolve it by hand and `git add` it
```

Taking one side drops the other person's work package. The route that loses nothing is to abandon the merge, drop your commit, take the pull and add yours again, which gives it the next free number:

```
  $ git merge --abort
  $ git reset --hard origin/main
  $ intent sync --apply
  ok: took 1 change(s) from the files into the store: ST0001; and 1 event file(s)
  $ intent wp new ST0001 "Add screenshots"
  created: ST0001/03
  $ intent wp list ST0001
  WP   | Title                                           | Scope       | Status
  -----|-------------------------------------------------|-------------|------------------------------
  01   | Write the guide                                 | S           | Not Started
  02   | Record a walkthrough                            | S           | Not Started
  03   | Add screenshots                                 | S           | Not Started
```

`git reset --hard` runs no hook, which is why `intent sync --apply` follows it.

## When two people edit one thread

Two people who change different fields of one thread conflict on neighbouring lines of its canon JSON:

```
  $ git pull --no-rebase
  CONFLICT (content): Merge conflict in intent/.canon/st/ST0002.json
  $ intent sync
    1. take a side (non-reversible): both sides changed intent/.canon/st/ST0002.json: a person chooses ours or theirs, and it is staged -- or resolve it by hand and `git add` it
```

**Resolve the JSON by hand, keeping both edits, and stage it before you sync.** Until it is staged, git still lists it as unmerged and the plan keeps the take-a-side step:

```
  $ $EDITOR intent/.canon/st/ST0002.json
  $ git add intent/.canon/st/ST0002.json
  $ intent sync --apply --yes
  ok: took 1 change(s) from the files into the store: ST0002; and 1 event file(s)
  $ grep -n -E '"objective"|"context"' intent/.canon/st/ST0002.json
  8:  "objective": "Bring a new engineer to a first merged PR in a day",
  9:  "context": "Requested by support after three onboarding escalations",
```

`intent st show` prints only a thread's objective, so read the merged fields in the JSON.

**Never merge a generated view by hand.** `steel_threads.md`, `todo.md` and each realised thread's views are committed, so two branches that both touch threads conflict in them. `intent sync --apply --yes` regenerates and stages each one from the merged canon, as the id collisions above show.

## Commit the views with what they were rendered from

**A view committed without the change it renders from fails the gate on every other clone.** The commit gate `claude upgrade --apply` installs runs `doctor`, and refuses a commit whose views disagree with the store. Here Bob edited a thread's Objective in `info.md` by hand and committed without a sync:

```
  intent/st/ST0006/info.md -- generated view differs from the model (847 bytes on disk, 836 rendered, first difference at byte 120) in its Objective or Context, which a hand edit changes and the store carries back -- `intent sync --to-store` carries the edit into the store and keeps it; `intent sync --to-disk` would regenerate the cover from the store and discard it
  intent doctor: the estate disagrees with the store -- commit refused.
```

The Objective and the Context are the two sections of `info.md` the store takes back, so the finding names the verb that keeps the edit. Run it, and the commit passes:

```
  $ intent sync --to-store
  note: no thread the store already holds differs on disk, so this restore overwrites nothing (a thread the extract has and the store does not is an ADD and is not examined here)
  ok: store rewritten from the canon extract; nothing the store already held was overwritten
  $ intent st show ST0006
  ST0006: Bob's next
  status: WIP
  created: 2026-09-24
  objective:
    Edited by hand and not synced.
```

**Write a thread's objective and context with `intent set`, not by editing `info.md`.** The hooks' `intent sync --apply` after a pull, a checkout or a rebase takes a hand edit to either section into the store, but any other Intent write that renders the cover first writes the store's version over it, with a warning. An edit anywhere else in `info.md` is not carried at all.

The same holds for anything generated from one store state: commit it together. A commit that carries a whiteboard board without that node's inbox views leaves views on `main` that no longer match their model, and a fresh clone's `intent doctor` counts them.

## History travels

Every project act is its own file under `intent/.canon/events/`, carrying who did it, so a teammate's acts are in your event log after a pull:

```
  $ intent events --subject ST0002
  2026-09-24T10:16:22.383Z  01M39EN3KE8XVCGWRX3R2ZVQ2G  st.new  ST0002  by Alice <alice@example.com>
  2026-09-24T10:16:30.899Z  01M39ENBXKXDM0DRRFJGN8204G  thread.set  ST0002  by Alice <alice@example.com>
  2026-09-24T10:16:31.695Z  01M39ENCPFZZQMXNXN2TEZ8EQ3  thread.set  ST0002  by Bob <bob@example.com>
  2026-09-24T10:16:32.067Z  01M39END23F8CTQ8MTE37200KJ  thread.set  ST0002  by Bob <bob@example.com>
  2026-09-24T10:16:43.520Z  01M39ENR7ZXG2G0JTN5CETZHGG  thread.set  ST0002  by Alice <alice@example.com>
  2026-09-24T10:16:43.980Z  01M39ENRPC58BMRN2MT7CBQT3S  thread.set  ST0002  by Bob <bob@example.com>
  events: showing 6 of 6 matched, 27 in this store.
```

Both clones print those same six lines; the store's total under them differs, because each store also holds its own machine events. The author is git's identity. Machine events, [listed on the store's page](the-store.md#what-lives-only-in-the-store), stay in the store, because they describe one machine and would be false on every other.

### A project made with Intent 3.0

**A project made with Intent 3.0 has its history only in the store that made it, and may have committed that store.** Intent 3.0's `init` wrote no `.gitignore`, so the first `git add -A` took `intent/.cache/intent.db` with everything else. Run `intent upgrade` once on that clone. It backfills an event file for the history the store holds and writes the ignore lines. An ignore line does not untrack a file git already tracks, so the upgrade names the command that does, and `intent doctor` reports a tracked store until it runs:

```
  $ git ls-files intent/.cache
  intent/.cache/intent.db
  $ intent upgrade
  migrated: 1 thread(s), 0 issue(s), 5 file(s) written
  not carried into the model: prose, shipped content and wip/restart are not modelled and are unchanged on disk
  already migrated: 1 thread(s) had committed canon and were re-emitted from it rather than converted -- their content is unchanged
  backfilled: 3 event file(s) under intent/.canon/events/ for the history this store held -- commit them and it travels with the project
  untrack: git tracks intent/.cache/intent.db, and the ignore rule this upgrade wrote does not untrack it -- run `git rm --cached intent/.cache/intent.db` and commit that with this upgrade, or every commit carries this machine's store
  ok: this project is now Intent ... -- commit the canon and the generated views
  $ git rm -q --cached intent/.cache/intent.db
  $ git status --short --untracked-files=all
  D  intent/.cache/intent.db
   M intent/.config/config.json
   M intent/st/ST0001/acceptance.md
   M intent/st/ST0001/info.md
   M intent/st/steel_threads.md
   M intent/todo.md
  ?? .gitignore
  ?? intent/.canon/events/2026/09/24/01M3AJ0G88RKV13NJFS3RQPVJ3.json
  ?? intent/.canon/events/2026/09/24/01M3AJ0G8G8F3KRWQJZAS3AB76.json
  ?? intent/.canon/events/2026/09/24/01M3AJ0G8QJN22K1WP6G4211F6.json
```

Commit all of it. Left tracked, the store rides in every teammate's next commit, and their pulls collide on it. A second `intent upgrade` backfills nothing and leaves the tree as the first run left it, and `git ls-files intent/.cache` then lists nothing.

## Everyone runs a compatible Intent

`intent/.config/config.json` records the `intent_version` that wrote the project. Each canon file names its schema, and a binary refuses one it does not know. Here one file claims a newer schema than this Intent reads:

```
  $ grep -n '"schema"' intent/.canon/st/ST0002.json
  2:  "schema": "intent/thread@9.0",
  $ intent sync --apply
  error: could not read the committed canon
    caused by: residue: intent/.canon/st/ST0002.json -- schema-invalid -- schema is "intent/thread@9.0"; this binary reads "intent/thread@3.0"
    remedy: correct the field named above; `intent schema` prints the shape the file must match
  error: refused 1 finding(s) -- schema-invalid: 1
  $ intent doctor
  residue: schema-invalid -- 1 finding
    intent/.canon/st/ST0002.json -- schema is "intent/thread@9.0"; this binary reads "intent/thread@3.0"
  doctor: 1 finding(s) -- the estate could not be read, so no thread, issue, view or file was checked -- ...
```

A read verb such as `intent st show ST0002` still answers, from what the store held before: the store is truth on your machine, and it is a sync that reads the canon. Nothing more arrives until the clone runs an Intent that reads the newer schema. **Upgrade together.**

## The whiteboard across clones

A whiteboard's boards are committed files, and they travel like the rest of the canon. **A pull does not take them into your store**, so after pulling a teammate's board every `intent wb` verb refuses and names the step:

```
  $ git pull
  intent (post-merge): took 2 event file(s) from the files into the store
  $ intent wb status
  error: board.json on disk holds a board this store does not, for al: a migrated board the store never took in, or a change that reached the file from outside the store, most often a pull
    remedy: nothing was written. `intent sync --to-store` carries each board.json into this store with everything it holds, OVER what the store holds for that node: ...
  $ intent sync --to-store
  warning: replacing the store from the extract OVERWRITES:
    board al: node on disk only, would be ADDED
    board al: [todo] 1 on disk only, would be ADDED
  ok: store replaced from the canon extract, taking the 2 difference(s) listed above
```

The same holds for everything a board carries, messages included, and for a board your store already holds. Bob's node sent Alice's a message, and Alice's pull brought it. Until her store took it in, her own board write refused rather than render her store's older board over the file:

```
  $ intent wb pickup --node al
  error: board.json on disk holds a board this store does not, for al, bo: a migrated board the store never took in, or a change that reached the file from outside the store, most often a pull
    remedy: nothing was written. `intent sync --to-store` carries each board.json into this store with everything it holds, OVER what the store holds for that node: ...
  $ intent sync --to-store
  warning: replacing the store from the extract OVERWRITES:
    board bo: node on disk only, would be ADDED
    board al: message from bo recorded 2026-09-24T20:34:17.598Z on disk only, would be ADDED
  ok: store replaced from the canon extract, taking the 2 difference(s) listed above
  $ intent wb pickup --node al
  ...
  messages (1)
    bo -> al Can I take the release checklist?
```

**Run `intent sync --to-store` after pulling a teammate's board, and read its warning.** It replaces the store from the files on disk and lists each difference it takes before it takes it. When the same pull changes a thread too, the hook's own pass leaves the pulled board as it is and names it on its `left:` line with the same verb. A checkout of an older commit puts an older board on disk the same way; carrying that one rolls your store's board back, and the refusal names the other way out, which keeps the store's. Carrying boards on a pull the way threads are carried is issue 0554.

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
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
      with:
        workspaces: native/rust
    - name: Build intent
      run: cargo build --release --manifest-path native/rust/Cargo.toml -p intent-cli
    - name: intent doctor
      run: native/rust/target/release/intent doctor
```

A project that uses Intent installs the release instead of building it. Intent is published for macOS on Apple silicon only, through its Homebrew tap, so the job runs on an Apple silicon macOS runner. This job was not driven for this page, because it runs on GitHub, which the drive cannot reach:

```yaml
intent-doctor:
  name: Intent doctor on the merge result
  runs-on: macos-15
  steps:
    - uses: actions/checkout@v4
    - name: Install Intent
      run: brew install matthewsinclair/intent/intent
    - name: intent doctor
      run: intent doctor
```

A finding fails the job. The runner's store is cold and is built from the committed canon, the same way a fresh clone builds one:

```
  $ git clone origin.git ci && cd ci
  $ intent doctor
  doctor: 0 finding(s) across 8 thread(s), 3 issue(s), 29 view(s), 68 file(s) -- ...
```

### Make it a required check

A job that fails without blocking the merge protects nothing. In the repository's settings on GitHub, add a rule for the default branch that requires status checks to pass before merging, either a branch ruleset or a classic branch protection rule, and choose the check by its job name, **Intent doctor on the merge result**. GitHub offers a check by name once it has run in the repository, so open one pull request first. This step is in GitHub's settings rather than in a command, so the drive could not reach it either.

---

Back to [Concepts](index.md), or on to the [command reference](../reference/).
