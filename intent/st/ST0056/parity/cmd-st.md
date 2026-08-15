# Parity inventory: `intent st`

> Measured at `69d42a7` from `parity/probes/toplevel.tsv`. Rendered 2026-08-15 by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.

- **v2 source**: `bin/intent_st`
- **help file**: none -- `intent help st` falls through to the "no help available" path (`bin/intent_help:37`)

## Subcommands

Taken from the main dispatch and verified by invocation. `help`/`--help`/`-h` arms are omitted; they are covered in the probe table.

| verb       |
| ---------- |
| `new`      |
| `done`     |
| `cancel`   |
| `start`    |
| `list`     |
| `show`     |
| `edit`     |
| `sync`     |
| `repair`   |
| `organize` |
| `zero`     |

## Flags

Parsed as `case` arms anywhere in the script, including nested arg loops.

| flag         |
| ------------ |
| `--markdown` |
| `--start`    |
| `--status`   |
| `--width`    |
| `--write`    |
| `-s`         |

## Observed behaviour

| probe             | exit | stdout | stderr | first line                                       |
| ----------------- | ---- | ------ | ------ | ------------------------------------------------ |
| `intent st`       | 1    | 0B     | 40B    | error: Steel thread command is required          |
| `--help`          | 1    | 2330B  | 0B     | Usage: intent st <command> [options] [arguments] |
| unknown flag      | 1    | 0B     | 41B    | error: Unknown command: --zzz-not-a-flag         |
| outside a project | 1    | 0B     | 110B   | error: not in an Intent project directory        |
