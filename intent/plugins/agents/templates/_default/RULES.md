# RULES.md

Mandatory coding rules for this project. Every statement is "must" or "never".

## Canon Rules

The four agnostic principles enforced across every Intent project:

- **Highlander** (`IN-AG-HIGHLANDER-001`) -- there can be only one; no divergent copies of the same concern.
- **PFIC** (`IN-AG-PFIC-001`) -- Pure-Functional-Idiomatic-Coordination; pattern match, pipe, tag, compose.
- **Thin Coordinator** (`IN-AG-THIN-COORD-001`) -- coordinators parse to call to render; business logic lives elsewhere.
- **No Silent Errors** (`IN-AG-NO-SILENT-001`) -- every failure surfaces; rescue-and-swallow is forbidden.

Full rule files: `intent/plugins/claude/rules/agnostic/<slug>/RULE.md`.

## Language-Specific Rules

Language-specific concretisations live at `intent/plugins/claude/rules/<lang>/`. Per-language canon (`intent/llm/RULES-<lang>.md`, `intent/llm/ARCHITECTURE-<lang>.md`) is opt-in: install via `intent lang init <lang>` per language used in this project, or pass `--lang <lang1>,<lang2>` to `intent init`.

Per-language rule packs available in canon: `elixir`, `rust`, `swift`, `lua`, `shell`.

## Language Packs

<!-- intent-lang-packs:start -->
<!-- intent-lang-packs:end -->

## NEVER DO

- NEVER write backwards-compatible code (Intent is fail-forward).
- NEVER duplicate a code path (Highlander).
- NEVER swallow errors silently (No Silent Errors).
- NEVER bypass coordinator/business-logic separation (Thin Coordinator).
- NEVER manually wrap lines in markdown files.

## Project-Specific Rules

<!-- Add rules unique to this project below this line. Cite IN-* IDs where applicable. -->
