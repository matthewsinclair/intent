---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-05
title: "CLI in-process mode and BATS conformance harness"
scope: L
status: WIP
---

# WP-05: CLI in-process mode and BATS conformance harness

## Objective

Ship the `intent` binary in daemonless mode: a clap spine over the facade, the v2 voice and exit codes carried over, and the v2 BATS estate wired up as the conformance harness proving the core families indistinguishable.

## Deliverables

- The dispatch-table SSOT (verb -> facade op + arg spec) feeding clap; later the MCP tool list and `intent llm` agent guide render from the same table (Lamplight DD-6)
- `ok:`/`error:` lowercase voice + exit codes identical to v2 (0023)
- BATS harness retarget: the existing suite runs against an `INTENT_BIN` override; per-test classification into the keep/retire/deviate register
- Core families (st/wp/ac/at/list/show/status/todo) green under the narrowed conformance contract
- Thin-coordinator compliance: the CLI parses, calls the facade, renders -- nothing else

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-05` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-04. The harness retarget + classification halves are design-neutral v2-side work and can start early (IC-friendly).
