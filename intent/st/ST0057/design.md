---
verblock: "18 Aug 2026:v0.1: vc - disk as a sparse projection of the store"
---

# ST0057 -- Disk as a sparse projection of the store

## Objective

Make disk optional without making anything unrecoverable.

Today `intent/st` holds 233 files per the census and 1220 in total, and every one of them is on disk because that is where v2 kept truth. Under D01-as-reversed the store is truth and the files are a projection -- **but a projection nobody can regenerate is not a projection, it is the original with extra steps.** This thread ends with three things true at once: `intent/st` holds little more than an index and the live work; anything not on disk is provably reproducible; and a human with an editor and no working tool can still read the whole project.

## The three-layer model, because the brief collapses two of them

hv's framing was _"the DB is SSOT and disk is a sparse copy of the db out to realised files based on an index."_ That is right for two of the three things on disk and fatal for the third.

| layer     | what                                           | committed            | who derives it              |
| --------- | ---------------------------------------------- | -------------------- | --------------------------- |
| **canon** | `thread.json`, `issues/NNNN.json`              | **yes**              | authored through the facade |
| **store** | `intent/.cache/intent.db`                      | **NO -- gitignored** | rebuilt from canon          |
| **views** | `info.md`, `acceptance.md`, `steel_threads.md` | yes                  | rendered from the store     |

**The store is SSOT for a running tool on one machine. It is not what travels.** `intent/.cache/` is gitignored at `.gitignore:127` and the ignore rule's own comment says the name contradicts the model. `intent sync --to-disk` is documented as writing _"the committed extract"_ -- the vocabulary already distinguishes the runtime store from the durable record.

**So sparseness applies to VIEWS. Canon is never sparse.** If the manifest governed canon, an unrealised artefact would exist only inside a gitignored database: absent from a fresh clone, and destroyed by `rm -rf intent/.cache`. **D29 -- a gitignored path is never canon -- is not a style rule here, it is what makes a clone complete.**

That single ruling is what makes every deletion in this design safe rather than careful: **`organize` only ever removes a file it can regenerate, from a source sitting beside it in the same commit.**

### Measured, because the sizes settle the argument

```
canon      thread.json x56 + issues/NNNN.json x40    1,602,351 bytes   ALWAYS on disk
views      *.md under intent/st                      2,704,783 bytes   sparse
  of which ST0056, the ONE live thread                1,021,934        38% of all markdown
  ST0011, a completed one                                 7,199
```

**The single live thread is 38% of all rendered markdown.** Sparse views buy nearly everything; complete canon costs 1.6 MB, which is the price of a clone that works.

## D57-1 -- Canon relocates, and stays one file per artefact

Keeping canon complete does not mean keeping it in the way. hv's requirement is that `intent/st` stop holding a bajillion files, and canon-in-place does not deliver that: 56 directories remain whatever happens to the markdown.

- **A -- canon in place.** Bytes fall ~60%; `intent/st` still lists 56 entries. **Does not meet the requirement.**
- **B -- one consolidated `threads.jsonl`.** **Rejected: four concurrent writers make a single canon file a merge-conflict generator**, and this estate has four nodes plus, until today, a test suite.
- **C -- `intent/.canon/st/ST0011.json`, still per-artefact. RULED.**

C is the only one that empties `intent/st`, keeps a clone complete, and keeps a per-artefact diff so two nodes editing two threads never conflict. It is a path change, not a model change.

**After C, `intent/st` holds `steel_threads.md` and the realised threads. Nothing else.**

## D57-2 -- `.intentfiles` is durable state

**SUPERSEDED 2026-08-19. This decision used to read _`.intentfiles`, and why it has two regions_, and hv replaced it outright mid-implementation.** The two-region text is kept below the rule so a reader meeting the retired shape in an old commit can tell which one they are looking at.

**THE RULE, hv's own words:**

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

The manifest is committed, and it has to be: the set it governs is committed. **A machine-local manifest would let `organize` on one machine delete files another machine depends on, and land the deletion in a commit with no declared reason.** Committed, hydration and dehydration are reviewable -- the diff shows the manifest line changing beside the files appearing or disappearing. **That half of the original decision survives unchanged.**

