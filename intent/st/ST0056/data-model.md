# Data model - ST0056: the reified Intent model (WP-01 spec)

Status: ratified structure per design.md D01-D05. WP-02 landed the schemars faces, so **this document describes, it does not define** -- the authored master is the Rust type layer and the committed faces under `schema/` are generated from it. The WP-01 draft schema that stood at the foot of this file is pruned; see "The schema face" below.

Amendments after WP-01 (vc, 2026-08-14, ADOPTED under hv standing authorisation): `objective`/`context`/`related` modelled on `steel_thread`; the marked-legacy `legacy` form on `acceptance_test`; the no-clock law on generated views. Each carries its rationale inline below.

## Entities

Identity convention (D15): natural keys stay human-legible; `(project_id, natural_id)` is the global identity. All dates ISO 8601 (`YYYY-MM-DD`); all timestamps UTC RFC 3339.

### Time convention (D42, AC-02.8 -- ruled 2026-08-15)

**Two kinds of time live in this schema and they must never be one column.** The tables below carry (b); (a) is the column AC-02.8 adds, and zero of eight tables had it.

- **(a) RECORD timestamp** -- when THIS store wrote THIS row. Set by the database as part of the write (a `DEFAULT`, never a caller value). Per-machine, **not carried in the extract**, and **correctly re-stamped on every rebuild** -- the row genuinely was written then.
- **(b) DOMAIN timestamp** -- when the thing happened in the project's history: `threads.created`, `threads.completed`, `issues.created`, `issues.closed`. **FOUR, and this bullet said three until 2026-08-17** -- see the door ruling below, where the same omission is recorded in the DDL's own defining comment. **Carried in the extract, never re-stamped**, and displayed by `st show` / `st list` and the `.md` views.

**Replacing (b) with (a) means a colleague who clones and rebuilds sees every thread created today.** The two doors keep them apart: **create** (the DB stamps) and **restore** (the recorded stamp is carried). Restoring is not creating.

Per-table naming, ruled on cc's measurement that the store has **no `UPDATE` anywhere** -- every write is `DELETE` + `INSERT`, so an `ON UPDATE` trigger can never fire and a `created_at` would silently record the latest write:

| tables                                | columns                     | why                                                                                          |
| ------------------------------------- | --------------------------- | -------------------------------------------------------------------------------------------- |
| `threads`, `issues`, `file_index`     | `created_at` + `updated_at` | upserted, so the row survives: `created_at` fires once, `updated_at` moves DB-side           |
| `related`, `wps`, `criteria`, `tests` | `written_at`                | replaced wholesale, so the honest record is _when this version of this row was written_      |
| `event_log`                           | `ts` (already)              | `ts` IS the record timestamp and rows are immutable -- no second column, and the DDL says so |

**A column is named for what it can honestly record, never for uniformity across tables.** `created_at` on a wholesale-replaced table would be a lie the moment a rebuild ran.

`threads.created` / `completed` are **DERIVED FROM THE EVENT LOG** -- the `ts` of the thread's `st.new` and `st.done`/`st.cancel` events. Those stamps are DB-set and are the one thing that merges across machines under D34, so (b) is a time that went end-to-end through the database. A v2-migrated thread has an authored `created:` and no `st.new`, so migration **restores** an `st.new` carrying the authored date. `issues.created` stays authored -- users write it in frontmatter and it is genuinely a fact about the world.

#### THE DOOR IS A PROPERTY OF THE ACT, NOT OF THE ENTITY (ruling, vc, 2026-08-17, on cc's finding)

**There are FOUR domain dates, not three.** `threads.created`, `threads.completed`, `issues.created`, **and `issues.closed`** -- which the schema carries (`store.rs:187`) and which **both** enumerations of the set omit: the bullet above, and the DDL's own defining comment at `store.rs:88-89`. **The comment that defines what a domain date IS is the comment a future author reads to decide whether a new column needs a door**, so a column missing from it is a column that will be added without one. Both homes are corrected to four.

**`store.rs::write_issue` has no `Stamp` parameter** (`:1012`), where `write_thread` has both doors (`:895`). It binds `created` and `closed` as raw `?6, ?7` -- which is `CarriedFromTheExtract` hard-coded, with no create door at all. **The ruling: every entity carrying a domain date needs both doors, because the door is chosen by the ACT and both acts reach every such entity.** `write_issue` is not a smaller `write_thread`; it is `write_thread` with the create door missing.

**`issues.closed` takes the three-state form `threads.completed` already has** -- `None` stays null, `Some("")` is stamped by the database, `Some(date)` is carried -- because `issues close` is precisely the act that produces it, and the alternative is the facade reading a clock, which D42 forbids.

**The `stays authored` rationale is correct and does not reach the case it is about to be asked about.** It answers "why is `created` not replaced by `created_at`" -- a migration question, where a v2 author really did write the date. It is silent on who fills `created` when **nobody authored it**, which is every v3-native `issues add`. Amended rather than deleted: the migration reading stays true.

**It is correct TODAY only because the mutation direction does not exist yet**, and that is the finding rather than an excuse for it. `write_issue` is reached from `rebuild` (restore -- carrying verbatim is right) and from `commit_mutation` (create -- which has no caller). **A file clean by luck and a file clean by construction look identical in a diff, and only one of them stays clean** -- the dispatch table names this as `known_exposures`, and this is a live instance of it in the store. The moment `issues add` ships, an undoored `write_issue` offers exactly two options and both are defects: write an empty `created` through to `intent/issues/<NNNN>.json`, or read a clock in the facade.

**`commit_mutation` returns `Vec<ThreadDates>` only** (`:1046`), so there is no channel to hand a stamped issue date back for the extract to render from -- **truth and its projection disagreeing on the one field neither can recompute**, which is D42's own stated hazard arriving at the second entity. The return needs to carry issue dates too. **The SHAPE of that channel is cc's call, not this document's**: the model says both entities' stamped dates must come back from the write, and whether that is a tuple, a second `Vec`, or one named struct is an encoding decision. Recorded preference only, freely overridable -- a single named struct with a field per entity, because a tuple of two is the shape that becomes an unreadable tuple of three.

**Scope call with its reversibility** (D39): `wps` and `criteria` have stable IDs, so wholesale-replace is a property of today's write strategy, not of the domain. Delete-missing + upsert-present is the upgrade path and `written_at` does not block it.

### project (`intent/.config/config.json` -- as today, plus)

| Field          | Type   | Notes                                                         |
| -------------- | ------ | ------------------------------------------------------------- |
| project_id     | uuid   | stamped at migration (D15); never changes                     |
| intent_version | string | `3.0.0`+                                                      |
| name, author   | string | as v2                                                         |
| languages      | array  | as v2 (ST0037)                                                |
| server         | object | RESERVED, absent in v3 (D15); intentc-era binding             |
| ~~st_prefix~~  | --     | **RETIRED (hv, 2026-08-16, issue 0040)** -- see below         |
| todo           | object | `{window_hours: int}` (D44) -- **NOT a watermark**; see below |

#### `st_prefix` -- RETIRED (hv ruled 2026-08-16, issue 0040)

**The thread-id prefix is fixed at `ST` in v3. The configurable knob is gone.**

v2 let a project change the two letters at the front of every thread id and honoured that in six places (`bin/intent_st:75` and onward through the directory glob, the id parse, the file glob and the allocator); `bin/intent_init:120` wrote the field into every project it ever created. **v3's `Config` kept the field, gave it a serde default, and read it nowhere** -- while `facade.rs:1895` hardcoded `format!("ST{:04}")` and `legacy.rs:198` hardcoded `starts_with("ST")` **and the length**.

**Retiring it is not a change of direction, and this is the part worth recording: `st_prefix` appears in NO ST0056 spec.** The config table above listed six fields and never included it. **The design had already dropped the knob and nobody propagated that to the type** -- so the field was not a feature awaiting wiring, it was residue of a decision already taken. Same shape as the day's other findings: a document and an implementation disagreeing, with nothing comparing them.

**Grounds for fixing rather than honouring, measured:** all 16 fleet projects use `ST` (Laksa by omission, the rest explicitly), so the entire migration corpus is unaffected; and every future id-touching feature -- search, code parsing, the daemon -- would otherwise have to honour a variable that is always the same value.

**Two obligations that come WITH the retirement, and the first is the whole reason this is a decision rather than a deletion:**

1. **The migrator NAMES the field when a project carries a non-`ST` value.** A project silently losing a setting it configured is exactly the failure the residue report exists to prevent. Retiring a knob nobody uses is fine; retiring it under someone who does use it, without telling them, is not.
2. **`legacy.rs:198` loses its hardcoded `name.len() == 6`** at the same time. It encodes "two letters plus four digits" a second, independent way, and a fixed prefix means the length is derivable rather than asserted -- leaving it is how the next person finds two encodings of one fact.

