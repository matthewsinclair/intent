# Claude Code Session Restart

## WIP

ST0031: Agentic Coding Course -- WP-06 complete. Full course content produced: 26 polished ACIs, 5 theory guides with dual pathway sections, 5 exercise documents, 3-part MeetZaya case study with full "10 Rules" framework, course syllabus. WP-07 (packaging) is next.

## TODO

1. WP-07: Packaging & delivery assets
   - Course README with setup instructions
   - Per-day handout materials (extract student-facing content from exercise docs)
   - Sample/exercise repo for practical sessions
   - Enterprise-specific framing materials (ROI, governance)
   - Standalone tool comparison matrix (data exists in landscape.md)
   - Reading list / resource guide (sources exist in landscape.md)
   - Facilitation guide (Blog 0196: storytelling as truth vehicle)
2. Note: Blog 0197 (expanding pie) for enterprise framing materials

## Key Files

| File                                               | Purpose                                                                    |
| -------------------------------------------------- | -------------------------------------------------------------------------- |
| `intent/st/ST0031/`                                | Steel thread with 7 WPs                                                    |
| `intent/st/ST0031/tasks.md`                        | Task tracking (WP-01-06 done, WP-07 next)                                  |
| `intent/st/ST0031/meetzaya-interview-notes.md`     | MeetZaya failure interview (8 reasons, full 10 Rules + 2 bonus, blog refs) |
| `docs/course/content/syllabus.md`                  | Course overview and materials list                                         |
| `docs/course/outline.md`                           | 5-day arc, ACI sequencing, dual pathways                                   |
| `docs/course/taxonomy.md`                          | 5 categories, 26 ACIs, cross-refs                                          |
| `docs/course/content/days/`                        | 5 theory guides (facilitator session plans)                                |
| `docs/course/content/exercises/`                   | 5 exercise documents (student + facilitator)                               |
| `docs/course/content/insights/`                    | 26 ACI files (ACI-001 through ACI-026)                                     |
| `docs/course/content/meetzaya-case-study.md`       | 3-part Day 5 case study (2,984 words)                                      |
| `docs/course/evaluation-framework.md`              | 6-dimension evaluation rubric                                              |
| `docs/course/landscape.md`                         | Landscape survey (verified 06 Apr 2026)                                    |
| `docs/course/content/how-this-course-was-built.md` | Meta-chapter (updated through WP-06)                                       |
| `intent/wip.md`                                    | Session tracker                                                            |

## Course Structure

| Day | Theme            | ACIs                                             | Status   |
| --- | ---------------- | ------------------------------------------------ | -------- |
| 1   | The Landscape    | 015, 013, 018, 020                               | Complete |
| 2   | The Conversation | 017, 014, 024, 003, 007                          | Complete |
| 3   | The Architecture | 002, 008, 006, 001, 011, 016                     | Complete |
| 4   | The Process      | 005, 019, 022, 021, 009, 012                     | Complete |
| 5   | The Reckoning    | 010, 023, 004, 026, 025 + case study + framework | Complete |

## External References

- Blog 0194: https://matthewsinclair.com/blog/0194-the-time-cost-of-how-is-zero
- Blog 0195: https://matthewsinclair.com/blog/0195-taste-as-art
- Blog 0196: https://matthewsinclair.com/blog/0196-storytelling-in-a-post-truth-world (WP-07)
- Blog 0197: https://matthewsinclair.com/blog/0197-the-expanding-pie-and-the-cleanup-bill (enterprise)
- "10 Rules": in meetzaya-interview-notes.md and meetzaya-case-study.md (unpublished)

## Conventions

- ALWAYS use `intent` CLI for ST/WP operations
- NEVER manually wrap lines in markdown
- Detrope every content file (mechanical + full LLM)
- NO Claude attribution in commits
- Separate process/ from content/ in docs/course/
