    bin/devbin check <option> [args...]
    bin/devbin check all

Gates that examine the project. Each option devbin ships exits non-zero on a breach and seals its verdict in a file. An option is offered when `project.languages` declares its language, and an `any` option is, unless the tree lacks what it needs (`critic`, below); `bin/devbin check -h` lists the options active here.

    compile     elixir  mix compile --warnings-as-errors --force
    format      any     each declared language's formatter, in check mode
    deps        elixir  mix deps.unlock --check-unused
    clippy      rust    cargo clippy --all-targets -- -D warnings
    autotests   rust    no crate auto-discovers more than one test target
    targetdir   rust    cargo builds into the shared target directory, or each crate's own
    toolchain   any     elixir and erlang pins in .tool-versions match what runs
    critic      any     intent critic, per language the Intent config declares
    all         every active option in one pass

- `format` runs `mix format --check-formatted`, `cargo fmt --check` and `swift-format lint --strict` for whichever of elixir, rust and swift are declared, skipping one whose `fmt` option is disabled; nothing else is checked, Markdown included. A missing formatter fails it, and so does a Swift file list `find` could not finish, which it names, a directory it can list but not search included.
- `toolchain` compares the elixir and erlang pins in `.tool-versions` with the running toolchain, and with a Dockerfile's `ARG ELIXIR_VERSION` and `ARG OTP_VERSION`. With no `.tool-versions`, or no elixir or erlang pin in it, it passes and says so.
- `critic` needs `intent` and `jq`. It runs `intent critic <lang>` for each language the project's Intent config declares that has a headless critic, whatever `project.languages` says, over staged files unless given arguments (eg `--files <path>`); nothing staged passes, a project that is not a git checkout fails, saying so, and where git cannot say what is staged it fails with git's reason, where git gives one. In a project with no `intent` directory it is inactive, as an option whose language is undeclared is, so `all` counts it inactive and `bin/devbin help --why check critic` says why; declaring `commands.check.options.critic` runs it anyway. A project with an `intent` directory and no Intent config fails it, naming the config.
- `autotests` counts a crate's test targets by cargo's own discovery rule -- each `.rs` entry directly in its tests directory that is not dotted and not a real directory, whatever it is a link to, and each subdirectory there, not dotted and not a link, that holds a `main.rs`, which is asked of the filesystem as cargo asks it, so a `Main.rs` counts where the disk folds case -- and fails a crate with more than one while cargo's `autotests` is on, or with any under `autotests = false` and no `[[test]]` declared. A tests directory reached through a link is counted through it. One it cannot read fails the check, named by `find`'s own error, and so does an undotted subdirectory there that it cannot search, which is named.
- `targetdir` fails when `CARGO_TARGET_DIR` builds anywhere but the shared tree, or `CARGO_BUILD_TARGET_DIR` does while `CARGO_TARGET_DIR`, which cargo puts first, is unset, or either is set but empty, which cargo refuses. The shared tree is the project's `target` with a `Cargo.toml` beside it, the test `clean` removes a tree by. With no `Cargo.toml` at the project root there is none: `clean` never removes `<root>/target` there and `fullcycle` fails a cycle built into it, so either variable set at all fails, and each crate builds into its own workspace's `target`. Where a cargo config names `target-dir` or `include`, cargo is asked where each crate builds, run from the crate's directory with neither variable in its environment, so a config is judged whatever the environment of whoever runs the check, and `jq` reads its answer. A crate that builds anywhere but its own workspace's `target` or the shared tree fails, the cause named: the project's config, or one nearer the crate, whose cause is not named. A fork the machine-wide config alone causes, asked of cargo in an empty directory, is reported, not failed, and so is a project config no crate builds with. With no crate here the machine-wide config is still reported, as cargo reads it: where it moves the build tree, that it moves none, or that it could not be measured. Each crate with such a config in its directory or above it is asked, and every crate when the config cargo reads in `CARGO_HOME`, its `config` before its `config.toml`, names one, a template's included, and one cargo cannot read fails the check in cargo's own words. A directory the walk for crates cannot read fails the check when a cargo config naming `target-dir` or `include` is in play: one with a crate beneath it that cargo answered builds elsewhere than its own `target` or the shared one, other than a fork the machine-wide config alone accounts for, which stays reported, or one with the unreadable directory beneath it, where a throwaway crate would build anywhere but that crate's own `target`, the shared one or the machine-wide one: cargo is asked from the directory's parent, or from inside it where it can be searched but not listed, reading every config that applies there, nearest first. A config deeper inside such a directory is not seen, as nothing there can be listed, and neither is one inside a directory that cannot be read at all. A directory no such config applies at puts none in play. One whose place cannot be read back is asked of every config, and a config above the project is then asked from the project root, so the project's own config is read first, as a crate here reads it. Where cargo cannot answer, or the machine-wide tree cannot be measured, as when a cargo config sits at or above `$TMPDIR`, the walk counts as in play, and the failure says which. So a workspace whose own config pins `target-dir = "target"` fails over an unreadable directory beneath it: a crate there builds into that workspace's `target` whether it is a member or a second workspace, and only what the directory hides tells which. The failure names the tree and says what clears it: make the directory readable to the user running the check, or move it out of the workspace. Otherwise the directory is reported, not failed. The machine-wide config is never in play: it sends a crate beneath the directory where it sends every crate on the machine, and is reported, not failed. A machine-wide config met again on the way up from a scratch directory under `$HOME` is still read as the machine's.
- `format`, `toolchain`, `autotests` and `targetdir` refuse any argument but devbin's own flags, such as `--purge`; every other option adds its arguments to its command line.

