---
title: "Intent Agents: Supercharging Claude Code Collaboration"
date: "2025-07-27"
author: "Matthew Sinclair"
draft: false
word_count: 1847
---

# Intent Agents: Supercharging Claude Code Collaboration

We've built a system that [captures intention](./0003-intent-capture-in-software-development.md) and enables [powerful LLM collaboration](./0004-llm-collaboration-with-intent.md). Now, Intent v2.1.0 takes this to the next level with Claude Code sub-agent integration and proper initialization.

## The Context Problem

Every time you start a new Claude session, you face the same ritual:

```
You: "Help me add a caching layer"
Claude: "I'll help you add caching. What's your project structure?"
You: "I use Intent with steel threads for organization..."
Claude: "Can you explain what steel threads are?"
You: "They're self-contained units of work..."
[5 minutes of explanation later]
Claude: "OK, so should I create a new file for the cache?"
You: "No, first run 'intent st new'..."
```

It's like having a brilliant colleague with amnesia. Every. Single. Session.

## Enter Intent Agents

With Intent v2.1.0, two simple commands change everything:

```bash
$ intent agents init  # Initialize agent configuration
$ intent agents install intent  # Install the Intent agent
```

Now Claude instantly knows:
- What steel threads are and how to use them
- All Intent commands and their purposes
- Your project organization patterns
- Backlog.md integration
- Best practices for Intent development

## The Transformation

### Before Intent Agent

```
You: "I need to add user authentication"
Claude: "I'll help you create an authentication system. What framework are you using?"
You: "The project uses Intent for management..."
Claude: "What's Intent? How should I structure the authentication code?"
[Several minutes of back-and-forth]
Claude: "Should I create a new module in src/auth?"
You: "No, we need to create a steel thread first..."
[More explanation needed]
```

### With Intent Agent

```
You: "I need to add user authentication"
Claude: "I'll help you create a steel thread for authentication:

    intent st new 'User Authentication System'

This creates ST0042. Now let's document the intent. Based on Intent best practices:

1. What authentication method? (JWT, sessions, OAuth?)
2. What are your security requirements?
3. Do you need multi-device support?

Once we clarify the intent, I'll help you:
- Break this into Backlog tasks
- Design the implementation approach
- Maintain proper Intent documentation throughout

Would you like me to also check for related steel threads first?"
```

The conversation starts at an architectural level, not a remedial one.

## How Intent Agents Work

### Architecture

Intent agents are Claude Code sub-agents - specialized AI assistants with focused knowledge:

```
~/.claude/agents/
├── intent.md      # Intent methodology expert
└── elixir.md      # Elixir patterns expert (bonus!)
```

### The Intent Agent

When installed, the Intent agent provides Claude with:

```markdown
---
name: intent
description: Intent-aware development assistant
tools: Bash, Read, Write, Edit, Grep
---

You are an Intent-aware development assistant specialized in:
- Steel thread methodology
- Intent project management
- Backlog task integration
- Structured documentation

## Key Commands
- intent st new "Title" - Create steel thread
- intent bl task new ST#### "Task" - Create linked task
- intent agents status - Check agent health
[... comprehensive Intent knowledge ...]
```

### Installation and Management

```bash
# See what's available
$ intent agents list
Available Agents:
  intent - Intent-aware development assistant [NOT INSTALLED]
  elixir - Elixir code doctor with Usage Rules [NOT INSTALLED]

# Install the Intent agent
$ intent agents install intent
Installing agent: intent
Installed successfully

# Install all available agents
$ intent agents install --all

# Check agent health
$ intent agents status
Checking agent status...
intent    [OK]
elixir    [OK]

# Keep agents updated
$ intent agents sync
Syncing installed agents...
intent    [UP TO DATE]
```

## Real-World Impact

### Scenario 1: New Feature Development

**Without agent:**
- 10 minutes explaining Intent
- 5 minutes clarifying commands
- Back-and-forth on structure
- Finally start actual work

**With agent:**
- 0 minutes on Intent explanation
- Immediate strategic discussion
- Claude suggests proper structure
- Straight to implementation

