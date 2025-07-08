---
title: "Introduction to STP"
date: "2025-07-08"
author: "Matthew Sinclair"
draft: false
word_count: 1623
---

# Introduction to STP: A Better Way to Build Software

If you've ever lost track of why a piece of code exists, struggled to onboard a new team member, or watched an LLM confidently solve the wrong problem, you understand the cost of lost intention in software development. In our [previous post](./0000-motivation-for-stp.md), we explored why capturing and preserving intention is crucial for modern development, especially when collaborating with AI.

Today, I want to introduce you to the Steel Thread Project (STP) – a practical solution to the intention problem. STP isn't another heavyweight methodology or a complex framework. It's a lightweight system that enhances your existing workflow with intention-aware structure, making both human and AI collaboration more effective.

## Building on the Intention Foundation

We established that the fundamental challenge in modern development isn't just building software – it's ensuring that what we build aligns with why we're building it. This challenge intensifies when working with LLMs, which excel at pattern matching but lack understanding of underlying purpose.

STP addresses this challenge by making intention explicit and structural. Rather than hoping developers will document the "why" or expecting LLMs to infer our goals, STP builds intention capture into the development workflow itself.

The shift from theoretical understanding to practical implementation happens through three key innovations:
1. **Steel threads** that encapsulate both intent and implementation
2. **Structured templates** that prompt for intention at every stage
3. **Integration with task management** that maintains the intent-to-execution link

## What is STP?

The Steel Thread Project is a lightweight methodology for structuring both development and documentation around clearly captured intentions. At its heart, STP is surprisingly simple: shell scripts + markdown templates + task tracking = intention-aware development.

Let me break this down:

**Shell Scripts**: A collection of simple bash scripts that automate common tasks:
- `stp st new` - Create a new steel thread with intention-capturing template
- `stp st list` - View all steel threads and their status
- `stp bl` - Integrate with Backlog.md for task management
- `stp status` - Synchronise steel thread status with task completion

**Markdown Templates**: Structured documents that prompt for intention:
- Steel thread templates that start with "why" before "what"
- Technical design documents with intention sections
- User guides that explain purpose alongside usage

**Task Tracking**: Fine-grained visibility through Backlog.md integration:
- Each steel thread can have multiple associated tasks
- Tasks track the detailed work while threads maintain the big picture
- Automatic status updates based on task completion

The magic happens when these simple components work together. A steel thread captures your intention, tasks track your implementation, and templates ensure nothing important gets lost along the way.

Importantly, STP is designed to work alongside your existing practices. Whether you use Agile, Waterfall, or something in between, STP adds intention-awareness without disrupting your workflow. It's an enhancement, not a replacement.

## Core Principles of STP

STP is built on eight core principles that guide its design and implementation:

### 1. Documentation as a First-Class Citizen
In STP, documentation isn't something you do after coding – it's an integral part of the development process. Every steel thread starts with documentation that captures intention, and this documentation evolves alongside the code. When documentation drives development, both humans and LLMs have the context they need to make good decisions.

### 2. Intent Capture Throughout the Lifecycle
Intention isn't just captured at the beginning – it's maintained and referenced throughout development. From initial conception through implementation to future maintenance, the "why" remains visible and relevant. This creates a traceable lineage from business need to technical implementation.

### 3. Incremental Development Through Steel Threads
Rather than tackling entire features or epics, STP encourages breaking work into steel threads – complete, minimal paths through your system. Each thread can be understood, implemented, and validated independently, making development more manageable and progress more visible.

### 4. Task Tracking Linked to Steel Threads
While steel threads capture the big picture intention, individual tasks track the detailed work. STP's integration with Backlog.md creates a two-level system: strategic intent at the thread level, tactical execution at the task level. This separation keeps both perspectives clear and connected.

### 5. Process-Agnostic Compatibility
STP doesn't dictate how you should develop software. Whether you're using Scrum, Kanban, or any other methodology, STP layers intention-awareness on top. It's designed to enhance, not replace, your existing workflow.

