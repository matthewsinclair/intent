    bin/devbin shell-init zsh [--autoload]
    bin/devbin shell-init bash

Emit a shell function, `dvb`, that walks up from the current directory to the nearest bin/devbin and runs it. A bin/devbin that is not executable stops the walk: `dvb` says so and returns 126 rather than reaching a parent project's. Nothing is vendored and nothing is configured.

## Using it

`shell-init` runs from any directory, so your shell rc can run it from the install Homebrew placed, the `devbin` on your PATH, which also gives `dvb` that install to fall back to (below):

    eval "$(devbin shell-init zsh)"     # ~/.zshrc
    eval "$(devbin shell-init bash)"    # ~/.bashrc

A launcher named by its full path, `/path/to/project/bin/devbin shell-init zsh`, emits the same walk.

Or, for zsh's autoload, write the function BODY to a file on your fpath:

    bin/devbin shell-init zsh --autoload > ~/.config/zsh/functions/dvb

The eval form defines `dvb() { ... }`. The autoload form emits the body alone, because zsh's autoload supplies the wrapper and a pasted definition would put a function inside a function. Re-run the command to update either one. Do not edit the output by hand.

## Shells

zsh and bash each have a body, and both are rendered from one source: the function is written in what the two shells share, so the line naming the shell is the only one that differs. `--autoload` is zsh's alone, since bash has no autoload. Any other shell is refused with that list, rather than handed a body that would be wrong in it. Bare `shell-init` is a usage error: it prints its usage on stderr, so a shell rc that names no shell evaluates nothing, and exits 1. `-h` prints the same usage on stdout and exits 0.

## When the walk finds nothing

The walk finds devbin through a project that already has one, so it cannot install into a project that does not. A `dvb` emitted by the install Homebrew placed falls back to that install, so in a new project `dvb install` seeds it from there:

    cd ~/some/new/project && dvb install

Ahead of any machine install, `dvb` falls back to `DEVBIN_SOURCE` when it is set: a devbin checkout or install, its root or its bin/devbin. A checkout is the maintainer's, and seeds a project with whatever that tree is mid-development. With it unset, `dvb` tries a machine install at `DEVBIN_INSTALL_PREFIX`, or else at the one baked in when the function was emitted: the install that emitted it, or the `install.prefix` the emitting project declares. An install Homebrew placed is baked in by its `opt` path, `$HOMEBREW_PREFIX/opt/devbin/libexec`, which follows `brew upgrade`, never by the versioned keg it runs from, which `brew upgrade` deletes. Otherwise it lists where it looked and returns 127. `DEVBIN_SOURCE` is the fallback's alone: `upgrade`, `vendor`, `doctor` and the update notice read the machine's switch instead -- `DEVBIN_INSTALL_PREFIX`, else the link `~/.local/bin/devbin` that `devbin use` writes, else the install Homebrew placed (see `bin/devbin help use`).

The fallback runs what you typed against that devbin, and that devbin decides. `install` and `shell-init` run, and so do the version commands that only report: `--version` and `-v`, `version` alone or with `show` or `check`, and any of them after `devbin`. Everything else is refused rather than acting on the wrong tree, `version set`, `bump` and `sync` included, since they write that project's version.
