    bin/devbin clean <option> [--dry-run] [--force-blocked | --force] [--refuse-partial]
    bin/devbin clean all [--dry-run] [--force-blocked | --force] [--refuse-partial]

Remove the build trees this project can regenerate. Each option finds its trees by walking the whole project, prints every one it finds with its size, the ones it will not touch included, and unless told otherwise asks before it removes anything. An option is offered when `project.languages` declares its language, so a project that declares none of these three, and adds no option of its own, is not offered `clean` at all.

    rust      rust    cargo target/ trees, release/ included
    elixir    elixir  _build/ trees; the sources under deps/ stay
    swift     swift   Swift .build/ trees
    all       every active option in one pass

`bin/devbin clean -h` prints the usage with the options active here and exits 0; bare `bin/devbin clean` prints the same and exits 1, since it names nothing to remove. An option removes whole trees and nothing beside them, and walks from the project root whichever directory inside the project it is run from. `clean rust` takes `release/` as well as `debug/`, so the next build starts cold.

## What it removes, and what it leaves

A `rust` candidate is a directory named `target`, or a directory of any name holding cargo's `.rustc_info.json`, or a `CACHEDIR.TAG` that names cargo as its writer; the tag is a convention other tools share, and theirs are not candidates. An `elixir` candidate is named `_build`, and a `swift` one `.build`. The walk has no depth limit and skips `.git`, `node_modules` and `.direnv`. It does not look inside a candidate it found by its name, since a tree inside a tree is part of it; a directory named like a candidate that holds its own manifest is a crate, not a tree, and is walked like any other directory, as is a candidate found by its marker, which has no name to stop at. A tree outside the project root is never a candidate, so one cargo is told to build outside it, by `CARGO_TARGET_DIR`, `CARGO_BUILD_TARGET_DIR` or a config's `build.target-dir`, is left as it is, and `fullcycle` refuses to run over one: every path is carried whole from the walk to the removal, whatever characters its name holds, and every path the option prints itself is shell-quoted when it holds any byte that is not printable ASCII; a path that is not beneath the project root stops the run as a devbin defect, removing nothing. The one program the walk runs, to ask whether a directory named like a tree holds its own manifest, and whether a directory missing a search bit can be searched, is `/bin/sh`, named whole; nothing it runs is found through PATH, so a project's own `test` there is never run. A candidate inside another is part of it, not a second tree.

A candidate is removed only when its manifest sits beside it, in the same directory: `Cargo.toml`, `mix.exs` or `Package.swift`. One without is SKIPPED and listed with its size, because a build tree with nothing beside it to rebuild from may be the only copy; it is given an empty `.metadata_never_index` file if it has none, meant to keep Spotlight from indexing it, and the run says so when that file cannot be written; a `--dry-run` says whether a real run would, and writes nothing. A directory named `target` that holds no cargo `CACHEDIR.TAG`, no `.rustc_info.json`, no `.fingerprint` directory under `debug/` or `release/`, and no per-node target directory of its own (below) is listed as ignored and not touched.

A candidate found by its marker that holds its own manifest is not a build tree, whatever markers it carries: `cargo build --target-dir .` writes cargo's markers into the crate itself, and mixes its output with the sources. It is listed as built in place, and is never removed or marked, and the trees beneath it are still found. So is the project root, whatever it holds.

A cargo tree holding a cargo target directory of its own, such as `target/cc` from one session's own `CARGO_TARGET_DIR`, or one a tool keeps there, has each listed with its size and a warning that the tree is shared: removing it costs each of them a full rebuild. A directory directly inside it counts when it holds cargo's `.rustc_info.json`, which a target-triple directory such as `target/aarch64-apple-darwin` does not. A `target` whose only cargo output is in such directories, as after `CARGO_TARGET_DIR=target/cc cargo build`, is a tree all the same. Nor does a target directory cargo built under `CARGO_CACHE_RUSTC_INFO=0`, so that one is not listed and raises no warning.

## When the walk cannot read everything

A directory the walk cannot read is named by `find`'s own error, and one it can read but not search as `cannot search <dir> -- nothing beneath it was looked at`, as every walk that descends names it, since macOS's `find` passes over such a directory without a word; a `CACHEDIR.TAG` it cannot read is named by `grep`'s error, since that may be a tree's only marker; a tag beside a `.rustc_info.json` is not read at all, since that file already says the tree is cargo's. One such directory is not recognised, a declared limit: one whose mode bits let you search it and whose access control list denies you, which macOS's `find` passes over silently, as it always has, since the walks read mode bits and not access lists. A candidate that cannot itself be read, whichever option found it, is named and listed apart as one that could not be read, neither removed nor ignored; so is a `rust` candidate with no marker of its own when a directory inside it, which could have held one, cannot be searched -- a `target` another tool made, holding a directory it locked, included. A tree beneath any of these may not have been seen, so the scan is partial. The option says so before it asks or removes anything, still handles what it did find as asked, listing, asking and removing as it otherwise would, and exits 1, `--dry-run` included. A partial clean is not a clean, and a zero would let `clean && build` build on trees nobody looked at. A path that vanishes while the walk reads it, as files do in a build running beside it, or a candidate that vanishes before it is classified, does not make the scan partial.

