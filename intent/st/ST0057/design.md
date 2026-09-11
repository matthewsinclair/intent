---
verblock: "18 Aug 2026:v0.1: vc - disk as a sparse projection of the store"
---

# ST0057 -- Disk as a sparse projection of the store

## Objective

Make disk optional without making anything unrecoverable.

When this thread opened, every file under `intent/st` was on disk because that is where v2 kept truth. Under D01-as-reversed the store is truth and the files are a projection -- **but a projection nobody can regenerate is not a projection, it is the original with extra steps.** This thread ends with three things true at once: `intent/st` holds little more than an index and the live work; anything not on disk is provably reproducible; and a human with an editor and no working tool can still read the whole project.

## The three-layer model, because the brief collapses two of them

hv's framing was _"the DB is SSOT and disk is a sparse copy of the db out to realised files based on an index."_ That is right for two of the three things on disk and fatal for the third.

| layer     | what (as built)                                                                                                                               | committed            | who derives it              |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- | --------------------------- |
| **canon** | `intent/.canon/st/<ID>.json`, `intent/.canon/st/<ID>/<path>` (opaque bytes), `intent/.canon/issues/<NNNN>.json`, `intent/.canon/project.json` | **yes**              | authored through the facade |
| **store** | `intent/.cache/intent.db`                                                                                                                     | **NO -- gitignored** | rebuilt from canon          |
| **views** | `info.md`, `acceptance.md`, `WP/<NN>/info.md`, `steel_threads.md`, `intent/todo.md`                                                           | yes, where realised  | rendered from the store     |

**The store is SSOT for a running tool on one machine. It is not what travels.** In this repository `intent/.cache/` is gitignored at `.gitignore:125` and the ignore rule's own comment says the name contradicts the model. The migration from v2 converges the same rule into a converted estate (`converge_gitignore`, `facade.rs:193`); **a project born by `intent init` gets no `.gitignore` at all**, so neither `intent/.cache/` nor `intent/.backup/` is ignored there -- a defect, not a design choice. `intent sync --to-disk` is documented as writing _"the committed extract"_ -- the vocabulary already distinguishes the runtime store from the durable record.

**So sparseness applies to VIEWS. Canon is never sparse.** If the manifest governed canon, an unrealised artefact would exist only inside a gitignored database: absent from a fresh clone, and destroyed by `rm -rf intent/.cache`. **D29 -- a gitignored path is never canon -- is not a style rule here, it is what makes a clone complete.**

That single ruling is what makes every deletion in this design safe rather than careful: **`organize` only ever removes a file it can regenerate, from a source sitting beside it in the same commit.**

### Measured, because the sizes settled the argument

Measured when this was designed (2026-08-18): **the single live thread, ST0056, carried more than a third of all the markdown under `intent/st`**, a completed thread a sliver of it, and the whole of canon weighed less than the rendered views. Sparse views buy nearly everything; complete canon is the price of a clone that works. `du` over `intent/.canon` and `intent/st` re-measures it for any tree.

## D57-1 -- Canon relocates, and stays one file per artefact

Keeping canon complete does not mean keeping it in the way. hv's requirement is that `intent/st` stop holding a bajillion files, and canon-in-place does not deliver that: one directory per thread remains whatever happens to the markdown.

- **A -- canon in place.** Bytes fall by more than half; `intent/st` still lists one entry per thread. **Does not meet the requirement.**
- **B -- one consolidated `threads.jsonl`.** **Rejected: concurrent writers make a single canon file a merge-conflict generator**, and this estate runs several nodes committing into one tree.
- **C -- `intent/.canon/st/ST0011.json`, still per-artefact. RULED, and built** (`Project::thread_json`, `project.rs:1375`).

C is the only one that empties `intent/st`, keeps a clone complete, and keeps a per-artefact diff so two nodes editing two threads never conflict. It is a path change, not a model change.

**After C, `intent/st` holds no canon: `steel_threads.md` and thread directories only.** Which thread directories is `.intentfiles`'s business (D57-2), and a directory lingers until something removes it: a lifecycle verb that delists a thread leaves its files where they are until the next `intent organize --apply`, and `intent doctor` does not report such a leftover directory as a finding.

## D57-2 -- `.intentfiles` is durable state

**SUPERSEDED 2026-08-19. This decision used to read _`.intentfiles`, and why it has two regions_, and hv replaced it outright mid-implementation.** The two-region text is kept below the rule so a reader meeting the retired shape in an old commit can tell which one they are looking at.

**THE RULE, hv's own words:**

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

The manifest is committed, and it has to be: the set it governs is committed. **A machine-local manifest would let `organize` on one machine delete files another machine depends on, and land the deletion in a commit with no declared reason.** Committed, hydration and dehydration are reviewable -- the diff shows the manifest line changing beside the files appearing or disappearing. **That half of the original decision survives unchanged.**

An excerpt of the header the tool writes (its one home is `DEFAULT_HEADER`, `intentfiles.rs:488`), with example entries:

