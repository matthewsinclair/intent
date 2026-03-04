---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-11
title: "TN004 Tech Note (Total Codebase Audit)"
scope: Small
status: Done
---

# WP-11: TN004 Tech Note (Total Codebase Audit)

## Objective

Port the TN004 "Total Codebase Audit" tech note from laksa-web into Intent's documentation as a generic, project-agnostic reference. The source document (910 lines) already describes a language-agnostic process with examples for Elixir, Rust, Swift, and TypeScript -- but it contains project-specific references to laksa-web (ST0058) and Lamplight (ST0098) that need to be generalized.

The result should be a standalone tech note that any Intent-managed project can follow to run a full codebase audit using Intent + Claude Code + Socrates sub-agents.

## Source

`../Laksa/laksa-web/intent/eng/notes/tn004-total-codebase-audit.md` (910 lines, v1.1)

## Target

`intent/docs/total-codebase-audit.md`

## What to Keep (as-is or lightly edited)

- The 5-phase pipeline structure (Provisioning, Component Audit, Synthesis, Review, Remediation)
- Rule set structure and per-ecosystem examples (Elixir, Rust, Swift, TypeScript)
- Component mapping strategies and sizing guidelines
- Sub-agent prompt templates and `max_turns` guidance
- Violation severity schema (P0-P3)
- Synthesis and deduplication process
- Remediation batching strategy (A-E ordering)
- The polyglot/universal framing ("the process is rule-driven, not language-driven")

## What to Generalize

- Replace "ST0058" / "laksa-web" / "Lamplight" / "ST0098" references with generic placeholders or anonymized examples
- Replace project-specific rule examples (R10 "Content Source Rules / serving_mode") with generic placeholders
- Generalize "~260-file Elixir project" stats into ranges ("typical 100-700 file project")
- Appendix F (Lamplight lessons learned): Extract the universal lessons into the main text, drop project-specific details
- Replace specific module names (e.g., `Lamplight.Core.Client.Helpers`) with generic examples

## What to Add

- Brief "Prerequisites" section: what an Intent project needs before running TN004 (RULES.md, component map, sub-agents installed)
- Cross-reference to ST0026 deliverables: "For prevention, see Steel Thread Zero (ST0000)"
- Note that `intent audit quick` (D5b) and `intent audit health` (D7) are the lightweight alternatives; TN004 is the full forensic audit

## What to Drop

- Appendix sections that are purely project-specific data (violation counts, specific file lists)
- Any references to specific people's decisions or review sessions
- Anything that only makes sense in the context of laksa-web or Lamplight

## Acceptance Criteria

- [ ] Tech note exists at `intent/docs/total-codebase-audit.md`
- [ ] No references to laksa-web, Lamplight, ST0058, or ST0098 remain
- [ ] Process is clearly described as applicable to any polyglot project
- [ ] Cross-references to ST0026 deliverables are present
- [ ] Document is self-contained (a reader can follow it without external context)

## Dependencies

- None (documentation-only, independent of all other WPs)
- Can be done at any point in the ST0026 timeline
