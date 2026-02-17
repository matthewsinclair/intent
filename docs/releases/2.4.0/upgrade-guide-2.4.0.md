---
verblock: "17 Feb 2026:v0.1: matts - Initial version"
---

# Intent Upgrade Guide: 2.4.0

This guide covers upgrading Intent-managed Elixir projects from v2.3.x to v2.4.0. Version 2.4.0 introduces:

- **Skills** — always-on coding rules installed to `.claude/skills/`
- **Three-file LLM guidance** — AGENTS.md + RULES.md + ARCHITECTURE.md
- **Expanded Elixir subagent** — 12 distilled rules, new reference docs (Ash/Ecto, LiveView, testing, project structure)
- **`intent claude skills` command** — lifecycle management for skills (list, install, sync, uninstall, show)
- **`intent agents init --template elixir`** — pre-populated Elixir templates for LLM guidance files

## Prerequisites

- Intent v2.4.0 installed (`intent version` to check)
- `jq` installed (`brew install jq` on macOS)
- Project using Intent for steel thread management

## Step 1: Diagnose Current State

Check what LLM guidance files exist in your project:

```bash
# Check for existing files
ls -la intent/llm/
ls -la .claude/agents/
ls -la .claude/skills/ 2>/dev/null || echo "No skills directory"
```

### Expected findings by project type

| File                         | If Present                    | Action                                     |
| ---------------------------- | ----------------------------- | ------------------------------------------ |
| `intent/llm/AGENTS.md`       | Likely stale (pre-2.4.0)      | Regenerate with `intent agents sync`       |
| `intent/llm/AGENTS-phx.md`   | Deprecated Phoenix rules file | Merge into RULES.md, then delete           |
| `intent/llm/llm_preamble.md` | Deprecated                    | Delete                                     |
| `intent/llm/usage-rules.md`  | Old format (pre-2.3.0)        | Delete (replaced by skills)                |
| `intent/llm/RULES.md`        | Keep -- human-curated         | Review, update if needed                   |
| `intent/llm/ARCHITECTURE.md` | Keep -- human-curated         | Review, update if needed                   |
| `.claude/agents/elixir.md`   | Old subagent version          | Update with `intent claude subagents sync` |
| `.claude/skills/`            | Should not exist pre-2.4.0    | Will be created by skill install           |

## Step 2: Update Subagents

The Elixir subagent has been significantly refactored (23 rules distilled to 12, new reference docs added):

```bash
# Update the Elixir subagent
intent claude subagents sync --force

# Verify
intent claude subagents status
```

## Step 3: Install Skills

Skills are new in 2.4.0. Four skills are available (one universal + three Elixir):

```bash
# Install all skills
intent claude skills install --all

# Or install individually
intent claude skills install intent-essentials
intent claude skills install elixir-essentials
intent claude skills install ash-ecto-essentials
intent claude skills install phoenix-liveview

# Verify
intent claude skills list
```

### What each skill does

- **intent-essentials** — Intent workflow enforcement (CLI usage, treeindex, steel thread conventions, session wrap-up)
- **elixir-essentials** — core Elixir patterns (pattern matching, tagged tuples, pipes, naming, assertive access)
- **ash-ecto-essentials** — Ash-first database access (code interfaces, migrations, actor placement, atomic changes)
- **phoenix-liveview** — LiveView lifecycle (two-phase mount, streams, navigation, assign_async, components)

## Step 4: Set Up Three-File Guidance

### If you have NO RULES.md or ARCHITECTURE.md

Use the Elixir template to create all three files:

```bash
intent agents init --template elixir --force
```

This creates:

- `intent/llm/AGENTS.md` — project overview with Elixir defaults
- `intent/llm/RULES.md` — pre-populated with 9 core Elixir rules + framework rules
- `intent/llm/ARCHITECTURE.md` — skeleton for your system architecture

Then customize each file for your project.

### If you HAVE an existing RULES.md

Keep your existing RULES.md. Just regenerate AGENTS.md:

```bash
intent agents sync
```

Review your RULES.md against the template at `intent/plugins/agents/templates/elixir/RULES.md` to see if you're missing any rules.

### If you have AGENTS-phx.md

The `AGENTS-phx.md` file is deprecated. Its Phoenix-specific rules should be merged into RULES.md:

1. Open `intent/llm/AGENTS-phx.md` and review the rules
2. Add any missing rules to the "Framework Rules > Phoenix" section of your RULES.md
3. Delete `intent/llm/AGENTS-phx.md`

## Step 5: Clean Up Deprecated Files

```bash
# Remove deprecated files (verify they exist first)
ls intent/llm/llm_preamble.md 2>/dev/null && rm intent/llm/llm_preamble.md
ls intent/llm/usage-rules.md 2>/dev/null && rm intent/llm/usage-rules.md

# Remove AGENTS-phx.md ONLY after merging its content into RULES.md
ls intent/llm/AGENTS-phx.md 2>/dev/null && echo "Merge into RULES.md first, then delete"
```

## Step 6: Create ARCHITECTURE.md (if missing)

If you don't have an `intent/llm/ARCHITECTURE.md`:

```bash
# Copy the template
cp "$INTENT_HOME/intent/plugins/agents/templates/elixir/ARCHITECTURE.md" intent/llm/ARCHITECTURE.md
```

Then fill in:

- System overview (2-3 sentences)
- Domain map (your Ash domains and their responsibilities)
- Data flow diagram
- Key patterns (auth, background jobs, integrations)
- Decision log

## Step 7: Verify

```bash
# Check subagent status
intent claude subagents status

# Check skills are installed
intent claude skills list

# Check LLM guidance files exist
ls -la intent/llm/AGENTS.md intent/llm/RULES.md intent/llm/ARCHITECTURE.md

# Run project doctor
intent doctor
```

## Project-Specific Notes

### Intent (this project)

- Delete `intent/llm/llm_preamble.md` (deprecated)
- Regenerate `intent/llm/AGENTS.md` (stale v2.2.1)
- No RULES.md or ARCHITECTURE.md needed (Intent is a Bash project, not Elixir)

### Prolix

- Create RULES.md from template + merge AGENTS-phx.md Phoenix rules
- Create ARCHITECTURE.md from template
- Install all skills
- Delete `intent/llm/llm_preamble.md`

### Laksa

- Merge AGENTS-phx.md into existing RULES.md
- Create ARCHITECTURE.md from template
- Install all skills
- Delete deprecated files

### Lamplight

- Install skills (most complete project, minimal changes)
- Verify RULES.md and ARCHITECTURE.md formats
- Remove any deprecated files

## Troubleshooting

### `intent claude skills install` fails with "jq not found"

Install jq: `brew install jq` (macOS) or `sudo apt-get install jq` (Linux).

### Skills not loading in Claude Code sessions

Verify skills are in the correct location:

```bash
ls ~/.claude/skills/intent-essentials/SKILL.md
ls ~/.claude/skills/elixir-essentials/SKILL.md
ls ~/.claude/skills/ash-ecto-essentials/SKILL.md
ls ~/.claude/skills/phoenix-liveview/SKILL.md
```

### Subagent sync reports "modified locally"

This is a known issue (see ST0020). If you haven't intentionally modified the subagent, use `--force`:

```bash
intent claude subagents sync --force
```
