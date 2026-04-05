# Claude Code Session Restart

## WIP

ST0031: Agentic Coding Course -- WP-02 + WP-03 substantially complete. 22 ACIs total. Detrope remediation in progress.

## TODO

1. Complete detrope remediation on ACIs 006-022 (batch 2 of 4 done, fixes applied to ACI-012/013/014)
2. Update `docs/course/content/how-this-course-was-built.md` with WP-02/03 process notes
3. Re-run WP-03 landscape research with web search enabled (training-data-only in prior session)
4. User interview: MeetZaya non-coding failure reasons (Lens 6 blocker)
5. WP-04: Taxonomy + Course Structure (after WP-02+03 sign-off)

## Key Files

| File                                               | Purpose                                      |
| -------------------------------------------------- | -------------------------------------------- |
| `intent/st/ST0031/`                                | Steel thread (info, design, tasks, 7 WPs)    |
| `docs/course/process/extraction-protocol.md`       | 6-lens protocol (calibration notes updated)  |
| `docs/course/process/detrope-checklist.md`         | Quality gate for every content step          |
| `docs/course/content/insights/`                    | 22 ACI files (ACI-001 through ACI-022)       |
| `docs/course/landscape.md`                         | External landscape survey (7 dimensions)     |
| `docs/course/evaluation-framework.md`              | Tool/method evaluation rubric (6 dimensions) |
| `docs/course/content/how-this-course-was-built.md` | Meta-chapter (needs WP-02/03 update)         |
| `intent/wip.md`                                    | Work in progress tracker                     |

## ACI Inventory (22 total)

| ID      | Name                                      | Source          |
| ------- | ----------------------------------------- | --------------- |
| ACI-001 | Highlander Rule                           | WP-01 sample    |
| ACI-002 | CLAUDE.md as Living Architecture          | WP-01 sample    |
| ACI-003 | Correction Erosion After Compaction       | WP-01 sample    |
| ACI-004 | When Coding Succeeds But Project Fails    | WP-01 sample    |
| ACI-005 | Steel Threads Beat Feature Branches       | WP-01 sample    |
| ACI-006 | The Great Simplification                  | WP-02 Lens 1    |
| ACI-007 | Cross-Project Contamination               | WP-02 Lens 1    |
| ACI-008 | Style Rules vs Architecture Rules         | WP-02 Lens 1    |
| ACI-009 | CLAUDE.md Stagnation as Health Signal     | WP-02 Lens 1    |
| ACI-010 | The Regression Cascade                    | WP-02 Lens 4    |
| ACI-011 | Namespace Archaeology                     | WP-02 Lens 4    |
| ACI-012 | Methodology Transfer Velocity             | WP-02 Lens 5    |
| ACI-013 | The Methodology Desert                    | WP-03 Landscape |
| ACI-014 | Context Engineering vs Prompt Engineering | WP-03 Landscape |
| ACI-015 | The Agentic-Assistive Spectrum            | WP-03 Landscape |
| ACI-016 | Skills as Procedural Memory               | WP-03 Landscape |
| ACI-017 | The Delegation Model                      | WP-03 Landscape |
| ACI-018 | The Ten Percent Problem                   | WP-03 Landscape |
| ACI-019 | Session Lifecycle as Missing Primitive    | WP-03 Landscape |
| ACI-020 | The Tool-Practice Gap                     | WP-03 Landscape |
| ACI-021 | The Batch-Creation Antipattern            | WP-02 Lens 6    |
| ACI-022 | Done Means Enough, Not All                | WP-02 Lens 2    |

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