```
# .intentfiles -- which database artefacts also have a realised form on disk.
# Realisation is driven from this file. Commands change it; `organize` realises
# it. Nothing recomputes it from status.

STEELTHREAD:ST0046
STEELTHREAD:ST0056
STEELTHREAD:ST0057
```

**MANY WRITERS, NO RECOMPUTATION.** `st new` adds the id, `st done` and `st cancel` remove it, `st hydrate` / `st dehydrate` and the ISSUE equivalents do it directly, and a human may edit it by hand. All of those are ordinary writers and none is privileged. **_AUTHORED_ WAS vc's WORD FOR IT AND IT WAS WRONG** -- hv corrected it before either builder committed to a shape. It does not mean untouched by commands; it means nothing recomputes it.

**THE REGIONS ARE GONE BECAUSE THE REGENERATION IS GONE.** hv's question settles it: _why isn't the organise operation simply: a) look at .intentfiles, b) hydrate the items in the file, c) dehydrate any previously hydrated items that are no longer in the file._ The two regions existed only because the file was machine-generated -- if `organize` rewrote the list from status every run, a hand-added line would be wiped, so a protected region was needed. **Remove the regeneration and the protected region has nothing to protect against.** A write is a CHANGE TO STATE, never a REGENERATION of it.

**IT ALSO EXPLAINS A LOOSE END NOBODY HAD ACCOUNTED FOR: `intentfiles::render` had ZERO production callers.** That looked like an unwired writer and was not -- the thing it does is not needed. Nobody failed to wire it.

**ABSENT IS NOT EMPTY, AND THE DIFFERENCE IS 423 FILES.** A missing manifest means nobody has said, so everything stays; a manifest present and declaring nothing means keep nothing. This estate carried a 26-line file of pure comment for a day, which is the second state, so 545 files sat on the removal path **by omission rather than by decision** -- held back only by the dehydration gate. Populating the list was what made the removal intentional.

**THE GRAMMAR REFUSES RATHER THAN SKIPS.** `<SIGIL>:<ID>`, sigil in `STEELTHREAD | ISSUE`, optional trailing comment, nothing else. A line the parser cannot read aborts the run with its line number. **A skipped line drops an artefact from realisation and leaves an estate indistinguishable from one that never listed it** -- the silent-drop shape v2.19.0 already paid for twice, in `ac gate`'s F1 fix and in the AT row grammar's `at lint`. **This half survives unchanged too.**

### The retired two-region shape, for readers of older commits

```
# Lines between BEGIN and END are GENERATED by `intent organize` from status.
# Lines outside the markers are PINS and are never rewritten or removed.

STEELTHREAD:ST0011        # pinned by hand: reading the old test-suite thread

<!-- STEEL THREADS: BEGIN -->
STEELTHREAD:ST0057
<!-- STEEL THREADS: END -->
```

**The pin bought exactly one property: a closed artefact staying on disk.** Under the rule above that is ordinary behaviour rather than an exception -- **the list wins over status, both directions, and status has no vote at `organize` time at all.** Measured on this estate at `e7f00e65` across four statuses: 52 `completed` and 2 `cancelled` removed while unlisted, 1 `not-started` and 2 `wip` kept while listed, and ST0010 (`cancelled`) hydrated and kept while listed.

## D57-3 -- `intent organize`, four answers and one refusal

Existing verb, currently v2's status-directory tidier, which under the flat layout has no work left. Inherit the name rather than invent one.

| declared | on disk                              | action                                                            |
| -------- | ------------------------------------ | ----------------------------------------------------------------- |
| yes      | absent                               | **HYDRATE**                                                       |
| yes      | present                              | **VERIFY** -- re-render in memory, write only if the bytes differ |
| no       | absent                               | nothing                                                           |
| no       | present                              | **DEHYDRATE**, subject to the gate below                          |
| --       | a path the renderer does not produce | **UNCLAIMED -- report, never remove**                             |

**The fifth row is the one that matters.** A file the renderer cannot make is either an attachment or something a human put there. **`organize` must not be the thing that decides an unrecognised file is rubbish.**

### The dehydration gate

Before removing any view: re-render it from the store into memory, compare to the bytes on disk, **refuse on any difference and report the path.** A difference means the disk holds something the store does not, and that something is destroyed by the removal. One render and one comparison per file -- **fail-safe by construction rather than by discipline**, and it is the same view-skew comparison `doctor` already performs.

