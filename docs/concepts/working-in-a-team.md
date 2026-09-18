# Working in a team

**Each person has their own store and the repository carries the canon.** Everything on this page follows from that: what you commit, what a reviewer reads, what happens to your store when you pull, and what to do when two people minted the same id.

Every command below was driven, in the order this page shows it, against two clones of one bare origin with no daemon running. The script and its log are in Intent's own repository, attached to the steel thread that built this page: `intent/st/ST0078/working-in-a-team-drive.sh` and `intent/st/ST0078/working-in-a-team-drive.log`.

## What travels and what does not

| Travels in git                                                             | Stays on your machine                                                                           |
| -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `intent/.canon/st/` and `intent/.canon/issues/` — one JSON file per record | `intent/.cache/` — the store, `intent/.cache/intent.db`                                         |
| `intent/.canon/events/YYYY/MM/DD/` — one JSON file per project act         | Machine events: heartbeats and pickups, organizes, syncs in either direction, text realisations |
| The generated views: `intent/st/`, `intent/issues/`, `intent/todo.md`      | `intent/.backup/` — backups                                                                     |
| `intent/.config/config.json`, `intent/.intentfiles`                        | The search index                                                                                |

**Your store is truth on your machine, and the canon is how it reaches anyone else's.** A clone's store starts empty and is built from the committed canon on the first verb, so a fresh clone needs no step of its own for the model.

`intent init` writes the ignore lines for what stays on your machine:

```
  $ grep -n -E '^intent/(\.cache|\.backup)' .gitignore
  3:intent/.cache/
  6:intent/.backup/
```

## Once per machine, once per clone

```
  $ intent bootstrap
  $ intent claude upgrade --apply --skip-settings
```

`bootstrap` sets up your machine, once per machine rather than per project. Run it first, because the git hooks the next command installs find Intent through what `bootstrap` records.

`claude upgrade --apply` wires those hooks into `.git/hooks`, and `.git/hooks` is not cloned, so every clone runs it once. On a fresh clone it writes:

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
   .../2026/09/18/01M2T9KXTC3AWTSH8C9MXWA4M8.json     | 14 ++++++++++
   .../2026/09/18/01M2T9KXTX27ZZ5RRAFQK1JT7E.json     | 16 +++++++++++
   .../2026/09/18/01M2T9KXVDZP8JDRWZWMJHB3Y1.json     | 15 +++++++++++
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

## After a pull

**`intent sync --apply` is the one command after a pull, and the hooks run it for you.** After a merge or a branch checkout, the hook prints one line when it changed the store:

```
  $ git pull
  intent (post-merge): took 1 change(s) from the files into the store: ST0002; and 1 event file(s)
  $ intent st show ST0002
  ST0002: Release checklist
  status: Triage
  created: 2026-09-18
  objective: _(not yet written)_
```

It prints nothing when nothing changed, which includes the first pull into a clone whose store is still empty: the next verb builds that store from the canon the pull brought.

`intent sync` with no flag prints the plan for this clone and writes nothing. `--apply` applies it and runs `doctor` last, whose verdict is the exit code. Each step says what kind it is:

- **quiet** steps run without asking: taking the files into the store, taking event files, regenerating views and bringing the search index up to date.
- **reversible** steps, a renumber or staging a regenerated view, are what `--yes` answers.
- a **non-reversible** step, taking one side of a conflicting canon file, is always left for a person, and no flag answers for it.

Without a terminal and without `--yes`, `--apply` runs the quiet steps and names what it left.

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
  $ git diff --stat
   intent/.canon/st/ST0002.json |  4 ++--
   intent/st/steel_threads.md   | 16 ++++++++--------
   intent/todo.md               |  4 ++--
```

After `git checkout -- .` to throw that away, the same write after a sync touches only the field it names:

```
  $ intent sync --apply
  ok: took 2 change(s) from the files into the store: ST0002, ST0004; and 1 event file(s)
  $ intent set ST0002 context "Requested by support; retitled upstream"
  $ git diff --stat
   intent/.canon/st/ST0002.json | 2 +-