### 6. Lightweight Enhancement
The entire STP system consists of simple shell scripts and markdown templates. No complex tools to learn, no vendor lock-in, no heavyweight processes. You can adopt STP incrementally, starting with a single steel thread and expanding as you see value.

### 7. Flexibility to Match Your Workflow
Every team works differently. STP's templates and processes are starting points, not rigid requirements. Modify templates, adjust workflows, and make STP work for your specific needs while maintaining the core principle of intention capture.

### 8. Integration with Modern LLM Tooling
STP was designed in the age of AI-assisted development. Its structured approach to intention and documentation makes it particularly effective when working with LLMs, providing the context and clarity these tools need to be truly helpful.

## The Steel Thread Concept

While we'll dive deep into steel threads in the next post, it's worth understanding the basic concept as it's central to how STP works.

A steel thread is a complete, minimal path through your system that delivers value. Think of it as the thinnest possible slice that:
- Solves a real problem
- Can be implemented independently
- Provides learning about the system
- Captures clear intention

Here's how the STP workflow typically looks:

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   Intention     │     │  Steel Thread    │     │     Tasks       │
│   Capture       │────▶│   Creation       │────▶│   Definition    │
│                 │     │  (stp st new)    │     │  (stp bl)       │
└─────────────────┘     └──────────────────┘     └─────────────────┘
         │                       │                         │
         │                       ▼                         ▼
         │              ┌──────────────────┐     ┌─────────────────┐
         │              │  Documentation   │     │ Implementation  │
         │              │   Templates      │────▶│    (coding)     │
         │              └──────────────────┘     └─────────────────┘
         │                       │                         │
         │                       ▼                         ▼
         │              ┌──────────────────┐     ┌─────────────────┐
         └─────────────▶│     Review &     │◀────│     Testing     │
                        │   Validation     │     │                 │
                        └──────────────────┘     └─────────────────┘
```

This differs from traditional work organisation where tasks often lose connection to their original purpose. In STP, every task links back to a steel thread, and every steel thread explicitly captures intention.

## Benefits of STP

Adopting STP brings concrete benefits to your development process:

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
When working with AI assistants, the structured intention and context in STP documentation provides exactly what LLMs need to give relevant, aligned suggestions. Instead of guessing at your goals, they can reference explicit intentions.

### More Efficient Development Process
While it might seem like additional overhead, STP actually streamlines development by:
- Reducing rework from misunderstood requirements
- Eliminating redundant status tracking
- Preventing scope creep through clear intentions
- Enabling parallel work through independent steel threads

## What's Coming in This Blog Series

This introduction has given you a high-level view of STP, but there's much more to explore. Here's what's coming in the rest of this series:

**[The Steel Thread Methodology](./0002-the-steel-thread-methodology.md)**: A deep dive into steel threads – what they are, how to create them, and why they're more effective than traditional work organisation.

**[Intent Capture in Software Development](./0003-intent-capture-in-software-development.md)**: Practical techniques for capturing, preserving, and leveraging intention throughout your development process.

**[LLM Collaboration with STP](./0004-llm-collaboration-with-stp.md)**: How STP's structure makes AI assistance more effective, with real examples of improved LLM interactions.

**[Getting Started with STP](./0005-getting-started-with-stp.md)**: A practical guide to implementing STP in your project, including installation, configuration, and your first steel thread.

**[Next Steps and Future Work](./0006-next-steps-and-future-work.md)**: Where STP is heading and how you can contribute to its development.

## Ready to Transform Your Development Process?

STP offers a pragmatic solution to the intention problem in modern software development. By making intention explicit and structural, it bridges the gap between why we build and what we build, creating better outcomes for both human and AI collaboration.

In the next post, we'll explore the steel thread methodology in detail, showing you exactly how to break down work in a way that preserves intention while enabling incremental progress.

[Continue to: The Steel Thread Methodology →](./0002-the-steel-thread-methodology.md)
