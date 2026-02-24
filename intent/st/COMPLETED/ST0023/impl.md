# Implementation - ST0023: Remove Backlog from Intent

## Implementation Log

### WP01 - Documentation

- Removed backlog from README.md, CLAUDE.md, user_guide.md, reference_guide.md, deployment_guide.md
- Removed backlog from intent/llm/AGENTS.md and .github/workflows/README.md
- Added DEPRECATIONS.md v2.5.0 entry
- Annotated 7 TPD files with "[Removed in v2.5.0]"
- Added editor's notes to 7 blog posts (0001-0007)
- Removed backlog from tests/README.md
- Cleaned intent/docs/creating-custom-agents.md

### WP02 - LLM Templates

- Cleaned lib/templates/llm/\_CLAUDE.md, \_llm_preamble.md
- Cleaned lib/help/agents.help.md
- Cleaned lib/templates/usr/\_reference_guide.md

### WP03 - Subagents

- Cleaned intent subagent (agent.md, metadata.json, global-agents.json manifest)
- Cleaned installed copy (.claude/agents/intent.md)
- Cleaned socrates (README.md and agent.md)
- Cleaned worker-bee (agent.md)

### WP04 - CLI Core

- Deleted: intent_backlog, intent_bl, intent_migrate, intent_status, intent_task
- Cleaned: intent, intent_help, intent_config, intent_helpers, intent_init, intent_bootstrap, intent_info, intent_doctor, intent_upgrade

### WP05 - Configuration

- Removed backlog_dir and backlog_list_status from .intent/config.json
- Removed backlog_dir from examples/hello-world/.intent/config.json
- Consolidated duplicate version/intent_version fields to just intent_version

### WP06 - Tests

- Deleted: bl_commands.bats, task_commands.bats, migration.bats
- Updated: core_functionality.bats, project_commands.bats, bootstrap.bats, agent_commands.bats, help_commands.bats, end_to_end.bats, init_commands.bats
- Updated: test_helper.bash

### WP07 - CI/CD, Examples, Cleanup

- Removed Node.js setup from both Linux and macOS CI jobs
- Deleted backlog/ directories: project root, 4 example projects
- Cleaned example files: README.md, CLAUDE.md, .gitignore, ST0001 docs
- Cleaned .claude/settings.local.json of stale STP/backlog permissions

### WP08 - Version Bump, Release

- VERSION: 2.4.0 -> 2.5.0
- .intent/config.json: updated intent_version
- CHANGELOG.md: added v2.5.0 entry
- Updated wip.md and restart.md
- 318 tests passing across 14 test files

### Post-WP: Test Side-Effect Fix

- Discovered tests in `agent_commands.bats` directly modify real source `agent.md` file
- Tests used `git checkout` in teardown to "undo" changes, which reverted ALL uncommitted edits
- Fixed by adding `create_source_sandbox()` helper that creates a temp INTENT_HOME with:
  - Copied subagent source files (safe to modify)
  - Symlinked bin/ scripts and other resources
- Three tests (sync source changes, status updates, outdated manifest) now use sandbox
- Removed `git checkout` from teardown; added INTENT_HOME save/restore
- All 318 tests pass; real source files are never modified by tests

### Post-WP: Highlander Rule Audit

- Performed comprehensive audit identifying 25 Highlander Rule violations
- Findings documented in ST0025 for future resolution
- Categories: shared helpers (8), template consolidation (7), plugin dedup (4), correctness (2), legacy (4)

## Challenges & Solutions

1. **Test side-effect destroying uncommitted edits** - Tests modified real `$INTENT_HOME/intent/plugins/claude/subagents/intent/agent.md` and used `git checkout` in teardown. This silently reverted our backlog-removal edits after each test run. Fixed with sandbox approach (see above).

2. **Manifest stores absolute paths** - The sandbox approach initially failed because the install manifest records the absolute `source_path` from install time. Tests must use the sandbox from the start (including install) so the manifest records sandbox paths.

3. **Duplicate version fields** - Found both `version` and `intent_version` in config.json across the codebase. Consolidated to just `intent_version` as the canonical field, with read-time fallback (`jq '.intent_version // .version'`) for legacy configs.

4. **Global manifest missed** - The `show` command reads description from `.manifest/global-agents.json`, not from `metadata.json`. Had to update both.