### An ATTACHMENT is not a VIEW, and one policy for both discards work

**Authority follows AUTHORSHIP.** A view is authored in the model, so disk divergence means the file is stale. **An attachment is authored ON DISK, so divergence means the STORE is stale.** Overwriting an attachment destroys the author's edit; ingesting a view promotes a stale generated file into canon. Same divergence, opposite remedies.

**`organize` NEVER resolves an attachment divergence.** It reports the path and names both verbs. _"Decide which way to sync"_ is a human decision by definition, and **a tool that decides it silently is choosing whose work to discard -- right most of the time and catastrophically wrong occasionally, which is the worst profile available.** `Project::classify` is the single answer to what a file is, so the asymmetry is implementable rather than aspirational.

### Idempotence is a measured requirement

`organize` run twice changes nothing, **including mtimes.** Measured 2026-08-18: the current render re-emits 255 of 1000 `.md` files byte-identically every pass, moving their mtime with zero content change. Harmless for conservation, not harmless for `file_index`, whose `clean`/`changed` state is computed from exactly that. **Write only on content difference.**

### It locks against ANY process, not other `organize` runs

`info`, `st list`, `doctor` and `export` all **materialise the store on access** -- a fresh clone's first read verb builds it. So a peer typing `intent st list` opens the same file `organize` is reconciling against. **"The estate was quiet when I measured" is not establishable over any window**, which makes the moment-of-act digest the only defence that survives a concurrent read: digest the measured tree, re-compute immediately before the irreversible step, refuse on any difference.

## D57-4 -- `intent edit <ID>`, and why not `wip`

hv proposed `intent wip <ID>`. **In this project `wip` already means a WP status, a thread status, `intent/wip.md`, and every node's board.** A fifth meaning collides with four live ones.

**`intent edit <ID>`**, dispatching on id shape: hydrate if needed, write the pinned region, print the path. `intent st edit` already prints the path to a thread file, so this is that verb learning to hydrate first -- one behaviour, one home.

**A hand realisation writes to the PINNED region.** Otherwise the next `organize` reverts it, because the generated region is a function of status and the artefact you opened is closed.

## D57-5 -- The full text realisation, and a refusal whose reason expires

**hv, 2026-08-18:** _"I would expect to be able to trivially generate a full text realisation of all of the files for the entire project into, say, `.backup/...` so that I have that on-disk fallback."_

This is **not** `organize`. `organize` decides which artefacts have _working_ form under `intent/st`. This is a complete, disposable, regenerable text snapshot **elsewhere**, and it is the assurance that makes sparseness acceptable to a person rather than merely to a tool.

**The two assurances are different and the design needs both.** The dehydration gate proves **the store holds it**. A complete text export proves **a human can get it back without the tool.** hv is asking for the second, and nothing in the design provided it.

**`export --format md` ALREADY EXISTS AND IS DELIBERATELY REFUSED:**

```
export.rs:192  "design.md:57 names `md` as an export projection and it is not one"
export.rs:208  "The views are already in the tree -- `intent sync --to-disk` rewrites
                them; for data a program will read, use `--format json`"
```

**That refusal is correct today and becomes false the moment this thread lands.** Its justification is _the views are already in the tree_, and the entire purpose of D57-1 through D57-3 is that they are not. **A claim that was true when written and expires on a change already scheduled** -- the same class as `Issue::body`'s trim, caught here before it ships rather than after.

**RULED: `--format md` is withdrawn from the refused set as part of this thread, not before.** The refusal stays honest while its premise holds.

Requirements on the realisation:

- **Complete, and it says so with a denominator.** Every thread, WP, issue, attachment and view, with a printed count against the canon totals. **A partial export that reads as complete is worse than no export.**
- **Out of the way and gitignored.** `.backup/text/<UTC>/` -- `.backup/` is already the local-never-commit namespace holding `upgrade/` and `db/`, so this is a third mechanism in an established home rather than a new convention.
- **Regenerable, never authoritative.** It is a fallback. Nothing reads it back; there is no import path; `classify` never sees it.
- **Cheap enough to be habitual.** If it is expensive nobody runs it, and an assurance nobody exercises is not one.

