    bin/devbin server [--iex] [<arg>...]

Start the Phoenix development server. devbin offers `server` in a project whose `project.languages` declares elixir, and runs it at the project root, from whichever directory of the project you call it:

    bin/devbin server <arg>...          mix phx.server <arg>...
    bin/devbin server --iex <arg>...    iex -S mix phx.server <arg>...

`-h` and `--help` are passed on like any other argument; devbin's help for `server` is `bin/devbin help server`. A shared option that devbin reads itself, listed by `bin/devbin help`, is taken by devbin and not passed on. devbin ships `server` with no options, so `all` is not devbin's here: a first argument of `all` is passed on like any other.

devbin replaces itself with the server rather than running it beneath itself, so the exit status is the server's own.

## --iex

`--iex` runs the same server inside an IEx shell. It is a flag devbin owns and may come anywhere after `server`: devbin takes it out of the arguments and exports `DEVBIN_OPT_IEX=true`, which is what `server` reads to choose the second command line.

As devbin ships it, `server` is its only reader. Given to another command, as in `bin/devbin iex --iex`, it is refused with exit 2, naming the commands that read it; given a value, as in `--iex=true`, it is refused with exit 1. A project handler that reads `DEVBIN_OPT_IEX` becomes a reader when its command is listed under `options.iex.commands` in `bin/.devbin/config.yaml`.

Once the project owns `server`, what `--iex` does depends on how. A handler at `bin/.devbin/cmd/server` still reads it: devbin takes the flag and exports `DEVBIN_OPT_IEX=true` for the handler. A `commands.server.run` line is run as written, so devbin does not count it as a reader unless `options.iex.commands` lists `server`, which says the line reads `DEVBIN_OPT_IEX` itself. Not counted, `server` is treated as any command that does not read the flag: where nothing else here reads `--iex`, devbin does not take it at all and it reaches the line as an ordinary argument; where something else does, `server --iex` is refused with exit 2, naming the commands that read it.

## Changing it

    commands.server.run        a command line of the project's own, run at
                               the project root in place of devbin's
    bin/.devbin/cmd/server     a handler, when a command line cannot say it
    commands.server.enabled    false stops offering it, for an Elixir
                               project with no server

A `run:` line is split into words and never handed to a shell. Once the project declares either of the first two, `server` is the project's, and `bin/devbin help server` shows the project's own page at `bin/.devbin/help/server.md`, or a generated answer, in place of this one.
