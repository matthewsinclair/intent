# Tasks - ST0031: Agentic Coding Course

## WP-01: ACI Format & Extraction Protocol (Done)

- [x] Define final ACI markdown template
- [x] Write 3-5 hand-crafted ACIs from known lessons to validate format (wrote 5)
- [x] Document extraction protocol (6 lenses, tools, commands per lens)
- [x] Evaluate autopsy Elixir script for Lens 3 reuse (verdict: use as-is)
- [x] User review and approval of ACI format
- [x] Create `docs/course/content/templates/aci-template.md`
- [x] Create detrope checklist and integrate into every content step
- [x] Start "How This Course Was Built" meta-chapter
- [x] Full LLM-based detrope analysis on all content files (0 flags, AI signal "low")

## WP-02: Pilot Extraction -- Intent + Lamplight + MeetZaya

- [x] Lens 1 (Rule Archaeology): Intent CLAUDE.md evolution (28 edits)
- [x] Lens 1 (Rule Archaeology): Lamplight CLAUDE.md evolution (10 edits)
- [x] Lens 1 (Rule Archaeology): MeetZaya CLAUDE.md evolution (17 edits)
- [x] Lens 2 (Plan-Outcome Delta): Intent STs (4 sampled: ST0025, 0026, 0028, 0030)
- [x] Lens 2 (Plan-Outcome Delta): Lamplight STs (4 sampled: ST0037, 0044, 0046, 0048)
- [x] Lens 2 (Plan-Outcome Delta): MeetZaya STs (5 sampled: ST0038, 0039, 0040, 0062, 0065)
- [x] Lens 3 (Correction Mining): Intent sessions (14 files, autopsy run)
- [ ] Lens 3 (Correction Mining): Lamplight sessions (70 files, 861MB -- deferred, large corpus)
- [x] Lens 4 (Architecture Forensics): Lamplight major refactors (cafe namespace, app rename oscillation)
- [x] Lens 4 (Architecture Forensics): MeetZaya architecture evolution (regression cascade)
- [x] Lens 5 (Methodology Evolution): Cross-repo timeline (3 repos)
- [x] Lens 6 (Failure Archaeology): MeetZaya failure trajectory (from ST artifacts + git history)
- [x] Lens 6 (Failure Archaeology): Lamplight cancelled STs (17 analyzed)
- [ ] User interview: MeetZaya non-coding failure reasons
- [x] Compile raw ACI candidates (22 total, target was 20-25)
- [x] Protocol calibration notes (all 6 lenses checked off in extraction-protocol.md)
- [x] Detrope gate: mechanical 0 flags, full LLM analysis (26 flags found and fixed)

## WP-03: Landscape Research (parallel with WP-02)

- [x] Dimension A: Anthropic's official agentic coding guidance
- [x] Dimension B: Tool landscape (Claude Code, Cursor, Aider, Windsurf, etc.)
- [x] Dimension C: Skills/plugins ecosystem assessment
- [x] Dimension D: Open source agentic methodology survey
- [x] Dimension E: Enterprise adoption patterns
- [x] Dimension F: Mental models and paradigms
- [x] Dimension G: Notable practitioners and voices
- [x] Write evaluation framework document
- [x] Compile landscape ACIs (8 produced: ACI-013 through ACI-020)
- [x] Create `docs/course/landscape.md`
- [x] Create `docs/course/evaluation-framework.md`
- [ ] Live web verification of post-mid-2025 developments (see Freshness Notes)
- [ ] Tool comparison matrix as standalone artifact
- [ ] Reading list / resource guide as standalone artifact

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
