# . Project Guidelines

This is an Intent v2.2.0 project (formerly STP).

## Project Structure

- `intent/` - Project artifacts (steel threads, docs, work tracking)
  - `st/` - Steel threads organized as directories
  - `docs/` - Technical documentation
  - `llm/` - LLM-specific guidelines
- `backlog/` - Task management (if using Backlog.md)
- `.intent/` - Configuration and metadata

## Steel Threads

Steel threads are organized as directories under `intent/st/`:

- Each steel thread has its own directory (e.g., ST0001/)
- Minimum required file is `info.md` with metadata
- Optional files: design.md, impl.md, tasks.md

## Commands

### Core Commands
- `intent st new "Title"` - Create a new steel thread
- `intent st list` - List all steel threads
- `intent st show <id>` - Show steel thread details
- `intent doctor` - Check configuration
- `intent help` - Get help

### AGENTS.md Commands (NEW in v2.3.0)
- `intent agents init` - Create AGENTS.md for the project
- `intent agents sync` - Update AGENTS.md with latest project state
- `intent agents validate` - Check AGENTS.md compliance

### Claude Subagent Commands (renamed in v2.3.0)
- `intent claude subagents init` - Initialize Claude subagent configuration
- `intent claude subagents list` - List available Claude subagents
- `intent claude subagents install <name>` - Install a Claude subagent

## Migration Notes

This project was migrated from STP to Intent v2.0.0 on 2025-07-16, upgraded to v2.1.0 on 2025-07-27, upgraded to v2.2.0 on 2025-08-05, and upgraded to v2.3.0 on 2025-08-20 with plugin architecture and AGENTS.md support.

- Old structure: `stp/prj/st/`, `stp/eng/`, etc.
- New structure: `intent/st/`, `intent/docs/`, etc.
- Configuration moved from YAML to JSON format

## Intent Agents

This project has access to specialized AI agents through Intent's agent system. These agents are Claude Code sub-agents with domain-specific expertise.

### Available Agents

1. **intent** - Intent methodology specialist
   - Steel thread management and best practices
   - Backlog task tracking
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
- Working with backlog tasks
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

## Author

matts

## Usage Rules

- DO NOT ADD CLAUDE TO GIT COMMITS. EVER.