# RULES-lua.md

Mandatory Lua coding rules for this project. Concretises the four agnostic principles for Lua idioms.

## Canon Concretisations

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- one canonical implementation per concern. No divergent helper modules. Concretised by `IN-LU-CODE-001`.
- **PFIC** (`IN-AG-PFIC-001`) -- prefer table destructuring, multi-return propagation, explicit `nil` checks at boundaries. Concretised by `IN-LU-CODE-002`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- handlers / callbacks parse to call to render; business logic in dedicated modules. Concretised by `IN-LU-CODE-003`.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- never discard error returns from `pcall`, library calls, or multi-return functions. Always check, log, or propagate. Concretised by `IN-LU-CODE-004`.

Full Lua rule pack: `intent/plugins/claude/rules/lua/<category>/<slug>/RULE.md`.

## NEVER DO

- NEVER discard the second return value (error) from `pcall` without inspection.
- NEVER use globals for shared state; declare module-local tables and `return` them.
- NEVER mutate function arguments unless documented as mutating; prefer returning new tables.
- NEVER duplicate utility functions across modules; centralise in a shared `util.lua`.
- NEVER manually wrap lines in markdown files.

## Project-Specific Rules

<!-- Add Lua-specific rules unique to this project below this line. Cite IN-LU-* IDs where applicable. -->