```
# .intentfiles -- WHICH DATABASE ARTEFACTS ALSO HAVE A REALISED FORM ON DISK.
#
# REALISATION IS DRIVEN FROM THIS FILE. Commands change it; `intent organize`
# realises it. Nothing recomputes it from status afterwards, so a write here is
# a CHANGE TO STATE and never a REGENERATION of it -- which is why re-running
# `--default` over a file that already exists changes nothing without `--force`.

STEELTHREAD:ST0056
STEELTHREAD:ST0057
```

**MANY WRITERS, NO RECOMPUTATION.** As built, the op decides, not the status it lands on: `st start`, `st resume` and `st reopen` add the id; `st done`, `st cancel`, `st hold` and `st triage` remove it; `st new` and `st reinstate` do neither; `st hydrate` / `st dehydrate` act on it directly; `intent edit` adds the id of the thread it realises; and a human may edit it by hand. `st done --keep` and `st cancel --keep` close a thread and leave its entry. All of those are ordinary writers and none is privileged. **_AUTHORED_ WAS vc's WORD FOR IT AND IT WAS WRONG** -- hv corrected it before either builder committed to a shape. It does not mean untouched by commands; it means nothing recomputes it.

**THE ONE DERIVATION FROM STATUS IS THE DEFAULT, AND IT NEVER OVERWRITES ON ITS OWN (WP-11).** `intentfiles::default_declaration` (`intentfiles.rs:542`) writes the header plus one `STEELTHREAD:` line per WIP thread and nothing else. `intent organize --default` writes it when the file is absent and changes nothing when it is present; `intent organize --default --force` regenerates it from status only after a confirmation on a terminal, and refuses without one. The migration from v2 and `intent upgrade` write the default when the file is absent. `intent init` writes the header with no entries.

**A LIFECYCLE VERB CHANGES THE LIST, NOT THE TREE.** `st done` on a realised thread removes its line and leaves its files on disk; the next `intent organize --apply` removes them. `st dehydrate` is the verb that does both at once.

**THE REGIONS ARE GONE BECAUSE THE REGENERATION IS GONE.** hv's question settles it: _why isn't the organise operation simply: a) look at .intentfiles, b) hydrate the items in the file, c) dehydrate any previously hydrated items that are no longer in the file._ The two regions existed only because the file was machine-generated -- if `organize` rewrote the list from status every run, a hand-added line would be wiped, so a protected region was needed. **Remove the regeneration and the protected region has nothing to protect against.** A write is a CHANGE TO STATE, never a REGENERATION of it.

**IT ALSO EXPLAINED A LOOSE END NOBODY HAD ACCOUNTED FOR: `intentfiles::render` had ZERO production callers.** That looked like an unwired writer and was not -- the thing it does is not needed. Nobody failed to wire it. It has since been deleted (D57-9).

**ABSENT IS NOT EMPTY.** A missing manifest means nobody has said, so everything stays: `organize` plans to realise every thread and removes nothing, and `doctor` checks every view as realised. A manifest present and declaring nothing means keep nothing. This estate once carried a file of pure comment for a day, which is the second state, so its whole closed estate sat on the removal path **by omission rather than by decision** -- held back only by the dehydration gate. Populating the list was what made the removal intentional. Because `intent init` writes the present-and-empty state, a project born on v3 realises no thread until one is started, hydrated or opened with `intent edit`.

**THE GRAMMAR REFUSES RATHER THAN SKIPS.** `<SIGIL>:<ID>` with sigil `STEELTHREAD` alone, an optional trailing `# comment`, blank lines and whole-line comments, nothing else -- except that the parser still admits a `# BEGIN INTENT` / `# END INTENT` pair, which D57-9 ruled out and which is not yet removed. `ISSUE` left the grammar on hv's ruling of 2026-08-20 (`d855ea1f`): an issue lives in canon and the store only and has no realised form for a line to declare, and WP-12, which would have given it one, is cancelled. A line the parser cannot read aborts the run with its line number: the acting verbs (`organize`, `edit`, `st hydrate`, the lifecycle verbs that write the list) refuse. `doctor`, a reporter, reads an unparseable manifest as realising everything and, as built, does not report the parse failure -- AC-10.2 names that reporter as owed. **A skipped line drops an artefact from realisation and leaves an estate indistinguishable from one that never listed it** -- the silent-drop shape v2.19.0 already paid for twice, in `ac gate`'s F1 fix and in the AT row grammar's `at lint`. **This half survives unchanged too.**

### The retired two-region shape, for readers of older commits

```
# Lines between BEGIN and END are GENERATED by `intent organize` from status.
# Lines outside the markers are PINS and are never rewritten or removed.

STEELTHREAD:ST0011        # pinned by hand: reading the old test-suite thread

<!-- STEEL THREADS: BEGIN -->
STEELTHREAD:ST0057
<!-- STEEL THREADS: END -->
```

**The pin bought exactly one property: a closed artefact staying on disk.** Under the rule above that is ordinary behaviour rather than an exception -- **the list wins over status, both directions, and status has no vote at `organize` time at all.** Measured on this estate at `e7f00e65` across four statuses: `completed` and `cancelled` threads removed while unlisted, `not-started` and `wip` threads kept while listed, and ST0010 (`cancelled`) hydrated and kept while listed.

