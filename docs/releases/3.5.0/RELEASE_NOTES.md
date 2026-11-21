# Intent v3.5.0

**v3.5.0 gives every new steel thread, work package and issue an id that is unique by construction, and lets a forge merge any number of open merge requests with no hand step.** An id is now derived from its record's creation event, as git derives a commit's, so a branch, a worktree or a clone mints one with no counter and no network, and two never collide. Every existing ordinal keeps its meaning for ever and nothing is renumbered. The two generated lists and each thread's `acceptance.md` leave git, and whether a record is realised moves from one shared manifest into the record itself, so merge requests that create or change different records no longer conflict. A project moves to the new format once, with `intent upgrade`. The release also carries the fixes found while building it (hv decision 69): attachments on disks that fold case, `intent sync` after a pull, doctor's remedies, and the doors that read an id their own way.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** They were written from the CHANGELOG's 3.5.0 section as drafted against main at 8c76c3b27. Each entry below was read against that section's text, and claims nothing it does not.

**The section headings follow the CHANGELOG's 3.5.0 section**, after the upgrade, which leads because it is what a reader must do first; this page groups related entries, with what a reader upgrading needs.

## Upgrading

**Upgrade Intent first on every machine that opens a project, then run `intent upgrade` once in each project and commit what its closing line names** (ST0095 WP-06, ST0096 WP-01 and WP-02, hv decisions 70 and 71). The order matters, because Intent 3.4.x cannot read an upgraded project: it reports each canon file as schema-invalid, refuses, and changes nothing.

- **What `intent upgrade` does.** It writes every issue's and work package's id as a string (`703` becomes `"0703"`), folds `intent/.intentfiles` into the records it lists and removes it, writes each thread as its own file plus one file per work package, criterion, test and attachment, adds the aggregate views to `.gitignore` and takes them out of git's index while leaving them on disk, and writes the format into `intent/.canon/project.json` last. It runs only on a clean tree, naming each uncommitted or untracked path it refuses for. It names every path it changed in `intent/.cache/upgrade.paths`, a file for this machine that git ignores, and its closing line gives the two steps that commit exactly those as one mechanical change (issue 0745):

  ```
  git add --pathspec-from-file=intent/.cache/upgrade.paths --pathspec-file-nul
  git commit
  ```

  A plain `git commit` carries the deletions the run staged; `git commit --only` would track the two lists again. A re-run that changes nothing says it left nothing for git to commit. A `.gitignore` or `.prettierignore` it cannot read now halts the upgrade with its cause, where it was read as empty and overwritten. It changes no id. It is byte-deterministic, so two branches upgraded apart merge clean, and an upgrade cut short runs again.

- **A project you have not upgraded keeps working.** This build reads it, works on its existing ids as 3.4.0 does, and writes canon back in the project's own format, so a machine still on 3.4.0 can keep working beside it. Only a mint refuses: `st new`, `wp new` and `issues add` say "nothing was written", with `intent upgrade` as the one remedy. `intent doctor` shows `canon-format-behind`, and does not count it, so the commit gate passes.
- **An open branch upgrades before it merges.** Two branches both still in 3.4.0's format meet the conflicts they meet today; `intent upgrade` on each is the remedy, and because it is deterministic their merge is then clean.
- **Before you run `intent upgrade`, land or merge any open branch that adds work packages to a thread.** Two branches cut before the upgrade that each made the same ordinal package would meet, once upgraded, as an add/add on that package's one record file. This release refuses that merge, naming the thread and the package, and says how to resolve it by hand.
- **Remove a closed record's tracked views** (issue 0707). **[GAP, filled by intent-cc's upgrade unit before the cut: issue 0707's remedy for a tracked view of an unrealised record, in the order the product then has (vc, 2026-11-21).]**
- **The store migrates on open, one way.** This build moves each machine's store to this release's schema the first time it opens it, so a 3.4.0 binary on that machine refuses the store afterwards. `intent doctor --read-only` (below) judges a project without migrating it.
- **A new project** made by `intent init` starts in the new format, with the aggregate views already ignored.
- **Homebrew** (hv decision 77): the release and its assets are published at the cut, and the tap moves to this release only on hv's word. Until then `brew upgrade intent` stays on 3.4.0.

