---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-10
title: Intent Usage-Rules.md
scope: Medium
status: Done
---

# WP-10: Intent Usage-Rules.md

## Objective

Create a comprehensive usage-rules.md for Intent itself, following the Ash usage-rules.md pattern, so LLMs know how to use Intent correctly.

## Deliverables

- `usage-rules.md` in Intent's package root (<500 lines)

## Content Structure

1. **What Intent is** — one paragraph
2. **Project structure** — directory layout with purposes
3. **Core commands** — every command with usage, options, examples
4. **Steel thread methodology** — create, manage, complete steel threads
5. **LLM guidance file conventions** — AGENTS.md, RULES.md, ARCHITECTURE.md
6. **Treeindex conventions** — usage and maintenance
7. **Common workflows** — step-by-step for typical tasks
8. **NEVER DO** — common LLM mistakes with Intent

## Format

- Direct imperative voice ("Use X", "Never Y")
- Concrete command examples for every concept
- No preamble or motivation — just "here's how to use this tool"
- Under 500 lines — dense and scannable
- Organized by topic with clear headers

## Acceptance Criteria

- [ ] Covers all Intent commands with examples
- [ ] Follows imperative voice convention throughout
- [ ] Under 500 lines
- [ ] NEVER DO section covers common LLM mistakes
- [ ] An LLM reading this file can correctly use Intent without additional guidance

## Dependencies

- WP-06 (references skill commands)
- WP-07 (references template system)
