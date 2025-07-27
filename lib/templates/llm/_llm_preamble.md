---
verblock: "27 Jul 2025:v0.2: Matthew Sinclair - Updated for Intent v2.1.0 with agent system"
intent_version: 2.1.0
---
# LLM Preamble

This document provides essential context for LLMs working on the [[PROJECT_NAME]] project. Share this document at the beginning of each LLM session to establish baseline understanding.

## Project Context

[[PROJECT_NAME]] follows the Intent methodology (formerly Steel Thread Process), which organizes development into discrete "steel threads" - self-contained units of functionality that enable incremental progress with clear documentation.

## Navigation Guide

When working with this repository, focus on these key documents in order:

1. **START HERE**: `CLAUDE.md` - Project-specific guidelines and instructions
2. **NEXT**: `intent/st/` - Review steel thread directories for project history
3. **THEN**: `Backlog.md` (if exists) - Current tasks and priorities
4. **REFERENCE**: `intent/docs/` - Technical documentation

## Documentation Structure

The Intent methodology organizes project information through this directory structure:

- **intent/**: Project artifacts
  - **intent/st/**: Steel thread directories (ST0001/, ST0002/, etc.)
  - **intent/docs/**: Technical documentation
  - **intent/llm/**: LLM-specific guidelines
- **backlog/**: Task management (if using Backlog.md)
- **.intent/**: Configuration and metadata

## Steel Thread Process

Work in this project is organized through steel threads:

1. **Definition**: A steel thread is a self-contained unit of work representing a logical piece of functionality
2. **Structure**: Each steel thread has its own directory with:
   - `info.md` - Metadata and overview (required)
   - `design.md` - Design documentation (optional)
   - `impl.md` - Implementation notes (optional)
   - `tasks.md` - Task breakdown (optional)
3. **Management**: Steel threads are created and tracked using Intent commands

## Intent Agent System

This project can leverage specialized AI agents through Intent's agent system:

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

### Using Agents

Delegate tasks to specialized agents using the Task tool:

```
Task(
  description="Short task description",
  prompt="Detailed instructions for the agent",
  subagent_type="agent_name"
)
```

### When to Use Agents

**Use specialized agents when:**

- Task requires deep domain knowledge
- Performing focused code reviews
- Following specific methodologies
- Task is well-bounded and focused

**Use main Claude when:**

- Task requires full project context
- Integrating multiple systems
- General programming tasks
- Exploratory work or debugging

## Command Usage

The Intent system provides these commands:

- `intent st new "Title"` - Create a new steel thread
- `intent st list` - List all steel threads
- `intent st show <id>` - Show steel thread details
- `intent agents init` - Initialize agent configuration
- `intent agents list` - List available agents
- `intent agents install <agent>` - Install an agent
- `intent agents sync` - Update agents to latest versions
- `intent agents status` - Check agent health
- `intent doctor` - Check configuration
- `intent help` - Get help

## Code Style and Conventions

The following guidelines apply to this project:

- **Indentation**: Use 2-space indentation in all programming languages
- **Documentation**: Update documentation alongside code changes
- **Naming**: Use descriptive variable and function names
- **Error Handling**: Implement robust error handling
- **Testing**: Include appropriate tests for new functionality
- **Markdown**: Maintain consistent formatting

[[Add specific code style guidelines for your project's primary languages]]

## How to Help

When assisting with this project:

1. Review CLAUDE.md for project-specific guidelines
2. Use specialized agents for domain-specific tasks
3. Maintain consistency with existing patterns
4. Update documentation alongside code changes
5. Track progress using Backlog.md if available
6. Create steel threads for new features or significant work

## Project-Specific Information

[[Add essential project-specific information here:

- Key technologies used
- External dependencies
- Development setup instructions
- Architectural principles
- Known limitations or considerations]]