#### The todo watermark: a generated view that was its own database -- RETIRED BY D44, kept as archaeology

> **The v3 mechanism described below no longer exists** (hv ruled `todo --flush` / `--prune` dead; D44; cc unbuilt it at `7663fb19`, removing the watermark, the `todo.flush` op, `Facade::todo_flush`, `RenderContext.todo_watermark` and `in_done_bucket`). **This section's own last paragraph predicted exactly that** -- _"if they retire, the watermark retires with them and DONE filtering becomes a query parameter over the `completed` dates already in the model"_ -- and that is what D44 ruled, so the field is a **display window** now, not durable state. **The v2 archaeology below is kept because it is still the reason v3 does not do this**, and a retired mechanism whose reasoning is deleted gets reinvented. Flagged by cc, 2026-08-16, as stale in vc's file; corrected rather than removed.

Found by cc at WP-03 when the no-clock law (D23) forced the question of whether `todo.md`'s `## DONE:<timestamp>` heading was render time or data. It is data, and the mechanism is worse than a stray timestamp:

- `bin/intent_todo:20` calls it "the last-flush watermark", advanced only by `done --flush` / `--prune`.
- `read_done_watermark()` at `:157` **greps it back out of the generated `todo.md`**, and falls back to `date -u '+%Y-%m-%dT00:00:00Z'` when the file or heading is absent.

So the view is the only durable home of a fact the tool reads back as truth. Three consequences, all fatal to v3's model: the view cannot be regenerated from truth (deleting `todo.md` silently resets the watermark to start-of-today); a generated artefact is authoritative, which is the exact inverse of D02; and the render path reads a clock, which D23 forbids.

Ruling (vc, 2026-08-14; ADOPTED under hv standing authorisation): the watermark is **durable project state**, homed in `config.json` under a `todo` block, **always materialised and never defaulted at render time**. The render path receives it as an input and never reads it back. The v2 start-of-today fallback does not survive -- a default computed from a clock is the defect wearing a different hat.

~~Open for hv, and it decides whether this field exists at all~~ **ANSWERED: hv retired `todo --flush` / `--prune` (D44, 2026-08-16), so the watermark retired with them and DONE filtering became exactly the "query parameter over the `completed` dates already in the model" this paragraph named.** Recorded as a hit rather than deleted: the field was marked provisional _because_ it was downstream of a behaviour question, the question was asked of the right person, and the answer removed the field. That is the provisional marking working, and it is worth one line of evidence that it does.

**What replaces it, and the constraint that shapes it (D44 + D42).** The window is **configuration, not state**: `todo.window_hours`, default 24, non-destructive -- nothing is flushed, pruned or forgotten, because every completed date is already in the model and the file is regenerated from it. **The reason it must be config rather than a flag is measured: all six `todo` verbs regenerate the file, so a flag on any one of them is a silent-revert generator** -- the next verb rewrites the file without it.

**WHICH SURFACE THE WINDOW APPLIES TO (vc ruled 2026-08-16 under hv's standing "go with your recs"; raised because hv ruled the window and not its surface, and they are different questions).**

**The window applies to the TERMINAL render. The committed `todo.md` carries everything.**

A window resolved against a clock makes the file's content depend on **when it was generated rather than on what happened**, and this repository commits `todo.md`. So regenerating tomorrow drops rows and produces **a diff with no cause in the estate** -- committed churn under D02, where a generated artefact is supposed to be a function of the model and nothing else. **A terminal render is a moment and may legitimately depend on now; a committed file is a record and may not.**

This also keeps `datetime('now', ...)` inside the query legal under D42 without buying a second problem with it: the read-side clock stays where no artefact preserves its answer.

**If a later ruling reverses this and the file must carry the window, then the file must also record WHICH window generated it** -- otherwise a row dropped by the window and a row deleted from the model are indistinguishable in a diff, which is the same absence-is-ambiguous defect D05 refuses everywhere else.

### steel_thread (`st/<ID>/thread.json`)

| Field          | Type    | Notes                                                                                                  |
| -------------- | ------- | ------------------------------------------------------------------------------------------------------ |
| schema         | string  | `intent/thread@3.0` -- lets validators pick the schema                                                 |
| id             | string  | `ST0056`                                                                                               |
| title          | string  |                                                                                                        |
| slug           | string  |                                                                                                        |
| preamble       | string? | authored prose ABOVE the first `## `, minus the `# ` title -- carried verbatim, never classified       |
| objective      | string  | authored prose; may be empty (see below)                                                               |
| context        | string  | authored prose, markdown, carried verbatim                                                             |
| related        | array   | `{id, note?}` -- the Related Steel Threads block                                                       |
| body           | string? | authored prose, every other section verbatim -- the thread-level twin of D28's WP catch-all            |
| status         | enum    | `triage · not-started · wip · hold · completed · cancelled` -- **`tbc` is NOT a v3 value** (see below) |
| status\_reason | string? | the reason for the CURRENT status; cleared by any transition that does not carry one                   |
| created        | date    |                                                                                                        |
| completed      | date?   |                                                                                                        |
| acceptance     | enum?   | `exempt` (ST0048) or absent = enforced                                                                 |
| wps            | array   | work_package records, ordered by seq                                                                   |
| criteria       | array   | acceptance_criterion records                                                                           |
| tests          | array   | acceptance_test records                                                                                |

**`tbc` was in this row until 2026-08-15 and it was WRONG in two directions at once**, which is why it is called out rather than quietly swapped. The ratified machine (Machine 1 below) has no such state; `triage` is the real entry state. And v2's `tbc` means **To Be Commenced**, not triage -- so it maps to `not-started`, never to `triage` (migration.md carries the rule and ic's independent witness for it). **v3 must not accept `--status tbc` nor abbreviate `Triage` as `TBC`**: reusing the letters is exactly how a mapping rule gets undone by a surface.

