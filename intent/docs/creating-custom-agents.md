---
verblock: "15 Aug 2025:v1.0: Created comprehensive guide for Intent agent creation"
intent_version: 2.2.0
---

# Creating Custom Intent Agents

This guide provides step-by-step instructions for creating custom Intent agents that integrate with Claude Code's sub-agent system.

## Overview

Intent agents are specialized AI assistants with domain-specific knowledge and focused expertise. They extend Claude's capabilities by providing:

- Dedicated context windows separate from main conversation
- Specialized system prompts and knowledge
- Focused tool access appropriate to their domain
- Comprehensive results for specific tasks

## Prerequisites

- Intent v2.2.0 or later installed
- Claude Code CLI installed and configured
- Basic understanding of YAML frontmatter and JSON

## Agent Structure

Each Intent agent consists of:

- **Directory**: `intent/agents/agent-name/`
- **Agent Definition**: `agent.md` with YAML frontmatter and system prompt
- **Metadata**: `metadata.json` with version and configuration details

## Step-by-Step Creation Process

### 1. Create Agent Directory

Create a new directory under `intent/agents/` for your agent:

```bash
mkdir -p intent/agents/your-agent-name/
cd intent/agents/your-agent-name/
```

**Naming Convention:**

- Use lowercase with hyphens (eg `security-reviewer`, `api-designer`)
- Be descriptive but concise
- Avoid spaces or special characters

### 2. Create Agent Definition (`agent.md`)

Create the main agent file with YAML frontmatter and system prompt:

```markdown
---
name: your-agent-name
description: Brief one-line description of your agent's purpose and expertise
tools: Bash, Read, Write, Edit, Grep
---

You are a specialized [DOMAIN] expert assistant with deep knowledge in [SPECIFIC AREAS].

## Your Expertise

You have extensive experience in:

- [Primary capability 1]
- [Primary capability 2]
- [Primary capability 3]
- [Framework/tool expertise if applicable]

## Your Role

When working with users, you should:

1. [Specific behavior 1]
2. [Specific behavior 2]
3. [Domain-specific guidelines]

## Best Practices

Always follow these principles:

- [Domain-specific best practice 1]
- [Domain-specific best practice 2]
- [Quality standards for your domain]

## When to Use This Agent

Use this agent for:

- [Specific use case 1]
- [Specific use case 2]
- [Complex workflow description]

## Integration with Intent

When working within Intent projects:

- Reference steel threads when relevant
- Document decisions in appropriate locations
- Generate tasks for tracking when needed
- Follow Intent project structure and conventions

## Example Usage Patterns

### Basic Pattern
```

Task(
description="Short description of task",
prompt="Detailed instructions for the agent including context and requirements",
subagent_type="your-agent-name"
)

```

### Complex Workflow
[Describe how this agent fits into larger workflows]

## Quality Standards

Ensure your responses:
- [Quality standard 1]
- [Quality standard 2]
- [Output format requirements]
```

**Required YAML Fields:**

- `name`: Must match directory name
- `description`: One-line summary (used in agent listings)
- `tools`: Array of Claude Code tools this agent can access

**Available Tools:**

- `Bash`: Execute shell commands
- `Read`: Read files from filesystem
- `Write`: Create new files
- `Edit`: Modify existing files
- `Grep`: Search file contents
- `WebFetch`: Fetch web content
- `Glob`: Find files by pattern
- `LS`: List directory contents

### 3. Create Metadata File (`metadata.json`)

Create the metadata configuration:

```json
{
  "name": "your-agent-name",
  "version": "1.0.0",
  "description": "Detailed description of agent capabilities and use cases",
  "author": "Your Name or Organization",
  "tools": ["Bash", "Read", "Write", "Edit", "Grep"],
  "tags": ["domain", "framework", "specialty", "relevant-keywords"]
}
```

**Required Fields:**

- `name`: Must match directory name and agent.md frontmatter
- `version`: Semantic version (start with 1.0.0)
- `description`: Detailed explanation of capabilities
- `author`: Creator information
- `tools`: Must match tools list in agent.md
- `tags`: Keywords for discovery and categorization

### 4. Install the Agent

Install your custom agent to make it available in Claude Code:

```bash
intent agents install your-agent-name
```

This copies the agent to `~/.claude/agents/` where Claude Code can access it.

**Installation Options:**

- `intent agents install your-agent-name` - Install specific agent
- `intent agents install --force` - Skip confirmation prompts
- `intent agents install --all` - Install all available agents

### 5. Verify Installation

Check that your agent is properly installed:

```bash
# List all agents to see your new agent
intent agents list

# Show detailed information about your agent
intent agents show your-agent-name

# Check agent health and integrity
intent agents status
```

### 6. Test the Agent

Test your agent through Claude Code using the Task tool:

```
Task(
  description="Test custom agent",
  prompt="Perform a simple task to verify the agent is working correctly",
  subagent_type="your-agent-name"
)
```

## Example: Creating a Security Review Agent

Here's a complete example for a security-focused agent:

