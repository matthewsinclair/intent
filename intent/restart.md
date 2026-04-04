# Session Restart Context

## Project

Intent v2.8.0 -- CLI tool for managing steel threads, project docs, and LLM guidance. Bash + BATS tests. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

22 skills, 5 subagents, 462 tests. ST0030 complete. ST0031 (Agentic Coding Course) fully elaborated with 7 WPs, ready for WP-01.

## Recent Work (2026-04-04)

- ST0030 completed: 3 new skills (in-verify, in-debug, in-review), Red Flags tables, chains_to, plan quality standards
- Autopsy 20260404: clean results, "overall" removed from banned words, MEMORY.md consolidated
- ST0031 fully elaborated: ACI model, 6 extraction lenses, 5-day course arc, dual audience pathways, MeetZaya failure case study, landscape research dimension

## Next Session: ST0031/WP-01

Define the ACI format and extraction protocol:

1. Create `docs/course/templates/aci-template.md`
2. Hand-craft 3-5 ACIs from known lessons to validate format
3. Document extraction protocol (6 lenses, specific commands per lens)
4. Decide on autopsy Elixir script reuse for Lens 3 (correction mining)

After WP-01: WP-02 (pilot extraction) and WP-03 (landscape research) can run in parallel.

## Key Decisions Made

- Content atom = Agentic Coding Insight (ACI) with structured frontmatter
- Two audiences: individual/advanced + enterprise, shared atoms with different pathways
- Pilot repos: Intent (process), Lamplight (code), MeetZaya (failure case study)
- MeetZaya is a Day 5 centerpiece ("The Reckoning"), not an appendix
- Day 1 = Landscape (state of the art), Day 5 = Reckoning + evaluation framework
- Course format: 1-1.5hr theory + 1-1.5hr practical mornings, self-directed afternoons

## Key Files

| File                                        | Purpose                                   |
| ------------------------------------------- | ----------------------------------------- |
| `intent/st/ST0031/`                         | Steel thread (info, design, tasks, 7 WPs) |
| `docs/course/`                              | Course content (empty, to be populated)   |
| `~/.claude/plans/temporal-wishing-panda.md` | Full plan with all details                |
| `intent/wip.md`                             | Work in progress tracker                  |
| `intent/done.md`                            | Completed work log                        |

## Conventions

- ALWAYS use `intent` CLI for ST/WP operations (never manual file moves)
- NO Claude attribution in commit messages
- Tag workflow: `git tag -f vX.Y.Z HEAD` then force-push to both remotes
- Never use em dashes in skill files (multi-byte truncation bugs)
- Run `tests/run_tests.sh` before committing (462 tests across 22 files)
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
