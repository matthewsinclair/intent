---
verblock: "14 Aug 2026:v0.1: matts - Initial version"
intent_version: 2.19.0
status: WIP
slug: add-a-rust-based-cli-with-a-local-sqlite-db-with
created: 20260814
completed:
---

# ST0056: Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files that exposes an MCP server with full API access to Intent

## LLM Preamble

Intent has got to the point where it is large enough to be just about too large to continue as a bunch of shell scripts that update markdown files. It is well and truly time to create a proper data model, use a local SQLite db, and make a local native CLI (in Rust).

These changes warrant a new major version of Intent: v3.0.0.

What I want to do with this steel thread is first of all rubber duck the ideas with you and thoroughly plan out the model, the tools and processes, and a workplan (using Intent, of course) to build it all out, release it -- an importantly, build an automated migration process to migrate Intent v2 projects over to Intent v3.

A couple of constraints to kick things off:

- it must always remain possible to have .md versions of all artefacts
- there will always be on-disk the .mds
- there will (probably?) be an intentd rust daemon that does the heavy lifting witht the cli talking to it
- we should always apply the thin coordinator model (ie cli is a thin coordinator over the GraphQL API to intentd and so on)
- the cli will be native (macOS, later Linux)
- we'll use Rust as the native language/runtime
- everything needs to be not just Elixir friendly, but Elixir-oriented (as most Intent projects are Elixir)
- everything needs to be AI agent aware, so the full CLI surface is exposed both as a tool and as an MCP
- we have a very good example to go from for the rust cli in Lamplight (see ../Lamplight) with strict= typing of GraphQL commands etc

And some stretch goals (in no particular order):

- I want to double down on the multi-agent coding model pioneered with Intent 2 (ie VC, CC, IC, etc)
- I want to explore how the agents can better talk to each other without needing human coordination of the comms, but by still maintaining human oversight
- We can look at making some kind of dashboard that is either a web page (easy/simple) or maybe a macos menubar app (see ../Conflab) or maybe just a TUI that can be run inside the shell (preference)
- We need to make a web page (in Laksa) for Intent's "home on the web"
- We need to 'brew it' so that it can be installed via homebrew
- ... and there will be other stretch goals too

So, the job of work here now is to do a technical deep dive on the possibilities and the build out the docs for review. And then when that's done, we crack on and build it!

## Objective

[Clear statement of what this steel thread aims to accomplish]

## Context

[Background information and context for this steel thread, including why it's needed and how it fits into the larger project]

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- [List any related steel threads here]

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
