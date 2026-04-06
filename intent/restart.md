# Claude Code Session Restart

## WIP

ST0031: Agentic Coding Course -- WP-06 complete. WP-07 (Packaging & Delivery Assets) is next and has a detailed plan at `.claude/plans/immutable-cooking-simon.md`.

## What's Done (WP-01 through WP-06)

The course content is complete: 48 files, ~440KB across `docs/course/`. Specifically:

- **26 ACIs** in `docs/course/content/insights/` -- all polished, all pass detrope, frontmatter categories corrected, cross-references verified
- **5 theory guides** in `docs/course/content/days/` (day-1 through day-5) -- facilitator session plans with ACI sequencing, teaching notes, transitions, dual pathway sections (individual + enterprise)
- **5 exercise documents** in `docs/course/content/exercises/` (day-1 through day-5) -- combined facilitator/student instructions with setup, verification criteria, stretch goals
- **MeetZaya case study** `docs/course/content/meetzaya-case-study.md` -- 2,984 words, 3 parts (technical trajectory, strategic failure + full "10 Rules" self-audit, When Code is Free thesis)
- **Course syllabus** `docs/course/content/syllabus.md` -- overview, prerequisites, course arc, assessment approach, materials list, evidence base
- **Outline** `docs/course/outline.md` -- 5-day structure, ACI sequencing, practical sessions, dual pathways
- **Taxonomy** `docs/course/taxonomy.md` -- 5 categories, 26 ACIs, cross-reference map, gap analysis
- **Evaluation framework** `docs/course/evaluation-framework.md` -- 6 dimensions, 1-5 rubrics, sample evaluations (already standalone, no WP-07 action needed)
- **Landscape survey** `docs/course/landscape.md` -- 7 dimensions, tool comparison (7 tools), 11 reading sources
- **Process docs**: extraction-protocol.md, cross-repo-patterns.md, detrope-checklist.md, how-we-built/
- **Meta-chapter** `docs/course/content/how-this-course-was-built.md` -- updated through WP-06

## What's Next: WP-07 (Packaging)

**Plan**: `.claude/plans/immutable-cooking-simon.md` -- approved, ready to execute.

**11 new files**, all in `docs/course/delivery/` (new directory):

| Phase | Files                                                      | Purpose                                            |
| ----- | ---------------------------------------------------------- | -------------------------------------------------- |
| 1     | `README.md` + 5 `day-{1-5}-summary.md`                     | Entry point + desk reference cards                 |
| 2     | `sample-repo-spec.md` + `reading-list.md`                  | Practical session prep + annotated resources       |
| 3     | `enterprise-supplement.md`                                 | ROI framework, governance, risk (the CTO document) |
| 4     | `facilitation-guide.md` + `day-5-facilitation-addendum.md` | Logistics + storytelling guidance                  |
| 5     | Tracking doc updates, consistency checks                   | Integration + mark ST0031 complete                 |

**Key decisions already made** (in plan):

1. Sample repo: specification document only, not actual construction
2. Per-day handouts: summary sheets (new value), not exercise extracts (would duplicate)
3. Eval framework: already standalone, README references it
4. Tool comparison matrix: folded into reading list (data from landscape.md)
5. Enterprise materials: one supplement document, not separate templates
6. Facilitation: logistics guide + separate Day 5 addendum with blog 0196 integration

## Key Files

| File                                                             | Purpose                                                  |
| ---------------------------------------------------------------- | -------------------------------------------------------- |
| `.claude/plans/immutable-cooking-simon.md`                       | WP-07 plan (5 phases, 5 commits)                         |
| `intent/st/ST0031/tasks.md`                                      | Task tracking (WP-01-06 done, WP-07 tasks listed)        |
| `intent/st/ST0031/meetzaya-interview-notes.md`                   | Full 10 Rules + 2 bonus, 8 failure reasons, blog refs    |
| `docs/course/content/syllabus.md`                                | Course overview (materials list for README cross-ref)    |
| `docs/course/outline.md`                                         | Source for summary sheets (ACI sequencing per day)       |
| `docs/course/content/days/`                                      | Source for facilitation guide (timing, pathway notes)    |
| `docs/course/content/exercises/`                                 | Source for sample repo spec (setup requirements per day) |
| `docs/course/content/insights/ACI-025-the-productivity-claim.md` | Source for enterprise supplement ROI data                |
| `docs/course/landscape.md`                                       | Source for reading list + tool comparison data           |

## External References (for WP-07 content)

- Blog 0194: https://matthewsinclair.com/blog/0194-the-time-cost-of-how-is-zero (reading list: essential)
- Blog 0195: https://matthewsinclair.com/blog/0195-taste-as-art (reading list: essential)
- Blog 0196: https://matthewsinclair.com/blog/0196-storytelling-in-a-post-truth-world (Day 5 facilitation addendum)
- Blog 0197: https://matthewsinclair.com/blog/0197-the-expanding-pie-and-the-cleanup-bill (enterprise supplement)
- Blog source files: `/Users/matts/Devel/prj/Sites/matthewsinclair/posts/2026/` (0194-0197)
- "10 Rules": in meetzaya-interview-notes.md and meetzaya-case-study.md

## Course Structure (as-built)

| Day | Theme            | ACIs                                           | Content                                                       |
| --- | ---------------- | ---------------------------------------------- | ------------------------------------------------------------- |
| 1   | The Landscape    | 015, 013, 018, 020 (4, all foundational)       | Theory guide + exercises                                      |
| 2   | The Conversation | 017, 014, 024, 003, 007 (5, 1f + 4i)           | Theory guide + exercises                                      |
| 3   | The Architecture | 002, 008, 006, 001, 011, 016 (6, 4f + 2i)      | Theory guide + exercises                                      |
| 4   | The Process      | 005, 019, 022, 021, 009, 012 (6, 1f + 3i + 2a) | Theory guide + exercises                                      |
| 5   | The Reckoning    | 010, 023, 004, 026, 025 (5, 1i + 4a)           | Theory guide + exercises + 3-part case study + eval framework |

## Conventions

- ALWAYS use `intent` CLI for ST/WP operations
- NEVER manually wrap lines in markdown
- Detrope every content file (mechanical + full LLM, <2/1000 words; <1/1000 for enterprise-facing)
- NO Claude attribution in commits
- Separate process/ from content/ from delivery/ in docs/course/
