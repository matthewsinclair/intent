# RULES-swift.md

Mandatory Swift coding rules for this project. Concretises the four agnostic principles for Swift idioms.

## Canon Concretisations

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- one canonical implementation per concern. No divergent helper modules. Concretised by `IN-SW-CODE-001`.
- **PFIC** (`IN-AG-PFIC-001`) -- prefer pattern matching with `switch`, value types, `Result<Success, Failure>` propagation. Concretised by `IN-SW-CODE-002`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- view models / view controllers parse to call to render; business logic in dedicated services. Concretised by `IN-SW-CODE-003`.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- never swallow `try?` or `try!` for fallible work outside tests. Surface errors via `do/catch`, `Result`, or thrown propagation. Concretised by `IN-SW-CODE-004`.

Full Swift rule pack: `intent/plugins/claude/rules/swift/<category>/<slug>/RULE.md`.

## NEVER DO

- NEVER use `try!` outside of test code or where the error contract is statically documented.
- NEVER use force-unwrap (`!`) on optionals in production code -- use `guard let` or `if let`.
- NEVER use force-cast (`as!`) -- use `as?` and handle the `nil` case.
- NEVER duplicate error enum definitions across modules -- one canonical error type per concern.
- NEVER manually wrap lines in markdown files.

## Project-Specific Rules

<!-- Add Swift-specific rules unique to this project below this line. Cite IN-SW-* IDs where applicable. -->
