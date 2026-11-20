# Design: a forge merges every open merge request with no hand step

**Status: under way since 2026-11-19 11:38Z by vc's date -u, on hv's go (hv decision 71).** hv decision 66 rules the direction, hv decision 67 the Highlander check, and hv decision 70 brings the same-thread case into this release. Written by vc from its own measurements on brew 3.4.0. A claim says whether it was measured or read. The defect is issue 0705.

## The problem, measured

Once ST0095 lands, two branches never mint the same id. They still both write three files that every record shares, so on a forge the first merge request to land blocks every other one that is open.

- **The reproduction** (vc, 2026-11-19, brew 3.4.0, banked under `refs/bank/vc/st0095/drive-*`): five branches each minted and started a thread from one main, and a second clone merged them as a forge's merge button does. After the first merge, none of the other four merged. Each conflicted with main in `intent/.intentfiles`, `intent/st/steel_threads.md` and `intent/todo.md`.
- **The census** (vc, 2026-11-19, brew 3.4.0): every lifecycle verb run in a scratch project, each change committed on its own, and the files it changed listed. The shared files each verb writes:

| Verb                                          | `.intentfiles` | `steel_threads.md` | `todo.md` |
| --------------------------------------------- | -------------- | ------------------ | --------- |
| `st new`                                      |                | yes                | yes       |
| `st start`, `st hold`, `st resume`, `st done` | yes            | yes                | yes       |
| `wp new`, `wp start`, `wp done`               |                |                    | yes       |
| `issues add`, `issues close`                  | yes            |                    |           |
| `st hydrate`, `st dehydrate`                  | yes            |                    |           |
| `ac new`, `ac satisfy`, `at new`              |                |                    |           |

Every other file a verb writes belongs to one record: its canon file, its event file and its realised views. There is no aggregate list of issues, since each issue has its own view.

- **git's union merge driver is no fix** (vc, 2026-11-19, git 2.56.1). Declared in `.gitattributes`, it resolves both kinds of file in a working tree's `git merge`. A bare repository's `git merge-tree --write-tree`, which is how a forge merges, ignores it unless `attr.tree` or `--attr-source` is set, and only the forge's configuration decides that.

## The ruling

hv decision 66: a forge merges any number of open merge requests with no hand step, by construction. So creating or changing one record writes no file that another record's change also writes:

1. **The two generated lists leave git.** Every clone renders them from its own store.
2. **Realisation stops being one shared manifest.** Whether a record is on disk is carried by the record or derived from its status.

The alternative, keeping the files with one hand step per merge request, is not taken.

## The design

### The two lists leave git (ic, with cc on what upgrade does)

- `intent/st/steel_threads.md` and `intent/todo.md` are still rendered on disk by the verbs that change them and by `intent sync`. Their content does not change.
- **A new project ignores them.** `intent init` writes both into `.gitignore` beside `intent/.cache/` and `intent/.backup/`, through the writer that writes those lines today.
- **An existing project stops tracking them once.** `intent upgrade` adds the two lines to `.gitignore` and takes both files out of git's index, leaving them on disk. It is one mechanical change, committed with the rest of the upgrade.
- **doctor names a project whose lists are still tracked**, with `intent upgrade` as the fix, as an advisory that is not counted. That follows cc's meaning for a project in today's format, given for ST0095 on 2026-11-19: such a project is consistent and readable and only cannot mint yet, so the new build works on it and refuses only the first mint, with `intent upgrade` as the one remedy.
- **What a reader loses**: the forge's web view no longer shows the two lists. Each realised thread's `info.md` and each issue's view stay tracked, so a thread or an issue is still readable on the forge. The index of them is not.
- **A thread's `acceptance.md` leaves git too** (hv decision 74, 2026-11-20, on ic's and cc's position). It is rendered from every criterion and test of its thread, so it joins the declared set as `intent/st/*/acceptance.md`: `intent init` ignores it, `intent upgrade` untracks it, and doctor's advisory names it. No package of ic's is needed. A split into a file per criterion and test was not taken: it would cost an M package and a tracked file per record, and it still could not show a test-backed criterion's verdict, which is computed from its tests. The forge then no longer shows a thread's criteria and tests as a page, nor, once WP-05 lands, the cover's package table. It still shows the cover's objective and context, each package's own view, the attachments and each record's canon JSON.

### Realisation leaves the shared manifest (cc)

Today `intent/.intentfiles` lists each realised record as a row, `STEELTHREAD:ST0095` or `ISSUE:0704` (read: `intentfiles.rs:106-121`), and its header says a row is state, never regenerated from status. The verbs in the table above rewrite it.

There are two ways to make it per record, and the choice is cc's call on meaning:

