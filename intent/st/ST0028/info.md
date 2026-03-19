---
verblock: "19 Mar 2026:v0.1: matts - Initial version"
intent_version: 2.7.0
status: Done
slug: tca-v3-process-doc-update-skill-suite
created: 20260319
completed: 20260319
---

# ST0028: TCA v3.0 -- Process Doc Update + Skill Suite

## Objective

Update the Total Codebase Audit process document from v2.0 to v3.0, incorporating lessons from two real TCA runs (Conflab ST0055: polyglot 256 files, Lamplight ST0108: umbrella 792 files). Promote TCA from a static process document to an operational skill suite of 5 skills with 3 automation scripts.

## Context

Two forensically detailed feedback reports identified 10 themes of process improvement:

1. Rule precision boundaries (R5 matchable-values-only, R7 defstruct-only)
2. Validated Rust/Swift rules replacing hypothetical ones
3. Ash Framework supplemental rules (A1-A5)
4. Effective file count model for WP sizing
5. Pre-filtering mechanical rules via grep (Phase 0.5)
6. Confidence field on audit findings
7. P2 split into P2a (mechanical) and P2b (refactoring)
8. Cluster dedup by root cause, not rule number
9. Remediation in main conversation, not sub-agents
10. Test optimization with mix test --failed

Two work streams:

- **Stream A**: Update `intent/docs/total-codebase-audit.md` to v3.0
- **Stream B**: Create 5 TCA skills + 3 scripts (`/in-tca-*`)

## Related Steel Threads

- ST0026: Steel Thread Zero (prevention framework)
- Conflab ST0055: Polyglot TCA run (Elixir+Rust+Swift+Lua)
- Lamplight ST0108: Umbrella TCA run (Elixir)

## Context for LLM

This ST has two parallel streams. Stream A updates the reference document with validated improvements. Stream B creates operational skills that make TCA invokable as slash commands. The doc has the "why and what"; the skills have the "how and when".
