# Data model - ST0056: the reified Intent model (WP-01 spec)

Status: ratified structure per design.md D01-D05. **For the entities and their fields this document describes, it does not define** -- the authored master is the Rust type layer and the committed faces under `schema/` are generated from it. The WP-01 draft schema that stood at the foot of this file is pruned; see "The schema face" below. **For the state machines this document IS the authority**: `transitions.rs` transcribes them, and the commit gate runs `machine_table_check.sh` to hold the two together.

Amendments after WP-01 (vc, 2026-08-14, ADOPTED under hv standing authorisation): `objective`/`context`/`related` modelled on `steel_thread`; the marked-legacy `legacy` form on `acceptance_test`; the no-clock law on generated views. Each carries its rationale inline below.

## Entities

Identity convention (D15): natural keys stay human-legible; `(project_id, natural_id)` is the global identity. All dates ISO 8601 (`YYYY-MM-DD`); all timestamps UTC RFC 3339.

### Time convention (D42, AC-02.8 -- ruled 2026-08-15)

**Two kinds of time live in this schema and they must never be one column.** The entity tables below carry (b); (a) is the record-timestamp column AC-02.8 added to the store's tables.

- **(a) RECORD timestamp** -- when THIS store wrote THIS row. Set by the database as part of the write (a `DEFAULT`, never a caller value). Per-machine, **not carried in the extract**, and **correctly re-stamped on every rebuild** -- the row genuinely was written then.
- **(b) DOMAIN timestamp** -- when the thing happened in the project's history: `threads.created`, `threads.completed`, `issues.created`, `issues.closed` -- see the door ruling below. **Carried in the extract, never re-stamped**, and displayed by `st show` / `st list` and the `.md` views.

**Replacing (b) with (a) means a colleague who clones and rebuilds sees every thread created today.** The two doors keep them apart: **create** (the DB stamps) and **restore** (the recorded stamp is carried). Restoring is not creating.

Per-table naming follows how each table is written. Rows with durable identity are upserted (`INSERT ... ON CONFLICT DO UPDATE`, with `updated_at` moved inside the conflict clause); child rows are deleted and re-inserted with their parent. No entity table is written by a bare `UPDATE`, so an `ON UPDATE` trigger would never fire, and a `created_at` on a replaced row would silently record the latest write:

| tables                                               | columns                                  | why                                                                                          |
| ---------------------------------------------------- | ---------------------------------------- | -------------------------------------------------------------------------------------------- |
| `threads`, `issues`, `file_index`                    | `created_at` + `updated_at`              | upserted, so the row survives: `created_at` fires once, `updated_at` moves DB-side           |
| `related`, `attachments`, `wps`, `criteria`, `tests` | `written_at`                             | replaced wholesale, so the honest record is _when this version of this row was written_      |
| `event_log`                                          | `ts` (already)                           | `ts` IS the record timestamp and rows are immutable -- no second column, and the DDL says so |
| `snapshots`, `ingests`                               | `taken_at` / `started_at` + `updated_at` | operations logs: a row is written before the attempt and updated after it                    |
| `project`                                            | `updated_at`                             | a singleton holding project state (the todo watermark, below)                                |

**A column is named for what it can honestly record, never for uniformity across tables.** `created_at` on a wholesale-replaced table would be a lie the moment a rebuild ran.

