    bin/devbin use [dev|opt|brew]

Choose which of devbin's trees this machine runs: the dev checkout, its opt install, or the install Homebrew placed. The choice is one link, `~/.local/bin/devbin`, and nothing else records it. No link means brew, so a machine that only ever installed devbin with Homebrew is on brew without running this.

    (none)  report the tree the link serves, by its tree line, and check
            that it is the devbin a login shell runs; changes nothing
    dev     point the link at this checkout's bin/devbin
    opt     point the link at the opt install, install.prefix in the
            checkout's bin/.devbin/config.yaml
    brew    remove the link, so Homebrew's own devbin answers

Each prints the tree line after it, `devbin <version> (<tree>, <commit>)`, which `--version` prints too. It names no path: `command -v devbin` says where the running copy lives.

## Where each one runs

`use dev` and `use opt` run in the dev checkout, the one tree that knows where it is and what its config names as `install.prefix`; an opt install and a Homebrew keg record neither, so from them both are refused, naming the command to run in the checkout. Once the link names the checkout, the `devbin` on PATH is the checkout, so `devbin use opt` works. `use brew` and a bare `use` run from any tree, the keg on PATH included.

A project with devbin vendored into it is not one of devbin's trees, so its `bin/devbin` does not offer `use`: it refuses with exit 6, naming `devbin use`, and its help leaves the command out. `use --help` and `use -h` there answer as `bin/devbin help use` does: refused, naming `devbin use`, with exit 2. The switch is the machine's, thrown from devbin itself, never from a project.

`use opt` refuses a prefix with no opt install, naming `install`, and one whose install has the manifest form written before the tree line, naming `install --force`. `use brew` removes only a link into one of devbin's trees: a link to anything else, or a file standing where the link belongs, is named and left for you to remove. It also refuses, removing nothing, when no Homebrew devbin would answer once the link is gone (the formula is not installed, its keg is not linked, Homebrew's bin is not on PATH, Homebrew's devbin is not executable, Homebrew's devbin is linked only through the link, or another devbin comes first on PATH), naming the link, the tree it serves, why, and what to do instead. With no link and no Homebrew devbin, a bare `use` says why, and gives the remedy for that cause: export `HOMEBREW_PREFIX` as `brew shellenv` exports it when it is unset or relative, or install devbin with Homebrew; only run from a dev or opt tree does it add `bin/devbin use dev` or `opt` in the checkout.

## The link is the one switch

Every project on the machine reads the same link. With no `--from`, a project's `bin/devbin upgrade` reads `DEVBIN_INSTALL_PREFIX` when it is set, else the tree the link leads to, else the install Homebrew placed, and so do `vendor`, `doctor` and the update notice. The link is read by its own path, never through PATH, so cron and a hook with no login PATH still find the tree it names. A project's manifest records the kind of tree its bytes came from, so an upgrade while the link names the checkout records `checkout`, with `+dirty` when the checkout is dirty.

No other verb points or removes the link.

## The shadow check

The link works only where `~/.local/bin` comes before Homebrew's bin on PATH. A bare `use`, and `doctor`, fail when the link is not the first `devbin` on PATH, naming what shadows it. `use dev` and `use opt` run the same check after pointing the link: when it is shadowed they exit 1 with the link left pointed, and say so, naming the link and what shadows it, so either fix PATH as that says or run `use brew`. With no link, which means brew, they fail when the first `devbin` on PATH is not Homebrew's, naming that copy, when Homebrew's would not run, and when no `devbin` is on PATH at all, each with the remedy `use brew` gives for the same cause. Directories are compared as they resolve, so a `~/bin` that links to `~/.local/bin` is the same directory, and a first `devbin` that is itself a link through the link, such as a `~/bin/devbin` made to `../.local/bin/devbin`, is the link. Only absolute PATH entries are searched: an empty or relative one answers from wherever a command is typed, so it shadows nothing.

The switch governs login shells. A process started without the login PATH, such as an app's hook or an MCP server that names `devbin` by its bare name, may run Homebrew's devbin while the link names another tree, and nothing here can see that.

## See also

    bin/devbin help install      publishing an opt install, and replacing it
    bin/devbin help upgrade      where a project's upgrade reads from
    bin/devbin help doctor       the tree and switch rows
