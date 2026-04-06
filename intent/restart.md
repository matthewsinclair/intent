# Claude Code Session Restart

## WIP

ST0031: Agentic Coding Course -- WP-04 complete. 22 ACIs organized into 5-category taxonomy, mapped to 5-day course outline with dual audience pathways. Gap analysis done. WP-05 (scale extraction) is next.

## TODO

1. Complete web verification of landscape.md post-mid-2025 claims (started, may need restart)
2. User interview: MeetZaya non-coding failure reasons (blocker for Day 5 case study)
3. WP-05: Scale extraction on Conflab, Laksa, Molt, Prolix -- guided by gap analysis in taxonomy.md
4. After WP-05: WP-06 (content production)

## Key Files

| File                                               | Purpose                                               |
| -------------------------------------------------- | ----------------------------------------------------- |
| `intent/st/ST0031/`                                | Steel thread (info, design, tasks, 7 WPs)             |
| `docs/course/taxonomy.md`                          | 5 categories, cross-refs, gap analysis, WP-05 targets |
| `docs/course/outline.md`                           | 5-day arc with ACI sequencing + dual pathway framing  |
| `docs/course/content/insights/`                    | 22 ACI files (ACI-001 through ACI-022)                |
| `docs/course/landscape.md`                         | External landscape survey (7 dimensions)              |
| `docs/course/evaluation-framework.md`              | Tool/method evaluation rubric                         |
| `docs/course/content/how-this-course-was-built.md` | Meta-chapter (updated through WP-03)                  |
| `docs/course/process/extraction-protocol.md`       | 6-lens protocol (calibration notes updated)           |
| `intent/wip.md`                                    | Work in progress tracker                              |

## Course Structure (from WP-04)

| Day | Theme            | ACIs                              | Key content                |
| --- | ---------------- | --------------------------------- | -------------------------- |
| 1   | The Landscape    | 013, 015, 018, 020                | Ecosystem orientation      |
| 2   | The Conversation | 003, 007, 014, 017                | Communication + delegation |
| 3   | The Architecture | 001, 002, 006, 008, 011, 016      | CLAUDE.md + code structure |
| 4   | The Process      | 005, 009, 012, 019, 021, 022      | Workflow + lifecycle       |
| 5   | The Reckoning    | 004, 010 + case study + framework | Failure + evaluation       |

## Gap Analysis Summary

- Day 2: Tight at 4 ACIs. Need a "prompt quality" ACI from Conflab/Laksa Lens 2.
- Day 5: Thin at 2 ACIs. Need recovery protocol ACI (Lamplight Lens 4) + productivity claim ACI (cross-repo Lens 5). MeetZaya interview still blocked.

## WP-05 Extraction Targets (prioritized)

1. Lamplight Lens 4 → Recovery Protocol ACI (Day 5)
2. Conflab/Laksa Lens 2 → Prompt Quality ACI (Day 2)
3. Cross-repo Lens 5 → Productivity Claim ACI (Day 5 enterprise)
4. Lamplight Lens 3 → deferred autopsy (Day 2/4 if gaps persist)
5. Molt/Prolix Lens 1 → CLAUDE.md patterns in smaller repos (Day 3 depth)

## External References

- [Highlander Rule blog](https://matthewsinclair.com/blog/0189-the-unreasonable-effectiveness-of-the-highlander-rule)
- [Throwing Away Code blog](https://matthewsinclair.com/blog/0188-on-knowing-what-code-to-throw-away)
- [CTO Review Socratic Dialog blog](https://matthewsinclair.com/blog/0182-cto-review-socratic-dialog-ai)

## Project Conventions

- ALWAYS use `intent` CLI for ST/WP operations (never manual file moves)
- NEVER manually wrap lines in markdown files
- Detrope every content file (mechanical + full LLM-based)
- Separate process/ from content/ in docs/course/
- NO Claude attribution in commit messages
- Run `tests/run_tests.sh` before committing
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
