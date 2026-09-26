    bin/devbin precommit [<option>|all] [args...]

Run this project's pre-commit gate. devbin ships the name `precommit` and nothing behind it: what a pre-commit gate checks belongs to the project, so each project declares what `precommit` runs (devbin design D18, tier 4).

It is also opt-in (devbin design D22): absent until a project asks for it, with `commands.precommit.enabled: true` or by filling it as below. `enabled: false` switches it off again, whatever fills it.

## Until the project fills it

Run with nothing behind it, it refuses on stderr, saying it `resolves, but nothing here runs it` and naming what would make it run. It exits 3, devbin's code for a command that resolves but has nothing to run. `-h` and `bin/devbin precommit all` get the same refusal. None of these writes anything.

That is not a fault: `bin/devbin doctor` notes it as offered with nothing to run, a note rather than a failure.

## Filling it

Any of these fills it, and switches it on with or without `enabled: true`:

    commands.precommit.run          one command line, run at the project root
    commands.precommit.options.<o>  named checks, each with its own run:, plus
                                    a synthesised `all` that runs every one
    bin/.devbin/cmd/precommit       an executable handler, for what a command
                                    line cannot say

`bin/devbin new precommit` scaffolds that handler, which exits 3 until it is written, and a page for it at `bin/.devbin/help/precommit.md`.

A `run:` line is split into words and never handed to a shell, so quoting, pipes and redirection need a handler. Arguments after `precommit` are added to the end of the line, except `all` when there is no option to run, none declared or every declared one switched off: `bin/devbin precommit all` then says it has no options to run, names `bin/devbin precommit` as what runs the line, and exits 1. A handler is handed every argument, `all` included when there is no option to run, and a leading word that names a sub-handler at `bin/.devbin/cmd/precommit.d/<word>` runs that sub-handler with the rest.

Declare `commands.precommit.log: true` beside the `run:` and it becomes a sealed gate: its output is logged to `tmp/precommit/<stamp>.PRECOMMIT.out` under the project root, beside an `.errors` file that is empty exactly when a completed run passed, it prints `verdict: <path>`, and it exits with the command's own status. devbin's own flags, such as `--purge`, are then devbin's and do not reach the command.

With options declared and neither a `run:` nor a handler, a bare `precommit` prints its usage and exits 1, because naming no option is a usage error, and `precommit --help` prints the same usage and exits 0. `precommit all` runs every active option not marked `in_all: false`, in the order config declares them, does not stop at a failure, and exits non-zero if any failed. With a handler as well, each option is passed to the handler as its first word and its `run:` is not used, so `all` calls the handler once per option.

## No git hook

Running it when you commit is not devbin's to arrange: devbin installs no git hook, for `precommit` or for any other command.

## When this page stops answering

Once a `run:` or a handler fills it, the project owns `precommit`, and `bin/devbin help precommit` prints the project's own `bin/.devbin/help/precommit.md` instead, or, until that exists, a generated answer saying where the command resolves from and where to write that page. Declared options leave this page in place, and the lines below it list them. A project's own page replaces this one whenever it exists.
