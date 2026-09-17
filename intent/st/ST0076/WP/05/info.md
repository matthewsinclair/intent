---
wp_id: WP-05
title: Rust resolved references through rust-analyzer's SCIP export
scope: L
status: Done
---

# WP-05: Rust resolved references through rust-analyzer's SCIP export

## Objective

Resolve Rust references to the definitions they name with rust-analyzer's SCIP export, on an explicit verb, into the level-3 core both languages share: a resolved row joins a written reference on its file, line and name, `intent index status` counts what each run matched, left unmatched, dropped and found ambiguous and names the stale paths, and a missing or failed run is recorded with its reason and named by `intent index resolve` and `intent index status` rather than stored as a current run that resolved nothing. The search envelope's level-3 facts are WP-07's.

## Measured first (cc, 2026-09-17)

On a fresh clone of main at `c800b8319`, indexed by the installed pair at the same commit (extractor 2) under an isolated HOME. The tool is rust-analyzer 0.0.0 (682a84e95b 2026-09-13), installed by Homebrew at `/opt/homebrew/bin/rust-analyzer` and the only one on PATH; there is no rustup, and Homebrew rust 1.98.1's sysroot carries the library sources and `rust-analyzer-proc-macro-srv`.

- **The export runs project code.** `rust-analyzer scip native/rust` took 28 s wall and 3.2 GB peak memory, exited 0 and wrote a 29 MB index. It ran 42 build scripts, the workspace's own `build.rs` files among them, built 25 proc macros for expansion, and wrote 304 MB of check artefacts into the workspace's target directory. Its cargo metadata step waited on the shared `~/.cargo` package lock.
- **Nothing turns that off.** `scip` has no disable flag, and `cargo.buildScripts.enable` and `procMacro.enable` set to false through its `--config-path` changed nothing: the same build scripts and proc macros ran, into a fresh `CARGO_TARGET_DIR`, and the decoded occurrences were identical. `CARGO_TARGET_DIR` is honoured.
- **Absence and failure are distinguishable.** A missing binary is ENOENT on spawn (127 through a shell). A directory that is not a Cargo workspace exits 1 with `Error: no projects` and a backtrace, and writes no index.
- **The join fields.** `project_root` is the path the export was given, and each document's `relative_path` is relative to it, so the index's path is the workspace root plus that path. Every document is UTF-8 with 0-based lines; `symbol_roles` bit 1 marks a definition; a symbol reads `rust-analyzer cargo <crate> <version> <descriptors>` and a local `local N`. The name is not a field: it is the source text at the occurrence's range.
- **The counts on this repository.** 235,355 references: 70,743 to definitions in the index, 108,852 to std and dependencies, 55,760 to locals. 146,743 written Rust reference rows (level 1 120,471, level 2 26,272); 144,554 of them join a reference on file, line and name, 118,387 at level 1 and 26,167 at level 2. 90,801 references have no written row (most internal ones are fields, variants and constants, which the syntax does not write); 2,189 written rows have none (367 in files outside the Cargo workspace). 939 keys name two or more definitions, such as `expect` on an `Option` and a `Result` on one line.
- **References inside macro invocations resolve.** The `is_local` calls inside `assert!` and the `nearest_project` calls at `views.rs:437` and `:439` inside `assert_eq!` each join their written row.

## Rulings (vc, under the pen, 2026-09-17, vc decision 25)

1. **One explicit verb for both languages**, `intent index resolve [--lang] [--full]` (the name agreed by hv, 2026-09-17), never intentd, reconcile or a hook, and not offered on the MCP tool tier in this release.
2. **Every written reference row is the join's population**, level 1 and level 2.
3. **Five counts per language, from the last run**: matched, unmatched, dropped, ambiguous, and the stale paths.
4. **A side table, one row per (path, line, name, target), with no tie-break at write**: a line that calls two definitions of one name stores both. The target is one printable name per language, with no tool or crate version, and the definition's path and line where the index holds it. The staleness facts (the file's content hash when the tool read it, the language, the run) are held once per file. A run joins only against the bytes the tool read, replaces exactly the files it resolved in one transaction, writes nothing when it fails, and purges rows whose path has left the index; reconcile never deletes resolved rows. One schema step.
5. **The decoder is hand-written**: a pure module in intentsvcs, fields by number with the numbers named, unknown fields skipped by wire type.
6. **The export builds into Intent's own ignored directory under `intent/.cache`**, never the workspace's target directory.
7. **Arms**: decode and join read a checked-in fixture index, and the absence arm runs with the tool off PATH. **The toolchain arm (hv, 2026-09-17, first-hand)**: one end-to-end arm per language on a tiny fixture project (for Rust: spawn rust-analyzer, export, decode, join, store), marked `#[ignore = "needs <tool> on PATH; bin/devbin test all runs it"]`, with `needs_the_toolchain` in its module path, failing by name when the tool is absent and never skipping. It stays out of CI for now and runs on this machine whenever `bin/devbin test all` and `bin/devbin fullcycle` run; dc adds the devbin option that runs `-- --ignored needs_the_toolchain`, and the lane that lands the first such arm carries that line in the same landing, so no such arm exists unrun.