**AND IT HAS A DEPENDENCY NOBODY HAD NAMED (cc, 2026-08-18, found while bootstrapping a scratch project to drive something else): `intent init` IS NOT IMPLEMENTED.** `error: 'init' is a known command that is not implemented yet`. A v3 project has to be hand-bootstrapped with a v3-stamped `config.json`.

**That is a precondition of D57-5 rather than a neighbouring gap.** The whole value of a fallback is that someone can exercise it -- and the natural way to exercise "can I get everything back into text" is _make an empty project, export into it, read it_. **You cannot demonstrate a fallback from a clean directory if you cannot create a clean directory**, so `init` gates the assurance, not merely the onboarding. cc's line is the one to keep: _a text-export assurance nobody can exercise from a clean directory is exactly the assurance nobody exercises._

## D57-6 -- The 165, RULED

**`design.md`, `impl.md` and `tasks.md` are carried as ATTACHMENTS. `THREAD_PROSE` is deleted from the classifier.**

The inventory this was decided against -- every `.md` under `intent/st` the census never enumerates:

```
canonical three (design/impl/tasks)   165 files    702,880 bytes   580 sections
one-off .md under a thread             68 files    809,832 bytes   323 sections
                                     ----------------------------------------
TOTAL never enumerated as prose        233 files  1,512,712 bytes   903 sections
```

**The 68 one-offs were already carried** -- `.md` is in the declared extension set and `classify` does not call them typed. **So the ruling governs exactly the 165, and the change is subtractive: three strings come out of a constant.**

Two arguments carried it. **Rule 2 of the attachment spec:** a typed field earns its parsing because the model has fields for what comes out. These are freeform prose under arbitrary headings and the model has none, **so parsing them would discard structure into nothing -- which is precisely how `## Related Steel Threads` became 52 rows of `LOST-PROSE`.** And **it is about ongoing behaviour, not legacy data**: `st new` does not create these files, but `THREAD_PROSE` still names them, so the moment anyone hand-writes a `design.md` -- which the v2 habit, the templates, and all 55 existing threads teach -- **it is indexed for search and carried by nothing. The 166th has the same problem as the 165.**

**This document is the demonstration.** Written as `design.md` it was, until this ruling, the one filename in the estate that would not be carried; written as anything else it would have been. **The tool's own default name for a design document was the only name it would not keep.**

## D57-7 -- Where ancillary files live, and the one directory they cannot

Text attachments need no home: they live inline in canon as `Attachment.text`, and the working copy dehydrates. **Everything else does need one**, and the estate is mostly everything else.

```
under intent/st           787 files
  md                      490      (of which the generated views dehydrate today)
  tap                     196      ST0056/parity/tools/tap-baseline, ONE directory
  json                     57
  sh                       39
  txt / tsv                 4
                        ------
  NEITHER md NOR json     240
```

**REJECTED -- `intent/.cache/`.** It is gitignored at `.gitignore:127`, holds exactly one file (`intent.db`, 5.7 MB), and **0 files under it are tracked**. The ignore rule's own comment records what it is for: the database was showing as `?? intent/.cache/`, _"one `git add -A` from entering history as a binary blob"_, and the line **is the precondition for D29** -- the ingest corpus excludes ignored paths, so until it existed the DB was outside the corpus only by accident of path shape. Committing that directory reverses both protections at once.

**And the name is the sharper objection, because it fails in the dangerous direction.** `cache` means regenerable-and-discardable. An opaque attachment is precisely what is NOT regenerable. `rm -rf intent/.cache` is a reasonable thing to do to a cache, and under that layout it would destroy the only copy of a hand-authored file. The ignore rule already concedes _"`intent/.cache/` remains a name that contradicts the model"_; adding committed content compounds a documented defect rather than repairing one.

