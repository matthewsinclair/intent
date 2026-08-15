# Parity inventory: `intent doctor`

> Measured at `69d42a7` from `parity/probes/toplevel.tsv`. Rendered 2026-08-15 by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.

- **v2 source**: `bin/intent_doctor`
- **help file**: none -- `intent help doctor` falls through to the "no help available" path (`bin/intent_help:37`)

## Subcommands

None -- single-action command (no dispatch `case` on a command variable).

## Flags

Parsed as `case` arms anywhere in the script, including nested arg loops.

| flag        |
| ----------- |
| `--fix`     |
| `--help`    |
| `--quiet`   |
| `--verbose` |
| `-f`        |
| `-h`        |
| `-q`        |
| `-v`        |

## Observed behaviour

| probe             | exit | stdout | stderr | first line                       |
| ----------------- | ---- | ------ | ------ | -------------------------------- |
| `intent doctor`   | 0    | 563B   | 0B     | doctor: intent v2.19.0           |
| `--help`          | 0    | 384B   | 0B     | Usage: intent_doctor [OPTIONS]   |
| unknown flag      | 1    | 0B     | 66B    | Unknown option: --zzz-not-a-flag |
| outside a project | 0    | 397B   | 0B     | doctor: intent v2.19.0           |
