---
title: "Introduction to Intent"
date: "2025-07-08"
author: "Matthew Sinclair"
draft: false
word_count: 1623
---

# Introduction to Intent: A Better Way to Build Software

If you've ever lost track of why a piece of code exists, struggled to onboard a new team member, or watched an LLM confidently solve the wrong problem, you understand the cost of lost intention in software development.

## See the Difference

**Without Intent:**

```python
# cache.py
def get_user_profile(user_id):
    # Check cache first
    cached = redis.get(f"user:{user_id}")
    if cached:
        return json.loads(cached)

    # Complex caching logic with multiple layers...
    # 200 lines of sophisticated caching code
```

6 months later: "Why is this cache so complex? Let's simplify it!"

**With Intent:**

```bash
$ intent st show ST0015
# Reveals: "Multi-layer cache because:
#   - API rate limits: 100 requests/minute
#   - Black Friday traffic: 10,000 requests/second
#   - Customer requirement: <50ms response time"
```

Now you know: That "complex" cache is saving your business!

In our [previous post](./0000-motivation-for-intent.md), we explored why capturing and preserving intention is crucial for modern development, especially when collaborating with AI. Today, I'll show you exactly how Intent solves this problem with a practical, lightweight system that enhances your existing workflow.

## Building on the Intention Foundation

We established that the fundamental challenge in modern development isn't just building software – it's ensuring that what we build aligns with why we're building it. This challenge intensifies when working with LLMs, which excel at pattern matching but lack understanding of underlying purpose.

Intent addresses this challenge by making intention explicit and structural. Rather than hoping developers will document the "why" or expecting LLMs to infer our goals, Intent builds intention capture into the development workflow itself.

The shift from theoretical understanding to practical implementation happens through three key innovations:

1. **Steel threads** that encapsulate both intent and implementation
2. **Structured templates** that prompt for intention at every stage
3. **Integration with task management** that maintains the intent-to-execution link

## What is Intent?

Intent is a lightweight methodology that captures the "why" behind your code. Here's how it works in practice:

### Try This Example

```bash
# 1. Start a new feature
$ intent st new "Add password reset functionality"
Created: ST0023

# 2. Capture the REAL requirements (not just "add password reset")
$ intent st edit ST0023
```

In your editor, document the actual constraints:

```markdown
## Objective

Implement secure password reset that prevents account takeover

## Context

- Recent security audit flagged email-based reset as vulnerable
- 15% of support tickets are password-related
- Must comply with SOC2 requirements
- Cannot break existing mobile app (v2.3.x)

## Approach

- Time-limited tokens (15 minutes)
- Rate limiting (3 attempts per hour)
- Multi-factor verification for high-value accounts
- Backward compatible API endpoints
```

### Now Share with Your AI Assistant

```bash
$ intent st show ST0023 | pbcopy
# Paste into Claude/ChatGPT/Copilot
```

The AI immediately understands:

- ❌ Not just "implement password reset"
- ✅ Security requirements, compliance needs, compatibility constraints
- ✅ Suggests appropriate security patterns
- ✅ Knows to maintain backward compatibility

### The Three Components

**Shell Scripts**: Simple automation

- `intent st new` - Create steel threads
- `intent st show` - View intentions
- `intent bl` - Manage tasks

**Markdown Templates**: Structured capture

- Forces "why" before "what"
- Consistent format for AI parsing
- Human-readable documentation

**Task Tracking**: Execution management

- Break threads into concrete tasks
- Track progress visually
- Maintain thread-to-task linkage

The magic: Your future self (and AI) always knows WHY code exists.

## Core Principles of Intent

Intent is built on eight core principles that guide its design and implementation:

### 1. Documentation as a First-Class Citizen

In Intent, documentation isn't something you do after coding – it's an integral part of the development process. Every steel thread starts with documentation that captures intention, and this documentation evolves alongside the code. When documentation drives development, both humans and LLMs have the context they need to make good decisions.

### 2. Intent Capture Throughout the Lifecycle

Intention isn't just captured at the beginning – it's maintained and referenced throughout development. From initial conception through implementation to future maintenance, the "why" remains visible and relevant. This creates a traceable lineage from business need to technical implementation.

### 3. Incremental Development Through Steel Threads

Rather than tackling entire features or epics, Intent encourages breaking work into steel threads – complete, minimal paths through your system. Each thread can be understood, implemented, and validated independently, making development more manageable and progress more visible.

### 4. Task Tracking Linked to Steel Threads

While steel threads capture the big picture intention, individual tasks track the detailed work. Intent's integration with Backlog.md creates a two-level system: strategic intent at the thread level, tactical execution at the task level. This separation keeps both perspectives clear and connected.

### 5. Process-Agnostic Compatibility