**REJECTED -- opaque attachments never dehydrate.** This was the first answer and it was wrong, and **the error was reading D57-3's "regenerate" as "re-render from JSON"** -- the rule says _a source sitting beside it in the same commit_, and it does not say the source must be JSON. That is the whole reason, and it stands on its own. **THE COST FIGURE THIS PARAGRAPH USED TO CARRY WAS WRONG TWICE AND IS WITHDRAWN RATHER THAN QUIETLY DELETED (cc measured it, vc ruled, 2026-08-19).** It read _"pins all 240 files to disk permanently -- 196 `.tap` baselines surviving ST0056 forever"_. **Wrong population: all 196 are valid UTF-8**, so under AC-03.2's own words -- form follows CONTENT -- a `.tap` is a TEXT attachment, and the sidecar mechanism this paragraph rejects cannot reach them under any reading. What actually holds them on disk is `ATTACHMENT_EXTENSIONS` (`md`, `txt`, `sh`): they are not carried at all, so `organize` reports them UNCLAIMED and never removes them. **Wrong cost: those files are pinned to disk by GIT regardless**, because they are committed and tracked. The alternative was never _not on disk_; it was _on disk AND in the store_. **An argument that rejects an option by charging it a cost both options pay is not an argument**, and the rejection was carried by the D57-3 misreading alone the entire time. **The `.tap` baselines' fate is a CARRY-LIST question and belongs to no criterion in WP-03, whose population is _not valid UTF-8_ -- see `project.rs:33`, whose conclusion stands and whose stated reason does not: _"a tool's committed output is regenerable, so carrying it buys nothing"_ is true of the mechanism and false of the meaning, since a baseline exists to be the recorded PAST and regenerating one destroys the comparison it exists to make.**

**RULED. Canon holds the truth; the working copy dehydrates like everything else.**

```
intent/.canon/st/ST0056.json          text attachments, inline as `text`
intent/.canon/st/ST0056/<path>        opaque attachments, as FILES
intent/st/ST0056/<path>               realised working copy -- DEHYDRATES
intent/.cache/                        gitignored, store only, never committed
```

**One rule, uniformly: everything under `intent/st/` is a realised working copy, and everything dehydrates.** Canon holds truth in whichever form suits it -- inline for text, as a sibling file for bytes. Regeneration of an opaque attachment is a byte copy from a source in the same commit, which satisfies D57-3 exactly rather than bending it.

Three things fall out, and each removes work rather than adding it:

- **No `.intentfiles` grammar change.** Attachments are per-thread, so `STEELTHREAD:ST0056` hydrates the thread AND its files. The manifest still names threads only.
- **`sha256` in canon buys drift detection on the working copy** -- an attachment edited on disk without the store knowing is 5.1b's divergence case, and `doctor` can now report it by comparison rather than by inference.
- **The naming rule protects two things with one check.** A file that cannot be named safely can be given neither a canon path nor a URL (D57-8), so rejection at the gate covers storage and addressing together. Rejection is NOT retroactive: existing violators -- the estate already carries a `.webloc` with spaces in its name, which split into eight fragments under `xargs` while the run still printed a plausible total -- are reported by `organize` as UNCLAIMED under its fifth row and never silently removed.

**OPEN, and it is an optimisation rather than a design question:** copy versus hardlink on hydration. Copy is simpler and doubles bytes for hydrated threads only, which for 196 tap files is nothing.

## D57-8 -- `intent://`, the address of a piece of data

**Hydration makes a file path a statement about a moment.** `intent/st/ST0034/design.md` either exists or does not depending on what `organize` last did, so every reference to it is conditional. **Measured at `ce532a97`: 80 citations of `intent/st/ST####/<file>.md` in tracked estate prose, the most-cited single artefact being `ST0034/design.md` at 23 -- a COMPLETED thread, and therefore among the first to dehydrate.** Three AT rows also point inside `intent/`; the other 87 point at `native/rust` code and are unaffected.

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

**Empty authority means THIS project.** Nearly every reference is intra-project, and one that hard-codes the project name breaks on rename or fork. Cross-project references carry the slug and resolve against intentd's project registry.

**VIEWS GET NO URL, and this is what stops the scheme becoming a path alias.** A view is derivable from its entity, so a reference to a view is a reference to its source. Giving views addresses would re-create, inside the scheme, the exact conditionality the scheme exists to remove. **`?format=` selects a REPRESENTATION of the addressed entity; a path segment would name a separate thing.**

**READ/WRITE (hv, 2026-08-18). The URI is the address of a piece of data, and the data is mutable.** The write path is **DB first, then canon ALWAYS, then views IF MARKED** -- not "disk if marked", which collapses canon into views and would leave a dehydrated artefact inside a gitignored database, absent from a fresh clone. That collapse is the one this document opened by separating, and it is an attractor rather than a slip: the same wording produced it twice, from the same author, a day apart.

