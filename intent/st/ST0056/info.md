---
verblock: "14 Aug 2026:v0.2: vc - Objective + context ratified with hv; LLM Preamble folded into design.md and removed"
intent_version: 2.19.0
status: WIP
slug: add-a-rust-based-cli-with-a-local-sqlite-db-with
created: 20260814
completed:
---

# ST0056: Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files that exposes an MCP server with full API access to Intent

## Objective

Ship **Intent v3.0.0**: replace the v2 shell implementation with a native Rust system -- `intentsvcs` (the library owning all functionality, the SQLite store, and the file canon), `intent` (a thin-coordinator CLI running in-process or via GraphQL), and `intentd` (one daemon per machine serving N projects) -- built around a reified, schema-validated data model. Markdown is always realised on disk (generated views for structure, the authoring surface for prose); the full surface is exposed as both CLI and MCP; an automated migrator brings v2.19.0+ projects across with refuse-lossy discipline; distribution is `brew install intent`. One major release, patched by 3.0.x.

## Context

v2 is 12,492 lines of bash across 27 files where every reader reimplements parsing -- the "answers confidently from partial evidence" bug class that v2.19.0 spent five of its fifteen issues on, and that v2 tried to patch by bolting schema onto markdown three separate times (0012, 0017, the close-gate). v3 reifies the model instead: one schema authored once in the type layer, generating its JSON Schema / SQL DDL / GraphQL SDL faces; committed JSON as durable truth; a rebuildable per-project SQLite DB as runtime truth; markdown demoted to generated views plus authored prose.

Constraints ratified with hv (2026-08-14): .md realisations always on disk; thin coordinator at every seam (the CLI speaks only the intentsvcs facade or GraphQL); Rust native, macOS first then Linux; Elixir-oriented (SDL + JSON Schema artefacts, Phoenix-channels-shaped cloud seam); AI-agent aware (CLI + MCP parity); intentd in the 3.0.0 gate. Prior art: Lamplight `native/cli` (dispatch-spine SSOT, typed transport errors, MCP bridge) and Conflab `native/daemon` (the conflabd stack: async-graphql + axum, rmcp streamable HTTP, launchd lifecycle owned by the CLI, debounced watching, policy-stamp self-healing). Full architecture, decision log (D01-D17), and alternatives: `design.md`. Work breakdown: `tasks.md` + the 12-WP ladder.

Stretch goals are parked as their own 3.x steel threads: TUI dashboard, the agent bus (with the whiteboard restructure and hv oversight gates), Laksa web page, macOS menubar app, `intent_ex` hex client.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- ST0043: Rethink `intent upgrade` -- the v2 convergent orchestrator; v3's migrator supersedes it, and its ledger is the first hop of the two-hop migration policy
- ST0044: acceptance.md + the AC/AT process -- the contract model v3 reifies as first-class entities
- ST0045: Whiteboard Protocol 3.0 -- deferred to the 3.2 agent-bus ST; md-authored through 3.0.0/3.1

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