Intent doesn't dictate how you should develop software. Whether you're using Scrum, Kanban, or any other methodology, Intent layers intention-awareness on top. It's designed to enhance, not replace, your existing workflow.

### 6. Lightweight Enhancement

The entire Intent system consists of simple shell scripts and markdown templates. No complex tools to learn, no vendor lock-in, no heavyweight processes. You can adopt Intent incrementally, starting with a single steel thread and expanding as you see value.

### 7. Flexibility to Match Your Workflow

Every team works differently. Intent's templates and processes are starting points, not rigid requirements. Modify templates, adjust workflows, and make Intent work for your specific needs while maintaining the core principle of intention capture.

### 8. Integration with Modern LLM Tooling

Intent was designed in the age of AI-assisted development. Its structured approach to intention and documentation makes it particularly effective when working with LLMs, providing the context and clarity these tools need to be truly helpful.

## The Steel Thread Concept

While we'll dive deep into steel threads in the next post, it's worth understanding the basic concept as it's central to how Intent works.

A steel thread is a complete, minimal path through your system that delivers value. Think of it as the thinnest possible slice that:

- Solves a real problem
- Can be implemented independently
- Provides learning about the system
- Captures clear intention

### Real Impact: Intent at Work

Here's a real example from building Intent itself:

```bash
$ intent st show ST0012
# "Document Sync Command - sync steel thread index"

$ intent st show ST0016
# "Rename STP to Intent - complete v2.1.0 refactoring"
```

These aren't just task lists. Each thread contains:

- **Why** we needed these features
- **What** constraints we faced
- **How** we approached the solution
- **Learnings** from implementation

When working on ST0016 (the major refactoring), the AI assistant could see:

- Why the rename was necessary
- What backward compatibility to maintain
- Which patterns to follow

Result: The AI provided targeted, context-aware suggestions instead of generic refactoring advice.

## Benefits of Intent

Adopting Intent brings concrete benefits to your development process:

### Better Alignment Between Intent and Implementation

When every piece of code traces back to a clearly stated intention, misalignment becomes obvious and correctable. Reviews shift from "Is this good code?" to "Does this serve our purpose?" – a much more valuable question.

### Documentation That Stays Up-to-Date

Because documentation drives development rather than following it, it naturally stays current. The templates prompt for updates at each stage, and the documentation evolves alongside the implementation. No more archaeology to understand why code exists.

### Fine-Grained Visibility Into Work Progress

The two-tier system of steel threads and tasks provides both strategic and tactical visibility. Stakeholders can track high-level progress through steel threads, while developers manage day-to-day work through linked tasks. Everyone gets the view they need.

### Automatic Status Synchronization

As tasks complete, steel thread status updates automatically. This isn't just convenient – it ensures that high-level tracking reflects ground truth. No more status meetings to figure out where things really stand.

### Improved Onboarding Experience

New team members can understand not just what the code does, but why it exists. Each steel thread tells a complete story from intention to implementation. This context dramatically reduces the time needed to become productive.

### Enhanced Collaboration with LLMs

When working with AI assistants, the structured intention and context in Intent documentation provides exactly what LLMs need to give relevant, aligned suggestions. Instead of guessing at your goals, they can reference explicit intentions.

### More Efficient Development Process

While it might seem like additional overhead, Intent actually streamlines development by:

- Reducing rework from misunderstood requirements
- Eliminating redundant status tracking
- Preventing scope creep through clear intentions
- Enabling parallel work through independent steel threads

## What's Coming in This Blog Series

This introduction has given you a high-level view of Intent, but there's much more to explore. Here's what's coming in the rest of this series:

**[The Steel Thread Methodology](./0002-the-steel-thread-methodology.md)**: A deep dive into steel threads – what they are, how to create them, and why they're more effective than traditional work organisation.

**[Intent Capture in Software Development](./0003-intent-capture-in-software-development.md)**: Practical techniques for capturing, preserving, and leveraging intention throughout your development process.

**[LLM Collaboration with Intent](./0004-llm-collaboration-with-intent.md)**: How Intent's structure makes AI assistance more effective, with real examples of improved LLM interactions.

**[Getting Started with Intent](./0005-getting-started-with-intent.md)**: A practical guide to implementing Intent in your project, including installation, configuration, and your first steel thread.

**[Next Steps and Future Work](./0006-next-steps-and-future-work.md)**: Where Intent is heading and how you can contribute to its development.

## Ready to Transform Your Development Process?

Intent offers a pragmatic solution to the intention problem in modern software development. By making intention explicit and structural, it bridges the gap between why we build and what we build, creating better outcomes for both human and AI collaboration.

In the next post, we'll explore the steel thread methodology in detail, showing you exactly how to break down work in a way that preserves intention while enabling incremental progress.

[Continue to: The Steel Thread Methodology →](./0002-the-steel-thread-methodology.md)