**`status_reason` is a denormalised read of the latest guarded transition, never a second source for history** (cc, 2026-08-15, on hv's _"feel free to add to the schema to support this kind of thing"_). The history is the event envelope, which every guarded verb writes. It is cleared by any transition that does not carry a reason -- otherwise `st hold --reason "waiting on the fleet"` followed by `st resume` leaves a running thread explaining why it was paused, which is a stale value that reads as current. **It is in AC-02.6's scope like every other field**: a file form must carry it, or the round-trip loses the reason at the clone boundary.

No verblock: git is the history of structured files. Authored prose files keep the v2 verblock convention unchanged; generated views carry a generated-banner footer instead (the AGENTS.md pattern).

#### `preamble` -- the region above the first heading, and it is a CONSERVATION FIX rather than an additive field (ruling, vc, 2026-08-17, on cc's measurement)

**Definition, stated first because the boundary IS the measurement: everything after the frontmatter and before the first `## `, minus the `# ` title line, stripped.**

**IT IS NOT CARRIED TODAY. IT IS LOST.** `legacy.rs:1373` buffers only once `current.is_some()`, so every byte before the first `## ` falls on the floor -- confirmed at source by cc, and reported independently as `LOST-PROSE ... in no section, no objective, no body` by `conservation_check.sh` since that arm was written. **cc proposed this field believing the region was carried verbatim and unclassified. The field is worth building under either premise, but its PURPOSE differs between them, which is why it is specced before it is written rather than after.**

**Population: 396 regions / 88,648 bytes across nine projects** (cc, fleet-wide) -- Lamplight 194, Laksa 79, Conflab 47, Baize 29, Intent 20, then Utilz, Devbin, Courses, Prolix. On the canary at `42fb5269`: **20 regions, 15 thread-level and 5 work-package, 102 to 1020 bytes each.**

**WHAT IS IN IT DECIDES THE CLASSIFICATION.** ST0010's 485 bytes are a deprecation blockquote carrying a supersession pointer, plus an authored metadata block. **A cancelled thread's "superseded by X" note is precisely what the cancellation discipline exists to preserve, and it is dropped with no drop record** -- so this is a conservation defect and not a convenience.

**IT NEEDS ITS OWN FIELD AND MUST NOT GO IN `body`** (cc, and the reason is load-bearing rather than aesthetic): `wp_info` renders `body` after `## Objective`, so a preamble carried there comes back in the wrong place. **Bytes preserved, position changed -- which trades a silent DROP for a silent MOVE, and the second is harder to see than the first.**

**CARRIED VERBATIM, NEVER CLASSIFIED.** cc's split of the canary regions found them largely metadata restatement (`- **Status**: ... / - **Created**: ... / - **Author**: ...`), and that is exactly why no classifier is built for them: a model naming the shapes it foresaw drops what it did not, and here the unforeseen remainder is the load-bearing half.

**THE BOUNDARY IS THE STRIPPED FORM, AND THIS PARAGRAPH IS THE RULING THE CHECK REFUSES TO MAKE FOR ITSELF.** `conservation_check.sh` deliberately does not rule that trimming is acceptable -- _"a check that silently adopted the migrator's own normalisation would be certifying it"_ -- it only reports WHICH KIND of difference occurred. The contract rules it here: **the field stores the stripped region, and the surrounding blank lines are markdown layout the renderer re-emits.** Consequence, stated so nobody later reads it as a regression: **the 20 regions land as `NORMALISED-PROSE`, not `CONSERVED` -- a reported, counted, NON-finding, the same treatment every other section already receives.**

**Both byte totals are correct and differ only by that strip, verified at the pin rather than argued.** Intent @ `42fb5269` reads **6135 stripped** and **6213 unstripped**, reproducing cc's figure and the census's figure exactly on one corpus; the 78 bytes are leading and trailing whitespace. **196 `info.md` at the pin against 197 in the working tree -- so the corpus moved and this measurement did not**, which is a stronger statement than agreement across two revisions.

**THE PRICE, recorded before the field exists so the after can be checked against it** (canary, `conservation_check.sh` at `ed29ce08`): `LOST-PROSE` **575 -> 555**; those 20 move into `NORMALISED-PROSE` **267 -> 287**; `MODELLED` **237 -> 257**, because a declared field is modelled rather than carried. **Zero new findings in either direction, and 6213 census-boundary bytes move from lost to accounted.**

#### `attachments` -- arbitrary authored files under a thread, and the rule that keeps disk optional (spec, vc, 2026-08-18, on hv's ask)

**hv's requirement: _"whatever is in the `ST####/**/*.{md,txt,...}` can also be attached to the db in a lossless manner."_** It exists because hv ruled that disk becomes optional -- an index plus render-on-demand -- and **the moment disk is optional, anything the store does not hold is destroyed by the first render.** Measured on this estate the day the hoist landed: of 485 `.md` under `intent/st`, **380 are in the store, 52 are NOT, and 53 are too short for the probe to test either way.** The 52 are one-off documents nobody modelled: `reference.md`, `dogfood-journal.md`, `phase0_summary.md`, `phase1_plan.md`, `done.md`, `README.md`.

**THE POPULATION IS NOT WHAT THE ASK SOUNDS LIKE, AND THE DESIGN TURNS ON IT.** 304 files under thread directories are not one of the canonical five:

| ext    | count | what it is                                        |
| ------ | ----- | ------------------------------------------------- |
| `.tap` | 196   | TAP baselines under `ST0056/parity/tap-baseline/` |
| `.md`  | 66    | authored prose                                    |
| `.sh`  | 38    | the parity instruments -- executable code         |
| `.txt` | 2     | data                                              |
| `.tsv` | 2     | census output                                     |

**So "attach everything under `ST####/**`" would put 196 generated test baselines and 38 executable shell instruments into the record of intent.** Those are files the repository versions and tools consume; they are not what anyone authored _as the record of the work_. **The store is the record of intent, not a second filesystem** -- and a store holding executables needs mode bits, binary payloads and a merge story, which is a version control system that already exists one directory up.

**THE LINE, and it is mechanical rather than a judgement made per file: attachment is by DECLARED EXTENSION.** `.md`, `.txt` and `.sh` attach. The declared set is a list, extending it is an explicit decision, and nothing is classified by inspecting content or by anyone's opinion about whether a file "feels authored".

**`.sh` ADDED 2026-08-18, and the extension rule now has a stated principle rather than an inherited list: the line separates "no tool can make this again" from "a tool made this and can again."** That is the authorship axis the view/attachment split already runs on, applied one level down.

**The evidence, measured, and it is a population of ONE thread.** Of 240 files under `intent/st` that are neither `.md` nor `.json`, **every single one is under ST0056** -- 55 of 56 threads hold none at all:

```
196  *.tap   intent/st/ST0056/parity/tools/tap-baseline/   336,050 bytes -- GENERATED by a tool
 39  *.sh    intent/st/ST0056/parity/tools/                              -- HAND-AUTHORED, unreproducible
  2  *.tsv     2  *.txt     1  .DS_Store (gitignored, correctly excluded)
```

**Without `.sh` in the set, the 39 files the model does not hold are the instruments that verify the migration** -- `conservation_check.sh`, `estate_census.sh`, `realise_plan.sh`, the interrupt rig, the parity tools. **Including the tool whose entire job is to prevent content being lost.** They are safe from deletion either way, because `realisation.md` 5.1's fourth disposition reports a path the renderer does not produce as UNCLAIMED and never removes it -- but safe-from-deletion is not the same as carried, and a clone of the thread's canon would not contain the tools that prove the canon.

**`.tap` stays OUT, and naming why is the point of a declared set.** A TAP baseline is a tool's output committed as pinned evidence; a tool made it and a tool can make it again. It is not authored, so carrying it would put 336 KB of regenerable bytes in the model to no end. It stays on disk under the UNCLAIMED rule -- **the evidence survives a dehydration that removes the views, which is the correct outcome rather than a compromise.**

**And this is a burn-down rather than a standing report.** `doctor`'s uncarried list is the pre-deletion risk register for disk-optional, so the number should fall as the declared set earns its members. 237 to 198 with `.sh`; the remaining 196 are one declared class with one reason. **A project without a test corpus under a thread prints nothing at all.**

**AND THE PROPERTY THAT MATTERS MORE THAN THE LINE ITSELF: A FILE THAT DOES NOT ATTACH MUST BE NAMED, NEVER SILENTLY SKIPPED.** `doctor` reports every unattached file under a thread by path. Without that, the rule reproduces the exact defect this whole thread has spent a week cataloguing -- **disk becomes optional and something vanishes because nothing ever said it was not covered**, and an absence is indistinguishable from a decision. The 52 above were found by measurement, not by any surface saying so, which is the evidence that silence is not good enough.

| Field    | Type   | Notes                                                                                                                 |
| -------- | ------ | --------------------------------------------------------------------------------------------------------------------- |
| `path`   | string | relative to the THREAD root, so `WP/01/notes.md` is covered without a WP-level collection                             |
| `text`   | string | **VERBATIM. Never parsed, never normalised, never section-split.** The round trip is byte-exact or it is not lossless |
| `bytes`  | int    | as read                                                                                                               |
| `sha256` | string | the round-trip test: write it back and the hash matches, or `doctor` reports skew                                     |

**Four rules.**

1. **A file is a typed doc OR an attachment, never both.** The canonical five are parsed into the model; everything else in the declared set is carried verbatim. Two homes for one file is the Highlander violation this field would otherwise introduce.
2. **`text` is opaque.** The typed docs earn their parsing because the model has fields for what comes out. An attachment has no such fields, so parsing it would discard structure into nothing -- which is how `## Related Steel Threads` became 52 rows of `LOST-PROSE`.
3. **Non-UTF-8 is REFUSED BY NAME with a remedy**, never stored as a blob and never skipped. `sync` already does this (`unknown-file-shape -- not valid UTF-8`) and it is the correct posture.
4. **Attachments feed `doc_sections` so `search` finds them.** That index is derived and rebuildable; the attachment record is the authority.

**What this discharges.** It is the precondition for hv's disk-optional model, and the gate is already built and does not need writing: `conservation_check.sh` asks whether every authored section has a destination and whether the bytes that arrive are the bytes that left.

**THE PHRASING THAT STOOD HERE UNTIL 2026-08-18 WAS "when `LOST-PROSE` and `UNACCOUNTED` reach zero, disk is safe to make optional", AND IT WOULD HAVE AUTHORISED DELETING 192 FILES HOLDING 748 AUTHORED SECTIONS WHILE PRINTING A ZERO.** ic drove both counters to zero on an unconserved estate in the morning and found them pinned above zero on a conserved one at night; the same day, `LOST-PROSE` reached zero legitimately and **that is when the shorthand became lethal, because it is TRUE and it means something narrower than it reads.** `LOST-PROSE 0` means _every section that HAS a destination reached it_. It says nothing about sections with no destination, and **those are exactly the population disk-optional deletes.**

**Measured on the live estate, all of it rather than a sample: 163 `design.md` / `impl.md` / `tasks.md` files probed for their longest distinctive line in their own thread's canon -- 0 found. `thread.json` has no `design`, `impl` or `tasks` field, so the content exists nowhere but those files.**

**THE GATE IS THE VERDICT AT FULL SCOPE, NEVER A COUNTER:** exit 0 AND a printed `conservation: 0 finding(s)` line (its ABSENCE is a refusal, not a zero) AND the denominator AND a pinned subject, per `conservation_check.sh`'s own header. **On this estate that is unreachable while one file is STRANDED, which is the correct behaviour and is why the verdict is the gate.** The issue `body` field landed; the 748 sections are an open design question -- see below.

#### Why objective / context / related are modelled (the info.md mixed-file resolution)

D02 forbids mixed files, and v2's `info.md` is flatly one: frontmatter and status (structure), Objective and Context (authored prose), Related Steel Threads (structured links), and a "Context for LLM" template block. design.md's layout table makes `info.md` a generated cover while listing "objective/context prose" as authored -- naming no file for it. Surfaced by cc at WP-03 start, when the view renderer became the thing that would have had to discover the answer.

Ruling (vc, 2026-08-14; ADOPTED under hv standing authorisation): the three fields are **modelled on `steel_thread`**, and `info.md` becomes 100% generated. There is no sixth default steel-thread doc.

- `objective` is already a field the tool has an opinion about -- the 0010 empty-objective warning -- which is the signature of a modelled field rather than free prose. The warning stays **computed from emptiness, never stored**, on the same double-truth grounds as `satisfied`.
- `context` and `related` follow it into the model because splitting one cover across a modelled half and an authored half rebuilds the mixed file one level down.
- The alternative -- a new authored `context.md` -- was rejected on reversal cost, not on taste. A field-add reverses by moving two fields; a sixth default doc changes the template set, `intent st new`, the migrator and every consumer's mental model.

Named cost, accepted deliberately: multi-paragraph markdown lives inside a JSON string field. It is tool-written and authored via mutation, which is what D02 asks for, and prose bodies are still stored verbatim and never reflowed.

Deferring this to WP-10 was the rejected option. The migrator would have discovered the missing prose home, and a migrator that meets an unspecified half of its own target is the `at lint --fix` scar repeating: a tool that cannot finish a job must not start it.

### work_package (inside thread.json)

| Field          | Type    | Notes                                                                                                                              |
| -------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| seq            | int     | rendered `WP-01`                                                                                                                   |
| title          | string  |                                                                                                                                    |
| scope          | enum    | `XS · S · M · L · XL · XXL`, plus a marked-legacy form for a v2 value outside the set (see below)                                  |
| status         | enum    | `not-started · wip · done`                                                                                                         |
| status\_reason | string? | as `steel_thread.status_reason` -- current status only, cleared by a transition carrying none                                      |
| preamble       | string? | as `steel_thread.preamble` -- 5 of the canary's 20 regions are WP-level                                                            |
| objective      | string? | authored prose, the `## Objective` section (D28)                                                                                   |
| body           | string? | authored prose, every other section verbatim (D28) -- `## Deliverables` and `## Dependencies` live here, deliberately unstructured |

#### scope: canonicalisation is not loss, but one v2 value is outside the set

Measured on this repository's own corpus (vc, 2026-08-15, on cc's WP-06 finding): v2 reads `scope` as **free text**, and 129 work packages carry **eleven spellings** -- `Small` 56, `Medium` 34, `Large` 8, `L` 8, `XL` 5, `M` 5, `S` 4, `ExtraSmall` 4, `Extra Small` 3, `XS` 1 -- and `Medium-Large` 1.

