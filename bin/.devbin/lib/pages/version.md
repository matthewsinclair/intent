    bin/devbin version [show|set <version>|bump <major|minor|patch>|sync|check]

Show, set, bump, sync and check the project version.

## Sub-commands

    (bare)         the identity line: name, version, commit, and runtime digest
    show           the bare number, for scripts
    set <v>        write <v> to the version file and every declared sidecar
    bump <part>    advance major, minor or patch, then write as set does
    sync           re-write the CURRENT version everywhere, healing drift
    check          report drift and write nothing; exits non-zero on any

The bare form is the line `bin/devbin --version` prints. Its runtime digest reads `UNREADABLE` when devbin's runtime cannot be read whole, a directory `find` cannot read or can list but not search, or a file that cannot be opened, and the error names it: a digest of part of the runtime would name another one. `set` takes X.Y.Z only. These are sub-commands, not options, so there is no `version all`: show, then set, then bump, then sync, in one run, would make no sense.

## Where the version lives

The version file is `project.version_from` (default `VERSION`). Sidecars are the other files carrying the same number, listed in `project.version_sidecars`. Each is read and written by recipe: a file holding only the version, `mix.exs`, `Cargo.toml`, `package.json`, or a `*.plist` (macOS only). With no sidecars declared, `check` says so rather than implying it compared something:

    version: <version>, NO sidecars declared -- nothing was compared

That exits 0: a single version file is an ordinary setup, not a fault. A sidecar that is missing, or that devbin has no recipe for, fails `check`, and `set`, `bump` and `sync` refuse it before writing anything.

## Two rules the writes follow

The version file is written LAST, after every sidecar. If a write fails partway, the version file has not moved, so re-running `bump patch` computes the same target and retries cleanly instead of bumping twice.

`sync` always writes, even when nothing looks out of date. git shows what actually changed.

## What version never touches

`devbin_version` in `bin/.devbin/config.yaml` is not this project's version: it is the devbin runtime the project requires, which `bin/devbin doctor` checks. `version` never writes it.
