---
title: "LLM Collaboration with Intent"
date: "2025-07-08"
author: "Matthew Sinclair"
draft: false
word_count: 2274
---

# LLM Collaboration with Intent: Multiplying Development Capabilities

We've built a foundation of [captured intention](./0003-intent-capture-in-software-development.md) using [steel threads](./0002-the-steel-thread-methodology.md). Now let me show you exactly how this transforms your AI coding experience.

## The Difference is Night and Day

### Without Intent

```
You: "Help me add caching to the user service"
Claude: "Here's a simple Redis cache implementation..."
[Generic code that misses your actual needs]

You: "No, we need to handle traffic spikes"
Claude: "Oh, here's a different approach..."
[Still not right]

You: "Also, we have API rate limits"
Claude: "Ah, now I understand. Let me start over..."
[Finally getting closer after 3 attempts]
```

### With Intent

```
You: "I'm working on ST0042" [paste steel thread]
Claude: "I see you need caching for:
  - API rate limit: 100 req/min
  - Expected traffic: 10K req/s during sales
  - Constraint: Redis cluster already deployed
  
Here's a multi-layer cache with rate limiting, 
burst handling, and automatic failover..."
[Exactly what you need, first try]
```

The difference? Intent gives AI the context to understand your actual problem, not just your immediate request.

## Why AI Assistants Need Intent

### The Token Economy

Every conversation with an AI has a context limit. Without Intent, you waste tokens re-explaining:

```markdown
# Typical conversation without Intent:
Tokens used on explanation: 2,000
Tokens used on actual problem: 500
Total: 2,500 tokens, mostly wasted

# With Intent:
Tokens for steel thread: 500
Tokens for actual problem: 2,000
Total: 2,500 tokens, mostly productive
```

### Intent Keeps AI Focused

**Problem**: Give AI your entire codebase, it gets lost
**Solution**: Give AI one steel thread, it stays focused

```bash
# This overwhelms the AI:
$ find . -name "*.py" | xargs cat | wc -l
45,000 lines of code

# This focuses the AI:
$ intent st show ST0042
200 lines of structured context
```

### Real Example: Authentication Implementation

I used Intent to build authentication with Claude:

```markdown
# ST0042: Multi-Service Authentication

## Context
- 5 microservices need shared auth
- Cannot use sessions (stateless requirement)
- Must support enterprise SSO
- Peak load: 10K concurrent users

## Constraints  
- Existing users in PostgreSQL
- 15-minute token expiry (security audit)
- Zero-downtime migration required
```

Claude immediately suggested:

- JWT with refresh tokens (not sessions)
- Token relay pattern for microservices
- Gradual migration strategy
- Redis for token revocation

No back-and-forth. No wrong assumptions. Just solutions that fit MY constraints.

## The LLM Collaboration Challenge

Even powerful LLMs face fundamental challenges in software development collaboration:

### Context Window Constraints

LLMs have finite context windows. Dumping your entire codebase exceeds these limits and creates noise. The challenge: How do you provide enough context without overwhelming the model?

### Information Overload

More context isn't always better. LLMs can get lost in irrelevant details, missing the crucial information buried in thousands of lines of code. Quality beats quantity.

### The Stale Context Problem

Yesterday's context might mislead today's decisions. As code evolves, old assumptions become dangerous. Static documentation quickly becomes a liability rather than an asset.

### Project Structure Complexity

Explaining how different parts of your system interact requires more than showing code. LLMs need to understand relationships, dependencies, and architectural decisions.

### Session Continuity

Each new conversation starts fresh. Without systematic context management, you waste time re-explaining your project, and the LLM loses valuable understanding built in previous sessions.

## How Intent Is Designed for LLM Collaboration

Intent addresses each collaboration challenge through deliberate design choices:

### The "Preamble to Claude"

Our Technical Product Design starts with explicit instructions for LLMs:

```markdown
## Preamble to Claude

This document is a Technical Product Design (TPD) for the Intent (Intent) system. When processing this document, please understand:

1. This is a comprehensive technical specification...
2. The system is designed to facilitate collaboration between developers and LLMs...
```

This isn't just documentation – it's a handshake protocol between human intent and AI understanding.

### Structured Documentation That Fits LLM Thinking

Intent templates mirror how LLMs process information:

