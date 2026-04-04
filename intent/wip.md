---
verblock: "04 Apr 2026:v0.25: matts - ST0031/WP-01 complete"
intent_version: 2.8.0
---

# Work In Progress

## Current State

v2.8.0 with 22 skills, 5 subagents, 462 tests. ST0031/WP-01 complete. WP-02 and WP-03 ready to start in parallel.

## This Session (2026-04-04, session 2)

- Completed ST0031/WP-01: ACI Format & Extraction Protocol
- Created 10 files under `docs/course/` (process/ and content/ split)
- Defined ACI template with 5 mandatory sections and structured frontmatter
- Hand-crafted 5 sample ACIs covering 5 of 6 extraction lenses
- Wrote 6-lens extraction protocol with concrete commands per lens
- Assessed autopsy script for Lens 3 reuse (verdict: use as-is, no changes needed)
- Established detrope quality gate (mechanical + full LLM-based analysis)
- Started "How This Course Was Built" meta-chapter
- Collected 3 external blog post references from user
- Full detrope analysis: 0 flags, AI signal "low" across all content files

## Active Steel Threads

- ST0031: Agentic Coding Course -- WP-01 done, WP-02 and WP-03 next (parallel)

## TODO

- ST0031/WP-02: Pilot extraction on Intent + Lamplight + MeetZaya (20-25 ACI candidates)
- ST0031/WP-03: Landscape research -- 7 dimensions, evaluation framework, 8-12 landscape ACIs
- After WP-02+03: WP-04 (taxonomy + course structure)

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
- Consider peer language skills (in-rust-essentials, in-swift-essentials)
- Run detrope on other Intent docs

## Key References

- ST0031 steel thread: `intent/st/ST0031/`
- Course content: `docs/course/` (process/ for methodology, content/ for deliverables)
- Plan file: `~/.claude/plans/dynamic-hatching-shannon.md`
- Test suite: `tests/run_tests.sh` (22 .bats files, 462 tests)
