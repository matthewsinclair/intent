---
description: "Core Intent workflow rules for steel threads, work packages, agents, skills, and session management"
---

# Intent Essentials

Core Intent workflow rules enforced on every interaction. These are mandatory -- no exceptions.

## Rules

### 1. Use `intent st` commands for steel thread management

NEVER manually create directories under `intent/st/`. NEVER manually edit `status:` fields in steel thread frontmatter. Use the CLI to manage lifecycle.

```bash
# BAD — manual creation
mkdir -p intent/st/ST0000
echo "status: active" > intent/st/ST0000/info.md

# GOOD — use the CLI
intent st new "My steel thread"
intent st list
intent st show ST0000
intent st edit ST0000
```

### 2. Use `intent agents sync` to update AGENTS.md

NEVER edit root `AGENTS.md` directly — it is auto-generated from project state by `intent agents sync`. Manual edits will be overwritten on the next sync. `AGENTS.md` lives at the project root as a real file (not a symlink); older projects may still have a legacy `intent/llm/AGENTS.md` which is retired and should be removed.

```bash
# BAD — direct edit
echo "New section" >> AGENTS.md

# GOOD — regenerate from project state
intent agents sync
```

### 3. Use `intent claude skills` for skill management

NEVER manually create or edit files in `.claude/skills/`. Use the CLI for install, sync, and removal. Skills use SHA256 manifests for tracking.

```bash
# BAD — manual copy
cp some-skill/SKILL.md ~/.claude/skills/my-skill/SKILL.md

# GOOD — use the CLI
intent claude skills install in-elixir-essentials
intent claude skills sync
intent claude skills uninstall in-elixir-essentials
```

### 4. Steel thread document conventions

Each steel thread lives in `intent/st/<ID>/`. The minimum required file is `info.md` with frontmatter metadata. Optional companion files provide design and tracking.

- `info.md` — required, contains title, status, dates, description
- `design.md` — architecture and design decisions
- `impl.md` — implementation notes and as-built state
- `tasks.md` — work breakdown and progress tracking
- `WP/<NN>/info.md` — work packages within a steel thread

Frontmatter is written by v3 from the store, so do not hand-author it: `info.md` carries `st_id`, `title`, `status`, `created`, `completed`; `WP/<NN>/info.md` carries `wp_id`, `title`, `scope`, `status`. **This line claimed `verblock:` until 2026-09-08 and v3 writes no such field on a thread view** -- that is v2's shape, and v3 emits it only when ingesting a v2 tree. `verblock` remains the house style for HAND-AUTHORED persistent documents such as `intent/wip.md`, which is a different document class and the reason the wrong claim read as plausible.

**AND THE FILES ABOVE ARE REALISED LAZILY.** `intent st new` writes the store, not the tree, so `intent/st/<ID>/` does not exist until something realises it -- `intent edit <kind> <ID> --path` realises one, `intent organize` reconciles the tree against `.intentfiles`. An agent that runs `st new` and then lists the directory will find nothing, and nothing is wrong.

### 5. Session wrap-up workflow

Before ending a session, update tracking files to preserve context for the next session:

1. Update `intent/wip.md` with current state and what is next -- **DOING and TODO only; done work is illegal in it**
2. Update `intent/restart.md` with session restart context
3. **Leave `.claude/restart.md` alone** unless the entry procedure itself changed -- it is the ENTRY POINT and holds NO STATE
4. Commit changes before ending session

**STEP 3 SAID _update `.claude/restart.md` with WIP/TODO focus_ UNTIL 2026-09-08, WHICH IS THE OPPOSITE OF WHAT `/in-finish` SAYS ABOUT THE SAME FILE.** Two skills, one file, contradictory orders -- and the one that was wrong is the one loaded at every session start, so it was read far more often. The three tracking files were three copies of one narrative until 2026-08-24, each opening with a banner claiming to supersede the others; **state written into the entry point is how that rebuilds.** `/in-finish` step 4 carries the full reasoning and is the one home for it.

### 6. Use `intent wp` commands for work package management

NEVER manually create directories under `intent/st/STXXXX/WP/`. Use the CLI.

```bash
# BAD -- manual creation
mkdir -p intent/st/ST0000/WP/01
echo "status: WIP" > intent/st/ST0000/WP/01/info.md

# GOOD -- use the CLI
intent wp new ST0000 "Implement core logic"
intent wp list ST0000
intent wp start ST0000/01
intent wp done ST0000/01
```

### 7. Use `intent todo` for the flat work view

`intent todo` projects a flat DOING / TODO / DONE view across all steel threads and work packages from their real status (into `intent/todo.md`). Reach for it to see where things stand and to drive next steps. It is GENERATED from ST/WP status -- do not hand-maintain a separate list that would drift from the threads.

```bash
intent todo              # show the flat DOING / TODO / DONE view
intent todo update       # regenerate from current ST/WP status
intent todo --json       # machine-readable view
```

### 8. Never invoke `intent fc` -- fiat close is the human's verb

`IN-AG-FIAT-001`, severity critical. Read it with `intent claude rules show IN-AG-FIAT-001`; it is not restated here, because a second copy of a rule's content in a skill is exactly the Highlander violation the rule library exists to prevent.

**Nothing stops you from running it.** You share the human's uid and their shell, so this is a contract you hold and not a wall you would hit -- and every invocation is attributable to you, permanently. Do not run it, do not put it in a script, a hook, a skill or a Makefile, and do not ask a peer to run it on your behalf.

Proposing a fiat close to the human, with the reason you would give, is the correct move when a requirement genuinely is not worth finishing.

## Red Flags

| Rationalization                        | Reality                                          |
| -------------------------------------- | ------------------------------------------------ |
| "I'll use the CLI later"               | Manual creation causes drift. Use the CLI now.   |
| "This is too small for a steel thread" | Every piece of work gets tracked. No exceptions. |
| "I'll update docs at the end"          | Sessions get interrupted. Update docs as you go. |
