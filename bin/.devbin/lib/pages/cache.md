    bin/devbin cache key <artefact> [--commit <sha>]
    bin/devbin cache get <artefact> [--commit <sha>]
    bin/devbin cache put <artefact>
    bin/devbin cache list [<artefact>]
    bin/devbin cache rm <key>
    bin/devbin cache gc [--keep <n>]

Keep this machine's built artefacts under a key made from the inputs a build reads, so a build whose inputs did not change pulls what it built before instead of building again. A project's `build` handler calls `get` before it builds and `put` after.

## The key

An artefact's key is the sha256 of the sorted lines `<path><TAB><object id>` over its input paths at a commit, the id being git's own for that path (a tree for a directory, a blob for a file), plus a `toolchain` line with the tool's version, a `platform` line with this host's platform word, and an `arch` line with its CPU architecture as `uname -m` names it, so an artefact built for one architecture is never pulled on another. The tool is asked its version where the build runs: cargo in `project.cargo_root`, where rustup reads a `rust-toolchain` file, every other tool at the project root. A commit that changes nothing the inputs hold keeps every id, so the key stands and the artefact is pulled. At HEAD, an input path with uncommitted changes is no commit's object: it has no key, `key` and `put` refuse it, and the build runs and records nothing. A key is printed whole.

## The declaration

    build:
      inputs:
        - Cargo.toml
        - Cargo.lock
        - src
      artefacts:
        app:
          tool: cargo
          path: target/release/app

`build.inputs` is the input paths every artefact reads; an artefact's own `inputs` narrows it where it reads fewer. An artefact's name starts with a letter or a digit, so none reads as an option. `tool` is one of the toolchain table's words (`cargo`, `elixir`, `swift`, `xcodebuild`), whose version the key names, so no project writes a version anywhere. `path` is what the build writes, read by `put` and written back by `get`. Every path is relative to the project root and written once, as git names it. A tool word outside the table is refused at config read, and so is a path that is empty, absolute, holds an empty, `.` or `..` segment, or lies in `.git` (in any case of its letters, as a case-folding filesystem reads it), so `get` never writes or removes anything outside the project or over it; and so is an artefact's `path` that is, holds or lies in any input path the block declares, compared in any case too, since a pull replaces what is at the path. `get` also refuses a path the repository tracks files at, in any case of its letters (git folds the ASCII ones).

## The registry

The registry's root is `DEVBIN_CACHE_HOME`, an absolute path; unset or empty, it is the XDG cache directory's `devbin`. An entry is `<root>/<project>/<artefact>/<key>/`, holding the artefact (a file, copied with its mode and times, and on macOS whatever extended attributes or ACL it holds, but no file flags; or a tar of a directory's contents: its files and their mode bits, no owner, no extended attributes, no ACLs, no file flags, written back at the path the declaration names then; a flag such as uchg would leave an entry nothing could replace or remove), `manifest.yaml` (the project, the artefact, the key and its lines, the commit and whether the tree was clean outside the inputs, the host, the clock, the size and the artefact's sha256) and `SHA256SUMS`.

A root that does not exist yet is an empty registry: `get` answers absent and the build runs, and `put` makes the root. A root that cannot be used (not absolute, a file, a link to nothing, not writable, or not to be made because the nearest path above it that exists is not a directory this user can write, or is a link to nothing) is said once on stderr, naming the root and why, before the key asks the toolchain anything, and the verb is refused: the registry is a cache, so its absence costs a build and nothing else.

`bin/devbin doctor` names the root it resolves, the default marked, and whether it can be written, and fails an input path the block declares that HEAD does not hold, since no key can read it.

## The answers

    key    the key, one line; refused with the reason
    get    `pulled <artefact> <key>` at exit 0, the artefact written to its
           path; `absent <artefact> <key>` at exit 1
    put    `put <artefact> <key>`
    list   `<artefact> <key>` for each entry of this project
    rm     `removed <artefact> <key>` for each, or `absent <key>`
    gc     `no release seal names a key: ...` when none does, then
           `removed <artefact> <key>` for each, `left <artefact> <name>: ...`
           for a temporary entry a put left behind, which gc removes none
           of, then `freed <n> bytes`

Whatever a verb cannot do, an argument it cannot read or a write it cannot finish among them, is a refusal: exit 7, the reason on stderr, the cache's own code in devbin's exit-code contract, apart from `get`'s 1 and the dispatcher's 2 and 3, so a build handler never reads a failure as absent. An argument it cannot read is refused with the usage, and `-h` or `--help`, wherever it stands, is the usage at exit 0. A `put` that stops leaves nothing under the root. An entry whose `SHA256SUMS` or manifest key does not verify is never pulled: it is removed with a line, `get` answers absent, and the build runs. `gc` keeps the newest entries per artefact, 3 unless `--keep` says, and every key the newest release seal, by its time, names on a `cache<TAB><artefact><TAB><key>` line; a seal it cannot read, or a run-logs directory it cannot list, stops it, removing nothing. A directory of the registry that `list`, `rm` or `gc` cannot list is refused, never read as empty.
