---
verblock: "05 Apr 2026:v0.2: matts - As-built: 7 dimensions, 8 landscape ACIs, 2 framework docs"
intent_version: 2.8.0
status: WIP
---

# WP-03: Landscape Research

## Objective

Comprehensive survey of the agentic coding landscape. Produce landscape ACIs and an evaluation framework for tools/methodologies.

## As-Built (2026-04-05)

All 7 dimensions researched. 8 landscape ACIs produced. landscape.md and evaluation-framework.md created. Detrope gate passed.

### Dimension Coverage

| Dimension                  | Coverage | Key finding                                                    |
| -------------------------- | -------- | -------------------------------------------------------------- |
| A: Anthropic guidance      | Solid    | Tool-practice gap is the core course positioning               |
| B: Tool landscape          | Solid    | Agentic-assistive spectrum framework                           |
| C: Skills/plugins          | Solid    | Claude Code has deepest extensibility (5 layers)               |
| D: Open source methodology | Solid    | Near-empty -- this is the course's value proposition           |
| E: Enterprise adoption     | Good     | 10% utilization problem; ROI measurement broken                |
| F: Mental models           | Good     | Delegation model most productive; context engineering emerging |
| G: Notable practitioners   | Good     | Willison, swyx, Thorsten Ball, Paul Gauthier                   |

### Landscape ACIs Produced

ACI-013 (Methodology Desert), ACI-014 (Context Engineering), ACI-015 (Agentic-Assistive Spectrum), ACI-016 (Skills as Procedural Memory), ACI-017 (Delegation Model), ACI-018 (Ten Percent Problem), ACI-019 (Session Lifecycle), ACI-020 (Tool-Practice Gap)

### What's Missing

- Live web verification needed for post-mid-2025 developments (see Freshness Notes in landscape.md)
- Tool comparison matrix (not produced as standalone artifact; data is in landscape.md tables)
- Reading list / resource guide (sources listed in landscape.md but not formatted as standalone handout)
- obra/Jesse Vincent "Superpowers" content needs proper sourcing

## Deliverables

- [x] Landscape survey (`docs/course/landscape.md`) covering 7 dimensions
- [ ] Tool comparison matrix (data captured in landscape.md tables, not standalone)
- [x] Evaluation framework (`docs/course/evaluation-framework.md`)
- [x] 8-12 landscape ACIs (8 produced: ACI-013 through ACI-020)
- [ ] Reading list / resource guide (sources in landscape.md, not standalone)

## Acceptance Criteria

- [x] All 7 landscape dimensions have substantive coverage
- [x] Evaluation framework is concrete enough to actually use (6-dimension scoring rubric with sample evaluations)
- [x] Landscape ACIs follow standard ACI format

## Dependencies

WP-01 (need ACI format). Runs in PARALLEL with WP-02.