**The first ten are `corrected`, and rendering them canonically is not lossy.** The model declares `scope` an enum, so the enum is the truth and the spelling was always incidental presentation of one of six values; `Extra Small` and `XS` carry identical information. "As observed" cannot mean reproducing ten spellings for six sizes, because the thing observed was a free-text field standing in for an enum.

**`Medium-Large` is the eleventh and it decides the rule.** It maps to nothing in `XS · S · M · L · XL · XXL` -- it sits between two of them -- and it lives at `intent/st/COMPLETED/ST0020/WP/09/info.md`, in a **CLOSED** thread. hv's ratified carry policy is that CLOSED threads are lossless-by-carrying and LIVE threads are BLOCKED-until-clean, and **neither is ever lossy**. So all three obvious moves are forbidden at once: normalising it to `M` or `L` is a guess and lossy; blocking on it violates lossless-by-carrying for a closed thread; dropping it is loss outright.

**Ruling: `scope` carries a marked-legacy form for a value outside the enum**, following the precedent this model already sets for `acceptance_test`'s marked-legacy shape. A closed thread carries losslessly, the value stays visible AS legacy rather than being silently canonicalised into a lie, and the enum stays honest for everything new. A LIVE thread carrying an unmappable scope still BLOCKS, per the same policy. The general form is D05's posture applied one level down: an unknown enum VALUE is refused or marked by name, never guessed -- exactly as an unknown FIELD is.

`objective` and `body` exist because `WP/<NN>/info.md` is the same mixed file `steel_thread`'s `info.md` was, and D22 was never applied one level down -- see D28. Without them the WP-10 migration drops every work package's authored prose, which AC-10.5's prose-conservation clause forbids. **The contract already carried that gate; the model did not carry the field.** `## Acceptance` is not modelled: its text is fixed boilerplate pointing at `acceptance.md`, so it is generated, and restating ACs in a WP view would be the double truth the single-source rule exists to stop.

### acceptance_criterion (inside thread.json)

| Field | Type   | Notes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ----- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| id    | string | `AC-01.1` -- **the group is a GROUP, and it names a WP only when the thread HAS work packages.** `00` is thread-level. In a thread with no WPs the group is a numbering device and references nothing: six threads in this estate (ST0043, ST0044, ST0045, ST0046, ST0050, ST0051) carry 73 such rows across groups 01-08 with zero WP directories between them. The earlier wording here -- _group = WP seq or `00` for ST-level_ -- was written from ST0056's shape, which has fifteen work packages, and it made a convention into a rule the estate had never followed. `doctor` implemented that wording faithfully and produced **73 of its 78 findings** against six closed threads on the first run after the hoist. **The model was wrong, not the estate, and the fix is one clause: only assert the WP reference when `!thread.wps.is_empty()`.** The check keeps its value where it has one -- 37 threads DO carry work packages, and in those a group naming a missing WP is a real inconsistency; not one of the 37 appears in the finding set |
| text  | string |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| kind  | enum   | `test · non-test` -- **the DISCRIMINATOR for `state`'s shape below; it already existed and is not being added**                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| state | tagged | `{is: computed}` · `{is: unsatisfied}` · `{is: satisfied, evidence}` · `{is: descoped, to: STxxxx, by?, reason?}` · `{is: withdrawn, reason, by?}` -- **REQUIRED on every criterion.** `computed` is the in-scope value for a `test` criterion; `satisfied`/`unsatisfied` are refused on one. FIVE values, RATIFIED by hv 2026-08-15 -- see "The fifth state"                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

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

> **RATIFIED (hv, direct, 2026-08-15): "Ratified".** Machine 3 has FIVE states. The section below is kept as written because the reasoning is the record of how a divergence between a ratified machine and its implementation was found and resolved -- but the open question it ends on is now closed, and `mutation_completeness.rs` measures hv's ratification rather than cc's transcription.
>
> **What this closes, and it is the reason it was escalated rather than absorbed:** for about 75 minutes the conformance test that proves the code implements the ratified machines was asserting against **one node's reading of them**. A green there meant "cc and cc agree". Two artefacts transcribed by one author from one document in one session are not two witnesses -- they agree with each other and both differ from the source.

**Machine 3 as ratified had FOUR values. The implementation has five.** `transitions.rs` declares `initial: &["computed", "unsatisfied"]` and `mutation_completeness.rs` transcribes `ac.rescope` and `ac.reinstate` as **two edges each**, landing on `AcState::entry(kind)` -- `unsatisfied` for an authored criterion, `computed` for a test-backed one. cc wrote the divergence into the comment rather than hiding it: _"That is not a transcription error ... the ratified table's single `-> Unsatisfied` row is written for the authored criterion it had in mind."_

**The problem cc hit is real and the ratified table does not answer it.** `ac rescope` on a **test-backed** AC has to land somewhere, and landing it on `Unsatisfied` stores a satisfaction claim about a criterion whose satisfaction is computed. There is no fourth value that fits.

**And `computed` beats the form I ruled, on my own grounds:**