- **Clear hierarchies** that LLMs can navigate
- **Consistent patterns** that reduce parsing complexity
- **Explicit sections** for objectives, context, approach
- **Metadata frontmatter** for quick classification

### Just-in-Time Context Loading

Instead of overwhelming LLMs with everything, Intent enables focused context:

```bash
# Load specific steel thread context
$ cat intent/prj/st/ST0042.md

# Show current tasks for that thread
$ intent task list ST0042

# Check implementation status
$ intent status show ST0042
```

Each command provides exactly the context needed for the current task.

### The Information Flow Architecture

```
WIP (Current Focus)
 │
 ├──▶ Steel Threads (Intent & Strategy)
 │     │
 │     └──▶ Tasks (Granular Work)
 │
 └──▶ Journal (Historical Context)
```

This flow ensures LLMs always have:

1. Current focus (WIP)
2. Strategic context (Steel Threads)
3. Tactical details (Tasks)
4. Historical decisions (Journal)

## Context Management Strategies

Effective LLM collaboration requires strategic context management. Here's how Intent enables it:

### Start with WIP (Work In Progress)

The WIP document acts as a conversation starter:

```markdown
# Work in Progress

## Current Focus
Implementing authentication system (ST0042)
- Decided on JWT tokens over sessions
- Need to handle refresh token rotation
- Integrating with existing user service

## Blockers
- Unclear how to handle multi-device login
```

This immediately orients the LLM to your current state and challenges.

### Use Steel Threads as Context Containers

Each steel thread provides bounded context:

```bash
# Share a complete context unit
$ cat intent/prj/st/ST0042.md | pbcopy
# Now paste into LLM conversation
```

The LLM receives:

- Clear objectives
- Relevant constraints
- Design decisions
- Current progress

### Progressive Context Loading

Start minimal, add detail as needed:

1. **Initial**: "Working on ST0042 - Authentication System"
2. **If needed**: Share the steel thread document
3. **For specifics**: Show relevant task details
4. **For history**: Reference journal entries

This prevents context overload while ensuring completeness.

### Task Status as Progress Indicators

```bash
$ intent status show ST0042
Steel Thread: ST0042
Current Status: In Progress
Task Summary:
  Total Tasks: 8
  - Done: 5
  - In Progress: 1
  - Todo: 2
```

LLMs immediately understand what's complete and what needs attention.

## Templates and Structure that Enhance LLM Effectiveness

Intent templates aren't arbitrary – they're designed to match how LLMs process information.

### Why Structure Matters to LLMs

LLMs excel at pattern recognition. Consistent structure becomes a pattern they can leverage:

```markdown
---
status: In Progress
created: 20250308
---
# ST0042: Authentication System

## Objective
[LLMs immediately understand this is the goal]

## Context
[LLMs know to find background information here]

## Approach
[LLMs expect implementation strategy here]
```

The predictable structure reduces cognitive load and improves comprehension.

### Frontmatter Metadata

YAML frontmatter provides machine-readable context:

```yaml
---
status: In Progress       # LLM knows work is active
created: 20250308         # LLM understands timeline
completed:                # LLM sees this isn't done
author: Jane Smith        # LLM knows who to reference
dependencies: [ST0038]    # LLM understands relationships
---
```

This metadata helps LLMs make contextual decisions without parsing prose.

### Section Organisation for LLM Reasoning

Intent sections follow a logical flow that mirrors problem-solving:

1. **Objective**: What are we trying to achieve?
2. **Context**: Why does this matter?
3. **Approach**: How will we solve it?
4. **Tasks**: What specific work is needed?
5. **Implementation Notes**: What have we learned?
6. **Results**: What was the outcome?

This progression helps LLMs understand not just the current state but the journey.

### Consistent Formatting Patterns

Intent uses consistent markers that LLMs can recognise:

