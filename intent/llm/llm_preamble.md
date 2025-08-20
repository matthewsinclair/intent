---
verblock: "20 Aug 2025:v0.3: DEPRECATED - Replaced by AGENTS.md in Intent v2.3.0"
intent_version: 2.1.0
---
# LLM Preamble [DEPRECATED]

> **⚠️ DEPRECATION NOTICE**: This file is deprecated as of Intent v2.3.0.
> LLM context is now provided via AGENTS.md which follows the universal 
> AGENTS.md specification. See the AGENTS.md file in this project.
>
> This file is kept for backward compatibility but will be removed in a future version.

This document provides essential context for LLMs working with Intent projects.

## Project Overview

Intent (formerly STP - Steel Thread Process) is a system designed to create a structured workflow and documentation process for developers collaborating with Large Language Models. It provides:

1. A standardized directory structure for project documentation
2. Shell scripts for managing project workflows
3. A methodology centered around "steel threads" - self-contained units of work
4. Markdown templates for documentation
5. Integration with Claude Code sub-agents for specialized assistance

The system is intentionally lightweight, using shell scripts and markdown files to maximize portability and minimize dependencies. It integrates with existing development workflows and helps preserve context across development sessions with LLMs.

## Navigation Guide

When working with an Intent project, focus on these key documents in order:

1. **START HERE**: `CLAUDE.md` - Project-specific guidelines and instructions
2. **NEXT**: `intent/st/` - Review steel thread directories for project history
3. **THEN**: `Backlog.md` (if exists) - Current tasks and priorities
4. **REFERENCE**: `intent/docs/` - Technical documentation

## Key System Components

The Intent system consists of:

1. **Core Commands**:
   - `intent st new "Title"` - Create a new steel thread
   - `intent st list` - List all steel threads
   - `intent st show <id>` - Show steel thread details
   - `intent agents` - Manage AI agents
   - `intent doctor` - Check configuration

2. **Directory Structure**:
   - `intent/` - Project artifacts (steel threads, docs, work tracking)
   - `intent/st/` - Steel threads organized as directories
   - `intent/docs/` - Technical documentation
   - `intent/llm/` - LLM-specific guidelines
   - `backlog/` - Task management (if using Backlog.md)
   - `.intent/` - Configuration and metadata

3. **Agent System**: Specialized AI assistants for domain-specific tasks

# Intent Agent System Guide

## Overview

Intent integrates with Claude Code's sub-agent feature to provide specialized AI assistants. These agents extend Claude's capabilities with domain-specific knowledge and focused expertise.

## Understanding Intent Agents

Intent agents are Claude Code sub-agents that:
- Have their own context window separate from the main conversation
- Possess specialized knowledge and system prompts
- Access a focused set of tools appropriate to their domain
- Return comprehensive results that you can use in your main workflow

## Agent Architecture

```
intent agents init        # Initialize agent configuration
intent agents list        # Show available and installed agents  
intent agents install     # Install agents to ~/.claude/agents/
intent agents sync        # Update agents to latest versions
intent agents status      # Check agent health
```

Agents are stored in:
- `$INTENT_HOME/agents/` - Global agents shipped with Intent
- `./intent/agents/` - Project-specific custom agents
- `~/.claude/agents/` - Where Claude Code reads installed agents

## Available Intent Agents

### intent - Intent Methodology Specialist

**Expertise:**
- Steel thread methodology and management
- Intent command usage and best practices
- Backlog task tracking with Backlog.md
- Project structure and organization
- Migration from STP to Intent v2.x

**Use Cases:**
- `Task(description="Create steel thread", prompt="/create-st 'User authentication system'", subagent_type="intent")`
- `Task(description="Review project structure", prompt="Analyze this project's Intent setup and suggest improvements", subagent_type="intent")`
- `Task(description="Manage backlog", prompt="Review Backlog.md and prioritize tasks", subagent_type="intent")`

### elixir - Elixir Code Doctor

**Expertise:**
- Elixir Usage Rules (19 best practices)
- Functional programming patterns
- Ash framework patterns and resources
- Phoenix framework best practices
- Code review and optimization

**Use Cases:**
- `Task(description="Review Elixir code", prompt="Review this module for Usage Rules compliance", subagent_type="elixir")`
- `Task(description="Implement Ash resource", prompt="Create an Ash resource for user profiles with proper actions", subagent_type="elixir")`
- `Task(description="Optimize GenServer", prompt="Review this GenServer for performance and suggest improvements", subagent_type="elixir")`

## When to Use Agents vs Main Claude

### Use Specialized Agents When:
- Task requires deep domain knowledge (Intent methodology, Elixir patterns)
- Performing focused code reviews or audits
- Following specific methodologies or frameworks
- Task is well-bounded and doesn't require broad context
- You need expert guidance on best practices

### Use Main Claude When:
- Task requires understanding the full project context
- Integrating multiple systems or languages
- General programming tasks
- Tasks that span multiple domains
- Exploratory work or debugging

## Agent Invocation Patterns

### Basic Pattern
```
Task(
  description="Short task description",
  prompt="Detailed instructions for the agent",
  subagent_type="agent_name"
)
```

### Complex Workflow Pattern
Chain multiple agents for sophisticated workflows:

1. Use intent agent to understand project structure
2. Use main Claude to implement features
3. Use elixir agent to review and optimize code

### Slash Command Pattern
Agents can execute custom slash commands:
```
Task(
  description="Execute command",
  prompt="/check-usage-rules path/to/module.ex",
  subagent_type="elixir"
)
```

## Best Practices

1. **Be Specific**: Agents work best with clear, focused prompts
2. **Provide Context**: Include relevant file paths and specific requirements
3. **Batch Related Tasks**: Group similar tasks for the same agent
4. **Trust Agent Expertise**: Agents have specialized knowledge - defer to their recommendations
5. **Use Proactively**: Don't wait for users to ask - use agents when appropriate

## Creating Custom Project Agents

Projects can define custom agents in `./intent/agents/`:

```yaml
---
name: security-reviewer
tools:
  - Read
  - Grep
  - Bash
---

You are a security specialist for this project...
```

Install with: `intent agents install ./intent/agents/security-reviewer.md`

## Integration with Intent Workflow

1. **During intent init**: Offers to install agents if Claude Code is detected
2. **With steel threads**: Intent agent understands ST methodology
3. **For code review**: Agents can be invoked after significant code changes
4. **In CI/CD**: Project agents can enforce standards

## Troubleshooting

- Use `intent agents status` to check agent health
- Use `intent doctor` to verify agent configuration
- Agents are loaded fresh each session - modifications persist
- Check `~/.intent/agents/installed-agents.json` for installation metadata

Remember: Intent agents are powerful tools that extend your capabilities. Use them proactively to deliver higher quality results and follow best practices consistently.

## Development Guidelines

1. **Code Style**:
   - Use 2-space indentation in any programming language
   - Follow language-specific conventions as noted in CLAUDE.md
   - Maintain POSIX compatibility for scripts

2. **Documentation**:
   - Keep markdown documents consistently formatted
   - Update documentation as part of any implementation work
   - Follow the verblock pattern for versioning

3. **Steel Thread Process**:
   - Work is organized into steel threads (ST####)
   - Each steel thread has its own directory in `intent/st/`
   - Minimum required file is `info.md` with metadata

## How to Help

When assisting with this project:

1. Review CLAUDE.md for project-specific guidelines
2. Use specialized agents for domain-specific tasks
3. Maintain consistency with existing patterns
4. Update documentation alongside code changes
5. Track progress using Backlog.md if available