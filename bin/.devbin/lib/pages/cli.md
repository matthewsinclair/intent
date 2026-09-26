    bin/devbin cli [<arg>...]

Run this project's own command-line interface, a mix task. devbin offers `cli` in a project whose `project.languages` declares elixir, and runs it at the project root, from whichever directory of the project you call it:

    bin/devbin cli <arg>...    mix <task> <arg>...

`<task>` is `project.cli_task` in `bin/.devbin/config.yaml`, or `<name>.cli` when that is not set, `<name>` being `project.name` exactly as written: a project named `my-app` runs `mix my-app.cli`. Where the task is named otherwise, set the key:

    project:
      name: my-app
      cli_task: my_app.cli

`bin/devbin repl` runs the same task with `repl` after it, so the one key serves both.

`-h` and `--help` go to the task like any other argument; devbin's help for `cli` is `bin/devbin help cli`. A shared option that devbin reads itself, listed by `bin/devbin help`, is taken by devbin and not passed on. devbin ships `cli` with no options, so `all` is not devbin's here: a first argument of `all` is passed to the task like any other.

devbin replaces itself with mix rather than running it beneath itself, so the exit status is the task's own.

## Changing it

    project.cli_task         the task, for cli and repl alike
    commands.cli.run         a command line of the project's own, run at
                             the project root in place of devbin's
    bin/.devbin/cmd/cli      a handler, when a command line cannot say it
    commands.cli.enabled     false stops offering it

A `run:` line is split into words and never handed to a shell. Once the project declares `commands.cli.run` or a handler, `cli` is the project's, and `bin/devbin help cli` shows the project's own page at `bin/.devbin/help/cli.md`, or a generated answer, in place of this one.
