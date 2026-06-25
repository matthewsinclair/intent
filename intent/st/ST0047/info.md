---
verblock: "25 Jun 2026:v0.2: matts - Elaborate objective, context, design, acceptance + 4 WPs"
intent_version: 2.12.0
status: WIP
slug: add-claude-with-intent-script-to-intent-to
created: 20260625
completed:
---

# ST0047: Add claude_with_intent script to Intent to support muilt-agent agentic coding (MAAC)

## Objective

Add a `claude_with_intent` command to Intent that launches and manages **multi-agent agentic coding (MAAC)** sessions over the Intent Claude Whiteboard. One command provisions whiteboard **workstreams** (the Protocol 3.0 _nodes_), launches a Claude Code session bound to a workstream with the right effort / permission / context, and manages the workstream lifecycle (`new` / `list` / `archive` / `hygiene`). The whiteboard process was pioneered in Lamplight by convention; this ST productises it -- Baize is the MVP (the first first-class, scripted use), proven there, then promoted into Intent proper and back-filled into the sibling projects.

## Context

Intent's Whiteboard (Protocol 3.0) coordinates concurrent Claude Code sessions, each a _node_ / _workstream_ with a single-writer board + inboxes. Starting a workstream session is today a manual, error-prone ritual: compose context, set effort, run `/in-session`, set a permission mode, paste a board -- and the naive form deadlocks against the `require-in-session` gate. This ST turns that ritual into one deterministic command, and adds the lifecycle ops the protocol describes but never automated (scaffold a node, list, archive, mechanical hygiene).

It sits beside the `/in-whiteboard` skill, not on top of it: the **skill** is the Claude-driven protocol (judgement ops -- pickup / ask / announce / decide / claim / the semantic archive); the **script** is the deterministic lifecycle + launch. Both honour one SSOT -- the Protocol 3.0 on-disk format.

**Lamplight (`../Lamplight/intent/whiteboard`) is the prototype** -- the process was pioneered there by convention (five hand-run nodes) and it stays the reference for how MAAC actually works; consult it for any practical question. **Baize is the MVP** -- the first productised version, standing the process up as a first-class Intent capability rather than by hand. Its `hv` + `cc` + `ic` + `vc` whiteboard was hand-scaffolded to the protocol spec and is the **golden reference** that `ws new` must reproduce. Once proven there, the capability promotes into the existing `intent claude` namespace (which already serves `rules` + `skills`) and back-fills Laksa, Lamplight, and Intent itself. MAAC = several specialised Claudes (control / interface / validation / ...) plus the human (`hv`) working one codebase in concert.

## Acceptance

Acceptance Criteria and Acceptance Tests for this steel thread live in `acceptance.md` (the single source of truth). Do not restate ACs here -- see that file for the ratified completeness boundary and live status.

## Related Steel Threads

- First exercised in Baize ST0012 (staff venue tools) + ST0013 (player participation) -- the two threads whose cc/ic/vc workstreams drove the need. (Baize repo, cross-project.)

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
