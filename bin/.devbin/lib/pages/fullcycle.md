    bin/devbin fullcycle [--dry-run] [--force]

From a total clean: remove the project's cargo build trees, `release/` included, rebuild, and run the tests. It is DESTRUCTIVE, and it is offered where `project.languages` declares rust.

## The three phases

    clean rust --force-blocked --force --refuse-partial   remove the cargo build trees
    build all, or build                                   rebuild, as this project builds
    test all, or test                                     run this project's tests

Each phase is a separate `bin/devbin` run, so it runs what that command runs in this project and seals its own verdict as it always does. The first phase to fail stops the cycle: the phases after it do not run.

The rebuild is the project's, because devbin ships nothing behind `build`. The build and test phases are what those commands run in this project, worked out from its config without running anything. Where the command has options, declared under `commands.<command>.options` or shipped by devbin, or a handler at `bin/.devbin/cmd/<command>` or under `commands.<command>.handler`, the phase is `<command> all`. Where a command-level `commands.<command>.run` line is all there is, the phase is the bare `<command>`, which runs that line: `all` is refused over it. A phase that the config alone shows cannot run is refused before anything is shown, asked or removed, with exit 1, because the clean would otherwise remove the trees and the cycle stop at that phase: nothing behind the command at all; a `run:` line with no words; a handler under `commands.<command>.handler` that does not resolve; options every one kept out of `all` by `in_all: false`; or any option `all` would run that has nothing to run it, whose devbin handler is missing from the install, or whose own `run:`, or the command-level `run:` it is handed to, holds no words. Each reason is given a line of its own, in the words `bin/devbin <command>` or `bin/devbin <command> <option>` would use. For `build` it opens `this project has no build phase the cycle can run` and prints the rest of the cycle to run by hand; for `test`, `nothing runs test in this project`. A build or test line that runs and fails is a failing phase, as any other is.

The clean phase always carries `--force-blocked`. Where a binary on PATH resolves into a tree about to be removed, `clean` refuses without it; fullcycle accepts that for you, because rebuilding what it darkens is its job and the checks after the phases verify the result. It always carries `--force` as well, because by then you have answered the question below or passed `--force`. And it always carries `--refuse-partial`: a clean that could not read the whole project refuses, removing nothing, because a cycle that stopped after removing some trees would leave them unbuilt. These three flags bind devbin's own `clean rust`. Where a project replaces it with a `run:` line or a handler of its own, that code receives them, and nothing enforces them.

A `CARGO_TARGET_DIR` the artefact check cannot examine is refused before anything is shown, asked or removed, exit 1: one that is relative, since cargo reads it from the directory it runs in, which fullcycle cannot know; one that holds a newline; one set but empty, which cargo itself refuses, but only once the clean has run; and `/` itself, or any run of slashes alone, which names no tree the check can read. The refusal shows the value shell-quoted. `./target` is relative too, and is refused like any other relative value, though it reads as the project's own tree: cargo reads it from wherever it runs. So is the per-session `target/<node>` that `clean` describes, spelled relative. So is an absolute value outside the project, or one inside it that reaches outside through a link: the clean does not remove a tree outside the project, so the release artefacts a previous cycle left there would read as this cycle's. So is the project root itself, a value naming `.` or `..` beneath a directory that does not exist yet, and one whose path runs through a file or a link that cannot be followed, which cannot be placed. Any other absolute value inside the project, a trailing `/` included, is examined. The clean finds a candidate by its name or its marker and classifies it by its marker, but removes a tree only where a `Cargo.toml` is beside it, and never under `.git`, `node_modules` or `.direnv`; so a tree inside the project may survive it. After the clean, each tree the check will examine is read: one whose `release/` still holds a binary is not compared, since that binary may be the last cycle's, and the check is incomplete and fails the run, naming the tree and what to do. For the tree `CARGO_TARGET_DIR` names, where no `Cargo.toml` is beside it, unset it, so cargo builds beside its manifest, where the clean removes the tree: with the crate nested below the project root, as at `native/cli`, the shared `target/` that `bin/devbin check targetdir` accepts is not one the clean removes, so every cycle after the first reads it as kept. For a `target/` the clean does not recognise as cargo's, by the same markers it classifies with, it says so. Otherwise, remove it or build into a tree `clean rust` removes, which `bin/devbin clean rust --dry-run` lists. A walk after the clean that cannot read everything fails the run too, since a tree beneath what it could not read may have kept its release artefacts unseen. The read is taken once, right after the clean: something beside the cycle that puts a release binary back between the clean and the check, such as another session building into the same tree, is read as this cycle's, and the cycle cannot tell. `bin/devbin check targetdir` holds the wider rule, refusing any value but the project's shared `target/`, an empty one included.

`CARGO_BUILD_TARGET_DIR` is judged the same way when `CARGO_TARGET_DIR` is not set, since cargo puts `CARGO_TARGET_DIR` first, and either over any config. With neither set, a cargo config can still move a crate's tree, as `build.target-dir` in a `.cargo/config.toml`, or in a file that one names with `include`. Where a config naming `target-dir` or `include` sits in a crate's directory, in a directory above it, or in `CARGO_HOME`, cargo itself is asked where that crate builds: `cargo metadata --offline --no-deps`, run from the crate's directory, which reads no network and leaves `Cargo.lock` alone, and `jq` reads its answer. The tree it names is judged and examined as above. Each crate with such a config in its directory or above it is asked, and every crate when `CARGO_HOME`'s config names one, a template or fixture among them included, so one cargo cannot read is refused before anything is removed, naming the crate's directory and cargo's own first error line, and so is cargo missing from `PATH`. A crate beneath a directory the walk cannot read is not asked about; devbin's own clean refuses such a directory, and where a project's own clean line does not, the walk after the clean fails the cycle over it. With no such config, cargo is not run and nothing changes. A build line that runs cargo from another directory, or passes `--target-dir` or `--config` itself, can build elsewhere, and the cycle cannot see it.

