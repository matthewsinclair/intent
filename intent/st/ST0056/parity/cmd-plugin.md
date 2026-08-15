# Parity inventory: `intent plugin`

> Measured at `69d42a7` from `parity/probes/toplevel.tsv`. Rendered 2026-08-15 by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.

- **v2 source**: `bin/intent_plugin`
- **help file**: `lib/help/plugin.help.md`

## Subcommands

Taken from the main dispatch and verified by invocation. `help`/`--help`/`-h` arms are omitted; they are covered in the probe table.

| verb   |
| ------ |
| `list` |
| `show` |

## Flags

None parsed.

## Observed behaviour

| probe             | exit | stdout | stderr | first line                                                                               |
| ----------------- | ---- | ------ | ------ | ---------------------------------------------------------------------------------------- |
| `intent plugin`   | 0    | 1076B  | 0B     | Intent Plugins                                                                           |
| `--help`          | 0    | 371B   | 0B     | Usage: intent plugin [command]                                                           |
| unknown flag      | 1    | 0B     | 89B    | error: Unknown plugin subcommand '--zzz-not-a-flag'. Run 'intent plugin help' for usage. |
| outside a project | 0    | 1076B  | 0B     | Intent Plugins                                                                           |
