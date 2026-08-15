# Parity inventory: `intent config`

> Measured at `69d42a7` from `parity/probes/toplevel.tsv`. Rendered 2026-08-15 by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.

- **v2 source**: `bin/intent_config`
- **help file**: none -- `intent help config` falls through to the "no help available" path (`bin/intent_help:37`)

## Subcommands

None -- single-action command (no dispatch `case` on a command variable).

## Flags

None parsed.

## Observed behaviour

| probe             | exit | stdout | stderr | first line                                |
| ----------------- | ---- | ------ | ------ | ----------------------------------------- |
| `intent config`   | 0    | 0B     | 0B     | _(no output)_                             |
| `--help`          | 0    | 0B     | 0B     | _(no output)_                             |
| unknown flag      | 0    | 0B     | 0B     | _(no output)_                             |
| outside a project | 1    | 0B     | 114B   | error: not in an Intent project directory |
