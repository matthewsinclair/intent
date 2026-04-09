---
verblock: "06 Mar 2026:v0.1: matts - Initial version"
intent_version: 2.6.0
status: Completed
slug: add-in-cost-analysis-skill
created: 20260306
completed: 20260307
---

# ST0027: Add /in-cost-analysis skill

## Objective

Add a new `/in-cost-analysis` skill that estimates the development cost of reproducing any codebase from scratch. The skill is language-agnostic, uses a Bash script for automated metrics collection, and produces a structured cost estimate report.

## Context

A candidate skill existed at `~/tmp/in-cost-analysis-skill.md` that was project-specific (Swift/Metal) and overly long (~360 lines with ROI analysis). This steel thread generalizes it into a clean, reusable Intent skill following existing conventions.

Key decisions:

- Drop ROI analysis (pure cost estimation only)
- Bash supporting script for automated metrics collection
- Reference data (rate tables, multipliers) in a separate `data/` file
- Follows `in-autopsy` pattern (SKILL.md + scripts/ + data/)

## Related Steel Threads

- ST0026: Intent v2.6.0 (skills system matured here)
