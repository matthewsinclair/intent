---
verblock: "18 Aug 2026:v0.1: vc - selective realisation: .intentfiles, organize, edit"
---

# Selective realisation -- disk as a sparse projection of the store

hv's brief, 2026-08-18: _"The DB is ssot and disk is a sparse copy of the db out to realised files based on an index."_ A manifest at `intent/.intentfiles` names the artefacts that have on-disk form; open and WIP artefacts are in it by default; `intent organize` reconciles disk to the manifest; a per-artefact verb realises one thing on demand so it can be edited.

This document takes that brief, names the one thing in it that cannot be true as written, and specifies the version that can.

## 1. The model is THREE layers, and the brief collapses two of them

| layer     | what                                           | where                           | committed            | who derives it              |
| --------- | ---------------------------------------------- | ------------------------------- | -------------------- | --------------------------- |
| **canon** | `thread.json`, `issues/NNNN.json`              | on disk today, one per artefact | **yes**              | authored through the facade |
| **store** | `intent/.cache/intent.db`                      | machine-local                   | **NO -- gitignored** | rebuilt from canon          |
| **views** | `info.md`, `design.md`, `steel_threads.md` ... | on disk                         | yes                  | rendered from the store     |

**The store is SSOT for a running tool on one machine. It is not what travels.** `intent/.cache/` is gitignored at `.gitignore:127`, and the ignore rule's own comment says the name contradicts the model. `intent sync --to-disk` is documented as _"write the store out to **the committed extract**"_ -- the tool's own vocabulary already distinguishes the runtime store from the durable record.

**So "disk is a sparse copy of the DB" is safe for VIEWS and unsafe for CANON.** If `.intentfiles` governed canon as well, an unrealised artefact would exist only inside a gitignored database. A fresh clone would not contain it. `rm -rf intent/.cache` would not be recoverable. **D29 -- a gitignored path is never canon -- is not a style rule here; it is the thing that makes a clone complete.**

**Ruling this document proposes: `.intentfiles` governs VIEWS ONLY. Canon is never sparse.**

That is what makes every deletion in this design safe rather than merely careful: **`organize` only ever removes a file it can regenerate, from a source sitting beside it in the same commit.**

## 2. Measured: sparse views buy nearly everything, and canon costs nearly nothing

Intent's own estate, 2026-08-18:

```
canon      thread.json x56          1,147,384 bytes
           issues/NNNN.json x40       454,967
                                    ---------
                                    1,602,351   ALWAYS on disk under this design

views      *.md under intent/st     2,704,783   sparse
  of which ST0056 (the ONE live thread)  1,021,934
           ST0011 (a completed one)          7,199

other      non-md non-json          1,063,284   attachments (cc, in flight)
```

**The single live thread is 38% of all rendered markdown.** Realising the live set and dehydrating the other 55 removes roughly 1.68 MB of view and keeps the thing anyone is actually working on. Canon stays whole for 1.6 MB, which is the price of a complete clone and is not a price worth arguing about.

## 3. The fork hv still has to pick: WHERE canon lives

Keeping canon complete does not require keeping it in the way. Three arrangements, and the difference is what `ls intent/st` shows:

- **A -- canon in place (today).** `intent/st/<ID>/thread.json` for all 56. Bytes drop ~60%; **the directory still has 56 entries**, so hv's _"little more than an index file"_ is not achieved.
- **B -- canon consolidated.** One committed `threads.jsonl`. `intent/st/` becomes an index plus realised threads. **Rejected: four nodes writing one file is a merge-conflict generator**, and today's estate has four concurrent writers plus a test suite.
- **C -- canon relocated, still per-artefact.** `intent/.canon/st/ST0011.json`, `intent/.canon/issues/0061.json`. `intent/st/` holds `steel_threads.md` plus only realised threads.

**Recommended: C.** It is the only one that gives hv the directory they asked for, keeps a clone complete, and keeps a per-artefact diff so two nodes editing two threads never conflict. It is a path change, not a model change.

## 4. `.intentfiles`

### 4.1 It is committed, and it has to be

