# [[PROJECT_NAME]] Project Guidelines

This is an Intent v[[INTENT_VERSION]] project.

## Rules

1. **The Highlander Rule**: There can be only one. Never duplicate code paths, modules, or logic for the same concern. Before creating anything new, check MODULES.md.
2. **Thin controllers and LiveViews**: Business logic lives in service modules or Ash domains, never in controllers, LiveViews, or CLI commands.
3. **Tagged tuples everywhere**: Functions return `{:ok, result}` or `{:error, reason}`. Never return bare values from fallible operations.
4. **No silent failures**: Every error path must be handled explicitly. No bare `rescue`, no `catch-all` that swallows errors.
5. **Check before you create**: Before creating a new module, check MODULES.md. If a module already owns that concern, use it.
6. **Register before you code**: When you must create a new module, add it to MODULES.md FIRST, then create the file.

## Project Structure

- `intent/` - Project artifacts (steel threads, docs, work tracking)
  - `st/` - Steel threads organized as directories
  - `docs/` - Technical documentation
  - `llm/` - LLM-specific guidelines (MODULES.md, DECISION_TREE.md, ARCHETYPES.md)
- `.intent/` - Configuration and metadata

## Key Reference Files

Read these on every session start and after every context reset:

- `CLAUDE.md` (this file) - Project rules and structure
- `intent/llm/MODULES.md` - Module registry (the Highlander enforcer)
- `intent/llm/DECISION_TREE.md` - Where does this code belong?
- `intent/wip.md` - Current work in progress
- `intent/restart.md` - Session restart context (if exists)

## Steel Threads

Steel threads are organized as directories under `intent/st/`:

- Each steel thread has its own directory (eg ST0001/)
- Minimum required file is `info.md` with metadata
- Optional files: design.md, impl.md, tasks.md

## Commands

### Core Commands

- `intent st new "Title"` - Create a new steel thread
- `intent st list` - List all steel threads
- `intent st show <id>` - Show steel thread details
- `intent wp new <STID> "Title"` - Create a new work package
- `intent wp list <STID>` - List work packages for a steel thread
- `intent wp start <STID/NN>` - Mark work package as WIP
- `intent wp done <STID/NN>` - Mark work package as Done
- `intent doctor` - Check configuration
- `intent help` - Get help

### Claude Commands

- `intent claude subagents <command>` - Manage Claude subagents
- `intent claude skills <command>` - Manage Claude skills
- `intent claude prime` - Synthesize project knowledge into MEMORY.md

## Session Workflow

### On session start

1. Read this file, MODULES.md, DECISION_TREE.md, wip.md, restart.md
2. Understand current state before making any changes
3. Ask clarifying questions if the task is ambiguous

### Before creating code

1. Check MODULES.md -- does a module already own this concern?
2. Check DECISION_TREE.md -- where does this code belong?
3. If creating a new module: register in MODULES.md first

### On session end

1. Update intent/wip.md with current state
2. Update intent/restart.md with context for next session
3. Commit with descriptive message

## Author

[[AUTHOR]]
