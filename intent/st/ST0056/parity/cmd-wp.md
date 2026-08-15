# Parity inventory: `intent wp`

> Measured at `69d42a7` from `parity/probes/toplevel.tsv`. Rendered 2026-08-15 by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.

- **v2 source**: `bin/intent_wp`
- **help file**: `lib/help/wp.help.md`

## Subcommands

Taken from the main dispatch and verified by invocation. `help`/`--help`/`-h` arms are omitted; they are covered in the probe table.

| verb    |
| ------- |
| `new`   |
| `done`  |
| `start` |
| `list`  |
| `show`  |

## Flags

None parsed.

## Observed behaviour

| probe             | exit | stdout | stderr | first line                                                                   |
| ----------------- | ---- | ------ | ------ | ---------------------------------------------------------------------------- |
| `intent wp`       | 1    | 0B     | 40B    | error: Work package command is required                                      |
| `--help`          | 1    | 658B   | 0B     | Usage: intent wp <command> [options] [arguments]                             |
| unknown flag      | 1    | 0B     | 77B    | error: Unknown wp command: --zzz-not-a-flag. Run 'intent wp help' for usage. |
| outside a project | 1    | 0B     | 110B   | error: not in an Intent project directory                                    |
