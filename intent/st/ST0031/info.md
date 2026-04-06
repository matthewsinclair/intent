---
verblock: "06 Apr 2026:v0.7: matts - WP-06 complete: case study, theory guides, exercises, syllabus"
intent_version: 2.8.0
status: WIP
slug: agentic-coding-course-using-intent
created: 20260404
completed:
---

# ST0031: Agentic Coding Course

## Objective

Build a forensically detailed, evidence-backed course on agentic coding, mined from 18 months of real development across 12+ repos (3750+ commits, 155 Claude Code sessions, 1.46GB of session data). The course serves two audiences via shared content atoms with different pathways: sophisticated individual/small-team users, and enterprise engineering teams.

## Context

The user has built real software at scale using Claude Code for 18 months. The raw material includes:

- **Lamplight**: 1496 commits, 119 steel threads (96 done, 17 cancelled), 70 sessions (861MB). Complex Elixir umbrella app with significant architectural evolution. Richest source.
- **MeetZaya**: 1536 commits, 65 steel threads, 17 CLAUDE.md edits. 12 months of intensive development that FAILED -- invaluable failure case study.
- **Intent**: 270 commits, 31 steel threads, 28 CLAUDE.md edits. Process-focused -- the methodology tool itself.
- **Conflab, Laksa, Molt, Prolix**: Additional repos with varying richness.

A real consulting client wants this as a deliverable. Format: 5 days, each with 1-1.5hr theory + 1-1.5hr practical morning sessions, then self-directed afternoon work.

The fundamental content unit is the **Agentic Coding Insight (ACI)** -- a named, categorized, evidence-backed lesson. ACIs are atoms; course days are molecules. Two audience pathways traverse the same ACI pool in different sequences.

## Related Steel Threads

- ST0030: Cherry-Pick Superpowers Patterns (in-verify, in-debug, in-review, Red Flags, chains_to) -- direct methodology precedent
- ST0028: TCA v3.0 -- dual-stream structure precedent (docs + skills)
- ST0026: Skill system foundation, STZero retrofit
- ST0025: Highlander audit -- architecture lessons

## Context for LLM

This is a research-heavy ST. The first 3 WPs involve deep forensic analysis of git histories, session logs, and steel thread artifacts across multiple repos. The extraction protocol uses 6 "lenses" (rule archaeology, plan-outcome delta, correction mining, architecture forensics, methodology evolution, failure archaeology).

Key constraint: MeetZaya failed for both coding and non-coding reasons. The non-coding reasons require user interview -- they can't be extracted from data alone.

Course content goes in `docs/course/`. Steel thread artifacts stay in `intent/st/ST0031/`.
