# Data model - ST0056: the reified Intent model (WP-01 spec)

Status: ratified structure per design.md D01-D05. WP-02 landed the schemars faces, so **this document describes, it does not define** -- the authored master is the Rust type layer and the committed faces under `schema/` are generated from it. The WP-01 draft schema that stood at the foot of this file is pruned; see "The schema face" below.

Amendments after WP-01 (vc, 2026-08-14, ADOPTED under hv standing authorisation): `objective`/`context`/`related` modelled on `steel_thread`; the marked-legacy `legacy` form on `acceptance_test`; the no-clock law on generated views. Each carries its rationale inline below.

## Entities

Identity convention (D15): natural keys stay human-legible; `(project_id, natural_id)` is the global identity. All dates ISO 8601 (`YYYY-MM-DD`); all timestamps UTC RFC 3339.

### Time convention (D42, AC-02.8 -- ruled 2026-08-15)

**Two kinds of time live in this schema and they must never be one column.** The tables below carry (b); (a) is the column AC-02.8 adds, and zero of eight tables had it.

- **(a) RECORD timestamp** -- when THIS store wrote THIS row. Set by the database as part of the write (a `DEFAULT`, never a caller value). Per-machine, **not carried in the extract**, and **correctly re-stamped on every rebuild** -- the row genuinely was written then.
- **(b) DOMAIN timestamp** -- when the thing happened in the project's history (`threads.created`, `threads.completed`, `issues.created`). **Carried in the extract, never re-stamped**, and displayed by `st show` / `st list` and the `.md` views.

**Replacing (b) with (a) means a colleague who clones and rebuilds sees every thread created today.** The two doors keep them apart: **create** (the DB stamps) and **restore** (the recorded stamp is carried). Restoring is not creating.

Per-table naming, ruled on cc's measurement that the store has **no `UPDATE` anywhere** -- every write is `DELETE` + `INSERT`, so an `ON UPDATE` trigger can never fire and a `created_at` would silently record the latest write:

| tables                                | columns                     | why                                                                                          |
| ------------------------------------- | --------------------------- | -------------------------------------------------------------------------------------------- |
| `threads`, `issues`, `file_index`     | `created_at` + `updated_at` | upserted, so the row survives: `created_at` fires once, `updated_at` moves DB-side           |
| `related`, `wps`, `criteria`, `tests` | `written_at`                | replaced wholesale, so the honest record is _when this version of this row was written_      |
| `event_log`                           | `ts` (already)              | `ts` IS the record timestamp and rows are immutable -- no second column, and the DDL says so |

**A column is named for what it can honestly record, never for uniformity across tables.** `created_at` on a wholesale-replaced table would be a lie the moment a rebuild ran.

`threads.created` / `completed` are **DERIVED FROM THE EVENT LOG** -- the `ts` of the thread's `st.new` and `st.done`/`st.cancel` events. Those stamps are DB-set and are the one thing that merges across machines under D34, so (b) is a time that went end-to-end through the database. A v2-migrated thread has an authored `created:` and no `st.new`, so migration **restores** an `st.new` carrying the authored date. `issues.created` stays authored -- users write it in frontmatter and it is genuinely a fact about the world.

**Scope call with its reversibility** (D39): `wps` and `criteria` have stable IDs, so wholesale-replace is a property of today's write strategy, not of the domain. Delete-missing + upsert-present is the upgrade path and `written_at` does not block it.

### project (`intent/.config/config.json` -- as today, plus)

| Field          | Type   | Notes                                             |
| -------------- | ------ | ------------------------------------------------- |
| project_id     | uuid   | stamped at migration (D15); never changes         |
| intent_version | string | `3.0.0`+                                          |
| name, author   | string | as v2                                             |
| languages      | array  | as v2 (ST0037)                                    |
| server         | object | RESERVED, absent in v3 (D15); intentc-era binding |
| todo           | object | `{done_watermark: rfc3339}` -- see below          |

#### The todo watermark: a generated view that was its own database

Found by cc at WP-03 when the no-clock law (D23) forced the question of whether `todo.md`'s `## DONE:<timestamp>` heading was render time or data. It is data, and the mechanism is worse than a stray timestamp:

- `bin/intent_todo:20` calls it "the last-flush watermark", advanced only by `done --flush` / `--prune`.
- `read_done_watermark()` at `:157` **greps it back out of the generated `todo.md`**, and falls back to `date -u '+%Y-%m-%dT00:00:00Z'` when the file or heading is absent.

So the view is the only durable home of a fact the tool reads back as truth. Three consequences, all fatal to v3's model: the view cannot be regenerated from truth (deleting `todo.md` silently resets the watermark to start-of-today); a generated artefact is authoritative, which is the exact inverse of D02; and the render path reads a clock, which D23 forbids.

Ruling (vc, 2026-08-14; ADOPTED under hv standing authorisation): the watermark is **durable project state**, homed in `config.json` under a `todo` block, **always materialised and never defaulted at render time**. The render path receives it as an input and never reads it back. The v2 start-of-today fallback does not survive -- a default computed from a clock is the defect wearing a different hat.