- **Ground 1 (a non-test AC that LOST its state must not validate).** Under `computed`, `state` is **REQUIRED on every criterion** -- there is always a value -- so a missing `state` is a refusal for both kinds. Under my absent-for-test-backed form, absence had to be permitted schema-wide, which is the hole I was arguing against. **cc's form closes it more completely than mine did.**
- **Ground 3 (usable WITHOUT Intent).** `{state: computed}` says on its face that this criterion is derived. My form required a conditional -- "absence is legal iff `kind` is test" -- which is precisely a rule an external reader would have to reimplement. **I argued against transferring a rule to the reader and then chose the form that transfers one.**
- Ground 2 (`kind` already exists) was never an argument for absence; it survives as the reason `kind` stays.

**The honest cost, stated because it is the only thing in cc's favour I am not counting: two fields can express nonsense.** `{kind: non-test, state: computed}` and `{kind: test, state: satisfied}` are representable and meaningless. The API already refuses both (`Guard::NonTestOnly` on satisfy/unsatisfy), so the door is shut at the gate that matters under D01 -- but the schema face should refuse them too, or the extract can carry a combination ingest will reject, which is a round-trip failure waiting at the clone boundary.

**THE ACTUAL DEFECT IS NOT THE DESIGN, IT IS WHERE THE DESIGN LIVES.** A fifth state value in a machine hv ratified with four exists, today, only in the implementation and in the test that checks the implementation. **Two witnesses transcribed by one author from one document are not two witnesses** -- they agree with each other and both differ from the ratified table, which is the exact failure mode a second transcription is supposed to prevent. Recorded here so the canon carries it; **flagged to hv as an extension of a ratified decision rather than an implementation detail, because that is hv's call and not vc's.**

### acceptance_test (inside thread.json)

| Field  | Type    | Notes                                                    |
| ------ | ------- | -------------------------------------------------------- |
| id     | string  | `AT-01.1`                                                |
| kind   | enum    | `test · non-test`                                        |
| file   | string? | test kind: repo-relative path (the 0017 reference rules) |
| prose  | string? | non-test kind: what was read/eyeballed                   |
| covers | array   | AC ids                                                   |
| status | enum    | `to-write · red · green · n-a` (`n-a` non-test only)     |
| note   | string? | the free trailing note                                   |
| legacy | object? | the marked-legacy carry form (see below)                 |

#### The marked-legacy AT form (the closed-thread carry policy's model consequence)

Required by the hv carry ruling in `migration.md`: CLOSED threads convert lossless-by-carrying, so a legacy-grammar AT row must land in the model whole, with nothing guessed, dropped or reformatted.

```
legacy: { raw: string }   -- absent on every row authored under the v2.19 grammar
```

`raw` is the **verbatim v2 row text**, byte-for-byte as it appeared, carried and never parsed. When `legacy` is present, `file` may be absent -- a `::name` citation or a multi-file list has no single repo-relative path, and inventing one is precisely the destruction the 0017 refusal was about.

The distinction that makes this safe: carrying a whole row into a richer model destroys nothing, where a fixer that rewrites one end of a two-ended reference destroys the link. Migrating data and improving it are different operations, and only the first one is the migrator's job.

Consequences that must hold together:

- Ingest accepts `legacy` (it is in the schema from the start, so `thread.schema.json` is blessed once, not twice).
- `legacy` is **carried, never interpreted**. No command reads `raw` to answer a question; anything that did would be the v2 "answers confidently from partial evidence" class rebuilt inside v3.
- A row carrying `legacy` is reported as carried-legacy in coverage views, never silently counted as an ordinary green.
- LIVE threads never produce one: they stay BLOCKED-until-clean, so a `legacy` row appearing in a live thread is itself a defect.

### issue (`issues/<n>.json` + authored body `issues/<n>.md`)

| Field    | Type   | Notes                                                                                 |
| -------- | ------ | ------------------------------------------------------------------------------------- |
| schema   | string | `intent/issue@3.0`                                                                    |
| number   | int    | rendered `0021`                                                                       |
| slug     | string |                                                                                       |
| title    | string |                                                                                       |
| status   | enum   | `open · closed` -- directories stop encoding status (parity deviation, see parity.md) |
| severity | enum?  | as v2                                                                                 |
| created  | date   |                                                                                       |
| closed   | date?  |                                                                                       |

### event_log (DB-only, append-only; D15)

Envelope: `{id: ulid, ts: rfc3339, principal, project_id, op, subject: {type, id}, payload}`. Written by every mutation (WP-02). **Corrected 2026-08-15**: the previous text said DB-only state must be losable and that the event log is explicitly NOT durable truth. Both are false under the reversed D01. **The DB is the durable SSOT, so the event log in it is durable truth like everything else there.** hv has ruled it a file form -- **`events.jsonl`, append-only** -- as a secondary artefact, alongside an `intent events` surface for query/extract/ingest/egest and a READ-ONLY `intent db sql`. Read-only is the boundary that matters: write-SQL would be a second door into the SSOT, and the typed API being the only door is the whole reason the DB's contents conform by construction.

### file_index (DB-only -- the sync engine's git-style index)

`{path, size, mtime, sha256, state: clean · changed · unparsed, findings[]}`. Scope: `intent/**` + named root files.

### doc_section (DB-only, from prose ingest)

`{owner_type, owner_id, file, seq, heading, level, body}` -- FTS5-indexed; powers `intent search`. Prose bodies are stored verbatim, never modelled.

### wb_node / wb_item / wb_message (`whiteboard/<node>/board.json`; D30, WP-14)

The coordination entities. Durable form is committed JSON canon per D01; `wip.md` and `inbox.<sender>.md` become generated views per D02, ending the hand-authored board.

| Entity       | Fields                                                                                                                          |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| `wb_node`    | `moniker` (PK), `name`, `role`, `session_id?`, `heartbeat_at`, `status` (`active · paused`), `focus`, `claims[]`                |
| `wb_item`    | `node`, `kind` (`doing · todo · decision · watchout`), `seq`, `text`, `state` (`live · archived`), `created_at`, `archived_at?` |
| `wb_message` | `sender`, `recipient`, `sent_at`, `body`, `re?` (prior anchor), `fyi` (bool), `state` (`live · handled`), `handled_at?`         |

Three properties are the point of modelling these rather than parsing them (D30):

- **Timestamps are read from the clock by the API**, never supplied by the caller, so a fabricated stamp stops being constructible rather than being detected after the fact.
- **Bounds are enforced on write and refused by name.** Per-entry body size, live items per node per kind, and live messages per inbox are configured, and an over-bound write is refused with the bound and the remedy stated -- the D05 posture applied to size, never truncation, never a silent accept.
- **`state` transitions are the API's**, so archival happens on schedule rather than when a node remembers, which is what produced 251KB of `.history`.

The header block stays line-oriented `key: value` in the rendered view (D13) -- it is generated from `wb_node` rather than parsed into it.

## Generated views: the renderer has no clock

Every generated view carries a generated-banner footer instead of a verblock (the AGENTS.md pattern). One constraint governs the banner and every other byte a renderer emits:

**No generated view contains a render-time value, and the view renderer has no clock.** Its inputs are the model and the tool version -- no clock, no locale, no `$USER`, no `$HOSTNAME`, no absolute paths, no environment. The banner names the tool version and the source template; it never names when it ran. Git already records when a view was regenerated, and it does it correctly.

This is derived from the contract, not a preference. AC-03.2 requires a view to render the same bytes twice; AC-03.4 requires regenerate-and-diff to come back empty. A view that stamps its own render time fails the first and makes the second diff every file on every run -- so the skew check becomes either useless or trained-to-be-ignored, which is the same outcome arriving later.

Three live v2 instances, found at `f7434f1` when cc raised the first one:

| Instance                       | Value                                              |
| ------------------------------ | -------------------------------------------------- |
| `intent/todo.md`               | `## DONE:2026-07-10T17:18:19Z`                     |
| `AGENTS.md`                    | `_Generated by Intent v2.19.0 on 2026-08-14_`      |
| `lib/templates/llm/_CLAUDE.md` | `on [[DATE]] for Intent v[[INTENT_VERSION]]` (:59) |

The third is why this is a law rather than a fix: the banner pattern this document ratifies for every v3 view is itself one of the instances, so repairing the reported file would have left the defect in every view not yet written. Fix the class by removing the capability.

Open at time of writing: whether todo.md's `DONE:<ts>` is a render stamp or real data (it is stale against current content, which is not how a render stamp behaves). If it is data -- the last DONE transition -- it is modelled and rendered from the model. If it is render time, it dies. Not split.

## Canonical JSON form

UTF-8, LF, 2-space indent, trailing newline, object keys in schema-declared order (serde struct order, not alphabetical), arrays in natural order. Tool-written always; hand-edits are legal but validated strictly on ingest (D05).

## The schema face: `schema/thread.schema.json`

