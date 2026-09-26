    bin/devbin iex [<arg>...]

An interactive Elixir shell with the project loaded. devbin offers `iex` in a project whose `project.languages` declares elixir, and runs it at the project root, from whichever directory of the project you call it:

    bin/devbin iex <arg>...    iex -S mix <arg>...

So `bin/devbin iex phx.server` runs `iex -S mix phx.server`, which is also what `bin/devbin server --iex` runs. `-h` and `--help` are passed on like any other argument; devbin's help for `iex` is `bin/devbin help iex`. A shared option that devbin reads itself, listed by `bin/devbin help`, is taken by devbin and not passed on. devbin ships `iex` with no options, so `all` is not devbin's here: a first argument of `all` is passed on like any other.

devbin replaces itself with iex rather than running it beneath itself, so the exit status is iex's own.

## Changing it

    commands.iex.run         a command line of the project's own, run at
                             the project root in place of devbin's
    bin/.devbin/cmd/iex      a handler, when a command line cannot say it
    commands.iex.enabled     false stops offering it

A `run:` line is split into words and never handed to a shell. Once the project declares either of the first two, `iex` is the project's, and `bin/devbin help iex` shows the project's own page at `bin/.devbin/help/iex.md`, or a generated answer, in place of this one.