## Changed

### Git-style ids

**New threads, work packages and issues get git-style ids** (ST0095 WP-04 and WP-06, hv decision 61). An id is its kind's marker and the front of the SHA-256 of its creation event's id: 16 hex digits for a thread (`ST3f9a2c1d4e5f6a7b`) or an issue (`I3f9a2c1d4e5f6a7b`), 8 for a work package. The record carries that event's id as `origin`. Every table, view and message shows a short form: `ST3f9a2c1`, `I3f9a2c1`, `WP-c2e4`. A mint whose short form would equal a visible record's of the same kind draws again before anything is written. An existing thread keeps its ordinal and gains hashed work packages beside its ordinal ones, eg `ST0094/01` beside `ST0094/c2e41a0b`. After the upgrade no verb mints an ordinal.

**Type any unique prefix** (ST0095 WP-07 and WP-10). Every verb, MCP tool and `intent:///` address takes an id of either family whole, or a prefix of at least four hex digits of a hashed one (a prefix made only of digits needs one more than its kind's ordinal width), with or without the marker, in any case. A short form copied out of a table always works. An ambiguous prefix is refused with each candidate's id, kind, title and created date; none is picked. Every ordinal spelling keeps its meaning: `46`, `0046`, `ST46`, `ST0046`, `s46`, `i21`, `ST0092/09`. **One spelling stops** (hv decision 70): an over-padded five-digit ordinal such as `00046` is now a hash prefix and no longer reaches ST0046.

**Order** (ST0095 WP-08). Lists put ordinals first, by number, then hashed records by creation, in their present buckets and direction. A project of ordinals alone renders every view as before.

**Output a script reads** (ST0095 WP-02 and WP-10). `st new`, `wp new` and `issues add` print the id they minted on one `created:` line, the whole id; `issues add`'s first line is now that line. `st list`, `issues list` and `intent outstanding` show the short form unclipped beside ordinals, and the issues table gains a Created column; `--format json` and MCP carry the whole id.

**The explorer, Intent.app and the served page** (ST0095 WP-11). `intent explore` lists, opens and moves between hashed threads, work packages and issues by their stored ids, and prints each by its short form on the app row, the trail and the omnibox; a short form made only of digits keeps every digit, its leading zero included. Typing a short form into the omnibox finds the record. Intent.app shows hashed records, and the served page's refusals name both id families.

**GraphQL** (ST0095 WP-06 and WP-10, issue 0732). The SDL is version 25. `Query.issue(number: Int!)` is now `issue(id: String!)`, and `Issue.number` and `WorkPackage.seq` are `id: String!`, with no alias, so **a client of the old fields must move**. `thread(id:)` and `issue(id:)` take a whole id or a unique prefix, answer `null` for one that names nothing, and refuse an ambiguous prefix or a bad spelling. Threads, issues and work packages gain `origin` and `realised`, and a work package gains `created`, its creation day (`null` for an ordinal package). The thread and issue JSON Schemas and the DDL move with them.

**Renumber is retired; remint replaces it** (ST0095 WP-10 and WP-05). `st remint <id>` and `issues remint <id>` move an ordinal to its git-style id from the record's own creation event, with one argument. `st renumber` and `issues renumber` are refused, at exit 2 and over MCP, with remint as the remedy. In a merge, `intent sync --apply --yes` repairs an ordinal both branches minted before the upgrade by moving this clone's record to a git-style id.

**The project config carries no `st_prefix`** (ST0095 WP-14). A config that still has it loads unchanged.

### Merge requests merge with no hand step

**The two lists and every `acceptance.md` leave git** (ST0096 WP-02, hv decisions 66 and 74). `intent/st/steel_threads.md`, `intent/todo.md` and `intent/st/*/acceptance.md` are still rendered on disk, unchanged, by the verbs and by `intent sync`; every clone renders its own from its store. doctor shows a view still tracked as `aggregate-view-tracked` (run `intent upgrade`), and an ignored one missing or behind as `aggregate-view-stale` (run `intent sync --to-disk`), and counts neither. A view of a record that is not realised is `unrealised-view`, also uncounted (issue 0707, its doctor half). **What a reader loses**: a forge's web view no longer shows the two lists, or a thread's criteria and tests as a page. Each thread's `info.md` and each issue's view stay tracked.

**Realisation lives on the record** (ST0096 WP-01). Whether a thread or an issue is on disk is a field of its own record, and `intent/.intentfiles` goes. Each verb writes only its own record's canon, events and views, and `st hydrate` and `st dehydrate` write their thread's canon. Together these mean that merge requests opened from one main, each creating and starting a thread, adding a work package and adding an issue, merge in turn as a forge merges them, with no conflict and no hand step (ST0096 WP-04).

**A thread is its own file, and each of its records is a file of its own** (ST0096 WP-05). In an upgraded project a thread's canon is `intent/.canon/st/<thread>.json`, holding the thread's own fields, and one file per work package, criterion, test and attachment under `intent/.canon/records/<thread>/`, each named by its id or label; 3.4.0's format keeps one file per thread, byte for byte. Two branches that change different records of one thread change different files, and git merges them clean; two that edit the thread's own fields still take a side. A doctor finding about a record names that record's file, and an upgraded thread's `info.md` points at the verbs that list its packages and criteria instead of tabling them.

**Two branches that each add a work package to one thread keep both** (ST0096 WP-05). In an upgraded project each package is its own record file, under an id no other branch can share, so the merge has nothing to resolve. **In a project not yet upgraded, or for a package two branches made before the upgrade, a merge that would lose one is refused** (cc's ruling), with nothing written: in a project still in 3.4.0's format whenever both sides added packages to one thread's file, and in an upgraded project merging two pre-upgrade branches that each made the same ordinal package. In 3.4.0's format it prints "`sync` refused, and nothing was written: both sides of this merge added work packages to <thread>, which this project's ordinal format keeps inside <path>, so taking either side would drop the other side's", and its remedy is to resolve that file by hand as 3.4.0 resolved it: keep both sides' packages, give one of them the next number no package in the thread uses, `git add` it and run `intent sync --apply --yes`. In an upgraded project it prints "`sync` refused, and nothing was written: both sides of this merge added a work package as <thread>/<package> (<path>), and taking either side would drop the other side's", and its remedy is `git merge --abort`, then, on your own branch, re-creating the package with `intent wp new`, carrying its objective and body across with `intent set`, cancelling the old one, committing and merging again. Issue 0709 closes with the removal of the step that kept both by moving one, which this replaces.

### Doctor

**`intent doctor` is declared a write** (issue 0721, hv decision 74). Nothing it does changes: it still makes a missing store from canon and migrates an older one, one way, as every verb's open does. The register and its MCP description now say so.

## Added

**`intent id <token>...`** (ST0095 WP-10) says, for each token, whether it is an id, of which kind, and which record it names: `id`, `ambiguous`, `unknown` or `not-an-id`, with `key=value` fields, or JSON with `--format json`. The MCP tool `intent_id` gives the same answer. A script that needs to recognise an id asks the binary rather than carrying a pattern. `--read-only` writes nothing on disk: with no store, or a current one, it answers from what is there; where reading the store would migrate it or it refuses the store, it answers nothing, exits 2 and names the opening at `.store.opening`, as `doctor --read-only` does. The commit gate's parity scripts ask it that way, so a newer build never migrates the store it judges.

**`intent doctor --read-only`** (issue 0717) never creates or migrates the store. A store at this build's schema is judged read-only and left byte for byte; one that opening would create or migrate is not opened, and doctor exits 2, its JSON naming the store, its schema, this build's and what opening would do.

**doctor verifies hashed ids, and `wp done` judges a hashed package by its own criteria** (ST0095 WP-09). `id-not-from-origin`, counted, names a hashed id that is not its marker plus the hash of its `origin`; `short-form-shared`, shown and not counted, names two records of one kind that share a short form, which only two branches that have not yet met can make. A criterion's group is its package's id, eg `AC-c2e41a0b.3`, shown `AC-c2e4.3`.

## Fixed

### Ids at every door

**A canon file named in neither id family is refused by name** (ST0095 WP-06). 3.4.0 skipped such an issue file, and a later write deleted it.

**A work package's gate judges criteria written with a one-digit group** (ST0095 WP-04, issue 0712). `AC-1.1` passed the gate unjudged, because the gate asked for the group `01`.

**Four doors read an id as every other door does** (issue 0713). `intent events --subject 94` finds ST0094's and issue 0094's events where it found none, and `s94` or `i94` narrows to one. `todo done 56/02` works as `wp done 56/02` does. `ac descope --to` takes any spelling of its thread. MCP's `ac list`, `at list` and `at lint` refuse a `/NN` target, as the CLI does, rather than widening it to the whole thread.

**`intent search --kind wp` finds a work package's prose** (issue 0708), which it reported as kind `file`.

**`intent wb claim`'s refusal says what a board can claim** (issue 0710): "not an address a board can claim", with a remedy that names issues too.

### Attachments and disks that fold case

**`intent st attach --from` over an existing attachment replaces it** (issue 0715). Beside a watching intentd it printed ok and changed nothing. It now replaces the file and the store together, refuses over an unsynced edit to the file on disk, and refuses a new attachment over a file in the thread's directory the store does not carry, so an author's bytes are never overwritten.

**Two names that differ only in case are one file** (issues 0718, 0719, 0722 and 0711). An attachment whose name differs from a held one only in case is refused by the held name. An author's `Acceptance.md` is refused by name rather than stored over the view. `sync --to-store`, `organize`, `doctor`, the attachment reads and realisation read the disk's case fold once per project, and report a second spelling as `case-pair` where the disk does not fold, or `case-fold-unread` where the fold cannot be read. The v2 migration refuses two attachment paths that differ only in case, naming both.

### Sync, the store and the event log

**A write over a store that does not hold this checkout's canon is refused before anything is written** (issue 0739). A working copy that switched branches without its hooks re-rendered the other branch's records, then refused the project as unmigrated. It now refuses up front, naming the paths, with `intent sync --apply` as the remedy.

**After a pull, `intent sync` plans and reports only what its own run does** (issues 0741 and 0742). `sync --apply` no longer leaves a views step for a person after a pull of records another copy minted, a fresh `intent sync` afterwards plans nothing, and the dry run no longer promises view writes the run's own ingest has already made.

**Sync, organize and intentd's watch read the project's configured intent directory** (issue 0729), not `intent/` whatever `intent_dir` says; the watch refuses to start when its scope cannot be built rather than running deaf.

**One answer to whether git tracks a path** (issue 0730): a git that cannot be asked is an error at every door, not an untracked path at some.

**Ingest refuses an event whose `ts` is not a written event's stamp** (issue 0735), naming the file; doctor reports it as schema-invalid. **A failed log read is an error** where a work package's created day is read (issue 0732), not a missing day.

### Doctor and root files

**A doctor finding prints one remedy** (issues 0738 and 0740). A finding's own remedy replaces its class's rather than printing beside it, so the event-file findings no longer contradict themselves. Every stale-view finding, an unreadable cover, a section of `info.md` that does not round-trip and an unmigrated board now say what is true in their line and print their one remedy on their group's `remedy:` line, where each used to print a second remedy inside its own line. The report groups by class and remedy, and a finding's JSON carries `remedy` only when it has its own. The commit gate's refusal now says that each group's remedy line names what clears it.

**The first `st hydrate` of an upgraded project counts the views it wrote** (issue 0744), where it said 0 while its `wrote:` lines named them.

**`intent claude upgrade` leaves a `.claude/settings.json` the merge would not change byte for byte** (issue 0706), and doctor's `root-file-behind` line says what each file's upgrade keeps. A project's own deny rules after Intent's no longer carry the advisory.
