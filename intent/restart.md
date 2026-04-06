# Claude Code Session Restart

## WIP

ST0031: Agentic Coding Course -- WP-05 complete. 26 ACIs in 5-category taxonomy, mapped to 5-day outline with dual pathways. MeetZaya interview done. Day 5 restructured with 3-part case study. WP-06 (content production) is next.

## TODO

1. WP-06: Content production -- polish ACIs, write 10 theory guides (5 days x 2 pathways), design exercises, write 3-part MeetZaya case study, course syllabus
2. WP-07: Packaging (after WP-06)
3. Note: Blog 0196 (storytelling) for WP-07 facilitation guide

## Key Files

| File                                               | Purpose                                                                |
| -------------------------------------------------- | ---------------------------------------------------------------------- |
| `intent/st/ST0031/`                                | Steel thread with 7 WPs                                                |
| `intent/st/ST0031/tasks.md`                        | Task tracking (WP-01-05 done, WP-06 next)                              |
| `intent/st/ST0031/meetzaya-interview-notes.md`     | MeetZaya non-coding failure interview (8 reasons, 10 Rules, blog refs) |
| `docs/course/taxonomy.md`                          | 5 categories, 26 ACIs, cross-refs, gap analysis (all gaps filled)      |
| `docs/course/outline.md`                           | 5-day arc with ACI sequencing, 3-part Day 5 case study, dual pathways  |
| `docs/course/content/insights/`                    | 26 ACI files (ACI-001 through ACI-026)                                 |
| `docs/course/process/cross-repo-patterns.md`       | 6 cross-repo patterns from WP-05                                       |
| `docs/course/landscape.md`                         | Landscape survey (verified 06 Apr 2026)                                |
| `docs/course/evaluation-framework.md`              | Evaluation rubric                                                      |
| `docs/course/process/extraction-protocol.md`       | 6-lens protocol                                                        |
| `docs/course/content/how-this-course-was-built.md` | Meta-chapter (updated through WP-05)                                   |
| `intent/wip.md`                                    | Session tracker                                                        |

## Course Structure

| Day | Theme            | ACIs                                                    | Status      |
| --- | ---------------- | ------------------------------------------------------- | ----------- |
| 1   | The Landscape    | 013, 015, 018, 020                                      | Strong (4)  |
| 2   | The Conversation | 003, 007, 014, 017, 024                                 | Strong (5)  |
| 3   | The Architecture | 001, 002, 006, 008, 011, 016                            | Strong (6)  |
| 4   | The Process      | 005, 009, 012, 019, 021, 022                            | Strong (6)  |
| 5   | The Reckoning    | 004, 010, 023, 025, 026 + 3-part case study + framework | Strong (5+) |

## Day 5 Case Study Structure

1. Technical Trajectory -- regression cascade, 1536 commits, project cancelled (ACIs 010, 004)
2. Strategic Failure -- co-founder departure, positioning, data moat, underfunding + "10 Rules" self-audit
3. When Code is Free -- time-cost-of-how thesis (blog 0194), taste as art (blog 0195), generation trap (ACI-026)

## External References

- Blog 0194: https://matthewsinclair.com/blog/0194-the-time-cost-of-how-is-zero
- Blog 0195: https://matthewsinclair.com/blog/0195-taste-as-art
- Blog 0196: https://matthewsinclair.com/blog/0196-storytelling-in-a-post-truth-world (WP-07 facilitation)
- Blog 0197: https://matthewsinclair.com/blog/0197-the-expanding-pie-and-the-cleanup-bill (enterprise framing)
- "10 Rules for Building Stuff People Give a Shit About" -- unpublished, in interview notes
- Innovation Antibodies: https://matthewsinclair.medium.com/0057-anti-antibodies-874bf8b52b5d

## Conventions

- ALWAYS use `intent` CLI for ST/WP operations
- NEVER manually wrap lines in markdown
- Detrope every content file (mechanical + full LLM)
- NO Claude attribution in commits
