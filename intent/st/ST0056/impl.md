# Implementation - ST0056: Intent v3.0.0

## Implementation

Nothing built yet -- WP-01 (design canon) is the gate before any Rust exists. This document records as-built state and deltas from `design.md` as the ladder executes; the stack shortlist with provenance (Lamplight / Conflab) lives in `design.md`.

## Technical Details

Reference implementations trawled 2026-08-14:

- **Lamplight `native/cli`** (15k lines): dispatch-spine SSOT (verb table feeding clap + daemon + generated agent guide), typed transport errors rendering remedies with full cause chains, `serde_ignored` inbound refusal, MCP stdio bridge with per-request target resolution.
- **Conflab `native/daemon`** (conflabd): async-graphql + axum + rmcp streamable HTTP, rusqlite index with SHA-256 skip-unchanged file sync (`db_sync.rs`), CLI-owned launchd lifecycle (`daemon_cmd/lifecycle.rs`), mgmt plane split from domain API, policy-stamp self-healing on boot, `notify-debouncer-full` + `ignore` watching, `serial_test` file_locks for HOME-mutating tests, Apple TN3171 cert policy (395-day leaves).

## Challenges & Solutions

(recorded as encountered)
