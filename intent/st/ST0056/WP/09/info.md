---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-09
title: "MCP server and agent guide"
scope: L
status: Not Started
---

# WP-09: MCP server and agent guide

## Objective

Expose the full surface to agents: an rmcp-based MCP server with the tiered typed tools + GraphQL escape hatch, and the generated agent guide -- all rendered from the same dispatch-table SSOT as the CLI.

## Deliverables

- `intent mcp` stdio server: ~10-12 typed high-traffic tools (st/wp lifecycle, ac/at, issues, todo, search, wb, doctor) with schemars-derived params (the conflabd `tool_router` pattern)
- The `intent_graphql` escape hatch tool: full API access without tool bloat
- MCP resources for read surfaces (wip, boards, ST docs)
- Bridge mode: when intentd is up, stdio relays with per-request target resolution (the Lamplight `mcp.rs` pattern -- daemon restarts never strand a session)
- `intent llm`: the agent guide generated from the dispatch table (no hand-maintained prose list)

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-09` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-05 (stdio mode needs only the facade); WP-08 for bridge mode.
