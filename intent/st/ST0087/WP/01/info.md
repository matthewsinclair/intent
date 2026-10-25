---
wp_id: WP-01
title: The tree function, the runtime set and the version line
scope: M
status: WIP
---

# WP-01: The tree function, the runtime set and the version line

## Objective

One function in intentsvcs names the running tree, read in the contract's order (brew when the manifest says `managed_by: brew`; opt when its `tool` is `intent`; dev when the root is a git work tree with no manifest; unknown otherwise, which doctor fails), and every caller asks it. One function lists the runtime set, the pair plus the support paths git names (`lib/templates`, `intent/plugins/claude`, `intent/plugins/agents/plugin.json`), and `bin/.devbin/cmd/shared/support.lib` reads that list from the built binary instead of keeping `SUPPORT_PATHS` of its own. The version line becomes `intent <version> (<tree>, <sha12>[+dirty]) at <root> <provenance>`: the sha is the commit the running pair was built from, read from the binary and never HEAD at run time; `+dirty` follows it in dev when the support set differs from that commit or the pair was built from a dirty tree, never in opt or brew; `<root>` is absolute, and a brew root is its opt link; the trailing provenance word (`release`, `dev`) stays, which readers of the first four fields ignore. Doctor prints the same line and says when HEAD has moved past the build. Every reader of the old line moves in the same bank: `bin/int`, the devbin commands `macos`, `brew`, `local`, `linux`, `prepush`, `shared/brew.lib`, `shared/releasestamp.lib` and `build.d/release`, the bats files that match the line, the ST0056 parity tools, and the formula's `test do`. Arms: the tree function on each of the four answers from scratch roots; the line's shape and the sha source in dev with a pair built behind HEAD; the dirty marker with a dirty support set.

## Measurements, and the readers that move (cc, 2026-10-25 07:08Z by date -u)

Static measurements from main's tree, taken before the bank; the readers below are driven again against a pair printing the new line at the judge.

- **M1, the runtime set.** It had one definition, `SUPPORT_PATHS` in `bin/.devbin/cmd/shared/support.lib`, and the binary held no copy (only `install::MARKER`, the root marker). It is used by `cmd/macos`'s stage (coverage, the coverage loop, `provenance_blockers`, the support asset and its drift check), `cmd/linux`'s package dirt check, `support.lib`'s own `git archive`, and the `macos_release_assets.bats` fixture. `cmd/brew` and `shared/brew.lib` name it only in comments.
- **M2, the readers of the line.** Field 2, the version, survives unchanged: `brew.lib`'s `staged_version`, `build.d/release`'s stamp check, `macos publish` through `staged_version`, and the formula's `test do`. These print or record the whole line and survive too: `brew smoke`, `cmd/linux`'s build and smoke, `prepush` (rc only), the pre-commit hooks' parentheticals, and `burn.sh`'s TSV header. `bin/int` prints devbin's line, not Intent's. The `dirty-` readers under `bin/.devbin` read the embedded `[intent-source-commit:...]` marker, which this bank leaves as it is.
- **M3, `at <root>` for brew.** On this Mac `/opt/homebrew/opt/intent` links to `../Cellar/intent/<version>`, and the binary's real path is under the Cellar, so the root prints through `stable_root`'s Cellar-to-opt mapping. Linux (kovacs) is unmeasured, and that keg is hv's.
- **M4, the provenance word.** No shell reads it: the gate's self-provenance arm and `verify_pair` read the embedded marker. It stays last at no cost.

### What this bank moves

- The two printers, intent-cli's `spine.rs` (with `intent version` in `render.rs`) and intentd's `main.rs`, now call one formatter, `intentsvcs::install::version_line`. The CLI builds the line when it is asked for, not when the Command is, since a dev tree's line reads git.
- `intent/st/ST0056/parity/tools/gen_explorer_shots.sh`, which split the line by position.
- The Rust pins `intent-cli/tests/version_spellings_agree.rs` and `intentd/tests/arguments_do_not_start_a_daemon.rs`.
- The bats fixtures: `macos_release_assets.bats` reads the list from `$INTENT_BIN`, and `burn_subject_provenance.bats` fakes the new line.
- `support.lib`'s `SUPPORT_PATHS` is gone. `support_paths_from <intent>` reads it from the named pair (`cmd/macos` stage from the release pair it stages, `cmd/linux` package from the pair it packages), and refuses when that pair is absent or cannot answer.

### A known reader that moves in WP-03 (vc's ruling, 2026-10-25)

`native/macos/Intent/Intent/Services/VersionService.swift`'s `parse` reads `intent version` for the masthead's version, commit and kind. Read from source, not driven: under the new line its pattern does not match, so it returns the whole line as the version, with commit and kind nil and no crash. The fix is Swift and rides ic's WP-03. Both land in 3.2.7, so no shipped state shows it. The dev Intent.app shows it between this bank's build and ic's landing.

### The judge, named before the run

One heavy run at molt-vc's desk, in the worktree at the bank's tip: `cargo fmt --check`; workspace clippy at `-D warnings`; the whole `intentsvcs`, `intent-cli` and `intentd` suites, `intentd` included because the bank adds files under `crates/*/tests`; `macos_release_assets.bats` and `burn_subject_provenance.bats` with `INTENT_BIN` naming the worktree's build. Then every named reader is driven against that build: `intent --version`, `intent version` and `intentd --version` (shape and byte identity); `intent doctor`'s first line; `intent install --print-runtime-set` and bare `install` (rc 2); `support_paths_from` against the build; `brew.lib`'s `staged_version` over a staged copy of it; `build.d/release`'s `"intent $TARGET "*` match; `burn.sh`'s VERSION capture; and `gen_explorer_shots.sh`'s parse of the line.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0087/acceptance.md`, under the `WP-01` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
