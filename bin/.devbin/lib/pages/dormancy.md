    bin/devbin dormancy

How many tests this project does NOT run: it runs the project's rust tests twice, as declared and with `--all-features`, and reports the difference. A dormant test sits behind a feature the declared line never enables: it is not failing and not deleted, and the declared line never runs it. `dormancy` is offered where `project.languages` declares rust, and it takes no arguments.

## The two legs

    leg 1    the line `test rust` runs here, eg  cargo test
    leg 2    that line with --all-features       eg  cargo test --all-features

Each leg runs that line itself, at the project root, and is sealed under `tmp/test/` as `test rust` is sealed: under `commands.test.options.rust.log` and `.grammar` where config gives `test rust` a `run:` line of its own, and otherwise under devbin's `RUST` suffix and cargo's grammar, since a `log:` beside no `run:` is read by nothing. A line declared unsealed, `log: "-"`, is refused before either leg: each leg is a sealed run, sealed as `test rust` is. Neither re-points `LATEST_RUST`: a probe about feature flags is not that gate's current verdict, so each leg is quoted by its own dated file. A leg that runs no test is a count of zero, not a failure: the legs measure, where `test rust` is a gate that refuses an empty run. `--all-features` goes where cargo reads it, before any `--` in the line, since what follows `--` goes to the test binaries. A leg's output is printed when that leg finishes. Two full test runs cost time, so dormancy is a command to run when you want the number and not a `test` option: `test all` never runs it.

Before the legs run, it prints the line it compares and where it came from, `commands.test.options.rust.run` when config declares one and devbin's catalogue otherwise, and that line with `--all-features` placed. It is the line `test rust` runs, and dormancy refuses, exit 1, before either leg when `test rust` runs something else or nothing: a `test` handler of the project's own, a command-level `commands.test.run`, `test rust` switched off, or a declared line with no words, each named.

## What it reports

It counts the tests each leg executed from cargo's `test result:` lines, summed over every test target; ignored and filtered-out tests are not counted. It prints both counts and one of:

    DORMANT: <n> test(s) run ONLY under --all-features.
    NO DORMANCY: both invocations execute the same <n> test(s).
    NOT NESTED: --all-features runs <n> FEWER test(s) than the declared line.
    DORMANCY INCONCLUSIVE -- one of the two legs is not a count of tests.

NOT NESTED means the two lines run different suites rather than a suite and a superset of it, as when a feature replaces an implementation, so no dormancy figure follows. INCONCLUSIVE means a leg exited non-zero or printed no `test result:` line: a leg that failed to build reached no tests, and a leg that ran red is a different suite, so no figure is reported, only each leg's count and exit code. A leg that ran and executed no test is not inconclusive: its count is zero, so a suite wholly behind features reads DORMANT.

## When there is nothing to compare

If the declared line already passes `--all-features`, it is its own maximum: dormancy says so, runs nothing and exits 0. If the line does not contain `cargo test`, there is no `--all-features` form of it to compare against, and dormancy refuses with exit 1.

## Exit codes

    0    DORMANT, NO DORMANCY or NOT NESTED; a declared line that already
         passes --all-features; or --help
    1    INCONCLUSIVE, a declared line with no cargo test in it, test rust
         run by something other than a line of its own, switched off or
         with no words, a config that is missing or cannot be loaded, or
         an argument other than --help
    2    not offered here: rust is not declared, or dormancy is disabled

## Changing or replacing it

The legs run the line the project's own `test rust` runs: `commands.test.options.rust.run` changes both, the first leg running that line and the second it with `--all-features` placed. `commands.dormancy.run` or a handler at `bin/.devbin/cmd/dormancy` replaces the command, and `commands.dormancy.enabled: false` drops it.

## See also

    bin/devbin help test    the command each leg runs
