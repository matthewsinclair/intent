# Data model - ST0056: the reified Intent model (WP-01 spec)

Status: WP-01 draft, ratified structure per design.md D01-D05. The schemars-generated JSON Schema (WP-02) supersedes the draft schema below the moment it exists; this document then describes, never defines.

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

### steel_thread (`st/<ID>/thread.json`)

| Field      | Type   | Notes                                                    |
| ---------- | ------ | -------------------------------------------------------- |
| schema     | string | `intent/thread@3.0` -- lets validators pick the schema   |
| id         | string | `ST0056`                                                 |
| title      | string |                                                          |
| slug       | string |                                                          |
| status     | enum   | `not-started · wip · tbc · hold · completed · cancelled` |
| created    | date   |                                                          |
| completed  | date?  |                                                          |
| acceptance | enum?  | `exempt` (ST0048) or absent = enforced                   |
| wps        | array  | work_package records, ordered by seq                     |
| criteria   | array  | acceptance_criterion records                             |
| tests      | array  | acceptance_test records                                  |

No verblock: git is the history of structured files. Authored prose files keep the v2 verblock convention unchanged; generated views carry a generated-banner footer instead (the AGENTS.md pattern).

### work_package (inside thread.json)

| Field  | Type   | Notes                       |
| ------ | ------ | --------------------------- |
| seq    | int    | rendered `WP-01`            |
| title  | string |                             |
| scope  | enum   | `XS · S · M · L · XL · XXL` |
| status | enum   | `not-started · wip · done`  |

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

## Canonical JSON form

UTF-8, LF, 2-space indent, trailing newline, object keys in schema-declared order (serde struct order, not alphabetical), arrays in natural order. Tool-written always; hand-edits are legal but validated strictly on ingest (D05).

## Draft JSON Schema: thread.json

Draft for ratification shape only -- superseded by the schemars face in WP-02.

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://intent.dev/schema/thread-3.0.json",
  "type": "object",
  "required": ["schema", "id", "title", "status", "created"],
  "additionalProperties": false,
  "properties": {
    "schema": { "const": "intent/thread@3.0" },
    "id": { "type": "string", "pattern": "^ST[0-9]{4}$" },
    "title": { "type": "string", "minLength": 1 },
    "slug": { "type": "string" },
    "status": {
      "enum": ["not-started", "wip", "tbc", "hold", "completed", "cancelled"]
    },
    "created": { "type": "string", "format": "date" },
    "completed": { "type": ["string", "null"], "format": "date" },
    "acceptance": { "enum": ["exempt"] },
    "wps": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["seq", "title", "scope", "status"],
        "additionalProperties": false,
        "properties": {
          "seq": { "type": "integer", "minimum": 1 },
          "title": { "type": "string" },
          "scope": { "enum": ["XS", "S", "M", "L", "XL", "XXL"] },
          "status": { "enum": ["not-started", "wip", "done"] }
        }
      }
    },
    "criteria": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "text", "kind", "scope"],
        "additionalProperties": false,
        "properties": {
          "id": { "type": "string", "pattern": "^AC-[0-9]{2}\\.[0-9]+$" },
          "text": { "type": "string" },
          "kind": { "enum": ["test", "non-test"] },
          "scope": { "type": "object" },
          "evidence": { "type": "string" },
          "satisfied": { "type": "boolean" }
        }
      }
    },
    "tests": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "kind", "covers", "status"],
        "additionalProperties": false,
        "properties": {
          "id": { "type": "string", "pattern": "^AT-[0-9]{2}\\.[0-9]+$" },
          "kind": { "enum": ["test", "non-test"] },
          "file": { "type": "string", "pattern": "^[^:]*/[^:]*$" },
          "prose": { "type": "string" },
          "covers": {
            "type": "array",
            "items": { "type": "string" },
            "minItems": 1
          },
          "status": { "enum": ["to-write", "red", "green", "n-a"] },
          "note": { "type": "string" }
        }
      }
    }
  }
}
```

Validation posture (D05): `additionalProperties: false` everywhere -- an unknown field is refused by name, never dropped (the serde_ignored discipline at the schema layer). Schema evolution is a schema-version bump, visible in the `schema` field, with the DB rebuilt (D01).

## What is deliberately not modelled

Prose (stored verbatim, FTS-indexed). The whiteboard (D14: md-authored until the 3.2 bus ST). Rules/skills/templates (shipped content, embedded in the binary, indexed at most). wip.md / restart.md (authored tracking prose).
