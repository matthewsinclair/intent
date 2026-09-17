---
wp_id: WP-05
title: Rust resolved references through rust-analyzer's SCIP export
scope: L
status: WIP
---

# WP-05: Rust resolved references through rust-analyzer's SCIP export

## Objective

Resolve Rust references to the definitions they name with rust-analyzer's SCIP export, on an explicit verb, into the level-3 core both languages share: a resolved row joins a written reference on its file, line and name, `intent index status` counts what each run matched, left unmatched, dropped and found ambiguous and names the stale paths, and a missing, failed or stale tier is named in the search envelope rather than answering as an empty one.

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

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0076/acceptance.md`, under the `WP-05` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