`threads.created` / `completed` are **stamped by the database inside the thread write** on the create door (SQLite's `strftime('%Y-%m-%d','now')` in the `INSERT`, returned to the caller by `RETURNING`) and **carried verbatim** on the restore door, so (b) is a time that went end-to-end through the database. A v2-migrated thread carries its authored `created:` into canon; migration writes no events, so a migrated thread has its dates and no history. `issues.created` from v2 stays authored -- users wrote it in frontmatter and it is genuinely a fact about the world; a v3-native `issues add` has it stamped by the database.

#### THE DOOR IS A PROPERTY OF THE ACT, NOT OF THE ENTITY (ruling, vc, 2026-08-17, on cc's finding)

**There are FOUR domain dates.** `threads.created`, `threads.completed`, `issues.created` **and `issues.closed`**. **The comment that defines what a domain date IS is the comment a future author reads to decide whether a new column needs a door**, so a column missing from it is a column that will be added without one.

**The ruling: every entity carrying a domain date needs both doors, because the door is chosen by the ACT and both acts reach every such entity.** As built, `Store::write_thread` and `Store::write_issue` each take a `Stamp` (`store.rs:1991`, `:2176`): `Stamp::ByTheDatabase` is the create door, where SQLite fills an empty date inside the `INSERT`, and `Stamp::CarriedFromTheExtract` is the restore door, where the recorded date is written verbatim. `rebuild` uses the restore door; `commit_mutation` uses the create door.

**`issues.closed` takes the three-state form `threads.completed` has** -- `None` stays null, `Some("")` is stamped by the database, `Some(date)` is carried -- because `issues close` is precisely the act that produces it, and the alternative is the facade reading a clock, which D42 forbids.

**The `stays authored` rationale answers a migration question only**: why a v2 `created` is not replaced by `created_at`, where a v2 author really did write the date. When **nobody authored it** -- every v3-native `issues add` -- the create door stamps it.

**`commit_mutation` returns `StoredDates`** (`store.rs:1607`, `:2317`), one named struct carrying the thread dates, the issue dates and the stamp on the mutation's own event, so every date the database set comes back from the write for the extract to render from. Without that channel, truth and its projection would disagree on the one field neither can recompute, which is D42's own stated hazard.

**Scope call with its reversibility** (D39): `wps` and `criteria` have stable IDs, so wholesale-replace is a property of today's write strategy, not of the domain. Delete-missing + upsert-present is the upgrade path and `written_at` does not block it.

### project (`intent/.config/config.json` -- as today, plus)

| Field                | Type   | Notes                                                                                           |
| -------------------- | ------ | ----------------------------------------------------------------------------------------------- |
| project_id           | uuid   | stamped at migration (D15); never changes                                                       |
| intent_version       | string | `3.0.0`+                                                                                        |
| project_name, author | string | as v2                                                                                           |
| languages            | array  | as v2 (ST0037)                                                                                  |
| server               | object | RESERVED, absent in v3 (D15); intentc-era binding                                               |
| ~~st_prefix~~        | --     | **RETIRED (hv, 2026-08-16, issue 0040)** -- see below                                           |
| todo                 | object | `{window_hours: int}`, default 24 -- **NOT the watermark**; see below                           |
| backup               | object | the D35 snapshot schedule and per-tier retention                                                |
| doctor               | object | findings a project has acknowledged; absent unless used                                         |
| _(any other key)_    | --     | carried verbatim (`Config::extra`), so a rewrite never drops a block this version does not know |

Project-level **state** -- as opposed to configuration -- lives in committed canon at `intent/.canon/project.json` (`schema: intent/project@3.0`), carried into the store's `project` singleton table. Its one field today is `todo_watermark`, below.

#### `st_prefix` -- RETIRED (hv ruled 2026-08-16, issue 0040)

**The thread-id prefix is fixed at `ST` in v3. The configurable knob is gone.**

v2 let a project change the two letters at the front of every thread id, and v2's `intent init` wrote the field into every project it created. v3 fixes the prefix at `model::THREAD_PREFIX`, and `Config` has no `st_prefix` field: a declared value lands in `Config::extra` and is carried byte for byte, never honoured.

**Retiring it is not a change of direction: `st_prefix` appears in no ST0056 spec.** The design had already dropped the knob; the field was residue of a decision already taken, not a feature awaiting wiring.

**Grounds for fixing rather than honouring, measured:** every fleet project uses `ST` (Laksa by omission, the rest explicitly), so the migration corpus is unaffected; and every id-touching feature -- search, code parsing, the daemon -- would otherwise have to honour a variable that is always the same value.

**Two obligations come WITH the retirement, and both are built:**

1. **The migrator BLOCKS, naming the field and its value, when a project carries a non-`ST` value** (`legacy.rs` `retired_settings`, finding class `RetiredSetting`). A project whose threads sit on another prefix would otherwise have none of them recognised and report a clean conversion of an empty estate. Retiring a knob nobody uses is fine; retiring it under someone who does use it, without telling them, is not.
2. **The thread-id width is derived, never asserted**: `is_thread_id` and the legacy scan compute it as `THREAD_PREFIX.len() + THREAD_DIGITS`, so there is one encoding of "prefix plus digits".

#### The todo watermark: project state, never read back out of the view

Found by cc at WP-03 when the no-clock law (D23) forced the question of whether `todo.md`'s `## DONE:<timestamp>` heading was render time or data. It is data. v2 called it "the last-flush watermark", advanced it only by `done --flush` / `--prune`, and **grepped it back out of the generated `todo.md`**, falling back to `date -u '+%Y-%m-%dT00:00:00Z'` when the file or heading was absent. So the view was the only durable home of a fact the tool read back as truth: deleting `todo.md` silently reset the watermark to start-of-today, a generated artefact was authoritative (the exact inverse of D02), and the render path read a clock (which D23 forbids).

Ruling (vc, 2026-08-14; ADOPTED under hv standing authorisation): the watermark is **durable project state**, **always materialised and never defaulted at render time**. The render path receives it as an input and never reads it back. The v2 start-of-today fallback does not survive -- a default computed from a clock is the defect wearing a different hat.

**As built (hv, 2026-08-26):**

- `intent todo done --flush` advances the watermark to now, and `--prune` emits the DONE items for archiving and then flushes. The database stamps the watermark in the same transaction as the `todo.flush` event: the event is history (a flush happened), the watermark is state (the cutoff is now this).
- The watermark lives in `intent/.canon/project.json` as `todo_watermark` and in the store's `project` table. **Absent means never flushed** and hides nothing -- a state v2 could not represent.
- DONE holds every finished thread whose `completed` date, widened to an instant, is at or after the watermark. The heading reads `## DONE:<T>` when a watermark is set and `## DONE` when none is.
- **The terminal render and the committed `todo.md` are the same bytes.** The view is a function of the model and the watermark, never of the hour it was rendered.

**`todo.window_hours`** (config, default 24) is read only by `doctor`, which reports a value that is not a whole multiple of 24 (`completed` is a date, so a finer cutoff has nothing to bite on). No render applies it.

### steel_thread (`intent/.canon/st/<ID>.json`, the thread canon)

| Field          | Type    | Notes                                                                                                          |
| -------------- | ------- | -------------------------------------------------------------------------------------------------------------- |
| schema         | string  | `intent/thread@3.0` -- lets validators pick the schema                                                         |
| id             | string  | `ST0056`                                                                                                       |
| title          | string  |                                                                                                                |
| slug           | string? |                                                                                                                |
| preamble       | string  | authored prose ABOVE the first `## `, minus the `# ` title -- carried verbatim, never classified; may be empty |
| objective      | string  | authored prose; may be empty (see below)                                                                       |
| context        | string  | authored prose, markdown, carried verbatim                                                                     |
| related        | array   | `{id, note?}` -- the Related Steel Threads block                                                               |
| body           | string  | authored prose, every other section verbatim -- the thread-level twin of D28's WP catch-all; may be empty      |
| status         | enum    | `triage · not-started · wip · hold · completed · cancelled` -- **`tbc` is NOT a v3 value** (see below)         |
| status\_reason | string? | the reason for the CURRENT status; cleared by any transition that does not carry one                           |
| fiat           | object? | the fiat-close record, present exactly when the thread reached `completed` through `st.fc` (Machine 1)         |
| created        | date    |                                                                                                                |
| completed      | date?   |                                                                                                                |
| acceptance     | enum?   | `exempt` (ST0048) or absent = enforced                                                                         |
| wps            | array   | work_package records, ordered by seq                                                                           |
| criteria       | array   | acceptance_criterion records                                                                                   |
| tests          | array   | acceptance_test records                                                                                        |
| attachments    | array   | every other authored file under the thread directory, carried verbatim (see below)                             |

**`tbc` is not a thread status value.** The ratified machine (Machine 1 below) has no such state; `triage` is the real entry state. And v2's `tbc` means **To Be Commenced**, not triage -- so it maps to `not-started`, never to `triage` (migration.md carries the rule and ic's independent witness for it). **v3 must not accept `--status tbc` nor abbreviate `Triage` as `TBC`**: reusing the letters is exactly how a mapping rule gets undone by a surface. As built, `st list --status tbc` is accepted and filters on `not-started` (v2's synonym, the same mapping the migrator applies); nothing reads `tbc` as `Triage`.

**`status_reason` is a denormalised read of the latest guarded transition, never a second source for history** (cc, 2026-08-15, on hv's _"feel free to add to the schema to support this kind of thing"_). The history is the event envelope, which every guarded verb writes. It is cleared by any transition that does not carry a reason -- otherwise `st hold --reason "waiting on the fleet"` followed by `st resume` leaves a running thread explaining why it was paused, which is a stale value that reads as current. **It is in AC-02.6's scope like every other field**: a file form must carry it, or the round-trip loses the reason at the clone boundary.

No verblock: git is the history of structured files. Authored prose files keep the v2 verblock convention unchanged; generated views carry a generated-banner footer instead (the AGENTS.md pattern).

#### `preamble` -- the region above the first heading, and it is a CONSERVATION FIX rather than an additive field (ruling, vc, 2026-08-17, on cc's measurement)

**Definition, stated first because the boundary IS the measurement: everything after the frontmatter and before the first `## `, minus the `# ` title line, stripped.** Carried on both `steel_thread` and `work_package`.

**Before the field existed the region was LOST**: the legacy parser buffered only once a section had opened, so every byte before the first `## ` fell on the floor, and `conservation_check.sh` reported it as `LOST-PROSE ... in no section, no objective, no body`.

**WHAT IS IN IT DECIDES THE CLASSIFICATION.** ST0010's region is a deprecation blockquote carrying a supersession pointer, plus an authored metadata block. **A cancelled thread's "superseded by X" note is precisely what the cancellation discipline exists to preserve**, so dropping it with no drop record is a conservation defect and not a convenience.

**IT HAS ITS OWN FIELD AND DOES NOT GO IN `body`** (cc, and the reason is load-bearing rather than aesthetic): `body` renders after `## Objective`, so a preamble carried there would come back in the wrong place. **Bytes preserved, position changed -- which trades a silent DROP for a silent MOVE, and the second is harder to see than the first.** The views render `preamble` above the first heading.

**CARRIED VERBATIM, NEVER CLASSIFIED.** cc's split of the canary regions found them largely metadata restatement (`- **Status**: ... / - **Created**: ... / - **Author**: ...`), and that is exactly why no classifier is built for them: a model naming the shapes it foresaw drops what it did not, and here the unforeseen remainder is the load-bearing half.

**THE BOUNDARY IS THE STRIPPED FORM, AND THIS PARAGRAPH IS THE RULING THE CHECK REFUSES TO MAKE FOR ITSELF.** `conservation_check.sh` deliberately does not rule that trimming is acceptable -- _"a check that silently adopted the migrator's own normalisation would be certifying it"_ -- it only reports WHICH KIND of difference occurred. The contract rules it here: **the field stores the stripped region, and the surrounding blank lines are markdown layout the renderer re-emits.** Consequence, stated so nobody later reads it as a regression: **preamble regions land as `NORMALISED-PROSE`, not `CONSERVED` -- a reported, counted, NON-finding, the same treatment every other section already receives.**

#### `attachments` -- arbitrary authored files under a thread, and the rule that keeps disk optional (spec, vc, 2026-08-18, on hv's ask)

**hv's requirement: _"whatever is in the `ST####/**/*.{md,txt,...}` can also be attached to the db in a lossless manner."_** It exists because hv ruled that disk becomes optional -- an index plus render-on-demand -- and **the moment disk is optional, anything the store does not hold is destroyed by the first render.** Measured on this estate the day the hoist landed, a population of one-off documents nobody modelled -- `reference.md`, `dogfood-journal.md`, `phase0_summary.md`, `phase1_plan.md`, `done.md`, `README.md` -- was not in the store, and no surface said so.

**THE LINE IS OWNERSHIP, THEN SIZE -- NEVER EXTENSION.** `Project::classify` is the one classifier, and ingest, `doctor` and the renderer all ask it:

- A file the renderer owns -- the thread's `info.md`, its `acceptance.md`, a work package's `WP/<NN>/info.md` -- is a generated view. A stray `thread.json` (a v2 tree, or one caught mid-move) is canon. Everything else under the thread directory is an **attachment**, including `design.md`, `impl.md` and `tasks.md`.
- An attachment is carried when it is at most `ATTACHMENT_CAP_BYTES` (1 MiB, a placeholder that stands until attachment bytes live outside the extract). A file over the cap is **refused by name with its size and left exactly where it is** -- never deleted.
- **The form follows the content, decided by decoding**: valid UTF-8 is carried inline as `text`; anything else is carried OPAQUE, its bytes in the store's `attachments.blob` and in canon as a sidecar file at `intent/.canon/st/<ID>/<path>`.
- A path that escapes the thread directory or is not already normalised is refused at ingest (`project::attachment_name`), so a name nothing could store or address never becomes canon.

**No extension is consulted.** An extension answers a question about a NAME, and the questions here are whose file this is and whether its bytes fit; a declared-extension set, the first design, excluded valid-UTF-8 files by name and left the opaque path unreachable. Size is the honest gate because size is the actual cost.

**The store is the record of intent, not a second filesystem.** One consequence is carried openly rather than solved: **a mode bit does not survive** -- an executable written back from the store arrives without its `+x`. A script that has to be `chmod +x` is recoverable and a script nobody kept is not.

**AND THE PROPERTY THAT MATTERS MORE THAN THE LINE ITSELF: A FILE THAT IS NOT CARRIED MUST BE NAMED, NEVER SILENTLY SKIPPED.** `doctor` lists every file under a thread that is over the cap, by path and size, asking the same `within_attachment_cap` function the carrier asks. Without that, **disk becomes optional and something vanishes because nothing ever said it was not covered**, and an absence is indistinguishable from a decision. A project with no oversized file under a thread prints nothing at all.

**Four rules.**

1. **A file is a typed doc OR an attachment, never both.** The generated views are rendered from the model; everything else is carried verbatim. Two homes for one file is the Highlander violation this field would otherwise introduce.
2. **`text` is opaque to the model.** The typed docs earn their parsing because the model has fields for what comes out. An attachment has no such fields, so parsing it would discard structure into nothing -- which is how `## Related Steel Threads` became `LOST-PROSE`.
3. **Non-UTF-8 is carried as bytes, never refused and never skipped.** An unreadable file is still refused by name -- nothing can carry bytes it could not obtain.
4. **Text attachments feed `doc_sections` so `search` finds them**, as ONE unsplit section per file -- no headings parsed, nothing carved. An opaque attachment contributes no section. That index is derived and rebuildable; the attachment record is the authority.

`intent st attach <ID> <PATH> --from <file>` writes an attachment's content from a local file.

##### `attachment` (inside the thread canon)

| Field  | Type    | Notes                                                                                                                   |
| ------ | ------- | ----------------------------------------------------------------------------------------------------------------------- |
| path   | string  | relative to the THREAD's own directory, eg `reference.md` -- never to the project                                       |
| text   | string? | the content when it is TEXT, byte for byte. **Absent means OPAQUE, and that absence is the ONLY marker of which it is** |
| bytes  | u64     | the content's length                                                                                                    |
| sha256 | string  | of the content; derived with `bytes` at construction, never set by a caller                                             |
| blob   | bytes?  | `#[serde(skip)]` -- the opaque body in memory, never inline in the extract; it travels as the sidecar file              |

**Why `text` is OPTIONAL rather than paired with an `opaque: true`:** a second field asserting what the first already shows is a way for the two to disagree -- the same argument `Attachment::new` makes about `bytes` and `sha256`. A reader asks whether the text is here, and there is nothing else to consult and nothing to contradict. **The absence is unambiguous rather than merely convenient:** every attachment written before the field became optional carries a `text`, so no existing artefact reads as opaque by omission, and `Some(s)` serialises exactly as the old `String` did -- the canon files do not move a byte.

**What this discharges.** It is the precondition for hv's disk-optional model, and the gate is `conservation_check.sh`: it asks whether every authored section has a destination and whether the bytes that arrive are the bytes that left.

**`LOST-PROSE 0` is not that gate, and reading it as one is lethal because it is TRUE and means something narrower than it reads.** It means _every section that HAS a destination reached it_. It says nothing about sections with no destination, and **those are exactly the population disk-optional deletes.**

**THE GATE IS THE VERDICT AT FULL SCOPE, NEVER A COUNTER:** exit 0 AND a printed `conservation: 0 finding(s)` line (its ABSENCE is a refusal, not a zero) AND the denominator AND a pinned subject, per `conservation_check.sh`'s own header.

#### Why objective / context / related are modelled (the info.md mixed-file resolution)

D02 forbids mixed files, and v2's `info.md` is flatly one: frontmatter and status (structure), Objective and Context (authored prose), Related Steel Threads (structured links), and a "Context for LLM" template block. design.md's layout table makes `info.md` a generated cover while listing "objective/context prose" as authored -- naming no file for it. Surfaced by cc at WP-03 start, when the view renderer became the thing that would have had to discover the answer.

Ruling (vc, 2026-08-14; ADOPTED under hv standing authorisation): the three fields are **modelled on `steel_thread`**, and `info.md` becomes 100% generated. There is no sixth default steel-thread doc.

- `objective` was already a field v2 had an opinion about -- the 0010 empty-objective warning -- which is the signature of a modelled field rather than free prose. Any such warning is **computed from emptiness, never stored**, on the same double-truth grounds as `satisfied`. v3 emits none: `st done` and `wp done` close a unit with an empty objective without comment.
- `context` and `related` follow it into the model because splitting one cover across a modelled half and an authored half rebuilds the mixed file one level down.
- The alternative -- a new authored `context.md` -- was rejected on reversal cost, not on taste. A field-add reverses by moving two fields; a sixth default doc changes the template set, `intent st new`, the migrator and every consumer's mental model.

##### `related` (inside the thread canon, the `related` array)

| Field | Type    | Notes                                                      |
| ----- | ------- | ---------------------------------------------------------- |
| id    | string  | the related thread's natural id, eg `ST0000`               |
| note  | string? | absent where v2's Related Steel Threads block carried none |

Named cost, accepted deliberately: multi-paragraph markdown lives inside a JSON string field. It is tool-written and authored via mutation, which is what D02 asks for, and prose bodies are still stored verbatim and never reflowed.

Deferring this to WP-10 was the rejected option. The migrator would have discovered the missing prose home, and a migrator that meets an unspecified half of its own target is the `at lint --fix` scar repeating: a tool that cannot finish a job must not start it.

### work_package (inside the thread canon)

| Field          | Type    | Notes                                                                                                                              |
| -------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| seq            | int     | rendered `WP-01`                                                                                                                   |
| title          | string  |                                                                                                                                    |
| scope          | enum?   | `XS · S · M · L · XL · XXL`; absent exactly when `scope_legacy` holds a v2 value outside the set (see below)                       |
| scope\_legacy  | object? | the marked-legacy carry form, `{raw}`                                                                                              |
| status         | enum    | `not-started · wip · done · cancelled`                                                                                             |
| status\_legacy | object? | a v2 status outside the vocabulary, carried verbatim beside the status it was read as                                              |
| status\_reason | string? | as `steel_thread.status_reason` -- current status only, cleared by a transition carrying none                                      |
| fiat           | object? | the fiat-close record, present exactly when the package reached `done` through `wp.fc` (Machine 2)                                 |
| preamble       | string  | as `steel_thread.preamble`; may be empty                                                                                           |
| objective      | string  | authored prose, the `## Objective` section (D28); may be empty                                                                     |
| body           | string  | authored prose, every other section verbatim (D28) -- `## Deliverables` and `## Dependencies` live here, deliberately unstructured |

#### scope: canonicalisation is not loss, but one v2 value is outside the set

Measured on this repository's own corpus (vc, 2026-08-15, on cc's WP-06 finding): v2 reads `scope` as **free text**, and work packages carry many spellings -- `Small`, `Medium`, `Large`, `L`, `XL`, `M`, `S`, `ExtraSmall`, `Extra Small`, `XS` -- and one `Medium-Large`.

**Every spelling of a size is `corrected`, and rendering it canonically is not lossy.** The model declares `scope` an enum, so the enum is the truth and the spelling was always incidental presentation of one of the sizes; `Extra Small` and `XS` carry identical information. "As observed" cannot mean reproducing every spelling of a size, because the thing observed was a free-text field standing in for an enum.

**`Medium-Large` decides the rule.** It maps to nothing in `XS · S · M · L · XL · XXL` -- it sits between two of them -- and it lives in ST0020's WP-09, in a **CLOSED** thread. hv's ratified carry policy is that CLOSED threads are lossless-by-carrying and LIVE threads are BLOCKED-until-clean, and **neither is ever lossy**. So all three obvious moves are forbidden at once: normalising it to `M` or `L` is a guess and lossy; blocking on it violates lossless-by-carrying for a closed thread; dropping it is loss outright.

**Ruling: `scope` carries a marked-legacy form for a value outside the enum**, following the precedent this model already sets for `acceptance_test`'s marked-legacy shape. A closed thread carries losslessly, the value stays visible AS legacy rather than being silently canonicalised into a lie, and the enum stays honest for everything new. A LIVE thread carrying an unmappable scope still BLOCKS, per the same policy. The general form is D05's posture applied one level down: an unknown enum VALUE is refused or marked by name, never guessed -- exactly as an unknown FIELD is.

`objective` and `body` exist because `WP/<NN>/info.md` is the same mixed file `steel_thread`'s `info.md` was, and D22 was never applied one level down -- see D28. Without them the WP-10 migration drops every work package's authored prose, which AC-10.5's prose-conservation clause forbids. **The contract already carried that gate; the model did not carry the field.** `## Acceptance` is not modelled: its text is fixed boilerplate pointing at `acceptance.md`, so it is generated, and restating ACs in a WP view would be the double truth the single-source rule exists to stop.

### acceptance_criterion (inside the thread canon)

| Field | Type   | Notes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ----- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| id    | string | `AC-01.1` -- **the group is a GROUP, and it names a WP only when the thread HAS work packages.** `00` is thread-level. In a thread with no WPs the group is a numbering device and references nothing, and several threads in this estate number criteria across groups with no WP directories at all. **`doctor` asserts the WP reference only when `!thread.wps.is_empty()`** (`doctor.rs:924`): in a thread that DOES carry work packages, a group naming a missing WP is a real inconsistency and is reported. |
| text  | string |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| kind  | enum   | `test · non-test` -- **the DISCRIMINATOR for `state`'s shape below**                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| state | tagged | `{is: computed}` · `{is: unsatisfied, note?}` · `{is: satisfied, evidence}` (evidence non-empty) · `{is: descoped, to: STxxxx, by?, reason?}` · `{is: withdrawn, reason, by?}` · `{is: fiat, because, by, at, invoker, inherited_from?, inherited_event?}` -- **REQUIRED on every criterion.** `computed` is the in-scope value for a `test` criterion; `satisfied`/`unsatisfied` are refused on one. `computed` RATIFIED by hv 2026-08-15 (see "The fifth state"), `fiat` by hv 2026-08-29 (Machine 3)            |

**The tag is `is`, not `state`** (cc, 2026-08-15), so the extract reads `"state": {"is": "satisfied", "evidence": "..."}` rather than doubling the word. **Nesting rather than `#[serde(flatten)]` is forced rather than chosen**: flatten and `deny_unknown_fields` do not compose in serde, and D05's refuse-unknown-fields posture wins over a flatter shape. Worth recording because the flat form is what anyone would reach for first, and the reason it is absent is not taste.

**Replaces the pre-ratification `scope` object + `satisfied: bool?` pair** (2026-08-15). The ratified Machine 3 collapses two fields into one enum, which is what kills "three stored values, two meanings, one never written" by construction.

#### The JSON form differs by AC kind, and `kind` is the discriminator (ruling, vc, 2026-08-15)

**Asked by cc before cutting the collapse, which is the cheap moment.** Two candidate forms were put to me: an **absent `state` key** on a test-backed AC (smaller diff), or a **discriminated shape** where the absence is structural. **Ruled: discriminated, on `kind`.** Three grounds, and the first is decisive on its own.

> **REVERSED THE SAME DAY, on cc's implementation, which I found by checking rather than being told.** A third form existed that was not on the table when I ruled: **`computed` as an explicit fifth state value**, already built in `transitions.rs` and `mutation_completeness.rs`. **It satisfies my own two strongest grounds BETTER than the form I chose**, so the grounds stand and the verdict does not. See "The fifth state" below. What survives untouched is the second half of the ruling -- the data-loss correction -- because it never depended on which encoding won.

**1. Under the absent-key form, `state` must be optional for EVERY criterion -- so a non-test AC that LOST its state validates cleanly.** Absence would carry two meanings: "computed, by construction" and "the field went missing". That is data loss indistinguishable from correctness under D05's strict validation, and it is the fourth appearance of the class this thread keeps meeting -- `event_log`'s missing artefact, `file_index`'s missing exemption, ic's banner-sniffing backstop, vc's `hooksPath` grep. **AC-02.6's posture is already the answer: absence is never the answer.** With `kind` declared, absence becomes decidable rather than ambiguous: `state` is REQUIRED on `non-test` and its absence is a refusal.

**2. `kind` is ALREADY a modelled enum**, so the discriminated form adds no field and the "smaller diff" argument mostly evaporates. cc framed this as adding a discriminator; the model has carried one since the 0013 work.

**3. AC-02.6 requires the file form to be usable WITHOUT Intent.** Under the absent-key form an external reader must reimplement the rule "if kind is test then satisfaction is computed from the covering ATs, else it is stored" before it can read the data correctly. A self-describing shape needs no such transfer, and "use my data somewhere else" is hv's stated requirement, not a preference.

#### The correction cc's question needs, and it is bigger than the question

**"Test-backed ACs store no state at all" is TOO STRONG and would lose data.** Read Machine 3 again: `ac descope`, `ac withdraw`, `ac rescope` and `ac reinstate` carry **no kind guard**. Only the `Unsatisfied <-> Satisfied` edges are kind-restricted, and only because for a test-backed AC they are consequences of AT status rather than verbs.

**So a test-backed AC that has been DESCOPED must store that.** It is a scope decision no amount of AT status can recompute, and under D34 a state the extract cannot represent is data loss at the clone boundary, not a gap. The stored state therefore has two axes with different rules:

| axis              | values                                | stored for                                             |
| ----------------- | ------------------------------------- | ------------------------------------------------------ |
| scope disposition | `in-scope` · `descoped` · `withdrawn` | **both kinds** -- authored decisions, not recomputable |
| satisfaction      | `satisfied{evidence}` · `unsatisfied` | **non-test only** -- computed for test-backed          |

The ratified enum flattens both axes into four mutually-exclusive values, which is correct as a machine. **The storage rule is per-axis, and that is what the JSON form has to express.** Consequence for the schema: on a `test` criterion, `satisfied` and `unsatisfied` are **REFUSED in the stored form** -- storing either is precisely the double truth the collapse exists to remove -- while `descoped` and `withdrawn` are required to round-trip.

**Not a change to the ratified machine, and deliberately not**: it specifies the file form the machine implies. If hv reads it as altering Machine 3, the machine wins and this paragraph is the thing that is wrong.

#### The fifth state: `computed` -- RATIFIED by hv 2026-08-15

> **RATIFIED (hv, direct, 2026-08-15): "Ratified".** `computed` is a state of Machine 3. The section below is kept as written because the reasoning is the record of how a divergence between a ratified machine and its implementation was found and resolved -- but the open question it ends on is now closed, and `mutation_completeness.rs` measures hv's ratification rather than cc's transcription.
>
> **What this closes, and it is the reason it was escalated rather than absorbed:** for about 75 minutes the conformance test that proves the code implements the ratified machines was asserting against **one node's reading of them**. A green there meant "cc and cc agree". Two artefacts transcribed by one author from one document in one session are not two witnesses -- they agree with each other and both differ from the source.

**Machine 3 as first ratified had FOUR values. The implementation had five.** `transitions.rs` declares `initial: &["computed", "unsatisfied"]` and `mutation_completeness.rs` transcribes `ac.rescope` and `ac.reinstate` as **two edges each**, landing on `AcState::entry(kind)` -- `unsatisfied` for an authored criterion, `computed` for a test-backed one. cc wrote the divergence into the comment rather than hiding it: _"That is not a transcription error ... the ratified table's single `-> Unsatisfied` row is written for the authored criterion it had in mind."_

**The problem cc hit is real and the ratified table does not answer it.** `ac rescope` on a **test-backed** AC has to land somewhere, and landing it on `Unsatisfied` stores a satisfaction claim about a criterion whose satisfaction is computed. There is no fourth value that fits.

**And `computed` beats the form I ruled, on my own grounds:**

- **Ground 1 (a non-test AC that LOST its state must not validate).** Under `computed`, `state` is **REQUIRED on every criterion** -- there is always a value -- so a missing `state` is a refusal for both kinds. Under my absent-for-test-backed form, absence had to be permitted schema-wide, which is the hole I was arguing against. **cc's form closes it more completely than mine did.**
- **Ground 3 (usable WITHOUT Intent).** `{state: computed}` says on its face that this criterion is derived. My form required a conditional -- "absence is legal iff `kind` is test" -- which is precisely a rule an external reader would have to reimplement. **I argued against transferring a rule to the reader and then chose the form that transfers one.**
- Ground 2 (`kind` already exists) was never an argument for absence; it survives as the reason `kind` stays.

**The honest cost, stated because it is the only thing in cc's favour I am not counting: two fields can express nonsense.** `{kind: non-test, state: computed}` and `{kind: test, state: satisfied}` are representable and meaningless. The API refuses both (`Guard::NonTestOnly` on satisfy/unsatisfy), and the published JSON Schema face refuses them too (the `allOf` on `Criterion`, `model.rs:1145`, held by `tests/ac_kind_state_invariant.rs`), so the extract cannot carry a combination ingest would reject.

### acceptance_test (inside the thread canon)

| Field  | Type    | Notes                                                                                                 |
| ------ | ------- | ----------------------------------------------------------------------------------------------------- |
| id     | string  | `AT-01.1`                                                                                             |
| kind   | enum    | `test · non-test`                                                                                     |
| file   | string? | test kind: repo-relative path (the 0017 reference rules)                                              |
| prose  | string? | non-test kind: what was read/eyeballed                                                                |
| covers | array   | AC ids                                                                                                |
| status | enum    | `to-write · red · green · n-a · fiat` (`n-a` non-test only; see Machine 5)                            |
| fiat   | object? | the fiat-close record, present exactly when `status` is `fiat`; it sits BESIDE the status (Machine 5) |
| note   | string? | the free trailing note                                                                                |
| legacy | object? | the marked-legacy carry form (see below)                                                              |

#### The marked-legacy AT form (the closed-thread carry policy's model consequence)

Required by the hv carry ruling in `migration.md`: CLOSED threads convert lossless-by-carrying, so a legacy-grammar AT row must land in the model whole, with nothing guessed, dropped or reformatted.

##### `legacy` (carried beside a typed field the enum cannot express)

| Field | Type   | Notes                                                        |
| ----- | ------ | ------------------------------------------------------------ |
| raw   | string | the verbatim v2 reference, exactly as it appeared on the row |

Absent on every row authored under the v2.19 grammar.

**AND THE CARRIER IS THE INTENDED PATTERN, APPLIED ON ONE AXIS ONLY -- which is why this is a table and not a sentence.** `WorkPackage::scope` is `Option<TShirt>` and absent exactly when `scope_legacy` holds something the enum cannot say, so scope PRESERVES what it could not parse. **Its sibling `status` has no absent form**, so it keeps the value it was read as and `status_legacy` carries what v2 actually wrote beside it (issue 0100) -- the same carry on the other field the migrator cannot always map.

`raw` is the **verbatim v2 row text**, byte-for-byte as it appeared, carried and never parsed. When `legacy` is present, `file` may be absent -- a `::name` citation or a multi-file list has no single repo-relative path, and inventing one is precisely the destruction the 0017 refusal was about.

The distinction that makes this safe: carrying a whole row into a richer model destroys nothing, where a fixer that rewrites one end of a two-ended reference destroys the link. Migrating data and improving it are different operations, and only the first one is the migrator's job.

Consequences that must hold together:

- Ingest accepts `legacy` (it is in the schema from the start, so `thread.schema.json` is blessed once, not twice).
- `legacy` is **carried, never interpreted**. No command reads `raw` to answer a question; anything that did would be the v2 "answers confidently from partial evidence" class rebuilt inside v3.
- A row carrying `legacy` is reported as carried-legacy in coverage views, never silently counted as an ordinary green.
- LIVE threads never produce one: they stay BLOCKED-until-clean, so a `legacy` row appearing in a live thread is itself a defect.

#### Figure provenance: MEASURED versus RECORDED (ruling, vc, 2026-08-19, on findings by dc and ic)

**THE FIX FOR AN UNVERIFIABLE FIGURE IS NEVER A BETTER FIGURE.** A number written into a durable record reads as a claim about a measurement, and nothing in this model distinguishes a figure that can be re-derived from one that was observed once and cannot. That gap is not theoretical: the instances below surfaced on 2026-08-19, none of them found by looking.

- **An `EXAMINED` figure in AC-00.11.** It named no revision. ic reproduced the defect but not the figures, and could not determine whether the row was wrong or whether it named a revision they had not swept. **That inability is the defect, not a step on the way to finding it.**
- **A second figure in AC-00.11.** The nominating probe was never committed: no tool, no board history carrying its text, and dc's reconstruction returned a different number. A recorded number wearing a derived number's clothes.
- **ST0011's `completed`.** The value survives only in an undeclared home (body prose); the declared home was empty and the carry read the declared home correctly. Two homes for one fact, silent until they disagreed.

Both peers declined to hand over replacement numbers, independently and for the same reason: **a replacement mints a third figure on a fresh decay schedule and retires nothing.**

The form:

```
MEASURED   <figure> @ <revision>      -- reproducible; anyone can re-derive it
RECORDED   <figure> (RECORDED)        -- observed once; explicitly not reproducible
```

`<revision>` is a commit sha. Not a date, not a branch, and never `HEAD` -- `HEAD` names a different tree tomorrow, which is the property being defended against. A figure carrying neither mark is a defect.

**WHY THIS IS A PROSE CONVENTION AND NOT A JSON FIELD.** These figures live inside criterion text and AT notes, embedded in argument. A structured field beside the prose would be a second home for one fact, on its own decay schedule, repaired independently -- which is the exact failure this section exists to name, rebuilt one level up. **The mark lives where the figure lives.**

**DISTINCT FROM `legacy`, AND THE TWO MUST NEVER MERGE.** They are adjacent in this document and they answer different questions:

- `legacy.raw` answers **this row came from the v2 grammar and we did not parse it**. It is about MIGRATION, and a row authored under the v3 grammar never carries one.
- A provenance mark answers **this figure was observed once and cannot be re-derived**. It is about PROVENANCE, and a row authored today under the v3 grammar can need one. AC-00.11 is precisely that: minted 2026-08-18, no `legacy`, and unverifiable figures.

Merging them would admit members under a key named for another reason, and a marked-legacy row would lend its migration excuse to a figure that has none. **They share exactly one rule, and it is the load-bearing one in both: a marked thing is never silently counted as an unmarked one.** A `legacy` row is reported as carried-legacy and never as an ordinary green; a RECORDED figure is never counted as a measurement.

**WHAT THE MARK DOES NOT DO.** It does not make the figure true, and it is not an apology for it. It stops a reader spending an afternoon reproducing something unreproducible, and it stops a one-off observation being read as a measurement by the next person to build on it. ic's sentence is the specification and is better than any restatement of it: _I cannot tell you whether the row is wrong or whether it is a revision I have not swept, which is precisely the point._

**THIS SECTION IS THE FORM. IT IS NOT THE ENFORCEMENT.** A convention recorded in prose is invariant under nothing -- only a mechanism that refuses is invariant under copying, and a document's source is its own output. The criterion and its instrument are separate work and are what make this bind.

**THE POPULATION IS DELIBERATELY NOT SIZED HERE.** The form is minted before the population can be, and sizing it is not a precondition for minting it. A sweep run now would size the remedy to the reach of the probe that ran it, which is how a partial count becomes a total.

### issue (`intent/.canon/issues/<nnnn>.json`, body included)

| Field    | Type    | Notes                                                                                                     |
| -------- | ------- | --------------------------------------------------------------------------------------------------------- |
| schema   | string  | `intent/issue@3.0`                                                                                        |
| number   | int     | rendered `0021`                                                                                           |
| slug     | string  |                                                                                                           |
| title    | string  |                                                                                                           |
| status   | enum    | `open · closed` -- directories stop encoding status (parity deviation, see parity.md)                     |
| severity | string? | `critical · high · medium · low`, refused at the door on write; an out-of-roster stored value still reads |
| created  | date    |                                                                                                           |
| closed   | date?   |                                                                                                           |
| reporter | string? |                                                                                                           |
| body     | string  | the authored issue text, carried verbatim (see below)                                                     |

**The body lives in the JSON (hv ruling, 2026-08-18).** Before the ruling the spec named a sibling authored `issues/<n>.md` while `schema/issue.schema.json` declared `body`, and neither was implemented: no canon file carried a `body` key, so the authored issue text lived ONLY in `intent/.cache/intent.db`. That path is gitignored, so by D29 the content was never canon and by D34 it never travelled -- **absent from every clone, and nothing anywhere said so.** It surfaced only because `sync --to-store` replaced the store from an extract that had never carried it.

The issue is self-contained the way a thread is and round-trips under AC-02.6. **A sibling authored file whose canon record does not hold it is the two-ended shape this thread has now paid for twice.** v3 renders no per-issue `.md`; `intent issues show` reads the issue from the store.

**The bodies were recovered from a store snapshot and proved interchangeable with the v2 originals rather than assumed:** every snapshot body compared byte-for-byte against `9b73e98f`'s authored v2 originals with the YAML frontmatter stripped, and every one was identical, the per-file gap being exactly the frontmatter block the migrator parsed into fields.

### event_log (DB-only, append-only; D15)

Written by every mutation (WP-02).

| Field      | Type   | Notes                                                                                                       |
| ---------- | ------ | ----------------------------------------------------------------------------------------------------------- |
| id         | ulid   | lexically sortable, globally unique                                                                         |
| ts         | string | RFC 3339 UTC, MILLISECOND precision: `YYYY-MM-DDTHH:MM:SS.sssZ`                                             |
| principal  | string | who performed the operation                                                                                 |
| project_id | string | stamped at migration (D15); never changes                                                                   |
| op         | string | the FACADE operation, eg `st.done` -- **not** a CLI invocation                                              |
| subject    | object | `{kind, id}` -- kind eg `thread`, `wp`, `issue`; id eg `ST0000/02`; stored as `subject_type` / `subject_id` |
| payload    | json   | operation-specific detail, **opaque to the log**                                                            |

**AND A CASCADE WRITES EVENTS NAMED FOR OPS NOBODY INVOKED, WHICH IS HONEST AND READS AS WRONG.** A fiat close on a thread writes `st.fc` and then one `ac.fc` or `at.fc` per child it reached -- events no invocation of `ac fc` or `at fc` produced. That follows from `op` being the facade operation: the child's state moved through the criterion machine, so `ac.fc` is what happened to it. **It is stated here because a reader who concludes the log is lying about provenance will not check twice**, and the thing that distinguishes a cascaded event from a directly-invoked one is the `inherited_event` in its payload, not the op.

**`op` is the FACADE operation and never the CLI spelling**, which is the same distinction Machine 5 records one section down: `at.set` is one op reached by `at green`, `at red` and `at na`, and `fc` is one command reaching one op per target kind (`st.fc`, `wp.fc`, `ac.fc`, `at.fc`). A log keyed on invocations would be a log of what people typed rather than of what the store did.

**`ts` IS ABSENT ON A MINTED-BUT-UNWRITTEN ENVELOPE, AND THAT IS D42 IN THE ONE PLACE IT IS EASIEST TO BREAK.** `Envelope::minted` deliberately produces an envelope with NO time, because the clock belongs to the WRITE -- a log entry is the last thing anyone would think to withhold a timestamp from, and it is exactly where a caller-supplied time would become unfalsifiable history. The number of `minted` call sites is deliberately not written here; it is a thing to derive, not to keep a second copy of.

**The DB is the durable SSOT, so the event log in it is durable truth like everything else there** (D01 as reversed). **It has ONE home and it is the store (D53)**: no events file is kept in the working tree. `intent events` queries it (`--op`, `--subject`, `--limit`), and `intent export` produces the lossless `events.jsonl` form on demand. The read-only query door is `intent graphql`, through intentd; there is no SQL surface. Read-only is the boundary that matters: write-SQL would be a second door into the SSOT, and the typed API being the only door is the whole reason the DB's contents conform by construction.

### file_index (DB-only -- the sync engine's git-style index)

`{path, size, mtime, sha256, state: clean · changed · unparsed, findings[]}`. Scope: `intent/**` + named root files.

### doc_section (DB-only, from prose ingest)

`{owner_type, owner_id, file, seq, heading, level, body}` -- FTS5-indexed; powers `intent search`. Prose bodies are stored verbatim, never modelled. A text attachment contributes one unsplit section.

### wb_node / wb_item / wb_message (`whiteboard/<node>/board.json`; D30, WP-14)

The coordination entities, **specified and NOT BUILT in 3.0.1**: WP-14 was cancelled with its criteria descoped to ST0069, so the store has no whiteboard tables and `intent/whiteboard/` stays hand-authored markdown on disk. The migrator reports it as modelled-but-unbuilt and leaves the files untouched. The specification: durable form is committed JSON canon per D01; `wip.md` and `inbox.<sender>.md` become generated views per D02, ending the hand-authored board.

| Entity       | Fields                                                                                                                          |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| `wb_node`    | `moniker` (PK), `name`, `role`, `session_id?`, `heartbeat_at`, `status` (`active · paused`), `focus`, `claims[]`                |
| `wb_item`    | `node`, `kind` (`doing · todo · decision · watchout`), `seq`, `text`, `state` (`live · archived`), `created_at`, `archived_at?` |
| `wb_message` | `sender`, `recipient`, `sent_at`, `body`, `re?` (prior anchor), `fyi` (bool), `state` (`live · handled`), `handled_at?`         |

Three properties are the point of modelling these rather than parsing them (D30):

- **Timestamps are stamped by the database on the write** (D42), never supplied by the caller, so a fabricated stamp stops being constructible rather than being detected after the fact.
- **Bounds are enforced on write and refused by name.** Per-entry body size, live items per node per kind, and live messages per inbox are configured, and an over-bound write is refused with the bound and the remedy stated -- the D05 posture applied to size, never truncation, never a silent accept.
- **`state` transitions are the API's**, so archival happens on schedule rather than when a node remembers, which is what produced 251KB of `.history`.

The header block stays line-oriented `key: value` in the rendered view (D13) -- it is generated from `wb_node` rather than parsed into it.

## Generated views: the renderer has no clock

Every generated view carries a generated-banner footer instead of a verblock (the AGENTS.md pattern). One constraint governs the banner and every other byte a renderer emits:

**No generated view contains a render-time value, and the view renderer has no clock.** Its inputs are the model and the tool version -- no clock, no locale, no `$USER`, no `$HOSTNAME`, no absolute paths, no environment. The banner names the tool version and what the view was rendered from; it never names when it ran. Git already records when a view was regenerated, and it does it correctly.

This is derived from the contract, not a preference. AC-03.2 requires a view to render the same bytes twice; AC-03.4 requires regenerate-and-diff to come back empty. A view that stamps its own render time fails the first and makes the second diff every file on every run -- so the skew check becomes either useless or trained-to-be-ignored, which is the same outcome arriving later.

The live v2 instances, found at `f7434f1` when cc raised the first one:

| Instance                       | Value                                              |
| ------------------------------ | -------------------------------------------------- |
| `intent/todo.md`               | `## DONE:2026-07-10T17:18:19Z`                     |
| `AGENTS.md`                    | `_Generated by Intent v2.19.0 on 2026-08-14_`      |
| `lib/templates/llm/_CLAUDE.md` | `on [[DATE]] for Intent v[[INTENT_VERSION]]` (:59) |

The template instance is why this is a law rather than a fix: the banner pattern this document ratifies for every v3 view is itself one of the instances, so repairing the reported file would have left the defect in every view not yet written. Fix the class by removing the capability.

`todo.md`'s `DONE:<ts>` is data, not a render stamp: it is the flush watermark, held in project canon and rendered from it (see "The todo watermark" above).

## Canonical JSON form

UTF-8, LF, 2-space indent, trailing newline, object keys in schema-declared order (serde struct order, not alphabetical), arrays in natural order. Tool-written always; hand-edits are legal but validated strictly on ingest (D05).

## The schema face: `schema/thread.schema.json`

The WP-01 draft schema that stood here is **pruned**. WP-02 landed the schemars face, so the supersession this document declared at the top has already happened, and a second copy of the schema in prose is the divergent-copy drift Highlander exists to stop -- proved in the act: the draft went stale the moment `objective`, `context`, `related` and `legacy` were added above, and a reader building from it would have built the wrong type.

The authored master is the Rust type layer (`native/rust/crates/intentsvcs/src/model.rs`). The committed faces are generated from it into `schema/` at the repo root -- `thread.schema.json`, `issue.schema.json`, `event.schema.json`, `ddl.sql`, `schema.graphql` -- by `native/rust/crates/intentsvcs/src/faces.rs`, and `native/rust/crates/intentsvcs/tests/schema_faces_drift.rs` fails CI on any diff. Re-bless deliberately and in the same commit as the type change:

```
INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift
```

Read the face for the schema; read this document for why the model has the shape it has.

Validation posture (D05): `additionalProperties: false` everywhere -- an unknown field is refused by name, never dropped (the serde_ignored discipline at the schema layer). Schema evolution bumps the published faces' version (D41) and the store's `SCHEMA_VERSION` (stamped as `PRAGMA user_version`); an existing store is migrated in place by the versioned ladder in `store.rs`. Migrations are normal.

## What is deliberately not modelled

Prose (stored verbatim, FTS-indexed). Rules/skills/templates (shipped content -- templates compiled into the binary, rules and skills read from the install tree -- indexed at most). wip.md / restart.md (authored tracking prose -- the project-level pair at `intent/`, not the whiteboard's per-node boards).

This set is what an `intent export` cannot reproduce from the DB alone.

**The whiteboard left this set at D30** (hv ruling, 2026-08-15) and is modelled above as `wb_node`/`wb_item`/`wb_message`. That model is not built in 3.0.1 (WP-14 cancelled, descoped to ST0069), so `intent/whiteboard/` is reported as modelled-but-unbuilt rather than as out of model, and its files stay on disk untouched.

## State machines (RATIFIED by hv, 2026-08-15)

> **RATIFIED.** hv answered the four open questions on 2026-08-15: (1) `st new` enters at **`Triage`** -- yes; (2) `wp done` is **refused** on a BLOCKED gate **and** `doctor` reports any unit whose status disagrees with its gate -- both, as recommended; (3) **no** `Hold`/`Cancelled` at WP level -- confirmed; (4) a test-backed AC is **never** `satisfy`-ed by hand and the AC machine therefore has two variants -- confirmed. **The `Tbc` -> `Triage` rename is ratified by hv's use of the name in answer (1)**; it is stated here rather than inferred, so a disagreement surfaces now rather than at a WP close.

> **AMENDED by hv, 2026-08-18: `st.start` is legal from `Triage` as well as from `NotStarted`.** Answer (1) stands -- `st new` still enters at `Triage` -- and the reason it was right is untouched: `Triage` is not v2's `TBC` renamed, so it begins with zero legacy members and no thread is given a triage decision nobody made.
>
> **What changed is that the ruling was made in this document and then met by a human for the first time on 2026-08-18, and the ratified path cost TWO EXTRA COMMANDS on the only route anyone actually walks.** `st new` then `st triage` then `st start`, where v2 was `st new` then `st start`. hv, on typing it: _"this is STOOPID... I'd expect it to just end up at WIP."_
>
> **The project had already recorded the ADJACENT drift and missed this one.** design.md's standing-obligation note (ic, EXP-04) records that `st new -s|--start` came through the parity register as `keep` while _"the meaning moved, because `st new` now enters at `Triage` and the flag spans two transitions instead of one"_. **The semantic cost of Triage-as-entry was predicted; the ergonomic cost was not, and nothing short of a human typing the sequence would have surfaced it** -- neither the test suite nor this estate's migration did, because neither exercises "a person creates a thread and starts working on it".
>
> **Why this edge rather than defaulting `st new` to `--start`:** starting work on a triaged item IS accepting it -- `st triage` already prints `accepted out of triage` -- so `Triage -> Wip` is a legitimate compound rather than a bypass, and the guard is gained rather than weakened. Defaulting `st new` to `--start` would instead leave `Triage` with almost no population, which is the state's whole justification.

Drafted on hv's instruction after the `TBC` / `On Hold` / `satisfied` rulings: _"we will obviously need state toggles and a state machine process that moves threads programmatically thru the states. So we should take a beat now and define the states and the legal transitions."_ They were ratified as recorded above; `transitions.rs` transcribes every machine below, and the commit gate holds the two together.

### Why now, with live evidence

Measured 2026-08-15 in this thread's own tracking data -- **several WPs had a status that disagreed with their own gate**:

| WP    | `status:` | gate verdict      |
| ----- | --------- | ----------------- |
| WP-02 | **Done**  | BLOCKED (AC-02.6) |
| WP-03 | WIP       | BLOCKED (AC-03.9) |
| WP-04 | **Done**  | BLOCKED           |
| WP-05 | WIP       | **PASS**          |
| WP-06 | WIP       | BLOCKED           |

Two of those were caused by vc adding ACs to a closed WP: **`wp done` existed and nothing undid it, so a WP reopened in the contract kept saying `Done`.** That is AC-04.6's own defect class -- a state entered and not leavable -- occurring live, in the tracking tool, committed by the verifier enforcing the rule that names it. WP-05 was the inverse: gate PASS, status WIP, because nothing moves a status forward on evidence either.

### Three findings that shape the draft

1. **`TBC` already means "To Be Commenced", not "to be confirmed".** v2's `bin/intent_helpers` mapped `"tbc"` and `"to be commenced"` to the SAME canonical value, `Not Started`. So hv's new meaning -- the raw pre-triage state -- is a **new meaning for a token that is already spoken for**, in every v2 document and every user's head. **Recommendation: name the state `Triage`, not `Tbc`.** Reusing a token for a second meaning is the defect class this thread has spent two days removing, and migration then has an unambiguous rule: every v2 `TBC` is `NotStarted`, and `Triage` starts with no legacy members.
2. **In v2, `Hold` was reachable only by hand-editing a file.** It was recognised by the status filter (`hold, on hold -> HOLD`) and no verb set it. cc's archaeology, confirmed.
3. **In v2, `Completed` and `Done` were one-way doors.** No `reopen` at either level.

**CORRECTED 2026-08-17 (vc, measured -- issue 0046). FINDING 3 IS FALSE, AND IT IS FALSE IN THE DIRECTION THAT MATTERS: the doors are not one-way, they are unlocked and unlabelled.** v2's `intent wp start` on a `Done` work package wrote `WIP` over it (an unconditional `sed`, no read of the current status), and `intent st start` on a `Completed` thread did the same **and relocated the directory out of `COMPLETED/`**. Both at exit 0, both printing the sentence they print for work that was never done. Measured in a throwaway project, not read.

**Three consequences for the machines above, none of which change a ratified table.**

- **v2 had an UNDECLARED EDGE at both levels.** Machine 1 gives `st start` exactly `NotStarted -> Wip`; Machine 2 gives `wp start` exactly `NotStarted -> Wip`. **`Completed -> Wip` and `Done -> Wip` belong to `reopen`, with `reason recorded` as the guard.** AC-04.6's strengthened form forbids an undeclared edge in as many words, so **`start` classified `keep`/`as-observed` would ship one**, and `AT-04.6`'s walk is the test that should say so.
- **The stated cause of the live inconsistency is wrong.** Machine 2 calls `wp reopen` _"the one whose absence is causing the live inconsistency above"_. **The transition was never absent.** What produced the disagreement is that criteria changed under closed units -- AC-04.6 ADDED on hv's D32, AC-04.1 STRENGTHENED -- which is the same defect the `doctor` recommendation below addresses and is not a missing verb at all.
- **The design premise survives and gets sharper.** **The thing missing was never the move -- it is the RECORD.** A reason-carrying door beside an open, unlabelled one is worth building only if the unlabelled one is closed at the same time: **`start` must REFUSE a terminal state and name `reopen`.** As built, `st reopen` and `wp reopen` ship, and `st start` on a `Completed` thread or `wp start` on a `Done` package is refused at exit 1 as an illegal transition. The refusal's remedy names `start`'s declared from-states; it does not name `reopen`.

### The rules these machines obey

- **No terminal states.** Every state has at least one declared exit (D32/AC-04.6). A state that should be hard to leave gets a **guard**, not a missing verb.
- **Every transition names a verb**, reachable from every surface (D32).
- **Guards are declared, not implied** -- eg `Completed` requires a gate PASS.
- **Direct vs Incidental** (cc, 2026-08-15): an edge that exists only as a side effect of changing a different field counts for reachability and **never discharges a trap**.
- **SELF-LOOPS ARE LEGAL, AND THEY ARE ACCEPTED-AND-REPORTED AT EXIT 0 WITHOUT RE-RUNNING THE GUARD** -- RATIFIED (hv, 2026-08-17, on vc's recommendation; applies to every machine). Asking a verb for the state an entity is already in is not a movement, so it is not a transition to declare and not an illegal one to refuse. **This CHANGES Machines 1-3 as previously ratified**, which refused a self-loop as `IllegalTransition`, and it brings v3 back to v2's measured behaviour: `intent issues close` on a closed issue returns 0 with `already CLOSED`.
  - **"Without re-running the guard" is the load-bearing half, not a performance note.** `Completed` requires a gate PASS. Re-running the gate on `st done` against an already-completed thread would let a criterion added AFTER the close BLOCK a thread that is legitimately finished -- which is precisely the live inconsistency recorded below, where AC-04.6 was added under closed units on hv's D32. **A self-loop must not be able to fail for a reason that did not exist when the state was entered.**
  - **The exit code follows from that**: refusing self-loops makes every idempotent script a special case, and idempotence is the property callers actually want from `done` / `close`.
  - **This does NOT license the 0046 class, and the distinction is exact.** `wp start` on a `Wip` work package is a self-loop and is now accepted at 0. `wp start` on a `Done` work package is `Done -> Wip`, an UNDECLARED EDGE belonging to `wp reopen` with `reason recorded` as its guard, and it stays refused. A ruling that self-loops are legal says nothing about movements, and reading it as amnesty for undeclared edges would reintroduce the two-doors defect that issue 0046 is about.
  - **THE PREDICATE (vc's reading of hv's ruling, 2026-08-17; the shape found by cc while implementing it).** **A verb applied to an entity already in that verb's TARGET state is a self-loop, and is accepted at exit 0 without running the verb's guards. Whether the verb is DECLARED from the current state is a separate question, and it is asked only when the current state differs from the target.**
    - **Recorded as a reading rather than as ruling text, because it is not derivable from the ruling.** hv ruled the behaviour; the predicate is the form that makes it decidable at a call site, and it should be argued with as vc's, not deferred to as hv's.
    - **The second clause is what keeps 0046 refused**, and it is why "is the verb declared from here" cannot be the test. `wp start` on `Done`: the target is `Wip`, `Wip != Done`, so it is not a self-loop; it goes to the declared-edge test and fails there. Test the verb's TARGET, never its declared origins -- a verb declared from many states would otherwise self-loop from all of them.
    - **`transitions.rs` transcribes this; it does not restate it.** A predicate living only in the test that exercises it is the declaration-enforced-by-hand shape deleted from `facade.rs` on the same day -- `Guard::GatePass` was declared and hand-enforced in two call sites, so deleting the declaration changed nothing. AC-04.6 holds the pair.

### CLOSURE IS MEASURED AT THE FACADE, AND SURFACE REACHABILITY IS A SEPARATE QUESTION (ruling, vc, 2026-08-17, on ic's issue 0052 and cc's `WorkPackage.scope` reasoning)

**Both nodes were right about different layers, and the disagreement only looked like one because neither statement named its layer.** cc's `transitions.rs` comment justifies `WorkPackage.scope` as a `State` on the grounds that _"`wp_new` takes the size from the caller"_, so every size is genuinely ENTERED. **That is true**: `facade.rs:5568` is `pub fn wp_new(&mut self, st: &str, title: &str, scope: TShirt)`. ic's 0052 says the premise is false, because `wp new` has no options. **That is also true**: the dispatch table declares `wp new` with two args and no flags, and `render.rs:2582` passes `model::DEFAULT_WP_SCOPE` (`S`, v2's default).

**So the scope machine is fully reachable through the typed API and only partly through the CLI: `wp new` enters one initial value, and `wp rescope` is the one verb that moves it afterwards.** `State` stays -- the disposition is correct and so is cc's reasoning, including the `absent`-by-ingest value, which is the entry route this AC's second condition names.

**The general rule, and it is why this belongs here rather than in the issue.** The mutation walk drives the FACADE. So `no_state_can_be_entered_and_not_left` and the edge-for-edge walk establish that a machine is closed **to the API**, which is not the same claim as closed **to an operator** -- and nothing in the walk can tell the difference, because the walk is an API caller. **A field can satisfy every closure condition in this document while the surface offers no way to reach most of its values.** AC-04.6 is an API-closure criterion; surface reachability is the dispatch table's question and needs its own evidence. Neither answers the other, and a green walk must not be read as either.

**`WorkPackage.scope` therefore has no ratified machine table below and does not need one.** It is the T-shirt enum with a marked-legacy carry, ruled above as an attribute of the package; its `State` disposition records that the values are ENTERABLE, not that the sizes form a lifecycle. The edge-for-edge walk covers every field with a ratified machine below, and **scope's exclusion from it is by design and is now stated rather than left as an omission**, because an omission and a gap are indistinguishable to the next reader.

### Machine 1 -- Steel thread (`ThreadStatus`)

States: `Triage` (proposed rename of `Tbc`) | `NotStarted` | `Wip` | `Hold` | `Completed` | `Cancelled`. **Entry: `Triage`.**

| From         | To           | Verb           | Guard              |
| ------------ | ------------ | -------------- | ------------------ |
| _(none)_     | `Triage`     | `st new`       | --                 |
| `Triage`     | `NotStarted` | `st triage`    | --                 |
| `Triage`     | `Wip`        | `st start`     | --                 |
| `Triage`     | `Cancelled`  | `st cancel`    | reason recorded    |
| `NotStarted` | `Wip`        | `st start`     | --                 |
| `NotStarted` | `Hold`       | `st hold`      | reason recorded    |
| `NotStarted` | `Cancelled`  | `st cancel`    | reason recorded    |
| `Wip`        | `Completed`  | `st done`      | **`ac gate` PASS** |
| `Wip`        | `Completed`  | `st.fc`        | reason recorded    |
| `Wip`        | `Hold`       | `st hold`      | reason recorded    |
| `Wip`        | `Cancelled`  | `st cancel`    | reason recorded    |
| `Hold`       | `Wip`        | `st resume`    | --                 |
| `Hold`       | `Cancelled`  | `st cancel`    | reason recorded    |
| `Completed`  | `Wip`        | `st reopen`    | reason recorded    |
| `Cancelled`  | `NotStarted` | `st reinstate` | reason recorded    |

**The verbs this machine needed beyond v2 -- `st triage`, `st hold`, `st resume`, `st reopen`, `st reinstate` -- all ship.** `st new` enters at `Triage`, and `-s|--start` composes declared transitions to land at `Wip` (below).

#### `st new -s|--start` -- a convenience flag COMPOSES declared transitions, it never introduces an edge (ruling, vc, 2026-08-15)

**Flagged by ic as "two edges at once", handed to vc and cc by hv.** The measurement settles most of it: **`-s|--start` is v2 parity, not new surface** -- `st new [-s|--start] <title>` in v2's own help, and the register carries it `keep`.

**Nothing about the flag changed. The machine grew a state underneath it.** In v2, `st new` landed at not-started, so `-s` was ONE transition. In v3, `st new` enters at `Triage`, so the same flag now spans **two**: `Triage -> NotStarted -> Wip`. A `keep` disposition is honest about the surface and silent about the semantics, which is exactly the kind of drift the register cannot see.

**Ruled: keep the flag, and it performs BOTH declared transitions in sequence.** The triage decision is not being skipped -- **a user who types `--start` has decided the thread is real work, which IS the triage decision, made explicitly by the same act.** Refusing the flag would ask them to state a conclusion they have already stated.

**The load-bearing constraint, and it is where the natural implementation goes wrong: `st new -s` must COMPOSE `st triage` and `st start`, never construct the thread directly in `Wip`.** Building the end state is the obvious way to write it and it produces a state history with no triage event, and it drives construction around `transitions.rs` entirely, contradicting D32's "no surface mutates state except through a service call".

**Discriminating test: after `st new -s`, the event log carries BOTH transitions.** A test asserting only the final status passes on the defect, and the defect is invisible from the outside because the resulting status is correct either way. As built, `intent st new -s` writes `st.new`, `st.triage` and `st.start`, in that order.

**The general rule, for anything that bundles: a convenience flag is sugar over declared transitions and never a new edge.** If a bundle cannot be expressed as a sequence of declared transitions, the bundle is proposing a machine change and goes to hv as one.

**THE `st.fc` ROW IS hv's D1 OF 2026-08-29, AND IT SITS DELIBERATELY BESIDE `st done` RATHER THAN REPLACING IT.** Same from-state, same landing state, and **`reason recorded` where `st done` has the gate** -- because a fiat close is BY DEFINITION the case where the gate does not pass, so guarding this edge with the gate would make it unreachable at exactly the moment it is for. **The option hv declined was relaxing `st done` to accept either guard**, which would have left one verb with two meanings and made `st done` silently fiat-capable.

**The status VALUE does not move, which is what keeps hv's 2026-08-28 ruling intact** -- `fiat` sits BESIDE the status, so `Completed` still means `Completed` and every consumer reading it keeps working. What distinguishes the two edges after the fact is the record on `Thread.fiat`, and **no type holds that record and the status in agreement**: every verb touching this machine's status must clear it, which `Facade::set_thread_status_fiat` does in one place so that a verb added later inherits it.

**The Verb cell carries the OP TOKEN and not a CLI invocation**, for the reason Machine 3 records against `ac.fc`: the CLI spelling is one top-level `intent fc <target>`, whose second word is the target rather than a subcommand.

### Machine 2 -- Work package (`WpStatus`)

States: `NotStarted` | `Wip` | `Done` | `Cancelled`. **Entry: `NotStarted`.**

| From         | To           | Verb           | Guard              |
| ------------ | ------------ | -------------- | ------------------ |
| _(none)_     | `NotStarted` | `wp new`       | --                 |
| `NotStarted` | `Wip`        | `wp start`     | --                 |
| `NotStarted` | `Cancelled`  | `wp cancel`    | reason recorded    |
| `Wip`        | `Done`       | `wp done`      | **`ac gate` PASS** |
| `Wip`        | `Done`       | `wp.fc`        | reason recorded    |
| `Wip`        | `NotStarted` | `wp unstart`   | --                 |
| `Wip`        | `Cancelled`  | `wp cancel`    | reason recorded    |
| `Done`       | `Wip`        | `wp reopen`    | reason recorded    |
| `Done`       | `Cancelled`  | `wp cancel`    | reason recorded    |
| `Cancelled`  | `NotStarted` | `wp reinstate` | reason recorded    |

**The verbs this machine needed beyond v2 -- `wp reopen`, `wp unstart`, `wp cancel`, `wp reinstate` -- all ship.**

**`Cancelled` WAS PROPOSED AGAINST HERE AND hv RULED THE OTHER WAY ON 2026-08-21. The original text is kept because it was REASONED, not overlooked, and the way it failed is the lesson:**

> ~~No `Hold` or `Cancelled` at WP level is proposed -- a WP that stops mattering is a scope change on the thread, not a state on the package. Open for hv if that is wrong.~~

**It was wrong, and nothing tracked that the question had been asked.** A question addressed to a human, in prose, in a design document, with no counter reading it -- so the default shipped by silence and the omission survived a deliberate redesign of this very machine (`wp reopen`, `wp unstart` and `wp rescope` were all added without it registering).

**A live consumer proved it:** scope removed, every AC withdrawn, and `wp done` refused **forever** -- because [`contract::gate`] correctly declines to infer an exemption from an emptied contract, which is ST0048's rule that an exemption is ANNOUNCED and never inferred from emptiness. The only announced exemption was `acceptance: exempt`, which is **thread-scoped**: closing one unit with it would have discarded the standing of every AC in that thread. The interim convention -- mark it `Done` and write a note -- put the distinction in prose **because there was no field for it**, which is the same defect one layer along: a `Done` work package that delivered nothing is indistinguishable, by query, from one that delivered.

**`wp cancel` is that announcement as DATA, in a field the gate reads. It satisfies ST0048 rather than bypassing it.** It is deliberately **not** `ac gate`-guarded: every other close consults the contract, and this verb is the statement that there is no contract to consult, so gating it would rebuild the deadlock it exists to break.

**`Hold` is still not proposed** -- a paused package is `Wip` that nobody is touching, and no consumer has hit the absence.

**Refused on the way in AND reported afterwards (hv ruled both, 2026-08-15).** `wp done` is refused while its gate is BLOCKED, and `doctor` reports any unit whose status disagrees with its gate as `status-gate-disagreement`, in both directions -- recorded `Done` over a BLOCKED gate, or a PASS gate under a status that is not `Done` -- because the contract can change under a status that was true when it was set. A closed thread's findings are withheld by the default `--scope live` and read with `--scope all`.

**THE `wp.fc` ROW IS hv's D1 OF 2026-08-29, AND IT SITS DELIBERATELY BESIDE `wp done` RATHER THAN REPLACING IT.** Same from-state, same landing state, and **`reason recorded` where `wp done` has the gate** -- because a fiat close is BY DEFINITION the case where the gate does not pass, so guarding this edge with the gate would make it unreachable at exactly the moment it is for. **The option hv declined was relaxing `wp done` to accept either guard**, which would have left one verb with two meanings and made `wp done` silently fiat-capable.

**The status VALUE does not move, which is what keeps hv's 2026-08-28 ruling intact** -- `fiat` sits BESIDE the status, so `Done` still means `Done` and every consumer reading it keeps working. What distinguishes the two edges after the fact is the record on `WorkPackage.fiat`, and **no type holds that record and the status in agreement**: every verb touching this machine's status must clear it, which `Facade::set_wp_status_fiat` does in one place so that a verb added later inherits it.

**The Verb cell carries the OP TOKEN and not a CLI invocation**, for the reason Machine 3 records against `ac.fc`: the CLI spelling is one top-level `intent fc <target>`, whose second word is the target rather than a subcommand.

**AND THIS IS THE FIELD A CASCADE WRITES INTO.** A fiat close on a thread reaches its work packages, and a cascaded record carries `inherited_from` naming the ancestor -- so a package closed in its own right and one closed because its thread was are distinguishable by query rather than by reading the reason.

#### The cascade closes exactly what the machines already allow (hv 2026-08-28; the rule derived, vc 2026-08-30)

**WHICH CHILDREN A CASCADE CLOSES IS NOT A POLICY AND WAS NOT RULED -- IT IS THESE TABLES READ DOWNWARD.** A child is closed if and only if its own machine declares an `fc` edge from the state it is IN. `Facade::cascade_fiat` asks `transitions::permits` and nothing else, so the from-sets keep one home and the cascade cannot drift from the tables above.

**Every skip that rule produces is one that would have been argued for on its own**, which is the evidence it is derived rather than merely convenient. A `Satisfied` criterion is skipped because a fiat close is how an UNMET requirement is closed and offering it on a met one would let a close-on-authority overwrite a close-on-evidence. A `green` acceptance test is skipped for the same reason, an `n-a` one because it is not pending, a `done` or `cancelled` package because it is already closed, and `Descoped` or `Withdrawn` criteria because they are off scope entirely.

**Scope needs no new rule either**: a thread close reaches everything, and a work-package close reaches its own criteria and the tests covering them, selected by the AC group exactly as `Scope::WorkPackage` already does for the gate. Tests are reached through `covers` rather than through their id, because an AT's number need not match the criterion it covers and the coverage list is the modelled relationship.

**A SKIPPED CHILD WRITES NO EVENT, AND THAT IS ANSWERABLE ONLY BECAUSE OF THE EVENT-COUNT RULING.** _Was this child considered and skipped, or never in scope?_ is derivable by reading the machine against the child's state at the time -- and its state AT THE TIME is recoverable only because every state change writes its own event. **So one-event-per-entity is what makes the skip derivable, and one-event-per-cascade would have left the question genuinely unanswerable.** The derivability is a consequence of a ruling that was not about it.

**THE WHOLE CASCADE IS ONE TRANSACTION.** The ancestor's envelope and every child's are committed together, because N transactions would put a window between a thread's close and its children's, and a cascade interrupted there is the state-without-an-event defect one layer along. **That is possible because `Envelope::minted` generates the ULID in Rust**, so the ancestor's event id exists before any write and the children's records can carry it -- consistent with D42 rather than an exception to it, since the ULID is an IDENTITY and the `ts` is the STAMP, and only the stamp belongs to the write.

### Machine 3 -- Acceptance criterion

**One enum replaced two fields.** `Criterion` carried `satisfied: Option<bool>` AND `scope: AcScope`, which is what produced "three stored values, two meanings, one of them never written"; it now carries one `state: AcState` -- `Computed | Unsatisfied | Satisfied | Descoped | Withdrawn | Fiat`. **Entry: `AcState::entry(kind)` -- `Unsatisfied` for an authored criterion, `Computed` for a test-backed one.**

> The ratification of `computed`, its reasoning and vc's reversal onto cc's form are recorded above under "The fifth state"; `fiat` is ruled below the table. **A superseded table beside its own correction is worse than either alone**, because agreeing with the document is no longer a test of anything. The rows below are the ratified machine.

| From          | To            | Verb                    | Guard                    |
| ------------- | ------------- | ----------------------- | ------------------------ |
| _(none)_      | `Unsatisfied` | authored                | --                       |
| _(none)_      | `Computed`    | authored                | --                       |
| `Unsatisfied` | `Satisfied`   | `ac satisfy --evidence` | non-test; evidence given |
| `Satisfied`   | `Unsatisfied` | `ac unsatisfy`          | non-test                 |
| `Computed`    | `Descoped`    | `ac descope --to <ID>`  | target thread exists     |
| `Unsatisfied` | `Descoped`    | `ac descope --to <ID>`  | target thread exists     |
| `Satisfied`   | `Descoped`    | `ac descope --to <ID>`  | target thread exists     |
| `Descoped`    | `Unsatisfied` | `ac rescope`            | --                       |
| `Descoped`    | `Computed`    | `ac rescope`            | --                       |
| `Computed`    | `Withdrawn`   | `ac withdraw --reason`  | reason recorded          |
| `Unsatisfied` | `Withdrawn`   | `ac withdraw --reason`  | reason recorded          |
| `Satisfied`   | `Withdrawn`   | `ac withdraw --reason`  | reason recorded          |
| `Withdrawn`   | `Unsatisfied` | `ac reinstate`          | --                       |
| `Withdrawn`   | `Computed`    | `ac reinstate`          | --                       |
| `Fiat`        | `Unsatisfied` | `ac reinstate`          | --                       |
| `Fiat`        | `Computed`    | `ac reinstate`          | --                       |
| `Computed`    | `Fiat`        | `ac.fc`                 | reason recorded          |
| `Unsatisfied` | `Fiat`        | `ac.fc`                 | reason recorded          |

**THE GUARD COLUMN IS A CLOSED VOCABULARY AS OF 2026-08-30 (hv, D1), AND THAT IS WHAT LET AXIS C START GATING.** The permitted cells are exactly `--`, `reason recorded`, `ac gate pass`, `target thread exists`, `non-test` and `evidence given`, combined with `; ` where an edge carries two. `machine_table_check.sh` refuses a cell outside the set instead of reporting it, and every machine on this page is now measured on all three axes rather than two.

**WHAT CAME OUT OF THE COLUMN WAS NEVER A GUARD.** A guard is a PRECONDITION -- something that must hold before the edge may be taken. `clears evidence first` is an EFFECT, which is what happens after. `**non-test** -- lands on entry state` is a LANDING RULE, which tells you which of two rows you are reading. **Neither can be true or false of a transition being attempted**, so neither could ever be checked, and mixing them into the same column is why such cells read as UNMEASURED or outright DISAGREE against a transcription that was correct all along.

**Both facts survive, stated ONCE rather than repeated per row.** Every edge leaving `Satisfied` clears the evidence on the way out -- `ac unsatisfy`, `ac descope` and `ac withdraw` alike -- because evidence is proof that the criterion was MET, and a criterion that is no longer satisfied has none. **And every row landing on `Unsatisfied` or `Computed` out of `Descoped`, `Withdrawn` or `Fiat` lands on `AcState::entry(kind)`**, which is the entry rule this section already states above the table; the To column is what distinguishes those pairs, and the code declares no guard on any of them. Repeating either fact in every row was a second home for it, and the copy in the Guard column was the one that drifted.

**`Fiat` IS THE SIXTH STATE, AND ITS TWO EXIT ROWS ARE hv's RULING OF 2026-08-29** -- put up by dc and ruled dc's way: **the exit is `ac reinstate`, the same verb, not a new one.** Reinstating already means bringing a requirement that was closed-without-being-met back into play, landing on `AcState::entry(kind)`, and a fiat close is closed-without-being-met by definition -- one operation on one axis. The alternative, a terminal `fiat`, was declined on its cost: a terminal value in a `State` field is precisely what `no_state_can_be_entered_and_not_left` refuses, so taking it would have meant weakening a guard that protects every other machine on this page. **This is also what makes "renders distinctly forever" and reversibility compatible rather than opposed** -- forever is a claim about RENDERING, and the permanent record of the close lives in the event log.

**THE TWO ENTRY ROWS LANDED 2026-08-29 WITH THE VERB, WHICH IS WHAT THE PREVIOUS VERSION OF THIS PARAGRAPH SAID WOULD HAPPEN.** It read: _the entry row lands here in the same change that builds the verb_ -- and it did, so the paragraph is replaced rather than left standing beside rows that now contradict it. `transitions.rs` declares `ac.fc` from both open states and its `orphans:` list is empty; the exits landing first was the deliberate ordering, because an entry edge alone would have made `fiat` a trap state, the one shape `no_state_can_be_entered_and_not_left` exists to refuse.

**`Satisfied` IS ABSENT FROM THE FROM-COLUMN AND THE OMISSION IS THE RULING.** `ac withdraw` and `ac descope` are both declared from all three in-scope states, so the asymmetry is deliberate rather than transcription drift: **a fiat close is how an UNMET requirement is closed, and offering it on a met one would let a close-on-authority overwrite a close-on-evidence** -- destroying the evidence in order to record that no evidence was needed. `Descoped` and `Withdrawn` are absent because they are off scope entirely, and the off-scope refusal already names their undo.

**THE VERB IS SPELLED `fc`, TOP-LEVEL, NOT `ac fc`** (ic's surface ruling, 2026-08-29). This table names the AC machine's edge, whose op token is `ac.fc`; the CLI path is one top-level verb whose kind is derived from its target, because `AC-00.3` makes a fiat close cross-family by construction -- **a family-rooted verb cannot reach an ST's ATs, and a family verb that writes into two other families is the thing family roots exist to prevent.** **AND THAT IS WHY THE TWO `Fiat` ENTRY CELLS CARRY `ac.fc` WHILE EVERY OTHER CELL IN THAT COLUMN READS AS A CLI INVOCATION.** `machine_table_check.sh` normalises a Verb cell to an op token by taking its FIRST TWO WORDS and joining them with a dot, which coincides with the CLI spelling for every `<family> <verb>` row and CANNOT for a top-level one -- word two of `fc --because` is a flag, so it normalised to `fc.--because` and diverged from the code's `ac.fc`. **These are the first rows in this document where the CLI spelling and the op token are not the same string**, so they carry the op token directly rather than a spelling that would normalise to something no machine declares. The flag is not lost: `reason recorded` in the Guard column is the same requirement, stated where the checker reads guards.

### The two lists are a second witness to each other, and a commit gate holds them together

**`transitions.rs` carries the tables on this page transcribed into code.** hv's fiat ruling once reached that copy in the commit implementing it (`b7a3e771`) while this table sat unchanged, and nothing detected the gap: at the time no instrument parsed these tables, so the copy was machine-guarded and the authority was not, and drift could only run in the direction where the ratified document goes wrong unseen. **`machine_table_check.sh` closes that**: it parses every `### Machine N` table here and the declarations in `transitions.rs`, compares entry states, every expanded `(from, to, verb)` edge and every guard, and the commit gate runs it. A disagreement refuses the commit and names the side that moved.

**For the MACHINES, this page is the authority** -- `transitions.rs` says _the ratified AC machine (data-model.md "Machine 3"), transcribed_, and the status line at the head of this page limits "describes, does not define" to the entities and their fields.

**THE COUNTING RULE IS THE TRAP IN ANY SUCH DETECTOR.** The two notations do not count the same things: this table writes one row per `(from, to)` pair, while `transitions.rs` writes one `Edge::` per verb-and-landing with a LIST of from-states, so `ac.descope` is several rows here and one edge there. **Both counts are correct and they never match, so any comparison that counts before it expands will report a permanent false divergence** -- rule 6's shape, a check that reds forever is one the operator learns to ignore. The checker expands to pairs first, then compares sets.

**`Descoped` and `Withdrawn` are NOT the same state** and there is no direct edge between them -- descoped means the requirement still exists on a named thread and is a pointer you can follow; withdrawn means it does not exist at all. Moving between them routes through `Unsatisfied`, so the audit trail records the intermediate decision rather than smearing two facts into one.

**The asymmetry that must be explicit: a TEST-BACKED AC is never `satisfy`-ed by hand.** Its state is COMPUTED from its covering ATs -- green ATs satisfy it, anything else does not. So for test-backed ACs the `Unsatisfied <-> Satisfied` edges are not verbs at all; they are consequences of the AT status changing. `ac satisfy` applies only to `(non-test)` ACs. **This means the AC machine has two variants and only one of them has a satisfy verb** -- enforced in the model by `Guard::NonTestOnly` on `ac.satisfy` / `ac.unsatisfy`, in the published schema face by the `kind`/`state` invariant, and by `at lint`'s L5.

### One state field is deliberately not tabled here (recorded 2026-08-16; halved 2026-08-30)

**`transitions.rs` classifies each state field as `Disposition::State`, and this page ratifies a machine for every one of them but `WorkPackage.scope`.** That completeness is not something this paragraph asserts: `machine_table_check.sh` refuses any `Disposition::State` field that appears in neither its `MACHINE_MAP` nor its `UNTABLED` list, on every commit -- a count kept in prose beside a checker that computes it drifts in the direction that reads as complete.

- **`WorkPackage.scope`** -- every T-shirt value is `initial`, because `wp new` takes the size from the caller, with `wp rescope` the single exit. A value the caller supplies at creation has been ENTERED, so before `wp rescope` existed every size was a trap and neither v2 nor v3 had the verb. The reasoning lives in the code comment; it does not need a table because the graph is "any value, one verb, any value".

**`AcceptanceTest.status` is Machine 5** (hv, 2026-08-29): the omission once argued its graph was trivial -- one verb, no from-restriction, reaching every value -- and `at.fc` is a second verb with a from-set and a guard.

**`WorkPackage.scope` is not a gap in `transitions.rs`** -- it is classified, it is closed, and `mutation_completeness.rs` would refuse it if it were not.

### The `Unbuilt` fields -- RULED (hv, 2026-08-17, on vc's recommendation)

`transitions.rs` dispositioned **`Thread.acceptance`, `Criterion.kind`, `AcceptanceTest.kind` and `Issue.status`** as `Disposition::Unbuilt`. cc measured AC-04.6's second condition and found each is a value authored canon puts there with no verb to move it -- entered and unleaveable. Paying that debt means declaring edges, and declaring edges means declaring machines, which is the same criterion's FIRST condition. **So it was one ruling across the rows, not a call per row, and it is the block `intent issues add|close|open` had been standing behind.**

**The ruling is that only ONE of them is a state machine.** A field that cannot move on its own is not a state variable, it is a **component** of one -- and the other three cannot move on their own:

| field                 | disposition                                                         | as built                                                                                                                          |
| --------------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `Issue.status`        | **Machine 4**, below                                                | `Disposition::State`; `issues add`, `issues close`, `issues open`                                                                 |
| `Criterion.kind`      | folded into **Machine 3** as a `(kind, state)` pair; no new machine | still `Disposition::Unbuilt`: no verb converts a criterion's kind (`ac edit` takes `--text` and `--note` only)                    |
| `AcceptanceTest.kind` | folded into the **`AcceptanceTest.status`** machine; no new machine | `at edit --kind` re-kinds a row, refused where the current status cannot hold the new kind; `transitions.rs` still says `Unbuilt` |
| `Thread.acceptance`   | **immutable after creation**; no machine, no edge                   | `Disposition::Immutable`                                                                                                          |

**The pairing is enforced already and that is what settles it.** The `allOf` on `Criterion` (`model.rs:1145`) carries the `kind`/`state` invariant in the JSON Schema face, held by `tests/ac_kind_state_invariant.rs`: `{kind: test, state: satisfied}` records a satisfaction nothing computed, `{kind: non-test, state: computed}` claims a derivation with nothing to derive. **Flipping `kind` alone is schema-invalid**, so a kind conversion is one act moving two fields, which is a transition of the pair rather than of either field. `AcceptanceTest` has the identical shape -- a `(non-test)` AT is `n/a` by definition and can never be green -- so its `kind` folds into its own status machine the same way. For the AT the pairing is not in the schema face: `doctor` reports a test-backed row recording `n/a` as `model-inconsistent`, and `at na` does not refuse one. **ic hit this from the register side independently, having no notation for a multi-field atomic move; that gap was diagnostic rather than clerical.**

**`Thread.acceptance` is `Option<AcceptanceMode>` -- `exempt` or absent.** That is an attribute of a thread, not a lifecycle: changing it is AUTHORING, not a transition, and it gets no verb.

**Correction to vc's own shorthand, made here rather than carried into the ratified text.** The recommendation hv adopted said "widen Machine 3 over the (kind, state) pair" for **both** `kind` rows. That is wrong as written: `AcceptanceTest.kind` pairs with `AcceptanceTest.status`, not with the criterion's state. **Each `kind` folds into the machine of the entity that owns it.** The shape of the recommendation is what was ruled; transcribing the shorthand literally would have put `AcceptanceTest.kind` in the wrong machine.

#### Machine 4 -- Issue (`IssueStatus`)

States: `Open` | `Closed`. **Entry: `Open`.** Declared from v2's MEASURED behaviour rather than designed, because `intent issues` is `keep`-classified and v3 reproduces it.

| From     | To       | Verb           | Guard |
| -------- | -------- | -------------- | ----- |
| _(none)_ | `Open`   | `issues add`   | --    |
| `Open`   | `Closed` | `issues close` | --    |
| `Closed` | `Open`   | `issues open`  | --    |

**No guards, deliberately.** v2 had none, the row is `keep`, and inventing one here would be a parity break wearing a ratification. **Self-loops are accepted at exit 0 per the rule above** -- which is exactly v2's `already CLOSED` behaviour, and the reason the self-loop question had to be settled before this machine could be declared.

**What this discharges.** `intent issues add`, `close` and `open` ship. The other owed mutations become non-mutations: the two `kind` fields are pair transitions inside existing machines, and `Thread.acceptance` is owed nothing at all -- **no new machine is built for any of them**, which is materially less surface than the rows implied.

### Machine 5 -- Acceptance test (`AtStatus`)

States: `to-write` | `red` | `green` | `n-a` | `fiat`. **Entry: `to-write`.**

**RATIFIED by hv, 2026-08-29, as half of D1.** From 2026-08-16 this machine was ratified in PROSE, listed in the section above as needing no table on the ground that its graph was _"one verb, any value -> any value"_. `at.fc` is a second verb WITH a from-set AND a guard, so the premise is false and the table is owed. **`RATIFIED_WITHOUT_A_TABLE` in `mutation_completeness.rs` is the trigger that fired, and it fired on the commit that added the verb** -- a dormant assertion checking a ratification rather than altering one, red for exactly as long as this document disagreed with the code.

**THE GRAPH IS TRIVIAL AND THE SEMANTICS ARE NOT, which is what kept it out of a table.** `to-write` means the test is UNWRITTEN; `red` means it EXISTS and fails; **neither means "the criterion is unmet"**, which is the AC's own state and lives on Machine 3. That distinction is enforced by the linter's L2/L3 and, until this table, was written down on a whiteboard.

| From       | To         | Verb     | Guard           |
| ---------- | ---------- | -------- | --------------- |
| _(none)_   | `to-write` | `at.new` | --              |
| `(any)`    | `to-write` | `at.set` | --              |
| `(any)`    | `red`      | `at.set` | --              |
| `(any)`    | `green`    | `at.set` | --              |
| `(any)`    | `n-a`      | `at.set` | --              |
| `to-write` | `fiat`     | `at.fc`  | reason recorded |
| `red`      | `fiat`     | `at.fc`  | reason recorded |

**`(any)` IS NOT A STATE, AND THIS IS THE FIRST TABLE ON THE PAGE TO CARRY IT.** `at.set` declares `from: &[]`, which `Edge::accepts` reads as _no from-restriction_ and `machine_table_check.sh` renders as the literal `(any)`. Writing the states out instead would declare a DIFFERENT machine -- that `at.set` is legal from exactly those values -- and a value added later would then owe a new row per landing rather than none.

**EVERY CELL IN THE VERB COLUMN IS AN OP TOKEN AND NONE IS A CLI INVOCATION; this is the first table on the page where that is true of the whole column.** Machine 3's two `ac.fc` cells were the first such cells and the reason here is broader than a top-level verb: **`at.set` has no CLI spelling at all.** The surface offers `at green`, `at red` and `at na` -- each onto one facade call, the single `at_set` call site at `render.rs:3179`. The checker normalises a Verb cell by joining its first two words with a dot, so each of them would normalise to a token no edge declares. `at.fc` is the top-level-verb case Machine 3 already records.

**AND THE SURFACE CANNOT DRIVE ONE OF ITS LANDING STATES.** `at new` enters at `to-write` by default -- it also takes `--status` for any ordinary value, writing the new row through `Facade::put` as one `at.put` event -- and no command returns an existing test to `to-write`: `at edit` re-cites file, prose, kind and coverage, and does not touch status. So `(any) -> to-write` is reachable through the typed API and through no command. **That is this page's own closure-is-measured-at-the-facade ruling with a live instance** -- the machine is closed to the API, `no_state_can_be_entered_and_not_left` is satisfied, and neither fact says an operator can walk it. Recorded rather than fixed: inventing a verb so that a table looks symmetrical is how a surface grows commands nobody asked for.

**THE FROM AND TO CELLS CARRY WIRE FORMS RATHER THAN RUST VARIANT NAMES, AND ONE CELL FORCES IT.** Machines 1-3 write variant names -- `Triage`, `Unsatisfied` -- which the checker kebab-cases into the wire strings the code compares, and that works only while the two agree. Here they do not: `AtStatus::Na` carries `#[serde(rename = "n-a")]`, and kebab-casing `Na` yields `na`. The column is therefore wire form throughout, which is internally consistent and is literally the string each edge declares. **An editor "correcting" `n-a` to `Na` for consistency with the tables above will red this check** -- correctly, and confusingly, which is why it is written here rather than left for them to find.

**THE FROM-SET IS `to-write` AND `red`, AND IT IS dc's READING RATHER THAN A RULING.** hv settled that ATs get a fiat variant (2026-08-28) and settled nothing about which states reach it. Those two are where the test does not pass, which is the only situation a fiat close is for -- the motivating case was an AT unobservable by unit test, which sits at `to-write`. `green` is excluded because fiat-closing a passing test asserts human authority against evidence that already agrees; `n-a` is not a pending state. **Narrow deliberately: widening a from-set later is additive, and a fiat close reachable from a passing row is not recoverable by a later narrowing once estates carry them.**

**THE EXIT FROM `fiat` IS UNGUARDED HERE AND GUARDED IN MACHINE 3, and that asymmetry is recorded for hv rather than closed by inventing a guard.** `at.set` declares no from-restriction, so every ordinary value is already an exit and `fiat` is not a trap without a new verb. But an AC leaves `fiat` only through `ac reinstate` under `reason recorded`, while an AT's fiat close is undone by a bare `at green`. Putting a guard on `at.set` would add a precondition to the ordinary path that its other edges do not have -- a bigger change than the one it fixes.

**THE `fiat` / FIAT-RECORD INVARIANT IS ENFORCED BY NO TYPE, AND THIS TABLE CANNOT EXPRESS IT.** `AcState::Fiat` carries its `FiatRecord` INSIDE the variant, so on Machine 3 the state and its evidence cannot disagree -- the type holds them together and no verb can separate them. `AtStatus` derives `Copy` and async-graphql `Enum`, both of which forbid a payload, so the record sits BESIDE the status on `AcceptanceTest.fiat`. **A `fiat` status with no record, and a record on a row that is not `fiat`, are each expressible, and nothing structural refuses either.** The invariant is therefore carried by EVERY verb that touches `status`, and each one has to re-establish it: `at_fc` writes the pair together, `at_set` clears the record on any other landing, and `Facade::put` refuses to write the field at all (hv, D7). **A verb added to this machine later inherits that obligation and gets no reminder from the compiler** -- which is the most important thing on this page about Machine 5, and the reason it is stated in prose beneath the table rather than left to the rows.

### What this gives cc's `transitions.rs`

`transitions.rs` declares each ratified machine -- its `initial` values and every `Edge` -- and `tests/mutation_completeness.rs` drives the service layer against that **declared** graph, so the question is "does the code implement the ratified machine, exactly?" rather than "is the code closed?", and a missing verb is a red test rather than a fact nobody noticed until a WP could not be reopened. `machine_table_check.sh` holds the declaration to the tables on this page.