Open for hv, and it decides whether this field exists at all: whether `todo --flush` / `--prune` semantics carry into v3. If they retire, the watermark retires with them and DONE filtering becomes a query parameter over the `completed` dates already in the model. The field is provisional precisely because it is downstream of that behaviour question.

### steel_thread (`st/<ID>/thread.json`)

| Field          | Type    | Notes                                                                                                  |
| -------------- | ------- | ------------------------------------------------------------------------------------------------------ |
| schema         | string  | `intent/thread@3.0` -- lets validators pick the schema                                                 |
| id             | string  | `ST0056`                                                                                               |
| title          | string  |                                                                                                        |
| slug           | string  |                                                                                                        |
| objective      | string  | authored prose; may be empty (see below)                                                               |
| context        | string  | authored prose, markdown, carried verbatim                                                             |
| related        | array   | `{id, note?}` -- the Related Steel Threads block                                                       |
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
| objective      | string? | authored prose, the `## Objective` section (D28)                                                                                   |
| body           | string? | authored prose, every other section verbatim (D28) -- `## Deliverables` and `## Dependencies` live here, deliberately unstructured |

#### scope: canonicalisation is not loss, but one v2 value is outside the set

Measured on this repository's own corpus (vc, 2026-08-15, on cc's WP-06 finding): v2 reads `scope` as **free text**, and 129 work packages carry **eleven spellings** -- `Small` 56, `Medium` 34, `Large` 8, `L` 8, `XL` 5, `M` 5, `S` 4, `ExtraSmall` 4, `Extra Small` 3, `XS` 1 -- and `Medium-Large` 1.

**The first ten are `corrected`, and rendering them canonically is not lossy.** The model declares `scope` an enum, so the enum is the truth and the spelling was always incidental presentation of one of six values; `Extra Small` and `XS` carry identical information. "As observed" cannot mean reproducing ten spellings for six sizes, because the thing observed was a free-text field standing in for an enum.

**`Medium-Large` is the eleventh and it decides the rule.** It maps to nothing in `XS · S · M · L · XL · XXL` -- it sits between two of them -- and it lives at `intent/st/COMPLETED/ST0020/WP/09/info.md`, in a **CLOSED** thread. hv's ratified carry policy is that CLOSED threads are lossless-by-carrying and LIVE threads are BLOCKED-until-clean, and **neither is ever lossy**. So all three obvious moves are forbidden at once: normalising it to `M` or `L` is a guess and lossy; blocking on it violates lossless-by-carrying for a closed thread; dropping it is loss outright.

**Ruling: `scope` carries a marked-legacy form for a value outside the enum**, following the precedent this model already sets for `acceptance_test`'s marked-legacy shape. A closed thread carries losslessly, the value stays visible AS legacy rather than being silently canonicalised into a lie, and the enum stays honest for everything new. A LIVE thread carrying an unmappable scope still BLOCKS, per the same policy. The general form is D05's posture applied one level down: an unknown enum VALUE is refused or marked by name, never guessed -- exactly as an unknown FIELD is.

`objective` and `body` exist because `WP/<NN>/info.md` is the same mixed file `steel_thread`'s `info.md` was, and D22 was never applied one level down -- see D28. Without them the WP-10 migration drops every work package's authored prose, which AC-10.5's prose-conservation clause forbids. **The contract already carried that gate; the model did not carry the field.** `## Acceptance` is not modelled: its text is fixed boilerplate pointing at `acceptance.md`, so it is generated, and restating ACs in a WP view would be the double truth the single-source rule exists to stop.

### acceptance_criterion (inside thread.json)

| Field | Type   | Notes                                                                                                                                                                                                                                                                                                                                                         |
| ----- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| id    | string | `AC-01.1` (group = WP seq or `00` for ST-level)                                                                                                                                                                                                                                                                                                               |
| text  | string |                                                                                                                                                                                                                                                                                                                                                               |
| kind  | enum   | `test · non-test` -- **the DISCRIMINATOR for `state`'s shape below; it already existed and is not being added**                                                                                                                                                                                                                                               |
| state | tagged | `{is: computed}` · `{is: unsatisfied}` · `{is: satisfied, evidence}` · `{is: descoped, to: STxxxx, by?, reason?}` · `{is: withdrawn, reason, by?}` -- **REQUIRED on every criterion.** `computed` is the in-scope value for a `test` criterion; `satisfied`/`unsatisfied` are refused on one. FIVE values, RATIFIED by hv 2026-08-15 -- see "The fifth state" |

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

### The rules these machines obey

- **No terminal states.** Every state has at least one declared exit (D32/AC-04.6). A state that should be hard to leave gets a **guard**, not a missing verb.
- **Every transition names a verb**, reachable from every surface (D32).
- **Guards are declared, not implied** -- eg `Completed` requires a gate PASS.
- **Direct vs Incidental** (cc, 2026-08-15): an edge that exists only as a side effect of changing a different field counts for reachability and **never discharges a trap**.

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

### What this gives cc's `transitions.rs`

The mutation-completeness walk built on 2026-08-15 currently DISCOVERS edges from the service layer and checks the graph is closed. With these tables ratified it gets a **declared** graph to check the implementation against, so the walk changes from "is the code closed?" to "does the code implement the ratified machine, exactly?" -- and a missing verb becomes a red test rather than a fact nobody noticed until a WP could not be reopened.