The WP-01 draft schema that stood here is **pruned**. WP-02 landed the schemars face, so the supersession this document declared at the top has already happened, and a second copy of the schema in prose is the divergent-copy drift Highlander exists to stop -- proved in the act: the draft went stale the moment `objective`, `context`, `related` and `legacy` were added above, and a reader building from it would have built the wrong type.

The authored master is the Rust type layer (`native/rust/crates/intentsvcs/src/model.rs`). The committed faces are generated from it into `schema/` at the repo root -- `thread.schema.json`, `issue.schema.json`, `event.schema.json`, `ddl.sql`, `schema.graphql` -- by `native/rust/crates/intentsvcs/src/faces.rs`, and `native/rust/crates/intentsvcs/tests/schema_faces_drift.rs` fails CI on any diff. Re-bless deliberately and in the same commit as the type change:

```
INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift
```

Read the face for the schema; read this document for why the model has the shape it has.

Validation posture (D05): `additionalProperties: false` everywhere -- an unknown field is refused by name, never dropped (the serde_ignored discipline at the schema layer). Schema evolution is a schema-version bump, visible in the `schema` field, with the DB rebuilt (D01).

## What is deliberately not modelled

Prose (stored verbatim, FTS-indexed). Rules/skills/templates (shipped content, embedded in the binary, indexed at most). wip.md / restart.md (authored tracking prose -- the project-level pair at `intent/`, not the whiteboard's per-node boards).

**The whiteboard left this set at D30** (hv ruling, 2026-08-15) and is modelled above as `wb_node`/`wb_item`/`wb_message`, built in WP-14. It was the largest entry here, and its removal is also the largest single closure of the egest-symmetry gap: what remains below is what an `intent export` cannot reproduce from the DB alone.

## State machines (RATIFIED by hv, 2026-08-15)

> **RATIFIED.** hv answered the four open questions on 2026-08-15: (1) `st new` enters at **`Triage`** -- yes; (2) `wp done` is **refused** on a BLOCKED gate **and** `doctor` reports any unit whose status disagrees with its gate -- both, as recommended; (3) **no** `Hold`/`Cancelled` at WP level -- confirmed; (4) a test-backed AC is **never** `satisfy`-ed by hand and the AC machine therefore has two variants -- confirmed. **The `Tbc` -> `Triage` rename is ratified by hv's use of the name in answer (1)**; it is stated here rather than inferred, so a disagreement surfaces now rather than at a WP close.

Drafted on hv's instruction after the `TBC` / `On Hold` / `satisfied` rulings: _"we will obviously need state toggles and a state machine process that moves threads programmatically thru the states. So we should take a beat now and define the states and the legal transitions."_ These are **proposals for ratification**, not canon; the enums they describe are already in `model.rs` and the transitions mostly are not.

### Why now, with live evidence

Measured 2026-08-15 in this thread's own tracking data -- **three of five WPs have a status that disagrees with their own gate**:

| WP    | `status:` | gate verdict          |
| ----- | --------- | --------------------- |
| WP-02 | **Done**  | BLOCKED 5/6 (AC-02.6) |
| WP-03 | WIP       | BLOCKED 8/9 (AC-03.9) |
| WP-04 | **Done**  | BLOCKED 4/6           |
| WP-05 | WIP       | **PASS 4/4**          |
| WP-06 | WIP       | BLOCKED 4/7           |

Two of those were caused by vc adding ACs to a closed WP: **`wp done` exists and nothing undoes it, so a WP reopened in the contract keeps saying `Done`.** That is AC-04.6's own defect class -- a state entered and not leavable -- occurring live, in the tracking tool, committed by the verifier enforcing the rule that names it. WP-05 is the inverse: gate PASS, status WIP, because nothing moves a status forward on evidence either.

### Three findings that shape the draft

1. **`TBC` already means "To Be Commenced", not "to be confirmed".** `bin/intent_helpers:544` maps `"tbc"` and `"to be commenced"` to the SAME canonical value, `Not Started`. So hv's new meaning -- the raw pre-triage state -- is a **new meaning for a token that is already spoken for**, in every v2 document and every user's head. **Recommendation: name the state `Triage`, not `Tbc`.** Reusing a token for a second meaning is the defect class this thread has spent two days removing, and migration then has an unambiguous rule: every v2 `TBC` is `NotStarted`, and `Triage` starts with no legacy members.
2. **`Hold` is reachable only by hand-editing a file.** It is recognised by the status filter (`hold, on hold -> HOLD`) and no verb sets it. cc's archaeology, confirmed.
3. **`Completed` and `Done` are one-way doors.** No `reopen` at either level.

**CORRECTED 2026-08-17 (vc, measured -- issue 0046). FINDING 3 IS FALSE, AND IT IS FALSE IN THE DIRECTION THAT MATTERS: the doors are not one-way, they are unlocked and unlabelled.** `intent wp start` on a `Done` work package writes `WIP` over it (`bin/intent_wp:208`, unconditional `sed`, no read of the current status), and `intent st start` on a `Completed` thread does the same **and relocates the directory out of `COMPLETED/`**. Both at exit 0, both printing the sentence they print for work that was never done. Measured in a throwaway project, not read.

**Three consequences for the machines above, none of which change a ratified table.**

- **v2 has an UNDECLARED EDGE at both levels.** Machine 1 gives `st start` exactly `NotStarted -> Wip`; Machine 2 gives `wp start` exactly `NotStarted -> Wip`. **`Completed -> Wip` and `Done -> Wip` belong to `reopen`, with `reason recorded` as the guard.** AC-04.6's strengthened form forbids an undeclared edge in as many words, so **`start` classified `keep`/`as-observed` would ship one**, and `AT-04.6`'s walk is the test that should say so.
- **The stated cause of the live inconsistency is wrong.** Machine 2 calls `wp reopen` _"the one whose absence is causing the live inconsistency above"_. **The transition was never absent.** What produced the disagreement is that criteria changed under closed units -- AC-04.6 ADDED on hv's D32, AC-04.1 STRENGTHENED -- which is the same defect the `doctor` recommendation below addresses and is not a missing verb at all.
- **The design premise survives and gets sharper.** `reopen` is still owed, because **the thing missing was never the move -- it is the RECORD.** A reason-carrying door beside an open, unlabelled one is worth building only if the unlabelled one is closed at the same time: **`start` must REFUSE a terminal state and name `reopen`.** That refusal is the cheap half and it is what makes the guard real.

**Re-measured for the same reason, since the table above is dated 2026-08-15: it is now three of SEVEN, and in two opposite directions.** WP-03 gate PASS 11/11 and WP-05 gate PASS 6/6, both still `WIP`; WP-04 `Done` against a gate BLOCKED 4/6. WP-02 and WP-06 have since converged.

### The rules these machines obey

- **No terminal states.** Every state has at least one declared exit (D32/AC-04.6). A state that should be hard to leave gets a **guard**, not a missing verb.
- **Every transition names a verb**, reachable from every surface (D32).
- **Guards are declared, not implied** -- eg `Completed` requires a gate PASS.
- **Direct vs Incidental** (cc, 2026-08-15): an edge that exists only as a side effect of changing a different field counts for reachability and **never discharges a trap**.
- **SELF-LOOPS ARE LEGAL, AND THEY ARE ACCEPTED-AND-REPORTED AT EXIT 0 WITHOUT RE-RUNNING THE GUARD** -- RATIFIED (hv, 2026-08-17, on vc's recommendation; applies to all four machines at once). Asking a verb for the state an entity is already in is not a movement, so it is not a transition to declare and not an illegal one to refuse. **This CHANGES Machines 1-3 as previously ratified**, which refused a self-loop as `IllegalTransition`, and it brings v3 back to v2's measured behaviour: `intent issues close` on a closed issue returns 0 with `already CLOSED`.
  - **"Without re-running the guard" is the load-bearing half, not a performance note.** `Completed` requires a gate PASS. Re-running the gate on `st done` against an already-completed thread would let a criterion added AFTER the close BLOCK a thread that is legitimately finished -- which is precisely the live inconsistency recorded below, where AC-04.6 was added under closed units on hv's D32. **A self-loop must not be able to fail for a reason that did not exist when the state was entered.**
  - **The exit code follows from that**: refusing self-loops makes every idempotent script a special case, and idempotence is the property callers actually want from `done` / `close`.
  - **This does NOT license the 0046 class, and the distinction is exact.** `wp start` on a `Wip` work package is a self-loop and is now accepted at 0. `wp start` on a `Done` work package is `Done -> Wip`, an UNDECLARED EDGE belonging to `wp reopen` with `reason recorded` as its guard, and it stays refused. A ruling that self-loops are legal says nothing about movements, and reading it as amnesty for undeclared edges would reintroduce the two-doors defect that issue 0046 is about.
  - **THE PREDICATE (vc's reading of hv's ruling, 2026-08-17; the shape found by cc while implementing it).** **A verb applied to an entity already in that verb's TARGET state is a self-loop, and is accepted at exit 0 without running the verb's guards. Whether the verb is DECLARED from the current state is a separate question, and it is asked only when the current state differs from the target.**
    - **Recorded as a reading rather than as ruling text, because it is not derivable from the ruling.** hv ruled the behaviour; the predicate is the form that makes it decidable at a call site, and it should be argued with as vc's, not deferred to as hv's.
    - **The second clause is what keeps 0046 refused**, and it is why "is the verb declared from here" cannot be the test. `wp start` on `Done`: the target is `Wip`, `Wip != Done`, so it is not a self-loop; it goes to the declared-edge test and fails there. Test the verb's TARGET, never its declared origins -- a verb declared from many states would otherwise self-loop from all of them.
    - **`transitions.rs` transcribes this; it does not restate it.** A predicate living only in the test that exercises it is the declaration-enforced-by-hand shape deleted from `facade.rs` on the same day -- `Guard::GatePass` was declared and hand-enforced in two call sites, so deleting the declaration changed nothing. AC-04.6 holds the pair.

### CLOSURE IS MEASURED AT THE FACADE, AND SURFACE REACHABILITY IS A SEPARATE QUESTION (ruling, vc, 2026-08-17, on ic's issue 0052 and cc's `WorkPackage.scope` reasoning)

**Both nodes were right about different layers, and the disagreement only looked like one because neither statement named its layer.** cc's `transitions.rs` comment justifies `WorkPackage.scope` as a `State` on the grounds that _"`wp_new` takes the size from the caller"_, so every size is genuinely ENTERED. **That is true**: `facade.rs:1353` is `pub fn wp_new(&mut self, st: &str, title: &str, scope: TShirt)`. ic's 0052 says the premise is false, because `wp new` has no options. **That is also true**: the dispatch table declares `wp new` with two args and ZERO flags, and `render.rs:574` hardcodes `TShirt::S` to match v2's default.

**So the scope machine is fully reachable through the typed API and almost entirely unreachable through the CLI: the surface can enter ONE of seven initial values and drive NONE of six edges.** `State` stays -- the disposition is correct and so is cc's reasoning, including the `absent`-by-ingest seventh, which is the entry route this AC's second condition names.

**The general rule, and it is why this belongs here rather than in the issue.** The mutation walk drives the FACADE. So `no_state_can_be_entered_and_not_left` and the edge-for-edge walk establish that a machine is closed **to the API**, which is not the same claim as closed **to an operator** -- and nothing in the walk can tell the difference, because the walk is an API caller. **A field can satisfy every closure condition in this document while the surface offers no way to reach six of its seven values.** AC-04.6 is an API-closure criterion; surface reachability is the dispatch table's question and needs its own evidence. Neither answers the other, and a green walk must not be read as either.

**`WorkPackage.scope` therefore has no ratified machine table below and does not need one.** It is a six-value enum with a marked-legacy seventh, ruled above as an attribute of the package; its `State` disposition records that the values are ENTERABLE, not that the sizes form a lifecycle. The edge-for-edge walk covers `Thread.status`, `WorkPackage.status` and `Criterion.state` -- the three fields with ratified machines -- and **scope's exclusion from it is by design and is now stated rather than left as an omission**, because an omission and a gap are indistinguishable to the next reader.

### Machine 1 -- Steel thread (`ThreadStatus`)

States: `Triage` (proposed rename of `Tbc`) | `NotStarted` | `Wip` | `Hold` | `Completed` | `Cancelled`. **Entry: `Triage`.**

| From         | To           | Verb           | Guard              |
| ------------ | ------------ | -------------- | ------------------ |
| _(none)_     | `Triage`     | `st new`       | --                 |
| `Triage`     | `NotStarted` | `st triage`    | --                 |
| `Triage`     | `Cancelled`  | `st cancel`    | reason recorded    |
| `NotStarted` | `Wip`        | `st start`     | --                 |
| `NotStarted` | `Hold`       | `st hold`      | reason recorded    |
| `NotStarted` | `Cancelled`  | `st cancel`    | reason recorded    |
| `Wip`        | `Completed`  | `st done`      | **`ac gate` PASS** |
| `Wip`        | `Hold`       | `st hold`      | reason recorded    |
| `Wip`        | `Cancelled`  | `st cancel`    | reason recorded    |
| `Hold`       | `Wip`        | `st resume`    | --                 |
| `Hold`       | `Cancelled`  | `st cancel`    | reason recorded    |
| `Completed`  | `Wip`        | `st reopen`    | reason recorded    |
| `Cancelled`  | `NotStarted` | `st reinstate` | reason recorded    |

**New verbs required: `st triage`, `st hold`, `st resume`, `st reopen`, `st reinstate`.** Open for hv: does `st new` enter at `Triage` (every thread is triaged) or does `st new --start` skip to `NotStarted`? The `-s|--start` flag already exists and today jumps to `Wip`.

#### `st new -s|--start` -- a convenience flag COMPOSES declared transitions, it never introduces an edge (ruling, vc, 2026-08-15)

**Flagged by ic as "two edges at once", handed to vc and cc by hv.** The measurement settles most of it: **`-s|--start` is v2 parity, not new surface** -- `bin/intent_st:302,381,425`, `st new [-s|--start] <title>` in v2's own help, and the register carries it `keep`.

**Nothing about the flag changed. The machine grew a state underneath it.** In v2, `st new` landed at not-started, so `-s` was ONE transition. In v3, `st new` enters at `Triage`, so the same flag now spans **two**: `Triage -> NotStarted -> Wip`. A `keep` disposition is honest about the surface and silent about the semantics, which is exactly the kind of drift the register cannot see.

**Ruled: keep the flag, and it performs BOTH declared transitions in sequence.** The triage decision is not being skipped -- **a user who types `--start` has decided the thread is real work, which IS the triage decision, made explicitly by the same act.** Refusing the flag would ask them to state a conclusion they have already stated.

**The load-bearing constraint, and it is where the natural implementation goes wrong: `st new -s` must COMPOSE `st triage` and `st start`, never construct the thread directly in `Wip`.** Building the end state is the obvious way to write it and it produces two defects at once -- a state history with no triage event, and an effective `Triage -> Wip` edge that **is not in the ratified machine**, which either forces AC-04.6 to accept an undeclared edge or drives construction around `transitions.rs` entirely, contradicting D32's "no surface mutates state except through a service call".

**Discriminating test: after `st new -s`, the event log carries BOTH transitions.** A test asserting only the final status passes on the defect, and the defect is invisible from the outside because the resulting status is correct either way.

**The general rule, because more of these are coming** (`wp new --start`, and anything else that bundles): **a convenience flag is sugar over declared transitions and never a new edge.** If a bundle cannot be expressed as a sequence of declared transitions, the bundle is proposing a machine change and goes to hv as one.

### Machine 2 -- Work package (`WpStatus`)

States: `NotStarted` | `Wip` | `Done`. **Entry: `NotStarted`.**

| From         | To           | Verb         | Guard              |
| ------------ | ------------ | ------------ | ------------------ |
| _(none)_     | `NotStarted` | `wp new`     | --                 |
| `NotStarted` | `Wip`        | `wp start`   | --                 |
| `Wip`        | `Done`       | `wp done`    | **`ac gate` PASS** |
| `Done`       | `Wip`        | `wp reopen`  | reason recorded    |
| `Wip`        | `NotStarted` | `wp unstart` | --                 |

**New verbs required: `wp reopen` (the one whose absence is causing the live inconsistency above), `wp unstart`.** No `Hold` or `Cancelled` at WP level is proposed -- a WP that stops mattering is a scope change on the thread, not a state on the package. Open for hv if that is wrong.

**Open for hv, and it is the sharper question**: should `wp done` be **refused** while the gate is BLOCKED, or should a status that disagrees with its gate be **reported** as the defect it is? Today it is neither -- `wp done` consults the gate, but nothing re-checks afterwards, so a WP that was legitimately `Done` silently becomes a false green the moment its contract grows. **Recommendation: both.** Refuse on the way in, and have `doctor` report any unit whose status disagrees with its gate, because the contract can change under a status that was true when it was set.

### Machine 3 -- Acceptance criterion

**One enum replaces two fields.** Today `Criterion` carries `satisfied: Option<bool>` AND `scope: AcScope`, which is what produces "three stored values, two meanings, one of them never written". Per hv: `Computed | Satisfied | Unsatisfied | Descoped | Withdrawn`. **Entry: `AcState::entry(kind)` -- `Unsatisfied` for an authored criterion, `Computed` for a test-backed one.**

> **This table carried FOUR states until 2026-08-16 and the machine has had five since hv ratified `computed` on 2026-08-15.** The ratification, the reasoning and my own reversal onto cc's form are all recorded above under "The fifth state" -- **230 lines above, under a different heading, which is the whole problem.** The document was never wrong as a whole; it was wrong at the one place an implementer reads to find out what to build, with the correction filed where someone would only look if they already knew. **A superseded table beside its own correction is worse than either alone**, because agreeing with the document is no longer a test of anything. The rows below are now the ratified machine; the section above remains the reasoning for it.

| From          | To            | Verb                    | Guard                                |
| ------------- | ------------- | ----------------------- | ------------------------------------ |
| _(none)_      | `Unsatisfied` | authored                | **non-test** criterion               |
| _(none)_      | `Computed`    | authored                | **test-backed** criterion            |
| `Unsatisfied` | `Satisfied`   | `ac satisfy --evidence` | **non-test AC only**; evidence given |
| `Satisfied`   | `Unsatisfied` | `ac unsatisfy`          | clears evidence (cc built this)      |
| `Computed`    | `Descoped`    | `ac descope --to <ID>`  | target thread exists                 |
| `Unsatisfied` | `Descoped`    | `ac descope --to <ID>`  | target thread exists                 |
| `Satisfied`   | `Descoped`    | `ac descope --to <ID>`  | clears evidence first                |
| `Descoped`    | `Unsatisfied` | `ac rescope`            | **non-test** -- lands on entry state |
| `Descoped`    | `Computed`    | `ac rescope`            | **test-backed** -- lands on entry    |
| `Computed`    | `Withdrawn`   | `ac withdraw --reason`  | reason recorded                      |
| `Unsatisfied` | `Withdrawn`   | `ac withdraw --reason`  | reason recorded                      |
| `Satisfied`   | `Withdrawn`   | `ac withdraw --reason`  | clears evidence first                |
| `Withdrawn`   | `Unsatisfied` | `ac reinstate`          | **non-test** -- lands on entry state |
| `Withdrawn`   | `Computed`    | `ac reinstate`          | **test-backed** -- lands on entry    |

**`Descoped` and `Withdrawn` are NOT the same state** and there is no direct edge between them -- descoped means the requirement still exists on a named thread and is a pointer you can follow; withdrawn means it does not exist at all. Moving between them routes through `Unsatisfied`, so the audit trail records the intermediate decision rather than smearing two facts into one.

**The asymmetry that must be explicit: a TEST-BACKED AC is never `satisfy`-ed by hand.** Its state is COMPUTED from its covering ATs -- green ATs satisfy it, anything else does not. So for test-backed ACs the `Unsatisfied <-> Satisfied` edges are not verbs at all; they are consequences of the AT status changing. `ac satisfy` applies only to `(non-test)` ACs. **This means the AC machine has two variants and only one of them has a satisfy verb** -- a fact currently enforced by L5 in the linter and nowhere in the model.

### Two more state fields exist and are deliberately not tabled here (recorded 2026-08-16)

**Three machines are documented above; `transitions.rs` classifies FIVE fields as `Disposition::State`.** The other two are `WorkPackage.scope` and `AcceptanceTest.status`, and naming them stops this section reading as the complete set -- which it does not say and which a reader would reasonably assume.

- **`WorkPackage.scope`** -- six T-shirt values, **all six `initial`** because `wp new` takes the size from the caller, with `wp rescope` the single exit. A value the caller supplies at creation has been ENTERED, so before `wp rescope` existed all six were traps and neither v2 nor v3 had the verb. The reasoning lives in the code comment; it does not need a table because the graph is "any value, one verb, any value".
- **`AcceptanceTest.status`** -- `to-write` / `red` / `green` / `n-a`, entry `to-write`, one verb `at.set` reaching all four. **This is the machine with the most operational subtlety and the least written down**: that `to-write` means the test is UNWRITTEN while `red` means it EXISTS and fails, and that neither means "the criterion is unmet", is enforced by the linter's L2/L3 and recorded on a whiteboard, not here. The graph is trivial and the SEMANTICS are not, which is the opposite shape to `scope` and the reason it is called out rather than merely counted.

**Neither is a gap in `transitions.rs`** -- both are classified, both are closed, and `mutation_completeness.rs` would refuse them if they were not. The gap was this document implying three.

### The four `Unbuilt` fields -- RULED (hv, 2026-08-17, on vc's recommendation)

`transitions.rs` carries four rows dispositioned `Disposition::Unbuilt`: **`Thread.acceptance`, `Criterion.kind`, `AcceptanceTest.kind`, `Issue.status`.** cc measured AC-04.6's second condition and found all four are values authored canon puts there with no verb to move them -- entered and unleaveable. Paying that debt means declaring edges, and declaring edges means declaring machines, which is the same criterion's FIRST condition. **So it was one ruling across four rows, not four calls, and it is the block `intent issues add|close|open` had been standing behind.**

**The ruling is that only ONE of the four is a state machine.** A field that cannot move on its own is not a state variable, it is a **component** of one -- and three of the four cannot move on their own:

| field                 | disposition                                                         |
| --------------------- | ------------------------------------------------------------------- |
| `Issue.status`        | **Machine 4**, below                                                |
| `Criterion.kind`      | folded into **Machine 3** as a `(kind, state)` pair; no new machine |
| `AcceptanceTest.kind` | folded into the **`AcceptanceTest.status`** machine; no new machine |
| `Thread.acceptance`   | **immutable after creation**; no machine, no edge                   |

**The pairing is enforced already and that is what settles it.** `model.rs:414-432` carries the `kind`/`state` invariant in the JSON Schema face, held by `tests/ac_kind_state_invariant.rs`: `{kind: test, state: satisfied}` records a satisfaction nothing computed, `{kind: non-test, state: computed}` claims a derivation with nothing to derive. **Flipping `kind` alone is schema-invalid**, so a kind conversion is one act moving two fields, which is a transition of the pair rather than of either field. `AcceptanceTest` has the identical shape -- a `(non-test)` AT is `n/a` by definition and can never be green -- so its `kind` folds into its own status machine the same way. **ic hit this from the register side independently, having no notation for a multi-field atomic move; that gap was diagnostic rather than clerical.**

**`Thread.acceptance` is `Option<AcceptanceMode>` -- `exempt` or absent.** That is an attribute of a thread, not a lifecycle: changing it is AUTHORING, not a transition, and it gets no verb.

**Correction to vc's own shorthand, made here rather than carried into the ratified text.** The recommendation hv adopted said "widen Machine 3 over the (kind, state) pair" for **both** `kind` rows. That is wrong as written: `AcceptanceTest.kind` pairs with `AcceptanceTest.status`, not with the criterion's state. **Each `kind` folds into the machine of the entity that owns it.** The shape of the recommendation is what was ruled; transcribing the shorthand literally would have put `AcceptanceTest.kind` in the wrong machine.

#### Machine 4 -- Issue (`IssueStatus`)

States: `Open` | `Closed`. **Entry: `Open`.** Declared from v2's MEASURED behaviour rather than designed, because `intent issues` is `keep`-classified and v3 reproduces it.

| From     | To       | Verb           | Guard |
| -------- | -------- | -------------- | ----- |
| _(none)_ | `Open`   | `issues add`   | --    |
| `Open`   | `Closed` | `issues close` | --    |
| `Closed` | `Open`   | `issues open`  | --    |

**No guards, deliberately.** v2 has none, the row is `keep`, and inventing one here would be a parity break wearing a ratification. **Self-loops are accepted at exit 0 per the rule above** -- which is exactly v2's `already CLOSED` behaviour, and the reason the self-loop question had to be settled before this machine could be declared.

**What this discharges.** cc's block on the three `intent issues` mutations is lifted. Three of the four owed mutations become non-mutations: two are pair transitions inside existing machines, and `Thread.acceptance` is owed nothing at all. **AC-04.6's `Unbuilt` rows are re-dispositioned accordingly rather than built as four new machines**, which is materially less surface than the row implied.

### What this gives cc's `transitions.rs`

The mutation-completeness walk built on 2026-08-15 currently DISCOVERS edges from the service layer and checks the graph is closed. With these tables ratified it gets a **declared** graph to check the implementation against, so the walk changes from "is the code closed?" to "does the code implement the ratified machine, exactly?" -- and a missing verb becomes a red test rather than a fact nobody noticed until a WP could not be reopened.
