---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
intent_version: 2.0.0
status: Completed
created: 20250307
completed: 20250307
---
# ST0012: Document Sync Command

- **Status**: Completed
- **Created**: 2025-03-07
- **Completed**: 2025-03-07
- **Author**: Matthew Sinclair

## Objective

Create a new `stp st sync` command that will maintain the steel_threads.md document by synchronizing it with the current state of individual steel thread files.

## Context

Currently, the `stp/prj/st/steel_threads.md` document needs to be manually kept in sync with the individual ST####.md files. The `stp st list` command now reads directly from the ST files, but the summary document needs to be updated separately.

This causes inconsistencies when steel thread status changes or when new steel threads are added. A mechanism is needed to ensure the summary document accurately reflects the current state of all steel threads.

## Related Steel Threads

- [List any related steel threads here]

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Check off tasks as they are completed
3. Add implementation notes as decisions are made or challenges encountered
4. Add results when the steel thread is completed
5. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