| Option                        | What a record carries                                                         | Which verbs write realisation                                                                    | What changes in meaning                                                                                                     |
| ----------------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| R1, the record carries it     | whether it is realised                                                        | every verb in the `.intentfiles` column, each on its own record, which all but two already write | nothing: realisation stays state                                                                                            |
| R2, derived, with an override | an override, only when a person hydrated or dehydrated it against the default | `st hydrate` and `st dehydrate` only                                                             | the default becomes a function of status: a WIP thread or an OPEN issue is realised, as `organize --default` computes today |

- **cc chose R1** on 2026-11-19 (AC-01.3). In cc's words: realisation is state today, not a function of status. Every transition verb edits it, and each takes `ListEdit::{AsDeclared, Suppressed}`, so a caller can move a thread's status and leave its realisation alone. R1 moves that state from one shared file into each record with no change in meaning, and upgrade's fold maps rows to records exactly: a listed record carries `realised: true`, and the field is absent otherwise. R2 would change meaning one of two ways: either an override outlives a transition, so a thread once dehydrated stays dehydrated after `st reopen`, which realises it today, or each transition clears it, which is R1's state under another name. R1's writes all fall on the record each verb already writes, except `st hydrate` and `st dehydrate`, which now write their thread's canon instead of the manifest.
- **Either way `.intentfiles` goes.** `intent upgrade` folds its rows into the records and removes it. The fold is byte-deterministic, so two branches upgraded apart merge clean.
- **It rides ST0095's schema bump.** ST0095 already gives the thread and issue canon a new schema version. With this field in the same unreleased version, a project migrates its canon once.
- **Not taken: keeping `.intentfiles` for overrides only.** A rare conflict is still a hand step.

### What this changes in ST0095

- **ST0095's WP-05 merges `.intentfiles` as a set, and regenerates the two lists in a conflicted merge**, which closes issue 0704. In a project this thread has upgraded, neither file is in git, so neither conflict occurs. A branch cut before the upgrade upgrades before it merges (ST0095's design, "Upgrading a project", point 3), so the set merge has no case left. cc dropped both on 2026-11-19 (AC-01.3): WP-05 goes from M to S, AC-05.2 is withdrawn, and issue 0704 is superseded by this thread and closes when it lands. A merge of two branches both still in today's format meets the conflict it meets today, and `intent upgrade` is that state's one remedy.
- **ST0095's WP-13** (ic) carries this thread's docs: the team page's merge sections are rewritten once, for both threads.
- **ST0095's brew smoke and drill** (dc's WP-03 and WP-12) run with the two lists untracked once this thread has landed.

### Two merge requests that change one thread: closed in this release (WP-05)

- **Measured**: `wp new`, `wp start`, `wp done`, the `ac` and `at` verbs and the thread's own lifecycle verbs all write the thread's canon file and its realised views. Two merge requests that change one thread meet there.
- After ST0095 the ids inside never collide, so `intent sync --apply --yes` keeps both sides. A forge cannot run it, so today such a merge request takes one hand step: merge main, run the sync, commit and push.
- **hv decision 70 closes it by construction in this release**, against vc's recommendation to leave it: each work package, criterion and test gets its own canon file, and the thread views are laid out so that no two of them share one. That is a second change to the canon's layout and a rewrite of the thread views, L to XL, on cc's chain, so it moves the release.
- **cc's design is the next section**, given on 2026-11-19 after ST0095 WP-04 landed. ic says how the views divide.

### Each record its own canon file (WP-05; cc's meaning, 2026-11-19)