## D57-3 -- `intent organize`, four answers and one refusal

v2's `organize` was a status-directory tidier with no work left under the flat layout, so v3 inherited the name rather than inventing one; `intent organise` is the same handler. It **previews by default and acts only with `--apply`** -- the preview is the same run with the removal, the write and the digest guard withheld (`organize::Mode`, `organize.rs:150`). `--default` and `--default --force` write the manifest itself (D57-2).

| declared | on disk                              | action                                                            |
| -------- | ------------------------------------ | ----------------------------------------------------------------- |
| yes      | absent                               | **HYDRATE**                                                       |
| yes      | present                              | **VERIFY** -- re-render in memory, write only if the bytes differ |
| no       | absent                               | nothing                                                           |
| no       | present                              | **DEHYDRATE**, subject to the gate below                          |
| --       | a path the renderer does not produce | **UNCLAIMED -- report, never remove**                             |

**The fifth row is the one that matters.** A file the renderer cannot make is either an attachment or something a human put there. **`organize` must not be the thing that decides an unrecognised file is rubbish.**

**As built (`organize::plan`, `organize.rs:496`), the rows cover attachments the store carries as well as views, and the fifth row is narrower than "not rendered".** A carried attachment under a declared thread is written from the store when absent (HYDRATE) and reported when its bytes differ from canon's `sha256` (DIVERGED, below); under an undeclared thread it is DEHYDRATED through the same gate as a view. UNCLAIMED is a file under a thread directory that is neither a generated view nor an attachment the store carries -- one refused at ingest by the size cap or the naming gate (D57-7) lands there. Two additions the table does not show: `steel_threads.md` and `intent/todo.md` are EXEMPT, always kept, because no manifest entry can imply an index view; and a declared thread whose files still sit in a v2 status bucket (`intent/st/COMPLETED/<ID>/` and siblings) is refused realisation by name rather than given a second, smaller copy. A run also removes any directory its own removals emptied, and reports it as pruned.

### The dehydration gate

Before removing any view: re-render it from the store into memory, compare to the bytes on disk, **refuse on any difference and report the path.** A difference may mean the disk holds something the store does not, and that something is destroyed by the removal. One render and one comparison per file -- **fail-safe by construction rather than by discipline** (`organize::gate`, `organize.rs:1073`), and it is the same render-against-disk comparison `doctor`'s view-skew check makes (`views::skew`, `views.rs:1512`) over the same renderer. The refusal is per file and the run continues; `st dehydrate` instead refuses the whole thread if any one of its files cannot be shown to be in the store.

**As built, a difference is not always an edit, and the gate cannot tell which.** A view rendered before the store moved on differs too -- and every view's banner carries the tool version, so a version bump makes every realised view of an undeclared thread differ from the current render. The gate refuses both alike and its message names both causes (`OrganizeError::HandEdited`, `organize.rs:279`).

### An ATTACHMENT is not a VIEW, and one policy for both discards work

**Authority follows AUTHORSHIP.** A view is authored in the model, so disk divergence means the file is stale. **An attachment is authored ON DISK, so divergence means the STORE is stale.** Overwriting an attachment destroys the author's edit; ingesting a view promotes a stale generated file into canon. Same divergence, opposite remedies.

**`organize` NEVER resolves an attachment divergence.** It reports the path as diverged and names both remedies -- `intent sync --to-store` to take the disk copy, or restoring the file if the store is right -- and touches neither side. _"Decide which way to sync"_ is a human decision by definition, and **a tool that decides it silently is choosing whose work to discard -- right most of the time and catastrophically wrong occasionally, which is the worst profile available.** `Project::classify` is the single answer to what a file is, so the asymmetry is implementable rather than aspirational.

### Idempotence is a measured requirement

`organize` run twice changes nothing, **including mtimes.** Measured 2026-08-18: the render of the day re-emitted a large share of the `.md` files byte-identically every pass, moving their mtime with zero content change. Harmless for conservation, not harmless for `file_index`, whose `clean`/`changed` state is computed from exactly that. **Write only on content difference** -- as built, every write goes through one `WriteSet`, which skips unchanged bytes, and two consecutive `organize --apply` runs leave every mtime where it was.

### It locks against ANY process, not other `organize` runs

`info`, `st list`, `doctor` and `export` all **materialise the store on access** -- a fresh clone's first read verb builds it. So a peer typing `intent st list` opens the same file `organize` is reconciling against. **"The estate was quiet when I measured" is not establishable over any window**, which makes the moment-of-act digest the only defence that survives a concurrent read: digest the measured tree, re-compute immediately before the irreversible step, refuse on any difference.

## D57-4 -- `intent edit <ID>`, and why not `wip`

hv proposed `intent wip <ID>`. **In this project `wip` already means a WP status, a thread status, `intent/wip.md`, and every node's board.** A fifth meaning collides with four live ones.