The set it governs is committed. If the manifest were machine-local, `organize` on my machine would delete files from the repository that yours depends on, and the deletion would land in a commit with no declared reason. **A committed manifest makes hydration and dehydration reviewable: the diff shows the manifest line changing beside the files appearing or disappearing.**

### 4.2 Two regions, because a computed default and a written file collide

hv's sketch already has the mechanism -- the marker comments. Make the split load-bearing:

```
# .intentfiles -- which intent artefacts are realised as on-disk files
#
# Lines between BEGIN and END are GENERATED by `intent organize` from status.
# Lines outside the markers are PINS and are never rewritten or removed.

STEELTHREAD:ST0011        # pinned by hand: reading the old test-suite thread

<!-- ISSUES: BEGIN -->
<!-- ISSUES: END -->

<!-- STEEL THREADS: BEGIN -->
STEELTHREAD:ST0056
<!-- STEEL THREADS: END -->
```

**Without the split, the first status change silently deletes a human's pin.** Pin ST0011 to read it, it closes, `organize` regenerates the block from status, the pin is gone and so are the files -- with nothing in the output naming what it decided. The generated region is rewritten wholesale; the pinned region is read-only to the tool.

### 4.3 The grammar is enforced, and a malformed line REFUSES

`<SIGIL>:<ID>` with `SIGIL` in `STEELTHREAD | ISSUE`, `ID` matching that kind's id shape, optional trailing `# comment`. Nothing else.

**A line `organize` cannot parse must abort the run, naming the line number.** Not skip it. A skipped line silently drops an artefact from realisation, and the estate looks exactly like a correct one that never listed it. **This project has paid for that shape twice already** -- v2.19.0's `ac gate` F1 fix turned malformed AC/AT lines from a silent drop into a block precisely because a silent drop produced a vacuous green, and the AT row grammar with `at lint` L1-L5 exists for the same reason. The manifest is a third instance of the same file class and gets the same rule from the start.

## 5. `intent organize` -- a reconciler with three answers per artefact

```
intent organize [--dry-run] [--json]
```

Existing verb, currently v2's status-directory tidier. Under the flat layout it has no work left, so it is the right name to inherit rather than a new one to invent.

For every artefact in canon, compare **declared** (in the manifest) against **actual** (on disk):

| declared | actual  | action                                                            |
| -------- | ------- | ----------------------------------------------------------------- |
| yes      | absent  | **HYDRATE** -- render views from the store                        |
| yes      | present | **VERIFY** -- re-render in memory; write only if the bytes differ |
| no       | absent  | nothing                                                           |
| no       | present | **DEHYDRATE -- but only if it passes the gate in 5.1**            |

Plus one row that is not about an artefact at all:

| -- | a path under a realised artefact that the renderer does not produce | **UNCLAIMED -- report, never remove** |

**A file the renderer does not know how to make is either an attachment or something a human put there, and both are content. `organize` must not be the thing that decides an unrecognised file is rubbish.**

### 5.1 The dehydration gate -- the only irreversible act in the design

Before removing any view, per file:

1. **Re-render it from the store into memory.**
2. **Compare to the bytes on disk.** Identical -> safe, the file is a pure projection. **Different -> REFUSE and report the path.** A difference means the disk holds something the store does not, and that something is destroyed by the removal.
3. Never remove a path outside the render set (rule above).

This is a per-file, in-process check costing one render and one comparison. **It makes `organize` fail-safe by construction rather than by discipline**, and it is the same view-skew comparison the doctor already performs -- one mechanism, two callers.

### 5.1b An ATTACHMENT is not a VIEW, and one policy for both is wrong (cc, 2026-08-18)

**5.1 as written treats every file under a thread the same way, and that is a hole cc found before it was built.** The gate re-renders and refuses on difference, which is right for a generated view and **backwards for an attachment**.

**The direction of authority follows the direction of authorship.**

|                                               | authored in | disk divergence means                                                                | correct move                                                                                                               |
| --------------------------------------------- | ----------- | ------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| **view** (`info.md`, `design.md`)             | the model   | the file is stale, or someone hand-edited a generated file the canon forbids editing | regenerate -- **but only once the model demonstrably holds everything the file holds**, which is what 5.1's refusal proves |
| **attachment** (`data-model.md`, `parity/**`) | **on disk** | the STORE is stale                                                                   | **ingest**, never overwrite                                                                                                |

