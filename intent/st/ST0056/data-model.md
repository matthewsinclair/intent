# Data model - ST0056: the reified Intent model (WP-01 spec)

Status: ratified structure per design.md D01-D05. WP-02 landed the schemars faces, so **this document describes, it does not define** -- the authored master is the Rust type layer and the committed faces under `schema/` are generated from it. The WP-01 draft schema that stood at the foot of this file is pruned; see "The schema face" below.

Amendments after WP-01 (vc, 2026-08-14, ADOPTED under hv standing authorisation): `objective`/`context`/`related` modelled on `steel_thread`; the marked-legacy `legacy` form on `acceptance_test`; the no-clock law on generated views. Each carries its rationale inline below.

## Entities

Identity convention (D15): natural keys stay human-legible; `(project_id, natural_id)` is the global identity. All dates ISO 8601 (`YYYY-MM-DD`); all timestamps UTC RFC 3339.

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

| Field      | Type   | Notes                                                    |
| ---------- | ------ | -------------------------------------------------------- |
| schema     | string | `intent/thread@3.0` -- lets validators pick the schema   |
| id         | string | `ST0056`                                                 |
| title      | string |                                                          |
| slug       | string |                                                          |
| objective  | string | authored prose; may be empty (see below)                 |
| context    | string | authored prose, markdown, carried verbatim               |
| related    | array  | `{id, note?}` -- the Related Steel Threads block         |
| status     | enum   | `not-started · wip · tbc · hold · completed · cancelled` |
| created    | date   |                                                          |
| completed  | date?  |                                                          |
| acceptance | enum?  | `exempt` (ST0048) or absent = enforced                   |
| wps        | array  | work_package records, ordered by seq                     |
| criteria   | array  | acceptance_criterion records                             |
| tests      | array  | acceptance_test records                                  |

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

| Field     | Type    | Notes                                                                                                                              |
| --------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| seq       | int     | rendered `WP-01`                                                                                                                   |
| title     | string  |                                                                                                                                    |
| scope     | enum    | `XS · S · M · L · XL · XXL`, plus a marked-legacy form for a v2 value outside the set (see below)                                  |
| status    | enum    | `not-started · wip · done`                                                                                                         |
| objective | string? | authored prose, the `## Objective` section (D28)                                                                                   |
| body      | string? | authored prose, every other section verbatim (D28) -- `## Deliverables` and `## Dependencies` live here, deliberately unstructured |

#### scope: canonicalisation is not loss, but one v2 value is outside the set

Measured on this repository's own corpus (vc, 2026-08-15, on cc's WP-06 finding): v2 reads `scope` as **free text**, and 129 work packages carry **eleven spellings** -- `Small` 56, `Medium` 34, `Large` 8, `L` 8, `XL` 5, `M` 5, `S` 4, `ExtraSmall` 4, `Extra Small` 3, `XS` 1 -- and `Medium-Large` 1.

**The first ten are `corrected`, and rendering them canonically is not lossy.** The model declares `scope` an enum, so the enum is the truth and the spelling was always incidental presentation of one of six values; `Extra Small` and `XS` carry identical information. "As observed" cannot mean reproducing ten spellings for six sizes, because the thing observed was a free-text field standing in for an enum.

**`Medium-Large` is the eleventh and it decides the rule.** It maps to nothing in `XS · S · M · L · XL · XXL` -- it sits between two of them -- and it lives at `intent/st/COMPLETED/ST0020/WP/09/info.md`, in a **CLOSED** thread. hv's ratified carry policy is that CLOSED threads are lossless-by-carrying and LIVE threads are BLOCKED-until-clean, and **neither is ever lossy**. So all three obvious moves are forbidden at once: normalising it to `M` or `L` is a guess and lossy; blocking on it violates lossless-by-carrying for a closed thread; dropping it is loss outright.

**Ruling: `scope` carries a marked-legacy form for a value outside the enum**, following the precedent this model already sets for `acceptance_test`'s marked-legacy shape. A closed thread carries losslessly, the value stays visible AS legacy rather than being silently canonicalised into a lie, and the enum stays honest for everything new. A LIVE thread carrying an unmappable scope still BLOCKS, per the same policy. The general form is D05's posture applied one level down: an unknown enum VALUE is refused or marked by name, never guessed -- exactly as an unknown FIELD is.

`objective` and `body` exist because `WP/<NN>/info.md` is the same mixed file `steel_thread`'s `info.md` was, and D22 was never applied one level down -- see D28. Without them the WP-10 migration drops every work package's authored prose, which AC-10.5's prose-conservation clause forbids. **The contract already carried that gate; the model did not carry the field.** `## Acceptance` is not modelled: its text is fixed boilerplate pointing at `acceptance.md`, so it is generated, and restating ACs in a WP view would be the double truth the single-source rule exists to stop.

### acceptance_criterion (inside thread.json)

| Field     | Type    | Notes                                                                                                                             |
| --------- | ------- | --------------------------------------------------------------------------------------------------------------------------------- |
| id        | string  | `AC-01.1` (group = WP seq or `00` for ST-level)                                                                                   |
| text      | string  |                                                                                                                                   |
| kind      | enum    | `test · non-test`                                                                                                                 |
| scope     | object  | `{state: in-scope}` · `{state: descoped, to: STxxxx, by?, reason?}` · `{state: withdrawn, reason, by?}` (the 0013 model)          |
| evidence  | string? | non-test only                                                                                                                     |
| satisfied | bool?   | **non-test only.** Test-backed satisfaction is COMPUTED from covering green ATs, never stored -- storing it would be double truth |

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

Envelope: `{id: ulid, ts: rfc3339, principal, project_id, op, subject: {type, id}, payload}`. Written by every mutation (WP-02). DB-only state must be losable; the event log is the deliberate exception to derivability and is explicitly NOT durable truth in v3 -- it becomes a sync substrate only when intentc gives it somewhere to go.

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
