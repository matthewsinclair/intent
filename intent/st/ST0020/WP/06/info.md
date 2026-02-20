---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-06
title: Skill Installation Infrastructure
scope: Large
status: Done
---

# WP-06: Skill Installation Infrastructure

## Objective

Create `intent claude skills` CLI command for managing skill lifecycle, mirroring the existing subagent architecture.

## Deliverables

- `intent/plugins/claude/bin/intent_claude_skills` — new CLI command
- BATS tests for all subcommands

## Commands

- `intent claude skills list` — show available and installed skills
- `intent claude skills install <name>` — copy SKILL.md to `.claude/skills/<name>/`
- `intent claude skills sync` — update installed skills (SHA256 checksums)
- `intent claude skills uninstall <name>` — remove skill from project
- `intent claude skills show <name>` — display skill content and status

## Architecture

- Source directory: `intent/plugins/claude/skills/<name>/SKILL.md`
- Target directory: `.claude/skills/<name>/SKILL.md` in target projects
- Manifest: `~/.intent/skills/installed-skills.json` (SHA256 pattern from subagents)

## Implementation Approach

Factor out common install/sync/uninstall logic from `intent_claude_subagents` into shared functions. Build `intent_claude_skills` using the same patterns. Register in Intent's CLI command routing.

## Acceptance Criteria

- [ ] All 5 subcommands working
- [ ] SHA256 manifest tracking matches subagent pattern
- [ ] BATS tests for install, sync, uninstall, list, show
- [ ] Command registered in Intent CLI routing
- [ ] Installs SKILL.md to correct target path
- [ ] Sync detects modified/stale skills correctly

## Dependencies

- WP-02 (needs at least one skill to install/test with)
