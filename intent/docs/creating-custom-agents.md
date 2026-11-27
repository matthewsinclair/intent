---
verblock: "27 Nov 2026:v1.5: ic - leaned to what Intent reads"
intent_version: 3.5.6
---

# Creating Custom Intent Agents

This guide says how to add a subagent to Intent's canon, so that `intent claude subagents install` puts it where Claude Code's sub-agent system finds it.

## Canon vs extension subagents

Subagents ship in canon, in `intent/plugins/claude/subagents/<name>/` of the Intent install. Examples: `socrates`, `diogenes`, the `critic-<lang>` family. The extension layout `~/.local/share/intent/ext/<name>/subagents/` is declared and not built: nothing in this build reads it (see `intent/docs/writing-extensions.md`).

The choice between them is one question: **is this useful to every Intent user, or only to some?**

- Useful to every Intent user → canon. The remainder of this guide applies.
- Useful only to some users → keep it outside canon in your own `~/.claude/agents/`, where `intent claude subagents list` reports it as `unlisted`.

## Agent Structure

Each Intent canon subagent consists of:

- **Directory**: `intent/plugins/claude/subagents/<name>/` in the Intent install. `intent claude subagents install` reads canon from the INSTALL, the ancestor of the running `intent` binary that holds `lib/templates/`, not from the current project. For a source checkout, that is the checkout root.
- **Agent Definition**: `agent.md`, with YAML frontmatter and a system prompt. It is the one file that makes the directory a subagent, and install copies it alone to `~/.claude/agents/<name>.md`.
- **Name**: letters, digits, `-` and `_` only, eg `security-reviewer`. Install refuses any other name.

Intent does not read a `metadata.json` beside `agent.md`, and no canon subagent carries one.

## Write `agent.md`

```markdown
---
name: your-agent-name
description: Brief one-line description of your agent's purpose and expertise
tools: Bash, Read, Write, Edit, Grep
---

You are a specialized [DOMAIN] expert assistant with deep knowledge in [SPECIFIC AREAS].

## Your Expertise

- [Primary capability 1]
- [Primary capability 2]

## Your Role

When working with users, you should:

1. [Specific behavior 1]
2. [Specific behavior 2]

## When to Use This Agent

- [Specific use case 1]
- [Specific use case 2]
```

**Frontmatter** (Claude Code reads it; Intent does not: `intent claude subagents install` copies `agent.md` as it is):

- `name` (required): the id `Task(subagent_type=...)` uses. Keep it equal to the directory name, which is the name `intent claude subagents` knows the agent by; nothing checks that the two agree.
- `description` (required): when Claude should delegate to this agent.
- `tools` (optional): a comma-separated list of Claude Code tools, eg `Bash, Read, Grep`. Omitted, the agent inherits every tool available to subagents. Claude Code's full tool list: https://code.claude.com/docs/en/sub-agents.

## Install and verify

```bash
intent claude subagents install your-agent-name
```

This copies `agent.md` to `~/.claude/agents/your-agent-name.md` and records the install in `~/.local/share/intent/subagents/installed-subagents.v3.json`.

- `intent claude subagents install --all` installs every subagent this install carries.
- `-f, --force` overwrites an installed subagent that install holds (changed here, conflicted, or with no install record), and reports the checksum of what was discarded. Installing again over an unchanged copy needs no flag: it is `up to date`, or updated when the shipped one is newer.

```bash
intent claude subagents list -v                # name, installed state, provenance and source path
intent claude subagents show your-agent-name   # prints its agent.md
```

Then call it through Claude Code's Task tool:

```
Task(
  description="Test custom agent",
  prompt="Perform a simple task to verify the agent is working correctly",
  subagent_type="your-agent-name"
)
```

## Updating Agents

Edit `agent.md` in canon, then run `intent claude subagents sync`. It brings every installed subagent up to its canon copy and holds one you edited locally: `--dry-run` previews, and `--force` takes the canon copy and reports the checksum of what it discarded. `intent claude subagents install your-agent-name --force` overwrites a single agent outright.

## Troubleshooting

**Agent Not Listed**: check that `agent.md` is at `intent/plugins/claude/subagents/<agent-name>/` in the Intent install.

**Installation Fails**

- `no source for this name in this install`: `agent.md` is not at `<install>/intent/plugins/claude/subagents/<name>/`, where `<install>` is the tree the running `intent` binary belongs to (a source checkout's root).
- `modified here since it was installed -- HELD`, or another HELD line: install decides an installed subagent as `sync` does, and holds one it cannot replace without losing something. `--force` replaces it and reports the checksum of what it discarded.

Frontmatter mistakes do not fail the install: Intent copies `agent.md` without reading it, and Claude Code reports them when it loads the agent.

## Slash Commands

A subagent cannot define a slash command: Claude Code reads only its frontmatter and system prompt. A workflow you want to invoke as `/name` is a skill, `intent/plugins/claude/skills/<slug>/SKILL.md`, installed with `intent claude skills install <slug>`.

## References

- [Claude Code Sub-Agents](https://code.claude.com/docs/en/sub-agents)
- [Agent Examples](../plugins/claude/subagents/)
- `intent claude subagents --help` for the command reference