```

## When two people mint the same id

Ids are the highest number plus one over your own canon, so two people who each create a thread before pulling get the same `ST` number. Git refuses the merge with an add/add conflict on the canon file:

```
  $ git pull --no-rebase
  CONFLICT (add/add): Merge conflict in intent/.canon/st/ST0003.json
  CONFLICT (content): Merge conflict in intent/st/steel_threads.md
  CONFLICT (content): Merge conflict in intent/todo.md
```

### Renumber before you merge

**The person on the losing side moves theirs to a free id and merges again:**

```
  $ git merge --abort
  $ intent st renumber ST0003 ST0004
  ok: ST0003 renumbered to ST0004
  moved: intent/.canon/st/ST0003.json removed
```

`st renumber` moves the thread's canon and its realised directory, and rewrites what names the thread structurally: other threads' `related` lists, the `.intentfiles` row and whiteboard claims on its work packages. **It does not rewrite prose.** It lists each line of prose that names the old id and leaves it, because a sentence that names `ST0003` might mean either thread. The id it moves to must be free in your store and in your tree, and a taken id is refused with nothing moved.

The next pull conflicts only in generated views, and one command regenerates and stages them:

```
  $ git pull --no-rebase
  CONFLICT (content): Merge conflict in intent/st/steel_threads.md
  CONFLICT (content): Merge conflict in intent/todo.md
  $ intent sync --apply --yes
  ok: took 1 change(s) from the files into the store: ST0003; and 1 event file(s)
  ok: regenerated and staged 2 view(s): intent/st/steel_threads.md, intent/todo.md
  doctor: 0 finding(s) across 4 thread(s), ...
```

### Or repair it mid-merge

**If you pulled first, `intent sync` sees the collision and plans the renumber:**

```
  $ intent sync
  plan: 5 step(s) for this clone (plan 889c9358...), and nothing has been written -- `intent sync --apply` applies them
    1. renumber (reversible): both sides minted steel thread ST0005: this clone's moves to ST0006 and the pulled one keeps ST0005, both staged
    2. ingest (quiet): take the merged canon into the store; what it takes can be read once the conflicts are resolved
    3. resolve views (reversible): regenerate 2 unmerged view(s) from the merged canon and stage them: intent/st/steel_threads.md, intent/todo.md
    4. views (quiet): regenerate the views the merged canon changes; which ones can be read once the conflicts are resolved
    5. events (quiet): 1 event file(s) to take
    then: doctor (quiet): run doctor last; its verdict is the exit code
```

Without a terminal, `--apply` leaves the renumber and says how to run it:

```
  $ intent sync --apply
  left: 4 step(s): renumber (reversible), ingest (waits on the conflicts), resolve views (waits on the conflicts), views (waits on the conflicts) -- run `intent sync --apply` on a terminal, or with `--yes`
```

`--yes` answers it:

```
  $ intent sync --apply --yes
  ok: renumbered this clone's ST0005 to ST0006, kept the pulled ST0005, and staged 2 path(s)
  ok: took 1 change(s) from the files into the store: ST0005
  ok: regenerated and staged 2 view(s): intent/st/steel_threads.md, intent/todo.md
  doctor: 0 finding(s) across 6 thread(s), ...
```

The renumber moves your thread to the next id free in your store and your tree, keeps the pulled thread at the old id, and stages both. Commit the merge as usual.

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

**A view committed without the change it renders from fails the gate on every other clone.** The commit gate `claude upgrade --apply` installs runs `doctor`, and refuses a commit whose views disagree with the store. Here a view was edited by hand and committed without a sync:

```
  intent/st/ST0004/info.md -- generated view differs from the model (867 bytes on disk, 836 rendered, first difference at byte 120): ...
  intent doctor: the estate disagrees with the store -- commit refused.
```

The same holds for anything generated from one store state: commit it together. A commit that carries a whiteboard board without that node's inbox views leaves views on `main` that no longer match their model, and a fresh clone's `intent doctor` counts them.

## History travels

Every project act is its own file under `intent/.canon/events/`, carrying who did it, so a teammate's acts are in your event log after a pull:

```
  $ intent events --subject ST0002
  2026-09-18T12:59:45.243Z  01M2T9KYPV1D7332EZYYKK99YX  st.new  ST0002  by Alice <alice@example.com>
  2026-09-18T12:59:48.673Z  01M2T9M22194AX8M39FH15SJJC  thread.set  ST0002  by Alice <alice@example.com>
  2026-09-18T12:59:48.931Z  01M2T9M2A3TVZK2SJYEE4SF50Y  thread.set  ST0002  by Bob <bob@example.com>