### Scenario 2: Code Review

**Without agent:**
```
You: "Review this steel thread implementation"
Claude: "What's a steel thread? What should I look for?"
```

**With agent:**
```
You: "Review this steel thread implementation"
Claude: "I'll review ST0043 against Intent best practices:
- ✓ Clear objective documented
- ✓ Context provides business rationale
- ⚠ Implementation notes could use more detail
- ✓ Tasks properly linked to Backlog
- Suggestion: Add decision rationale for choosing PostgreSQL"
```

### Scenario 3: Onboarding

New developer joins your Intent-based project:

```bash
# They install Intent and the agent
$ intent agents install intent

# Now their Claude understands your entire methodology
# No senior dev time needed for Intent training
# They're productive immediately
```

## Custom Project Agents

Beyond the built-in Intent agent, create project-specific agents:

```bash
$ mkdir -p intent/agents/myproject
$ cat > intent/agents/myproject/agent.md << 'EOF'
---
name: myproject
description: Project-specific conventions
tools: Bash, Read, Write
---

You understand our project-specific conventions:

## API Standards
- All endpoints: /api/v2/{resource}
- Authentication: Bearer tokens
- Responses: JSend format
- Errors: RFC 7807 Problem Details

## Testing Requirements
- Minimum 80% coverage
- Integration tests for all endpoints
- Load tests for critical paths

## Git Workflow
- Feature branches: feature/JIRA-123-description
- Commits: conventional commits format
- PRs: require 2 approvals
EOF

$ intent agents install myproject
```

Now Claude knows YOUR specific rules too.

## The Compound Effect

Intent agents don't just save time – they fundamentally change what's possible:

### 1. Higher-Level Conversations
Skip the basics, discuss architecture and design decisions immediately.

### 2. Consistent Practices
Every Claude session follows the same patterns, maintaining code quality.

### 3. Knowledge Preservation
Team conventions are encoded, not just documented.

### 4. Accelerated Onboarding
New team members get AI that already knows your ways.

### 5. Evolution Support
As Intent evolves, update the agent – all future sessions improve.

## Implementation Details

### Manifest-Based Tracking

Intent tracks installed agents with checksums:

```json
{
  "installed": [{
    "name": "intent",
    "source": "global",
    "checksum": "sha256:abc123...",
    "installed_at": "2025-07-27T10:00:00Z"
  }]
}
```

### Modification Protection

```bash
$ intent agents status
Checking agent status...
intent    [MODIFIED] - Local changes detected

$ intent agents sync
Warning: Agent has been modified locally
Overwrite local changes? [y/N]
```

### Integration Points

- **intent init**: Detects Claude, offers agent installation
- **intent doctor**: Checks agent health
- **intent upgrade**: Preserves agents during migration

## Getting Started

### For New Projects

```bash
$ intent init "My Project"
Claude Code detected!
Would you like to install the Intent sub-agent? [Y/n] y
Intent agent installed successfully!
```

### For Existing Projects

```bash
$ intent agents install intent
$ intent agents status
```

### Daily Workflow

1. Agents are installed once, work forever
2. Claude automatically loads agent knowledge
3. You focus on building, not explaining

## The Future

This is just the beginning. Imagine:

- **Language-specific agents**: Python, Go, Rust experts
- **Framework agents**: React, Django, Rails specialists
- **Domain agents**: Financial, healthcare, gaming knowledge
- **Team agents**: Your specific architectural decisions

Each agent layers additional expertise while maintaining Intent's structured approach.

## Start Today

If you're using Intent and Claude Code:

```bash
$ intent agents install intent
```

That's it. Your next Claude session will be transformed.

If you're not using Intent yet, [get started here](./0005-getting-started-with-intent.md).

## Conclusion

Intent agents solve the context problem permanently. No more explaining your methodology every session. No more reminding Claude about your commands. No more inconsistent suggestions.

Just intelligent, context-aware assistance that understands your project as well as you do.

The future of development isn't just AI-assisted – it's AI that truly understands your intent.

[Back to Intent Blog Index](./README.md)