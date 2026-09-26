    bin/devbin which [<option>|all] [args...]

Report which binary or build a name resolves to. devbin ships the name `which` and nothing behind it: a project's binaries and builds are its own, so each project declares what `which` runs (devbin design D18, tier 4).

It is also opt-in (devbin design D22): absent until a project asks for it, with `commands.which.enabled: true` or by filling it as below. `enabled: false` switches it off again, whatever fills it.

## Until the project fills it

Run with nothing behind it, it refuses on stderr, saying it `resolves, but nothing here runs it` and naming what would make it run. It exits 3, devbin's code for a command that resolves but has nothing to run. A name to look up gets the same refusal, as do `-h` and `bin/devbin which all`. None of these writes anything.

That is not a fault: `bin/devbin doctor` notes it as offered with nothing to run, a note rather than a failure.

## Filling it

Any of these fills it, and switches it on with or without `enabled: true`:

    commands.which.run          one command line, run at the project root
    commands.which.options.<o>  named lookups, each with its own run:, plus
                                a synthesised `all` that runs every one
    bin/.devbin/cmd/which       an executable handler, for what a command
                                line cannot say

`bin/devbin new which` scaffolds that handler, and a page for it at `bin/.devbin/help/which.md`; the handler exits 3 until it is written.

A `run:` line is split into words and never handed to a shell, so quoting, pipes and redirection need a handler. Arguments after `which`, such as the name to look up, are added to the end of the line, except `all` when there is no option to run, none declared or every declared one switched off: `bin/devbin which all` then says it has no options to run, names `bin/devbin which` as what runs the line, and exits 1. A handler is handed every argument, `all` included when there is no option to run, and a leading word that names a sub-handler at `bin/.devbin/cmd/which.d/<word>` runs that sub-handler with the rest.

With options declared and neither a `run:` nor a handler, a bare `which` prints its usage and exits 1, because naming no option is a usage error, and `which --help` prints the same usage and exits 0. `which all` runs every active option not marked `in_all: false`, in the order config declares them, does not stop at a failure, and exits non-zero if any failed. With a handler as well, each option is passed to the handler as its first word and its `run:` is not used, so `all` calls the handler once per option.

## When this page stops answering

Once a `run:` or a handler fills it, the project owns `which`, and `bin/devbin help which` prints the project's own `bin/.devbin/help/which.md` instead, or, until that exists, a generated answer saying where the command resolves from and where to write that page. Declared options leave this page in place, and the lines below it list them. A project's own page replaces this one whenever it exists.
