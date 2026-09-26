    bin/devbin repl [<arg>...]

The project's REPL, under rlwrap. devbin offers `repl` in a project whose `project.languages` declares elixir, and runs it at the project root, from whichever directory of the project you call it:

    rlwrap -r -c -H <history> -s 2000 mix <task> repl <arg>...

`<task>` is the one `bin/devbin cli` runs: `project.cli_task`, or `<name>.cli` when that is not set. `-h` and `--help` are passed on like any other argument; devbin's help for `repl` is `bin/devbin help repl`. A shared option that devbin reads itself, listed by `bin/devbin help`, is taken by devbin and not passed on. devbin ships `repl` with no options, so `all` is not devbin's here: a first argument of `all` is passed on like any other.

devbin replaces itself with rlwrap rather than running it beneath itself, so the exit status is rlwrap's.

## History and completion

`<history>` is `$HOME/.<name>_repl_history`, `<name>` being `project.name`. To keep it elsewhere, set `<NAME>_REPL_HISTORY` in the environment, `<NAME>` being the name upper-cased with each `-` as `_`: a project named `my-app` reads `MY_APP_REPL_HISTORY`.

When `bin/completions/repl.txt` exists under the project root, devbin hands it to rlwrap as its completion word list, `-f <file>`, ahead of the other flags. devbin never writes that file; it is the project's to provide.

## Without rlwrap

With no rlwrap on PATH, the REPL still starts, as plain `mix <task> repl` with no history or completion, after a warning on stderr:

    <name>: rlwrap not found -- no history or tab completion (try: brew install rlwrap)

The exit status is then the task's own.

## Changing it

    project.cli_task         the task, for repl and cli alike
    commands.repl.run        a command line of the project's own, run at
                             the project root in place of devbin's; the
                             rlwrap wiring goes with it
    bin/.devbin/cmd/repl     a handler, when a command line cannot say it
    commands.repl.enabled    false stops offering it

A `run:` line is split into words and never handed to a shell. Once the project declares `commands.repl.run` or a handler, `repl` is the project's, and `bin/devbin help repl` shows the project's own page at `bin/.devbin/help/repl.md`, or a generated answer, in place of this one.
