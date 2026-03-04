# Intent Project Guidelines

This is an Intent v2.5.0 project.

## Rules

1. **The Highlander Rule**: There can be only one. Never duplicate code paths, modules, or logic for the same concern. Before creating anything new, check MODULES.md.
2. **Thin scripts**: Business logic lives in dedicated modules, not in command dispatch or inline heredocs.
3. **No silent failures**: Every error path must be handled explicitly via `error()` from intent_helpers.
4. **Check before you create**: Before creating a new script or function, check `intent/llm/MODULES.md`.
5. **Register before you code**: When you must create a new module, add it to MODULES.md FIRST, then create the file.
6. **Single template source**: All generated content comes from `lib/templates/` via sed substitution. No inline heredocs duplicating template content.

## Key Reference Files

Read these on every session start and after every context reset:

- `CLAUDE.md` (this file)
- `intent/llm/MODULES.md` - Module registry (the Highlander enforcer)
- `intent/llm/DECISION_TREE.md` - Where does this code belong?
- `intent/wip.md` - Current work in progress
- `intent/restart.md` - Session restart context (if exists)

## Project Structure

- `intent/` - Project artifacts (steel threads, docs, work tracking)
  - `st/` - Steel threads organized as directories
  - `docs/` - Technical documentation
  - `llm/` - LLM-specific guidelines (MODULES.md, DECISION_TREE.md)
- `.intent/` - Configuration and metadata

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
- `intent wp show <STID/NN>` - Show work package details
- Specifiers accept bare numbers (`5` = `ST0005`, `5/01` = `ST0005/01`)
- WP directories live under `STXXXX/WP/NN/info.md`; titles support special characters
- `intent plugin` - Discover plugins and their commands
- `intent treeindex <dir>` - Generate `.treeindex` directory summaries
- `intent doctor` - Check configuration
- `intent help` - Get help

### AGENTS.md Commands (NEW in v2.3.0)

- `intent agents init` - Create AGENTS.md for the project
- `intent agents sync` - Update AGENTS.md with latest project state
- `intent agents validate` - Check AGENTS.md compliance

### Claude Commands

- `intent claude subagents <command>` - Manage Claude subagents (init, list, install, sync, uninstall, show, status)
- `intent claude skills <command>` - Manage Claude skills (list, install, sync, uninstall, show)
- `intent claude upgrade [--apply]` - Diagnose and upgrade project LLM guidance files

## Migration Notes

This project was migrated from STP to Intent v2.0.0 on 2025-07-16, through v2.1.0, v2.2.0, v2.3.0, and is now at v2.5.0.

- Old structure: `stp/prj/st/`, `stp/eng/`, etc.
- New structure: `intent/st/`, `intent/docs/`, etc.
- Configuration moved from YAML to JSON format

## Intent Agents

This project has access to specialized AI agents through Intent's agent system. These agents are Claude Code sub-agents with domain-specific expertise.

### Available Agents

1. **intent** - Intent methodology specialist
   - Steel thread management and best practices
   - Intent command usage and workflows
   - Project structure guidance

2. **elixir** - Elixir code doctor
   - Functional programming patterns
   - Elixir Usage Rules and best practices
   - Ash and Phoenix framework expertise
   - Code review and optimization

3. **socrates** - CTO Review Mode
   - Technical decision-making via Socratic dialog
   - Architecture review and analysis
   - Strategic technology choices
   - Risk assessment and mitigation

4. **worker-bee** - Worker-Bee Driven Design specialist
   - WDD 6-layer architecture enforcement
   - Project structure mapping and validation
   - Code scaffolding with templates
   - Mix task generation for WDD compliance

5. **diogenes** - Elixir Test Architect
   - Socratic dialog for test specification generation
   - Two personas: Aristotle (Empiricist) + Diogenes (Skeptic)
   - Specify mode: produces formal test specs from module analysis
   - Validate mode: gap analysis of tests vs specifications

### Using Agents

To delegate tasks to specialized agents, use the Task tool with the appropriate subagent_type:

```
Task(
  description="Review Elixir code",
  prompt="Review the authentication module for Usage Rules compliance",
  subagent_type="elixir"
)
```

### When to Use Agents

**Use the intent agent for:**

- Creating or managing steel threads
- Understanding Intent project structure
- Following Intent best practices

**Use the elixir agent for:**

- Writing idiomatic Elixir code
- Reviewing code for Usage Rules
- Ash/Phoenix implementation guidance
- Functional programming patterns

**Use the socrates agent for:**

- Technical architecture reviews
- Strategic technology decisions
- Risk assessment for technical choices
- Facilitating thoughtful technical discussions

**Use the worker-bee agent for:**

- Enforcing Worker-Bee Driven Design principles
- Mapping project structure to WDD layers
- Validating WDD compliance
- Scaffolding WDD-compliant code
- Generating Mix tasks for WDD workflows

**Use main Claude for:**

- General programming tasks
- Cross-cutting concerns
- Integration between systems
- Tasks requiring broad context

### Best Practices

1. Delegate specialized tasks to appropriate agents
2. Provide clear, focused prompts to agents
3. Agents work best with specific, bounded tasks
4. Consider using multiple agents for complex workflows

## Treeindex

`.treeindex` files are pre-computed directory summaries that let Claude quickly orient itself in a codebase without reading every file. They contain a concise overview of each directory's contents, purpose, and key files.

**Convention:** Before exploring an unfamiliar directory, check `intent/.treeindex/<dir>/.treeindex` for an existing summary. This avoids redundant Glob/Grep/Read operations and saves context.

- **Location:** All `.treeindex` files live in the `intent/.treeindex/` shadow directory (eg `intent/.treeindex/lib/.treeindex`)
- **Regenerate:** Run `intent treeindex <dir>` to generate or refresh summaries for a directory tree

## Author

matts

## Usage Rules

- DO NOT ADD CLAUDE TO GIT COMMITS. EVER.