- `## Section Headers` for major divisions
- `- [ ] Task items` for work tracking  
- `` ```language `` for code blocks
- `**Bold**` for emphasis
- `[Links](./file.md)` for relationships

These patterns become navigational aids for LLM comprehension.

## The Documentation-Implementation Feedback Loop

Intent creates a virtuous cycle where documentation and implementation reinforce each other, with LLMs participating at every stage.

### Documentation Drives Implementation

```mermaid
Documentation → LLM Understanding → Better Suggestions → Quality Code
```

When you start with clear documentation:

1. LLMs understand the complete context
2. Suggestions align with documented intent
3. Generated code fits the design
4. Implementation matches expectations

### Implementation Updates Documentation

As you code, discoveries flow back:

```markdown
## Implementation Notes
[2024-03-15] Discovered rate limiting issue with auth tokens
[2024-03-16] Switched to sliding window approach
[2024-03-17] Added token bucket for burst capacity
```

LLMs learn from these updates, improving future suggestions.

### Real Example: The Feedback Loop in Action

**Initial Documentation**:

```markdown
## Approach
Implement simple cache with 1-hour TTL
```

**LLM Suggestion**: "Consider cache invalidation strategy for multi-region deployment"

**Updated Documentation**:

```markdown
## Approach
Implement cache with 1-hour TTL
- Use event-based invalidation for consistency
- Redis pub/sub for multi-region coordination
```

**Result**: Better implementation informed by LLM insight, captured in documentation.

### The Multiplier Effect

Each cycle improves both documentation and code:

- **Clearer intent** → Better LLM suggestions
- **Better suggestions** → Improved implementation  
- **Improved implementation** → Refined documentation
- **Refined documentation** → Even clearer intent

This isn't just about current development – it's about building a knowledge base that makes every future interaction more effective.

## Claude Code Sub-Agents: Taking Collaboration Further

Intent v2.1.0 includes integration with Claude Code's sub-agent system, revolutionizing AI-assisted development.

### The Problem with Session Amnesia

Every new Claude session typically means:

- Re-explaining your project structure
- Describing Intent and steel threads
- Teaching command syntax again
- Clarifying team conventions

It's like working with a brilliant colleague who has amnesia every morning.

### Intent Agent: Persistent Knowledge

With one command:

```bash
intent agents install intent
```

Claude permanently understands:

- Steel thread methodology
- All Intent commands and workflows
- Project organization patterns
- Best practices and conventions

### The Transformation in Practice

**Without Intent agent:**

```
You: "I need to add user notifications"
Claude: "I'll help you add notifications. What framework are you using?"
You: "We use Intent for project management..."
[5 minutes explaining]
Claude: "Where should I put the notification code?"
You: "First create a steel thread with intent st new..."
[More back and forth]
```

**With Intent agent:**

```
You: "I need to add user notifications"
Claude: "I'll help you create a steel thread for notifications:

    intent st new 'User Notification System'
    
This creates ST0044. Let's document the intent:
- What types of notifications (email, push, in-app)?
- Real-time requirements?
- User preferences and opt-out needs?

