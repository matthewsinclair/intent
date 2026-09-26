    bin/devbin docs <agents|all>

Regenerate generated documentation. devbin ships one option, offered in every project, and it drives Intent.

## Options

    agents    regenerate AGENTS.md from project state, via `intent agents sync`
    all       every option above, one pass

## What agents does

It warns before it runs `intent agents sync`: `AGENTS.md is generated -- a hand edit is lost here`. AGENTS.md looks like an ordinary Markdown file, and a hand edit to it is destroyed here without further comment. To change what it says, change the project state it is generated from.

It needs `intent` on PATH and refuses without it. If `intent agents sync` fails, its exit status is passed through and a second warning says AGENTS.md was NOT regenerated, so a failed run cannot read as a successful one.

`docs agents` is a sealed gate, run at the project root: its verdict goes to a log file (suffix `DOCSAGENT`) rather than scrolling past, and an empty verdict file means it passed.

## What is not here

Generated outputs that differ between projects, such as shell completions, an index or a publish step, belong to the project: options under `commands.docs.options`, which join `all`, or a handler at `bin/.devbin/cmd/docs`, which replaces the built-in `docs`, `agents` included. `treeindex` is no longer shipped: Intent v3 retired `intent treeindex`, so it could never succeed.