**`intent edit <ID>`**, dispatching on id shape: hydrate if needed, record it in `.intentfiles`, print the path. `intent st edit` already printed the path to a thread file, so this is that verb learning to hydrate first -- one behaviour, one home. As built, `intent edit [KIND] [ID] [FILE]` (FILE defaults to `info`) and `intent st edit <ID> <FILE>` both reach one handler (`edited`, `intent-cli/src/render.rs:1546`) and one service (`Facade::edit`, `facade.rs:4579`); `--path` prints without opening, `--editor` opens the file, and `--browser` (or `intent browse`) opens the entity through intentd.

**A hand realisation is recorded in the list.** Otherwise the next `organize` would dehydrate what you just opened, because the artefact you opened is typically one the list does not declare. This was written as _write the PINNED region_ while the manifest had two regions; since D57-2's replacement, `edit` adds the thread's `STEELTHREAD:` line to `.intentfiles` (`intentfiles::pin`, `intentfiles.rs:597`). An absent manifest is left absent -- creating one to hold a single entry would declare that entry the whole of what is realised (`Facade::hydration`, `facade.rs:3119`).

## D57-5 -- The full text realisation, and a refusal whose reason expires

**hv, 2026-08-18:** _"I would expect to be able to trivially generate a full text realisation of all of the files for the entire project into, say, `.backup/...` so that I have that on-disk fallback."_

This is **not** `organize`. `organize` decides which artefacts have _working_ form under `intent/st`. This is a complete, disposable, regenerable text snapshot **elsewhere**, and it is the assurance that makes sparseness acceptable to a person rather than merely to a tool.

**The two assurances are different and the design needs both.** The dehydration gate proves **the store holds it**. A complete text export proves **a human can get it back without the tool.** hv is asking for the second, and nothing in the design provided it.

**`export --format md` EXISTED AND WAS DELIBERATELY REFUSED when this was written.** The refusal read:

```
"design.md:57 names `md` as an export projection and it is not one"
"The views are already in the tree -- `intent sync --to-disk` rewrites
 them; for data a program will read, use `--format json`"
```

**That refusal was correct then and became false the moment this thread landed.** Its justification was _the views are already in the tree_, and the entire purpose of D57-1 through D57-3 is that they are not. **A claim that was true when written and expires on a change already scheduled** -- the same class as `Issue::body`'s trim, caught here before it shipped rather than after.

**RULED: `--format md` is withdrawn from the refused set as part of this thread, not before. BUILT that way.** `md` is now `Projection::Realises` (`export.rs:292`) and delegates to `Facade::realise` (`facade.rs:4109`), hv's ruling of 2026-08-20: `intent export --format md` writes a directory tree and prints its destination and a denominator per population, instead of a document on stdout. `yaml` stays refused.

Requirements on the realisation, each as built:

- **Complete, and it says so with a denominator.** Every thread, WP, issue, attachment and view, with a printed count against the canon totals -- the run prints `threads`, `wps`, `issues`, `attachments` and `views` each as written-of-canon, and names any population that fell short. **A partial export that reads as complete is worse than no export.**
- **Out of the way.** It lands at `intent/.backup/text/<UTC>/` (`canon/`, `views/` and `attachments/` beneath it), beside `intent/.backup/db/`, the store snapshots, so it is a second mechanism in an established local-never-commit home rather than a new convention. The repo-root `.backup/upgrade/` namespace is v2's rollback area and v3 does not write it. `intent/.backup/` is gitignored in this repository (`.gitignore:143`) and in a migrated estate; a project born by `intent init` does not ignore it (see the three-layer table).
- **Regenerable, never authoritative.** It is a fallback. Nothing reads it back; there is no import path; `classify` never sees it, and the estate scan skips `.backup` (`sync::SKIPPED_DIRS`, `sync.rs:51`).
- **Cheap enough to be habitual.** If it is expensive nobody runs it, and an assurance nobody exercises is not one.

**AND IT HAD A DEPENDENCY NOBODY HAD NAMED (cc, 2026-08-18, found while bootstrapping a scratch project to drive something else): `intent init` WAS NOT IMPLEMENTED.** `error: 'init' is a known command that is not implemented yet`. A v3 project had to be hand-bootstrapped with a v3-stamped `config.json`. It is implemented now: `intent init` creates a working v3 project from an empty directory, and AC-06.4 exercises the text realisation from one.

**That was a precondition of D57-5 rather than a neighbouring gap.** The whole value of a fallback is that someone can exercise it -- and the natural way to exercise "can I get everything back into text" is _make an empty project, export into it, read it_. **You cannot demonstrate a fallback from a clean directory if you cannot create a clean directory**, so `init` gates the assurance, not merely the onboarding. cc's line is the one to keep: _a text-export assurance nobody can exercise from a clean directory is exactly the assurance nobody exercises._

## D57-6 -- The canonical three, RULED

**`design.md`, `impl.md` and `tasks.md` are carried as ATTACHMENTS. `THREAD_PROSE` is deleted from the classifier.** Both are built: no `THREAD_PROSE` constant remains, and this document is carried in ST0057's canon as the attachment `design.md`.

The inventory this was decided against was every `.md` under `intent/st` the census never enumerated, in two populations: the canonical three per thread, and one-off `.md` files under a thread.