- **One canon file per record.** The thread keeps `intent/.canon/st/<thread>.json` with its own fields only: schema, id, origin, title, slug, status, created, completed, objective, context, body, preamble, related and realised. Each work package goes to `intent/.canon/st/<thread>/wp/<wp>.json`, each criterion to `.../ac/<label>.json`, each test to `.../at/<label>.json`, and each attachment to `.../attachments/<path>.json`. Attachments go too, beyond hv decision 70's three kinds, because they sit inline in the thread file today, so two merge requests that edit different attachments of one thread would still meet there. Issues keep `intent/.canon/issues/<issue>.json`.
- **The name is the key, and nothing lists the children.** A file is named by its record's stored id, or a criterion or test by its label. No file holds an array of a thread's children, counts them or orders them: they are what the thread's directory holds, read in the waist's order (ordinals by number, hashed records by origin).
- **What stays shared is shared by meaning.** Two merge requests that edit one record, eg both change a thread's objective, still meet in that record's file. That is a real conflict, as it is for one issue today.
- **A view rendered from more than one record cannot stay in git.** Today two are: the cover's work-package table and `acceptance.md`. The cover renders only the thread's own fields, and its Acceptance and Work Packages sections say where to read the rest (`intent ac list <ST>`, `intent wp list <ST>`), in ic's wording. `acceptance.md` leaves git like the two lists (hv decision 74). A package's `WP/<id>/info.md` already renders only its package.
- **One canon version for the release, so a project migrates once.** The store migrates itself on open, per machine; the canon migrates by `intent upgrade`, once per project, committed. The first of ST0095 WP-06 and this thread's WP-01 and WP-05 to land bumps the canon format, and the others amend that same unreleased version in place. That is safe because no released build writes the intermediate shape and no canon on main carries it: main is upgraded at the cut, and the drill runs on a clone.
- **`intent upgrade` is one function from 3.4.0's format to the release's**, extended by each package: ids as strings (ST0095 WP-06), the `.intentfiles` fold (WP-01), the split (WP-05) and the two lists untracked (WP-02). It is byte-deterministic, so two branches upgraded apart merge clean, and idempotent. It prints every path under `wrote:` and `removed:`, and writes `project.json`'s schema last, so an interrupted upgrade leaves the project unmarked and re-runnable. Before it runs, the new build reads and writes today's layout for every write but a mint (hv decision 70), choosing the layout in one place by `project.json`'s schema.
- **On an existing thread**, eg ST0095, the upgrade rewrites the thread file with its own fields and `realised: true`, writes `wp/01.json` to `wp/14.json` (`seq: 4` becomes `id: "04"`) and one file for each criterion, test and attachment. It changes no id and no value besides those spellings, and the store needs no rebuild, since it already holds one row per record. For an all-ordinal project, every view that keeps its layout is byte-identical.
- **The 22 closed records still on disk go.** Main commits the views of 21 closed issues and the realised tree of completed ST0093, none declared in `.intentfiles`, because closing took each row out and left its views. Every fresh clone's `sync` already plans to remove them, brew 3.4.0's included. R1's fold maps the manifest exactly, so the upgrade removes them in its own commit, under `removed:`. Nothing is lost, since each is rendered from the store or carried in canon. An attachment whose bytes on disk differ from canon (an edit never synced) is refused by name, never removed.
- **Size and order**: L, after ST0095 WP-06. cc builds ST0095 WP-05 and WP-06's canon change so that the split extends it: id and origin go on the record structs, which later move whole into their own files.

### Highlander (hv decision 67)

- **One writer of the ignore lines**: the one that writes `intent/.cache/` today.
- **One owner of realisation, cc's**, read by organize, hydrate, dehydrate, sync, doctor and the views.
- **The set of generated aggregate views, declared once**, and read by init, upgrade, doctor and the commit gate.
- **The check runs as vc decision 133 says**, for this thread too: the sweep of the existing code (its `.intentfiles` and aggregate-view sections), a critic review of every unit's diff read by vc before land, and the sweep again at the close.

## Work packages

Each is one seat's, with its size, what it lands after, and its criteria in `acceptance.md`.

| WP  | What                                                         | Seat | Size | Lands after  |
| --- | ------------------------------------------------------------ | ---- | ---- | ------------ |
| 01  | Realisation lives in the record, and the manifest goes       | cc   | M    | ST0095 WP-06 |
| 02  | The two lists leave git                                      | ic   | M    | -            |
| 03  | The commit gate and the pull-request check follow            | dc   | S    | 02           |
| 04  | The proof: five merge requests through a forge stand-in      | dc   | S    | 01, 02, 05   |
| 05  | Each work package, criterion and test has its own canon file | cc   | L    | ST0095 WP-06 |

## hv's rulings

1. **The same-thread case is closed in this release** (hv decision 70), as WP-05.
2. **The thread goes ahead as designed** (hv decision 71, 2026-11-19): "Ok, I see. ST0096: Do it."
3. **Settled by cc on 2026-11-19, for the record**: R1, and ST0095's WP-05 drops its `.intentfiles` set merge and its regeneration of the lists.
4. **A thread's `acceptance.md` and the cover's package table stop being committed** (hv decision 74, 2026-11-20), ic's and cc's position, over a split into a file per record and over keeping them committed.

## The proof

Five merge requests are opened from one main, in two clones and two worktrees. Each one creates and starts a thread, adds a work package and adds an issue. They are merged in turn by a bare repository's `git merge-tree`, as a forge merges. No merge conflicts, and after a pull of each one, `intent doctor` is clean and both lists name every record.

Two more merge requests from one main change one thread: one adds a work package, the other a criterion and a test. They merge the same way, with no conflict, and `intent doctor` is clean after each is pulled (AC-04.2).