**Overwriting an attachment from the store destroys the author's edit. Ingesting a view from disk promotes a stale generated file into canon.** Same divergence, opposite remedies, and a tool that guesses gets it right most of the time and catastrophically wrong occasionally -- the worst profile available.

**Ruling this document proposes: `organize` NEVER resolves an attachment divergence.** It reports the path and names the two verbs (`sync --to-store` already exists for one direction). _"Decide which way to sync"_ is a human decision by definition; a tool that decides it silently is choosing which of two people's work to discard.

**It can tell them apart, which is why it must.** cc's `Project::classify` is now the single answer to "what is this file" for ingest, the migrator and `doctor` -- so the asymmetry is implementable rather than aspirational, and one policy for both would be a deliberate discarding of information the tool already has.

**And declaring the issue view path in section 8.6 makes one of cc's conditionals real.** `Issue.body` is stored trimmed of surrounding blank lines and that is safe today only because _nothing renders an issue to disk_. `Attachment.text` is carried with no trim at all, precisely so a round trip cannot cost a byte per pass. **The moment `intent/issues/NNNN/NNNN.md` exists, `body` acquires the round trip `text` was protected from** -- so "the renderer must re-emit the trailing newline" stops being a note on cc's board and becomes a precondition of this design.

### 5.2 Idempotence is a measured requirement, not an aspiration

`organize` run twice must change nothing, **including mtimes.** Measured on this estate 2026-08-18: the current render re-emits 255 of 1000 `.md` files byte-identically on every pass, moving their mtime with zero content change. That is harmless for conservation and not harmless for `file_index`, whose `clean`/`changed` state is computed from exactly that. **Write only on content difference. Never `create` then `write`.**

### 5.3 It is a bulk writer in a repo with concurrent writers

Four nodes and, until `1ff7f2c1`, a test suite all write this estate. `organize` takes an exclusive lock for the run, and **re-checks the digest of anything it is about to delete immediately before deleting it** -- not over a window, at the moment of the act. A repo with live writers has no demonstrably quiet interval, so the check has to be instantaneous or it is unsatisfiable.

**And the lock is against ANY process, not other `organize` runs (ic, measured 2026-08-18).** `info`, `st list`, `doctor` and `export` all **materialise the store on access** -- a fresh clone's first read verb builds it, 4 MB, from canon. That is disk-optional arriving early and it is correct behaviour, but it means **a peer typing `intent st list` opens the same file `organize` is reconciling against.** So "the estate was quiet when I measured it" is not establishable over any window at all, and the moment-of-act digest stops being the careful option and becomes the only defence that survives a concurrent READ.

One consequence worth carrying rather than fixing: **the store is created at the BINARY's schema.** A read in a project with no store leaves one at the current rung, and an older binary then refuses -- which is exactly `target/release/intent` speaking 6 against a store at 8, met live this morning. The remedy is _build fresh before you measure_, not _stop things touching the estate_.

### 5.4 Output carries a denominator

```
organize: 41 artefact(s) declared of 96 in canon
organize: hydrated 3, verified 38, dehydrated 12, unchanged 43
organize: REFUSED 2 -- intent/st/ST0031/design.md, intent/st/ST0031/tasks.md
          (on disk and NOT reproducible from the store; nothing was removed for ST0031)
```

**A count with no denominator is a report rather than a measurement**, and `organize` is the command whose greens authorise deletions.

## 6. Realising one artefact on demand

hv proposed `intent wip {{STID}} | {{ISSUEID}}`.

**`wip` is the wrong word and it is not a style objection.** In this project `wip` already means a work-package status (`WpStatus::Wip`), a thread status (`ThreadStatus::Wip`), the project snapshot `intent/wip.md`, and every node's board `intent/whiteboard/<node>/wip.md`. A fifth meaning -- "make this file exist" -- collides with four live ones, in a tool whose own issue 0041 was about a vocabulary spelled two ways.

