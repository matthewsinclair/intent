---
title: "Intent Capture in Software Development"
date: "2025-07-08"
author: "Matthew Sinclair"
draft: false
word_count: 2156
---

# Intent Capture in Software Development: Bridging the Gap

We've explored [why intention matters](./0000-motivation-for-intent.md) and how [steel threads](./0002-the-steel-thread-methodology.md) provide a framework for preserving it. Now we turn to the critical skill that makes it all work: intent capture. How do we extract, document, and preserve the intentions that drive great software?

Intent capture isn't just about writing better documentation – it's about creating a shared understanding that survives the journey from conception to production and beyond. Today, we'll explore practical techniques for capturing intent effectively, whether you're working solo, with a team, or alongside AI assistants.

## Applying Intention Principles in Practice

The journey from understanding why intention matters to actually capturing it requires practical techniques and deliberate practice. Intent capture transforms abstract understanding into concrete documentation that serves both current and future needs.

Think of intent capture as archaeology in reverse. Instead of future developers excavating your code to understand what you were thinking, you preserve those thoughts as you have them. The key shift: recognise that your current clarity is a perishable resource.

Let's explore how to capture that clarity before it fades.

## The Intent Problem in Software Development

### What is "Intent" in Software Development?

Intent encompasses the full context behind technical decisions:

- **Business Goals**: What problem does this solve for users?
- **Technical Rationale**: Why this approach over alternatives?
- **Constraints**: What limitations shaped the solution?
- **Assumptions**: What did we believe to be true?
- **Trade-offs**: What did we optimise for?

### Why Intent Gets Lost

Intent erosion happens through predictable patterns:

