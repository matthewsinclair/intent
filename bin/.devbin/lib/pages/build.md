    bin/devbin build [<option>|all]

Build this project's targets. devbin ships the name `build` and nothing behind it: projects build different things, so each project declares what `build` runs (devbin design D18, tier 4).

## Until the project declares one

Run with nothing declared, it refuses and says what would make it run (`<name>` is the project's name):

    <name>: bin/devbin build resolves, but nothing here runs it.
           devbin ships the name only -- declare commands.build.run or add bin/.devbin/cmd/build

It exits 3, devbin's code for a command that resolves with nothing behind it. That is not a fault, and `bin/devbin doctor` reports it as a note, not a failure.

## Giving a project a build

Any of these gives it one:

    commands.build.run            one command line, run at the project root
    commands.build.options.<o>    named targets, each with its own run:, plus
                                  a synthesised `all` that runs every one
    bin/.devbin/cmd/build         a handler, when a command line cannot say it

A `run:` line is split into words and never handed to a shell, so quoting, pipes and redirection need a handler.

With options declared and neither a `commands.build.run` nor a handler, a bare `build` prints its usage and exits 1: naming no target is a usage error, because `build && deploy` must not read a run that built nothing as success. `build --help` prints the same usage and exits 0.

## Why it never guesses

A default `build` would be right for one project and quietly wrong for the rest, and a build that runs the wrong thing and exits 0 looks exactly like a build that worked.