**Directory:** `intent/agents/security-reviewer/`

**agent.md:**

```markdown
---
name: security-reviewer
description: Security specialist for code review and vulnerability assessment
tools: Bash, Read, Write, Edit, Grep
---

You are a cybersecurity expert specializing in application security, code review, and vulnerability assessment.

## Your Expertise

You have deep knowledge in:

- OWASP Top 10 vulnerabilities and mitigations
- Secure coding practices across multiple languages
- Authentication and authorization patterns
- Data protection and encryption standards
- Security testing methodologies

## Your Role

When reviewing code or designs:

1. Identify potential security vulnerabilities
2. Suggest specific remediation strategies
3. Recommend security best practices
4. Assess compliance with security standards

## Security Review Checklist

Always evaluate:

- Input validation and sanitization
- Authentication and session management
- Authorization and access controls
- Data encryption and protection
- Error handling and information disclosure
- Dependency vulnerabilities

## Integration with Intent

- Document security findings in steel thread design docs
- Create security tasks for remediation
- Reference security requirements in steel threads
- Maintain security documentation in intent/docs/
```

**metadata.json:**

```json
{
  "name": "security-reviewer",
  "version": "1.0.0",
  "description": "Security specialist for comprehensive code review and vulnerability assessment with OWASP expertise",
  "author": "Security Team",
  "tools": ["Bash", "Read", "Write", "Edit", "Grep"],
  "tags": ["security", "owasp", "vulnerability", "code-review", "compliance"]
}
```

## Best Practices for Agent Creation

### System Prompt Design

1. **Be Specific**: Define clear expertise boundaries and capabilities
2. **Provide Context**: Explain when and how the agent should be used
3. **Include Examples**: Show typical usage patterns and workflows
4. **Set Quality Standards**: Define output expectations and quality criteria

### Tool Selection

1. **Minimal Necessary**: Only include tools the agent actually needs
2. **Consider Security**: Be cautious with Bash access for security-focused agents
3. **Match Capabilities**: Ensure tools align with agent's intended functionality

### Documentation Quality

1. **Clear Instructions**: Write for someone unfamiliar with your domain
2. **Complete Examples**: Provide full, working examples
3. **Integration Guidance**: Explain how agent fits into Intent workflows
4. **Maintenance Notes**: Include version history and update guidance

### Testing and Validation

1. **Functional Testing**: Verify all advertised capabilities work
2. **Integration Testing**: Test within actual Intent project workflows
3. **Documentation Testing**: Ensure examples and instructions are accurate
4. **Performance Testing**: Check response quality and relevance

## Troubleshooting

### Common Issues

**Agent Not Listed**

- Check directory structure matches `intent/agents/agent-name/`
- Verify `agent.md` and `metadata.json` exist
- Ensure JSON syntax is valid

**Installation Fails**

- Verify name consistency across directory, agent.md, and metadata.json
- Check YAML frontmatter syntax in agent.md
- Ensure tools list is valid

**Agent Doesn't Respond Properly**

- Review system prompt clarity and specificity
- Check tool permissions and availability
- Verify agent scope matches intended use cases

**Performance Issues**

- Simplify system prompt if too complex
- Reduce tool set to essential capabilities only
- Focus agent scope on specific domain

### Debugging Commands

```bash
# Check agent configuration
intent agents show your-agent-name

# Verify installation status
intent agents status --verbose

# Reinstall agent
intent agents install your-agent-name --force

# Check Intent configuration
intent doctor
```

## Updating Agents

To update an existing agent:

1. Modify `agent.md` and/or `metadata.json`
2. Update version number in `metadata.json`
3. Reinstall: `intent agents install your-agent-name --force`
4. Test updated functionality

## Sharing Agents

To share agents with others:

1. **Package Directory**: Include entire `intent/agents/agent-name/` directory
2. **Document Dependencies**: List any required tools or configurations
3. **Provide Examples**: Include usage examples and test cases
4. **Version Control**: Use semantic versioning for updates

## Advanced Features

### Custom Slash Commands

Agents can implement custom slash commands for specialized workflows:

```markdown
## Custom Commands

This agent supports these slash commands:

### /security-scan

Performs comprehensive security scan of specified files or directories.

Usage: `/security-scan path/to/code`

### /compliance-check

Evaluates code against specific compliance standards.

Usage: `/compliance-check --standard=SOC2 path/to/files`
```

### Multi-Agent Workflows

Design agents to work together in complex workflows:

```markdown
## Workflow Integration

This agent works well with:

- `intent` agent for project structure
- `code-reviewer` agent for general code quality
- `documentation` agent for security documentation
```

## References

- [Intent Agent System Documentation](../llm/llm_preamble.md)
- [Claude Code Sub-Agents](https://docs.anthropic.com/en/docs/claude-code/sub-agents)
- [Intent Commands Reference](../../README.md#commands)
- [Agent Examples](../../agents/)

---

**Need Help?**

- Run `intent help agents` for command reference
- Use `intent doctor` to check configuration
- Check existing agents in `agents/` directory for examples