## Build order

The level-3 core first, alone, as its own bank (the tables, per-file replace, staleness, purge, the five counts, the verb and its register row), proven with an in-memory reader, so WP-06 stacks on it. The envelope's `index.resolution` field (a language whose tier is missing, failed or stale) is built on WP-04's landing. Then the SCIP decoder and the Rust reader, and the landing, which also corrects design.md's line that rust-analyzer is not installed on this host.

## The level-3 core, as landed

The core lands alone, ahead of any reader (vc, 2026-09-17), so `intent index resolve` refuses every language by name until a language's reader registers in `index::resolved::readers()`. Beyond decision 25 it carries vc's rulings of 2026-09-17 on dc's review of the reader contract:

- **A run is forced full** when the language has no stored run, its last run did not store, or its record's extractor version differs from the build's. An incremental run over a build cache that already exists would otherwise trace nothing and report an empty tier as a success.
- **The record carries the extractor version it joined against** (`resolution.symbols_version`), so a build writing another version makes every resolved path of that language stale.
- **The index catches up after the tool has read and before the join**, never before the tool runs: an edit made before the tool's read joins, and an edit made after it is dropped as moved and retraced by the next run. A refresh that fails fails the run.
- **`dropped` is counted by reason**: the core's own (`no-line`, `outside-the-project`, `unread`, `not-indexed`, `moved`) and a closed roster each reader declares, where a reason nobody declared fails the run. `matched`, `unmatched` and `dropped` sum to everything the tool emitted, and an arm holds that law.
- **A target the tool does not locate** is placed at the definition row the reader chooses among the rows that print it: exactly one by default, and the first clause for Elixir.
- **A project holding nothing for a tool** (no `Cargo.toml`, no `mix.exs`) is not applicable: a run over every declared language names it and leaves its record alone, and a run that asked for it with `--lang` records a failure.
- **A reader finds its project in the index's file rows** (`Scope::indexed`), never by walking the tree or asking git. Rust's roots are the root-most `Cargo.toml` manifests, each exported on its own.
- **A build directory Intent cannot create fails that language's run** rather than refusing the verb.

## The Rust reader, as landed

rust-analyzer is Rust's reader, registered in `index::resolved::readers()`, so `intent index resolve` runs it for a project that declares `rust`. It carries vc's rulings of 2026-09-17 on its three questions:

- **One export per root, built under Intent's own directory.** A root is a `Cargo.toml` the index holds with no ancestor holding another, and each is exported with `CARGO_TARGET_DIR` under `intent/.cache/resolve/rust`, never into the workspace's `target/`. `--full` changes nothing for this reader: every export is of the whole workspace, and cargo decides what to rebuild.
- **A file is offered as read only where its bytes hash the same before and after the export**, so a file saved during the export drops its references as unread rather than joining them against lines the tool never read.
- **One printed target per definition, with no tool or crate version in it**: `crate::path::Type::method()`, a field plain, `<Type as Trait>::method()`, `name!`, `#[Derive]`, and `<impl Type>` for an impl block itself. A callable ends in `()` because a field and a method of one name are two definitions. The module in a member's path is the one its impl block is written in, which is what SCIP carries, and not always the module that defines the type. A crate's `-` prints as `_`.
- **Four reasons of the reader's own, each declared**: `local` (a local variable, closure or parameter), `multiline` (an occurrence spanning lines), `operator` (a reference whose text holds no letter, digit or underscore, which the export writes for an overloaded operator on the operator and on the space either side of it) and `unprintable` (a symbol it cannot print). Empty text, `self.0`, `Self`, `crate`, `super` and a raw identifier are not operators: they go to the join, and count as unmatched where no written row holds them.
- **The export can write one file into the project.** Where a workspace root holds no `Cargo.lock`, cargo writes one there, as any cargo command would. The reader leaves it and never deletes or moves a file in the project tree, and the verb's register row says so beside the build scripts and proc macros it runs.
- **A tool that is not there is recorded as missing, naming the program**, and a project whose index holds no `Cargo.toml` is not applicable.
- **The arms**: the checked-in export of a tiny crate (`tests/fixtures/scip/tiny/`) proves the decode, the printed targets, the reasons and the conservation law in every build; a program that does not exist proves the missing record; and one `needs_the_toolchain` arm runs the real rust-analyzer over the same crate, under `bin/devbin test all` and never in CI.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0076/acceptance.md`, under the `WP-05` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