**The one-offs were already carried** -- `.md` was in the attachment extension set of the day and `classify` did not call them typed. **So the ruling governed exactly the canonical three, and the change was subtractive: three strings came out of a constant.** The extension set has since gone too: `classify` consults no extension (`project.rs:1406`), so every file under a thread that is not a generated view is an attachment, carried when it passes the 1 MiB cap and the naming gate (D57-7).

Two arguments carried it. **Rule 2 of the attachment spec:** a typed field earns its parsing because the model has fields for what comes out. These are freeform prose under arbitrary headings and the model has none, **so parsing them would discard structure into nothing -- which is precisely how `## Related Steel Threads` became rows of `LOST-PROSE`.** And **it is about ongoing behaviour, not legacy data**: `st new` does not create these files, but `THREAD_PROSE` still named them, so the moment anyone hand-wrote a `design.md` -- which the v2 habit, the templates, and every existing thread taught -- **it was indexed for search and carried by nothing. The next one had the same problem as every one before it.**

**This document is the demonstration.** Written as `design.md` it was, until this ruling, the one filename in the estate that would not be carried; written as anything else it would have been. **The tool's own default name for a design document was the only name it would not keep.**

## D57-7 -- Where ancillary files live, and the one directory they cannot

Text attachments need no home: they live inline in canon as `Attachment.text`, and the working copy dehydrates. **Everything else does need one**, and when this was decided the estate under `intent/st` was mostly everything else: `.md` alongside a large population of `.tap` baselines in one ST0056 directory, `.json`, `.sh`, `.txt` and `.tsv`.

**REJECTED -- `intent/.cache/`.** It is gitignored at `.gitignore:125`, holds only the store (`intent.db`), and **nothing under it is tracked** (`git ls-files intent/.cache` prints nothing). The ignore rule's own comment records what it is for: the database was showing as `?? intent/.cache/`, _"one `git add -A` from entering history as a binary blob"_, and the line **is the precondition for D29** -- the ingest corpus excludes ignored paths, so until it existed the DB was outside the corpus only by accident of path shape. Committing that directory reverses both protections at once.

**And the name is the sharper objection, because it fails in the dangerous direction.** `cache` means regenerable-and-discardable. An opaque attachment is precisely what is NOT regenerable. `rm -rf intent/.cache` is a reasonable thing to do to a cache, and under that layout it would destroy the only copy of a hand-authored file. The ignore rule already concedes _"`intent/.cache/` remains a name that contradicts the model"_; adding committed content compounds a documented defect rather than repairing one.

**REJECTED -- opaque attachments never dehydrate.** This was the first answer and it was wrong, and **the error was reading D57-3's "regenerate" as "re-render from JSON"** -- the rule says _a source sitting beside it in the same commit_, and it does not say the source must be JSON. That is the whole reason, and it stands on its own. **THE COST FIGURE THIS PARAGRAPH USED TO CARRY WAS WRONG TWICE AND IS WITHDRAWN RATHER THAN QUIETLY DELETED (cc measured it, vc ruled, 2026-08-19).** It charged this option with pinning every file that was neither `.md` nor `.json` to disk permanently, the `.tap` baselines surviving ST0056 forever. **Wrong population: every `.tap` is valid UTF-8**, so under AC-03.2's own words -- form follows CONTENT -- a `.tap` is a TEXT attachment, and the sidecar mechanism this paragraph rejects cannot reach them under any reading. **Wrong cost: those files are pinned to disk by GIT regardless**, because they are committed and tracked. The alternative was never _not on disk_; it was _on disk AND in the store_. **An argument that rejects an option by charging it a cost both options pay is not an argument**, and the rejection was carried by the D57-3 misreading alone the entire time. **The `.tap` baselines' fate was a CARRY-LIST question, and it has been answered by the carry list going away**: the `md` / `txt` / `sh` extension allowlist that left them uncarried (and reported UNCLAIMED) was replaced by a size cap, `ATTACHMENT_CAP_BYTES` at 1 MiB (`project.rs:50`), so they are now carried as text attachments in ST0056's canon and dehydrate with their thread. The concern that a baseline exists to be the recorded PAST, so regenerating one destroys the comparison it exists to make, is met by carrying its bytes rather than regenerating them.

**RULED. Canon holds the truth; the working copy dehydrates like everything else.**

```
intent/.canon/st/ST0056.json          text attachments, inline as `text`
intent/.canon/st/ST0056/<path>        opaque attachments, as FILES
intent/st/ST0056/<path>               realised working copy -- DEHYDRATES
intent/.cache/                        gitignored, store only, never committed
```

**One rule, uniformly: everything under `intent/st/` is a realised working copy, and everything dehydrates.** Canon holds truth in whichever form suits it -- inline for text, as a sibling file for bytes. Regeneration of an opaque attachment is a byte copy from a source in the same commit, which satisfies D57-3 exactly rather than bending it.

**As built, the canon half holds and the working-copy half does not yet hold for opaque attachments.** Ingest writes an opaque attachment's bytes to `intent/.canon/st/<ID>/<path>` and records its `sha256`; the test behind AC-03.1 (`opaque_attachment_canon.rs`) covers that canon round trip, not the working copy. But the verbs that realise and remove the working copy carry an attachment's bytes only from its inline `text`, which an opaque attachment does not have (`organize.rs:577`, `organize.rs:589`). So:

