---
verblock: "01 Jul 2025:v0.1: Intent User - Initial version"
intent_version: 2.1.0
status: Completed
created: 20250701
completed: 20250705
---
# ST0001: Hello World Project Setup

- **Status**: Completed
- **Created**: 2025-07-01
- **Completed**: 2025-07-05
- **Author**: Intent User

## Objective

Set up the initial hello-world project using Intent v2.0.0 structure.

## Context

This example demonstrates the new v2.0.0 structure with:
- JSON configuration instead of YAML
- Flattened directory structure (intent/st/ not stp/prj/st/)
- Tool executables in top-level bin/
- Templates in top-level lib/templates/
- Reference docs in intent/ref/ (was usr/)

## Related Steel Threads

None - this is the first steel thread.

## Context for LLM

This is a clean v2.0.0 project showing the target structure after migration. Key differences:
1. Configuration is JSON-based
2. Directory structure is flattened
3. Clear separation of tool (bin/, lib/) from project artifacts (intent/, backlog/)