`--refuse-partial` makes a partial scan a refusal instead: nothing is removed or marked, and it exits 1; with `--dry-run` it says at once, before listing anything, that a real run would refuse, and stops there, exit 1. `fullcycle` passes it to its clean phase, since a cycle that removed some trees and then stopped would leave them unbuilt.

## A tree PATH resolves into is refused

Before anything is removed, each tree is checked against PATH for a PATH entry inside the tree, and for a command that PATH resolves into it, symlinks followed; the commands looked up are the names of the executables at the top of the tree and in every `debug/` and `release/` within two levels of it: its own, a per-node directory's such as `target/cc`'s or a target triple's, and a triple's inside a per-node directory. A PATH entry, or a command PATH names, given as a relative path is read from the project root. What it finds is listed, and the option refuses, exit 1, having removed nothing, because those commands would stop resolving until rebuilt. Rebuild and re-point them first, or accept the risk by name:

    --force-blocked   accept it, and still ask before removing
    --force           accept it, and remove without asking

A tree removed this way leaves such a link dangling.

## Asking, and running without a terminal

Without `--force`, an option with trees to remove asks `proceed? [y/N]` on the terminal. Only `y` or `yes`, in any case, removes. Any other answer, or end of input, prints `left alone.`, removes nothing and exits 0, or 1 over a partial scan.

It asks only when the standard input of `bin/devbin` is a terminal. Where it is not, as in CI or with input piped in, the option refuses instead, exit 1, and removes nothing: `--force` removes without asking, and `--dry-run` shows what a run would do.

## --dry-run

`--dry-run` prints the whole scan, unless `--refuse-partial` meets a partial scan (above), changes nothing outside `tmp/clean/`, where every run's log goes, and never asks. Where a real run would refuse over a PATH link or for want of a terminal, it says so and still exits 0, and when it found trees to remove it ends by saying which a real run would do: refuse, ask first, or remove without asking. Over a partial scan it exits 1, as a real run does. A near miss such as `--dryrun` is refused, exit 1, rather than read as a real run: `--dry-run`, `--force`, `--force-blocked` and `--refuse-partial` are the only arguments devbin's options accept, besides devbin's own gate flags such as `--purge`.

## all

`bin/devbin clean all` runs every active option not marked `in_all: false`, those named in config first and then the rest in the order above, and does not stop at the first failure. Each option scans, asks and seals on its own, so a terminal is asked once per option with trees to remove. It ends with a count of the options that ran, were inactive here and were skipped, then `FAILED:` naming each failed option and its seal, or `all clean options passed`. It exits 1 if any option failed, or if none ran.

## The verdict is a file

Each option devbin ships runs as a sealed gate. Its output goes to `tmp/clean/<stamp>.<SUFFIX>.out` under the project root, with an `.errors` companion that is empty exactly when a completed run succeeded, and `verdict: <path>` names it. `<SUFFIX>` is `CLEANRS`, `CLEANEX` or `CLEANSW`, and `LATEST_<SUFFIX>.out` and `.errors` follow the newest run. `--purge` after one of these options also deletes its older runs, keeping the newest five, or `RUNLOG_KEEP_RUNS` of them.

## Exit codes

    0   removed; nothing to remove; declined at the prompt; a --dry-run scan,
        each over a scan that read everything
    1   the scan could not read everything: what it found was handled as
        asked, and the paths it could not read are named
    1   refused before removing anything: PATH resolves into a tree, no
        terminal to ask on, a partial scan under --refuse-partial, an option
        or argument it does not know, or a PATH it cannot read reliably
    1   --dry-run --refuse-partial over a scan that could not read
        everything: it says a real run would refuse, and lists nothing
    1   a devbin defect -- a path outside the project, a sort that
        failed, or a verdict clean does not know -- and nothing was removed
    1   a tree could not be removed; any removed before it stay removed
    1   bare bin/devbin clean, which names no option
    2   clean, or the option asked for, is not offered in this project
    4   run from outside the project this bin/devbin belongs to

## Changing it in a project

In `bin/.devbin/config.yaml`:

    commands:
      clean:
        options:
          swift:
            enabled: false     # not offered here
          elixir:
            in_all: false      # runs by name, left out of `all`
          assets:              # a new option, sealed as ASSETS
            run: scripts/clean-assets
            log: true

A new option is offered whatever the languages, and is sealed only if it declares `log:`. `clean all` passes each flag it is given, `--dry-run`, `--force`, `--force-blocked` and `--refuse-partial`, on to every option it runs, so an option a project adds must honour all four, or refuse the ones it cannot. A `run:` on one of devbin's options replaces that option's code entirely: none of the checks, refusals or prompts above run for it.

A handler at `bin/.devbin/cmd/clean` replaces the whole command, `all` included, and `bin/devbin help clean` no longer shows this page there.

## What is active here

Below this page, `bin/devbin help clean` prints an "In this project:" block: where the command and each active option resolve from, then the options not offered here and why.
