# Implementation - ST0051: intent output width: dft_width config for generated files, terminal width for stdout

## Implementation

- `bin/intent_helpers`: added `get_default_width()` (wraps `get_config_field dft_width 120`), exported alongside `get_terminal_width`.
- `bin/intent_st`: `sync` case `WIDTH=80` -> `WIDTH=$(get_default_width)`; `list` case width branch simplified (dead `sync`/`WRITE_MODE` sub-branch removed).
- `bin/intent_init`: `dft_width: 120` added to the generated config heredoc.
- `intent/.config/config.json`: `dft_width: 120` added (Intent dogfoods it).
- Regenerated `intent/st/steel_threads.md` at 120: ST0046's slug renders in full; ST0050 + ST0051 now listed.

## Code Examples

get_default_width (bin/intent_helpers):

    get_default_width() {
      get_config_field dft_width 120
    }

## Technical Details

Destination-based width. `sync` (file) -> `get_default_width`; `list` (stdout) -> `get_terminal_width`; `--width N` overrides both. `sync --write` re-invokes `list --width <default>`, so the file always receives an explicit width and the `list` else-branch only serves direct interactive `list`.

### Generator audit (AC-03.1)

| Generator                            | Output                  | Width source       | Verdict         |
| ------------------------------------ | ----------------------- | ------------------ | --------------- |
| `intent st sync --write`             | steel_threads.md (file) | get_default_width  | fixed (this ST) |
| `intent st list`                     | stdout                  | get_terminal_width | correct         |
| `intent ext` / `intent plugin`       | stdout                  | get_terminal_width | correct         |
| `intent claude skills` / `subagents` | stdout                  | get_terminal_width | correct         |
| `intent todo`                        | todo.md (file)          | none (checklist)   | width-agnostic  |
| `intent agents sync`                 | AGENTS.md (file)        | none (prose)       | width-agnostic  |

No other command sizes a file by the terminal.

## Challenges & Solutions

- The obvious suspect (the `list` case's `TABLE_WIDTH=80` branch) was dead code; the live default was the `sync` case's `WIDTH=80`. Traced by following `sync --write` -> `list --width 80`. Fixed the real site and pruned the dead branch.
