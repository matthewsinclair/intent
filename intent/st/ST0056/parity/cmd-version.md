# Parity inventory: `intent version`

> Measured at `69d42a7` from `parity/probes/toplevel.tsv`. Rendered 2026-08-15 by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.

- **v2 source**: `bin/intent`
- **help file**: none -- `intent help version` falls through to the "no help available" path (`bin/intent_help:37`)

## Subcommands

None -- single-action command (no dispatch `case` on a command variable).

## Flags

None parsed.

## Observed behaviour

| probe             | exit | stdout | stderr | first line            |
| ----------------- | ---- | ------ | ------ | --------------------- |
| `intent version`  | 0    | 22B    | 0B     | Intent version 2.19.0 |
| `--help`          | 0    | 22B    | 0B     | Intent version 2.19.0 |
| unknown flag      | 0    | 22B    | 0B     | Intent version 2.19.0 |
| outside a project | 0    | 22B    | 0B     | Intent version 2.19.0 |
