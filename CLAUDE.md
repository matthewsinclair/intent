# Intent Project Guidelines

This is an Intent v2.9.0 project.

## On every session start and after every `/compact`

Invoke `/in-session` before doing anything else. It auto-detects the project language and loads the right coding skills (`/in-essentials`, `/in-standards`, plus Elixir / Ash / LiveView skills when `mix.exs` matches). One command replaces the manual skill-reload list.

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
- `intent/docs/rules.md` - Rule library: schema, authoring, validation
- `intent/docs/critics.md` - Critic subagent contract and report format
- `intent/docs/writing-extensions.md` - User extensions at `~/.intent/ext/`
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

## Rules library

Intent's rule library is the single source of truth for coding standards. Each rule is a small Markdown file with structured frontmatter, a Detection heuristic, and bad/good examples. Skills cite rules by ID; Critic subagents enforce them.

- Library root: `intent/plugins/claude/rules/`
- Authoring guide: `intent/docs/rules.md`
- Schema reference: `intent/plugins/claude/rules/_schema/rule-schema.md`
- CLI: `intent claude rules list | show | validate | index`

## Critic subagents

Critics are thin orchestrators: they read the rule library at invocation time, apply each rule's Detection heuristic to target source files, and emit a stable severity-grouped report. They never autofix or shell out to external linters.

- Family: `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`
- Modes: `code` and `test` (`critic-shell` is `code` only)
- Contract: `intent/docs/critics.md`
- Per-project config: `.intent_critic.yml` at the project root (disable rules, set severity threshold)

## User extensions

Extensions live at `~/.intent/ext/<name>/` and contribute subagents, skills, or rule packs without modifying canon. Discovery is layered: canon is the default; user extensions override by name with a visible shadow warning.

- Authoring guide: `intent/docs/writing-extensions.md`
- Manifest schema: `intent/plugins/claude/ext-schema/extension.schema.json`
- CLI: `intent ext list | show | validate | new`
- Reference extension: `worker-bee` (relocated from canon in v2.9.0)

## Migration Notes

This project was migrated from STP to Intent v2.0.0 on 2025-07-16, through v2.1.0, v2.2.0, v2.3.0, v2.8.x, and is now at v2.9.0.

- Old structure: `stp/prj/st/`, `stp/eng/`, etc.
- New structure: `intent/st/`, `intent/docs/`, etc.
- Configuration moved from YAML to JSON format

### v2.8.2 → v2.9.0 jump

The v2.9.0 migration (`migrate_v2_8_2_to_v2_9_0` in `bin/intent_helpers`) does four things:

1. Stamps `.intent/config.json` with `intent_version: 2.9.0`.
2. Bootstraps `~/.intent/ext/` with a README on first run.
3. Seeds `~/.intent/ext/worker-bee/` from `lib/templates/ext-seeds/worker-bee/` (skipped if already present — the migration never overwrites user state).
4. Prunes installed copies of the deleted `elixir` subagent and the relocated `worker-bee` from `~/.claude/agents/` and `~/.intent/agents/installed-agents.json`.

To verify post-upgrade state:

```bash
intent doctor                                  # general health check
cat .intent/config.json | jq .intent_version   # should print "2.9.0"
intent ext list                                # should show worker-bee
intent claude subagents list | grep critic-    # should show 5 critic-* entries
```

Mid-session subagent registration freezes per Claude Code session — start a new session before invoking any newly-installed Critic via `Task()`.

## Intent Agents

This project has access to specialized AI agents through Intent's agent system. These agents are Claude Code sub-agents with domain-specific expertise.

### Available Agents

1. **intent** - Intent methodology specialist
   - Steel thread management and best practices
   - Intent command usage and workflows
   - Project structure guidance

2. **socrates** - CTO Review Mode
   - Technical decision-making via Socratic dialog
   - Architecture review and analysis
   - Strategic technology choices
   - Risk assessment and mitigation

3. **diogenes** - Elixir Test Architect
   - Socratic dialog for test specification generation
   - Two personas: Aristotle (Empiricist) + Diogenes (Skeptic)
   - Specify mode: produces formal test specs from module analysis
   - Validate mode: gap analysis of tests vs specifications

4. **critic-elixir** / **critic-rust** / **critic-swift** / **critic-lua** / **critic-shell** - Rule-library critics
   - Thin orchestrators: read `intent/plugins/claude/rules/`, apply Detection heuristics, report findings by severity
   - One critic per language; modes are `code` and `test` (`critic-shell` is `code` only)
   - Output is a stable severity-grouped report; never autofix, never shell out to external linters
   - Invocation: `Task(subagent_type="critic-<lang>", prompt="review <targets>")` or `prompt="test-check <targets>"`
   - See `intent/docs/critics.md` for the full contract

**Note:** The standalone `elixir` subagent was removed in v2.9.0. Its rule content lives in `intent/plugins/claude/rules/elixir/` and is enforced by `critic-elixir`. The `worker-bee` subagent was relocated from canon to the reference extension at `~/.intent/ext/worker-bee/` — install via `intent claude subagents install worker-bee` after a v2.9.0 upgrade. See `intent/docs/writing-extensions.md`.

### Using Agents

To delegate tasks to specialized agents, use the Task tool with the appropriate subagent_type:

```
Task(
  description="Review Elixir code",
  prompt="review lib/myapp/accounts.ex lib/myapp/accounts_test.exs",
  subagent_type="critic-elixir"
)
```

### When to Use Agents

**Use the intent agent for:**

- Creating or managing steel threads
- Understanding Intent project structure
- Following Intent best practices

**Use a critic-<lang> agent for:**

- Reviewing source files against Intent's rule library
- Stage-2 of `/in-review` (the skill auto-detects language and dispatches)
- Per-language code or test review with stable, severity-grouped output
- See `intent/docs/critics.md` for the contract

**Use the socrates agent for:**

- Technical architecture reviews
- Strategic technology decisions
- Risk assessment for technical choices
- Facilitating thoughtful technical discussions

**Use the worker-bee agent for** (install from `~/.intent/ext/worker-bee/` after v2.9.0 upgrade):

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
