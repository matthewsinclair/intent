    bin/devbin db <option> [<arg>...]
    bin/devbin db all

The local development database lifecycle. Each option devbin ships runs a mix task at the project root as a sealed gate: it exits non-zero when the task fails and seals its verdict in a file. The options are offered in a project whose `project.languages` declares elixir; `bin/devbin db -h` lists the options active here.

    setup      elixir  mix ecto.setup
    migrate    elixir  mix ecto.migrate
    reset      elixir  mix ecto.reset
    seed       elixir  mix run priv/repo/seeds.exs
    all        every active option in one pass, reset excepted

Arguments after one of these options are added to its command line, apart from devbin's own flags, such as `--purge`: `bin/devbin db migrate --step 1` runs `mix ecto.migrate --step 1`. A shared option that devbin reads itself, listed by `bin/devbin help`, is taken by devbin and not passed on. The task's stdin is `/dev/null`, not your terminal.

Bare `bin/devbin db` prints its usage and exits 1, because naming no option is a usage error; `bin/devbin db -h` prints the same usage and exits 0. An option that is neither shipped nor declared is refused with exit 1, and the refusal names the options offered here and `bin/devbin help db`.

## Never against production

When `MIX_ENV` is `prod`, `db` refuses before it runs anything, and exits 1. That covers `all`, every option and the command line, whether devbin ships the line or the project declared it in `bin/.devbin/config.yaml`:

    <name>: db refuses MIX_ENV=prod -- these commands are the LOCAL dev database lifecycle, and 'db reset' drops the database.
           What must run there belongs in a project handler at bin/.devbin/cmd/db,
           which replaces db entirely -- never behind a flag.

No flag lifts the refusal. A project handler -- a file at `bin/.devbin/cmd/db`, or one mapped with `commands.db.handler` -- replaces the built-in `db` entirely, options included, and is the one thing the refusal lets past.

## The verdict is a file

A run writes `tmp/db/<stamp>.<SUFFIX>.out` under the project root, with an `.errors` companion beside it. `<SUFFIX>` is `DBSETUP`, `DBMIGRATE`, `DBRESET` or `DBSEED`, and `<stamp>` is the UTC date and minute, with seconds and then a counter added on a collision. The `.out` log opens with a record of the command line devbin ran, one argument per line.

The `.errors` file starts as an in-flight marker and is sealed when the run ends, so it is EMPTY exactly when a completed run was green: a killed run keeps the marker, and a failed run always leaves its failures or its log tail. Each run prints `verdict: <path>` naming its own seal, and `LATEST_<SUFFIX>.out` and `.errors` follow the newest run. A failed run exits with the task's own status. `--purge` after the option deletes that option's older runs, keeping the newest five, or `RUNLOG_KEEP_RUNS` of them; `--purge-only` does the same without running the task.

## all

`bin/devbin db all` runs every active option not marked `in_all: false`, those named in config first and then the rest in the order above, and does not stop at the first failure. `reset` is not among them: devbin ships it marked `in_all: false`, so `db all` never drops the database unless the project sets `in_all: true` on `reset`. It ends with a count of the options that ran, were inactive here and were skipped, `reset` among them, then `FAILED:` naming each failed option and its seal, or `all db options passed`. It exits non-zero if any option failed, and refuses a verdict if no option ran.

## Adding, replacing and dropping options

In `bin/.devbin/config.yaml`:

    commands:
      db:
        options:
          rollback:          # a new option, sealed as ROLLBACK
            run: mix ecto.rollback
            log: true
          setup:             # a new command line; keeps DBSETUP
            run: mix ash.setup
          reset:
            in_all: true     # back into `all`, which then drops the database
          seed:
            enabled: false   # not offered here

A new option is offered whatever the languages, and is sealed only if it declares `log:` (`true`, or a suffix of its own). A `run:` is split into words and never given to a shell, so quotes, `$`, `;` and redirections are literal. `bin/devbin devbin` names the annotated reference config, which lists every key.

## What is active here

Below this page, `bin/devbin help db` prints an "In this project:" block: where the command and each active option resolve from, then the options not offered here and why. `bin/devbin help --why` shows the same for every command. Asking for an option that is not offered here is refused with exit 2 and that reason.