- **Create splits two ways.** Caller-assigned ids (`AC-10.11`, `AT-10.11`) are a `PUT` to the entity address. Server-assigned ids (threads, issues, WP seq) are a `POST` to the COLLECTION address, which returns the new address -- you cannot address `ST0058` before the tool has decided it is `ST0058`.
- **The mutation format IS the interchange format.** `GET ?format=json`, modify, `PUT` the same shape back. **This gives AC-02.6 a second job: a field that does not round-trip is now a field that cannot be WRITTEN**, so the lossless 1-1 mapping stops being only a durability guarantee at the clone boundary and becomes the completeness guarantee for the whole mutation surface.
- **`GET` accepts `json` and `md`. `PUT` accepts `json` only.** Writing markdown to an address would promote a stale rendering into canon, which 5.1b forbids. **The one exception is not an exception:** an ATTACHMENT is authored on disk, so for attachments the authority runs the other way and text-in is correct. Authorship decides direction, and `Project::classify` is the single answer to what a file is, so the asymmetry is implementable rather than aspirational.
- **If an entity has more than one rendering, it is UNDER-ADDRESSED.** `/threads/ST0056` returns the cover; `/threads/ST0056/ac` returns the acceptance view. One rendering per address, and no `?view=` stacked on `?format=`.
- **Hold the format set at exactly `json` and `md` for 3.0.0.** Two formats with a ratified meaning each beats four that drift.

**Resolution lives in `intentsvcs`, which already owns the DB and the files. The CLI calls it in-process; intentd calls THE SAME FUNCTION and serves it over GraphQL.** Neither implements resolution. The failure mode to guard is intentd growing its own resolver because GraphQL wants different shapes -- two resolvers agreeing exactly until one moves, with nothing watching.

**And `?format=md` needs no new renderer.** `views::render_all` already returns `Vec<View { path, content }>` and already has three callers -- `facade.rs:1236` (what `sync` writes), `doctor.rs:737` (skew), `migrate.rs:293`. Serving `View.content` makes **the served markdown byte-match the file `organize` would hydrate TRUE BY CONSTRUCTION rather than by test**, because `View.path` is literally where it would land. `views::skew()` is already the dehydration gate's render-and-compare. One renderer, three jobs.

**REQUIREMENT, not a nicety: no daemon may be required to read your own project.** Because the CLI resolves in-process through the same `intentsvcs`, a fully dehydrated estate stays readable with intentd stopped, uninstalled or never started. **This is an acceptance criterion rather than an implementation accident** -- if reading a dehydrated thread came to need a running daemon, the disk model would have made the estate LESS accessible than a pile of markdown, which inverts D57-5's whole reason for existing. The risk is not that intentd serves content; it is that intentd becomes the only thing that does.

## What must be true before ANY dehydration ships

- **Attachments** -- landed `36bc02c5`. **`THREAD_PROSE` deletion outstanding (D57-6).**
- **Issue bodies** -- landed, carried verbatim rather than trimmed, so no renderer has to remember to put a byte back.
- **`ROOT_FILES`** -- `AGENTS.md`, `CLAUDE.md`, `usage-rules.md` have no v3 generator. Out of `.intentfiles` scope, but they are on disk and something emptied `AGENTS.md` on 2026-08-18, so they need their generator before anyone reasons about them as derivable.
- **The conservation verdict** -- green at full scope over a pinned subject, per `conservation_check.sh`'s header. **Currently `STRANDED 192` and that is the number the deletion rests on.**
- **The full text realisation (D57-5)** -- because the human fallback is a precondition of sparseness, not a nicety that follows it.
- **Opaque-attachment canon (D57-7)** -- an opaque attachment cannot be dehydrated until canon holds its bytes as a file, because until then there is no source beside it in the commit and the removal is unrecoverable rather than merely careless.
- **Addressing (D57-8)** -- 80 tracked citations name artefact FILES that dehydration makes conditional. Shipping sparseness before addressing breaks them with no replacement to migrate to.

**Build `organize` and `edit` now; ship dehydration behind the gate.** Hydration and realisation are additive and safe immediately.