## The verdict is a file

A sealed gate writes `tmp/check/<stamp>.<SUFFIX>.out` under the project root, with an `.errors` companion beside it. `<SUFFIX>` names the gate, eg `COMPILE`, and `<stamp>` is the UTC date and minute, with seconds and then a counter added on a collision.

The `.errors` file starts as an in-flight marker and is sealed when the run ends, so it is EMPTY exactly when a completed run was green: a killed run keeps the marker, and a failed run always leaves its failures or its log tail. Each gate prints `verdict: <path>` naming its own seal, and `LATEST_<SUFFIX>.out` and `.errors` follow the newest run. `--purge` after the option deletes that gate's older runs, keeping the newest five, or `RUNLOG_KEEP_RUNS` of them.

## all

`bin/devbin check all` runs every active option not marked `in_all: false`, those named in config first and then the rest in the order above, and does not stop at the first breach. It ends with a count of the options that ran, were inactive here and were skipped, then `FAILED:` naming each failed option and its seal, or `all check options passed`. The summary is a convenience; the seals are the record. It exits non-zero if any option failed, and refuses a verdict if no option ran.

## Adding, replacing and dropping options

In `bin/.devbin/config.yaml`:

    commands:
      check:
        options:
          dialyzer:          # a new option, sealed as DIALYZER
            run: mix dialyzer
            log: true
          compile:           # a new command line; keeps COMPILE and its grammar
            run: mix compile --warnings-as-errors
          deps:
            in_all: false    # runs by name, left out of `all`
          critic:
            enabled: false   # not offered here

A new option is offered whatever the languages, and is sealed only if it declares `log:` (`true`, or a suffix of its own); `grammar:` names the reader that extracts its failures, `generic` by default. `log: false` beside a `run:` of the option's own runs it unsealed; `log:` alone changes nothing, and `bin/devbin doctor` fails it. A `run:` is split into words and never given to a shell, so quotes, `$`, `;` and redirections are literal. If config replaces what `fmt <lang>` runs, `check format` refuses until `commands.check.options.format.run` says how to check it. `bin/devbin devbin` names the annotated reference config, which lists every key.

## check vs fmt

`check format` is the formatting gate, and there is no `fmt --check`. `fmt swift` and `fmt md` skip a formatter that is not installed; `check format` fails instead, because a gate that examined nothing must not pass.

## What is active here

Below this page, `bin/devbin help check` prints an "In this project:" block: where the command and each active option resolve from, then the options not offered here and why. `bin/devbin help --why` shows the same for every command. Asking for an option that is not offered here is refused with that reason.
