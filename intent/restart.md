# Session Restart Context

## Project

Intent v2.8.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

22 skills, 5 subagents, 462 tests. ST0031/WP-01 (ACI Format & Extraction Protocol) complete. WP-02 and WP-03 ready to start in parallel.

## Recent Work (2026-04-04)

- ST0031/WP-01 completed: ACI template, 5 sample ACIs, 6-lens extraction protocol, autopsy reuse assessment, detrope quality gate, meta-chapter started
- 10 files created under `docs/course/` with process/content directory split
- Full LLM-based detrope analysis: 0 flags, AI signal "low" across all content
- 3 blog post references collected from user (Highlander Rule, Throwing Away Code, Socratic Dialog)

## Next Session: ST0031/WP-02 + WP-03 (parallel)

**WP-02: Pilot Extraction** (3-5 sessions, heaviest WP)

Apply 6 extraction lenses to Intent, Lamplight, MeetZaya:

- Lens 1: Rule Archaeology on CLAUDE.md (28/10/17 edits across 3 repos)
- Lens 2: Plan-Outcome Delta on completed STs
- Lens 3: Correction Mining via autopsy script (use as-is)
- Lens 4: Architecture Forensics on Lamplight + MeetZaya
- Lens 5: Methodology Evolution cross-repo timeline
- Lens 6: Failure Archaeology on MeetZaya + Lamplight cancelled STs
- User interview needed: MeetZaya non-coding failure reasons
- Target: 20-25 raw ACI candidates
- Protocol calibration notes

**WP-03: Landscape Research** (2-3 sessions, parallel with WP-02)

- 7 dimensions: Anthropic guidance, tool landscape, skills ecosystem, open source methodology, enterprise adoption, mental models, notable practitioners
- Create `docs/course/content/landscape.md` and `docs/course/content/evaluation-framework.md`
- Target: 8-12 landscape ACIs

## Key Decisions Made

- ACI format: 5 sections (Thesis, Story, Evidence, Application, Anti-Pattern) + structured frontmatter
- Directory split: `docs/course/process/` (methodology) vs `docs/course/content/` (deliverables)
- Detrope at every step: mechanical pre-scan (`cleanz --detrope`) + full in-detrope skill analysis
- Autopsy script: use as-is for Lens 3, no extension needed
- 5 placeholder categories: communication, architecture, process, failure, methodology
- ACI IDs: sequential ACI-NNN (no category prefix, stable across taxonomy changes)
- Socratic Dialog blog post -> dedicated course chapter (not just an ACI), likely Day 2 or Day 4
- "How This Course Was Built" meta-chapter grows with each WP
- NEVER manually wrap lines in markdown files

## External References Collected

| Reference                                                                                                      | Course Use                      |
| -------------------------------------------------------------------------------------------------------------- | ------------------------------- |
| [Highlander Rule](https://matthewsinclair.com/blog/0189-the-unreasonable-effectiveness-of-the-highlander-rule) | In ACI-001 + ACI-002 evidence   |
| [Knowing What Code to Throw Away](https://matthewsinclair.com/blog/0188-on-knowing-what-code-to-throw-away)    | In ACI-004 evidence             |
| [CTO Review Socratic Dialog](https://matthewsinclair.com/blog/0182-cto-review-socratic-dialog-ai)              | Dedicated chapter (WP-04/WP-06) |

## Key Files

| File                                          | Purpose                                             |
| --------------------------------------------- | --------------------------------------------------- |
| `intent/st/ST0031/`                           | Steel thread (info, design, tasks, 7 WPs)           |
| `docs/course/process/`                        | Extraction protocol, detrope checklist, build notes |
| `docs/course/content/`                        | ACI template, 5 sample ACIs, meta-chapter           |
| `docs/course/process/extraction-protocol.md`  | 6-lens protocol (use this for WP-02)                |
| `docs/course/process/detrope-checklist.md`    | Quality gate for every content step                 |
| `~/.claude/plans/dynamic-hatching-shannon.md` | WP-01 implementation plan                           |
| `intent/wip.md`                               | Work in progress tracker                            |
| `intent/done.md`                              | Completed work log                                  |

## Conventions

- ALWAYS use `intent` CLI for ST/WP operations (never manual file moves)
- NO Claude attribution in commit messages
- NEVER manually wrap lines in markdown files
- Detrope every content file (mechanical + full LLM-based)
- Separate process/ from content/ in docs/course/
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (462 tests across 22 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
