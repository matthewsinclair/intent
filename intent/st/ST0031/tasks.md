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

## WP-02: Pilot Extraction -- Intent + Lamplight + MeetZaya (Done, 2 items deferred)

- [x] Lens 1 (Rule Archaeology): Intent CLAUDE.md evolution (28 edits)
- [x] Lens 1 (Rule Archaeology): Lamplight CLAUDE.md evolution (10 edits)
- [x] Lens 1 (Rule Archaeology): MeetZaya CLAUDE.md evolution (17 edits)
- [x] Lens 2 (Plan-Outcome Delta): Intent STs (4 sampled: ST0025, 0026, 0028, 0030)
- [x] Lens 2 (Plan-Outcome Delta): Lamplight STs (4 sampled: ST0037, 0044, 0046, 0048)
- [x] Lens 2 (Plan-Outcome Delta): MeetZaya STs (5 sampled: ST0038, 0039, 0040, 0062, 0065)
- [x] Lens 3 (Correction Mining): Intent sessions (14 files, autopsy run)
- [ ] Lens 3 (Correction Mining): Lamplight sessions (70 files, 861MB -- DEFERRED to WP-05 pending gap analysis)
- [x] Lens 4 (Architecture Forensics): Lamplight major refactors (cafe namespace, app rename oscillation)
- [x] Lens 4 (Architecture Forensics): MeetZaya architecture evolution (regression cascade)
- [x] Lens 5 (Methodology Evolution): Cross-repo timeline (3 repos)
- [x] Lens 6 (Failure Archaeology): MeetZaya failure trajectory (from ST artifacts + git history)
- [x] Lens 6 (Failure Archaeology): Lamplight cancelled STs (17 analyzed)
- [x] User interview: MeetZaya non-coding failure reasons (DONE -- 8 failure reasons, 10 Rules self-audit, blog refs)
- [x] Compile raw ACI candidates (22 total, target was 20-25)
- [x] Protocol calibration notes (all 6 lenses checked off in extraction-protocol.md)
- [x] Detrope gate: mechanical 0 flags, full LLM analysis (26 flags found and fixed)

## WP-03: Landscape Research (Done, 2 items deferred to WP-07)

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
- [x] Live web verification of post-mid-2025 developments (done: tool table, Section A, C updated; Cursor needs manual check)
- [ ] Tool comparison matrix as standalone artifact (DEFERRED to WP-07 -- data exists in landscape.md)
- [ ] Reading list / resource guide as standalone artifact (DEFERRED to WP-07 -- sources exist in landscape.md)

## WP-04: Taxonomy & Course Structure

- [x] Cluster all ACIs (internal + landscape) into 5 refined categories
- [x] Build taxonomy document with definitions (`docs/course/taxonomy.md`)
- [x] Map ACIs to 5-day structure (4/4/6/6/2 distribution)
- [x] Design individual pathway (sequencing + framing per day)
- [x] Design enterprise pathway (sequencing + framing per day)
- [x] Gap analysis: Day 2 tight (need prompt quality ACI), Day 5 thin (need recovery + productivity ACIs)
- [x] Create `docs/course/taxonomy.md`
- [x] Create `docs/course/outline.md`
- [x] Update ACI frontmatter categories for 3 migrated ACIs (009, 012, 014)
- [x] Update `how-this-course-was-built.md` with WP-02/03 process notes
- [x] Full detrope pass on ACIs 010-011, 015-022 (mechanical + LLM)

## WP-05: Scale Extraction (Done)

- [x] Priority 1: Lamplight Lens 4 → ACI-023 The Recovery Protocol (Day 5)
- [x] Priority 2: Laksa+Conflab Lens 2 → ACI-024 The Prompt That Worked (Day 2)
- [x] Priority 3: Cross-repo Lens 5 → ACI-025 The Productivity Claim (Day 5)
- [ ] Priority 4: Lamplight Lens 3 autopsy (SKIPPED -- gaps filled by 1-3)
- [ ] Priority 5: Molt/Prolix Lens 1 CLAUDE.md (SKIPPED -- folded into cross-repo patterns)
- [x] Identify cross-repo patterns (`docs/course/process/cross-repo-patterns.md`)
- [x] Fill taxonomy gaps from WP-04 (Day 2: 4→5, Day 5: 2→4)
- [x] Update taxonomy with new ACIs (3 added, total 25)
- [x] Update outline with new ACI placements and pathway framing
- [x] Detrope gate on all 3 new ACIs (mechanical + full LLM, all pass)
- [x] MeetZaya non-coding failure interview (UNBLOCKED -- 8 failure reasons captured)
- [x] ACI-026 The Generation Trap (Day 5, failure, advanced) -- from interview + blog posts
- [x] Detrope gate on ACI-026 (mechanical + full LLM, pass)
- [x] Updated taxonomy (26 ACIs), outline (Day 5 restructured with 3-part case study)

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
