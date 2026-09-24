---
verblock: "24 Sep 2026:v1.3: synced to Intent 3.2.1 as built (3.2.1 doc audit)"
intent_version: 3.2.1
---

# Creating Custom Intent Agents

This guide provides step-by-step instructions for creating custom Intent agents that integrate with Claude Code's sub-agent system.

## Overview

Intent agents are specialized AI assistants with domain-specific knowledge and focused expertise. They extend Claude's capabilities by providing:

- Dedicated context windows separate from main conversation
- Specialized system prompts and knowledge
- Focused tool access appropriate to their domain
- Comprehensive results for specific tasks

## Canon vs extension subagents

Subagents ship in canon, in `intent/plugins/claude/subagents/<name>/` of the Intent install. Examples: `intent`, `socrates`, `diogenes`, the `critic-<lang>` family. The extension layout `~/.local/share/intent/ext/<name>/subagents/` is declared and not built: nothing in this build reads it (see `intent/docs/writing-extensions.md`).

The choice between them is one question: **is this useful to every Intent user, or only to some?**

- Useful to every Intent user → canon. The remainder of this guide applies.
- Useful only to some users → keep it outside canon in your own `~/.claude/agents/`, where `intent claude subagents list` reports it as `unlisted`; Intent's extension route is declared and not built (`intent/docs/writing-extensions.md`).

## Prerequisites

- Intent v3 installed (`intent --version` reports it)
- Claude Code CLI installed and configured
- Basic understanding of YAML frontmatter and JSON

## Agent Structure

Each Intent canon subagent consists of:

- **Directory**: `intent/plugins/claude/subagents/<name>/`
- **Agent Definition**: `agent.md` with YAML frontmatter and system prompt
- **Metadata** (optional): `metadata.json`. Intent does not read it: only `agent.md` makes a directory a subagent, and install copies `agent.md` alone to `~/.claude/agents/<name>.md`.

## Step-by-Step Creation Process (Canon Subagent)

### 1. Create Agent Directory

Create a new directory under `intent/plugins/claude/subagents/` for your agent:

```bash
mkdir -p intent/plugins/claude/subagents/your-agent-name/
cd intent/plugins/claude/subagents/your-agent-name/
```

`intent claude subagents install` reads canon from the Intent INSTALL -- the ancestor of the running `intent` binary that holds `lib/templates/` -- at `<install>/intent/plugins/claude/subagents/`, not from the current project; create the directory there (for a source checkout, the checkout root).

**Naming Convention:**

- Use lowercase with hyphens (eg `security-reviewer`, `api-designer`)
- Be descriptive but concise
- Avoid spaces or special characters

### 2. Create Agent Definition (`agent.md`)

Create the main agent file with YAML frontmatter and system prompt:

````markdown
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
````

**Frontmatter** (Claude Code reads it; Intent does not -- `intent claude subagents install` copies `agent.md` as it is):

- `name` (required): the id `Task(subagent_type=...)` uses. Keep it equal to the directory name, which is the name `intent claude subagents` knows the agent by; nothing checks that the two agree.
- `description` (required): when Claude should delegate to this agent.
- `tools` (optional): a comma-separated list of Claude Code tools, eg `Bash, Read, Grep`. Omitted, the agent inherits every tool available to subagents.

**Available Tools:**

- `Bash`: Execute shell commands
- `Read`: Read files from filesystem
- `Write`: Create new files
- `Edit`: Modify existing files
- `Grep`: Search file contents
- `WebFetch`: Fetch web content
- `Glob`: Find files by pattern

Claude Code's full tool list: https://code.claude.com/docs/en/sub-agents.

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

**Conventional fields** (Intent does not read `metadata.json`; the canon subagents that carry one use these):

- `name`: Must match directory name and agent.md frontmatter
- `version`: Semantic version (start with 1.0.0)
- `description`: Detailed explanation of capabilities
- `author`: Creator information
- `tools`: Must match tools list in agent.md
- `tags`: Keywords for discovery and categorization

### 4. Install the Agent

Install your custom agent to make it available in Claude Code:

```bash
intent claude subagents install your-agent-name
```

This copies `agent.md` to `~/.claude/agents/your-agent-name.md` and records the install in `~/.local/share/intent/subagents/installed-subagents.v3.json`.

**Installation Options:**

- `intent claude subagents install your-agent-name` - Install one or more by name
- `intent claude subagents install --all` - Every subagent this install carries
- `-f, --force` - Reinstall a subagent that is already installed, overwriting a copy changed locally and reporting the checksum of what was discarded

### 5. Verify Installation

Check that your agent is properly installed:

```bash
intent claude subagents list            # name, installed state, provenance; -v adds the source path
intent claude subagents show your-agent-name   # prints its agent.md
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

**Directory:** `intent/plugins/claude/subagents/security-reviewer/`

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

- Check directory structure matches `intent/plugins/claude/subagents/<agent-name>/` in the Intent install
- Verify `agent.md` exists: it is the file that makes the directory a subagent (`metadata.json` is optional and not read)

**Installation Fails**

- `no source for this name in this install`: `agent.md` is not at `<install>/intent/plugins/claude/subagents/<name>/`, where `<install>` is the tree the running `intent` binary belongs to (a source checkout's root).
- `already installed`: add `--force` to overwrite it (the checksum of a locally changed copy is reported).
- A name may use only letters, digits, `-` and `_`.

Frontmatter mistakes do not fail the install -- Intent copies `agent.md` without reading it; Claude Code reports them when it loads the agent.

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
intent claude subagents show your-agent-name

# Verify installation status
intent claude subagents list -v

# Reinstall agent
intent claude subagents install your-agent-name --force

# Check Intent configuration
intent doctor
```

## Updating Agents

To update an existing agent:

1. Modify `agent.md` and/or `metadata.json`
2. Update version number in `metadata.json`
3. Reinstall: `intent claude subagents sync` brings every installed subagent up to its canon copy and holds one you edited locally (`--dry-run` previews; `--force` takes the canon copy and reports the checksum of what it discarded). `intent claude subagents install your-agent-name --force` overwrites a single agent outright.
4. Test updated functionality

## Sharing Agents

To share agents with others:

1. **Package Directory**: Include the entire `intent/plugins/claude/subagents/<agent-name>/` directory
2. **Document Dependencies**: List any required tools or configurations
3. **Provide Examples**: Include usage examples and test cases
4. **Version Control**: Use semantic versioning for updates

## Advanced Features

### Slash Commands

A subagent cannot define a slash command: Claude Code reads only its frontmatter and system prompt. A workflow you want to invoke as `/name` is a skill -- `intent/plugins/claude/skills/<slug>/SKILL.md`, installed with `intent claude skills install <slug>`.

### Multi-Agent Workflows

Design agents to work together in complex workflows:

```markdown
## Workflow Integration

This agent works well with:

- `intent` agent for project structure
- `critic-<lang>` for rule-library review
- `socrates` for architectural decisions
```

## References

- [Claude Code Sub-Agents](https://code.claude.com/docs/en/sub-agents)
- [Intent Commands Reference](../../README.md#commands)
- [Agent Examples](../plugins/claude/subagents/)

---

**Need Help?**

- Run `intent claude subagents --help` for command reference
- Use `intent doctor` to check configuration
- Check existing agents in `intent/plugins/claude/subagents/` for examples
