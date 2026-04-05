---
verblock: "05 Apr 2026:v0.26: matts - ST0031 WP-02+WP-03 substantial progress"
intent_version: 2.8.0
---

# Work In Progress

## Current State

v2.8.0 with 22 skills, 5 subagents, 462 tests. ST0031/WP-02 and WP-03 substantially complete. 22 ACIs total (5 existing + 17 new).

## This Session (2026-04-05)

- WP-02 Pilot Extraction: Applied all 6 lenses to Intent + Lamplight + MeetZaya
  - Lens 1 (Rule Archaeology): 4 ACIs (006-009). Cross-repo CLAUDE.md evolution analysis.
  - Lens 2 (Plan-Outcome Delta): 1 ACI (022). Agents analyzed 13 STs across 3 repos.
  - Lens 3 (Correction Mining): Autopsy on Intent sessions (thin corpus, 6 corrections)
  - Lens 4 (Architecture Forensics): 2 ACIs (010-011). Regression cascade + namespace evolution.
  - Lens 5 (Methodology Evolution): 1 ACI (012). Cross-repo adoption timeline.
  - Lens 6 (Failure Archaeology): 1 ACI (021). Batch-creation antipattern from 17 cancelled Lamplight STs.
- WP-03 Landscape Research: 8 landscape ACIs (013-020) + landscape.md + evaluation-framework.md
  - 7 dimensions researched (from training knowledge, web search blocked)
  - landscape.md and evaluation-framework.md created
  - Needs live web verification in future session
- Detrope quality gate: mechanical scan 0 flags; full LLM detrope in progress (batch 2 of 4 done, fixes applied to ACI-012, 013, 014)
- Calibration notes updated in extraction-protocol.md (all 6 lenses checked off)

## Active Steel Threads

- ST0031: Agentic Coding Course -- WP-02 and WP-03 substantially complete, detrope remediation in progress

## TODO

- Complete detrope remediation on remaining ACIs (3 agent batches pending)
- Update how-this-course-was-built.md meta-chapter with WP-02/03 process notes
- Re-run landscape research with web search enabled for live verification
- User interview: MeetZaya non-coding failure reasons (blocker for Lens 6 completion)
- After WP-02+03 sign-off: WP-04 (taxonomy + course structure)

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
- Consider peer language skills (in-rust-essentials, in-swift-essentials)
- Run detrope on other Intent docs

## Key References

- ST0031 steel thread: `intent/st/ST0031/`
- Course content: `docs/course/` (process/ for methodology, content/ for deliverables)
- Test suite: `tests/run_tests.sh` (22 .bats files, 462 tests)
- ACI inventory: 22 files in `docs/course/content/insights/`
