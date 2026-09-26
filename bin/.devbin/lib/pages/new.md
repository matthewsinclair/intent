    bin/devbin new <command> [--summary <text>] [--group <name>]

Scaffolds a new project command that resolves at once, with no config edit.

    --summary <text>  the one-line description help shows (default: a placeholder)
    --group <name>    the help heading it sits under (default: none, so it is
                      listed under Other)

## What it writes

    bin/.devbin/cmd/<command>       the handler, executable, with its header
    bin/.devbin/help/<command>.md   authored help, printed by
                                    bin/devbin help <command>
    <tests>/<command>.bats          a test stub, if this project has a bats suite

All three are yours: `bin/devbin upgrade` replaces only devbin's own runtime and never touches them.

It refuses when `bin/.devbin/cmd/<command>` already exists, and keeps a help file or test stub that is already there. A name is lower-case letters, digits, `-` and `_`, starting with a letter; `all` is reserved, because devbin synthesises it.

The test stub goes beside the first `.bats` file found under the project root, in sorted path order, and is skipped when there is none. A directory the search cannot read is named by `find`'s own error, and one it can list but not search, as at mode 0644, as `cannot search <dir> -- nothing beneath it was looked at`; `new` says a suite beneath it was not seen, and the stub still goes beside the first file found: `new` never invents a test directory, and never writes into devbin's runtime, which ships the template the stub is made from.

## It exits 3 until you write it

The scaffolded handler prints "not implemented yet", names its own path, and returns 3 -- "resolves, but is not built yet". Returning 0 from a command that does nothing is the false green devbin's gates exist to prevent, and a scaffold that did it would teach the habit at exactly the moment somebody is learning the tool.

## The header is the registration

    # devbin: summary = Ship it
    # devbin: group = Ship

Those two lines, in the handler's leading comment block, are the one place the command's summary and group are declared. Config can override them, with `summary:` and `group:` under `commands.<command>`; nothing else duplicates them. So a script dropped into `bin/.devbin/cmd/` appears in help with no config edit at all, as long as it is executable -- under the heading its header names, or under Other.

Commands devbin ships carry no such header: their wording lives in devbin's own catalogue, so the two sources can never both answer for one command.

## Sub-commands

Drop an executable at `bin/.devbin/cmd/<command>.d/<word>` and `bin/devbin <command> <word>` routes to it, at any depth. A word with no handler behind it stops the descent and reaches the handler above as an ordinary argument, so a command can take plain arguments beside its sub-commands. Help descends the same way: `bin/devbin help <command> <word>` prints `bin/.devbin/help/<command>.d/<word>.md` when it exists.

## When you want a gate instead

`new` scaffolds a handler, which is the right answer for anything with a conditional or a pipe in it: a `run:` line is split into words and never evaluated by a shell. If what you want is "run this command line and seal the verdict", declare it instead as an option under `commands.<command>.options.<name>` of a command with no handler of its own -- `run:` for the command line, `log:` to seal it, `grammar:` to read its failures -- and it joins that command's synthesised `all` in the same edit, unless it says `in_all: false`. An option declared under a command that has a handler is passed to that handler as its first word, and its `run:` is not used.

## See also

    bin/devbin devbin             names devbin's annotated reference config,
                                  and where its long-form guides are
    bin/devbin devbin templates   the templates new scaffolds from
