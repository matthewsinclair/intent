# RULES-rust.md

Mandatory Rust coding rules for this project. Concretises the four agnostic principles for Rust idioms.

## Canon Concretisations

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- one canonical implementation per concern. No divergent helper modules. Concretised by `IN-RS-CODE-001`.
- **PFIC** (`IN-AG-PFIC-001`) -- prefer pattern matching, iterator chains, `Result<T, E>` propagation with `?`. Concretised by `IN-RS-CODE-002`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- handlers parse to call to render; business logic in dedicated modules. Concretised by `IN-RS-CODE-003`.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- never swallow `Result` with `let _ =`, `.ok()`, or `.unwrap_or_default()` for fallible work. Surface errors via `?`, `match`, or explicit logging + propagation. Concretised by `IN-RS-CODE-004`.

Full Rust rule pack: `intent/plugins/claude/rules/rust/<category>/<slug>/RULE.md`.

## NEVER DO

- NEVER write `unwrap()` or `expect()` on `Result` outside of test code or `main` (when error contract is documented).
- NEVER use `panic!()` for recoverable errors -- return `Err` instead.
- NEVER duplicate error type definitions across crates -- one canonical error enum per concern.
- NEVER bypass the `?` operator with manual `match` on `Result` when `?` would suffice.
- NEVER manually wrap lines in markdown files.

## Project-Specific Rules

<!-- Add Rust-specific rules unique to this project below this line. Cite IN-RS-* IDs where applicable. -->
