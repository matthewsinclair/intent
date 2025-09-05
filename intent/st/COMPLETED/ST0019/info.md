---
verblock: "05 Sep 2025:v0.1: matts - Initial version"
intent_version: 2.3.2
status: Completed
created: 20250905
completed: 20250905
---
# ST0019: Ash-Expert Agent for Modern Ash Framework Code Quality and Architecture

## Objective

Create a specialized Intent agent that provides comprehensive Ash Framework expertise, focusing on modern Ash 3.0+ patterns, code quality enforcement, and architectural guidance to prevent common anti-patterns and promote best practices.

## Context

The Intent project's agent system needed a dedicated Ash Framework specialist to complement the existing elixir and worker-bee agents. While the elixir agent provides general Elixir Usage Rules and the worker-bee agent enforces WDD architecture, there was a gap in specialized Ash Framework knowledge.

This steel thread addresses the need for:
- Critical quality gates to catch Ecto/Ash anti-patterns before production
- Modern Ash 3.0+ pattern promotion over legacy approaches
- Performance optimization through proper query patterns and bulk operations
- Comprehensive migration and resource definition guidance
- Integration with existing Intent documentation at intent/docs/ref/ash/
- A "strict but helpful mentor" approach to Ash development

The ash-expert agent acts as the final quality layer for Ash Framework implementations, ensuring code follows modern patterns, performs well, and maintains proper domain-driven design principles.

## Related Steel Threads

- ST0017: Intent Agent System (foundational agent infrastructure)
- ST0018: Worker-Bee Intent Agent (complementary WDD architecture enforcement)
- Related to Elixir agent for general language patterns

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.