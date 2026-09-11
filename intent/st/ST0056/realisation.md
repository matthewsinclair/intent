---
verblock: "18 Aug 2026:v0.1: vc - selective realisation: .intentfiles, organize, edit"
---

# Selective realisation -- disk as a sparse projection of the store

hv's brief, 2026-08-18: _"The DB is ssot and disk is a sparse copy of the db out to realised files based on an index."_ A manifest at `intent/.intentfiles` names the artefacts that have on-disk form; `intent organize` reconciles disk to the manifest; a per-artefact verb realises one thing on demand so it can be edited.

This document takes that brief, names the one thing in it that cannot be true as written, and specifies the version that can. ST0057 built it; where the build departs from the proposal below, the section says what was built.

## 1. The model is THREE layers, and the brief collapses two of them

| layer     | what                                                                                     | where                                                                    | committed            | who derives it              |
| --------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | -------------------- | --------------------------- |
| **canon** | one JSON file per thread (carrying its attachments) and per issue                        | `intent/.canon/st/<ID>.json`, `intent/.canon/issues/<NNNN>.json`         | **yes**              | authored through the facade |
| **store** | `intent/.cache/intent.db`                                                                | machine-local                                                            | **NO -- gitignored** | rebuilt from canon          |
| **disk**  | a realised thread's views (`info.md`, `acceptance.md`, WP `info.md`) and its attachments | `intent/st/<ID>/`, plus the index views `steel_threads.md` and `todo.md` | yes                  | written from the store      |

**The store is SSOT for a running tool on one machine. It is not what travels.** `intent/.cache/` is gitignored at `.gitignore:125`, and the ignore rule's own comment says the name contradicts the model. `intent sync --to-disk` is documented as _"Write the store out to **the committed extract**"_ -- the tool's own vocabulary already distinguishes the runtime store from the durable record.

**So "disk is a sparse copy of the DB" is safe for the realised files and unsafe for CANON.** If `.intentfiles` governed canon as well, an unrealised artefact would exist only inside a gitignored database. A fresh clone would not contain it. **D29 -- a gitignored path is never canon -- is not a style rule here; it is the thing that makes a clone complete.**

**Ruling, as built: `.intentfiles` governs a thread's realised files -- its views and its attachments. Canon is never sparse.**

That is what makes every deletion in this design safe rather than merely careful: **`organize` only ever removes a file the store can put back byte for byte, from canon sitting beside it in the same commit.**

## 2. Measured: sparse views buy nearly everything, and canon costs nearly nothing

Measured on Intent's own estate on 2026-08-18: canon was smaller than the markdown rendered under `intent/st`, and the one live thread accounted for a large share of those rendered bytes. Realising the live set and dehydrating the rest removes most of the view bytes and keeps the thing anyone is actually working on. Keeping canon whole is the price of a complete clone and is not a price worth arguing about.

## 3. WHERE canon lives

Keeping canon complete does not require keeping it in the way. Three arrangements were weighed, and the difference is what `ls intent/st` shows:

- **A -- canon in place.** `intent/st/<ID>/thread.json` for every thread. Bytes drop, but **the directory still has an entry per thread**, so hv's _"little more than an index file"_ is not achieved.
- **B -- canon consolidated.** One committed `threads.jsonl`. **Rejected: concurrent writers to one file are a merge-conflict generator.**
- **C -- canon relocated, still per-artefact.** `intent/.canon/st/<ID>.json`, `intent/.canon/issues/<NNNN>.json`. `intent/st/` holds `steel_threads.md` plus only realised threads.

**Built: C.** It is the only one that gives hv the directory they asked for, keeps a clone complete, and keeps a per-artefact diff so two nodes editing two threads never conflict. It is a path change, not a model change.

## 4. `.intentfiles`

### 4.1 It is committed, and it has to be

The set it governs is committed. If the manifest were machine-local, `organize` on my machine would delete files from the repository that yours depends on, and the deletion would land in a commit with no declared reason. **A committed manifest makes hydration and dehydration reviewable: the diff shows the manifest line changing beside the files appearing or disappearing.** Intent's own `intent/.intentfiles` is tracked.

