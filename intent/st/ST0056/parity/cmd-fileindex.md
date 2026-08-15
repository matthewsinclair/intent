# Parity inventory: `intent fileindex`

> Measured at `69d42a7` from `parity/probes/toplevel.tsv`. Rendered 2026-08-15 by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.

- **v2 source**: `bin/intent_fileindex`
- **help file**: `lib/help/fileindex.help.md`

## Subcommands

None -- single-action command (no dispatch `case` on a command variable).

## Flags

Parsed as `case` arms anywhere in the script, including nested arg loops.

| flag           |
| -------------- |
| `--check`      |
| `--file`       |
| `--help`       |
| `--index`      |
| `--index-dir`  |
| `--intent-dir` |
| `--no-intent`  |
| `--toggle`     |
| `--uncheck`    |
| `-C`           |
| `-U`           |
| `-X`           |
| `-f`           |
| `-h`           |
| `-i`           |
| `-r`           |
| `-v`           |

## Observed behaviour

| probe              | exit | stdout | stderr | first line                                              |
| ------------------ | ---- | ------ | ------ | ------------------------------------------------------- |
| `intent fileindex` | 0    | 5B     | 0B     | [ ]                                                     |
| `--help`           | 1    | 1518B  | 0B     | Usage: intent_fileindex [OPTIONS] [STARTDIR] [FILESPEC] |
| unknown flag       | 1    | 1551B  | 0B     | Unknown option: --zzz-not-a-flag                        |
| outside a project  | 0    | 5B     | 0B     | [ ]                                                     |
