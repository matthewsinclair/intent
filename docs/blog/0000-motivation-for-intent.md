---
title: "The Motivation for Intent: Why Intention Matters"
date: "2025-07-08"
author: "Matthew Sinclair"
draft: false
word_count: 1507
---

# The Motivation for Intent: Why Intention Matters in LLM-Assisted Development

LLMs like Claude have changed how we write code, debug systems, and design solutions. But a hard problem remains: how do we make sure these tools understand not just what we want to build, but why we want to build it?

Intent is a response to the growing disconnect between developer intention and LLM execution. In our rush to use AI capabilities, we've overlooked the most important element of successful collaboration: shared understanding of purpose.

## The Challenge of LLM Collaboration

When you sit down with an LLM to solve a coding problem, you describe what you need, the LLM responds with seemingly perfect understanding, and code appears. But there's a hidden problem: the LLM doesn't actually understand your intentions. It's performing sophisticated pattern matching based on your words.

This creates what I call the "illusion of understanding." The LLM's responses are so coherent, so contextually appropriate, that we assume it grasps not just what we're asking for, but why we're asking for it. This assumption becomes dangerous as projects grow in complexity.

Consider a typical interaction:

- Developer: "Create a user authentication system"
- LLM: _Generates a complete auth system with login, registration, and password reset_

On the surface, this looks like success. But what if your intention was to create a passwordless system for enhanced security? What if you needed integration with an existing SSO provider? What if this was for an internal tool where email verification would be counterproductive?

The gap between what you want and what you get widens with each assumption the LLM makes. Standard prompting often leads to misalignment because we focus on describing the "what" rather than communicating the "why."

As projects grow, this miscommunication compounds. Each misunderstood intention becomes technical debt. Each assumption becomes a constraint. What started as rapid development transforms into a tangled web of corrections, clarifications, and workarounds.

## The Problem of Lost Intention

In traditional software development, we've long recognised the importance of requirements gathering and documentation. Yet even with these practices, we regularly see software that is technically correct but disconnected from the original problems it was meant to solve.

With LLM-assisted development, this problem intensifies. The speed at which we can now generate code means we can travel much farther down the wrong path before realising we've lost our way. The question "what problem are we solving?" becomes not just important, but critical to project success.

I've witnessed this firsthand in numerous projects:

- A caching system that improved performance but made debugging impossible
- An elegant API that satisfied all technical requirements but failed to meet actual user needs
- A refactoring that improved code quality while eliminating features users depended on

The business cost of this misalignment is real. Industry studies consistently show that fixing problems in production costs 100x more than preventing them during design. When working with LLMs, this multiplier accelerates because we can implement misaligned solutions faster than ever before.

Intention gets diluted through implementation phases in predictable ways:

1. Initial vision → Vague requirements
2. Requirements → Technical specifications
3. Specifications → Implementation details
4. Details → Code

At each translation, a little more of the "why" gets lost. By the time we're writing code, we're often solving a different problem than the one we started with. Without clear intention capture, we lack the North Star needed to evaluate whether our elegant solutions actually solve real problems.

## Enhancing Traditional Approaches for LLM Collaboration

You might be thinking, "Don't existing methodologies already address this?" Agile, Lean, XP, and other approaches do emphasise understanding user needs and maintaining alignment with business value. But there's a real intention-capture gap when these methodologies meet LLM collaboration.

Traditional approaches were designed for human-to-human communication. They assume shared context, cultural understanding, and the ability to read between the lines. LLMs, however brilliant, lack these implicit understandings. They need explicit intention to guide their pattern matching toward useful outcomes.

Intent doesn't seek to replace your existing methodology. Instead, it adds a layer of intention clarity that makes any approach more effective when working with LLMs. Think of it as adding semantic markup to your development process – making the implicit explicit.

In practice, this means:

- Agile stories gain intention statements that explain why this feature matters
- Lean experiments document not just what to measure, but what we hope to learn
- XP practices include intention context that helps LLMs suggest appropriate solutions