### 4.2 One flat list, changed by commands

**Built as one flat list (hv, 2026-08-19), not the proposed pair of a GENERATED region rewritten from status and a PINNED region the tool never touched.** Realisation is driven FROM the file, and nothing recomputes the file from status. A write to it is a change to state, never a regeneration of it, so a human's line cannot be silently deleted by a status change and no protected region is needed.

Many writers, one meaning:

- `st start`, `st resume` and `st reopen` add the thread's id; `st done`, `st cancel`, `st hold` and `st triage` remove it; `st new` and `st reinstate` do neither. The op decides, not the status it lands on.
- `st hydrate <ID>` and `st dehydrate <ID>` add or remove it directly and write or delete the files.
- `intent edit` adds the id of a thread it realises.
- A human may edit the file by hand.
- `intent organize --default` writes the file from status -- one `STEELTHREAD:` line per WIP thread, nothing else -- when it is absent, and `--force` regenerates an existing one (refused without a tty; confirmed on one). `intent init` writes the header with no declarations, and `intent upgrade` writes the default when the file is absent.

**ABSENT is not EMPTY.** A missing file means nobody has said, and everything stays; a present file declaring nothing means keep nothing.

```
# .intentfiles -- WHICH DATABASE ARTEFACTS ALSO HAVE A REALISED FORM ON DISK.
# ...
STEELTHREAD:ST0056
STEELTHREAD:ST0057
```

### 4.3 The grammar is enforced, and a malformed line REFUSES

`<SIGIL>:<ID>`, where `STEELTHREAD` is the only sigil and `ID` must have a steel-thread id's shape, with an optional trailing `# comment`. Nothing else. **`ISSUE:` was in the proposed grammar and hv retired it on 2026-08-20**: issues live in canon and the store only and have no realised form, so there is nothing for a manifest line to declare.

**A line the parser cannot read aborts the run, naming the line number.** Not skip it. A skipped line silently drops an artefact from realisation, and the estate looks exactly like a correct one that never listed it. **This project has paid for that shape twice already** -- v2.19.0's `ac gate` F1 fix turned malformed AC/AT lines from a silent drop into a block precisely because a silent drop produced a vacuous green, and the AT row grammar with `at lint` L1-L5 exists for the same reason. Driven at v3.0.1: an `ISSUE:0001` line refuses with ``line <n>: `ISSUE` is not a known sigil -- expected STEELTHREAD`` and exit 1.

## 5. `intent organize` -- a reconciler with a fixed set of answers per path

```
intent organize [--apply] [--default [--force]] [-v | --verbose] [-q | --quiet]
```

**Previews by default and touches nothing; `--apply` performs the reconciliation.** v2's `organize` was a status-directory tidier; under the flat layout that job disappeared, and hv reclaimed the name for this verb on 2026-08-19.

For every realisable path, compare **declared** (in the manifest) against **actual** (on disk):

| declared | actual  | action                                                                                    |
| -------- | ------- | ----------------------------------------------------------------------------------------- |
| yes      | absent  | **HYDRATE** -- render the view, or write the attachment from the store's copy             |
| yes      | present | **VERIFY** -- re-render in memory; write only if the bytes differ (`rewrite`/`unchanged`) |
| no       | absent  | nothing                                                                                   |
| no       | present | **DEHYDRATE -- but only if it passes the gates in 5.1**; emptied directories are pruned   |

Plus the rows that are not about a declared artefact:

| path                                                                     | action                                |
| ------------------------------------------------------------------------ | ------------------------------------- |
| a file under `intent/st/` that the store neither renders nor carries     | **UNCLAIMED -- report, never remove** |
| an index view no manifest line can imply (`steel_threads.md`, `todo.md`) | **EXEMPT -- kept, always**            |

**A file the renderer does not know how to make is either an attachment the store has not taken in or something a human put there, and both are content. `organize` must not be the thing that decides an unrecognised file is rubbish.**

### 5.1 The dehydration gates -- the only irreversible act in the design

Before removing any file, per file:

