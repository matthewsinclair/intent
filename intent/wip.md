---
verblock: "06 Apr 2026:v0.27: matts - ST0031 WP-04 complete, taxonomy + outline + gap analysis"
intent_version: 2.8.0
---

# Work In Progress

## Current State

v2.8.0 with 22 skills, 5 subagents, 462 tests. ST0031/WP-04 complete. 22 ACIs organized into 5-day course structure with dual audience pathways.

## This Session (2026-04-06)

- WP-02/WP-03 closed (with noted deferrals)
- Full detrope pass on ACIs 010-011, 015-022 (mechanical + LLM, all below 2/1000w target)
- Updated how-this-course-was-built.md with WP-02/03 process notes
- WP-04 Taxonomy & Course Structure: complete
  - 5 refined categories: Landscape & Orientation (4), Agent Communication (4), Codebase Architecture (6), Development Process (6), Failure & Evaluation (2)
  - 3 ACIs migrated: ACI-014 methodology→communication, ACI-009 methodology→process, ACI-012 methodology→process
  - `docs/course/taxonomy.md` created with category definitions, cross-reference map, difficulty distribution, migration log, gap analysis
  - `docs/course/outline.md` created with 5-day arc, ACI sequencing per day, individual + enterprise pathway framing
  - Gap analysis: Day 2 tight (need prompt quality ACI), Day 5 thin (need recovery + productivity ACIs, MeetZaya interview)
  - WP-05 extraction targets prioritized (Lamplight recovery, Conflab/Laksa prompt quality, cross-repo productivity)
- Web verification of landscape data: in progress (agent may have stalled, needs manual completion)

## Active Steel Threads

- ST0031: Agentic Coding Course -- WP-04 done, WP-05 next

## TODO

- Complete web verification of landscape.md post-mid-2025 claims
- User interview: MeetZaya non-coding failure reasons (blocker for Day 5 case study)
- WP-05: Scale extraction on Conflab, Laksa, Molt, Prolix (guided by gap analysis)
- After WP-05: WP-06 (content production)

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
- Consider peer language skills (in-rust-essentials, in-swift-essentials)
- Run detrope on other Intent docs

## Key References

- ST0031 steel thread: `intent/st/ST0031/`
- Course content: `docs/course/` (process/ for methodology, content/ for deliverables)
- Taxonomy: `docs/course/taxonomy.md` (categories + gap analysis)
- Outline: `docs/course/outline.md` (5-day arc + dual pathways)
- Test suite: `tests/run_tests.sh` (22 .bats files, 462 tests)
- ACI inventory: 22 files in `docs/course/content/insights/`
