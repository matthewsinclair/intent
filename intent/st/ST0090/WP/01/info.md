---
wp_id: WP-01
title: One level-3 state on every face, and the index report names its reconcile time
scope: S
status: Not Started
---

# WP-01: One level-3 state on every face, and the index report names its reconcile time

## Objective

The index report names a language's level 3 the way every search answer does, from one function in intentsvcs, so `intent index status`, `intent index rebuild` and `intent index resolve` can never say `current` above a list of stale files; and the report, text and JSON, names when the index was last reconciled, which Intent.app's row reads in WP-02.

## Builder and judge

cc builds this package and claims `ST0090/01` in the same chain as `intent wp start ST0090/01`, after checking with `intent wb status --json` that no active peer holds it. vc judges it and claims nothing. It lands alone, ahead of ic's WP-02, which reads the reconcile time this package adds.

## What moves

- **One derivation.** Move the rule `ResolutionState::of` applies (intentsvcs, `search.rs`) into one function every face asks: `current` only for a run that stored with nothing stale; `stale` for a stored run with stale paths, which includes the extractor's move because that fills `Run::stale` with every path; and the run's own `missing` or `failed` whatever is stale. `Run::state` keeps recording what the last run did, and no face prints it as the language's state. Where the function lives is cc's call: `index::resolved`, beside `Run`, is the natural home. The search envelope's `ResolutionState::of` and intent-cli's `resolution_lines` both call it.
- **The text report.** `resolution_lines` (intent-cli, `render.rs`), which `intent index status`, `intent index rebuild` and `intent index resolve` all print through, names that state on each language's first line. The stored line, the dropped line, the extractor line and the stale list stay as they are, the list in full.
- **The JSON.** `report_index` serialises `status.resolution`, the raw `Run`, so today its JSON says `"state": "current"` beside a non-empty `stale`. The JSON names the derived state for each language, and the last run's own outcome stays readable as a fact of its own. The field names are cc's call, and a renamed field is named under the next version in CHANGELOG. Measured readers of `index status` output: `the_index_says_what_it_holds.rs`, `every_face_says_what_level_three_knows.rs` and `render.rs`'s own unit tests. No Swift file, no script under `bin/` and no doc quotes the line.
- **The reconcile time.** `index::status::Status` gains the store's `reconciled_at` (`Store::reconciled_at`, stamped by `stamp_reconciled` in `Facade::index_rebuild` and `Facade::index_refresh`), and the report names it in text and JSON. It is a column the reconcile wrote, never a clock read at render (D42, `one_clock.rs`), and an index with no stamp names none. intentd's watcher refreshes stamp the same column, so it says when the index was last brought in line with the tree, not when it was last rebuilt.

## Arms, named before the bank

- A run that stored, with stale paths: the report's first line and the search envelope both say `stale`, and the JSON agrees. `common::level_three`'s fixture already drives `index status` over resolved, failed and unresolved runs.
- The same run with nothing stale says `current`, and the envelope names nothing.
- A failed or missing run with stale paths keeps its own state in both faces.
- The existing pins hold: `the_stored_line_says_how_many_files_the_stored_run_joined` (current, nothing stale), `every_face_says_what_level_three_knows`' failed line, and `the_index_says_what_it_holds`' rule that status and rebuild print one report.
- `index status --json` names a reconcile time after a rebuild and none before one, and the text says the same.

The AT rows cite `native/rust/crates/intent-cli/tests/the_index_report_names_level_three_as_search_does.rs` and `native/rust/crates/intent-cli/tests/the_index_report_names_its_reconcile_time.rs`. Re-cite them with `intent at edit` if the bank names the arms elsewhere.

## The judge, named before the run

One heavy run at the desk, in cc's worktree at the bank's tip: the format check, workspace clippy at `-D warnings`, the intentsvcs and intent-cli suites whole, and `intent critic rust` on every changed `.rs` file (decision 79). After the landing and the build, vc drives the installed pair in this checkout: `intent index status` and its `--json`, and `intent search --json` with one Rust file made stale, reading the state each names.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0090/acceptance.md`, under the `WP-01` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