Documentation transforms from an afterthought into a tool for understanding. Instead of documenting what we built after the fact, we document why we're building it before we start. This isn't additional overhead -- it's front-loading the thinking we'd do anyway, in a format that both humans and LLMs can use.

The impact on technical debt is worth noting. When every piece of code can be traced back to a clear intention, refactoring becomes purposeful rather than aesthetic. We can ask, "Does this still serve its original intention?" rather than just "Is this clean code?"

## The Missing Layer: Intent Architecture

What's been missing from our development stack is what I call "Intent Architecture" – a structured approach to capturing and maintaining the why behind every technical decision. Just as we have software architecture for the "how" and requirements for the "what," we need intent architecture for the "why."

Intention forms the foundation of all technical decisions, yet we rarely make it explicit. When intention is clear, constraints become clearer too. Instead of arbitrary technical requirements, we have purpose-driven boundaries that guide both human and LLM decision-making.

The relationship between intention clarity and implementation quality is direct and measurable:

- Clear intention → Focused solutions
- Vague intention → Over-engineered or off-target implementations
- No stated intention → Solutions in search of problems

Intention also works as a filter for evaluating LLM suggestions. When an LLM proposes a solution, we can ask: "Does this serve our stated intention?" This simple question turns LLM collaboration from a hit-or-miss affair into a guided process.

Building a shared mental model between humans and LLMs requires making our mental models explicit. We can't expect an LLM to infer our intentions from context clues the way a human colleague might. Instead, we need to architect our intentions as deliberately as we architect our systems.

## Intention-First Development

Intent changes how we approach development. Instead of starting with "what to build," we start with "why we're building it." This isn't just philosophical -- it changes how we interact with LLMs and how they can assist us.

When intention leads, the developer-LLM conversation transforms:

- Before: "Build me a REST API for user management"
- After: "We need to enable self-service user onboarding to reduce support load. Let's design an API that prioritises ease of use over flexibility."

The second prompt gives the LLM context for intelligent trade-offs. It can now suggest solutions optimised for your actual goals, not just technical correctness.

The return on investment in intention clarification is substantial:

- Reduced rework: solutions align with goals from the start
- Better suggestions: LLMs provide more relevant options
- Clearer evaluation: easy to assess if solutions meet intentions
- Knowledge preservation: future developers (and LLMs) understand the why

Consider how intention transforms implementation approaches. The same requirement – "add user notifications" – leads to vastly different solutions depending on intention:

- Intention: Increase engagement → Rich, frequent notifications
- Intention: Reduce cognitive load → Minimal, batched notifications
- Intention: Meet compliance requirements → Audit-focused notifications

Intention becomes the cornerstone of technical intuition. When we make intentions explicit, we're not just documenting for others – we're clarifying our own thinking and creating a foundation for better technical decisions.

## Intent: An Answer to the Intention Question

The Intent emerged from a simple observation: successful LLM collaboration requires explicit intention capture, but we lacked a systematic way to achieve this. Intent was designed specifically to bridge this gap.

At its core, Intent introduces the concept of a "steel thread" – not just a development task or user story, but an intention container. Each steel thread captures:

- The problem we're solving (the why)
- The success criteria (how we'll know we've succeeded)
- The constraints and context (the boundaries)
- The implementation approach (the how)

This structure prioritises the "why" before the "what." By the time we get to implementation details, both humans and LLMs have a clear understanding of purpose.

Intent creates intention alignment throughout development by:

1. Capturing intention at the moment of conception
2. Preserving intention through structured documentation
3. Referencing intention during implementation decisions
4. Validating against intention when evaluating solutions

The difference between tool-first and intention-first approaches matters. Tool-first asks "How can we use this LLM?" Intention-first asks "What are we trying to achieve, and how can an LLM help?"

In the following posts, we'll explore how Intent implements these principles, look closely at the steel thread methodology, and walk through real examples of intention-driven development. The starting point is simple: when working with AI, making your intentions explicit pays for itself many times over.

## Next Steps

Ready to see how Intent puts these principles into practice? Continue with our next post: [Introduction to Intent](./0001-introduction-to-intent.md), where we'll explore the concrete components and workflow that make intention-first development a reality.