I'll help structure this properly and create Backlog tasks."
```

The conversation starts at a much higher level because Claude already knows the foundation.

### Custom Project Agents

Beyond the Intent agent, create project-specific agents:

```bash
# Your team's conventions
$ cat intent/agents/team-standards/agent.md
---
name: team-standards
---
You know our standards:
- API versioning: /api/v{n}/{resource}
- Error format: RFC 7807 Problem Details
- Auth: JWT in Authorization: Bearer {token}
- Testing: 80% coverage minimum
- PR process: feature/* branches, 2 approvals
```

Now every Claude session knows YOUR specific rules without repeated explanation.

### Specialized Global Agents

Intent v2.2.0 includes specialized agents for common development scenarios:

#### Elixir Agent

For Elixir developers, the elixir agent provides:

- Pure functional programming patterns and Usage Rules
- Ash and Phoenix framework expertise
- Idiomatic Elixir code review and refactoring
- Railway-oriented error handling patterns

```bash
intent agents install elixir
```

#### Socrates Agent - CTO Review Mode

The socrates agent facilitates technical decision-making through Socratic dialog:

```bash
intent agents install socrates
```

This agent creates structured conversations between two personas:

- **Socrates (CTO)**: Strategic thinker with 30+ years experience
- **Plato (Tech Lead)**: Implementation expert with deep technical knowledge

Perfect for:

- Architecture decisions (microservices vs monolith)
- Technology selection (build vs buy, framework choices)
- Complex refactoring strategies
- API design and integration planning

Example usage:

```
You: "I need to decide between PostgreSQL and DynamoDB"
Claude (with socrates agent): "I'll facilitate a CTO Review Mode dialog:

**Socrates (CTO):** What's driving this database decision? Are we 
optimizing for consistency or scale?

**Plato (Tech Lead):** We're expecting 1M users in year one with 
burst traffic during sales. The team has PostgreSQL expertise...

[Dialog continues exploring trade-offs, revealing hidden complexities]

**Recommendation:** PostgreSQL with read replicas and caching layer
- Leverages team expertise
- Handles expected scale with proven patterns
- Avoids NoSQL learning curve during critical growth phase"
```

The socrates agent transforms rubber-duck debugging into structured technical exploration, creating decision documentation that explains not just what was decided, but why.

### The Multiplier Effect

Intent agents don't just save time – they change what's possible:

1. **Deeper Conversations**: Skip basics, discuss architecture
2. **Consistent Practices**: AI always follows your standards
3. **Knowledge Preservation**: Team wisdom encoded in agents
4. **Onboarding Acceleration**: New devs get AI that knows your ways

## Future Opportunities for LLM Integration

We're just scratching the surface of what's possible when development methodologies embrace LLM collaboration.

### Automated Documentation Validation

LLMs could continuously validate documentation against implementation:

```bash
$ stp validate ST0042
Checking documentation-implementation alignment...
⚠ Implementation includes rate limiting not mentioned in approach
⚠ Task list shows 8 items but only 6 are documented
✓ All objectives have corresponding implementation
```

### LLM-Powered Steel Thread Creation

Imagine describing a feature and having an LLM draft the steel thread:

```
You: "We need to add data export functionality for compliance"

LLM: "I'll create a steel thread for this. Based on your project:
- Objective: Enable GDPR-compliant data export
- Context: Legal requirement, 30-day deadline
- Approach: Queue-based async processing
- Tasks: [generates task breakdown]"
```

### Intelligent Context Selection

Future Intent could automatically select relevant context:

```bash
$ stp context "working on authentication"
Relevant context loaded:
- ST0042: Authentication System (current)
- ST0038: User Service (dependency)
- Journal: 2024-03-15 auth decisions
- Related tasks: task-45, task-46
```

### The MCP Revolution

Anthropic's Machine Control Protocol opens new possibilities:

- LLMs directly executing Intent commands
- Autonomous steel thread management
- Real-time documentation updates
- Integrated development environments

## Practical LLM Collaboration Workflow

### Morning Standup with Your AI

```bash
# Start your day by loading context
$ cat intent/wip.md
Current focus: Implementing auth refresh tokens (ST0042)
Blocker: Unsure about multi-device logout

$ intent st show ST0042 | pbcopy
# Paste to Claude/ChatGPT
```

**You**: "I'm stuck on multi-device logout with our JWT strategy"

**Claude**: "I see from ST0042 you're using 15-min access tokens with 7-day refresh tokens. For multi-device logout with your constraints:

1. Add a token family ID to track related tokens
2. Store revoked families in Redis (you already have it deployed)
3. Check revocation on token refresh, not every request (performance)
Here's the implementation..."

### Discovering Your Own Code

Six months later:

```bash
$ intent st list | grep cache
ST0015  [Completed]  Implement multi-layer cache for API protection
ST0023  [Completed]  Add cache warming for user profiles
ST0031  [In Progress] Migrate cache to Redis cluster

$ intent st show ST0015
# Aha! That's why the cache is so complex
# Shows: API limits, traffic patterns, decisions made
```

### Team Knowledge Sharing

New developer joins:

```bash
# Instead of: "Let me explain our auth system..."
$ intent st show ST0042 > auth-context.md
$ echo "Start here. This explains everything." 

# They read:
- Why JWT over sessions (microservices)
- Why 15-min expiry (security audit)  
- Why Redis for revocation (already in stack)
- Implementation gotchas discovered
```

The new developer is productive in hours, not weeks.

## Transforming Development Through Collaboration

Intent doesn't just make LLM collaboration possible – it makes it powerful. By providing structure, context, and clear intention, Intent transforms LLMs from code generators into true development partners.

The future of software development isn't human or AI – it's human and AI, working together with shared understanding. Intent provides the foundation for that collaboration.

Ready to put this into practice? Our next post will guide you through setting up Intent in your own projects and creating your first intention-aware, LLM-collaborative development workflow.

[Continue to: Getting Started with Intent →](./0005-getting-started-with-intent.md)
