# Design: a forge merges every open merge request with no hand step

**Status: for hv's review, 2026-11-19. Nothing here is built, and no code lands before that review.** hv decision 66 rules the direction and hv decision 67 the Highlander check. Written by vc from its own measurements on brew 3.4.0. A claim says whether it was measured or read. The defect is issue 0705.

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

### The case that remains: two merge requests that change one thread

- **Measured**: `wp new`, `wp start`, `wp done`, the `ac` and `at` verbs and the thread's own lifecycle verbs all write the thread's canon file and its realised views. Two merge requests that change one thread meet there.
- After ST0095 the ids inside never collide, so `intent sync --apply --yes` keeps both sides. A forge cannot run it, so such a merge request takes one hand step: merge main, run the sync, commit and push.
- **Closing it by construction** would take a canon file per work package, criterion and test, and thread views that no two of them share. That is a second change to the canon's layout and a rewrite of the thread views, L to XL, on cc's chain.
- **Recommended: not in this release.** Two people changing one thread at once is a point where they coordinate anyway, and the hand step is reliable. hv decides at review.

### Highlander (hv decision 67)

- **One writer of the ignore lines**: the one that writes `intent/.cache/` today.
- **One owner of realisation, cc's**, read by organize, hydrate, dehydrate, sync, doctor and the views.
- **The set of generated aggregate views, declared once**, and read by init, upgrade, doctor and the commit gate.
- **The check runs as vc decision 133 says**, for this thread too: the sweep of the existing code (its `.intentfiles` and aggregate-view sections), a critic review of every unit's diff read by vc before land, and the sweep again at the close.

## Work packages

Each is one seat's, with its size, what it lands after, and its criteria in `acceptance.md`.

| WP  | What                                                    | Seat | Size | Lands after  |
| --- | ------------------------------------------------------- | ---- | ---- | ------------ |
| 01  | Realisation lives in the record, and the manifest goes  | cc   | M    | ST0095 WP-06 |
| 02  | The two lists leave git                                 | ic   | M    | -            |
| 03  | The commit gate and the pull-request check follow       | dc   | S    | 02           |
| 04  | The proof: five merge requests through a forge stand-in | dc   | S    | 01, 02       |

## For hv at review

1. **The case that remains**, two merge requests that change one thread. Recommended: not in this release.
2. **Settled by cc on 2026-11-19, for the record**: R1, and ST0095's WP-05 drops its `.intentfiles` set merge and its regeneration of the lists.

## The proof

Five merge requests are opened from one main, in two clones and two worktrees. Each one creates and starts a thread, adds a work package and adds an issue. They are merged in turn by a bare repository's `git merge-tree`, as a forge merges. No merge conflicts, and after a pull of each one, `intent doctor` is clean and both lists name every record.