```

Both clones print those same three lines. The author is git's identity. Machine events, which are a heartbeat or a pickup's stamp, an organize, a sync in either direction and a text realisation, stay in the store, because they describe one machine and would be false on every other.

**A project older than this has its history only in the store that made it.** Run `intent upgrade` once on that clone and commit what it writes: the backfilled event files, a `.gitignore` without the retired `intent/events.jsonl` line, and `intent/.config/config.json` rewritten in key order. It says how many event files it backfilled, and a second run backfills none and leaves the tree as the first run left it:

```
  $ intent upgrade
  migrated: 1 thread(s), 0 issue(s), 5 file(s) written
  not carried into the model: prose, shipped content and wip/restart are not modelled and are unchanged on disk
  already migrated: 1 thread(s) had committed canon and were re-emitted from it rather than converted -- their content is unchanged
  backfilled: 3 event file(s) under intent/.canon/events/ for the history this store held -- commit them and it travels with the project
  ok: this project is now Intent v3.0.3 -- commit the canon and the generated views
  $ git status --short --untracked-files=all
   M .gitignore
   M intent/.config/config.json
  ?? intent/.canon/events/2026/09/18/01M2T9M3CRKAXHXTWRQN5QGWQP.json
  ?? intent/.canon/events/2026/09/18/01M2T9M3D6JZR0NP0ZXSHZYB2W.json
  ?? intent/.canon/events/2026/09/18/01M2T9M3DF9CFDG03AFSQ209DJ.json
  $ intent upgrade
  migrated: 1 thread(s), 0 issue(s), 5 file(s) written
  not carried into the model: prose, shipped content and wip/restart are not modelled and are unchanged on disk
  already migrated: 1 thread(s) had committed canon and were re-emitted from it rather than converted -- their content is unchanged
  ok: this project is now Intent v3.0.3 -- commit the canon and the generated views
  $ git status --short --untracked-files=all
   M .gitignore
   M intent/.config/config.json
  ?? intent/.canon/events/2026/09/18/01M2T9M3CRKAXHXTWRQN5QGWQP.json
  ?? intent/.canon/events/2026/09/18/01M2T9M3D6JZR0NP0ZXSHZYB2W.json
  ?? intent/.canon/events/2026/09/18/01M2T9M3DF9CFDG03AFSQ209DJ.json
```

## Everyone runs a compatible Intent

`intent/.config/config.json` records the `intent_version` that wrote the project. Each canon file names its schema, and a binary refuses one it does not know. Here one file claims a newer schema than this Intent reads:

```
  $ grep -n '"schema"' intent/.canon/st/ST0002.json
  2:  "schema": "intent/thread@9.0",
  $ intent sync --apply
  error: could not read the committed canon
    caused by: residue: intent/.canon/st/ST0002.json -- schema-invalid -- schema is "intent/thread@9.0"; this binary reads "intent/thread@3.0"
  $ intent doctor
    intent/.canon/st/ST0002.json -- schema is "intent/thread@9.0"; this binary reads "intent/thread@3.0"
  doctor: 1 finding(s) -- the estate could not be read, so no thread, issue, view or file was checked -- 1 advisory(ies), not counted
```

A read verb such as `intent st show ST0002` still answers, from what the store held before: the store is truth on your machine, and it is a sync that reads the canon. Nothing more arrives until the clone runs an Intent that reads the newer schema. **Upgrade together.**

## Check the merge in CI

The commit gate runs on the author's machine and judges the author's tree. **The merge the forge makes is a tree nobody's gate judged**, so build Intent and run `intent doctor` on it in CI. On `pull_request`, `actions/checkout` checks out the merge result. Intent's own job, from `.github/workflows/pr-checks.yml` without its comments:

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

A finding fails the job. The runner's store is cold and is built from the committed canon, the same way a fresh clone builds one:

```
  $ git clone origin.git ci && cd ci
  $ intent doctor
  doctor: 0 finding(s) across 6 thread(s), 0 issue(s), 15 view(s), 32 file(s) -- 1 advisory(ies), not counted -- ...
```

---

Back to [Concepts](index.md), or on to [the store](the-store.md).