- **An absent opaque working copy is never written back.** `organize --apply` lists it as `hydrated`, and both it and `st hydrate` record it as hydrated in the event log, while writing nothing; `sync --to-disk` leaves it absent too. The text realisation (D57-5) is the one path that does copy the bytes out.
- **A present opaque working copy is never removed.** The dehydration gate reads the file as text, fails on the first non-UTF-8 byte and refuses it -- through `organize --apply` as a per-file refusal, and through `st dehydrate` as a refusal of the whole thread.

Nothing is lost -- the bytes stay in canon and on disk -- but a thread carrying an opaque attachment cannot be fully dehydrated, and one whose opaque working copy was deleted cannot be rehydrated. That is a defect against this ruling, not a revision of it.

Three things fall out, and each removes work rather than adding it:

- **No `.intentfiles` grammar change.** Attachments are per-thread, so `STEELTHREAD:ST0056` hydrates the thread AND its files. The manifest still names threads only.
- **`sha256` in canon buys drift detection on the working copy** -- an attachment edited on disk without the store knowing is 5.1b's divergence case, and `doctor` reports it by comparison rather than by inference, as an `attachment-drift` finding naming the path and both hashes.
- **The naming rule protects two things with one check.** A file that cannot be named safely can be given neither a canon path nor a URL (D57-8), so rejection at the gate covers storage and addressing together. As built (`project::attachment_name`, `project.rs:1022`), the gate refuses a path that escapes the thread's canon directory, one that is not already normalised (`a/./b.md` would share a sidecar with `a/b.md`), and one that does not read back unchanged through an `intent:///threads/<ID>/attachments/<path>` address; a space in a name passes. Rejection is NOT retroactive: a file refused at ingest is not carried, so `organize` reports it as UNCLAIMED under its fifth row and never silently removes it.

**OPEN, and it is an optimisation rather than a design question:** copy versus hardlink on hydration. Copy is simpler and doubles bytes for hydrated threads only. Neither is built yet, because hydration does not yet write an opaque working copy at all.

## D57-8 -- `intent://`, the address of a piece of data

**Hydration makes a file path a statement about a moment.** `intent/st/ST0034/design.md` either exists or does not depending on what `organize` last did, so every reference to it is conditional. **Measured at `ce532a97`: tracked estate prose cited `intent/st/ST####/<file>.md` paths throughout, and the most-cited single artefact was `ST0034/design.md` -- a COMPLETED thread, and therefore among the first to dehydrate.** A few AT rows also pointed inside `intent/`; the rest point at `native/rust` code and are unaffected.

**So references must name the ENTITY, never the file.**

```
intent:///threads/{stid}
intent:///threads/{stid}/wp/{wpid}
intent:///threads/{stid}/ac/{acid}
intent:///threads/{stid}/at/{atid}
intent:///threads/{stid}/attachments/{path}
intent:///issues/{issueid}
intent:///nodes/{moniker}
intent:///nodes/{moniker}/inbox/{sender}/{stamp}
intent:///events/{id}
```

**AND FOUR COLLECTION ADDRESSES, WHICH THIS DOCUMENT ALREADY REQUIRED IN TWO OTHER CLAUSES AND OMITTED HERE (ic found it 2026-08-20; vc ruled it 16:20Z).**

```
intent:///threads
intent:///issues
intent:///threads/{stid}/wp
intent:///threads/{stid}/ac
```

**THE LIST ABOVE WAS NEVER AMENDED WHEN THE CLAUSES BELOW IT GREW, AND THE OMISSION IS THIS DOCUMENT'S OWN.** The READ/WRITE section says _server-assigned ids (threads, issues, WP seq) are a `POST` to the COLLECTION address_, which requires three of them by name. The under-addressing clause says _`/threads/ST0056` returns the cover; `/threads/ST0056/ac` returns the acceptance view_, which requires the fourth **and writes it out in full**. So the four were mandated in prose, implemented in `address.rs`, and absent from the only place a reader -- or a test -- goes to enumerate the grammar.

**THE COST WAS PAID BY SOMEBODY DOING IT RIGHT, AND THAT IS WHY THIS IS AMENDED RATHER THAN NOTED.** `d57_8_forms()` in `address_resolution_single_home.rs` is a hand-copy of the fence above, and its caller calls it _"the WHOLE list"_. It was built by reading the DESIGN rather than `address.rs`, on the sound reasoning that a denominator read out of the implementation agrees with it by construction. **The method was correct and it returned a list short by four, because the two clauses being sourced from contradict each other four paragraphs apart.** Every other instance of this class in the estate has been a document going stale against the CODE; this is a document going stale against ITSELF, and no instrument here looks there -- `at lint` checks rows against files, `doctor` checks views against canon, and a design document's clauses are checked by a reader noticing.

**COLLECTIONS ARE NOT ENTITY FORMS, AND KEEPING THEM APART IS LOAD-BEARING RATHER THAN TIDY.** AC-07.1's population is _every ENTITY form in D57-8's list_, and against nine entity forms it is faithful -- so it is not reopened by this amendment and no green rests on a falsehood. **What the four DO leave uncovered is their own resolution**, which satisfying AC-07.1 completely does not reach. That is a new criterion (AC-07.7) rather than a widening of an old one, on the discriminator this thread keeps returning to: **what does satisfying the existing row completely still leave broken?**

