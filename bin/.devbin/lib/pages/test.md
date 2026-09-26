    bin/devbin test <option> [args...]
    bin/devbin test all

The project's test gates. Each option devbin ships exits non-zero when it fails and seals its verdict in a file. An option is offered when `project.languages` declares its language; `bin/devbin test -h` lists the options active here.

    mix      elixir  mix test
    credo    elixir  mix credo --strict
    rust     rust    cargo test
    swift    swift   xcodebuild test
    lua      lua     busted
    shell    shell   bats tests
    all      every active option in one pass

Arguments after one of these options are added to its command line, apart from devbin's own flags: `bin/devbin test mix --trace` runs `mix test --trace`.

## The verdict is a file

A sealed gate writes `tmp/test/<stamp>.<SUFFIX>.out` under the project root, with an `.errors` companion beside it. `<SUFFIX>` names the gate, eg `MIX`, and `<stamp>` is the UTC date and minute, with seconds and then a counter added on a collision. The `.out` log opens with a record of the command line devbin ran, one argument per line.

The `.errors` file starts as an in-flight marker and is sealed when the run ends, so it is EMPTY exactly when a completed run was green: a killed run keeps the marker, and a failed run always leaves its failures or its log tail. Each run prints `verdict: <path>` naming its own seal, and `LATEST_<SUFFIX>.out` and `.errors` follow the newest run. `--purge` after the option deletes that gate's older runs, keeping the newest five, or `RUNLOG_KEEP_RUNS` of them.

## An empty run fails

A run whose output shows it examined nothing (zero tests, or zero source files for credo) is PROMOTED TO FAILED, and the reason is written into the seal: a filter that matches nothing exits 0 and would otherwise read as a pass. Pass `--allow-empty` after the option when an empty run is intended, or declare `grammar: generic` on the option in config to stop the count. `lua` is never promoted, because devbin cannot count its output, and neither is output with no summary line to count.

## all

`bin/devbin test all` runs every active option not marked `in_all: false`, those named in config first and then the rest in the order above, and does not stop at the first failure. It ends with a count of the options that ran, were inactive here and were skipped, then `FAILED:` naming each failed option and its seal, or `all test options passed`. The summary is a convenience; the seals are the record. It exits non-zero if any option failed, and refuses a verdict if no option ran.

## Adding, replacing and dropping options

In `bin/.devbin/config.yaml`:

    commands:
      test:
        options:
          e2e:               # a new option, sealed as E2E
            run: mix test --only e2e
            log: true
            grammar: mix
            in_all: false    # runs by name, left out of `all`
          mix:               # a new command line; keeps MIX and its grammar
            run: mix test --exclude e2e
          credo:
            enabled: false   # not offered here

A new option is offered whatever the languages, and is sealed only if it declares `log:` (`true`, or a suffix of its own). `grammar:` names the reader that extracts its failures and counts what it ran, one of `mix`, `credo`, `cargo`, `xcodebuild`, `lua`, `shell`, `critic` or `generic`; `generic` is the default, and it and `lua` count nothing. `log: false` beside a `run:` of the option's own runs it unsealed; `log:` alone changes nothing, and `bin/devbin doctor` fails it. A `run:` is split into words and never given to a shell, so quotes, `$`, `;` and redirections are literal. `bin/devbin devbin` names the annotated reference config, which lists every key.

## What is active here

Below this page, `bin/devbin help test` prints an "In this project:" block: where the command and each active option resolve from, then the options not offered here and why. `bin/devbin help --why` shows the same for every command. Asking for an option that is not offered here is refused with that reason.
