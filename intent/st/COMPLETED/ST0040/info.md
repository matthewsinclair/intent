---
verblock: "18 May 2026:v0.1: matts - Initial version"
intent_version: 2.4.0
status: Completed
slug: whiteboard-protocol-for-multi-claude-sessions-in
created: 20260518
completed: 20260518
---

# ST0040: Whiteboard protocol for multi-Claude sessions in the one repo

## Objective

Specify and ship a coordination protocol for multiple Claude Code sessions running concurrently against a single Intent project, plus the `in-whiteboard` skill that operationalises it. Each session belongs to a _stream_ (a durable identity, eg `control`, `ia-ux`) that owns a file under `intent/whiteboard/`. The whiteboard is the _live_ coordination channel; `wip.md` remains the post-session snapshot.

## Context

Lamplight (the live test environment) runs two concurrent Claude Code sessions -- an `apps/control/**` runtime stream and an `apps/frontdesk` / `apps/storyfield` / `apps/wrighter` IA/UX/UI stream. The existing `intent/wip.md` "add sections, don't overwrite" convention handles post-session snapshots but says nothing about live coordination. In-flight decisions in one stream (eg promoting a struct field, renaming a cross-app module, reserving a steel-thread number) became invisible to the other stream until next session-end -- a latency big enough to cause real collisions, including the ST0178 number-collision that pushed the MCP Player surface ST off its preferred slot.

The fix is a per-project `intent/whiteboard/` directory with per-stream files + shared `asks.md` (cross-stream handoffs) + shared `lamplight.md` (or equivalent shared-platform-layer edit channel). The `in-whiteboard` skill is the procedural shell around the directory, integrated into `/in-session` (auto-pickup) and `/in-finish` (auto-release).

The skill was landed out-of-cycle into Intent's canonical plugin pack on 2026-05-18 to unblock Lamplight; this ST captures the design, the as-built, and the work remaining to roll it into a formal Intent release. The Lamplight whiteboard at `/Users/matts/Devel/prj/Lamplight/intent/whiteboard/` is the live reference.

## Related Steel Threads

- (none) -- skill addition is self-contained. The latent `intent st new` max+1 race that motivated part of this work is its own bug; see `tasks.md` for the pointer.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