**THE COLLECTION IS THE ADDRESSEE AND THE VIEW IS ITS REPRESENTATION.** That is how `/threads/{stid}/ac` reaches the acceptance view without giving a VIEW an address, which the clause above forbids in the sentence that exists to stop the scheme becoming a path alias. It applies the entity-versus-representation split rather than extending it.

**Empty authority means THIS project.** Nearly every reference is intra-project, and one that hard-codes the project name breaks on rename or fork. Cross-project references carry the slug and are meant to resolve against intentd's project registry. **As built, nothing resolves one yet:** the grammar parses and round-trips a slug authority, and the in-process verbs refuse it: the writers name the registry as where a cross-project write would resolve (`Facade::post`, `Facade::put`, `Facade::set` at `facade.rs:7235`, `:7385`, `:8071`), and `Facade::hydration` refuses to realise another project's artefact into this tree (`facade.rs:3072`).

**VIEWS GET NO URL, and this is what stops the scheme becoming a path alias.** A view is derivable from its entity, so a reference to a view is a reference to its source. Giving views addresses would re-create, inside the scheme, the exact conditionality the scheme exists to remove. **`?format=` selects a REPRESENTATION of the addressed entity; a path segment would name a separate thing.**

**READ/WRITE (hv, 2026-08-18). The URI is the address of a piece of data, and the data is mutable.** The write path is **DB first, then canon ALWAYS, then views IF MARKED** -- not "disk if marked", which collapses canon into views and would leave a dehydrated artefact inside a gitignored database, absent from a fresh clone. That collapse is the one this document opened by separating, and it is an attractor rather than a slip: the same wording produced it twice, from the same author, a day apart.

- **Create splits two ways.** Caller-assigned ids (`AC-10.11`, `AT-10.11`) are a `PUT` to the entity address. Server-assigned ids (threads, issues, WP seq) are a `POST` to the COLLECTION address, which returns the new address -- you cannot address `ST0058` before the tool has decided it is `ST0058`.
- **The mutation format IS the interchange format.** `GET ?format=json`, modify, `PUT` the same shape back. **This gives AC-02.6 a second job: a field that does not round-trip is now a field that cannot be WRITTEN**, so the lossless 1-1 mapping stops being only a durability guarantee at the clone boundary and becomes the completeness guarantee for the whole mutation surface.
- **`GET` accepts `json` and `md`. `PUT` accepts `json` only.** Writing markdown to an address would promote a stale rendering into canon, which 5.1b forbids. **The one exception is not an exception:** an ATTACHMENT is authored on disk, so for attachments the authority runs the other way and text-in is correct. Authorship decides direction, and `Project::classify` is the single answer to what a file is, so the asymmetry is implementable rather than aspirational.
- **If an entity has more than one rendering, it is UNDER-ADDRESSED.** `/threads/ST0056` returns the cover; `/threads/ST0056/ac` returns the acceptance view. One rendering per address, and no `?view=` stacked on `?format=`.
- **Hold the format set at exactly `json` and `md` for 3.0.0.** Two formats with a ratified meaning each beats four that drift. As built, `address::Format` has exactly those two variants (`address.rs:66`).

**As built, this contract lives in `intentsvcs` and no command is spelled GET, PUT or POST.** `Facade::put` (`facade.rs:7384`, which refuses `?format=md` for everything but an attachment) is the write half, and the caller-assigned create goes through it: `intent ac new` and `intent at new` build the row and `PUT` it to the entity address (`Facade::ac_new`, `Facade::at_new` at `facade.rs:5963`, `:6028`). `Facade::post` (`facade.rs:7234`) implements the server-assigned create and `address::serve_md` (`address.rs:778`) the markdown read; both are driven by the test suite and called by no face -- `intent st new`, `intent wp new` and `intent issues add` create through their own verbs. The other shipped faces that take an address are `intent set <ADDRESS> <FIELD>` (one field of an entity, through `Facade::set`), `intent edit` / `intent browse`, `intent explore`, `intent st hydrate` / `st dehydrate`, and the MCP tools, which are derived from the CLI surface.

**Resolution lives in `intentsvcs`, which already owns the DB and the files. The CLI calls it in-process; intentd calls THE SAME CODE** -- it parses with `intentsvcs::address::parse` and acts through the `Facade` on its store thread (`intentd/src/store.rs:657`), and its GraphQL executes through `Facade::graphql` (`graphql.rs:272`). Neither implements resolution. The failure mode to guard is intentd growing its own resolver because GraphQL wants different shapes -- two resolvers agreeing exactly until one moves, with nothing watching.

