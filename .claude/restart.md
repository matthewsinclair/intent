# Claude Code Session Restart

## WIP

ST0031: Agentic Coding Course -- WP-01 done, WP-02 + WP-03 next (parallel). 22 skills, 5 subagents, 462 tests.

## TODO

ST0031/WP-02: Pilot extraction on Intent + Lamplight + MeetZaya.

- Apply 6 lenses from `docs/course/process/extraction-protocol.md`
- Use autopsy script as-is for Lens 3 (correction mining)
- User interview needed for MeetZaya non-coding failure reasons
- Target: 20-25 raw ACI candidates
- Detrope every ACI (see `docs/course/process/detrope-checklist.md`)

ST0031/WP-03: Landscape research (parallel with WP-02).

- 7 dimensions (Anthropic, tools, skills, open source, enterprise, mental models, practitioners)
- Create landscape.md + evaluation-framework.md
- Target: 8-12 landscape ACIs

Then WP-04 (taxonomy + course structure) after both converge.

## Key Files

| File                                               | Purpose                                   |
| -------------------------------------------------- | ----------------------------------------- |
| `intent/st/ST0031/`                                | Steel thread (info, design, tasks, 7 WPs) |
| `docs/course/process/extraction-protocol.md`       | 6-lens protocol (follow this for WP-02)   |
| `docs/course/process/detrope-checklist.md`         | Quality gate for every content step       |
| `docs/course/content/templates/aci-template.md`    | ACI format definition                     |
| `docs/course/content/insights/`                    | 5 sample ACIs from WP-01                  |
| `docs/course/content/how-this-course-was-built.md` | Meta-chapter (grows each WP)              |
| `intent/wip.md`                                    | Work in progress tracker                  |
| `intent/restart.md`                                | Full session restart context              |

## External References (collected during WP-01)

- [Highlander Rule blog](https://matthewsinclair.com/blog/0189-the-unreasonable-effectiveness-of-the-highlander-rule) -- in ACI-001/002
- [Throwing Away Code blog](https://matthewsinclair.com/blog/0188-on-knowing-what-code-to-throw-away) -- in ACI-004
- [CTO Review Socratic Dialog blog](https://matthewsinclair.com/blog/0182-cto-review-socratic-dialog-ai) -- dedicated chapter (WP-04/06)

## Project Conventions

- ALWAYS use `intent` CLI for ST/WP operations (never manual file moves)
- NEVER manually wrap lines in markdown files
- Detrope every content file (mechanical + full LLM-based)
- Separate process/ from content/ in docs/course/
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- Never use em dashes in skill files -- multi-byte truncation in list display
- macOS bash 3.x: no `declare -A`, no `${VAR^}` -- use explicit alternatives
