    bin/devbin version [show|set <version>|bump <major|minor|patch>|sync|check]

Show, set, bump, sync and check the project version.

## Sub-commands

    (bare)         the identity line: name, version, commit, and runtime digest
    show           the bare number, for scripts
    set <v>        write <v> to the version file and every declared sidecar
    bump <part>    advance major, minor or patch, then write as set does
    sync           re-write the CURRENT version everywhere, healing drift
    check          report drift and write nothing; exits non-zero on any

The bare form is the line `bin/devbin --version` prints. Its commit carries `+dirty` when `git status` lists anything, an untracked file included, and `+unreadable` where git names the commit but cannot give the tree's status, as over an index it cannot read. Its runtime digest reads `UNREADABLE` when devbin's runtime cannot be read whole, a directory `find` cannot read or can list but not search, or a file that cannot be opened, and the error names it: a digest of part of the runtime would name another one. `set` takes X.Y.Z only. These are sub-commands, not options, so there is no `version all`: show, then set, then bump, then sync, in one run, would make no sense.

## Where the version lives

The version file is `project.version_from` (default `VERSION`). Sidecars are the other files carrying the same number, listed in `project.version_sidecars`. Each is read and written by recipe: a file holding only the version, `mix.exs`, `Cargo.toml`, `package.json`, or a `*.plist` (macOS only). A recipe rewrites the one line devbin reads the version from, and no other: in a `mix.exs` the first `version:` line, in a `package.json` the first `"version"` key, and in a `Cargo.toml` the `[package]` or `[workspace.package]` table's `version`, which comes first in a manifest laid out as cargo writes one. A `Cargo.toml` whose first version line is elsewhere, a dependency's say, or that gives its version another way, such as `version.workspace = true`, holds no version devbin reads. With no sidecars declared, `check` says so rather than implying it compared something:

    version: <version>, NO sidecars declared -- nothing was compared

That exits 0: a single version file is an ordinary setup, not a fault. A sidecar that is missing, or that devbin has no recipe for, fails `check`, and `set`, `bump` and `sync` refuse it before writing anything. They refuse the same way, naming the file and the key that declares it, a version file or sidecar devbin cannot read or cannot write, and a `Cargo.toml`, `mix.exs`, `package.json` or plist that holds no version devbin reads, which its recipe cannot rewrite. Whether devbin can write a file is asked by what its recipe does: a file of the bare kind, version file or sidecar, or a plist, is written in place, so it must be writable; a `Cargo.toml`, `mix.exs` or `package.json` is rewritten as a copy renamed over it, so a read-only one is still rewritten and keeps its mode, and one in a directory that takes no new file is refused. `check` asks the same of a sidecar that has drifted or is empty: where `sync` could not write it, `check` says why after the version it read, and names no `sync`. A sidecar `check` cannot compare is listed as `not compared`, with the reason.

## With no version to read

When the version file is not there, or holds nothing devbin can read as a version, `show` prints nothing on stdout and exits 1, saying on stderr why, naming the file and `project.version_from`, and what to do: a script reads `show`'s line as the number, and the word `unknown` read as one would name a tag `vunknown`. `check`, `bump` and `sync` exit 1 the same way, since there is no version to compare a sidecar with or to start from. doctor notes it in the same words, and a release cut's pre-flight refuses it, so a cut never starts a version file as a side effect. The bare form and `--version` still print their one line, with `unknown` where the version stands.

The remedy fits the file. For one that holds only the number, as `VERSION` does, it is `bin/devbin version set 0.1.0`. For a `mix.exs`, `Cargo.toml`, `package.json` or plist, each of which holds more than the version, it is to put the version in that file, or to point `project.version_from` at the file that holds it. A link to no file, something that is not a regular file, a file devbin may not read, and a plist off macOS, where PlistBuddy is not, are each named as what they are.

`set` starts a version file that is not there yet when it holds only the number, as `VERSION` does, in a directory devbin can write in: the preflight passes it, and the write creates it. Before it writes any sidecar, it refuses a missing version file it cannot start: one in a directory that is not there, a link to no file, and a missing `mix.exs`, `Cargo.toml`, `package.json` or plist, since one written holding only the number would be wrong. A missing sidecar is refused too, naming `project.version_sidecars`.

## The rules the writes follow

The version file is written LAST, after every sidecar. If a write fails partway, the version file has not moved, so re-running `bump patch` computes the same target and retries cleanly instead of bumping twice.

A write's own exit status is not taken as proof: PlistBuddy exits 0 when it cannot write. So `set`, `bump` and `sync` read every sidecar back before the version file is written, and the version file after, and exit 1 naming each file that does not read the version written. A sidecar that did not take leaves the version file unwritten, and the remedy is `version set` with the version named, whichever file it was.

`sync` always writes, even when nothing looks out of date. git shows what actually changed.

## What version never touches

`devbin_version` in `bin/.devbin/config.yaml` is not this project's version: it is the devbin runtime the project requires, which `bin/devbin doctor` checks. `version` never writes it.