1. **Re-render it from the store into memory** (or take the store's copy, for an attachment).
2. **Compare to the bytes on disk.** Identical -> safe, the file is a pure projection. **Different -> REFUSE and report the path.** A difference means the disk holds something the store does not, and that something is destroyed by the removal.
3. Never remove a path the store neither renders nor carries (rule above).

This is a per-file, in-process check costing one render and one comparison. **It makes `organize` fail-safe by construction rather than by discipline**, and it is the same view-skew comparison the doctor already performs -- one mechanism, two callers. `st dehydrate` stands behind the same check: driven at v3.0.1, dehydrating a thread whose `info.md` carried a hand edit refused, removed nothing, and named `intent doctor` and `intent sync --to-store` in its remedy.

**A second, estate-level gate stands in front of it** (`preconditions.rs`): once per run, removals are permitted only if the project's declared dehydration preconditions are met. The declaration is the delimited `<<PRECONDITIONS ... PRECONDITIONS>>` block in ST0057's `AC-00.1`; a project that declares none is told so, and the per-file gate still applies to every removal.

### 5.1b An ATTACHMENT is not a VIEW, and one policy for both is wrong (cc, 2026-08-18)

**5.1 as first written treated every file under a thread the same way, and that was a hole cc found before it was built.** The gate re-renders and refuses on difference, which is right for a generated view and **backwards for an attachment**.

**The direction of authority follows the direction of authorship.**

|                                           | authored in | disk divergence means                                                                | correct move                                                                                                               |
| ----------------------------------------- | ----------- | ------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| **view** (`info.md`, `acceptance.md`)     | the model   | the file is stale, or someone hand-edited a generated file the canon forbids editing | regenerate -- **but only once the model demonstrably holds everything the file holds**, which is what 5.1's refusal proves |
| **attachment** (`design.md`, `parity/**`) | **on disk** | the STORE is stale                                                                   | **ingest**, never overwrite                                                                                                |

**Overwriting an attachment from the store destroys the author's edit. Ingesting a view from disk promotes a stale generated file into canon.** Same divergence, opposite remedies, and a tool that guesses gets it right most of the time and catastrophically wrong occasionally -- the worst profile available.

**As built: `organize` NEVER resolves an attachment divergence.** It reports the path as `diverged` and names both remedies -- `intent sync --to-store` takes the disk copy; restoring the file from git keeps the store's. _"Decide which way to sync"_ is a human decision by definition; a tool that decides it silently is choosing which of two people's work to discard.

**It can tell them apart, which is why it must.** `Project::classify` is the single answer to "what is this file" for ingest, the migrator, `organize` and `doctor`, so the asymmetry is implementable rather than aspirational.

### 5.1c An UNCARRIED file is not a DROPPED one, and the migration record must not say it is

cc asked whether the files the classifier reported as uncarried should enter the migrator's disposition record with verdict `dropped`. **No, and the reason is the founding sentence of the tool that reads that record:** _"WHY 'STILL ON DISK' IS NOT A DISPOSITION, WHICH IS THE ONE THING THIS TOOL EXISTS TO SAY."_

The two are different facts:

- **dropped** -- content existed, was deliberately not brought across, **and canon is verified empty for it.** Safe because nobody wanted it -- in practice, template boilerplate no author wrote.
- **uncarried** -- content is **still on disk**, is **not in the model**, and **is still the only copy.** Nothing was removed and nothing is safe.

**The disposition record is a LICENCE, not an account.** `conservation_check.sh` reads a declared drop as _"removed on purpose, not loss"_ and stops reporting it. Admitting uncarried files under that verdict would silence the exact population the check exists to find -- **which is the attack ic drove on 2026-08-18 through `--out-of-model`: the migrator zeroes a counter by naming everything, certifying its own denominator.** Same move, different door.

**Home for the fact: `doctor`.** As built, any file a human puts under a thread is an attachment when it fits under `ATTACHMENT_CAP_BYTES` (1 MiB, inclusive; `project.rs:50`), and a file over the cap is the uncarried case, which `doctor` names with its size (`doctor.rs:1220`). An uncarried file is a LIVE CONDITION, not a record of what a migration once did.

### 5.2 Idempotence is a measured requirement, not an aspiration

`organize` run twice must change nothing, **including mtimes.** Measured on this estate 2026-08-18, the render of the day re-emitted unchanged `.md` files byte-identically on every pass, moving their mtime with zero content change. That is harmless for conservation and not harmless for `file_index`, whose `clean`/`changed` state is computed from exactly that. **As built, `organize --apply` writes through the one `WriteSet`, which writes only on content difference.**

### 5.3 It is a bulk writer in a repo with concurrent writers

Several nodes and, until `1ff7f2c1`, a test suite all wrote this estate. A repo with live writers has no demonstrably quiet interval, so the check has to be instantaneous or it is unsatisfiable. **As built, `organize` takes no run lock; what stands over the irreversible step is the moment-of-act digest.** Planning records a digest over the path and content sha256 of every estate file (never mtime); immediately before the first removal the tree is observed again, and a different digest refuses the run (`TreeMoved`) with nothing removed. A re-observation that fails refuses too, rather than reading as unchanged. The guard stands only on `--apply` and only when there is something to remove, so a preview or a pure hydration is never refused because a peer wrote a file.

**The concurrent writer can be any process, not another `organize` run (ic, measured 2026-08-18).** `info`, `st list`, `doctor` and `export` all **materialise the store on access** -- a fresh clone's first read verb builds it from canon. That is correct behaviour, but it means **a peer typing `intent st list` opens the same file `organize` is reconciling against.** So "the estate was quiet when I measured it" is not establishable over any window at all, and the moment-of-act digest is the only defence that survives a concurrent READ.

One consequence worth carrying rather than fixing: **the store is created at the BINARY's schema.** A read in a project with no store leaves one at the current rung, and an older binary then refuses it. The remedy is _build fresh before you measure_, not _stop things touching the estate_.

### 5.4 Output names every bucket

As built, a run prints one summary line naming every bucket, and a preview says so:

```
organize (preview): <n> to hydrate, <n> to rewrite, <n> unchanged, <n> to remove, <n> to prune, <n> unclaimed (<digest>), <n> diverged, <n> refused
  to-rewrite: <path>
  unclaimed: <n> directory(ies) not listed -- `intent organize --verbose` lists them
organize: preview only -- nothing was written or removed. `intent organize --apply` performs it.
```

`--apply` prints the same buckets in the past tense (`hydrated`, `rewritten`, `removed`, `pruned`). Each refusal is named with its path and remedy, and a refusal on `--apply` moves the exit code. The `unclaimed` digest is over the sorted membership of the unclaimed set, so two runs can be compared by membership rather than by size. **A count that names every bucket is a measurement rather than a report**, and `organize` is the command whose greens authorise deletions.

## 6. Realising one artefact on demand

hv proposed `intent wip {{STID}} | {{ISSUEID}}`.

**`wip` is the wrong word and it is not a style objection.** In this project `wip` already means a work-package status (`WpStatus::Wip`), a thread status (`ThreadStatus::Wip`), the project snapshot `intent/wip.md`, and every node's board `intent/whiteboard/<node>/wip.md`. A fifth meaning -- "make this file exist" -- collides with four live ones, in a tool whose own issue 0041 was about a vocabulary spelled two ways.

**Built: `intent edit [KIND] [ID] [FILE]`.** It realises the thread if it is not on disk, adds the thread's id to `.intentfiles`, and prints the path of the file to open (`info` by default). `--path` prints the path whatever stdout is; `--editor[=<program>]` opens it in `$VISUAL`, `$EDITOR` or the named program; `--browser` opens the entity in the browser, served by `intentd`. A `FILE` the thread does not carry is refused with the list of what it does carry.

**`intent st edit <ID>` is the same behaviour spelled under the family** -- _"Print the path to a steel thread file, realising the thread if it is not on disk"_ -- one behaviour, one home. An issue has no realised form, so `intent edit issue <N>` refuses by name; `intent issues edit` corrects an issue's record (its prose, title and severity) through the store instead.

**A hand realisation survives `organize`** because nothing regenerates the manifest from status (4.2): the id `edit` added stays until a command or a human removes it.

## 7. Preconditions -- what had to be true before ANY dehydration shipped

**Dehydration is only sound when every file it may remove is reproducible from the store.** Each precondition this section listed, as built:

- **Attachments.** `Attachment{path,text,bytes,sha256}`, one constructor, `text` carried with NO trim so a round trip cannot cost a byte (landed `36bc02c5`). Any file under a thread that fits under the attachment cap is an attachment; the extension allowlist that once left files uncarried is retired. **A TEXT attachment's bytes are in canon, so it dehydrates and hydrates like a view. An OPAQUE one is recorded by path, size and sha256 with `text: None`, so its bytes cannot be put back: `organize` refuses to remove it and does not hydrate it as an empty file.**
- **The typed thread documents.** `design.md`, `impl.md` and `tasks.md` were classified as typed documents the model had no field for. **Resolved by D57-6: they left the classifier's `THREAD_PROSE` list and are carried as attachments**, verbatim, each indexed as one unsplit section for search (`ingest.rs`).
- **Issue bodies.** `Issue.body` is carried VERBATIM rather than trimmed -- cc's ruling: _a normalisation that requires a future component to compensate is a scheduled defect_. Issues have no realised form, so nothing renders them to disk.
- **`ROOT_FILES`.** `AGENTS.md`, `CLAUDE.md`, `usage-rules.md` stay out of `.intentfiles` scope. `AGENTS.md` has its generator (`intent agents init|generate|sync|validate`).
- **The estate-level verdict.** The dehydration ship gate reads the project's declared preconditions (5.1).

**Dehydration ships behind both gates. Hydration and `edit` are additive and safe.**

## 8. Questions this document put to hv, as settled

1. **Canon location -- A, B or C in section 3.** C, built.
2. **Manifest path.** `intent/.intentfiles`, built. It is a manifest, not an artefact.
3. **`intent edit` over `intent wip`.** `intent edit`, built.
4. **Default rule.** Proposed as every thread whose status is not terminal; **built as every WIP thread and nothing else** (hv, 2026-08-26: _"It should ONLY HAVE WIP STs!!!!!"_). Stated positively, the realised set cannot acquire members by accident: it is the set somebody is working on (`intentfiles::default_declaration`).
5. **Work packages.** A WP is realised with its thread, not independently. There is no `WORKPACKAGE:` sigil.
6. **The typed documents** (`design.md` / `impl.md` / `tasks.md`) -- give the model fields for them, or reclassify them as attachments. **Attachments, on this spec's own rule 2 (D57-6).** A typed field earns its parsing because the model has fields for what comes out. These are freeform prose under arbitrary headings and the model has no fields for what is inside them, **so parsing them would discard structure into nothing -- which is precisely how `## Related Steel Threads` became rows of `LOST-PROSE`.** Carry them verbatim. The one-off `.md` files under a thread (`parity.md`, `data-model.md`, the fleet reports) were already carried as attachments, so the fix was subtractive: three names came out of the classifier's list.
7. **Issues have no rendered-view path in v3.** Settled by retiring `ISSUE:` from the grammar (hv, 2026-08-20): an issue lives in canon (`intent/.canon/issues/<NNNN>.json`, carrying its `body`) and in the store, and has no realised form. The only rendered issue markdown in a v2 estate is its `issues/<BUCKET>/NNNN/NNNN-slug.md`, which is residue this design retires.

## 9. The second derivation

`intent/st/ST0056/parity/tools/realise_plan.sh` was this document made executable before `organize` existed. It reads canon (never the directory listing -- **a listing answers "what has a folder" and the question is "what artefacts exist", and those differ by exactly the set this design is about**), applies a default rule, parses a manifest with the refusing grammar, prints the plan with a denominator, and writes nothing.

**It encodes the pre-build rules**: a non-terminal default rather than WIP-only, the retired `ISSUE:` sigil, and issue canon at v2's `intent/issues/NNNN.json`. **Its DEHYDRATE list is UNGATED and it says so in its own output**, because rendering is the Rust binary's job and a shell script cannot run the reproducibility check. `intent organize` without `--apply` is the gated plan: it classifies exactly as `--apply` does, including the per-file gate, and writes nothing.
