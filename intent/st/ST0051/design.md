# Design - ST0051: intent output width: dft_width config for generated files, terminal width for stdout

## Approach

Two width sources, chosen by destination:

- **Files** (eg `steel_threads.md`) take `get_default_width` -- the project config's `dft_width`, defaulting to 120. A file is read anywhere, so its width must not depend on the terminal that happened to run the command.
- **Stdout** (`intent st list` and the other list/show displays) takes the existing `get_terminal_width` -- the live terminal.
- An explicit `--width N` overrides either.

`get_default_width` is a three-line Highlander helper in `bin/intent_helpers`, wrapping the existing `get_config_field dft_width 120`. `get_terminal_width` is unchanged.

## Design Decisions

- **Root cause was a single hardcoded literal.** `intent st sync --write` defaulted `WIDTH=80` (`bin/intent_st`, the `sync` case) and re-invoked `list --width 80`, so `steel_threads.md` was always sized to 80 -- the slug column computed to 25 chars and truncated (`add-modules-properly-t...`). The fix is `WIDTH=$(get_default_width)`; at 120 the slug column is ~58 and the full slug fits.
- **The `list` case's file-vs-stdout branch was dead code.** It read `if [ "$ST_COMMAND" = "sync" ] && [ $WRITE_MODE -eq 1 ]; then TABLE_WIDTH=80`, but `sync` re-invokes `list` as a subprocess (so `ST_COMMAND` is always `list` there) and always passes `--width` (so the `WIDTH -gt 0` branch wins first). The branch could never fire. Pruned to a plain `get_terminal_width` else (fail-forward -- no dead code left behind).
- **`dft_width` is a config knob, default 120.** Read via `get_config_field` (jq, no eval -- ST0042 T1). Added to the `intent init` config template and to Intent's own `config.json`. Absent config still yields 120 (the helper's built-in default), so pre-existing projects need no migration.
- **Destination, not command, decides width.** `sync` (file-oriented, both `--write` and its dry-run display) uses the default width; `list` (interactive stdout) uses the terminal. This is why `sync`'s dry-run now previews at 120 rather than 80 -- it shows what the file will contain.

## Architecture

`bin/intent_helpers`: new `get_default_width()` beside `get_terminal_width()` (both exported). `bin/intent_st`: `sync` case default `WIDTH=$(get_default_width)`; `list` case width branch simplified to `--width` override else `get_terminal_width`. Config: `dft_width` in the `bin/intent_init` heredoc + `intent/.config/config.json`.

Every other `get_terminal_width` caller was audited (`intent_ext`, `intent_plugin`, `intent_claude_skills`, `intent_claude_subagents`, and `intent st list`): all are stdout displays -- correct as-is. No other command sizes a file by the terminal. `intent todo`'s `todo.md` and `intent agents`' `AGENTS.md` are width-agnostic (checklist / prose).

## Alternatives Considered

- **Keep a fixed file width (just change 80 -> 120).** Rejected: matts asked for a config knob so the default is tunable per project, and a named helper documents the file-vs-stdout rule for future generators. A bare literal repeats the same trap.
- **Make the file track the terminal too.** Rejected: that is the bug. A committed file read on any screen must not carry the width of whoever last regenerated it.