**And `?format=md` needs no new renderer.** `views::render_all` (`views.rs:1435`) returns `Vec<View { path, content }>` and is the one renderer behind what `sync --to-disk` writes, what `organize` and `st hydrate` hydrate, `doctor`'s skew check (`views::skew`), the migration, and the text realisation. `address::serve_md` selects from it rather than rendering, which makes **the served markdown byte-match the file `organize` would hydrate TRUE BY CONSTRUCTION rather than by test**, because `View.path` is literally where it would land. The dehydration gate compares the same render to the disk (`organize::gate`, `organize.rs:1073`), as `views::skew` does for `doctor`. One renderer, every job.

**REQUIREMENT, not a nicety: no daemon may be required to read your own project.** Because the CLI resolves in-process through the same `intentsvcs`, a fully dehydrated estate stays readable with intentd stopped, uninstalled or never started. **This is an acceptance criterion rather than an implementation accident** -- if reading a dehydrated thread came to need a running daemon, the disk model would have made the estate LESS accessible than a pile of markdown, which inverts D57-5's whole reason for existing. The risk is not that intentd serves content; it is that intentd becomes the only thing that does.

## D57-9 -- The marker grammar goes; `.intentfiles` is one flat list

**hv ruling, 2026-08-20.** This is the question hv deliberately kept OUT of ruling 4 rather than an afterthought to it: ruling 4 retired the two-region API, and whether the FORMAT keeps its markers was left standing on its own.

**`# BEGIN INTENT` and `# END INTENT` go, with `Region`, `Manifest::generated()` and the region errors.** `intentfiles::render` and `Generated` were already deleted (ic, `c58e8bbb`); what survived did so **only because `pin` happened to use it**, which left the markers delimiting a region that **nothing regenerates**.

**THE FORMAT IS TO SAY WHAT D57-2 ALREADY SAID IT WAS.** `.intentfiles` is durable state: a flat list of artefact ids, many writers, no recomputation. A generated region was the two-region design's mechanism, and that design is the one hv replaced -- **so keeping its delimiters is not a forward declaration, it is a second design still visible in the file a user opens.**

**AND THE COST OF KEEPING THEM IS NOT ZERO, WHICH IS WHY IT WAS WORTH ASKING.** A parser nobody exercises is a parser nobody can trust: `UnopenedRegion`, `NestedRegion` and `UnclosedRegion` are three refusal paths over a construct no writer emits, so their first live exercise would be someone hand-writing a marker into a file that no longer has a region for it to mean anything.

**AS BUILT, THIS RULING IS NOT YET CARRIED OUT, AND NO CRITERION TRACKS IT.** `BEGIN_MARKER` / `END_MARKER` (`intentfiles.rs:52`), `Region` (`:135`), `Manifest::pinned()` / `generated()` (`:164`, `:169`) and the three region errors (`:315`-`:319`) are all still in the module, whose own doc comment still describes two regions. No writer emits a marker -- `default_declaration` writes none, and `pin` only places its line above a `# BEGIN INTENT` that is already there -- but the parser still reads one: a `# BEGIN INTENT` / `# END INTENT` pair is accepted with the lines between them declared like any other, and an unpaired marker aborts the run with its line number. So the first live exercise this paragraph predicts is available today.

## What had to be true before ANY dehydration shipped

**Dehydration has shipped** -- `intent organize --apply`, `intent st dehydrate`, and the closing lifecycle verbs delisting a thread for the next `organize` to remove -- **behind two gates.** The per-file gate is D57-3's (`organize::gate`). The ship gate (`preconditions::check`, `preconditions.rs:336`) resolves every criterion named in the `<<PRECONDITIONS ... PRECONDITIONS>>` block of AC-00.1 in ST0057's canon, which is the one machine-readable declaration of the list below, and refuses every removal while any of them is unmet -- a descoped, withdrawn or fiat-closed criterion counts as unmet. In a project that declares no preconditions, which is every project but Intent, it permits, and the per-file gate is what protects the bytes. `intent ac list ST0057` reports each criterion's state, and a refusing run prints the gate's checked-of-declared line.

- **Attachments** -- landed `36bc02c5`. `THREAD_PROSE` deleted (D57-6, AC-00.2).
- **Issue bodies** -- landed, carried verbatim rather than trimmed, so no renderer has to remember to put a byte back.
- **`ROOT_FILES`** -- `AGENTS.md`, `CLAUDE.md` and `usage-rules.md` have a v3 generator (`rootfiles.rs`, AC-00.4). They remain outside `.intentfiles` scope, and outside `organize`'s reach, because they are not under `intent/st`.
- **The conservation verdict** -- **WITHDRAWN, not met** (AC-00.3, hv 2026-08-19): git already holds every file dehydration removes, so the verdict would prove the absence of a loss that cannot occur. It is not in the ship gate's declaration.
- **The full text realisation (D57-5)** -- built: `intent export --format md` (AC-06.1, AC-06.2).
- **Opaque-attachment canon (D57-7)** -- canon holds an opaque attachment's bytes as a file (AC-03.1). The working copy is not yet regenerated from them or removed, so a thread carrying one does not fully dehydrate (D57-7).
- **Addressing (D57-8)** -- built (AC-07.1 to AC-07.7). Tracked prose that cites artefact FILES has an `intent://` address to migrate to.

**The build order was: `organize` and `edit` first, dehydration behind the gate.** Hydration and realisation were additive and safe immediately.
