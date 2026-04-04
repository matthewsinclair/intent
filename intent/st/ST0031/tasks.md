# Tasks - ST0031: Agentic Coding Course

## WP-01: ACI Format & Extraction Protocol

- [ ] Define final ACI markdown template
- [ ] Write 3-5 hand-crafted ACIs from known lessons to validate format
- [ ] Document extraction protocol (6 lenses, tools, commands per lens)
- [ ] Evaluate autopsy Elixir script for Lens 3 reuse
- [ ] User review and approval of ACI format
- [ ] Create `docs/course/templates/aci-template.md`

## WP-02: Pilot Extraction -- Intent + Lamplight + MeetZaya

- [ ] Lens 1 (Rule Archaeology): Intent CLAUDE.md evolution (28 edits)
- [ ] Lens 1 (Rule Archaeology): Lamplight CLAUDE.md evolution (10 edits)
- [ ] Lens 1 (Rule Archaeology): MeetZaya CLAUDE.md evolution (17 edits)
- [ ] Lens 2 (Plan-Outcome Delta): Intent STs (sample of 29 completed)
- [ ] Lens 2 (Plan-Outcome Delta): Lamplight STs (sample from 96 completed)
- [ ] Lens 2 (Plan-Outcome Delta): MeetZaya STs (sample from 52 completed)
- [ ] Lens 3 (Correction Mining): Intent sessions (14 files, 46MB)
- [ ] Lens 3 (Correction Mining): Lamplight sessions (70 files, 861MB)
- [ ] Lens 4 (Architecture Forensics): Lamplight major refactors
- [ ] Lens 4 (Architecture Forensics): MeetZaya architecture evolution
- [ ] Lens 5 (Methodology Evolution): Cross-repo timeline
- [ ] Lens 6 (Failure Archaeology): MeetZaya full case study
- [ ] Lens 6 (Failure Archaeology): Lamplight cancelled STs (17)
- [ ] User interview: MeetZaya non-coding failure reasons
- [ ] Compile raw ACI candidates (target: 20-25)
- [ ] Protocol calibration notes

## WP-03: Landscape Research (parallel with WP-02)

- [ ] Dimension A: Anthropic's official agentic coding guidance
- [ ] Dimension B: Tool landscape (Claude Code, Cursor, Aider, Windsurf, etc.)
- [ ] Dimension C: Skills/plugins ecosystem assessment
- [ ] Dimension D: Open source agentic methodology survey
- [ ] Dimension E: Enterprise adoption patterns
- [ ] Dimension F: Mental models and paradigms
- [ ] Dimension G: Notable practitioners and voices
- [ ] Write evaluation framework document
- [ ] Compile landscape ACIs (target: 8-12)
- [ ] Create `docs/course/landscape.md`
- [ ] Create `docs/course/evaluation-framework.md`

## WP-04: Taxonomy & Course Structure

- [ ] Cluster all ACIs (internal + landscape) into categories
- [ ] Build taxonomy document with definitions
- [ ] Map ACIs to 5-day structure
- [ ] Design individual pathway (sequencing + framing)
- [ ] Design enterprise pathway (sequencing + framing)
- [ ] Gap analysis: identify thin days/categories
- [ ] Create `docs/course/taxonomy.md`
- [ ] Create `docs/course/outline.md`

## WP-05: Scale Extraction

- [ ] Apply protocol to Conflab
- [ ] Apply protocol to Laksa
- [ ] Apply protocol to Molt
- [ ] Apply protocol to Prolix
- [ ] Identify cross-repo patterns
- [ ] Fill taxonomy gaps from WP-04
- [ ] Update taxonomy with new categories if needed

## WP-06: Course Content Production

- [ ] Polish all ACIs into final form
- [ ] Write Day 1 theory guides (individual + enterprise)
- [ ] Write Day 2 theory guides (individual + enterprise)
- [ ] Write Day 3 theory guides (individual + enterprise)
- [ ] Write Day 4 theory guides (individual + enterprise)
- [ ] Write Day 5 theory guides (individual + enterprise)
- [ ] Design practical exercises for each day
- [ ] Write MeetZaya case study document
- [ ] Write course overview / syllabus

## WP-07: Packaging & Delivery Assets

- [ ] Course README with setup instructions
- [ ] Per-day handout materials
- [ ] Sample/exercise repo for practical sessions
- [ ] Enterprise-specific framing materials (ROI, governance)
- [ ] Evaluation framework as standalone handout
- [ ] Reading list / resource guide

## Dependencies

```
WP-01 (format + protocol)
  |
  +---------+---------+
  |                   |
  v                   v
WP-02 (pilot)    WP-03 (landscape)
  |                   |
  +---------+---------+
            |
            v
       WP-04 (taxonomy + structure)
            |
            v
       WP-05 (scale extraction)
            |
            v
       WP-06 (content production)
            |
            v
       WP-07 (packaging)
```

WP-02 and WP-03 are parallel after WP-01.

## Verification

- [ ] After each WP: review deliverables against acceptance criteria
- [ ] User sign-off at each WP boundary
- [ ] End-to-end: facilitator can deliver course from `docs/course/` alone
