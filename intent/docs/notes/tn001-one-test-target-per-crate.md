---
verblock: "27 Nov 2026:v1.2: Matthew Sinclair - Synced to the code, history cut"
---

# TN001: One test target per crate

hv ruled this for every estate on 2026-08-27. Intent applied it at `71a96213` on 2026-09-01, and a test enforces it (below). A live figure on this page is given as the command that reads it. A historical figure is dated, and is here only as evidence.

## The defect

Cargo's `autotests` is `true` by default. Then every `.rs` file directly under a crate's `tests/` becomes its own test target: a separate compile and a separate full link of the crate and all its dependencies. Linking is most of the cost, so the cost grows with the number of test files, not with the number of tests. Nothing warns you, and the cost grows one file at a time.

## The fix, in four parts

Parts 1 to 3 are for each crate. Part 4 is for the workspace.

**1. Turn off discovery and declare one target.** In each crate's `Cargo.toml`:

```toml
autotests = false

[[test]]
name = "suite"
path = "tests/suite.rs"
```

**2. Make each former target a module of that one target.** `tests/suite.rs` is a list of `#[path]` module declarations, one for each file that was a target. The test bodies do not move or change.

**3. Guard against orphans.** After the change, a file added to `tests/` and not declared in `suite.rs` is compiled by nothing and runs in no suite, so it passes without running. Before the change, a stray file became an extra binary. Neither case announces itself, so the guard is part of the change, not a later task.

- Key the guard on `#[path = "..."]`, never on the `mod` name. Only the path decides what compiles.
- Prove the guard with a real file planted in `tests/`, not only a synthetic control. A walker that returns an empty list on a `read_dir` error finds no orphans and goes green on a moved directory. Make each instrument assert that its own corpus is not empty.
- Guard only the undeclared direction. A declared `#[path]` with no file behind it is already a compile error.
- The guard cannot catch its own removal from `suite.rs`. Do not tidy its declaration away.

**4. Cut debuginfo, once, at the workspace root.**

```toml
[profile.dev]
debug = "line-tables-only"
```

`cargo test` inherits `dev`, so this one key reaches every test target. `line-tables-only` keeps the file and line in a backtrace and drops the detail that only a debugger reads.

**Pass `--no-fail-fast` at every `cargo test` call site.** Cargo stops after the first failing target, so without the flag one failure hides the results of every target after it. This applies as soon as a workspace has more than one target, a lib and a bin included. Lamplight's CI lost every failing target's report but the first on 2026-08-27, when its targets became one.

## Keep the files where they are

Use `#[path]` so that no test file moves. An acceptance test (AT) row cites its test file by path, and `intent ac gate` refuses a green or red AT whose cited file does not exist (issue 0015). A move therefore blocks the gate of each thread that cites a moved file, until `intent at edit --file` retargets each row. Lamplight moved its files on 2026-08-27, and the stale citations stayed unseen for five days.

## Measure with cargo, not with grep

Test files are not cargo targets. They are equal only where `autotests` has its default. Count the real targets by asking cargo:

```
cargo test --manifest-path native/rust/Cargo.toml --workspace --no-run 2>&1 | grep -c 'Executable tests/'
```

`bin/devbin check autotests` reports each crate's declared targets, and `grep -c '^\[\[test\]\]' */Cargo.toml` and `grep -rn 'autotests' --include=Cargo.toml .` help to diagnose. Only the cargo count decides. A crate with one test file and no `autotests` key shows no declared target to grep, but cargo builds one.

A census that walks the file system also counts each crate once for each git worktree that holds a copy of it. Both errors make an estate that did the work look like one that did not.

## What enforces it in Intent

- `native/rust/crates/intentsvcs/tests/test_target_topology_guard.rs` walks every crate manifest with comments stripped. It asserts that a crate with `tests/*.rs` declares `autotests = false` and a `[[test]]` target, and that `autotests = false` never stands without one. It asserts its own population first.
- `testkit::assert_no_orphan_suite_members` is the one orphan guard. Each crate calls it from a thin `tests/no_orphan_suite_member.rs`.
- `IN-RS-TEST-003` (one declared test target per crate) and `IN-RS-TEST-004` (trim debuginfo, never fail fast) are in the rule library. The headless gate cannot answer them, because each violation is an absent key and the runner matches only what is present. `critic-rust` applies them in review, and the test above enforces them.
- `test rust` in `bin/.devbin/config.yaml` and the test step in `.github/workflows/rust.yml` both run `cargo test --workspace --no-fail-fast`. `--workspace` with no `--lib` builds and runs the `suite` targets and the orphan guards.

## Intent's separate targets

`intent-cli` declares `dual_path_conformance`, and `intentd` declares `daemon_subscriptions`, beside each crate's `suite`. In one target, tests are threads in one process. `dual_path_conformance.rs` changes the process's working directory, and `daemon_subscriptions.rs` drives a file-watch stream for each process, so each needs its own process.

Before you give a test its own target, ask what it asserts. A test about the process it runs in needs its own process. A test about the product needs the product fixed. `daemon_address.rs` also failed now and then in the merged suite. Its failure was a real race: a forked child inherited the daemon lock's descriptor (fixed at `3fc1d152`). Its own target would have hidden the race, not fixed it, so it stays in the suite.

## Verifying a consolidation

The risk of a merge is intermittent failure, because tests that were separate processes now share one. Clean runs cannot show that it is safe.

- Run the suite in a loop under full-workspace load until it fails, or until you have a number of runs you can defend.
- When you think you have found a cause, run the opposite case. If removing the supposed cause does not help, the diagnosis is wrong.
- `cargo test <filter>` exits 0 when the filter matches no test. A check that selects tests by name must assert how many ran, not only the exit code.

## Moving a file back out

A merge removes a file's `mod common;` and rewrites its paths to `crate::common::`. To give the file its own target again, put the `mod common;` declaration back. Otherwise the file does not compile, and the error names the helper, not the move.

## The target tree

On 2026-09-01 the target tree held 113 G, and per-node trees held much of it. `bin/devbin build all` rebuilt the release pair cold in 1 m 36 s, so almost all of the tree was test-build cache.

A session that waits on cargo's build lock cannot see the wait, so it can point `CARGO_TARGET_DIR` at its own tree to avoid it. `bin/devbin check targetdir` flags a per-node target tree, and devbin's gate runner prints a heartbeat when a gate goes quiet. A bare `cargo` run outside devbin still waits in silence.

## For another project

1. Count your real targets with cargo, as above.
2. Build the orphan guard before you consolidate.
3. Apply the four parts to each crate whose test files outnumber its declared targets.
4. Write your own note, about your own crates and call sites. File it in the notes tree you already have, with the next number in your own sequence.
5. Before you remove a declared language that you own no code in, find how the gate would reach that code. Where no staged file can be in that language, the declaration has no effect.
6. A crate with only inline `#[cfg(test)]` modules has no `tests/` and needs no change: `autotests = false` there would only make a quiet hole. Apply the four parts with the crate's first test file.
