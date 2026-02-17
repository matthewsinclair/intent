# Session Restart Context

## Project

Intent v2.4.0 -- a CLI tool for managing steel threads, project documentation, and LLM guidance (skills, subagents, templates). Written in bash, tests use BATS framework. Located at `/Users/matts/Devel/prj/Intent/`.

## Current State

ST0020 (Modernizing Elixir Support) is complete. v2.4.0 is on main but not yet tagged/released.

### What v2.4.0 Added

- **Skills system**: `intent claude skills list/install/sync/uninstall/show` — installs SKILL.md files into `.claude/skills/` in target projects
- **Three Elixir skills**: elixir-essentials (8 rules), ash-ecto-essentials (7 rules), phoenix-liveview (7 rules)
- **Upgrade command**: `intent claude upgrade [--apply] [--project-dir DIR]` — diagnoses and upgrades project LLM guidance files
- **Elixir templates**: `intent agents init --template elixir` creates RULES.md + ARCHITECTURE.md
- **Reference docs**: ash-ecto.md, liveview.md, testing.md, project-structure.md added to elixir subagent
- **usage-rules.md**: Intent's own LLM usage guide (root directory)
- **agent.md refactor**: 23 overlapping rules distilled to 12 non-overlapping

### Remaining v2.4.0 Tasks

1. Tag v2.4.0 and push to both remotes
2. Create GitHub release (demote v2.3.4 first: `gh release edit v2.3.4 --latest=false`)
3. Update user_guide.md and reference_guide.md for new commands
4. Regenerate .treeindex files

## Parked Steel Threads

- **ST0010**: `intent/st/NOT-STARTED/ST0010/info.md`
- **ST0015**: `intent/st/NOT-STARTED/ST0015/info.md`

## Key Patterns

- Commands live in `bin/intent_<name>` and are auto-routed by `bin/intent`
- Global commands are listed in `GLOBAL_COMMANDS` on line 41 of `bin/intent`
- Plugin commands: `intent agents` -> `intent/plugins/agents/bin/intent_agents`
- Plugin commands: `intent claude subagents` -> `intent/plugins/claude/bin/intent_claude_subagents`
- Plugin commands: `intent claude skills` -> `intent/plugins/claude/bin/intent_claude_skills`
- Plugin commands: `intent claude upgrade` -> `intent/plugins/claude/bin/intent_claude_upgrade`
- Subagent definitions: `intent/plugins/claude/subagents/<name>/`
- Skill definitions: `intent/plugins/claude/skills/<name>/SKILL.md`
- Templates: `intent/plugins/agents/templates/<name>/` (currently: default, elixir)
- Tests are in `tests/unit/` (15 .bats files, 292 tests) and run with `tests/run_tests.sh`
- Two git remotes: `local` (Dropbox) and `upstream` (GitHub)
- Always run tests before committing
- NO Claude attribution in commit messages