1. **Time Decay**: "I'll remember why I did this" (you won't)
2. **Context Switching**: Moving between projects erases mental state
3. **Team Changes**: Original authors leave, taking knowledge with them
4. **Documentation Lag**: "I'll document it after it works" (too late)
5. **Implicit Knowledge**: Assuming others share your context

### The Compounding Cost

Lost intent creates cascading problems:

```
Lost Intent → Confusion → Bad Decisions → Technical Debt → More Confusion
```

A developer encounters code without clear intent. They make assumptions. Those assumptions lead to changes that violate the original design. The codebase becomes harder to understand. The cycle accelerates.

### How Traditional Documentation Falls Short

Typical documentation captures the "what" but misses the "why":

**Traditional**: "This class manages user sessions using Redis."

**Intent-Aware**: "This class manages user sessions using Redis because we need horizontal scalability for our multi-region deployment. We chose Redis over database storage to keep session queries off our primary database, which was becoming a bottleneck. The 15-minute timeout balances security with user convenience based on our threat model."

## Why Intent Matters

Consider this scenario: You join a team maintaining a five-year-old codebase. You find a complex caching layer that seems over-engineered. Do you simplify it? Without understanding the original intent, you're gambling.

Maybe that "over-engineered" cache prevents database meltdowns during traffic spikes. Maybe it works around a third-party API's rate limits. Maybe it's genuinely unnecessary complexity. Without intent documentation, you can't know.

### The Telephone Game Effect

In long-lived codebases, understanding degrades like a game of telephone:

- Original developer: "Cache this because the API limits us to 100 calls/minute"
- Six months later: "There's some caching here for performance"
- Two years later: "Not sure why this is cached, probably outdated"
- Today: Cache removed, API rate limits hit, production down

### Intent as Future Context

Well-captured intent provides context for future decisions:

- **Refactoring**: "Can I change this safely?"
- **Debugging**: "Why does it work this way?"
- **Enhancement**: "Will this change align with the design?"
- **Evaluation**: "Is this still serving its purpose?"

### The What vs. Why Distinction

**What** (Code shows this):

```python
cache_timeout = 3600  # 1 hour
```

**Why** (Intent captures this):

```python
# Cache for 1 hour because:
# - User profiles change infrequently
# - Reducing database load is critical at our scale
# - 1 hour balances freshness with performance
# - Shorter timeouts caused 3x database load in testing
cache_timeout = 3600
```

## How Intent Addresses the Intent Problem

Intent attacks intent loss through systematic capture and preservation. Rather than hoping developers document intent, Intent makes it part of the natural workflow.

### Documentation Alongside Code

Intent keeps documentation in the repository, versioned with the code:

```
project/
├── src/           # Implementation
└── intent/
    ├── prj/st/    # Steel threads (intent)
    ├── eng/tpd/   # Technical design (rationale)
    └── usr/       # User perspective (purpose)
```

When code changes, documentation updates travel in the same commit. Intent and implementation stay synchronised.

### Steel Threads as Intent Containers

Each steel thread captures multi-level intent:

1. **Objective**: What we're trying to achieve
2. **Context**: Why this matters now
3. **Approach**: How we plan to solve it
4. **Decisions**: What we learned and chose
5. **Results**: What actually happened

### Structured Templates That Prompt

Intent templates ask the right questions at the right time:

```markdown
## Objective
[Forces you to articulate the goal clearly]

## Context  
[Prompts for the "why now" and background]

## Approach
[Captures your intended solution strategy]
```

The structure guides without constraining. You can't skip intent because the template won't let you.

### Living Intent Records

Intent maintains intent at multiple levels:

- **Technical Product Design**: System-wide architectural intent
- **Steel Threads**: Feature-level implementation intent  
- **Journal**: Daily decisions and discoveries
- **WIP**: Current thinking and active problems

Each document serves a different temporal scope, from permanent architectural decisions to ephemeral daily thoughts.

## Intent Capture Best Practices

### 1. Document Decisions, Not Just Outcomes

❌ **Poor**: "Using PostgreSQL for data storage"

✓ **Better**: "Chose PostgreSQL over MongoDB because:

- Need ACID compliance for financial data
- Complex relationships require joins
- Team expertise is stronger with SQL
- MongoDB's eventual consistency doesn't fit our use case"

### 2. Capture Alternatives Considered

Future developers need to know what you rejected and why:

```markdown
## Approach
Implementing rate limiting using token bucket algorithm.

### Alternatives Considered:
1. **Fixed window**: Rejected - allows burst attacks at window boundaries
2. **Sliding window**: Rejected - memory intensive at our scale
3. **Token bucket**: Selected - smooth rate limiting with burst capacity
```

### 3. Link Intent to Requirements

Connect technical decisions to business needs:

```markdown
## Context
Compliance requires us to retain user data for 7 years (REQ-2.3.1).
This drives our archive strategy and database partitioning scheme.
```

### 4. Update Intent as Understanding Evolves

Intent isn't static. As you learn, update the documentation:

```markdown
## Implementation Notes
[2024-03-01] Initial approach used simple caching
[2024-03-15] Discovered cache invalidation issues with multi-region
[2024-03-20] Moved to event-based invalidation - more complex but reliable
```

### 5. Use Progressive Detail

Capture intent at multiple levels:

- **Thread level**: Overall objective and approach
- **Task level**: Specific implementation choices
- **Code level**: Inline comments for non-obvious decisions

## Intent Capture and LLMs

LLMs excel at pattern matching but lack true understanding. Without explicit intent, they make plausible-sounding suggestions that miss the mark. With clear intent, they become powerful collaborators.

### How LLMs Struggle Without Intent

**Without Intent**:

```
Developer: "Optimise this database query"
LLM: [Suggests adding indexes, rewriting joins]
Result: Query is faster but breaks business logic that depended on row order
```

**With Intent**:

```
Developer: "Optimise this query that generates billing reports. Row order must be preserved for legal compliance. Current performance: 45s for monthly reports."
LLM: [Suggests partitioning strategy that maintains order while improving performance]
Result: Query runs in 3s while maintaining compliance
```

### Using LLMs to Refine Intent

LLMs can help clarify your own thinking:

```
You: "I need to add caching to this API endpoint"

LLM: "Let me help clarify the intent. Can you tell me:
- What problem is the lack of caching causing?
- What are the data freshness requirements?
- Who are the users of this endpoint?
- What's the current performance baseline?"

You: "Actually, the real problem is database connection exhaustion during peak hours..."
```

The LLM's questions help surface the true intent.

### LLMs as Documentation Co-pilots

LLMs can transform rough notes into structured intent:

**Your notes**: "need to fix login slow, maybe redis session?"

**LLM-assisted documentation**:

```markdown
## Objective
Reduce login latency from current 3-5 seconds to under 500ms

## Context  
User complaints about slow login are our #1 support issue. 
Profiling shows session validation queries taking 2-4 seconds.

## Approach
Implement Redis-based session cache to avoid database queries
for active sessions.
```

## Case Study: Before and After Intent Capture

Let's examine a real feature implemented both ways to see the dramatic difference intent capture makes.

### The Feature: API Rate Limiting

#### Before: Without Intent Capture

```python
# rate_limiter.py
class RateLimiter:
    def __init__(self):
        self.requests = {}
        self.limit = 100
        self.window = 3600
```

**Six months later**: "Why is the limit 100? Can we change it? What breaks if we do?"

#### After: With Intent-Style Intent Capture

```markdown
# ST0042: API Rate Limiting Implementation

## Objective
Protect our API from abuse while ensuring legitimate users maintain access

## Context
- Experiencing DoS attacks overwhelming our infrastructure
- Legitimate users making 20-50 requests/hour on average
- Need solution that scales horizontally across multiple servers
- Must not impact user experience for normal usage patterns

## Approach
Token bucket algorithm with Redis backend:
- 100 requests/hour limit (2x normal peak usage)
- Tokens refill continuously (smooth experience)
- Redis enables sharing state across servers
```

```python
# rate_limiter.py
class RateLimiter:
    def __init__(self):
        # Limits based on usage analysis (ST0042)
        # 99th percentile legitimate usage: 47 req/hour
        # Limit set to 2x that for safety margin
        self.limit = 100  
        
        # 1-hour window matches our abuse detection cycle
        # Shorter windows allowed attackers to burst
        self.window = 3600
```

### The Difference in Maintenance

**Scenario**: Business wants to offer a premium tier with higher limits

**Without Intent**:

- Developer guesses why 100 was chosen
- Increases to 500 "to be safe"
- Accidentally enables abuse patterns
- Infrastructure costs spike

**With Intent**:

- Developer understands the analysis behind 100
- Knows premium users also fit the usage pattern
- Implements tier-based limiting: 100 (free), 200 (premium)
- Maintains protection while enabling business goals

### Intent Flow Diagram

```
┌──────────────┐     ┌───────────────┐     ┌──────────────┐
│   Business   │     │   Technical   │     │    Future    │
│   Problem    │────▶│   Solution    │────▶│  Decisions   │
└──────────────┘     └───────────────┘     └──────────────┘
       │                     │                     │
       │                     │                     │
┌──────▼─────────────────────▼─────────────────────▼────┐
│            Captured Intent Documentation              │
│    (Context + Decisions + Rationale + Trade-offs)     │
└───────────────────────────────────────────────────────┘
```

### Measuring the ROI

Teams using Intent-style intent capture report:

- **50% reduction** in time spent understanding existing code
- **75% fewer** "why was this done?" meetings
- **90% faster** onboarding for new developers
- **Eliminated** rework from misunderstood requirements

## Making Intent Capture a Habit

Intent capture isn't a one-time activity – it's a continuous practice. Start small:

1. **Next PR**: Add a "Why" section to your description
2. **Next function**: Include a comment explaining the approach
3. **Next meeting**: Document decisions, not just action items
4. **Next debug session**: Write down what you learned

The goal isn't perfect documentation. The goal is preserving enough context that future-you (or your teammates, or an LLM) can make informed decisions.

## Intent Capture Transforms Development

When intent becomes explicit, development transforms from archaeology to architecture. Instead of excavating meaning from cryptic code, developers build on a foundation of clear purpose. LLMs shift from guessing your needs to understanding your goals. Teams move from confusion to clarity.

In our next post, we'll explore how this foundation of captured intent enables unprecedented collaboration with LLMs, turning AI assistants from code generators into true development partners.

[Continue to: LLM Collaboration with Intent →](./0004-llm-collaboration-with-intent.md)