## It asks first

Without `--force`, it prints what it has and has not agreed to on your behalf, shows what the clean phase would remove, and asks once:

    Remove these and run the full cycle? [y/N]

Only `y` or `yes`, in any case, runs the cycle. Any other answer removes nothing and exits 1, and so does a run whose standard input is not a terminal. The preview is the clean phase's own command with `--dry-run` added, so it seals a `clean rust` log whatever the answer, and a preview that fails removes nothing and exits 1. `--force` skips the preview and the question.

## --dry-run

Prints the commit and dirty count a real run opens with, the three phase commands exactly as a real run would run them, and the consent notice unless `--force` is given. It asks nothing, runs no phase and writes no log of its own. Nothing behind `build` or `test` runs to work out the phases: they are read from the config. It does not guess how long the clean would take or how much it would free.

## What it checks after the phases

Three green phases are not the verdict, because a rebuild can pass without putting back what the clean removed.

Before the clean, it records every symlink in a PATH directory that resolves into the project, wherever its target is spelled from: an absolute path, a relative one, or one through another link. A PATH entry that is itself a link to a directory is searched as that directory. To stay fast over a large PATH, it follows only a link whose target, as written, holds the project's directory name, or any link in a PATH directory inside the project; a link that reaches the project only through another linked directory, with the project's name nowhere in its target, is not watched. After the phases, passed or failed, each that no longer resolves is listed as darkened by this cycle and fails the run. A link already dangling before the run is reported as such and not counted against it. A PATH directory the watch can search and cannot list, as at mode 0111, may hold a link it never sees, so the run says it watched only the directories it could read, `find`'s own error names the one it could not, and a run that would otherwise pass fails. A PATH directory the user running the cycle cannot search resolves nothing for that user and is passed over.

When every phase, and the restore below if it ran, has passed, it examines each `target/` directory under the project, or `CARGO_TARGET_DIR` when that is set, which must then be an absolute path; `deps/`, `_build/`, `node_modules/` and `.venv/` are not searched. A tree whose `debug/` holds a binary and whose `release/` holds none fails the run, so the rebuild has to include the release profile. It reads each tree's own `debug/` and `release/` only: a target triple's profiles, as `target/<triple>/release` after `cargo build --target <triple>`, are not read. When it read every tree and none holds a binary in either profile, the check says it is inconclusive and does not fail the run. A directory under the project that `find` cannot read, or can list but not search, may hold a tree the check never examined, so the check says it is incomplete, the error above names the directory, and the run fails. The same holds for a tree the check cannot reach or search, or whose `debug/` or `release/` it cannot list: the check names it, compares neither profile, says it is incomplete, and the run fails. A tree that does not exist, where the directory that would hold it can be searched, is named and passed over. A `target` that is a link to a directory is examined as the directory it names, once however many paths name it, and a directory that is itself a link is not searched for trees.

## Putting release binaries back

devbin never runs a release build on its own: whether one is safe to run in a tree is the project's call. A project that wants the cycle to repair the links it darkens names the command in `bin/.devbin/config.yaml`:

    commands:
      fullcycle:
        restore: <this project's release build>

It runs at the project root, and only when the build phase has passed and a watched link is dark after the phases, even if the tests failed. The line is split into words and never given to a shell, so quotes, `$`, `;` and redirections are literal. Undeclared, the cycle says what it could not put back. A failed restore fails a cycle whose phases passed.

## What it prints and writes

It opens with the commit it started from, how many files `git status` lists as changed (or that the project is not a git checkout), and how many PATH links it is watching. Each phase is followed by its time and exit code, and the run ends `FULLCYCLE FINISHED -- rc=<n>`. When a phase fails, it says what that left: trees removed and not rebuilt, a rebuild that passed before the failure, or a clean that may have removed some trees before it failed.

The run is a sealed gate: its log is `tmp/fullcycle/<stamp>.FULLCYC.out` under the project root, with an `.errors` companion that is empty exactly when the cycle finished green, and `LATEST_FULLCYC.out` and `.errors` follow the newest run. The log records the consent: `--force`, or the `y` and how long it took to give.

## Exit codes

    0    every phase and every check after them passed; also --help, and
         a --dry-run that was not refused
    1    nothing was removed: refused, declined at the prompt, no terminal
         to ask, or the preview failed. Or the phases passed and then a
         watched link went dark, a build tree has debug binaries and no
         release ones, or the check or the watch could not read everything
    2    not offered here: rust is not declared, or fullcycle is disabled

A phase that fails gives the run its own exit code. A restore that fails after passing phases gives the run the restore's exit code.

## Changing or replacing it

The phases follow the project: its own `clean rust`, and the build and test phases above, are what run, and `commands.fullcycle.restore` is the only key fullcycle reads itself. `commands.fullcycle.run` or a handler at `bin/.devbin/cmd/fullcycle` replaces the whole command, and `commands.fullcycle.enabled: false` drops it. It is a command rather than an option of `clean`, `build` or `test`, so none of their `all` runs includes it.

## See also

    bin/devbin help clean    the command the clean phase runs
    bin/devbin help build    the command the build phase runs
    bin/devbin help test     the command the test phase runs
