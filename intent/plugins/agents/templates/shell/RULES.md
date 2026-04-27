# RULES-shell.md

Mandatory shell (bash + zsh) coding rules for this project. Concretises the four agnostic principles for shell idioms.

## Canon Concretisations

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- one canonical implementation per concern. No divergent helper scripts. Source shared functions from `lib/` or a `helpers` script. Concretised by `IN-SH-CODE-001`.
- **PFIC** (`IN-AG-PFIC-001`) -- prefer `case` for branching, pipelines for composition, parameter expansion over awk/sed when possible. Concretised by `IN-SH-CODE-002`.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- top-level scripts parse to call to render; business logic in sourced functions. Concretised by `IN-SH-CODE-003`.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- always `set -euo pipefail` (or document why not). Check exit codes; never `cmd || true` to suppress failures unless the failure is genuinely benign. Concretised by `IN-SH-CODE-004`.

Full shell rule pack: `intent/plugins/claude/rules/shell/<category>/<slug>/RULE.md`.

## NEVER DO

- NEVER omit `set -euo pipefail` at the top of a script without a documented reason.
- NEVER use unquoted `$var` expansions where word splitting matters (filenames, user input).
- NEVER use `eval` on user input.
- NEVER use bash 4+ features (associative arrays `declare -A`, `${VAR^}` upper-casing) when targeting macOS bash 3.x.
- NEVER duplicate helper functions across scripts; source from a shared library.
- NEVER manually wrap lines in markdown files.

## Project-Specific Rules

<!-- Add shell-specific rules unique to this project below this line. Cite IN-SH-* IDs where applicable. -->
