# Claude Code Session Restart

## WIP

ST0020 complete. Intent v2.4.0 is on main but not yet tagged/released.

## TODO

Remaining v2.4.0 release tasks:

1. **Tag and release** -- `git tag -f v2.4.0 HEAD`, push to both remotes, create GitHub release
   - Demote old release first: `gh release edit v2.3.4 --latest=false`
2. **Update docs** -- user_guide.md and reference_guide.md need v2.4.0 coverage (skills, upgrade commands)
3. **Regenerate .treeindex** -- new directories added (skills, templates/elixir, upgrade command)

Longer-term (check with user):

4. **Parked steel threads** -- review ST0010 and ST0015 to decide if still relevant
5. **Fix subagent sync false-positive** -- `intent claude subagents sync` reports "modified locally" when the source has changed but the installed copy is just stale. Logic in `intent/plugins/claude/bin/intent_claude_subagents` around lines 485-507

## Key Files

| File                                               | Purpose                                |
|----------------------------------------------------|----------------------------------------|
| `CHANGELOG.md`                                     | Feature history (v1.0.0 through v2.4.0)|
| `VERSION`                                          | Current version (2.4.0)                |
| `intent/wip.md`                                    | Work in progress tracker               |
| `intent/restart.md`                                | Detailed session restart context       |
| `intent/usr/user_guide.md`                         | User-facing guide (needs v2.4.0 update)|
| `intent/usr/reference_guide.md`                    | Command reference (needs v2.4.0 update)|
| `bin/intent`                                       | Main CLI (GLOBAL_COMMANDS on line 41)  |
| `intent/plugins/claude/bin/intent_claude_skills`   | Skills lifecycle management            |
| `intent/plugins/claude/bin/intent_claude_upgrade`  | Project upgrade command                |
| `intent/plugins/claude/bin/intent_claude_subagents`| Claude subagent management             |
| `intent/plugins/agents/bin/intent_agents`          | AGENTS.md management                   |
| `usage-rules.md`                                   | Intent's own LLM usage guide           |
| `tests/run_tests.sh`                               | Test runner (292 tests, 15 files)      |

## Project Conventions

- Bash CLI tool, tests use BATS framework
- Two git remotes: `local` (Dropbox), `upstream` (GitHub)
- Tag workflow: `git tag -f v2.4.0 HEAD` then force-push to both remotes
- NO Claude attribution in commit messages -- ever
- Run `tests/run_tests.sh` before committing
- Check `intent/.treeindex/<dir>/.treeindex` before exploring unfamiliar directories
