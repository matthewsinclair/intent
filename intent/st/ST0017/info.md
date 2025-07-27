---
verblock: "27 Jul 2025:v0.1: matts - Initial version"
stp_version: 2.0.0
status: In Progress
created: 20250727
completed: 
---
# ST0017: Add an Intent sub-agent for Claude Code to Intent

## Brief to prime Claude Code

Claude Code has just introduced [Sub Agents](https://docs.anthropic.com/en/docs/claude-code/sub-agents). Please read that now to get up to speed with how it works.

I want to do things to update Intent to work with Claude Code's sub-agents.

1. I want to add a new sub-agent called "Intent" which will act as a sub-agent and help Claude work with Intent-based projects. The sub-agent will know all about what Intent is, how it works, and what needs to happen to set it up, ec.

2. I want to extend Intent so that it is possible to easily add both global (ie available for all Intent instances) and local (ie inly available to the per-project Intent installation). I want a "pluggable sub-agent" capability that is built into Intent so that others can extend it with their own sub-agents. And I want the Intent sub-agent to be the first version of this kind of thing.

3. And then I want to build the second Intent sub-agent, which will be an "Elixir Code Doctor" that will know about how to write great Elixir code, as well as integrate with the [Usage Rules](https://www.zachdaniel.dev/p/usage-rules-leveling-the-playing) and [Useage Rules on GitHub](https://hexdocs.pm/usage_rules/readme.html) process and [Ash AI](https://github.com/ash-project/ash_ai/blob/main/usage-rules.md).

In doing #2, we will build the capability to do #1 (and #3), and then make it a plugable capability for Intent going forward.

To help refine the design, I want you to enter "Intellectual Sparring Partner Mode" and help me to refine the design. Once we have a refined design for adding sub-agents to Intent, we can then go about building it.

A few notes, in no particular order:

- We will add a new directory called agents and put it here intent/agents
- In agents/ we will have a sub-dir for each of the agents (ie intent/agents/{intent,elixir})
- We need a way to manage the installed agents for the Intent core installation (easy: git pull from the Intent repo) and then for the local per-project agents (TBC how this works)
- We will need to add a new command "intent agents" with appropriate support
- We will need commands for it such as "intent agents {list,add,delete,show}" and whatever options make sense

When you're ready, let's dive in and build out a design.

## Objective

[Clear statement of what this steel thread aims to accomplish]

## Context

[Background information and context for this steel thread, including why it's needed and how it fits into the larger project]

## Related Steel Threads

- [List any related steel threads here]

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