**Proposed: `intent edit <ID>`**, dispatching on id shape. It hydrates if needed, updates the manifest's pinned region, and prints the path (or opens `$EDITOR` behind a flag).

**`intent st edit <ID>` already exists and already prints the absolute path to a thread file.** So this is not a new verb, it is that verb learning to hydrate first -- Highlander, one behaviour, one home, with `st edit` and `issues edit` delegating.

**A hand realisation writes to the PINNED region, not the generated one.** Otherwise the next `organize` reverts it, because the generated region is a function of status and the artefact you opened is closed.

## 7. Preconditions -- what must be true before ANY dehydration ships

**None of this is safe today, and the gap is known and being closed.** Dehydration is only sound when every `.md` is reproducible from the store. Currently it is not:

- **Attachments.** `data-model.md`, `migration.md`, `parity/**` and the other untyped files are not modelled yet. cc is building it.
- **Issue bodies.** Landed; measured 40 of 40 conserved, **trailing newline normalised, not byte-identical.** Under 5.1 that difference REFUSES every issue -- correctly -- until the render re-emits the newline. **A working design failing safe on a one-byte difference is the gate doing its job, and it is worth seeing it happen once.**
- **`ROOT_FILES`.** `AGENTS.md`, `CLAUDE.md`, `usage-rules.md` have no v3 generator (`agents` is unimplemented). Out of `.intentfiles` scope, but they are on disk and something emptied `AGENTS.md` today, so they need their generator before anyone reasons about them as derivable.
- **The conservation verdict.** Green at full scope over a pinned subject, per the gate in `conservation_check.sh`'s header.

**Build `organize` now; ship dehydration behind the gate. Hydration and `edit` are additive and safe immediately.**

## 8. Open for hv

1. **Canon location -- A, B or C in section 3.** C is recommended and is the one that delivers _"little more than an index file"_.
2. **Manifest path.** `intent/.intentfiles` as briefed, or `intent/.config/` beside the other configuration. The scan must exclude it either way; it is a manifest, not an artefact.
3. **`intent edit` over `intent wip`** -- confirm, given the four existing meanings of `wip`.
4. **Default rule.** Proposed: realise every artefact whose status is not terminal. Terminal = `Completed | Cancelled` for threads, `Closed` for issues -- **reusing the `is_terminal()` cc is adding for the doctor's completion arm rather than spelling the vocabulary a second time.** On this estate that default realises 2 threads of 56 and 0 issues of 40.
5. **Work packages.** A WP is realised with its thread, not independently. Confirm, or `WORKPACKAGE:ST0056/10` becomes a third sigil.
6. **Issues have no rendered-view path in v3, and `intent edit <ISSUE>` therefore has nowhere to write.** `intent/issues/NNNN.json` is canon and now carries `body`; the only rendered issue markdown in the estate is v2's `issues/<BUCKET>/NNNN/NNNN-slug.md`, which is precisely the residue this design retires. A path has to be declared -- `intent/issues/NNNN/NNNN.md` is the obvious shape and matches the thread arrangement. **Found by the plan tool rather than by writing this document**, which is the argument for building the second derivation: a probe looking only at the v3 path reported all 40 issues absent, and "absent" reads as nothing-to-do.

## 9. A second derivation exists and disagrees usefully

`intent/st/ST0056/parity/tools/realise_plan.sh` is this document made executable. It reads canon (never the directory listing -- **a listing answers "what has a folder" and the question is "what artefacts exist", and those differ by exactly the set this design is about**), applies the default rule, parses a manifest with the refusing grammar, and prints the plan with a denominator. It writes nothing.

Against Intent's own estate today:

```
realise: canon -- 56 thread(s), 40 issue(s); 2 non-terminal
realise: manifest -- ABSENT; planning against the default rule alone
realise: plan -- HYDRATE 0, VERIFY 2, DEHYDRATE-CANDIDATE 94, already-absent 0 (of 96)
realise: NOTE -- 40 issue view(s) found ONLY at v2's bucket path ...
```

**Its DEHYDRATE list is UNGATED and it says so in its own output**, because rendering is the Rust binary's job and a shell script cannot run the reproducibility check. A plan that printed a dehydrate list without saying so would read as permission to delete.
