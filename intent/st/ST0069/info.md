---
st_id: ST0069
title: v3 post-cut: project search, store-backed coordination, and contract drift
status: WIP
created: 2026-08-30
completed:
---

# ST0069: v3 post-cut: project search, store-backed coordination, and contract drift

## Objective

Deliver the post-cut capabilities that ST0056 descoped from 3.0.1, so the shipped v3 line reaches the design it was specified to: project search over the whole repository, with the agent's tool, the CLI and the explorer as three skins of one implementation in intentsvcs; the coordination model moved into the store with a bounded API; and contract drift refused at the schema. Search is elaborated first, in this thread's design.md; the other two legs keep their inherited designs (ST0056 WP-14 and WP-16) until their turn.

## Context

Three work packages moved here whole from ST0056 on 2026-08-30 (hv's ruling, performed by vc) with their criteria and their ST0056 numbers, so that ST0056's gate measured what 3.0.1 shipped rather than what v3 eventually will. hv's sequencing was the warrant: fully ship v3, intentd first, then tree-sitter and full search.

3.0.1 shipped the first search tier only: `intent search` is FTS5 over the store's authored prose, with a generated MCP tool, an empty-index discriminator and a hardened query escaper. No source file is indexed; nothing structural or semantic exists; the explorer's `/search` shells out to the CLI; the skills still answer the Highlander question with `intent modules find` over a hand-maintained registry.

On 2026-09-12 hv asked for a full review of the search thread and its requirements, refined against how Intent now works, and a re-elaboration of the design and the work packages, with no work started: the corpus is the whole source tree; full-text and, if it makes sense, vector search; a structured query door beside the text one; one implementation in intentsvcs exposed equally through the CLI and the explorer's `/search`; and the LLM as a first-class consumer. vc's review is design.md. Its outcome: the umbrella search package is cancelled in favour of packages that each close on their own criteria; the inherited criteria are re-keyed to the packages that deliver them, the old rows withdrawn with the reason on the record; the tiers stand, with the semantic tier third and gated on one decision that is hv's; and the decisions that need hv are listed in the design. The thread stays in Triage until hv triages it.

## Work Packages

| WP    | Title                                                                                                                   | Size | Status      |
| ----- | ----------------------------------------------------------------------------------------------------------------------- | ---- | ----------- |
| WP-01 | Issues get a realised form and a sigil, then join the default declaration                                               | S    | Not Started |
| WP-02 | The v2 tree survives migration and disagrees with the store: ingest bucket files as attachments, then remove the bucket | S    | Not Started |
| WP-13 | Project search: full-text, structural, and the agent search surface                                                     | XL   | Cancelled   |
| WP-14 | Coordination model: whiteboard and inboxes in the store, with a bounded API                                             | L    | Not Started |
| WP-16 | Contract drift: a shipped field with no model row is refused                                                            | S    | Not Started |
| WP-17 | The structured query door: intent search --sql, read-only over the published schema                                     | S    | Done        |
| WP-18 | The corpus: the gitignore-aware repository, two staleness policies, the widened watcher                                 | L    | Done        |
| WP-19 | Lexical search over the whole corpus: the envelope, --json, the MCP tool, intent index                                  | M    | Done        |
| WP-20 | Structural search: tree-sitter symbols per declared language, and the agent canon that uses them                        | L    | Done        |
| WP-21 | The explorer's /search pane                                                                                             | M    | Done        |
| WP-22 | Daemon-served search with daemonless parity                                                                             | M    | Done        |
| WP-23 | Semantic seams: the embedder interface, the Null and HTTP embedders, the vector schema                                  | S    | Done        |
| WP-24 | The LLM boundary: the harness's own search becomes a door into the index                                                | M    | Done        |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0069.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.1 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
