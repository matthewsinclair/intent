---
description: "Core Intent workflow rules for steel threads, agents, treeindex, and session management"
---

# Intent Essentials

Core Intent workflow rules enforced on every interaction. These are mandatory -- no exceptions.

## Rules

### 1. Use `intent st` commands for steel thread management

NEVER manually create directories under `intent/st/`. NEVER manually edit `status:` fields in steel thread frontmatter. Use the CLI to manage lifecycle.

```bash
# BAD — manual creation
mkdir -p intent/st/ST0005
echo "status: active" > intent/st/ST0005/info.md

# GOOD — use the CLI
intent st new "My steel thread"
intent st list
intent st show ST0005
intent st edit ST0005
```

### 2. Use `intent agents sync` to update AGENTS.md

NEVER edit `intent/llm/AGENTS.md` directly — it is auto-generated from project state. Manual edits will be overwritten on next sync.

```bash
# BAD — direct edit
echo "New section" >> intent/llm/AGENTS.md

# GOOD — regenerate from project state
intent agents sync
```

### 3. Use `intent treeindex` on subdirectories only

NEVER run `intent treeindex` on the project root. Always target specific subdirectories. Run multiple in parallel for speed.

```bash
# BAD — project root
intent treeindex .

# GOOD — target subdirectories
intent treeindex bin
intent treeindex lib
intent treeindex docs

# GOOD — parallel execution
intent treeindex bin & intent treeindex lib & intent treeindex docs & wait
```

### 4. Check `.treeindex` before exploring unfamiliar directories

Before deep-diving into an unfamiliar directory with Glob/Grep/Read, check if a pre-computed summary exists. This saves context and avoids redundant exploration.

```bash
# GOOD — check the summary first
cat intent/.treeindex/lib/.treeindex
cat intent/.treeindex/bin/.treeindex
```

Read `intent/.treeindex/<dir>/.treeindex` before scanning the directory contents.

### 5. Use `intent claude skills` for skill management

NEVER manually create or edit files in `.claude/skills/`. Use the CLI for install, sync, and removal. Skills use SHA256 manifests for tracking.

```bash
# BAD — manual copy
cp some-skill/SKILL.md ~/.claude/skills/my-skill/SKILL.md

# GOOD — use the CLI
intent claude skills install intent-elixir-essentials
intent claude skills sync
intent claude skills uninstall intent-elixir-essentials
```

### 6. Steel thread document conventions

Each steel thread lives in `intent/st/<ID>/`. The minimum required file is `info.md` with frontmatter metadata. Optional companion files provide design and tracking.

- `info.md` — required, contains title, status, dates, description
- `design.md` — architecture and design decisions
- `impl.md` — implementation notes and as-built state
- `tasks.md` — work breakdown and progress tracking
- `WP/<NN>/info.md` — work packages within a steel thread

Frontmatter uses `verblock:` format: `"DD Mon YYYY:vX.Y: Author - Description"`

### 7. Session wrap-up workflow

Before ending a session, update tracking files to preserve context for the next session:

1. Update `intent/wip.md` with current state and what is next
2. Update `intent/restart.md` with session restart context
3. Update `.claude/restart.md` with WIP/TODO focus
4. Commit changes before ending session

### 8. Use `intent wp` commands for work package management

NEVER manually create directories under `intent/st/STXXXX/WP/`. Use the CLI.

```bash
# BAD -- manual creation
mkdir -p intent/st/ST0005/WP/01
echo "status: WIP" > intent/st/ST0005/WP/01/info.md

# GOOD -- use the CLI
intent wp new ST0005 "Implement core logic"
intent wp list ST0005
intent wp start ST0005/01
intent wp done ST0005/01
```
