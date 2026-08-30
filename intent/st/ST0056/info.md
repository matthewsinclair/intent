---
st_id: ST0056
title: Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files that exposes an MCP server with full API access to Intent
status: WIP
created: 2026-08-14
completed:
---

# ST0056: Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files that exposes an MCP server with full API access to Intent

## Objective

Ship **Intent v3.0.0**: replace the v2 shell implementation with a native Rust system -- `intentsvcs` (the library owning all functionality, the SQLite store, and the file canon), `intent` (a thin-coordinator CLI running in-process or via GraphQL), and `intentd` (one daemon per machine serving N projects) -- built around a reified, schema-validated data model. Markdown is always realised on disk (generated views for structure, the authoring surface for prose); the full surface is exposed as both CLI and MCP; an automated migrator brings v2.19.0+ projects across with refuse-lossy discipline; distribution is `brew install intent`. One major release, patched by 3.0.x.

## Context

v2 is 12,492 lines of bash across 27 files where every reader reimplements parsing -- the "answers confidently from partial evidence" bug class that v2.19.0 spent five of its fifteen issues on, and that v2 tried to patch by bolting schema onto markdown three separate times (0012, 0017, the close-gate). v3 reifies the model instead: one schema authored once in the type layer, generating its JSON Schema / SQL DDL / GraphQL SDL faces; committed JSON as durable truth; a rebuildable per-project SQLite DB as runtime truth; markdown demoted to generated views plus authored prose.

Constraints ratified with hv (2026-08-14): .md realisations always on disk; thin coordinator at every seam (the CLI speaks only the intentsvcs facade or GraphQL); Rust native, macOS first then Linux; Elixir-oriented (SDL + JSON Schema artefacts, Phoenix-channels-shaped cloud seam); AI-agent aware (CLI + MCP parity); intentd in the 3.0.0 gate. Prior art: Lamplight `native/cli` (dispatch-spine SSOT, typed transport errors, MCP bridge) and Conflab `native/daemon` (the conflabd stack: async-graphql + axum, rmcp streamable HTTP, launchd lifecycle owned by the CLI, debounced watching, policy-stamp self-healing). Full architecture, decision log (D01-D17), and alternatives: `design.md`. Work breakdown: `tasks.md` + the 12-WP ladder.

Stretch goals are parked as their own 3.x steel threads: TUI dashboard, the agent bus (with the whiteboard restructure and hv oversight gates), Laksa web page, macOS menubar app, `intent_ex` hex client.

## Related Steel Threads

- ST0043: Rethink `intent upgrade` -- the v2 convergent orchestrator; v3's migrator supersedes it, and its ledger is the first hop of the two-hop migration policy
- ST0044: acceptance.md + the AC/AT process -- the contract model v3 reifies as first-class entities
- ST0045: Whiteboard Protocol 3.0 -- the md-authored boards v3 reifies as `wb_node` / `wb_item` / `wb_message` (D30, WP-14); the deferral to the 3.2 agent-bus ST was argued on transport and did not reach shape, size or searchability

## Work Packages

| WP    | Title                                                                       | Size | Status      |
| ----- | --------------------------------------------------------------------------- | ---- | ----------- |
| WP-01 | Design canon: architecture, data model, migration and parity specs          | L    | Done        |
| WP-02 | Workspace and reified model: intentsvcs types, schema faces, store          | L    | Done        |
| WP-03 | Ingest, views and sync engine                                               | L    | Done        |
| WP-04 | intentsvcs facade: core command families                                    | XL   | WIP         |
| WP-05 | CLI in-process mode and BATS conformance harness                            | L    | WIP         |
| WP-06 | CLI parity long tail                                                        | XL   | WIP         |
| WP-07 | Canon and claude subsystem                                                  | L    | WIP         |
| WP-08 | intentd daemon                                                              | XXL  | WIP         |
| WP-09 | MCP server and agent guide                                                  | L    | WIP         |
| WP-10 | Migration and fleet ingest harness                                          | XL   | WIP         |
| WP-11 | Distribution: cargo-dist, Homebrew, signing                                 | M    | WIP         |
| WP-12 | Cutover and v3.0.0 release                                                  | L    | Not Started |
| WP-13 | Project search: full-text, structural, and the agent search surface         | XL   | Not Started |
| WP-14 | Coordination model: whiteboard and inboxes in the store, with a bounded API | L    | Not Started |
| WP-15 | Skills catalogue triage: KEEP, UPDATE or RETIRE every Intent2-era skill     | L    | Not Started |
| WP-16 | Contract drift: a shipped field with no model row is refused                | S    | Not Started |
| WP-17 | Form DSL: one declaration, TUI and WEB realisers, CRUD through intentsvcs   | XL   | WIP         |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0056.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
